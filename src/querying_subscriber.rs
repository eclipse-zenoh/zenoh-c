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
use crate::z_keyexpr_t;
use crate::z_owned_closure_sample_t;
use crate::z_reliability_t;
use crate::{
    z_closure_sample_call, z_get_options_t, z_query_consolidation_none, z_query_consolidation_t,
    z_query_target_default, z_query_target_t, z_session_t, zcu_locality_default, zcu_locality_t,
    zcu_reply_keyexpr_default, zcu_reply_keyexpr_t,
};
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::SessionDeclarations;
use zenoh::session::Session;
use zenoh::subscriber::Reliability;
use zenoh_ext::*;

use crate::opaque_types::ze_owned_querying_subscriber_t;
use crate::opaque_types::ze_querying_subscriber_t;
decl_transmute_owned!(
    Option<(zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session)>,
    ze_owned_querying_subscriber_t
);
decl_transmute_handle!(
    (zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session),
    ze_querying_subscriber_t
);

/// Constructs a null safe-to-drop value of 'ze_owned_querying_subscriber_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_querying_subscriber_null(
    this: *mut MaybeUninit<ze_owned_querying_subscriber_t>,
) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Represents the set of options that can be applied to a querying subscriber,
/// upon its declaration via :c:func:`ze_declare_querying_subscriber`.
///
/// Members:
///   z_reliability_t reliability: The subscription reliability.
///   zcu_locality_t allowed_origin: The restriction for the matching publications that will be
///                                  receive by this subscriber.
///   z_keyexpr_t query_selector: The selector to be used for queries.
///   z_query_target_t query_target: The target to be used for queries.
///   z_query_consolidation_t query_consolidation: The consolidation mode to be used for queries.
///   zcu_reply_keyexpr_t query_accept_replies: The accepted replies for queries.
///   uint64_t query_timeout_ms: The timeout to be used for queries.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ze_querying_subscriber_options_t {
    reliability: z_reliability_t,
    allowed_origin: zcu_locality_t,
    query_selector: *const z_keyexpr_t,
    query_target: z_query_target_t,
    query_consolidation: z_query_consolidation_t,
    query_accept_replies: zcu_reply_keyexpr_t,
    query_timeout_ms: u64,
}

/// Constructs the default value for :c:type:`ze_querying_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_options_default() -> ze_querying_subscriber_options_t {
    ze_querying_subscriber_options_t {
        reliability: Reliability::DEFAULT.into(),
        allowed_origin: zcu_locality_default(),
        query_selector: null(),
        query_target: z_query_target_default(),
        query_consolidation: z_query_consolidation_none(),
        query_accept_replies: zcu_reply_keyexpr_default(),
        query_timeout_ms: 0,
    }
}

/// Declares a Querying Subscriber for a given key expression.
///
/// Parameters:
///     z_session_t session: The zenoh session.
///     z_keyexpr_t keyexpr: The key expression to subscribe.
///     z_owned_closure_sample_t callback: The callback function that will be called each time a data matching the subscribed expression is received.
///     ze_querying_subscriber_options_t options: Additional options for the querying subscriber.
///
/// Returns:
///    :c:type:`ze_owned_subscriber_t`.
///
///    To check if the subscription succeeded and if the querying subscriber is still valid,
///    you may use `ze_querying_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
///
///    Like all `ze_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
///    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
///    After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// Example:
///    Declaring a subscriber passing ``NULL`` for the options:
///
///    .. code-block:: C
///
///       ze_owned_subscriber_t sub = ze_declare_querying_subscriber(z_loan(s), z_keyexpr(expr), callback, NULL);
///
///    is equivalent to initializing and passing the default subscriber options:
///
///    .. code-block:: C
///
///       z_subscriber_options_t opts = z_subscriber_options_default();
///       ze_owned_subscriber_t sub = ze_declare_querying_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_declare_querying_subscriber(
    this: *mut MaybeUninit<ze_owned_querying_subscriber_t>,
    session: z_session_t,
    key_expr: z_keyexpr_t,
    callback: &mut z_owned_closure_sample_t,
    options: ze_querying_subscriber_options_t,
) -> errors::ZCError {
    let this = this.transmute_uninit_ptr();
    let mut closure = z_owned_closure_sample_t::empty();
    std::mem::swap(callback, &mut closure);
    let session = session.transmute_ref();
    let mut sub = session
        .declare_subscriber(key_expr.transmute_ref())
        .querying();
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

/// Make a :c:type:`ze_owned_querying_subscriber_t` to perform an additional query on a specified selector.
/// The queried samples will be merged with the received publications and made available in the subscriber callback.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn ze_querying_subscriber_get(
    sub: ze_querying_subscriber_t,
    selector: z_keyexpr_t,
    options: Option<&z_get_options_t>,
) -> errors::ZCError {
    unsafe impl Sync for z_get_options_t {}
    let sub = sub.transmute_ref();
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

/// Undeclares the given :c:type:`ze_owned_querying_subscriber_t`, droping it and invalidating it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn ze_undeclare_querying_subscriber(
    this: &mut ze_owned_querying_subscriber_t,
) -> errors::ZCError {
    if let Some(s) = this.transmute_mut().extract().take() {
        if let Err(e) = s.0.close().res_sync() {
            log::error!("{}", e);
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}

/// Returns ``true`` if `this` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_check(this: &ze_owned_querying_subscriber_t) -> bool {
    this.transmute_ref().is_some()
}

/// Returns a :c:type:`ze_querying_subscriber_loan` loaned from `this`.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_loan(
    this: &ze_owned_querying_subscriber_t,
) -> ze_querying_subscriber_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}
