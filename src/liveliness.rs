//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use std::mem::MaybeUninit;

use prebindgen_proc_macro::prebindgen;
use zenoh::{
    handlers::Callback,
    liveliness::{LivelinessSubscriberBuilder, LivelinessToken},
    sample::Sample,
    Wait,
};
use zenoh_ffi_opaque_types::opaque_types::{
    z_loaned_liveliness_token_t, z_owned_liveliness_token_t,
};

use crate::{
    result,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_reply_call, z_closure_reply_loan, z_closure_sample_call, z_closure_sample_loan,
    z_loaned_keyexpr_t, z_loaned_session_t, z_moved_closure_reply_t, z_moved_closure_sample_t,
    z_moved_liveliness_token_t, z_owned_subscriber_t,
};
decl_c_type!(
    owned(z_owned_liveliness_token_t, z_moved_liveliness_token_t, option LivelinessToken),
    loaned(z_loaned_liveliness_token_t),
);

/// @brief Constructs liveliness token in its gravestone state.
#[prebindgen]
pub fn z_internal_liveliness_token_null(this_: &mut MaybeUninit<z_owned_liveliness_token_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @brief Returns ``true`` if liveliness token is valid, ``false`` otherwise.
#[prebindgen]
pub fn z_internal_liveliness_token_check(this_: &z_owned_liveliness_token_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @brief Undeclares liveliness token, frees memory and resets it to a gravestone state.
#[prebindgen]
pub fn z_liveliness_token_drop(this_: &mut z_moved_liveliness_token_t) {
    let _ = this_.take_rust_type();
}

/// @brief The options for `z_liveliness_declare_token()`.
#[prebindgen]
#[repr(C)]
pub struct z_liveliness_token_options_t {
    _dummy: u8,
}

/// @brief Constructs default value for `z_liveliness_token_options_t`.
#[prebindgen]
pub fn z_liveliness_token_options_default(this: &mut MaybeUninit<z_liveliness_token_options_t>) {
    this.write(z_liveliness_token_options_t { _dummy: 0 });
}

/// @brief Borrows token.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_liveliness_token_loan(
    this: &z_owned_liveliness_token_t,
) -> &z_loaned_liveliness_token_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @brief Moves token.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_liveliness_token_move(
    this_: &mut z_owned_liveliness_token_t,
) -> &mut z_moved_liveliness_token_t {
    std::mem::transmute(this_)
}

/// @brief Constructs and declares a liveliness token on the network.
///
/// Liveliness token subscribers on an intersecting key expression will receive a PUT sample when connectivity
/// is achieved, and a DELETE sample if it's lost.
///
/// @param session: A Zenos session to declare the liveliness token.
/// @param token: An uninitialized memory location where liveliness token will be constructed.
/// @param key_expr: A keyexpr to declare a liveliess token for.
/// @param _options: Liveliness token declaration properties.
#[prebindgen]
pub fn z_liveliness_declare_token(
    session: &z_loaned_session_t,
    token: &mut MaybeUninit<z_owned_liveliness_token_t>,
    key_expr: &z_loaned_keyexpr_t,
    _options: Option<&z_liveliness_token_options_t>,
) -> result::z_result_t {
    let this = token.as_rust_type_mut_uninit();
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    match session.liveliness().declare_token(key_expr).wait() {
        Ok(token) => {
            this.write(Some(token));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to undeclare liveliness token: {e}");
            this.write(None);
            result::Z_EGENERIC
        }
    }
}

/// @brief Destroys a liveliness token, notifying subscribers of its destruction.
#[prebindgen]
pub fn z_liveliness_undeclare_token(this: &mut z_moved_liveliness_token_t) -> result::z_result_t {
    if let Some(token) = this.take_rust_type() {
        if let Err(e) = token.undeclare().wait() {
            crate::report_error!("Failed to undeclare token: {e}");
            return result::Z_EGENERIC;
        }
    }
    result::Z_OK
}

/// @brief The options for `z_liveliness_declare_subscriber()`
#[prebindgen]
#[repr(C)]
pub struct z_liveliness_subscriber_options_t {
    /// If true, subscriber will receive the state change notifications for liveliness tokens that were declared before its declaration.
    pub history: bool,
}

/// @brief Constucts default value for `z_liveliness_declare_subscriber_options_t`.
#[prebindgen]
pub fn z_liveliness_subscriber_options_default(
    this: &mut MaybeUninit<z_liveliness_subscriber_options_t>,
) {
    this.write(z_liveliness_subscriber_options_t { history: false });
}

fn _liveliness_declare_subscriber_inner<'a, 'b>(
    session: &'a z_loaned_session_t,
    key_expr: &'b z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    options: Option<&mut z_liveliness_subscriber_options_t>,
) -> LivelinessSubscriberBuilder<'a, 'b, Callback<Sample>> {
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let sub = session
        .liveliness()
        .declare_subscriber(key_expr)
        .history(options.is_some_and(|o| o.history))
        .callback(move |sample| {
            let mut owned_sample = Some(sample);
            z_closure_sample_call(z_closure_sample_loan(&callback), unsafe {
                owned_sample
                    .as_mut()
                    .unwrap_unchecked()
                    .as_loaned_c_type_mut()
            })
        });
    sub
}
/// @brief Declares a subscriber on liveliness tokens that intersect `key_expr`.
///
/// @param session: A Zenoh session.
/// @param subscriber: An uninitialized memory location where subscriber will be constructed.
/// @param key_expr: The key expression to subscribe to.
/// @param callback: The callback function that will be called each time a liveliness token status is changed.
/// @param options: The options to be passed to the liveliness subscriber declaration.
///
/// @return 0 in case of success, negative error values otherwise.
#[prebindgen]
pub fn z_liveliness_declare_subscriber(
    session: &z_loaned_session_t,
    subscriber: &mut MaybeUninit<z_owned_subscriber_t>,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    options: Option<&mut z_liveliness_subscriber_options_t>,
) -> result::z_result_t {
    let this = subscriber.as_rust_type_mut_uninit();
    let subscriber = _liveliness_declare_subscriber_inner(session, key_expr, callback, options);
    match subscriber.wait() {
        Ok(subscriber) => {
            this.write(Some(subscriber));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to subscribe to liveliness: {e}");
            this.write(None);
            result::Z_EGENERIC
        }
    }
}

/// @brief Declares a background subscriber on liveliness tokens that intersect `key_expr`. Subscriber callback will be called to process the messages,
/// until the corresponding session is closed or dropped.
/// @param session: The Zenoh session.
/// @param key_expr: The key expression to subscribe to.
/// @param callback: The callback function that will be called each time a liveliness token status is changed.
/// @param options: The options to be passed to the liveliness subscriber declaration.
///
/// @return 0 in case of success, negative error values otherwise.
#[prebindgen]
pub fn z_liveliness_declare_background_subscriber(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    options: Option<&mut z_liveliness_subscriber_options_t>,
) -> result::z_result_t {
    let subscriber = _liveliness_declare_subscriber_inner(session, key_expr, callback, options);
    match subscriber.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            crate::report_error!("Failed to subscribe to liveliness: {e}");
            result::Z_EGENERIC
        }
    }
}

