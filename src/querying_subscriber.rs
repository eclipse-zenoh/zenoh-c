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

use std::mem::MaybeUninit;

use zenoh::{
    bytes::EncodingBuilderTrait, prelude::SessionDeclarations, pubsub::Reliability,
    sample::SampleBuilderTrait, session::Session, Wait,
};
use zenoh_ext::*;

use crate::{
    opaque_types::{ze_loaned_querying_subscriber_t, ze_owned_querying_subscriber_t},
    result,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_closure_sample_call, z_closure_sample_loan, z_get_options_t, z_loaned_keyexpr_t,
    z_loaned_session_t, z_moved_closure_sample_t, z_query_consolidation_none,
    z_query_consolidation_t, z_query_target_default, z_query_target_t, z_reliability_t,
};
#[cfg(feature = "unstable")]
use crate::{
    zc_locality_default, zc_locality_t, zc_reply_keyexpr_default, zc_reply_keyexpr_t,
    ze_moved_querying_subscriber_t,
};
decl_c_type!(
    owned(
        ze_owned_querying_subscriber_t,
        option(zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session),
    ),
    loaned(ze_loaned_querying_subscriber_t),
    moved(ze_moved_querying_subscriber_t)
);

/// Constructs a querying subscriber in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_querying_subscriber_null(
    this: &mut MaybeUninit<ze_owned_querying_subscriber_t>,
) {
    this.as_rust_type_mut_uninit().write(None);
}

/// A set of options that can be applied to a querying subscriber,
/// upon its declaration via `ze_declare_querying_subscriber()`.
///
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ze_querying_subscriber_options_t {
    /// The subscription reliability.
    reliability: z_reliability_t,
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

/// Constructs the default value for `ze_querying_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_options_default(
    this: &mut MaybeUninit<ze_querying_subscriber_options_t>,
) {
    this.write(ze_querying_subscriber_options_t {
        reliability: Reliability::DEFAULT.into(),
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

/// Constructs and declares a querying subscriber for a given key expression.
///
/// @param this_: An uninitialized memory location where querying subscriber will be constructed.
/// @param session: A Zenoh session.
/// @param key_expr: A key expression to subscribe to.
/// @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
/// @param options: Additional options for the querying subscriber.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_declare_querying_subscriber(
    this: &mut MaybeUninit<ze_owned_querying_subscriber_t>,
    session: &'static z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: z_moved_closure_sample_t,
    options: Option<&mut ze_querying_subscriber_options_t>,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let session = session.as_rust_type_ref();
    let Some(callback) = callback.into_rust_type() else {
        this.write(None);
        return result::Z_EINVAL;
    };
    let mut sub = session
        .declare_subscriber(key_expr.as_rust_type_ref())
        .querying();
    if let Some(options) = options {
        sub = sub
            .reliability(options.reliability.into())
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
        let sample = sample.as_loaned_c_type_ref();
        z_closure_sample_call(z_closure_sample_loan(&callback), sample);
    });
    match sub.wait() {
        Ok(sub) => {
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

/// Make querying subscriber perform an additional query on a specified selector.
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
                let mut get = session.get(selector).callback(cb);

                if let Some(options) = options {
                    if let Some(payload) = options.payload.take_rust_type() {
                        get = get.payload(payload);
                    }
                    if let Some(encoding) = options.encoding.take_rust_type() {
                        get = get.encoding(encoding);
                    }
                    #[cfg(feature = "unstable")]
                    if let Some(source_info) = options.source_info.take_rust_type() {
                        get = get.source_info(source_info);
                    }
                    if let Some(attachment) = options.attachment.take_rust_type() {
                        get = get.attachment(attachment);
                    }

                    get = get
                        .consolidation(options.consolidation)
                        .target(options.target.into());

                    if options.timeout_ms != 0 {
                        get = get.timeout(std::time::Duration::from_millis(options.timeout_ms));
                    }
                }

                get.wait()
            }
        })
        .wait()
    {
        tracing::debug!("{}", e);
        return result::Z_EGENERIC;
    }
    result::Z_OK
}

/// Undeclares the given querying subscriber, drops it and resets to a gravestone state.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_undeclare_querying_subscriber(
    this: ze_moved_querying_subscriber_t,
) -> result::z_result_t {
    if let Some(s) = this.into_rust_type() {
        if let Err(e) = s.0.close().wait() {
            tracing::error!("{}", e);
            return result::Z_EGENERIC;
        }
    }
    result::Z_OK
}

/// Drops querying subscriber. Also attempts to undeclare it.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_drop(this: ze_moved_querying_subscriber_t) {
    ze_undeclare_querying_subscriber(this);
}

/// Returns ``true`` if querying subscriber is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_check(this: &ze_owned_querying_subscriber_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Borrows querying subscriber.
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
