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

use crate::impl_guarded_transmute;
use crate::session::*;
use crate::z_bytes_t;
use crate::z_owned_str_t;
use crate::z_str_null;
use crate::GuardedTransmute;
use crate::LOG_INVALID_SESSION;
use libc::c_char;
use zenoh::prelude::keyexpr;
use zenoh::prelude::sync::SyncResolve;
use zenoh::prelude::KeyExpr;
use zenoh_util::core::zresult::ErrNo;

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
/// A `key expression <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Key%20Expressions.md>`_ can be either:
///   - A plain string expression.
///   - A pure numerical id.
///   - The combination of a numerical prefix and a string suffix.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[cfg(not(target_arch = "arm"))]
#[repr(C, align(8))]
pub struct z_owned_keyexpr_t([u64; 4]);
#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_owned_keyexpr_t([u64; 3]);

impl_guarded_transmute!(Option<KeyExpr<'static>>, z_owned_keyexpr_t);

impl From<Option<KeyExpr<'static>>> for z_owned_keyexpr_t {
    fn from(val: Option<KeyExpr<'static>>) -> Self {
        val.transmute()
    }
}
impl From<KeyExpr<'static>> for z_owned_keyexpr_t {
    fn from(val: KeyExpr<'static>) -> Self {
        Some(val).into()
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
        None::<KeyExpr>.into()
    }
}

/// Constructs a null safe-to-drop value of 'z_owned_keyexpr_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_keyexpr_null() -> z_owned_keyexpr_t {
    z_owned_keyexpr_t::null()
}

/// Constructs a :c:type:`z_keyexpr_t` departing from a string, copying the passed string.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_new(name: *const c_char) -> z_owned_keyexpr_t {
    if name.is_null() {
        return z_owned_keyexpr_t::null();
    }
    let name = std::slice::from_raw_parts(name as _, libc::strlen(name));
    match std::str::from_utf8(name) {
        Ok(name) => match KeyExpr::try_from(name) {
            Ok(v) => v.into_owned().into(),
            Err(e) => {
                log::error!("Couldn't construct a keyexpr from {:02x?}: {}", name, e);
                z_owned_keyexpr_t::null()
            }
        },
        Err(e) => {
            log::error!("{}", e);
            z_owned_keyexpr_t::null()
        }
    }
}

/// Returns a :c:type:`z_keyexpr_t` loaned from :c:type:`z_owned_keyexpr_t`.
#[no_mangle]
pub extern "C" fn z_keyexpr_loan(keyexpr: &z_owned_keyexpr_t) -> z_keyexpr_t {
    keyexpr.as_ref().map(|k| k.borrowing_clone()).into()
}

/// Frees `keyexpr` and invalidates it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_keyexpr_drop(keyexpr: &mut z_owned_keyexpr_t) {
    std::mem::drop(keyexpr.take())
}

/// Returns ``true`` if `keyexpr` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_keyexpr_check(keyexpr: &z_owned_keyexpr_t) -> bool {
    keyexpr.deref().is_some()
}

/// A loaned key expression.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"key/expression"``.
///    - ``"key/ex*"``.
///
/// Using :c:func:`z_declare_keyexpr` allows zenoh to optimize a key expression,
/// both for local processing and network-wise.
#[cfg(not(target_arch = "arm"))]
#[repr(C, align(8))]
pub struct z_keyexpr_t([u64; 4]);
#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_keyexpr_t([u64; 3]);

