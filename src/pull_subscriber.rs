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
use crate::collections::*;
use crate::commons::*;
use crate::keyexpr::*;
use crate::session::*;
use crate::z_closure_sample_call;
use crate::z_owned_closure_sample_t;
use crate::z_reliability_t;
use crate::LOG_INVALID_SESSION;
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::SplitBuffer;
use zenoh::subscriber::Reliability;
use zenoh_protocol_core::SubInfo;
use zenoh_util::core::zresult::ErrNo;

/**************************************/
/*            DECLARATION             */
/**************************************/
type PullSubscriber = Option<Box<zenoh::subscriber::PullSubscriber<'static, ()>>>;

/// An owned zenoh pull subscriber. Destroying the subscriber cancels the subscription.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_owned_pull_subscriber_t([usize; 1]);
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_pull_subscriber_t<'a>(&'a z_owned_pull_subscriber_t);

impl From<PullSubscriber> for z_owned_pull_subscriber_t {
    fn from(val: PullSubscriber) -> Self {
        unsafe { std::mem::transmute(val) }
    }
}

impl AsRef<PullSubscriber> for z_owned_pull_subscriber_t {
    fn as_ref(&self) -> &PullSubscriber {
        unsafe { std::mem::transmute(self) }
    }
}

impl AsMut<PullSubscriber> for z_owned_pull_subscriber_t {
    fn as_mut(&mut self) -> &mut PullSubscriber {
        unsafe { std::mem::transmute(self) }
    }
}

impl z_owned_pull_subscriber_t {
    pub fn new(sub: zenoh::subscriber::PullSubscriber<'static, ()>) -> Self {
        Some(Box::new(sub)).into()
    }
    pub fn null() -> Self {
        None.into()
    }
}

/// Constructs a null safe-to-drop value of 'z_owned_pull_subscriber_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_pull_subscriber_null() -> z_owned_pull_subscriber_t {
    z_owned_pull_subscriber_t::null()
}

/// Represents the set of options that can be applied to a pull subscriber,
/// upon its declaration via :c:func:`z_declare_pull_subscriber`.
///
/// Members:
///   z_reliability_t reliability: The subscription reliability.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_pull_subscriber_options_t {
    reliability: z_reliability_t,
}

/// Constructs the default value for :c:type:`z_pull_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn z_pull_subscriber_options_default() -> z_pull_subscriber_options_t {
    let info = SubInfo::default();
    z_pull_subscriber_options_t {
        reliability: info.reliability.into(),
    }
}

/// Declares a pull subscriber for a given key expression.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression to subscribe.
///     callback: The callback function that will be called each time a data matching the subscribed expression is received.
///     opts: additional options for the pull subscriber.
///
/// Returns:
///    A :c:type:`z_owned_subscriber_t`.
///
///    To check if the subscription succeeded and if the pull subscriber is still valid,
///    you may use `z_pull_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
///
///    Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
///    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
///    After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// Example:
///    Declaring a subscriber passing ``NULL`` for the options:
///
///    .. code-block:: C
///
///       z_owned_subscriber_t sub = z_declare_pull_subscriber(z_loan(s), z_keyexpr(expr), callback, NULL);
///
///    is equivalent to initializing and passing the default subscriber options:
///
///    .. code-block:: C
///
///       z_subscriber_options_t opts = z_subscriber_options_default();
///       z_owned_subscriber_t sub = z_declare_pull_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
///
///    Passing custom arguments to the **callback** can be done by defining a custom structure:
///
///    .. code-block:: C
///
///       typedef struct {
///         z_keyexpr_t forward;
///         z_session_t session;
///       } myargs_t;
///   
///       void callback(const z_sample_t sample, const void *arg)
///       {
///         myargs_t *myargs = (myargs_t *)arg;
///         z_put(myargs->session, myargs->forward, sample->value, NULL);
///       }
///
///       int main() {
///         myargs_t cargs = {
///           forward = z_keyexpr("forward"),
///           session = s,
///         };
///         z_pull_subscriber_options_t opts = z_pull_subscriber_options_default();
///         opts.cargs = (void *)&cargs;
///         z_owned_pull_subscriber_t sub = z_declare_pull_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
///       }
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_declare_pull_subscriber(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    callback: &mut z_owned_closure_sample_t,
    mut opts: *const z_pull_subscriber_options_t,
) -> z_owned_pull_subscriber_t {
    let mut closure = z_owned_closure_sample_t::empty();
    std::mem::swap(callback, &mut closure);

    let session: &'static z_owned_session_t = session.into();
    match session.as_ref() {
        Some(s) => {
            if opts.is_null() {
                let default = z_pull_subscriber_options_default();
                opts = &default;
            }
            let reliability: Reliability = (*opts).reliability.into();
            let res = s
                .declare_subscriber(keyexpr)
                .callback(move |sample| {
                    let payload = sample.payload.contiguous();
                    let bytes = z_bytes_t {
                        start: payload.as_ptr(),
                        len: payload.len(),
                    };
                    let sample = z_sample_t {
                        keyexpr: (&sample.key_expr).into(),
                        payload: bytes,
                        encoding: (&sample.encoding).into(),
                        kind: sample.kind.into(),
                        timestamp: sample.timestamp.as_ref().into(),
                    };
                    z_closure_sample_call(&closure, &sample)
                })
                .reliability(reliability)
                .pull_mode()
                .res();
            match res {
                Ok(sub) => z_owned_pull_subscriber_t::new(sub),
                Err(e) => {
                    log::debug!("{}", e);
                    z_owned_pull_subscriber_t::null() 
                }
            }
        }
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            z_owned_pull_subscriber_t::null() 
        }
    }
}

/// Undeclares the given :c:type:`z_owned_pull_subscriber_t`, droping it and invalidating it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_undeclare_pull_subscriber(sub: &mut z_owned_pull_subscriber_t) -> i8 {
    if let Some(s) = sub.as_mut().take() {
        if let Err(e) = s.undeclare().res_sync() {
            log::warn!("{}", e);
            return e.errno().get();
        }
    }
    0
}

/// Returns ``true`` if `sub` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_pull_subscriber_check(sub: &z_owned_pull_subscriber_t) -> bool {
    sub.as_ref().is_some()
}

/// Returns ``true`` if `sub` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_pull_subscriber_loan(
    sub: &z_owned_pull_subscriber_t,
) -> z_pull_subscriber_t {
    z_pull_subscriber_t(sub)
}

/// Pull data for :c:type:`z_owned_pull_subscriber_t`. The pulled data will be provided
/// by calling the **callback** function provided to the :c:func:`z_declare_subscriber` function.
///
/// Parameters:
///     sub: The :c:type:`z_owned_pull_subscriber_t` to pull from.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_subscriber_pull(sub: z_pull_subscriber_t) -> i8 {
    match sub.0.as_ref() {
        Some(tx) => {
            if let Err(e) = tx.pull().res_sync() {
                log::error!("{}", e);
                e.errno().get()
            } else {
                0
            }
        }
        None => i8::MIN,
    }
}
