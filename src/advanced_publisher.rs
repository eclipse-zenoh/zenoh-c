//
// Copyright (c) 2017, 2022 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//

use std::{mem::MaybeUninit, time::Duration};

use zenoh::{
    handlers::Callback,
    matching::MatchingStatus,
    qos::{CongestionControl, Priority},
    session::SessionClosedError,
    Wait,
};
use zenoh_ext::{AdvancedPublisherBuilderExt, CacheConfig, MissDetectionConfig};

use crate::{
    _apply_pubisher_delete_options, _apply_pubisher_put_options, _declare_publisher_inner,
    result::{self},
    transmute::{IntoCType, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_matching_status_call, z_closure_matching_status_loan, z_congestion_control_t,
    z_entity_global_id_t, z_loaned_keyexpr_t, z_loaned_session_t, z_matching_status_t,
    z_moved_bytes_t, z_moved_closure_matching_status_t, z_owned_matching_listener_t, z_priority_t,
    z_publisher_delete_options_t, z_publisher_options_t, z_publisher_put_options_t,
};

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Setting for advanced publisher's cache. The cache allows advanced subscribers to recover history and/or lost samples.
#[prebindgen]
#[repr(C)]
pub struct ze_advanced_publisher_cache_options_t {
    /// Must be set to ``true``, to enable the cache.
    pub is_enabled: bool,
    /// Number of samples to keep for each resource.
    pub max_samples: usize,
    /// The congestion control to apply to replies.
    pub congestion_control: z_congestion_control_t,
    /// The priority of replies.
    pub priority: z_priority_t,
    /// If set to ``true``, this cache replies will not be batched. This usually has a positive impact on latency but negative impact on throughput.
    pub is_express: bool,
}

impl Default for ze_advanced_publisher_cache_options_t {
    fn default() -> Self {
        Self {
            is_enabled: true,
            max_samples: 1,
            congestion_control: CongestionControl::DEFAULT_PUSH.into(),
            priority: Priority::default().into(),
            is_express: false,
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs the default value for `ze_advanced_publisher_cache_options_t`.
#[prebindgen]
pub fn ze_advanced_publisher_cache_options_default(
    this: &mut ::std::mem::MaybeUninit<ze_advanced_publisher_cache_options_t>,
) {
    this.write(ze_advanced_publisher_cache_options_t::default());
}

impl From<&ze_advanced_publisher_cache_options_t> for CacheConfig {
    fn from(val: &ze_advanced_publisher_cache_options_t) -> CacheConfig {
        let mut c = CacheConfig::default();
        c = c.max_samples(val.max_samples);
        let qos = zenoh_ext::RepliesConfig::default()
            .congestion_control(val.congestion_control.into())
            .express(val.is_express)
            .priority(val.priority.into());
        c = c.replies_config(qos);
        c
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ze_advanced_publisher_heartbeat_mode_t {
    /// Disable heartbeat-based last sample miss detection.
    NONE = 0,
    /// Allow last sample miss detection through periodic heartbeat.
    /// Periodically send the last published Sample's sequence number to allow last sample recovery.
    PERIODIC = 1,
    /// Allow last sample miss detection through sporadic heartbeat.
    /// Each period, the last published Sample's sequence number is sent with `z_congestion_control_t::BLOCK`
    /// but only if it changed since last period.
    SPORADIC = 2,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Settings for sample miss detection on Advanced Publisher.
#[prebindgen]
#[repr(C)]
pub struct ze_advanced_publisher_sample_miss_detection_options_t {
    /// Must be set to ``true``, to enable sample miss detection by adding sequence numbers.
    pub is_enabled: bool,
    /// Allow last sample miss detection through sporadic or periodic heartbeat.
    pub heartbeat_mode: ze_advanced_publisher_heartbeat_mode_t,
    /// If heartbeat_mode is not NONE, the publisher will send heartbeats with the specified period, which
    /// can be used by Advanced Subscribers for last sample(s) miss detection (if last sample miss detection with zero query period is enabled).
    pub heartbeat_period_ms: u64,
}

impl Default for ze_advanced_publisher_sample_miss_detection_options_t {
    fn default() -> Self {
        Self {
            is_enabled: true,
            heartbeat_mode: ze_advanced_publisher_heartbeat_mode_t::NONE,
            heartbeat_period_ms: 0,
        }
    }
}

impl From<&ze_advanced_publisher_sample_miss_detection_options_t> for MissDetectionConfig {
    fn from(val: &ze_advanced_publisher_sample_miss_detection_options_t) -> Self {
        let mut m = MissDetectionConfig::default();
        if val.heartbeat_mode == ze_advanced_publisher_heartbeat_mode_t::SPORADIC {
            m = m.sporadic_heartbeat(Duration::from_millis(val.heartbeat_period_ms))
        } else if val.heartbeat_mode == ze_advanced_publisher_heartbeat_mode_t::PERIODIC {
            m = m.heartbeat(Duration::from_millis(val.heartbeat_period_ms))
        } else if val.heartbeat_period_ms > 0 {
            tracing::warn!("ze_advanced_publisher_sample_miss_detection_options_t: heartbeat_mode=NONE but heartbeat_period_ms={}. heartbeat_mode=PERIODIC is used instead, but this behavor will be removed later", val.heartbeat_period_ms);
            m = m.heartbeat(Duration::from_millis(val.heartbeat_period_ms))
        }
        m
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs the default value for `ze_advanced_publisher_sample_miss_detection_options_t`.
#[prebindgen]
pub fn ze_advanced_publisher_sample_miss_detection_options_default(
    this: &mut MaybeUninit<ze_advanced_publisher_sample_miss_detection_options_t>,
) {
    this.write(ze_advanced_publisher_sample_miss_detection_options_t::default());
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Options passed to the `ze_declare_advanced_publisher()` function.
#[prebindgen]
#[repr(C)]
pub struct ze_advanced_publisher_options_t {
    /// Base publisher options.
    pub publisher_options: z_publisher_options_t,
    /// Publisher cache settings.
    pub cache: ze_advanced_publisher_cache_options_t,
    /// Settings to allow matching Subscribers to detect lost samples and optionally ask for retransimission.
    ///
    /// Retransmission can only be done if cache is enabled.
    pub sample_miss_detection: ze_advanced_publisher_sample_miss_detection_options_t,
    /// Allow this publisher to be detected through liveliness.
    pub publisher_detection: bool,
    /// An optional key expression to be added to the liveliness token key expression.
    /// It can be used to convey meta data.
    pub publisher_detection_metadata: Option<&'static z_loaned_keyexpr_t>,
}

/// Constructs the default value for `z_publisher_options_t`.
#[prebindgen]
pub fn ze_advanced_publisher_options_default(
    this_: &mut MaybeUninit<ze_advanced_publisher_options_t>,
) {
    let cache = ze_advanced_publisher_cache_options_t {
        is_enabled: false,
        ..Default::default()
    };
    let sample_miss_detection = ze_advanced_publisher_sample_miss_detection_options_t {
        is_enabled: false,
        ..Default::default()
    };
    this_.write(ze_advanced_publisher_options_t {
        publisher_options: z_publisher_options_t::default(),
        cache,
        sample_miss_detection,
        publisher_detection: false,
        publisher_detection_metadata: None,
    });
}

pub use crate::opaque_types::{
    ze_loaned_advanced_publisher_t, ze_moved_advanced_publisher_t, ze_owned_advanced_publisher_t,
};
decl_c_type!(
    owned(ze_owned_advanced_publisher_t, option zenoh_ext::AdvancedPublisher<'static>),
    loaned(ze_loaned_advanced_publisher_t),
);

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Constructs and declares an advanced publisher for the given key expression.
///
/// Data can be put and deleted with this publisher with the help of the
/// `ze_advanced_publisher_put()` and `ze_advanced_publisher_delete()` functions.
///
/// @param session: The Zenoh session.
/// @param publisher: An uninitialized location in memory where advanced publisher will be constructed.
/// @param key_expr: The key expression to publish to.
/// @param options: Additional options for the advanced publisher.
///
/// @return 0 in case of success, negative error code otherwise.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_declare_advanced_publisher(
    session: &'static z_loaned_session_t,
    publisher: &'static mut MaybeUninit<ze_owned_advanced_publisher_t>,
    key_expr: &'static z_loaned_keyexpr_t,
    mut options: Option<&'static mut ze_advanced_publisher_options_t>,
) -> result::z_result_t {
    let this = publisher.as_rust_type_mut_uninit();
    let p = _declare_publisher_inner(
        session,
        key_expr,
        options.as_mut().map(|o| &mut o.publisher_options),
    );
    let mut p = p.advanced();
    if let Some(options) = options {
        if options.publisher_detection {
            p = p.publisher_detection();
        }
        if options.sample_miss_detection.is_enabled {
            p = p.sample_miss_detection((&options.sample_miss_detection).into());
        }
        if let Some(pub_detection_metadata) = &options.publisher_detection_metadata {
            p = p.publisher_detection_metadata(pub_detection_metadata.as_rust_type_ref());
        }
        if options.cache.is_enabled {
            p = p.cache((&options.cache).into());
        }
    }
    match p.wait() {
        Err(e) => {
            crate::report_error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
        Ok(publisher) => {
            this.write(Some(publisher));
            result::Z_OK
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Constructs an advanced publisher in a gravestone state.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_internal_advanced_publisher_null(
    this_: &mut MaybeUninit<ze_owned_advanced_publisher_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Returns ``true`` if advanced publisher is valid, ``false`` otherwise.
#[allow(clippy::missing_safety_doc)]
#[prebindgen]
pub fn ze_internal_advanced_publisher_check(
    this_: &ze_owned_advanced_publisher_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Borrows advanced publisher.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn ze_advanced_publisher_loan(
    this_: &ze_owned_advanced_publisher_t,
) -> &ze_loaned_advanced_publisher_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Mutably borrows advanced publisher.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn ze_advanced_publisher_loan_mut(
    this: &mut ze_owned_advanced_publisher_t,
) -> &mut ze_loaned_advanced_publisher_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Options passed to the `ze_advanced_publisher_put()` function.
#[prebindgen]
#[repr(C)]
pub struct ze_advanced_publisher_put_options_t {
    /// Base put options.
    put_options: z_publisher_put_options_t,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Constructs the default value for `ze_advanced_publisher_put_options_t`.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_advanced_publisher_put_options_default(
    this: &mut MaybeUninit<ze_advanced_publisher_put_options_t>,
) {
    this.write(ze_advanced_publisher_put_options_t {
        put_options: z_publisher_put_options_t::default(),
    });
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Sends a `PUT` message onto the advanced publisher's key expression, transfering the payload ownership.
///
/// The payload and all owned options fields are consumed upon function return.
///
/// @param this_: The advanced publisher.
/// @param payload: The data to publish. Will be consumed.
/// @param options: The advanced publisher put options. All owned fields will be consumed.
///
/// @return 0 in case of success, negative error values in case of failure.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn ze_advanced_publisher_put(
    this: &ze_loaned_advanced_publisher_t,
    payload: &mut z_moved_bytes_t,
    options: Option<&mut ze_advanced_publisher_put_options_t>,
) -> result::z_result_t {
    let publisher = this.as_rust_type_ref();
    let payload = payload.take_rust_type();
    let mut put = publisher.put(payload);
    if let Some(options) = options {
        put = _apply_pubisher_put_options(put, &mut options.put_options);
    }
    match put.wait() {
        Ok(_) => result::Z_OK,
        Err(e) if e.downcast_ref::<SessionClosedError>().is_some() => result::Z_ESESSION_CLOSED,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Represents the set of options that can be applied to the delete operation by a previously declared advanced publisher,
/// whenever issued via `ze_advanced_publisher_delete()`.
#[prebindgen]
#[repr(C)]
pub struct ze_advanced_publisher_delete_options_t {
    /// Base delete options.
    pub delete_options: z_publisher_delete_options_t,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Constructs the default values for the delete operation via an advanced publisher entity.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_advanced_publisher_delete_options_default(
    this: &mut MaybeUninit<ze_advanced_publisher_delete_options_t>,
) {
    this.write(ze_advanced_publisher_delete_options_t {
        delete_options: z_publisher_delete_options_t::default(),
    });
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Sends a `DELETE` message onto the advanced publisher's key expression.
///
/// @return 0 in case of success, negative error code in case of failure.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_advanced_publisher_delete(
    publisher: &ze_loaned_advanced_publisher_t,
    options: Option<&mut ze_advanced_publisher_delete_options_t>,
) -> result::z_result_t {
    let publisher = publisher.as_rust_type_ref();
    let mut del = publisher.delete();
    if let Some(options) = options {
        del = _apply_pubisher_delete_options(del, &mut options.delete_options)
    }
    if let Err(e) = del.wait() {
        crate::report_error!("{}", e);
        result::Z_EGENERIC
    } else {
        result::Z_OK
    }
}
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the ID of the advanced publisher.
#[prebindgen]
pub fn ze_advanced_publisher_id(
    publisher: &ze_loaned_advanced_publisher_t,
) -> z_entity_global_id_t {
    publisher.as_rust_type_ref().id().into_c_type()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Returns the key expression of the publisher.
#[prebindgen]
pub fn ze_advanced_publisher_keyexpr(
    publisher: &ze_loaned_advanced_publisher_t,
) -> &z_loaned_keyexpr_t {
    publisher
        .as_rust_type_ref()
        .key_expr()
        .as_loaned_c_type_ref()
}

#[cfg(feature = "unstable")]
fn _advanced_publisher_matching_listener_declare_inner<'a>(
    publisher: &'a ze_loaned_advanced_publisher_t,
    callback: &mut z_moved_closure_matching_status_t,
) -> zenoh::matching::MatchingListenerBuilder<'a, Callback<MatchingStatus>> {
    let publisher = publisher.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let listener = publisher
        .matching_listener()
        .callback_mut(move |matching_status| {
            let status = z_matching_status_t {
                matching: matching_status.matching(),
            };
            z_closure_matching_status_call(z_closure_matching_status_loan(&callback), &status);
        });
    listener
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs matching listener, registering a callback for notifying subscribers matching with a given advanced publisher.
///
/// @param publisher: An advanced publisher to associate with matching listener.
/// @param matching_listener: An uninitialized memory location where matching listener will be constructed. The matching listener's callback will be automatically dropped when the publisher is dropped.
/// @param callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber disconnects or when the first subscriber connects).
///
/// @return 0 in case of success, negative error code otherwise.
#[prebindgen]
pub fn ze_advanced_publisher_declare_matching_listener(
    publisher: &'static ze_loaned_advanced_publisher_t,
    matching_listener: &mut MaybeUninit<z_owned_matching_listener_t>,
    callback: &mut z_moved_closure_matching_status_t,
) -> result::z_result_t {
    let this = matching_listener.as_rust_type_mut_uninit();
    let listener = _advanced_publisher_matching_listener_declare_inner(publisher, callback);
    match listener.wait() {
        Ok(listener) => {
            this.write(Some(listener));
            result::Z_OK
        }
        Err(e) => {
            this.write(None);
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Declares a matching listener, registering a callback for notifying subscribers matching with a given advanced publisher.
/// The callback will be run in the background until the corresponding publisher is dropped.
///
/// @param publisher: An advanced publisher to associate with matching listener.
/// @param callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber disconnects or when the first subscriber connects).
///
/// @return 0 in case of success, negative error code otherwise.
#[prebindgen]
pub fn ze_advanced_publisher_declare_background_matching_listener(
    publisher: &'static ze_loaned_advanced_publisher_t,
    callback: &mut z_moved_closure_matching_status_t,
) -> result::z_result_t {
    let listener = _advanced_publisher_matching_listener_declare_inner(publisher, callback);
    match listener.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Gets advanced publisher matching status - i.e. if there are any subscribers matching its key expression.
///
/// @return 0 in case of success, negative error code otherwise (in this case matching_status is not updated).
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_advanced_publisher_get_matching_status(
    this: &'static ze_loaned_advanced_publisher_t,
    matching_status: &mut MaybeUninit<z_matching_status_t>,
) -> result::z_result_t {
    match this.as_rust_type_ref().matching_status().wait() {
        Ok(s) => {
            matching_status.write(z_matching_status_t {
                matching: s.matching(),
            });
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_ENETWORK
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Frees memory and resets advanced_publisher to its gravestone state.
/// This is equivalent to calling `z_undeclare_publisher()` and discarding its return value.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_advanced_publisher_drop(this: &mut ze_moved_advanced_publisher_t) {
    std::mem::drop(this.take_rust_type())
}

#[prebindgen]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Undeclares the given advanced publisher.
///
/// @return 0 in case of success, negative error code otherwise.
pub fn ze_undeclare_advanced_publisher(
    this_: &mut ze_moved_advanced_publisher_t,
) -> result::z_result_t {
    if let Some(p) = this_.take_rust_type() {
        if let Err(e) = p.undeclare().wait() {
            crate::report_error!("{}", e);
            return result::Z_ENETWORK;
        }
    }
    result::Z_OK
}