impl_guarded_transmute!(Option<KeyExpr<'_>>, z_keyexpr_t);
impl_guarded_transmute!(z_keyexpr_t, z_owned_keyexpr_t);

impl<'a> From<KeyExpr<'a>> for z_keyexpr_t {
    fn from(val: KeyExpr<'a>) -> Self {
        Some(val).into()
    }
}

impl From<z_keyexpr_t> for z_owned_keyexpr_t {
    fn from(oke: z_keyexpr_t) -> Self {
        oke.transmute()
    }
}

impl<'a> From<Option<KeyExpr<'a>>> for z_keyexpr_t {
    fn from(val: Option<KeyExpr<'a>>) -> Self {
        val.transmute()
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
        None.into()
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

/// Returns ``true`` if `keyexpr` is initialized.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_keyexpr_is_initialized(keyexpr: &z_keyexpr_t) -> bool {
    keyexpr.deref().is_some()
}

/// Returns ``0`` if the passed string is a valid (and canon) key expression.
/// Otherwise returns error value
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_is_canon(start: *const c_char, len: usize) -> i8 {
    let name = std::slice::from_raw_parts(start as _, len);
    match std::str::from_utf8(name) {
        Ok(name) => match keyexpr::new(name) {
            Ok(_) => 0,
            Err(e) => {
                log::error!("Couldn't construct a keyexpr from `{}`: {}", name, e);
                e.errno().get()
            }
        },
        Err(e) => {
            log::error!("{:02x?} is not valid UTF8 {}", name, e);
            i8::MIN
        }
    }
}

/// Canonizes the passed string in place, possibly shortening it by placing a new null-terminator.
///
/// Returns ``0`` upon success, negative values upon failure.  
/// Returns a negative value if canonization failed, which indicates that the passed string was an invalid
/// key expression for reasons other than a non-canon form.
///
/// May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_canonize_null_terminated(start: *mut c_char) -> i8 {
    let mut len = libc::strlen(start);
    match z_keyexpr_canonize(start, &mut len) {
        0 => {
            *start.add(len) = 0;
            0
        }
        err => err,
    }
}
/// Canonizes the passed string in place, possibly shortening it by modifying `len`.
///
/// Returns ``0`` upon success, negative values upon failure.  
/// Returns a negative value if canonization failed, which indicates that the passed string was an invalid
/// key expression for reasons other than a non-canon form.
///
/// May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_canonize(start: *mut c_char, len: &mut usize) -> i8 {
    let name = std::slice::from_raw_parts_mut(start as _, *len);
    match std::str::from_utf8_mut(name) {
        Ok(mut name) => match keyexpr::autocanonize(&mut name) {
            Ok(k) => {
                *len = k.len();
                0
            }
            Err(e) => {
                log::error!("Canonization error: {e}");
                e.errno().get()
            }
        },
        Err(e) => {
            log::error!("{:02x?} is not valid UTF8 {}", name, e);
            i8::MIN
        }
    }
}

/// Constructs a :c:type:`z_keyexpr_t` departing from a string.
/// It is a loaned key expression that aliases `name`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_keyexpr_from_slice(name: *const c_char, len: usize) -> z_keyexpr_t {
    let name = std::slice::from_raw_parts(name as _, len);
    match std::str::from_utf8(name) {
        Ok(name) => match KeyExpr::try_from(name) {
            Ok(v) => v.into(),
            Err(e) => {
                log::error!("Couldn't construct a keyexpr from `{}`: {}", name, e);
                z_keyexpr_t::null()
            }
        },
        Err(e) => {
            log::error!("{:02x?} is not valid UTF8 {}", name, e);
            z_keyexpr_t::null()
        }
    }
}

/// Constructs a :c:type:`z_keyexpr_t` departing from a string.
/// It is a loaned key expression that aliases `name`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr(name: *const c_char) -> z_keyexpr_t {
    if name.is_null() {
        z_keyexpr_t::null()
    } else {
        zc_keyexpr_from_slice(name, libc::strlen(name))
    }
}

/// Constructs a :c:type:`z_keyexpr_t` departing from a string without checking any of `z_keyexpr_t`'s assertions:
/// - `name` MUST be valid UTF8.
/// - `name` MUST follow the Key Expression specification, ie:
///   - MUST NOT contain ``//``, MUST NOT start nor end with ``/``, MUST NOT contain any of the characters ``?#$``.
///   - any instance of ``**`` may only be lead or followed by ``/``.
///   - the key expression must have canon form.
///
/// It is a loaned key expression that aliases `name`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_keyexpr_from_slice_unchecked(
    start: *const c_char,
    len: usize,
) -> z_keyexpr_t {
    let name = std::slice::from_raw_parts(start as _, len);
    let name = std::str::from_utf8_unchecked(name);
    let name: KeyExpr = keyexpr::from_str_unchecked(name).into();
    name.into()
}

/// Constructs a :c:type:`z_keyexpr_t` departing from a string without checking any of `z_keyexpr_t`'s assertions:
///
///  - `name` MUST be valid UTF8.
///  - `name` MUST follow the Key Expression specification, ie:
///
///   - MUST NOT contain `//`, MUST NOT start nor end with `/`, MUST NOT contain any of the characters `?#$`.
///   - any instance of `**` may only be lead or followed by `/`.
///   - the key expression must have canon form.
///
/// It is a loaned key expression that aliases `name`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_unchecked(name: *const c_char) -> z_keyexpr_t {
    zc_keyexpr_from_slice_unchecked(name, libc::strlen(name))
}

/// Constructs a null-terminated string departing from a :c:type:`z_keyexpr_t`.
/// The user is responsible of droping the returned string using `z_drop`
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_to_string(keyexpr: z_keyexpr_t) -> z_owned_str_t {
    match keyexpr.as_ref() {
        Some(ke) => ke.as_bytes().into(),
        None => z_str_null(),
    }
}

