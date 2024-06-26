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
use crate::transmute2::LoanedCTypeRef;
use crate::transmute2::RustTypeRef;
use crate::z_closure_sample_call;
use crate::z_closure_sample_loan;
use crate::z_loaned_session_t;
use crate::z_owned_closure_sample_t;
use zenoh::core::Wait;
use zenoh::prelude::SessionDeclarations;
use zenoh::subscriber::Reliability;
use zenoh::subscriber::Subscriber;

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

pub use crate::opaque_types::z_loaned_subscriber_t;
pub use crate::opaque_types::z_owned_subscriber_t;
decl_transmute_owned!(Option<Subscriber<'static, ()>>, z_owned_subscriber_t);
decl_transmute_handle!(Subscriber<'static, ()>, z_loaned_subscriber_t);

validate_equivalence!(z_owned_subscriber_t, z_loaned_subscriber_t);

/// Constructs a subscriber in a gravestone state.
#[no_mangle]
pub extern "C" fn z_subscriber_null(this: *mut MaybeUninit<z_owned_subscriber_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Borrows subscriber.
#[no_mangle]
pub extern "C" fn z_subscriber_loan(this: &z_owned_subscriber_t) -> &z_loaned_subscriber_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
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
pub extern "C" fn z_subscriber_options_default(this: &mut z_subscriber_options_t) {
    *this = z_subscriber_options_t {
        reliability: Reliability::DEFAULT.into(),
    }
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
    this: *mut MaybeUninit<z_owned_subscriber_t>,
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_owned_closure_sample_t,
    options: Option<&mut z_subscriber_options_t>,
) -> errors::z_error_t {
    let this = this.transmute_uninit_ptr();
    let mut closure = z_owned_closure_sample_t::empty();
    std::mem::swap(callback, &mut closure);
    let session = session.transmute_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let mut subscriber = session
        .declare_subscriber(key_expr)
        .callback(move |sample| {
            let sample = sample.as_loaned_ctype_ref();
            z_closure_sample_call(z_closure_sample_loan(&closure), sample)
        });
    if let Some(options) = options {
        subscriber = subscriber.reliability(options.reliability.into());
    }
    match subscriber.wait() {
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
pub extern "C" fn z_subscriber_keyexpr(subscriber: &z_loaned_subscriber_t) -> &z_loaned_keyexpr_t {
    let subscriber = subscriber.transmute_ref();
    subscriber.key_expr().as_loaned_ctype_ref()
}

/// Undeclares subscriber and drops subscriber.
///
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_undeclare_subscriber(this: &mut z_owned_subscriber_t) -> errors::z_error_t {
    if let Some(s) = this.transmute_mut().extract().take() {
        if let Err(e) = s.undeclare().wait() {
            log::error!("{}", e);
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}

/// Drops subscriber and resets it to its gravestone state. Also attempts to undeclare it.
#[no_mangle]
pub extern "C" fn z_subscriber_drop(this: &mut z_owned_subscriber_t) {
    z_undeclare_subscriber(this);
}

/// Returns ``true`` if subscriber is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_subscriber_check(this: &z_owned_subscriber_t) -> bool {
    this.transmute_ref().is_some()
}
