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

use std::mem::MaybeUninit;

use zenoh::{
    handlers::Callback,
    matching::MatchingStatus,
    qos::{CongestionControl, Priority},
    session::SessionClosedError,
    Wait,
};
use zenoh_ext::{AdvancedPublisherBuilderExt, CacheConfig};

use crate::{
    _apply_pubisher_delete_options, _apply_pubisher_put_options, _declare_publisher_inner,
    result::{self},
    transmute::{IntoCType, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_congestion_control_t, z_entity_global_id_t, z_loaned_keyexpr_t, z_loaned_session_t,
    z_moved_bytes_t, z_priority_t, z_publisher_delete_options_t, z_publisher_options_t,
    z_publisher_put_options_t, zc_closure_matching_status_call, zc_closure_matching_status_loan,
    zc_matching_status_t, zc_moved_closure_matching_status_t, zc_owned_matching_listener_t,
};

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Setting for advanced publisher's cache.
#[repr(C)]
pub struct ze_advanced_publisher_cache_settings_t {
    /// Number of samples to keep for each resource
    pub max_samples: usize,
    /// The congestion control to apply to replies.
    pub congestion_control: z_congestion_control_t,
    /// The priority of replies.
    pub priority: z_priority_t,
    /// If true, Zenoh will not wait to batch the cache replies with other messages to reduce the bandwith.
    pub is_express: bool,
}

impl Default for ze_advanced_publisher_cache_settings_t {
    fn default() -> Self {
        Self {
            max_samples: 1,
            congestion_control: CongestionControl::default().into(),
            priority: Priority::default().into(),
            is_express: false,
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs the default value for `ze_advanced_publisher_cache_settings_t`.
#[no_mangle]
pub extern "C" fn ze_advanced_publisher_cache_settings_default(
    this: &mut MaybeUninit<ze_advanced_publisher_cache_settings_t>,
) {
    this.write(ze_advanced_publisher_cache_settings_t::default());
}

impl Into<CacheConfig> for &ze_advanced_publisher_cache_settings_t {
    fn into(self) -> CacheConfig {
        let mut c = CacheConfig::default();
        c = c.max_samples(self.max_samples);
        let qos = zenoh_ext::QoS::default()
            .congestion_control(self.congestion_control.into())
            .express(self.is_express)
            .priority(self.priority.into());
        c = c.replies_qos(qos);
        c
    }
}

/// Options passed to the `ze_declare_advanced_publisher()` function.
#[repr(C)]
pub struct ze_advanced_publisher_options_t {
    /// Base publisher options.
    pub publisher_options: z_publisher_options_t,
    /// Optional settings publisher history cache.
    pub cache: Option<&'static mut ze_advanced_publisher_cache_settings_t>,
    /// Allow matching Subscribers to detect lost samples and optionally ask for retransimission.
    ///
    /// Retransmission can only be done if history is enabled on subscriber side.
    pub sample_miss_detection: bool,
    /// Allow this publisher to be detected through liveliness.
    pub publisher_detection: bool,
    /// An optional key expression to be added to the liveliness token key expression.
    /// It can be used to convey meta data.
    pub publisher_detection_metadata: Option<&'static mut z_loaned_keyexpr_t>,
}

/// Constructs the default value for `z_publisher_options_t`.
#[no_mangle]
pub extern "C" fn ze_advanced_publisher_options_default(
    this_: &mut MaybeUninit<ze_advanced_publisher_options_t>,
) {
    this_.write(ze_advanced_publisher_options_t {
        publisher_options: z_publisher_options_t::default(),
        cache: None,
        sample_miss_detection: false,
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
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_declare_advanced_publisher(
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
        if let Some(pub_detection_metadata) = &options.publisher_detection_metadata {
            p = p.publisher_detection_metadata(pub_detection_metadata.as_rust_type_ref());
        }
        if let Some(cache) = &options.cache {
            p = p.cache((&**cache).into());
        }
    }
    match p.wait() {
        Err(e) => {
            tracing::error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
        Ok(publisher) => {
            this.write(Some(publisher));
            result::Z_OK
        }
    }
}

/// Constructs an advanced publisher in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_internal_advanced_publisher_null(
    this_: &mut MaybeUninit<ze_owned_advanced_publisher_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if advanced publisher is valid, ``false`` otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn ze_internal_advanced_publisher_check(
    this_: &ze_owned_advanced_publisher_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Borrows advanced publisher.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_advanced_publisher_loan(
    this_: &ze_owned_advanced_publisher_t,
) -> &ze_loaned_advanced_publisher_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Mutably advanced borrows publisher.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_advanced_publisher_loan_mut(
    this: &mut ze_owned_advanced_publisher_t,
) -> &mut ze_loaned_advanced_publisher_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// Options passed to the `ze_advanced_publisher_put()` function.
#[repr(C)]
pub struct ze_advanced_publisher_put_options_t {
    /// Base put options.
    put_options: z_publisher_put_options_t,
}

/// Constructs the default value for `ze_advanced_publisher_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_advanced_publisher_put_options_default(
    this: &mut MaybeUninit<ze_advanced_publisher_put_options_t>,
) {
    this.write(ze_advanced_publisher_put_options_t {
        put_options: z_publisher_put_options_t::default(),
    });
}

/// Sends a `PUT` message onto the advanced publisher's key expression, transfering the payload ownership.
///
///
/// The payload and all owned options fields are consumed upon function return.
///
/// @param this_: The advanced publisher.
/// @param payload: The data to publish. Will be consumed.
/// @param options: The advanced publisher put options. All owned fields will be consumed.
///
/// @return 0 in case of success, negative error values in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_advanced_publisher_put(
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
            tracing::error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// Represents the set of options that can be applied to the delete operation by a previously declared advanced publisher,
/// whenever issued via `ze_advanced_publisher_delete()`.
#[repr(C)]
pub struct ze_advanced_publisher_delete_options_t {
    /// Base delete options.
    pub delete_options: z_publisher_delete_options_t,
}

/// Constructs the default values for the delete operation via an advanced publisher entity.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_publisher_delete_options_default(
    this: &mut MaybeUninit<ze_advanced_publisher_delete_options_t>,
) {
    this.write(ze_advanced_publisher_delete_options_t {
        delete_options: z_publisher_delete_options_t::default(),
    });
}

/// Sends a `DELETE` message onto the advanced publisher's key expression.
///
/// @return 0 in case of success, negative error code in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_advanced_publisher_delete(
    publisher: &ze_loaned_advanced_publisher_t,
    options: Option<&mut ze_advanced_publisher_delete_options_t>,
) -> result::z_result_t {
    let publisher = publisher.as_rust_type_ref();
    let mut del = publisher.delete();
    if let Some(options) = options {
        del = _apply_pubisher_delete_options(del, &mut options.delete_options)
    }
    if let Err(e) = del.wait() {
        tracing::error!("{}", e);
        result::Z_EGENERIC
    } else {
        result::Z_OK
    }
}
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the ID of the advanced publisher.
#[no_mangle]
pub extern "C" fn ze_advanced_publisher_id(
    publisher: &ze_loaned_advanced_publisher_t,
) -> z_entity_global_id_t {
    publisher.as_rust_type_ref().id().into_c_type()
}

/// Returns the key expression of the publisher.
#[no_mangle]
pub extern "C" fn ze_advanced_publisher_keyexpr(
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
    callback: &mut zc_moved_closure_matching_status_t,
) -> zenoh::matching::MatchingListenerBuilder<'a, Callback<MatchingStatus>> {
    let publisher = publisher.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let listener = publisher
        .matching_listener()
        .callback_mut(move |matching_status| {
            let status = zc_matching_status_t {
                matching: matching_status.matching(),
            };
            zc_closure_matching_status_call(zc_closure_matching_status_loan(&callback), &status);
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
#[no_mangle]
pub extern "C" fn ze_advanced_publisher_declare_matching_listener(
    publisher: &'static ze_loaned_advanced_publisher_t,
    matching_listener: &mut MaybeUninit<zc_owned_matching_listener_t>,
    callback: &mut zc_moved_closure_matching_status_t,
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
            tracing::error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Declares a matching listener, registering a callback for notifying subscribers matching with a given advanced publisher.
/// The callback will be run in the background until the corresponding publisher is dropped.
///
/// @param publisher: A publisher to associate with matching listener.
/// @param callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber disconnects or when the first subscriber connects).
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_publisher_declare_background_matching_listener(
    publisher: &'static ze_loaned_advanced_publisher_t,
    callback: &mut zc_moved_closure_matching_status_t,
) -> result::z_result_t {
    let listener = _advanced_publisher_matching_listener_declare_inner(publisher, callback);
    match listener.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            tracing::error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Gets advanced publisher matching status - i.e. if there are any subscribers matching its key expression.
///
/// @return 0 in case of success, negative error code otherwise (in this case matching_status is not updated).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_advanced_publisher_get_matching_status(
    this: &'static ze_loaned_advanced_publisher_t,
    matching_status: &mut MaybeUninit<zc_matching_status_t>,
) -> result::z_result_t {
    match this.as_rust_type_ref().matching_status().wait() {
        Ok(s) => {
            matching_status.write(zc_matching_status_t {
                matching: s.matching(),
            });
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("{}", e);
            result::Z_ENETWORK
        }
    }
}

/// Frees memory and resets advanced_publisher to its gravestone state.
/// This is equivalent to calling `z_undeclare_publisher()` and discarding its return value.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_advanced_publisher_drop(this: &mut ze_moved_advanced_publisher_t) {
    std::mem::drop(this.take_rust_type())
}

#[no_mangle]
/// @brief Undeclares the given advanced publisher.
///
/// @return 0 in case of success, negative error code otherwise.
pub extern "C" fn ze_undeclare_advanced_publisher(
    this_: &mut ze_moved_advanced_publisher_t,
) -> result::z_result_t {
    if let Some(p) = this_.take_rust_type() {
        if let Err(e) = p.undeclare().wait() {
            tracing::error!("{}", e);
            return result::Z_ENETWORK;
        }
    }
    result::Z_OK
}
