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
use crate::{CallbackArgs, LOG_INVALID_SESSION};
use libc::c_void;
use zenoh::net::protocol::core::SubInfo;
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::SplitBuffer;
use zenoh::subscriber::Reliability;

/// The subscription reliability.
///
///     - **z_reliability_BEST_EFFORT**
///     - **z_reliability_RELIABLE**
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_reliability {
    BEST_EFFORT,
    RELIABLE,
}

impl From<Reliability> for z_reliability {
    #[inline]
    fn from(r: Reliability) -> Self {
        match r {
            Reliability::BestEffort => z_reliability::BEST_EFFORT,
            Reliability::Reliable => z_reliability::RELIABLE,
        }
    }
}

impl From<z_reliability> for Reliability {
    #[inline]
    fn from(val: z_reliability) -> Self {
        match val {
            z_reliability::BEST_EFFORT => Reliability::BestEffort,
            z_reliability::RELIABLE => Reliability::Reliable,
        }
    }
}

/**************************************/
/*            DECLARATION             */
/**************************************/
type Subscriber = Option<Box<zenoh::subscriber::CallbackSubscriber<'static>>>;

/// An owned zenoh subscriber. Destroying the subscriber cancels the subscription.
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
pub struct z_owned_subscriber_t([usize; 1]);

impl AsRef<Subscriber> for z_owned_subscriber_t {
    fn as_ref(&self) -> &Subscriber {
        unsafe { std::mem::transmute(self) }
    }
}

impl AsMut<Subscriber> for z_owned_subscriber_t {
    fn as_mut(&mut self) -> &mut Subscriber {
        unsafe { std::mem::transmute(self) }
    }
}

/// Declare a subscriber for a given key expression.
///
/// Members:
///     `z_reliability reliability`: The subscription reliability.
///     `void *cargs`: A pointer that will be passed to the **callback** at each call.
///
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_subscriber_options_t {
    pub reliability: z_reliability,
    pub cargs: *mut c_void,
}

/// Create a default subscription info.
#[no_mangle]
pub extern "C" fn z_subscriber_options_default() -> z_subscriber_options_t {
    let info = SubInfo::default();
    z_subscriber_options_t {
        reliability: info.reliability.into(),
        cargs: std::ptr::null_mut(),
    }
}

/// Declare a subscriber for a given key expression.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression to subscribe.
///     callback: The callback function that will be called each time a data matching the subscribed expression is received.
///     opts: The options to be passed to describe the options to be passed to the subscriber declaration.
///
/// Returns:
///    A :c:type:`z_owned_subscriber_t`.
///
///    To check if the subscription succeeded and if the subscriber is still valid,
///    you may use `z_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
///
///    Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
///    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
///    After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// Example:
///    Declaring a subscriber passing `NULL` for the options:
///    ```
///    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(expr), callback, NULL);
///    ```
///
///    is equivalent to initializing and passing the default subscriber options:
///    
///    ```
///    z_subscriber_options_t opts = z_subscriber_options_default();
///    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
///    ```
///
///    Passing custom arguments to the **callback** can be done by defining a custom structure:
///
///    ```
///    typedef struct {
///      z_keyexpr_t forward;
///      z_session_t session;
///    } myargs_t;
///
///    void callback(const z_sample_t sample, const void *arg)
///    {
///      myargs_t *myargs = (myargs_t *)arg;
///      z_put(myargs->session, myargs->forward, sample->value, NULL);
///    }
///
///    int main() {
///      myargs_t cargs = {
///        forward = z_keyexpr("forward"),
///        session = s,
///      };
///      z_subscriber_options_t opts = z_subscriber_options_default();
///      opts.cargs = (void *)&cargs;
///      z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
///    }
///    ```
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_declare_subscriber(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    callback: extern "C" fn(z_sample_t, *mut c_void),
    mut opts: *const z_subscriber_options_t,
) -> z_owned_subscriber_t {
    unsafe fn ok(sub: zenoh::subscriber::CallbackSubscriber<'_>) -> z_owned_subscriber_t {
        std::mem::transmute(Some(Box::new(sub)))
    }

    unsafe fn err() -> z_owned_subscriber_t {
        std::mem::transmute(None::<Box<zenoh::subscriber::CallbackSubscriber<'_>>>)
    }

    match session.as_ref() {
        Some(s) => {
            let default = z_subscriber_options_default();
            if opts.is_null() {
                opts = &default;
            }

            let cargs = CallbackArgs::from((*opts).cargs);
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
                    };
                    callback(sample, cargs.into());
                })
                .reliability(reliability)
                .mode(zenoh_protocol_core::SubMode::Push)
                .res();
            match res {
                Ok(sub) => ok(sub),
                Err(e) => {
                    log::debug!("{}", e);
                    err()
                }
            }
        }
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            err()
        }
    }
}

// Unsubscribes from the passed `sub`, droping it and invalidating it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_undeclare_subscriber(sub: &mut z_owned_subscriber_t) {
    if let Some(s) = sub.as_mut().take() {
        if let Err(e) = s.undeclare().res_sync() {
            log::warn!("{}", e)
        }
    }
}

/// Returns `true` if `sub` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_subscriber_check(sub: &z_owned_subscriber_t) -> bool {
    sub.as_ref().is_some()
}

// /// Pull data for a pull mode :c:type:`z_owned_subscriber_t`. The pulled data will be provided
// /// by calling the **callback** function provided to the :c:func:`z_subscribe` function.
// ///
// /// Parameters:
// ///     sub: The :c:type:`z_owned_subscriber_t` to pull from.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_pull(sub: &z_owned_subscriber_t) {
//     match sub.as_ref() {
//         Some(tx) => {
//             let _ = async_std::task::block_on(tx.send(SubOps::Pull));
//         }
//         None => (),
//     }
// }
