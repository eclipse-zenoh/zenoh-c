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
    handlers::Callback,
    pubsub::{Subscriber, SubscriberBuilder},
    sample::Sample,
    Wait,
};

pub use crate::opaque_types::{z_loaned_subscriber_t, z_moved_subscriber_t, z_owned_subscriber_t};
use crate::{
    keyexpr::*,
    result,
    transmute::{
        LoanedCTypeMut, LoanedCTypeRef, RustTypeMut, RustTypeMutUninit, RustTypeRef, TakeRustType,
    },
    z_closure_sample_call, z_closure_sample_loan, z_loaned_session_t, z_moved_closure_sample_t,
};
decl_c_type!(
    owned(z_owned_subscriber_t, option Subscriber<()>),
    loaned(z_loaned_subscriber_t),
);

/// Constructs a subscriber in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_subscriber_null(this_: &mut MaybeUninit<z_owned_subscriber_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Borrows subscriber.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_subscriber_loan(this_: &z_owned_subscriber_t) -> &z_loaned_subscriber_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Mutably borrows subscriber.
#[no_mangle]
pub extern "C" fn z_subscriber_loan_mut(
    this_: &mut z_owned_subscriber_t,
) -> &mut z_loaned_subscriber_t {
    this_.as_rust_type_mut().as_loaned_c_type_mut()
}

/// Takes ownership of the mutably borrowed subscriber
#[no_mangle]
pub extern "C" fn z_subscriber_take_loaned(
    dst: &mut MaybeUninit<z_owned_subscriber_t>,
    src: &mut z_loaned_subscriber_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(std::mem::take(src.as_rust_type_mut()));
}

/// Options passed to the `z_declare_subscriber()` function.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_subscriber_options_t {
    /// Dummy field to avoid having fieldless struct
    pub _0: u8,
}

/// Constructs the default value for `z_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn z_subscriber_options_default(this_: &mut MaybeUninit<z_subscriber_options_t>) {
    this_.write(z_subscriber_options_t { _0: 0 });
}

fn _declare_subscriber_inner<'a, 'b>(
    session: &'a z_loaned_session_t,
    key_expr: &'b z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    _options: Option<&mut z_subscriber_options_t>,
) -> SubscriberBuilder<'a, 'b, Callback<Sample>> {
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let subscriber = session
        .declare_subscriber(key_expr)
        .callback(move |sample| {
            let mut owned_sample = Some(sample);
            z_closure_sample_call(
                z_closure_sample_loan(&callback),
                owned_sample.as_loaned_c_type_mut(),
            )
        });
    subscriber
}

/// Constructs and declares a subscriber for a given key expression. Dropping subscriber undeclares its callback.
///
/// @param session: The zenoh session.
/// @param subscriber: An uninitialized location in memory, where subscriber will be constructed.
/// @param key_expr: The key expression to subscribe.
/// @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
/// @param _options: The options to be passed to the subscriber declaration.
///
/// @return 0 in case of success, negative error code otherwise (in this case subscriber will be in its gravestone state).
#[no_mangle]
pub extern "C" fn z_declare_subscriber(
    session: &z_loaned_session_t,
    subscriber: &mut MaybeUninit<z_owned_subscriber_t>,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    _options: Option<&mut z_subscriber_options_t>,
) -> result::z_result_t {
    let this = subscriber.as_rust_type_mut_uninit();
    let s = _declare_subscriber_inner(session, key_expr, callback, _options);
    match s.wait() {
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

/// Constructs and declares a background subscriber. Subscriber callback will be called to process the messages,
/// until the corresponding session is closed or dropped.
///
/// @param session: The zenoh session.
/// @param key_expr: The key expression to subscribe.
/// @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
/// @param _options: The options to be passed to the subscriber declaration.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_declare_background_subscriber(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    _options: Option<&mut z_subscriber_options_t>,
) -> result::z_result_t {
    let subscriber = _declare_subscriber_inner(session, key_expr, callback, _options);
    match subscriber.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            tracing::error!("{}", e);
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

/// Undeclares subscriber callback and resets it to its gravestone state.
/// This is equivalent to calling `z_undeclare_subscriber()` and discarding its return value.
#[no_mangle]
pub extern "C" fn z_subscriber_drop(this_: &mut z_moved_subscriber_t) {
    std::mem::drop(this_.take_rust_type())
}

/// Returns ``true`` if subscriber is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_internal_subscriber_check(this_: &z_owned_subscriber_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Undeclares the subscriber.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_undeclare_subscriber(this_: &mut z_moved_subscriber_t) -> result::z_result_t {
    if let Some(s) = this_.take_rust_type() {
        if let Err(e) = s.undeclare().wait() {
            tracing::error!("{}", e);
            return result::Z_EGENERIC;
        }
    }
    result::Z_OK
}