/// @brief The options for `z_liveliness_get()`
#[prebindgen]
#[repr(C)]
pub struct z_liveliness_get_options_t {
    /// The timeout for the liveliness query in milliseconds. 0 means default query timeout from zenoh configuration.
    timeout_ms: u64,
}

/// @brief Constructs default value `z_liveliness_get_options_t`.
#[prebindgen]
pub fn z_liveliness_get_options_default(this: &mut MaybeUninit<z_liveliness_get_options_t>) {
    this.write(z_liveliness_get_options_t { timeout_ms: 10000 });
}

/// @brief Queries liveliness tokens currently on the network with a key expression intersecting with `key_expr`.
///
/// @param session: The Zenoh session.
/// @param key_expr: The key expression to query liveliness tokens for.
/// @param callback: The callback function that will be called for each received reply.
/// @param options: Additional options for the liveliness get operation.
#[prebindgen]
pub fn z_liveliness_get(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_reply_t,
    options: Option<&mut z_liveliness_get_options_t>,
) -> result::z_result_t {
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let liveliness = session.liveliness();
    let mut builder = liveliness.get(key_expr).callback(move |response| {
        let mut owned_response = Some(response);
        z_closure_reply_call(z_closure_reply_loan(&callback), unsafe {
            owned_response
                .as_mut()
                .unwrap_unchecked()
                .as_loaned_c_type_mut()
        })
    });
    if let Some(options) = options {
        builder = builder.timeout(core::time::Duration::from_millis(options.timeout_ms));
    }
    match builder.wait() {
        Ok(()) => result::Z_OK,
        Err(e) => {
            crate::report_error!("Failed to subscribe to liveliness: {e}");
            result::Z_EGENERIC
        }
    }
}
