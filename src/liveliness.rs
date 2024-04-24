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
use zenoh::prelude::SyncResolve;
use zenoh::{
    liveliness::{Liveliness, LivelinessToken},
    prelude::SessionDeclarations,
};

use crate::transmute::TransmuteIntoHandle;
use crate::{
    errors,
    transmute::{Inplace, TransmuteFromHandle, TransmuteRef, TransmuteUninitPtr},
    z_closure_reply_call, z_closure_sample_call, z_keyexpr_t, z_owned_closure_reply_t,
    z_owned_closure_sample_t, z_owned_subscriber_t, z_session_t,
};

use crate::opaque_types::zc_liveliness_token_t;
use crate::opaque_types::zc_owned_liveliness_token_t;
decl_transmute_owned!(
    Option<LivelinessToken<'static>>,
    zc_owned_liveliness_token_t
);
decl_transmute_handle!(LivelinessToken<'static>, zc_liveliness_token_t);

/// The gravestone value for liveliness tokens.
#[no_mangle]
pub extern "C" fn zc_liveliness_token_null(this: *mut MaybeUninit<zc_owned_liveliness_token_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Returns `true` unless the token is at its gravestone value.
#[no_mangle]
pub extern "C" fn zc_liveliness_token_check(this: &zc_owned_liveliness_token_t) -> bool {
    this.transmute_ref().is_some()
}
/// The options for `zc_liveliness_declare_token`
#[repr(C)]
pub struct zc_liveliness_declaration_options_t {
    _dummy: u8,
}

#[no_mangle]
pub extern "C" fn zc_liveliness_declaration_options_default(this: &mut zc_liveliness_declaration_options_t)
{
    *this = zc_liveliness_declaration_options_t { _dummy: 0 };
}

/// Constructs and declares a liveliness token on the network.
///
/// Liveliness token subscribers on an intersecting key expression will receive a PUT sample when connectivity
/// is achieved, and a DELETE sample if it's lost.
///
/// Passing `NULL` as options is valid and equivalent to a pointer to the default options.
#[no_mangle]
pub extern "C" fn zc_liveliness_declare_token(
    this: *mut MaybeUninit<zc_owned_liveliness_token_t>,
    session: &z_session_t,
    key_expr: &z_keyexpr_t,
    _options: Option<&mut zc_liveliness_declaration_options_t>,
) -> errors::z_error_t {
    let this = this.transmute_uninit_ptr();
    let session = session.transmute_ref();
    let key_expr = key_expr.transmute_ref();
    match session.liveliness().declare_token(key_expr).res() {
        Ok(token) => {
            Inplace::init(this, Some(token));
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to undeclare token: {e}");
            Inplace::empty(this);
            errors::Z_EGENERIC
        }
    }
}

/// Destroys a liveliness token, notifying subscribers of its destruction.
#[no_mangle]
pub extern "C" fn zc_liveliness_undeclare_token(
    this: &mut zc_owned_liveliness_token_t,
) -> errors::z_error_t {
    let this = this.transmute_mut();
    if let Some(token) = this.extract().take() {
        if let Err(e) = token.undeclare().res() {
            log::error!("Failed to undeclare token: {e}");
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}

/// The options for :c:func:`zc_liveliness_declare_subscriber`
#[repr(C)]
pub struct zc_liveliness_declare_subscriber_options_t {
    _dummy: u8,
}

#[no_mangle]
pub extern "C" fn zc_liveliness_subscriber_options_default(this: &mut zc_liveliness_declare_subscriber_options_t) {
    *this = zc_liveliness_declare_subscriber_options_t { _dummy: 0 };
}

/// Declares a subscriber on liveliness tokens that intersect `key`.
///
/// Parameters:
///     z_session_t session: The zenoh session.
///     z_keyexpr_t key_expr: The key expression to subscribe.
///     z_owned_closure_sample_t callback: The callback function that will be called each time a
///                                        liveliness token status changed.
///     zc_owned_liveliness_declare_subscriber_options_t _options: The options to be passed to describe the options to be passed to the liveliness subscriber declaration.
///
/// Returns:
///    A :c:type:`z_owned_subscriber_t`.
///
///    To check if the subscription succeeded and if the subscriber is still valid,
///    you may use `z_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[no_mangle]
pub extern "C" fn zc_liveliness_declare_subscriber(
    this: *mut MaybeUninit<z_owned_subscriber_t>,
    session: &z_session_t,
    key_expr: &z_keyexpr_t,
    callback: &mut z_owned_closure_sample_t,
    _options: Option<&mut zc_liveliness_declare_subscriber_options_t>,
) -> errors::z_error_t {
    let this = this.transmute_uninit_ptr();
    let session = session.transmute_ref();
    let callback = core::mem::replace(callback, z_owned_closure_sample_t::empty());
    let key_expr = key_expr.transmute_ref();
    match session
        .liveliness()
        .declare_subscriber(key_expr)
        .callback(move |sample| {
            let sample = sample.transmute_handle();
            z_closure_sample_call(&callback, sample)
        })
        .res()
    {
        Ok(subscriber) => {
            Inplace::init(this, Some(subscriber));
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to subscribe to liveliness: {e}");
            Inplace::empty(this);
            errors::Z_EGENERIC
        }
    }
}

/// The options for :c:func:`zc_liveliness_declare_subscriber`
#[repr(C)]
pub struct zc_liveliness_get_options_t {
    timeout_ms: u32,
}

/// The gravestone value for `zc_liveliness_get_options_t`
#[no_mangle]
pub extern "C" fn zc_liveliness_get_options_default(this: &mut zc_liveliness_get_options_t) {
    *this = zc_liveliness_get_options_t { timeout_ms: 10000 };
}

/// Queries liveliness tokens currently on the network with a key expression intersecting with `key`.
///
/// Note that the same "value stealing" tricks apply as with a normal :c:func:`z_get`
///
/// Passing `NULL` as options is valid and equivalent to passing a pointer to the default options.
#[no_mangle]
pub extern "C" fn zc_liveliness_get(
    session: &z_session_t,
    key_expr: &z_keyexpr_t,
    callback: &mut z_owned_closure_reply_t,
    options: Option<&mut zc_liveliness_get_options_t>,
) -> errors::z_error_t {
    let session = session.transmute_ref();
    let key_expr = key_expr.transmute_ref();
    let callback = core::mem::replace(callback, z_owned_closure_reply_t::empty());
    let liveliness: Liveliness<'static> = session.liveliness();
    let mut builder = liveliness
        .get(key_expr)
        .callback(move |response| z_closure_reply_call(&callback, response.transmute_handle()));
    if let Some(options) = options {
        builder = builder.timeout(core::time::Duration::from_millis(options.timeout_ms as u64));
    }
    match builder.res() {
        Ok(()) => errors::Z_OK,
        Err(e) => {
            log::error!("Failed to subscribe to liveliness: {e}");
            errors::Z_EGENERIC
        }
    }
}
