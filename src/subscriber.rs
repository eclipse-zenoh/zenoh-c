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

use crate::commons::*;
use crate::impl_guarded_transmute;
use crate::keyexpr::*;
use crate::session::*;
use crate::z_closure_sample_call;
use crate::z_owned_closure_sample_t;
use crate::GuardedTransmute;
use crate::LOG_INVALID_SESSION;
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::SessionDeclarations;
use zenoh::prelude::SplitBuffer;
use zenoh::subscriber::Reliability;
use zenoh_protocol::core::SubInfo;
use zenoh_util::core::zresult::ErrNo;

/// The subscription reliability.
///
///     - **Z_RELIABILITY_BEST_EFFORT**
///     - **Z_RELIABILITY_RELIABLE**
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_reliability_t {
    BEST_EFFORT,
    RELIABLE,
}

impl From<Reliability> for z_reliability_t {
    #[inline]
    fn from(r: Reliability) -> Self {
        match r {
            Reliability::BestEffort => z_reliability_t::BEST_EFFORT,
            Reliability::Reliable => z_reliability_t::RELIABLE,
        }
    }
}

impl From<z_reliability_t> for Reliability {
    #[inline]
    fn from(val: z_reliability_t) -> Self {
        match val {
            z_reliability_t::BEST_EFFORT => Reliability::BestEffort,
            z_reliability_t::RELIABLE => Reliability::Reliable,
        }
    }
}

/**************************************/
/*            DECLARATION             */
/**************************************/
type Subscriber = Option<Box<zenoh::subscriber::Subscriber<'static, ()>>>;

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
#[cfg(not(target_arch = "arm"))]
#[repr(C, align(8))]
pub struct z_owned_subscriber_t([u64; 1]);

#[cfg(target_arch = "arm")]
#[repr(C, align(4))]
pub struct z_owned_subscriber_t([u32; 1]);

impl_guarded_transmute!(Subscriber, z_owned_subscriber_t);

impl From<Subscriber> for z_owned_subscriber_t {
    fn from(sub: Subscriber) -> Self {
        sub.transmute()
    }
}

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

impl z_owned_subscriber_t {
    pub fn new(sub: zenoh::subscriber::Subscriber<'static, ()>) -> Self {
        Some(Box::new(sub)).into()
    }
    pub fn null() -> Self {
        None.into()
    }
}

/// Constructs a null safe-to-drop value of 'z_owned_subscriber_t' type
#[no_mangle]
pub extern "C" fn z_subscriber_null() -> z_owned_subscriber_t {
    z_owned_subscriber_t::null()
}

/// A loaned zenoh subscriber.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct z_subscriber_t(*const z_owned_subscriber_t);

impl AsRef<Subscriber> for z_subscriber_t {
    fn as_ref(&self) -> &Subscriber {
        unsafe { (*self.0).as_ref() }
    }
}

/// Returns a :c:type:`z_subscriber_t` loaned from `p`.
#[no_mangle]
pub extern "C" fn z_subscriber_loan(p: &z_owned_subscriber_t) -> z_subscriber_t {
    z_subscriber_t(p)
}

/// Options passed to the :c:func:`z_declare_subscriber` or :c:func:`z_declare_pull_subscriber` function.
///
/// Members:
///     z_reliability_t reliability: The subscription reliability.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_subscriber_options_t {
    pub reliability: z_reliability_t,
}

/// Constructs the default value for :c:type:`z_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn z_subscriber_options_default() -> z_subscriber_options_t {
    let info = SubInfo::default();
    z_subscriber_options_t {
        reliability: info.reliability.into(),
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
///
///    .. code-block:: C
///
///       z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(expr), callback, NULL);
///
///    is equivalent to initializing and passing the default subscriber options:
///
///    .. code-block:: C
///
///       z_subscriber_options_t opts = z_subscriber_options_default();
///       z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_declare_subscriber(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    callback: &mut z_owned_closure_sample_t,
    opts: Option<&z_subscriber_options_t>,
) -> z_owned_subscriber_t {
    let mut closure = z_owned_closure_sample_t::empty();
    std::mem::swap(callback, &mut closure);

    match session.upgrade() {
        Some(s) => {
            let mut res = s.declare_subscriber(keyexpr).callback(move |sample| {
                let payload = sample.payload.contiguous();
                let owner = match payload {
                    std::borrow::Cow::Owned(v) => zenoh::buffers::ZBuf::from(v),
                    _ => sample.payload.clone(),
                };
                let sample = z_sample_t::new(&sample, &owner);
                z_closure_sample_call(&closure, &sample)
            });
            if let Some(opts) = opts {
                res = res.reliability(opts.reliability.into())
            }
            match res.res() {
                Ok(sub) => z_owned_subscriber_t::new(sub),
                Err(e) => {
                    log::debug!("{}", e);
                    z_owned_subscriber_t::null()
                }
            }
        }
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            z_owned_subscriber_t::null()
        }
    }
}

/// Returns the key expression of the subscriber.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_subscriber_keyexpr(subscriber: z_subscriber_t) -> z_owned_keyexpr_t {
    if let Some(p) = subscriber.as_ref() {
        p.key_expr().clone().into()
    } else {
        z_keyexpr_t::null().into()
    }
}

/// Undeclares the given :c:type:`z_owned_subscriber_t`, droping it and invalidating it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_undeclare_subscriber(sub: &mut z_owned_subscriber_t) -> i8 {
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
pub extern "C" fn z_subscriber_check(sub: &z_owned_subscriber_t) -> bool {
    sub.as_ref().is_some()
}
