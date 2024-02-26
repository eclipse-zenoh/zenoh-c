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

use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::KeyExpr;
use zenoh::prelude::SessionDeclarations;
use zenoh::prelude::SplitBuffer;
use zenoh_ext::*;
use zenoh_protocol::core::SubInfo;
use zenoh_util::core::zresult::ErrNo;

use crate::{
    impl_guarded_transmute, z_closure_sample_call, z_get_options_t, z_keyexpr_t,
    z_owned_closure_sample_t, z_query_consolidation_none, z_query_consolidation_t,
    z_query_target_default, z_query_target_t, z_reliability_t, z_sample_t, z_session_t,
    zcu_locality_default, zcu_locality_t, zcu_reply_keyexpr_default, zcu_reply_keyexpr_t,
    LOG_INVALID_SESSION,
};

pub struct FetchingSubscriberWrapper {
    fetching_subscriber: zenoh_ext::FetchingSubscriber<'static, ()>,
    session: z_session_t,
}
type FetchingSubscriber = Option<Box<FetchingSubscriberWrapper>>;
//type FetchingSubscriber = Option<Box<zenoh_ext::FetchingSubscriber<'static, ()>>>;

/// An owned zenoh querying subscriber. Destroying the subscriber cancels the subscription.
///
/// Like most `ze_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `ze_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
pub struct ze_owned_querying_subscriber_t([usize; 1]);

impl_guarded_transmute!(FetchingSubscriber, ze_owned_querying_subscriber_t);

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ze_querying_subscriber_t<'a>(&'a ze_owned_querying_subscriber_t);

impl<'a> AsRef<FetchingSubscriber> for ze_querying_subscriber_t<'a> {
    fn as_ref(&self) -> &FetchingSubscriber {
        self.0
    }
}

impl ze_owned_querying_subscriber_t {
    pub fn new(sub: zenoh_ext::FetchingSubscriber<'static, ()>, session: z_session_t) -> Self {
        Some(Box::new(FetchingSubscriberWrapper {
            fetching_subscriber: sub,
            session,
        }))
        .into()
    }
    pub fn null() -> Self {
        None.into()
    }
}

/// Constructs a null safe-to-drop value of 'ze_owned_querying_subscriber_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_querying_subscriber_null() -> ze_owned_querying_subscriber_t {
    ze_owned_querying_subscriber_t::null()
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
    query_selector: z_keyexpr_t,
    query_target: z_query_target_t,
    query_consolidation: z_query_consolidation_t,
    query_accept_replies: zcu_reply_keyexpr_t,
    query_timeout_ms: u64,
}

/// Constructs the default value for :c:type:`ze_querying_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_options_default() -> ze_querying_subscriber_options_t {
    ze_querying_subscriber_options_t {
        reliability: SubInfo::default().reliability.into(),
        allowed_origin: zcu_locality_default(),
        query_selector: z_keyexpr_t::null(),
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
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    callback: &mut z_owned_closure_sample_t,
    options: Option<&ze_querying_subscriber_options_t>,
) -> ze_owned_querying_subscriber_t {
    let mut closure = z_owned_closure_sample_t::empty();
    std::mem::swap(callback, &mut closure);

    match session.upgrade() {
        Some(s) => {
            let mut sub = s.declare_subscriber(keyexpr).querying();
            if let Some(options) = options {
                sub = sub
                    .reliability(options.reliability.into())
                    .allowed_origin(options.allowed_origin.into())
                    .query_target(options.query_target.into())
                    .query_consolidation(options.query_consolidation)
                    .query_accept_replies(options.query_accept_replies.into());
                if options.query_selector.is_some() {
                    let query_selector = options
                        .query_selector
                        .as_ref()
                        .map(|s| s.clone().into_owned());
                    if let Some(query_selector) = query_selector {
                        sub = sub.query_selector(query_selector)
                    }
                }
                if options.query_timeout_ms != 0 {
                    sub = sub
                        .query_timeout(std::time::Duration::from_millis(options.query_timeout_ms));
                }
            }
            match sub
                .callback(move |mut sample| {
                    if let std::borrow::Cow::Owned(v) = sample.payload.contiguous() {
                        sample.payload = v.into();
                    }
                    let sample = z_sample_t::new(&sample);
                    z_closure_sample_call(&closure, &sample)
                })
                .res()
            {
                Ok(sub) => ze_owned_querying_subscriber_t::new(sub, session),
                Err(e) => {
                    log::debug!("{}", e);
                    ze_owned_querying_subscriber_t::null()
                }
            }
        }
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            ze_owned_querying_subscriber_t::null()
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
) -> i8 {
    unsafe impl Sync for z_get_options_t {}

    if let Some(sub) = sub.as_ref() {
        match sub.session.upgrade() {
            Some(s) => {
                if let Err(e) = sub
                    .fetching_subscriber
                    .fetch({
                        let selector = KeyExpr::try_from(selector).unwrap();
                        move |cb| match options {
                            Some(options) => s
                                .get(selector)
                                .target(options.target.into())
                                .consolidation(options.consolidation)
                                .timeout(std::time::Duration::from_millis(options.timeout_ms))
                                .callback(cb)
                                .res_sync(),
                            None => s.get(selector).callback(cb).res_sync(),
                        }
                    })
                    .res()
                {
                    log::debug!("{}", e);
                    return -1;
                }
            }
            None => {
                log::debug!("{}", LOG_INVALID_SESSION);
                return -1;
            }
        }
    }
    0
}

/// Undeclares the given :c:type:`ze_owned_querying_subscriber_t`, droping it and invalidating it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn ze_undeclare_querying_subscriber(sub: &mut ze_owned_querying_subscriber_t) -> i8 {
    if let Some(s) = sub.take() {
        if let Err(e) = s.fetching_subscriber.close().res_sync() {
            log::warn!("{}", e);
            return e.errno().get();
        }
    }
    0
}

/// Returns ``true`` if `sub` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_check(sub: &ze_owned_querying_subscriber_t) -> bool {
    sub.as_ref().is_some()
}

/// Returns a :c:type:`ze_querying_subscriber_loan` loaned from `p`.
#[no_mangle]
pub extern "C" fn ze_querying_subscriber_loan(
    p: &ze_owned_querying_subscriber_t,
) -> ze_querying_subscriber_t {
    ze_querying_subscriber_t(p)
}
