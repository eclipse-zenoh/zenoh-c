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

use std::convert::TryFrom;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::session::*;
use crate::LOG_INVALID_SESSION;
use libc::c_char;
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::KeyExpr;

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
    pub _align: [u64; 2],
    pub _padding: [usize; 2],
}
impl From<KeyExpr<'static>> for z_owned_keyexpr_t {
    fn from(val: KeyExpr<'static>) -> Self {
        unsafe { std::mem::transmute(Some(val)) }
    }
}
impl Deref for z_owned_keyexpr_t {
    type Target = Option<KeyExpr<'static>>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for z_owned_keyexpr_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl z_owned_keyexpr_t {
    pub fn null() -> Self {
        unsafe { std::mem::transmute(None::<KeyExpr>) }
    }
}

/// Returns a :c:type:`z_keyexpr_t` loaned from :c:type:`z_owned_keyexpr_t`.
#[no_mangle]
pub extern "C" fn z_keyexpr_loan(keyexpr: &z_owned_keyexpr_t) -> z_keyexpr_t {
    keyexpr.as_ref().map(|k| k.borrowing_clone()).into()
}

/// Frees `keyexpr` and invalidates it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_keyexpr_free(keyexpr: &mut z_owned_keyexpr_t) {
    std::mem::drop(keyexpr.take())
}

/// Returns `true` if `keyexpr` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_keyexpr_check(keyexpr: &z_owned_keyexpr_t) -> bool {
    keyexpr.deref().is_some()
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
    pub _align: [u64; 2],
    pub _padding: [usize; 2],
}
impl<'a> From<KeyExpr<'a>> for z_keyexpr_t {
    fn from(val: KeyExpr<'a>) -> Self {
        Some(val).into()
    }
}
impl<'a> From<Option<KeyExpr<'a>>> for z_keyexpr_t {
    fn from(val: Option<KeyExpr<'a>>) -> Self {
        unsafe { std::mem::transmute(val) }
    }
}
impl Deref for z_keyexpr_t {
    type Target = Option<KeyExpr<'static>>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for z_keyexpr_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl z_keyexpr_t {
    pub fn null() -> Self {
        unsafe { std::mem::transmute(None::<KeyExpr>) }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UninitializedKeyExprError;
impl std::fmt::Display for UninitializedKeyExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Uninitialized Key Expression detected, make sure you use `z_keyexpr_check` or `z_loaned_keyexpr_check` after constructing your key expressions")
    }
}
impl std::error::Error for UninitializedKeyExprError {}
impl<'a> TryFrom<z_keyexpr_t> for KeyExpr<'a> {
    type Error = UninitializedKeyExprError;
    fn try_from(value: z_keyexpr_t) -> Result<Self, Self::Error> {
        match value.as_ref() {
            Some(ke) => {
                Ok(unsafe { std::mem::transmute::<KeyExpr, KeyExpr<'a>>(ke.borrowing_clone()) })
            }
            None => Err(UninitializedKeyExprError),
        }
    }
}

/// Returns `true` if `keyexpr` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_loaned_keyexpr_check(keyexpr: &z_keyexpr_t) -> bool {
    keyexpr.deref().is_some()
}

/// Constructs a :c:type:`z_keyexpr_t` departing from a string.
/// It is a loaned key expression.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr(name: *const c_char) -> z_keyexpr_t {
    match std::str::from_utf8(std::slice::from_raw_parts(name as _, libc::strlen(name))) {
        Ok(name) => match KeyExpr::try_from(name) {
            Ok(v) => v.into(),
            Err(e) => {
                log::error!("{}", e);
                z_keyexpr_t::null()
            }
        },
        Err(e) => {
            log::error!("{}", e);
            z_keyexpr_t::null()
        }
    }
}

/// Constructs a null-terminated string departing from a :c:type:`z_keyexpr_t`.
/// The user is responsible of freeing the allocated string being returned.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_to_string(keyexpr: z_keyexpr_t) -> *mut c_char {
    match keyexpr.as_ref() {
        Some(ke) => std::ffi::CString::new(ke.as_str()).unwrap().into_raw(),
        None => std::ptr::null_mut(),
    }
}

impl<'a> From<&'a z_owned_keyexpr_t> for z_keyexpr_t {
    fn from(oke: &'a z_owned_keyexpr_t) -> Self {
        unsafe { std::mem::transmute_copy(oke) }
    }
}

impl<'a> From<&'a KeyExpr<'a>> for z_keyexpr_t {
    fn from(key: &'a KeyExpr<'a>) -> Self {
        key.borrowing_clone().into()
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
    let key_expr = match keyexpr.as_ref() {
        Some(ke) => ke,
        None => {
            log::warn!("{}", UninitializedKeyExprError);
            return z_owned_keyexpr_t::null();
        }
    };
    match session.as_ref() {
        Some(s) => match s.declare_keyexpr(key_expr).res_sync() {
            Ok(id) => id.into_owned().into(),
            Err(e) => {
                log::debug!("{}", e);
                z_owned_keyexpr_t::null()
            }
        },
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            z_owned_keyexpr_t::null()
        }
    }
}

/// Undeclare the key expression generated by a call to :c:func:`z_declare_keyexpr`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_undeclare_keyexpr(
    session: z_session_t,
    keyexpr: &mut z_owned_keyexpr_t,
) {
    match session.as_ref() {
        Some(s) => match s
            .undeclare(keyexpr.as_ref().unwrap().borrowing_clone())
            .res()
        {
            Ok(()) => {}
            Err(e) => {
                log::debug!("{}", e);
            }
        },
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
        }
    }
}
