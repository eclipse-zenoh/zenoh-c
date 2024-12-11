//
// Copyright (c) 2023 ZettaScale Technology.
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
#![allow(deprecated)] 

use std::mem::MaybeUninit;

use zenoh::{handlers::Callback, sample::Sample, session::Session, Wait};
use zenoh_ext::*;

use crate::{
    opaque_types::{ze_loaned_querying_subscriber_t, ze_owned_querying_subscriber_t},
    result,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_sample_call, z_closure_sample_loan, z_get_options_t, z_loaned_keyexpr_t,
    z_loaned_session_t, z_moved_closure_sample_t, z_query_consolidation_none,
    z_query_consolidation_t, z_query_target_default, z_query_target_t,
};
#[cfg(feature = "unstable")]
use crate::{
    zc_locality_default, zc_locality_t, zc_reply_keyexpr_default, zc_reply_keyexpr_t,
    ze_moved_querying_subscriber_t,
};
decl_c_type!(
    owned(
        ze_owned_querying_subscriber_t,
        option(zenoh_ext::FetchingSubscriber<()>, &'static Session),
    ),
    loaned(ze_loaned_querying_subscriber_t),
);

/// Constructs a querying subscriber in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_internal_querying_subscriber_null(
    this: &mut MaybeUninit<ze_owned_querying_subscriber_t>,
) {
    this.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A set of options that can be applied to a querying subscriber,
/// upon its declaration via `ze_declare_querying_subscriber()`.
///
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ze_querying_subscriber_options_t {
    /// The restriction for the matching publications that will be receive by this subscriber.
    #[cfg(feature = "unstable")]
    allowed_origin: zc_locality_t,
    /// The selector to be used for queries.
    query_selector: Option<&'static z_loaned_keyexpr_t>,
    /// The target to be used for queries.
    query_target: z_query_target_t,
    /// The consolidation mode to be used for queries.
    query_consolidation: z_query_consolidation_t,
    #[cfg(feature = "unstable")]
    /// The accepted replies for queries.
    query_accept_replies: zc_reply_keyexpr_t,
    /// The timeout to be used for queries.
    query_timeout_ms: u64,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs the default value for `ze_querying_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_options_default(
    this: &mut MaybeUninit<ze_querying_subscriber_options_t>,
) {
    this.write(ze_querying_subscriber_options_t {
        #[cfg(feature = "unstable")]
        allowed_origin: zc_locality_default(),
        query_selector: None,
        query_target: z_query_target_default(),
        query_consolidation: z_query_consolidation_none(),
        #[cfg(feature = "unstable")]
        query_accept_replies: zc_reply_keyexpr_default(),
        query_timeout_ms: 0,
    });
}

unsafe fn _declare_querying_subscriber_inner<'a, 'b>(
    session: &'a z_loaned_session_t,
    key_expr: &'b z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    options: Option<&mut ze_querying_subscriber_options_t>,
) -> QueryingSubscriberBuilder<'a, 'b, UserSpace, Callback<Sample>> {
    let session = session.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let mut sub = session
        .declare_subscriber(key_expr.as_rust_type_ref())
        .querying();
    if let Some(options) = options {
        sub = sub
            .query_target(options.query_target.into())
            .query_consolidation(options.query_consolidation);
        #[cfg(feature = "unstable")]
        {
            sub = sub
                .query_accept_replies(options.query_accept_replies.into())
                .allowed_origin(options.allowed_origin.into());
        }
        if let Some(query_selector) = options.query_selector {
            let query_selector = query_selector.as_rust_type_ref().clone();
            sub = sub.query_selector(query_selector);
        }
        if options.query_timeout_ms != 0 {
            sub = sub.query_timeout(std::time::Duration::from_millis(options.query_timeout_ms));
        }
    }
    let sub = sub.callback(move |sample| {
        let mut owned_sample = Some(sample);
        z_closure_sample_call(
            z_closure_sample_loan(&callback),
            owned_sample
                .as_mut()
                .unwrap_unchecked()
                .as_loaned_c_type_mut(),
        );
    });
    sub
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs and declares a querying subscriber for a given key expression.
///
/// @param session: A Zenoh session.
/// @param querying_subscriber: An uninitialized memory location where querying subscriber will be constructed.
/// @param key_expr: A key expression to subscribe to.
/// @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
/// @param options: Additional options for the querying subscriber.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_declare_querying_subscriber(
    session: &'static z_loaned_session_t,
    querying_subscriber: &mut MaybeUninit<ze_owned_querying_subscriber_t>,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    options: Option<&mut ze_querying_subscriber_options_t>,
) -> result::z_result_t {
    let this = querying_subscriber.as_rust_type_mut_uninit();
    let sub = _declare_querying_subscriber_inner(session, key_expr, callback, options);
    match sub.wait() {
        Ok(sub) => {
            let session = session.as_rust_type_ref();
            this.write(Some((sub, session)));
            result::Z_OK
        }
        Err(e) => {
            tracing::debug!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Declares a background querying subscriber for a given key expression. Subscriber callback will be called to process the messages,
/// until the corresponding session is closed or dropped.
///
/// @param session: A Zenoh session.
/// @param key_expr: A key expression to subscribe to.
/// @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
/// @param options: Additional options for the querying subscriber.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_declare_background_querying_subscriber(
    session: &'static z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    options: Option<&mut ze_querying_subscriber_options_t>,
) -> result::z_result_t {
    let sub = _declare_querying_subscriber_inner(session, key_expr, callback, options);
    match sub.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            tracing::debug!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make querying subscriber perform an additional query on a specified selector.
/// The queried samples will be merged with the received publications and made available in the subscriber callback.
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn ze_querying_subscriber_get(
    this: &ze_loaned_querying_subscriber_t,
    selector: &z_loaned_keyexpr_t,
    options: Option<&mut z_get_options_t>,
) -> result::z_result_t {
    unsafe impl Sync for z_get_options_t {}
    let sub = this.as_rust_type_ref();
    let session = sub.1;
    let selector = selector.as_rust_type_ref().clone();
    if let Err(e) = sub
        .0
        .fetch({
            move |cb| {
                let mut get = session.get(selector);

                if let Some(options) = options {
                    if let Some(payload) = options.payload.take() {
                        get = get.payload(payload.take_rust_type());
                    }
                    if let Some(encoding) = options.encoding.take() {
                        get = get.encoding(encoding.take_rust_type());
                    }
                    if let Some(attachment) = options.attachment.take() {
                        get = get.attachment(attachment.take_rust_type());
                    }

                    get = get
                        .consolidation(options.consolidation)
                        .target(options.target.into())
                        .congestion_control(options.congestion_control.into())
                        .priority(options.priority.into())
                        .express(options.is_express);

                    #[cfg(feature = "unstable")]
                    {
                        if let Some(source_info) = options.source_info.take() {
                            get = get.source_info(source_info.take_rust_type());
                        }
                        get = get
                            .allowed_destination(options.allowed_destination.into())
                            .accept_replies(options.accept_replies.into());
                    }

                    if options.timeout_ms != 0 {
                        get = get.timeout(std::time::Duration::from_millis(options.timeout_ms));
                    }
                }

                get.callback(cb).wait()
            }
        })
        .wait()
    {
        tracing::debug!("{}", e);
        return result::Z_EGENERIC;
    }
    result::Z_OK
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Undeclares querying subscriber callback and resets it to its gravestone state.
/// This is equivalent to calling `ze_undeclare_querying_subscriber()` and discarding its return value.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_drop(this_: &mut ze_moved_querying_subscriber_t) {
    std::mem::drop(this_.take_rust_type())
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if querying subscriber is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn ze_internal_querying_subscriber_check(
    this_: &ze_owned_querying_subscriber_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows querying subscriber.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_querying_subscriber_loan(
    this: &ze_owned_querying_subscriber_t,
) -> &ze_loaned_querying_subscriber_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Undeclares the given querying subscriber.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_undeclare_querying_subscriber(
    this_: &mut ze_moved_querying_subscriber_t,
) -> result::z_result_t {
    if let Some(s) = this_.take_rust_type() {
        if let Err(e) = s.0.undeclare().wait() {
            tracing::error!("{}", e);
            return result::Z_EGENERIC;
        }
    }
    result::Z_OK
}
