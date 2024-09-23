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

use zenoh::{pubsub::Subscriber, Wait};

pub use crate::opaque_types::{z_loaned_subscriber_t, z_moved_subscriber_t, z_owned_subscriber_t};
use crate::{
    keyexpr::*,
    result,
    transmute::{LoanedCTypeMut, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
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
    _options: Option<&mut z_subscriber_options_t>,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let subscriber = session
        .declare_subscriber(key_expr)
        .callback(move |sample| {
            let mut owned_sample = Some(sample);
            z_closure_sample_call(z_closure_sample_loan(&callback), unsafe {
                owned_sample
                    .as_mut()
                    .unwrap_unchecked()
                    .as_loaned_c_type_mut()
            })
        });
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

/// Undeclares and drops subscriber.
///
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
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

/// Drops subscriber and resets it to its gravestone state.
/// The callback closure is not dropped and still keeps receiving and processing samples until the corresponding session is closed.
#[no_mangle]
pub extern "C" fn z_subscriber_drop(this_: &mut z_moved_subscriber_t) {
    std::mem::drop(this_.take_rust_type())
}

/// Returns ``true`` if subscriber is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_internal_subscriber_check(this_: &z_owned_subscriber_t) -> bool {
    this_.as_rust_type_ref().is_some()
}
