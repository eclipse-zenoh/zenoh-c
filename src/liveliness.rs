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

use zenoh::{
    liveliness::{Liveliness, LivelinessToken},
    prelude::{SessionDeclarations, SplitBuffer},
};
use zenoh_util::core::{zresult::ErrNo, SyncResolve};

use crate::{
    z_closure_reply_call, z_closure_sample_call, z_keyexpr_t, z_owned_closure_reply_t,
    z_owned_closure_sample_t, z_owned_subscriber_t, z_sample_t, z_session_t,
};

/// A liveliness token that can be used to provide the network with information about connectivity to its
/// declarer: when constructed, a PUT sample will be received by liveliness subscribers on intersecting key
/// expressions.
///
/// A DELETE on the token's key expression will be received by subscribers if the token is destroyed, or if connectivity between the subscriber and the token's creator is lost.
#[repr(C)]
pub struct zc_owned_liveliness_token_t {
    _inner: [usize; 4],
}

/// The gravestone value for liveliness tokens.
#[no_mangle]
pub extern "C" fn zc_liveliness_token_null() -> zc_owned_liveliness_token_t {
    zc_owned_liveliness_token_t { _inner: [0; 4] }
}

/// Returns `true` unless the token is at its gravestone value.
#[no_mangle]
pub extern "C" fn zc_liveliness_token_check(token: &zc_owned_liveliness_token_t) -> bool {
    token._inner.iter().any(|v| *v != 0)
}
/// The options for `zc_liveliness_declare_token`
#[repr(C)]
pub struct zc_owned_liveliness_declaration_options_t {
    _inner: u8,
}
/// The gravestone value for `zc_owned_liveliness_declaration_options_t`
#[no_mangle]
pub extern "C" fn zc_liveliness_declaration_options_null(
) -> zc_owned_liveliness_declaration_options_t {
    zc_owned_liveliness_declaration_options_t { _inner: 0 }
}
/// Returns `true` if the options are valid.
#[no_mangle]
pub extern "C" fn zc_liveliness_declaration_options_check(
    _opts: &zc_owned_liveliness_declaration_options_t,
) -> bool {
    true
}
/// Destroys the options.
#[no_mangle]
pub extern "C" fn zc_liveliness_declaration_options_drop(
    opts: &mut zc_owned_liveliness_declaration_options_t,
) {
    *opts = zc_liveliness_declaration_options_null()
}
impl From<LivelinessToken<'static>> for zc_owned_liveliness_token_t {
    fn from(value: LivelinessToken<'static>) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<zc_owned_liveliness_token_t> for Option<LivelinessToken<'static>> {
    fn from(value: zc_owned_liveliness_token_t) -> Self {
        if value._inner.iter().all(|v| *v == 0) {
            None
        } else {
            Some(unsafe { core::mem::transmute(value) })
        }
    }
}
/// Constructs and declares a liveliness token on the network.
///
/// Liveliness token subscribers on an intersecting key expression will receive a PUT sample when connectivity
/// is achieved, and a DELETE sample if it's lost.
///
/// Passing `NULL` as options is valid and equivalent to a pointer to the default options.
#[no_mangle]
pub extern "C" fn zc_liveliness_declare_token(
    session: z_session_t,
    key: z_keyexpr_t,
    _options: Option<&zc_owned_liveliness_declaration_options_t>,
) -> zc_owned_liveliness_token_t {
    let Some(session) = session.upgrade() else {
        log::error!("Failed to declare liveliness token: provided session was invalid");
        return zc_liveliness_token_null();
    };
    match session.liveliness().declare_token(key).res() {
        Ok(token) => unsafe { core::mem::transmute(token) },
        Err(e) => {
            log::error!("Failed to declare liveliness token: {e}");
            zc_liveliness_token_null()
        }
    }
}

/// Destroys a liveliness token, notifying subscribers of its destruction.
#[no_mangle]
pub extern "C" fn zc_liveliness_undeclare_token(token: &mut zc_owned_liveliness_token_t) {
    let Some(token): Option<LivelinessToken> =
        core::mem::replace(token, zc_liveliness_token_null()).into()
    else {
        return;
    };
    if let Err(e) = token.undeclare().res() {
        log::error!("Failed to undeclare token: {e}");
    }
}

/// The options for :c:func:`zc_liveliness_declare_subscriber`
#[repr(C)]
pub struct zc_owned_liveliness_declare_subscriber_options_t {
    _inner: u8,
}
/// The gravestone value for `zc_owned_liveliness_declare_subscriber_options_t`
#[no_mangle]
pub extern "C" fn zc_liveliness_subscriber_options_null(
) -> zc_owned_liveliness_declare_subscriber_options_t {
    zc_owned_liveliness_declare_subscriber_options_t { _inner: 0 }
}
/// Returns `true` if the options are valid.
#[no_mangle]
pub extern "C" fn zc_liveliness_subscriber_options_check(
    _opts: &zc_owned_liveliness_declare_subscriber_options_t,
) -> bool {
    true
}
/// Destroys the options.
#[no_mangle]
pub extern "C" fn zc_liveliness_subscriber_options_drop(
    opts: &mut zc_owned_liveliness_declare_subscriber_options_t,
) {
    *opts = zc_liveliness_subscriber_options_null()
}

/// Declares a subscriber on liveliness tokens that intersect `key`.
///
/// Parameters:
///     z_session_t session: The zenoh session.
///     z_keyexpr_t keyexpr: The key expression to subscribe.
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
    session: z_session_t,
    key: z_keyexpr_t,
    callback: &mut z_owned_closure_sample_t,
    _options: Option<&zc_owned_liveliness_declare_subscriber_options_t>,
) -> z_owned_subscriber_t {
    let Some(session) = session.upgrade() else {
        log::error!("Failed to declare liveliness token: provided session was invalid");
        return z_owned_subscriber_t::null();
    };
    let callback = core::mem::replace(callback, z_owned_closure_sample_t::empty());
    match session
        .liveliness()
        .declare_subscriber(key)
        .callback(move |sample| {
            let payload = sample.payload.contiguous();
            let owner = match payload {
                std::borrow::Cow::Owned(v) => zenoh::buffers::ZBuf::from(v),
                _ => sample.payload.clone(),
            };
            let sample = z_sample_t::new(&sample, &owner);
            z_closure_sample_call(&callback, &sample)
        })
        .res()
    {
        Ok(token) => z_owned_subscriber_t::new(token),
        Err(e) => {
            log::error!("Failed to subscribe to liveliness: {e}");
            z_owned_subscriber_t::null()
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
pub extern "C" fn zc_liveliness_get_options_null() -> zc_liveliness_get_options_t {
    zc_liveliness_get_options_t { timeout_ms: 0 }
}
/// The gravestone value for `zc_liveliness_get_options_t`
#[no_mangle]
pub extern "C" fn zc_liveliness_get_options_default() -> zc_liveliness_get_options_t {
    zc_liveliness_get_options_t { timeout_ms: 10000 }
}
/// Returns `true` if the options are valid.
#[no_mangle]
pub extern "C" fn zc_liveliness_get_options_check(_opts: &zc_liveliness_get_options_t) -> bool {
    true
}
/// Destroys the options.
#[no_mangle]
pub extern "C" fn zc_liveliness_get_options_drop(opts: &mut zc_liveliness_get_options_t) {
    *opts = zc_liveliness_get_options_null()
}

/// Queries liveliness tokens currently on the network with a key expression intersecting with `key`.
///
/// Note that the same "value stealing" tricks apply as with a normal :c:func:`z_get`
///
/// Passing `NULL` as options is valid and equivalent to passing a pointer to the default options.
#[no_mangle]
pub extern "C" fn zc_liveliness_get(
    session: z_session_t,
    key: z_keyexpr_t,
    callback: &mut z_owned_closure_reply_t,
    options: Option<&zc_liveliness_get_options_t>,
) -> i8 {
    let Some(session) = session.upgrade() else {
        log::error!("Failed to declare liveliness token: provided session was invalid");
        return i8::MIN;
    };
    let callback = core::mem::replace(callback, z_owned_closure_reply_t::empty());
    let liveliness: Liveliness<'static> = session.liveliness();
    let mut builder = liveliness
        .get(key)
        .callback(move |response| z_closure_reply_call(&callback, &mut response.into()));
    if let Some(options) = options {
        builder = builder.timeout(core::time::Duration::from_millis(options.timeout_ms as u64))
    }
    match builder.res() {
        Ok(()) => 0,
        Err(e) => {
            log::error!("Failed to subscribe to liveliness: {e}");
            e.errno().get()
        }
    }
}
