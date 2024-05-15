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
use crate::transmute::unwrap_ref_unchecked;
use crate::transmute::Inplace;
use crate::transmute::TransmuteFromHandle;
use crate::transmute::TransmuteIntoHandle;
use crate::transmute::TransmuteRef;
use crate::transmute::TransmuteUninitPtr;
use crate::z_loaned_keyexpr_t;
use crate::z_owned_closure_sample_t;
use crate::z_reliability_t;
use crate::{
    z_closure_sample_call, z_get_options_t, z_loaned_session_t, z_query_consolidation_none,
    z_query_consolidation_t, z_query_target_default, z_query_target_t, zcu_locality_default,
    zcu_locality_t, zcu_reply_keyexpr_default, zcu_reply_keyexpr_t,
};
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::SessionDeclarations;
use zenoh::session::Session;
use zenoh::subscriber::Reliability;
use zenoh_ext::*;

use crate::opaque_types::ze_loaned_querying_subscriber_t;
use crate::opaque_types::ze_owned_querying_subscriber_t;
decl_transmute_owned!(
    Option<(zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session)>,
    ze_owned_querying_subscriber_t
);
decl_transmute_handle!(
    (zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session),
    ze_loaned_querying_subscriber_t
);

validate_equivalence!(ze_owned_querying_subscriber_t, ze_loaned_querying_subscriber_t);

/// Constructs a querying subscriber in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_querying_subscriber_null(
    this: *mut MaybeUninit<ze_owned_querying_subscriber_t>,
) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
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
    this: *mut MaybeUninit<ze_owned_querying_subscriber_t>,
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_owned_closure_sample_t,
    options: Option<&mut ze_querying_subscriber_options_t>,
) -> errors::z_error_t {
    let this = this.transmute_uninit_ptr();
    let mut closure = z_owned_closure_sample_t::empty();
    std::mem::swap(callback, &mut closure);
    let session = session.transmute_ref();
    let mut sub = session
        .declare_subscriber(key_expr.transmute_ref())
        .querying();
    if let Some(options) = options {
        sub = sub
            .reliability(options.reliability.into())
            .allowed_origin(options.allowed_origin.into())
            .query_target(options.query_target.into())
            .query_consolidation(options.query_consolidation)
            .query_accept_replies(options.query_accept_replies.into());
        if !options.query_selector.is_null() {
            let query_selector = unsafe { *options.query_selector }.transmute_ref().clone();
            sub = sub.query_selector(query_selector)
        }
        if options.query_timeout_ms != 0 {
            sub = sub.query_timeout(std::time::Duration::from_millis(options.query_timeout_ms));
        }
    }
    let sub = sub.callback(move |sample| {
        let sample = sample.transmute_handle();
        z_closure_sample_call(&closure, sample);
    });
    match sub.res() {
        Ok(sub) => {
            Inplace::init(this, Some((sub, session)));
            errors::Z_OK
        }
        Err(e) => {
            log::debug!("{}", e);
            Inplace::empty(this);
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
    let sub = this.transmute_ref();
    let session = sub.1;
    let selector = selector.transmute_ref().clone();
    if let Err(e) = sub
        .0
        .fetch({
            move |cb| match options {
                Some(options) => session
                    .get(selector)
                    .target(options.target.into())
                    .consolidation(options.consolidation)
                    .timeout(std::time::Duration::from_millis(options.timeout_ms))
                    .callback(cb)
                    .res_sync(),
                None => session.get(selector).callback(cb).res_sync(),
            }
        })
        .res()
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
    if let Some(s) = this.transmute_mut().extract().take() {
        if let Err(e) = s.0.close().res_sync() {
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
    this.transmute_ref().is_some()
}

/// Borrows querying subscriber.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_loan(
    this: &ze_owned_querying_subscriber_t,
) -> &ze_loaned_querying_subscriber_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}