/// Returns the key expression's internal string by aliasing it.
///
/// Currently exclusive to zenoh-c
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_keyexpr_as_bytes(keyexpr: z_keyexpr_t) -> z_bytes_t {
    match keyexpr.as_ref() {
        Some(ke) => z_bytes_t {
            start: ke.as_ptr(),
            len: ke.len(),
        },
        None => z_bytes_t {
            start: std::ptr::null(),
            len: 0,
        },
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
pub extern "C" fn z_declare_keyexpr(
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
pub extern "C" fn z_undeclare_keyexpr(session: z_session_t, kexpr: &mut z_owned_keyexpr_t) -> i8 {
    let Some(kexpr) = kexpr.deref_mut().take() else {
        log::debug!("Attempted to undeclare dropped keyexpr");
        return i8::MIN;
    };

    match session.as_ref() {
        Some(s) => match s.undeclare(kexpr).res() {
            Ok(()) => 0,
            Err(e) => {
                log::debug!("{}", e);
                e.errno().get()
            }
        },
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            i8::MIN
        }
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// Returns ``0`` if `left` and `right` define equal sets and ``1`` if they don't
/// Returns negative values in case of error (if one of the key expressions is in an invalid state).
pub extern "C" fn z_keyexpr_equals(left: z_keyexpr_t, right: z_keyexpr_t) -> i8 {
    match (&*left, &*right) {
        (Some(l), Some(r)) => {
            if *l == *r {
                0
            } else {
                1
            }
        }
        _ => i8::MIN,
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// Returns ``0`` if `left` and `right` define sets that have at least one key in common, ``1`` if they don't.
/// Returns negative values in case of error (if one of the key expressions is in an invalid state).
pub extern "C" fn z_keyexpr_intersects(left: z_keyexpr_t, right: z_keyexpr_t) -> i8 {
    match (&*left, &*right) {
        (Some(l), Some(r)) => {
            if l.intersects(r) {
                0
            } else {
                1
            }
        }
        _ => i8::MIN,
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// Returns ``0`` if the set defined by `left` contains every key belonging to the set defined by `right`, ``1`` if they don't.
/// Returns negative values in case of error (if one of the key expressions is in an invalid state).
pub extern "C" fn z_keyexpr_includes(left: z_keyexpr_t, right: z_keyexpr_t) -> i8 {
    match (&*left, &*right) {
        (Some(l), Some(r)) => {
            if l.includes(r) {
                0
            } else {
                1
            }
        }
        _ => i8::MIN,
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// Performs string concatenation and returns the result as a `z_owned_keyexpr_t`.
/// In case of error, the return value will be set to its invalidated state.
///
/// You should probably prefer `z_keyexpr_join` as Zenoh may then take advantage of the hierachical separation it inserts.
///
/// To avoid odd behaviors, concatenating a key expression starting with `*` to one ending with `*` is forbidden by this operation,
/// as this would extremely likely cause bugs.
pub unsafe extern "C" fn z_keyexpr_concat(
    left: z_keyexpr_t,
    right_start: *const c_char,
    right_len: usize,
) -> z_owned_keyexpr_t {
    let left = match left.as_ref() {
        Some(l) => l,
        None => return z_owned_keyexpr_t::null(),
    };
    let right = std::slice::from_raw_parts(right_start as _, right_len);
    let right = match std::str::from_utf8(right) {
        Ok(r) => r,
        Err(e) => {
            log::error!(
                "Couldn't concatenate {:02x?} to {} because it is not valid UTF8: {}",
                right,
                left,
                e
            );
            return z_owned_keyexpr_t::null();
        }
    };
    match left.concat(right) {
        Ok(result) => result.into(),
        Err(e) => {
            log::error!("{}", e);
            z_owned_keyexpr_t::null()
        }
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// Performs path-joining (automatically inserting) and returns the result as a `z_owned_keyexpr_t`.
/// In case of error, the return value will be set to its invalidated state.
pub extern "C" fn z_keyexpr_join(left: z_keyexpr_t, right: z_keyexpr_t) -> z_owned_keyexpr_t {
    let left = match left.as_ref() {
        Some(l) => l,
        None => return z_owned_keyexpr_t::null(),
    };
    let right = match right.as_ref() {
        Some(r) => r,
        None => return z_owned_keyexpr_t::null(),
    };
    match left.join(right.as_str()) {
        Ok(result) => result.into(),
        Err(e) => {
            log::error!("{}", e);
            z_owned_keyexpr_t::null()
        }
    }
}
