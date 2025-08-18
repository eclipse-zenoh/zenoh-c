//
// Copyright (c) 2024 ZettaScale Technology.
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
use prebindgen_proc_macro::prebindgen;

use std::{mem::MaybeUninit, time::Duration};

use zenoh::{handlers::Callback, liveliness::LivelinessSubscriberBuilder, sample::Sample, Wait};
use zenoh_ext::{AdvancedSubscriberBuilderExt, HistoryConfig, RecoveryConfig, SampleMissListener};

use crate::{
    _declare_subscriber_inner, result,
    transmute::{IntoCType, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_sample_call, z_closure_sample_loan, z_entity_global_id_t,
    z_liveliness_subscriber_options_t, z_loaned_keyexpr_t, z_loaned_session_t,
    z_moved_closure_sample_t, z_owned_subscriber_t, z_subscriber_options_t, ze_closure_miss_call,
    ze_closure_miss_loan, ze_loaned_advanced_subscriber_t, ze_moved_advanced_subscriber_t,
    ze_moved_closure_miss_t, ze_moved_sample_miss_listener_t, ze_owned_advanced_subscriber_t,
    ze_owned_sample_miss_listener_t,
};

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Settings for retrievieng historical data for Advanced Subscriber.
#[prebindgen]
#[repr(C)]
pub struct ze_advanced_subscriber_history_options_t {
    /// Must be set to ``true``, to enable the history data recovery.
    pub is_enabled: bool,
    /// Enable detection of late joiner publishers and query for their historical data.
    /// Late joiner detection can only be achieved for Publishers that enable publisher_detection.
    /// History can only be retransmitted by Publishers that enable caching.
    pub detect_late_publishers: bool,
    /// Number of samples to query for each resource. ``0`` corresponds to no limit on number of samples.
    pub max_samples: usize,
    /// Maximum age of samples to query. ``0`` corresponds to no limit on samples' age.
    pub max_age_ms: u64,
}

impl Default for ze_advanced_subscriber_history_options_t {
    fn default() -> Self {
        Self {
            is_enabled: true,
            detect_late_publishers: false,
            max_samples: 0,
            max_age_ms: 0,
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs the default value for `ze_advanced_subscriber_history_options_t`.
#[prebindgen]
pub fn ze_advanced_subscriber_history_options_default(
    this: &mut MaybeUninit<ze_advanced_subscriber_history_options_t>,
) {
    this.write(ze_advanced_subscriber_history_options_t::default());
}

impl From<&ze_advanced_subscriber_history_options_t> for HistoryConfig {
    fn from(val: &ze_advanced_subscriber_history_options_t) -> Self {
        let mut h = HistoryConfig::default();
        if val.detect_late_publishers {
            h = h.detect_late_publishers();
        }
        if val.max_samples > 0 {
            h = h.max_samples(val.max_samples)
        }
        if val.max_age_ms > 0 {
            h = h.max_age(val.max_age_ms as f64 / 1000.0f64)
        }
        h
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Settings for detection of the last sample(s) miss by Advanced Subscriber.
#[prebindgen]
#[repr(C)]
pub struct ze_advanced_subscriber_last_sample_miss_detection_options_t {
    /// Must be set to ``true``, to enable the last sample(s) miss detection.
    pub is_enabled: bool,
    /// Period for queries for not yet received Samples.
    ///
    /// These queries allow to retrieve the last Sample(s) if the last Sample(s) is/are lost.
    /// So it is useful for sporadic publications but useless for periodic publications
    /// with a period smaller or equal to this period. If set to 0, the last sample(s) miss detection will be performed
    /// based on publisher's heartbeat if the latter is enabled.
    pub periodic_queries_period_ms: u64,
}

impl Default for ze_advanced_subscriber_last_sample_miss_detection_options_t {
    fn default() -> Self {
        Self {
            is_enabled: true,
            periodic_queries_period_ms: 0,
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs the default value for `ze_advanced_subscriber_last_sample_miss_detection_options_t`.
#[prebindgen]
pub fn ze_advanced_subscriber_last_sample_miss_detection_options_default(
    this: &mut MaybeUninit<ze_advanced_subscriber_last_sample_miss_detection_options_t>,
) {
    this.write(ze_advanced_subscriber_last_sample_miss_detection_options_t::default());
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Settings for recovering lost messages for Advanced Subscriber.
#[prebindgen]
#[repr(C)]
pub struct ze_advanced_subscriber_recovery_options_t {
    /// Must be set to ``true``, to enable the lost sample recovery.
    pub is_enabled: bool,
    /// Setting for detecting last sample(s) miss.
    /// Note that it does not affect intermediate sample miss detection/retrieval (which is performed automatically as long as recovery is enabled).
    /// If this option is disabled, subscriber will be unable to detect/request retransmission of missed sample until it receives a more recent one from the same publisher.
    pub last_sample_miss_detection: ze_advanced_subscriber_last_sample_miss_detection_options_t,
}

impl Default for ze_advanced_subscriber_recovery_options_t {
    fn default() -> Self {
        Self {
            is_enabled: true,
            last_sample_miss_detection:
                ze_advanced_subscriber_last_sample_miss_detection_options_t {
                    is_enabled: false,
                    ..Default::default()
                },
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs the default value for `ze_advanced_subscriber_recovery_options_t`.
#[prebindgen]
pub fn ze_advanced_subscriber_recovery_options_default(
    this: &mut MaybeUninit<ze_advanced_subscriber_recovery_options_t>,
) {
    this.write(ze_advanced_subscriber_recovery_options_t::default());
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Options passed to the `ze_declare_advanced_subscriber()` function.
#[prebindgen]
#[repr(C)]
pub struct ze_advanced_subscriber_options_t {
    /// Base subscriber options.
    pub subscriber_options: z_subscriber_options_t,
    /// Settings for querying historical data. History can only be retransmitted by Publishers that enable caching.
    pub history: ze_advanced_subscriber_history_options_t,
    /// Settings for retransmission of detected lost Samples. Retransmission of lost samples can only be done by Publishers that enable
    /// caching and sample_miss_detection.
    pub recovery: ze_advanced_subscriber_recovery_options_t,
    /// Timeout to be used for history and recovery queries.
    /// Default value will be used if set to ``0``.
    pub query_timeout_ms: u64,
    /// Allow this subscriber to be detected through liveliness.
    pub subscriber_detection: bool,
    /// An optional key expression to be added to the liveliness token key expression.
    /// It can be used to convey meta data.
    pub subscriber_detection_metadata: Option<&'static z_loaned_keyexpr_t>,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs the default value for `ze_advanced_subscriber_options_t`.
#[prebindgen]
pub fn ze_advanced_subscriber_options_default(
    this: &mut MaybeUninit<ze_advanced_subscriber_options_t>,
) {
    let history = ze_advanced_subscriber_history_options_t {
        is_enabled: false,
        ..Default::default()
    };
    let recovery = ze_advanced_subscriber_recovery_options_t {
        is_enabled: false,
        ..Default::default()
    };
    this.write(ze_advanced_subscriber_options_t {
        subscriber_options: z_subscriber_options_t::default(),
        history,
        recovery,
        query_timeout_ms: 0,
        subscriber_detection: false,
        subscriber_detection_metadata: None,
    });
}

fn _declare_advanced_subscriber_inner(
    session: &'static z_loaned_session_t,
    key_expr: &'static z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    mut options: Option<&'static mut ze_advanced_subscriber_options_t>,
) -> zenoh_ext::AdvancedSubscriberBuilder<'static, 'static, 'static, Callback<Sample>> {
    let sub = _declare_subscriber_inner(
        session,
        key_expr,
        callback,
        options.as_mut().map(|o| &mut o.subscriber_options),
    );
    let mut sub = sub.advanced();
    if let Some(options) = options {
        if options.query_timeout_ms > 0 {
            sub = sub.query_timeout(Duration::from_millis(options.query_timeout_ms));
        }
        if options.subscriber_detection {
            sub = sub.subscriber_detection()
        }
        if let Some(sub_detection_metadata) = &options.subscriber_detection_metadata {
            sub = sub.subscriber_detection_metadata(sub_detection_metadata.as_rust_type_ref());
        }
        if options.history.is_enabled {
            sub = sub.history((&options.history).into());
        }
        if options.recovery.is_enabled {
            if options.recovery.last_sample_miss_detection.is_enabled {
                sub = match options
                    .recovery
                    .last_sample_miss_detection
                    .periodic_queries_period_ms
                {
                    0 => sub.recovery(RecoveryConfig::default().heartbeat()),
                    p => sub.recovery(
                        RecoveryConfig::default().periodic_queries(Duration::from_millis(p)),
                    ),
                };
            } else {
                sub = sub.recovery(RecoveryConfig::default())
            }
        }
    }
    sub
}

decl_c_type!(
    owned(ze_owned_advanced_subscriber_t, option zenoh_ext::AdvancedSubscriber<()>),
    loaned(ze_loaned_advanced_subscriber_t),
);

/// Constructs a subscriber in a gravestone state.
#[prebindgen]
pub fn ze_internal_advanced_subscriber_null(
    this_: &mut MaybeUninit<ze_owned_advanced_subscriber_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Borrows subscriber.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn ze_advanced_subscriber_loan(
    this_: &ze_owned_advanced_subscriber_t,
) -> &ze_loaned_advanced_subscriber_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Moves subscriber.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn ze_advanced_subscriber_move(
    this_: &mut ze_owned_advanced_subscriber_t,
) -> &mut ze_moved_advanced_subscriber_t {
    std::mem::transmute(this_)
}

/// Undeclares advanced subscriber callback and resets it to its gravestone state.
/// This is equivalent to calling `ze_undeclare_advanced_subscriber()` and discarding its return value.
#[prebindgen]
pub fn ze_advanced_subscriber_drop(this_: &mut ze_moved_advanced_subscriber_t) {
    std::mem::drop(this_.take_rust_type())
}

/// Returns ``true`` if advanced subscriber is valid, ``false`` otherwise.
#[prebindgen]
pub fn ze_internal_advanced_subscriber_check(
    this_: &ze_owned_advanced_subscriber_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Constructs and declares an advanced subscriber for a given key expression. Dropping subscriber undeclares its callback.
///
/// @param session: The zenoh session.
/// @param subscriber: An uninitialized location in memory, where advanced subscriber will be constructed.
/// @param key_expr: The key expression to subscribe.
/// @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
/// @param options: The options to be passed to the subscriber declaration.
///
/// @return 0 in case of success, negative error code otherwise (in this case subscriber will be in its gravestone state).
#[prebindgen]
pub fn ze_declare_advanced_subscriber(
    session: &'static z_loaned_session_t,
    subscriber: &'static mut MaybeUninit<ze_owned_advanced_subscriber_t>,
    key_expr: &'static z_loaned_keyexpr_t,
    callback: &'static mut z_moved_closure_sample_t,
    options: Option<&'static mut ze_advanced_subscriber_options_t>,
) -> result::z_result_t {
    let this = subscriber.as_rust_type_mut_uninit();
    let s = _declare_advanced_subscriber_inner(session, key_expr, callback, options);
    match s.wait() {
        Ok(sub) => {
            this.write(Some(sub));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
    }
}

/// Constructs and declares a background advanced subscriber. Subscriber callback will be called to process the messages,
/// until the corresponding session is closed or dropped.
///
/// @param session: The zenoh session.
/// @param key_expr: The key expression to subscribe.
/// @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
/// @param options: The options to be passed to the subscriber declaration.
///
/// @return 0 in case of success, negative error code otherwise.
#[prebindgen]
pub fn ze_declare_background_advanced_subscriber(
    session: &'static z_loaned_session_t,
    key_expr: &'static z_loaned_keyexpr_t,
    callback: &'static mut z_moved_closure_sample_t,
    options: Option<&'static mut ze_advanced_subscriber_options_t>,
) -> result::z_result_t {
    let subscriber = _declare_advanced_subscriber_inner(session, key_expr, callback, options);
    match subscriber.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Undeclares the advanced subscriber.
///
/// @return 0 in case of success, negative error code otherwise.
#[prebindgen]
pub fn ze_undeclare_advanced_subscriber(
    this_: &mut ze_moved_advanced_subscriber_t,
) -> result::z_result_t {
    if let Some(s) = this_.take_rust_type() {
        if let Err(e) = s.undeclare().wait() {
            crate::report_error!("{}", e);
            return result::Z_EGENERIC;
        }
    }
    result::Z_OK
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A struct that represents missed samples.
#[prebindgen]
#[repr(C)]
pub struct ze_miss_t {
    /// The source of missed samples.
    pub source: z_entity_global_id_t,
    /// The number of missed samples.
    pub nb: u32,
}

decl_c_type!(
    owned(ze_owned_sample_miss_listener_t, option SampleMissListener<()>),
);

#[prebindgen]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs an empty sample miss listener.
pub fn ze_internal_sample_miss_listener_null(
    this_: &mut MaybeUninit<ze_owned_sample_miss_listener_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

#[prebindgen]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Checks the sample_miss listener is for the gravestone state
pub fn ze_internal_sample_miss_listener_check(
    this_: &ze_owned_sample_miss_listener_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Undeclares the given sample miss listener, droping and invalidating it.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_sample_miss_listener_drop(this: &mut ze_moved_sample_miss_listener_t) {
    std::mem::drop(this.take_rust_type())
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Undeclares the given sample miss listener, droping and invalidating it.
/// @return 0 in case of success, negative error code otherwise.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_undeclare_sample_miss_listener(
    this: &mut ze_moved_sample_miss_listener_t,
) -> result::z_result_t {
    if let Some(m) = this.take_rust_type() {
        if let Err(e) = m.undeclare().wait() {
            crate::report_error!("{}", e);
            return result::Z_ENETWORK;
        }
    }
    result::Z_OK
}

fn _advanced_subscriber_sample_miss_listener_declare_inner<'a>(
    subscriber: &'a ze_loaned_advanced_subscriber_t,
    callback: &mut ze_moved_closure_miss_t,
) -> zenoh_ext::SampleMissListenerBuilder<'a, Callback<zenoh_ext::Miss>> {
    let subscriber = subscriber.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let listener = subscriber.sample_miss_listener().callback_mut(move |miss| {
        let miss = ze_miss_t {
            source: miss.source().into_c_type(),
            nb: miss.nb(),
        };
        ze_closure_miss_call(ze_closure_miss_loan(&callback), &miss);
    });
    listener
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs sample miss listener, registering a callback for notifying subscriber about missed samples.
///
/// @param subscriber: A subscriber to associate with sample miss listener.
/// @param sample_miss_listener: An uninitialized memory location where sample miss listener will be constructed. The sample miss listener's callback will be automatically dropped when the subscriber is dropped.
/// @param callback: A closure that will be called every time the sample miss is detected.
///
/// @return 0 in case of success, negative error code otherwise.
#[prebindgen]
pub fn ze_advanced_subscriber_declare_sample_miss_listener(
    subscriber: &'static ze_loaned_advanced_subscriber_t,
    sample_miss_listener: &mut MaybeUninit<ze_owned_sample_miss_listener_t>,
    callback: &mut ze_moved_closure_miss_t,
) -> result::z_result_t {
    let this = sample_miss_listener.as_rust_type_mut_uninit();
    let listener = _advanced_subscriber_sample_miss_listener_declare_inner(subscriber, callback);
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

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Declares a sample miss listener, registering a callback for notifying subscriber about missed samples.
/// The callback will be run in the background until the corresponding subscriber is dropped.
///
/// @param subscriber: A subscriber to associate with sample miss listener.
/// @param callback: A closure that will be called every time the sample miss is detected.
///
/// @return 0 in case of success, negative error code otherwise.
#[prebindgen]
pub fn ze_advanced_subscriber_declare_background_sample_miss_listener(
    subscriber: &'static ze_loaned_advanced_subscriber_t,
    callback: &mut ze_moved_closure_miss_t,
) -> result::z_result_t {
    let listener = _advanced_subscriber_sample_miss_listener_declare_inner(subscriber, callback);
    match listener.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

fn _advanced_subscriber_detect_publishers_inner(
    subscriber: &'static ze_loaned_advanced_subscriber_t,
    callback: &'static mut z_moved_closure_sample_t,
    options: Option<&'static mut z_liveliness_subscriber_options_t>,
) -> LivelinessSubscriberBuilder<'static, 'static, Callback<Sample>> {
    let subscriber = subscriber.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let sub = subscriber
        .detect_publishers()
        .history(options.is_some_and(|o| o.history))
        .callback(move |sample| {
            let mut owned_sample = Some(sample);
            z_closure_sample_call(z_closure_sample_loan(&callback), unsafe {
                owned_sample
                    .as_mut()
                    .unwrap_unchecked()
                    .as_loaned_c_type_mut()
            })
        });
    sub
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Declares a liveliness token listener for matching publishers detection. Only advanced publishers, enabling publisher detection can be detected.
///
/// @param subscriber: The advanced subscriber instance.
/// @param liveliness_subscriber: An uninitialized memory location where liveliness subscriber will be constructed.
/// @param callback: The callback function that will be called each time a liveliness token status is changed.
/// @param options: The options to be passed to the liveliness subscriber declaration.
///
/// @return 0 in case of success, negative error values otherwise.
#[prebindgen]
pub fn ze_advanced_subscriber_detect_publishers(
    subscriber: &'static ze_loaned_advanced_subscriber_t,
    liveliness_subscriber: &mut MaybeUninit<z_owned_subscriber_t>,
    callback: &'static mut z_moved_closure_sample_t,
    options: Option<&'static mut z_liveliness_subscriber_options_t>,
) -> result::z_result_t {
    let liveliness_subscriber = liveliness_subscriber.as_rust_type_mut_uninit();
    let builder = _advanced_subscriber_detect_publishers_inner(subscriber, callback, options);
    match builder.wait() {
        Ok(s) => {
            liveliness_subscriber.write(Some(s));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to subscribe to liveliness: {e}");
            liveliness_subscriber.write(None);
            result::Z_EGENERIC
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Declares a background subscriber on liveliness tokens of matching publishers. Subscriber callback will be called to process the messages,
/// until the corresponding session is closed or dropped. Only advanced publishers. enabling publisher detection can be detected.
/// @param subscriber: The advanced subscriber instance.
/// @param callback: The callback function that will be called each time a liveliness token status is changed.
/// @param options: The options to be passed to the liveliness subscriber declaration.
///
/// @return 0 in case of success, negative error values otherwise.
#[prebindgen]
pub fn ze_advanced_subscriber_detect_publishers_background(
    subscriber: &'static ze_loaned_advanced_subscriber_t,
    callback: &'static mut z_moved_closure_sample_t,
    options: Option<&'static mut z_liveliness_subscriber_options_t>,
) -> result::z_result_t {
    let builder = _advanced_subscriber_detect_publishers_inner(subscriber, callback, options);
    match builder.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            crate::report_error!("Failed to subscribe to liveliness: {e}");
            result::Z_EGENERIC
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Returns the key expression of the advanced subscriber.
#[prebindgen]
pub fn ze_advanced_subscriber_keyexpr(
    subscriber: &ze_loaned_advanced_subscriber_t,
) -> &z_loaned_keyexpr_t {
    subscriber
        .as_rust_type_ref()
        .key_expr()
        .as_loaned_c_type_ref()
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the ID of the advanced subscriber.
#[prebindgen]
pub fn ze_advanced_subscriber_id(
    subscriber: &ze_loaned_advanced_subscriber_t,
) -> z_entity_global_id_t {
    subscriber.as_rust_type_ref().id().into_c_type()
}
