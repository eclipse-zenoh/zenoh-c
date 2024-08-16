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

use zenoh::{
    prelude::SessionDeclarations,
    pubsub::{Reliability, Subscriber},
    Wait,
};

use crate::{
    keyexpr::*,
    result,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_sample_call, z_closure_sample_loan, z_loaned_session_t, z_moved_closure_sample_t,
};

/// The subscription reliability.
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_reliability_t {
    /// Defines reliability as ``BEST_EFFORT``
    BEST_EFFORT,
    /// Defines reliability as ``RELIABLE``
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

pub use crate::opaque_types::{z_loaned_subscriber_t, z_moved_subscriber_t, z_owned_subscriber_t};
decl_c_type!(
    owned(z_owned_subscriber_t, option Subscriber<'static, ()>),
    loaned(z_loaned_subscriber_t),
);

/// Constructs a subscriber in a gravestone state.
#[no_mangle]
pub extern "C" fn z_subscriber_null(this: &mut MaybeUninit<z_owned_subscriber_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Borrows subscriber.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_subscriber_loan(this: &z_owned_subscriber_t) -> &z_loaned_subscriber_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Options passed to the `z_declare_subscriber()` function.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_subscriber_options_t {
    /// The subscription reliability.
    pub reliability: z_reliability_t,
}

/// Constructs the default value for `z_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn z_subscriber_options_default(this: &mut MaybeUninit<z_subscriber_options_t>) {
    this.write(z_subscriber_options_t {
        reliability: Reliability::DEFAULT.into(),
    });
}

/// Constructs and declares a subscriber for a given key expression. Dropping subscriber
///
/// @param this_: An uninitialized location in memory, where subscriber will be constructed.
/// @param session: The zenoh session.
/// @param key_expr: The key expression to subscribe.
/// @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
/// @param options: The options to be passed to the subscriber declaration.
///
/// @return 0 in case of success, negative error code otherwise (in this case subscriber will be in its gravestone state).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_declare_subscriber(
    this: &mut MaybeUninit<z_owned_subscriber_t>,
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    options: Option<&mut z_subscriber_options_t>,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let mut subscriber = session
        .declare_subscriber(key_expr)
        .callback(move |sample| {
            let sample = sample.as_loaned_c_type_ref();
            z_closure_sample_call(z_closure_sample_loan(&callback), sample)
        });
    if let Some(options) = options {
        subscriber = subscriber.reliability(options.reliability.into());
    }
    match subscriber.wait() {
        Ok(sub) => {
            this.write(Some(sub));
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
    }
}

/// Returns the key expression of the subscriber.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_subscriber_keyexpr(subscriber: &z_loaned_subscriber_t) -> &z_loaned_keyexpr_t {
    subscriber
        .as_rust_type_ref()
        .key_expr()
        .as_loaned_c_type_ref()
}

/// Undeclares subscriber and drops subscriber.
///
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_undeclare_subscriber(_this: &mut z_moved_subscriber_t) -> result::z_result_t {
    if let Some(s) = _this.take_rust_type() {
        if let Err(e) = s.undeclare().wait() {
            tracing::error!("{}", e);
            return result::Z_EGENERIC;
        }
    }
    result::Z_OK
}

/// Drops subscriber and resets it to its gravestone state. Also attempts to undeclare it.
#[no_mangle]
pub extern "C" fn z_subscriber_drop(this_: &mut z_moved_subscriber_t) {
    let _ = this_.take_rust_type();
}

/// Returns ``true`` if subscriber is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_subscriber_check(this: &z_owned_subscriber_t) -> bool {
    this.as_rust_type_ref().is_some()
}
