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

use std::mem::MaybeUninit;

use crate::errors;
use crate::keyexpr::*;
use crate::transmute::unwrap_ref_unchecked;
use crate::transmute::Inplace;
use crate::transmute::TransmuteFromHandle;
use crate::transmute::TransmuteIntoHandle;
use crate::transmute::TransmuteRef;
use crate::transmute::TransmuteUninitPtr;
use crate::z_closure_sample_call;
use crate::z_owned_closure_sample_t;
use crate::z_session_t;
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::SessionDeclarations;
use zenoh::subscriber::Reliability;
use zenoh::subscriber::Subscriber;

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

pub use crate::opaque_types::z_owned_subscriber_t;
pub use crate::opaque_types::z_subscriber_t;

decl_transmute_owned!(Option<Subscriber<'static, ()>>, z_owned_subscriber_t);
decl_transmute_handle!(Subscriber<'static, ()>, z_subscriber_t);

/// Constructs a null safe-to-drop value of 'z_owned_subscriber_t' type
#[no_mangle]
pub extern "C" fn z_subscriber_null(this: *mut MaybeUninit<z_owned_subscriber_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Returns a :c:type:`z_subscriber_t` loaned from `this`.
#[no_mangle]
pub extern "C" fn z_subscriber_loan(this: &z_owned_subscriber_t) -> z_subscriber_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
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
    z_subscriber_options_t {
        reliability: Reliability::DEFAULT.into(),
    }
}

/// Declare a subscriber for a given key expression.
///
/// Parameters:
///     session: The zenoh session.
///     key_expr: The key expression to subscribe.
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
    this: *mut MaybeUninit<z_owned_subscriber_t>,
    session: z_session_t,
    key_expr: z_keyexpr_t,
    callback: &mut z_owned_closure_sample_t,
    options: Option<&mut z_subscriber_options_t>,
) -> errors::z_error_t {
    let this = this.transmute_uninit_ptr();
    let mut closure = z_owned_closure_sample_t::empty();
    std::mem::swap(callback, &mut closure);
    let session = session.transmute_ref();
    let key_expr = key_expr.transmute_ref();
    let mut subscriber = session
        .declare_subscriber(key_expr)
        .callback(move |sample| {
            let sample = sample.transmute_handle();
            z_closure_sample_call(&closure, sample)
        });
    if let Some(options) = options {
        subscriber = subscriber.reliability(options.reliability.into());
    }
    match subscriber.res() {
        Ok(sub) => {
            Inplace::init(this, Some(sub));
            errors::Z_OK
        }
        Err(e) => {
            log::error!("{}", e);
            Inplace::empty(this);
            errors::Z_EGENERIC
        }
    }
}

/// Returns the key expression of the subscriber.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_subscriber_keyexpr(subscriber: z_subscriber_t) -> z_keyexpr_t {
    let subscriber = subscriber.transmute_ref();
    subscriber.key_expr().transmute_handle()
}

/// Undeclares the given :c:type:`z_owned_subscriber_t`, droping it and invalidating it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_undeclare_subscriber(
    subscriber: &mut z_owned_subscriber_t,
) -> errors::z_error_t {
    if let Some(s) = subscriber.transmute_mut().extract().take() {
        if let Err(e) = s.undeclare().res_sync() {
            log::error!("{}", e);
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}

/// Returns ``true`` if `sub` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_subscriber_check(subscriber: &z_owned_subscriber_t) -> bool {
    subscriber.transmute_ref().is_some()
}
