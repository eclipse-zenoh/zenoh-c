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
use std::ptr::null;

use crate::errors;
use crate::transmute::LoanedCTypeRef;
use crate::transmute::RustTypeRef;
use crate::transmute::RustTypeRefUninit;
use crate::z_closure_sample_loan;
use crate::z_loaned_keyexpr_t;
use crate::z_owned_closure_sample_t;
use crate::z_reliability_t;
use crate::{
    z_closure_sample_call, z_get_options_t, z_loaned_session_t, z_query_consolidation_none,
    z_query_consolidation_t, z_query_target_default, z_query_target_t, zcu_reply_keyexpr_default,
    zcu_reply_keyexpr_t,
};
#[cfg(feature = "unstable")]
use crate::{zcu_locality_default, zcu_locality_t};
use zenoh::core::Wait;
use zenoh::prelude::SessionDeclarations;
use zenoh::sample::EncodingBuilderTrait;
use zenoh::sample::SampleBuilderTrait;
use zenoh::session::Session;
use zenoh::subscriber::Reliability;
use zenoh_ext::*;

use crate::opaque_types::ze_loaned_querying_subscriber_t;
use crate::opaque_types::ze_owned_querying_subscriber_t;
decl_c_type!(
    owned(
        ze_owned_querying_subscriber_t,
        Option<(zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session)>,
    ),
    loaned(
        ze_loaned_querying_subscriber_t,
        (zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session),
    )
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
    allowed_origin: zcu_locality_t,
    /// The selector to be used for queries.
    query_selector: *const z_loaned_keyexpr_t,
    /// The target to be used for queries.
    query_target: z_query_target_t,
    /// The consolidation mode to be used for queries.
    query_consolidation: z_query_consolidation_t,
    /// The accepted replies for queries.
    query_accept_replies: zcu_reply_keyexpr_t,
    /// The timeout to be used for queries.
    query_timeout_ms: u64,
}

/// Constructs the default value for `ze_querying_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_options_default(
    this: &mut ze_querying_subscriber_options_t,
) {
    *this = ze_querying_subscriber_options_t {
        reliability: Reliability::DEFAULT.into(),
        #[cfg(feature = "unstable")]
        allowed_origin: zcu_locality_default(),
        query_selector: null(),
        query_target: z_query_target_default(),
        query_consolidation: z_query_consolidation_none(),
        query_accept_replies: zcu_reply_keyexpr_default(),
        query_timeout_ms: 0,
    };
}

/// Constructs and declares a querying subscriber for a given key expression.
///
/// @param this_: An unitialized memory location where querying subscriber will be constructed.
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
    callback: &mut z_owned_closure_sample_t,
    options: Option<&mut ze_querying_subscriber_options_t>,
) -> errors::z_error_t {
    let this = this.as_rust_type_mut_uninit();
    let mut closure = z_owned_closure_sample_t::empty();
    std::mem::swap(callback, &mut closure);
    let session = session.as_rust_type_ref();
    let mut sub = session
        .declare_subscriber(key_expr.as_rust_type_ref())
        .querying();
    if let Some(options) = options {
        sub = sub
            .reliability(options.reliability.into())
            .query_target(options.query_target.into())
            .query_consolidation(options.query_consolidation)
            .query_accept_replies(options.query_accept_replies.into());
        #[cfg(feature = "unstable")]
        {
            sub = sub.allowed_origin(options.allowed_origin.into());
        }
        if let Some(query_selector) = unsafe { options.query_selector.as_ref() } {
            let query_selector = query_selector.as_rust_type_ref().clone();
            sub = sub.query_selector(query_selector);
        }
        if options.query_timeout_ms != 0 {
            sub = sub.query_timeout(std::time::Duration::from_millis(options.query_timeout_ms));
        }
    }
    let sub = sub.callback(move |sample| {
        let sample = sample.as_loaned_c_type_ref();
        z_closure_sample_call(z_closure_sample_loan(&closure), sample);
    });
    match sub.wait() {
        Ok(sub) => {
            this.write(Some((sub, session)));
            errors::Z_OK
        }
        Err(e) => {
            log::debug!("{}", e);
            this.write(None);
            errors::Z_EGENERIC
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
    options: Option<&z_get_options_t>,
) -> errors::z_error_t {
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
                    if let Some(payload) = unsafe { options.payload.as_mut() } {
                        let payload = std::mem::take(payload.as_rust_type_mut());
                        get = get.payload(payload);
                    }
                    if let Some(encoding) = unsafe { options.encoding.as_mut() } {
                        let encoding = std::mem::take(encoding.as_rust_type_mut());
                        get = get.encoding(encoding);
                    }
                    if let Some(source_info) = unsafe { options.source_info.as_mut() } {
                        let source_info = std::mem::take(source_info.as_rust_type_mut());
                        get = get.source_info(source_info);
                    }
                    if let Some(attachment) = unsafe { options.attachment.as_mut() } {
                        let attachment = std::mem::take(attachment.as_rust_type_mut());
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
        log::debug!("{}", e);
        return errors::Z_EGENERIC;
    }
    errors::Z_OK
}

/// Undeclares the given querying subscriber, drops it and resets to a gravestone state.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_undeclare_querying_subscriber(
    this: &mut ze_owned_querying_subscriber_t,
) -> errors::z_error_t {
    if let Some(s) = this.as_rust_type_mut().take() {
        if let Err(e) = s.0.close().wait() {
            log::error!("{}", e);
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}

/// Drops querying subscriber. Also attempts to undeclare it.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_drop(this: &mut ze_owned_querying_subscriber_t) {
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
