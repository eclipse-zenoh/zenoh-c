use std::ffi::CString;

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
use crate::session::*;
use crate::LOG_INVALID_SESSION;
use libc::c_char;
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::{KeyExpr, ZInt};

/// A zenoh-allocated key expression.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"key/expression"``.
///    - ``"key/ex*"``.
///
/// Key expressions can be mapped to numerical ids through :c:func:`z_declare_expr`
/// for wire and computation efficiency.
///
/// A key expression can be either:
///   - A plain string expression.
///   - A pure numerical id.
///   - The combination of a numerical prefix and a string suffix.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
pub struct z_owned_keyexpr_t {
    pub _id: z_zint_t,
    pub _suffix: z_bytes_t,
    pub _session: z_session_t,
}

/// Returns a :c:type:`z_keyexpr_t` loaned from :c:type:`z_owned_keyexpr_t`.
#[no_mangle]
pub extern "C" fn z_keyexpr_loan(keyexpr: &z_owned_keyexpr_t) -> z_keyexpr_t {
    z_keyexpr_t {
        _id: keyexpr._id,
        _suffix: keyexpr._suffix,
        _session: keyexpr._session,
    }
}

/// Frees `keyexpr` and invalidates it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_keyexpr_free(keyexpr: &mut z_owned_keyexpr_t) {
    z_bytes_free(&mut keyexpr._suffix);
    keyexpr._id = 0;
    keyexpr._session = z_session_t::null();
}

/// Returns `true` if `keyexpr` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_keyexpr_check(keyexpr: &z_owned_keyexpr_t) -> bool {
    (keyexpr._id != 0 && keyexpr._session.check()) || z_bytes_check(&keyexpr._suffix)
}

impl<'a> From<KeyExpr<'a>> for z_owned_keyexpr_t {
    fn from(key: KeyExpr<'a>) -> Self {
        let key: z_keyexpr_t = (&key).into();
        key.into()
    }
}

impl<'a> From<z_owned_keyexpr_t> for KeyExpr<'a> {
    fn from(key: z_owned_keyexpr_t) -> Self {
        let key: z_keyexpr_t = (&key).into();
        key.into()
    }
}

impl From<z_keyexpr_t> for z_owned_keyexpr_t {
    fn from(oke: z_keyexpr_t) -> Self {
        unsafe { std::mem::transmute(oke) }
    }
}

/// A loaned key expression.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"key/expression"``.
///    - ``"key/ex*"``.
///
/// Key expressions can be mapped to numerical ids through :c:func:`z_declare_expr`
/// for wire and computation efficiency.
///
/// A key expression can be either:
///   - A plain string expression.
///   - A pure numerical id.
///   - The combination of a numerical prefix and a string suffix.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_keyexpr_t {
    pub _id: z_zint_t,
    pub _suffix: z_bytes_t,
    pub _session: z_session_t,
}

impl z_keyexpr_t {
    pub fn null() -> z_keyexpr_t {
        z_keyexpr_t {
            _id: 0,
            _suffix: z_bytes_t::empty(),
            _session: z_session_t::null(),
        }
    }
}

/// Constructs a :c:type:`z_keyexpr_t` departing from a string.
/// It is a loaned key expression.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr(name: *const c_char) -> z_keyexpr_t {
    z_keyexpr_t {
        _id: 0,
        _suffix: z_bytes_t {
            start: name as *const _,
            len: if name.is_null() {
                0
            } else {
                libc::strlen(name)
            },
        },
        _session: z_session_t::null(),
    }
}

/// Constructs a null-terminated string departing from a :c:type:`z_keyexpr_t`.
/// The user is responsible of freeing the allocated string being returned.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_to_string(keyexpr: z_keyexpr_t) -> *mut c_char {
    let ke: KeyExpr<'_> = keyexpr.into();
    match CString::new(ke.as_str().as_bytes().to_vec()) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

impl<'a> From<&'a z_owned_keyexpr_t> for z_keyexpr_t {
    fn from(oke: &'a z_owned_keyexpr_t) -> Self {
        unsafe { std::mem::transmute_copy(oke) }
    }
}

impl<'a> From<&'a KeyExpr<'a>> for z_keyexpr_t {
    fn from(key: &'a KeyExpr<'a>) -> Self {
        let (id, suffix) = key.as_id_and_suffix();
        z_keyexpr_t {
            _id: id as z_zint_t,
            _suffix: z_bytes_t {
                start: suffix.as_ptr() as *const _,
                len: suffix.len(),
            },
            _session: z_session_t::null(),
        }
    }
}

impl<'a> From<z_keyexpr_t> for KeyExpr<'a> {
    fn from(r: z_keyexpr_t) -> Self {
        unsafe {
            let len = r._suffix.len;
            match (r._id, len) {
                (id, 0) => KeyExpr::from(id as ZInt),
                (0, _) => std::str::from_utf8(std::slice::from_raw_parts(
                    r._suffix.start as *const _,
                    len,
                ))
                .unwrap()
                .into(),
                (id, _) => KeyExpr::from(id as ZInt).with_suffix(
                    std::str::from_utf8(std::slice::from_raw_parts(
                        r._suffix.start as *const _,
                        len,
                    ))
                    .unwrap(),
                ),
            }
        }
    }
}

/**************************************/
/*            DECLARATION             */
/**************************************/
/// Declare a key expression. The id is returned as a :c:type:`z_keyexpr_t` with a nullptr suffix.
///
/// This numerical id will be used on the network to save bandwidth and
/// ease the retrieval of the concerned resource in the routing tables.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_declare_keyexpr(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
) -> z_owned_keyexpr_t {
    fn ok(session: z_session_t, id: ZInt) -> z_owned_keyexpr_t {
        z_owned_keyexpr_t {
            _id: id,
            _suffix: z_bytes_t::empty(),
            _session: session,
        }
    }

    fn err() -> z_owned_keyexpr_t {
        z_owned_keyexpr_t {
            _id: 0,
            _suffix: z_bytes_t::empty(),
            _session: z_session_t::null(),
        }
    }

    match session.as_ref() {
        Some(s) => match s.declare_expr(keyexpr).res() {
            Ok(id) => ok(session, id),
            Err(e) => {
                log::debug!("{}", e);
                err()
            }
        },
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            err()
        }
    }
}

/// Undeclare the key expression generated by a call to :c:func:`z_declare_keyexpr`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_undeclare_keyexpr(keyexpr: &mut z_owned_keyexpr_t) {
    match keyexpr._session.as_ref() {
        Some(s) => {
            if let Err(e) = s.undeclare_expr(keyexpr._id as ZInt).res() {
                log::debug!("{}", e);
            }
            z_keyexpr_free(keyexpr);
        }
        None => log::debug!("{}", LOG_INVALID_SESSION),
    }
}
