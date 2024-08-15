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
use std::{error::Error, mem::MaybeUninit};

use libc::c_char;
#[cfg(feature = "unstable")]
use zenoh::key_expr::SetIntersectionLevel;
use zenoh::{
    key_expr::{keyexpr, Canonize, KeyExpr},
    prelude::*,
};

pub use crate::opaque_types::{
    z_loaned_keyexpr_t, z_moved_keyexpr_t, z_owned_keyexpr_t, z_view_keyexpr_t,
};
use crate::{
    result::{self, z_result_t, Z_OK},
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_loaned_session_t, z_view_string_from_substr, z_view_string_t,
};

decl_c_type! {
    owned(z_owned_keyexpr_t, option KeyExpr<'static>),
    loaned(z_loaned_keyexpr_t),
    view(z_view_keyexpr_t, Option<KeyExpr<'static>>),
}

/// Constructs an owned key expression in a gravestone state.
#[no_mangle]
pub extern "C" fn z_keyexpr_null(this: &mut MaybeUninit<z_owned_keyexpr_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Constructs a view key expression in empty state
#[no_mangle]
pub extern "C" fn z_view_keyexpr_empty(this: &mut MaybeUninit<z_view_keyexpr_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

fn keyexpr_create_inner(
    mut name: &'static mut str,
    should_auto_canonize: bool,
    should_copy: bool,
) -> Result<KeyExpr<'static>, Box<dyn Error + Send + Sync>> {
    if should_copy {
        let s = name.to_string();
        match should_auto_canonize {
            true => KeyExpr::<'static>::autocanonize(s),
            false => KeyExpr::<'static>::try_from(s),
        }
    } else {
        if should_auto_canonize {
            name.canonize();
        }
        return keyexpr::new(name).map(|k| k.into());
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
unsafe fn keyexpr_create(
    name: &'static mut [u8],
    should_auto_canonize: bool,
    should_copy: bool,
) -> Result<KeyExpr<'static>, result::z_result_t> {
    match std::str::from_utf8_mut(name) {
        Ok(name) => match keyexpr_create_inner(name, should_auto_canonize, should_copy) {
            Ok(v) => Ok(v),
            Err(e) => {
                tracing::error!("Couldn't construct a keyexpr: {}", e);
                Err(result::Z_EINVAL)
            }
        },
        Err(e) => {
            tracing::error!("{}", e);
            Err(result::Z_EPARSE)
        }
    }
}

/// Constructs a `z_owned_keyexpr_t` from a string, copying the passed string.
/// @return 0 in case of success, negative error code in case of failure (for example if `expr` is not a valid key expression or if it is
/// not in canon form.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_from_str(
    this: &mut MaybeUninit<z_owned_keyexpr_t>,
    expr: *const c_char,
) -> result::z_result_t {
    let len = if expr.is_null() {
        0
    } else {
        libc::strlen(expr)
    };
    z_keyexpr_from_substr(this, expr, len)
}

/// Constructs `z_owned_keyexpr_t` from a string, copying the passed string. The copied string is canonized.
/// @return 0 in case of success, negative error code in case of failure (for example if expr is not a valid key expression
/// even despite canonization).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_from_str_autocanonize(
    this: &mut MaybeUninit<z_owned_keyexpr_t>,
    expr: *const c_char,
) -> z_result_t {
    let mut len = if expr.is_null() {
        0
    } else {
        libc::strlen(expr)
    };
    z_keyexpr_from_substr_autocanonize(this, expr, &mut len)
}

/// Borrows `z_owned_keyexpr_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_keyexpr_loan(this: &z_owned_keyexpr_t) -> &z_loaned_keyexpr_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Borrows `z_view_keyexpr_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_keyexpr_loan(this: &z_view_keyexpr_t) -> &z_loaned_keyexpr_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Frees key expression and resets it to its gravestone state.
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_keyexpr_drop(this: z_moved_keyexpr_t) {}

/// Returns ``true`` if `keyexpr` is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_keyexpr_check(this: &z_owned_keyexpr_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Returns ``true`` if `keyexpr` is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_view_keyexpr_is_empty(this: &z_view_keyexpr_t) -> bool {
    this.as_rust_type_ref().is_none()
}

/// Returns 0 if the passed string is a valid (and canon) key expression.
/// Otherwise returns negative error value.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_is_canon(start: *const c_char, len: usize) -> z_result_t {
    let name = std::slice::from_raw_parts_mut(start as _, len);
    match keyexpr_create(name, false, false) {
        Ok(_) => result::Z_OK,
        Err(e) => e,
    }
}

/// Canonizes the passed string in place, possibly shortening it by placing a new null-terminator.
/// May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
///
/// @return 0 upon success, negative error values upon failure (if the passed string was an invalid
/// key expression for reasons other than a non-canon form).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_canonize_null_terminated(start: *mut c_char) -> z_result_t {
    let mut len = if start.is_null() {
        0
    } else {
        libc::strlen(start)
    };
    match z_keyexpr_canonize(start, &mut len) {
        Z_OK => {
            *start.add(len) = 0;
            Z_OK
        }
        err => err,
    }
}
/// Canonizes the passed string in place, possibly shortening it by modifying `len`.
///
/// May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
///  
/// @return 0 upon success, negative error values upon failure (if the passed string was an invalid
/// key expression for reasons other than a non-canon form).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_canonize(start: *mut c_char, len: &mut usize) -> z_result_t {
    if start.is_null() {
        return result::Z_EINVAL;
    }
    let name = std::slice::from_raw_parts_mut(start as _, *len);
    match keyexpr_create(name, true, false) {
        Ok(ke) => {
            *len = ke.len();
            result::Z_OK
        }
        Err(e) => e,
    }
}

/// Constructs a `z_view_keyexpr_t` by aliasing a substring.
/// `expr` must outlive the constucted key expression.
///
/// @param this_: An uninitialized location in memory where key expression will be constructed.
/// @param expr: A buffer with length >= `len`.
/// @param len: Number of characters from `expr` to consider.
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_view_keyexpr_from_substr(
    this: &mut MaybeUninit<z_view_keyexpr_t>,
    expr: *const c_char,
    len: usize,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    if expr.is_null() {
        this.write(None);
        return result::Z_EINVAL;
    }
    let expr = std::slice::from_raw_parts_mut(expr as _, len);
    match keyexpr_create(expr, false, false) {
        Ok(ke) => {
            this.write(Some(ke));
            result::Z_OK
        }
        Err(e) => {
            this.write(None);
            e
        }
    }
}

/// Constructs a `z_owned_keyexpr_t` by copying a substring.
///
/// @param this_: An uninitialized location in memory where key expression will be constructed.
/// @param expr: A buffer with length >= `len`.
/// @param len: Number of characters from `expr` to consider.
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_from_substr(
    this: &mut MaybeUninit<z_owned_keyexpr_t>,
    expr: *const c_char,
    len: usize,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    if expr.is_null() {
        this.write(None);
        return result::Z_EINVAL;
    }
    let expr = std::slice::from_raw_parts_mut(expr as _, len);
    match keyexpr_create(expr, false, true) {
        Ok(ke) => {
            this.write(Some(ke));
            result::Z_OK
        }
        Err(e) => {
            this.write(None);
            e
        }
    }
}

/// Constructs a `z_view_keyexpr_t` by aliasing a substring.
/// May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
/// `expr` must outlive the constucted key expression.
///
/// @param this_: An uninitialized location in memory where key expression will be constructed
/// @param expr: A buffer of with length >= `len`.
/// @param len: Number of characters from `expr` to consider. Will be modified to be equal to canonized key expression length.
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_view_keyexpr_from_substr_autocanonize(
    this: &mut MaybeUninit<z_view_keyexpr_t>,
    start: *mut c_char,
    len: &mut usize,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    if start.is_null() {
        this.write(None);
        return result::Z_EINVAL;
    }
    let name = std::slice::from_raw_parts_mut(start as _, *len);

    match keyexpr_create(name, true, false) {
        Ok(ke) => {
            *len = ke.len();
            this.write(Some(ke));
            result::Z_OK
        }
        Err(e) => {
            this.write(None);
            e
        }
    }
}

/// Constructs a `z_keyexpr_t` by copying a substring.
///
/// @param this_: An uninitialized location in memory where key expression will be constructed.
/// @param expr: A buffer of with length >= `len`.
/// @param len: Number of characters from `expr` to consider. Will be modified to be equal to canonized key expression length.
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_from_substr_autocanonize(
    this: &mut MaybeUninit<z_owned_keyexpr_t>,
    start: *const c_char,
    len: &mut usize,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    if start.is_null() {
        this.write(None);
        return result::Z_EINVAL;
    }
    let name = std::slice::from_raw_parts_mut(start as _, *len);

    match keyexpr_create(name, true, true) {
        Ok(ke) => {
            *len = ke.len();
            this.write(Some(ke));
            result::Z_OK
        }
        Err(e) => {
            this.write(None);
            e
        }
    }
}

/// Constructs a `z_view_keyexpr_t` by aliasing a string.
/// @return 0 in case of success, negative error code in case of failure (for example if expr is not a valid key expression or if it is
/// not in canon form.
/// `expr` must outlive the constucted key expression.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_view_keyexpr_from_str(
    this: &mut MaybeUninit<z_view_keyexpr_t>,
    expr: *const c_char,
) -> z_result_t {
    if expr.is_null() {
        this.as_rust_type_mut_uninit().write(None);
        result::Z_EINVAL
    } else {
        let len = if expr.is_null() {
            0
        } else {
            libc::strlen(expr)
        };
        z_view_keyexpr_from_substr(this, expr, len)
    }
}

/// Constructs a `z_view_keyexpr_t` by aliasing a string.
/// The string is canonized in-place before being passed to keyexpr, possibly shortening it by modifying `len`.
/// May SEGFAULT if `expr` is NULL or lies in read-only memory (as values initialized with string litterals do).
/// `expr` must outlive the constucted key expression.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_view_keyexpr_from_str_autocanonize(
    this: &mut MaybeUninit<z_view_keyexpr_t>,
    expr: *mut c_char,
) -> z_result_t {
    if expr.is_null() {
        this.as_rust_type_mut_uninit().write(None);
        result::Z_EINVAL
    } else {
        let mut len = libc::strlen(expr);
        let res = z_view_keyexpr_from_substr_autocanonize(this, expr, &mut len);
        if res == result::Z_OK {
            *expr.add(len) = 0;
        }
        res
    }
}

/// Constructs a `z_view_keyexpr_t` by aliasing a substring without checking any of `z_view_keyexpr_t`'s assertions:
///
/// - `start` MUST be valid UTF8.
/// - `start` MUST follow the Key Expression specification, i.e.:
///  - MUST NOT contain ``//``, MUST NOT start nor end with ``/``, MUST NOT contain any of the characters ``?#$``.
///  - any instance of ``**`` may only be lead or followed by ``/``.
///  - the key expression must have canon form.
///
/// `start` must outlive constructed key expression.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_view_keyexpr_from_substr_unchecked(
    this: &mut MaybeUninit<z_view_keyexpr_t>,
    start: *const c_char,
    len: usize,
) {
    if start.is_null() {
        this.as_rust_type_mut_uninit().write(None);
        return;
    }
    let name = std::slice::from_raw_parts(start as _, len);
    let name = std::str::from_utf8_unchecked(name);
    let name: KeyExpr = keyexpr::from_str_unchecked(name).into();
    this.as_rust_type_mut_uninit().write(Some(name));
}

/// Constructs a `z_view_keyexpr_t` by aliasing a string without checking any of `z_view_keyexpr_t`'s assertions:
///
///  - `s` MUST be valid UTF8.
///  - `s` MUST follow the Key Expression specification, i.e.:
///   - MUST NOT contain `//`, MUST NOT start nor end with `/`, MUST NOT contain any of the characters `?#$`.
///   - any instance of `**` may only be lead or followed by `/`.
///   - the key expression must have canon form.
///
/// `s` must outlive constructed key expression.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_view_keyexpr_from_str_unchecked(
    this: &mut MaybeUninit<z_view_keyexpr_t>,
    s: *const c_char,
) {
    let len = if s.is_null() { 0 } else { libc::strlen(s) };
    z_view_keyexpr_from_substr_unchecked(this, s, len)
}

/// Constructs a non-owned non-null-terminated string from key expression.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_as_view_string(
    this: &z_loaned_keyexpr_t,
    out_string: &mut MaybeUninit<z_view_string_t>,
) {
    let this = this.as_rust_type_ref();
    unsafe {
        z_view_string_from_substr(
            out_string,
            this.as_bytes().as_ptr() as _,
            this.as_bytes().len(),
        )
    };
}

/// Constructs and declares a key expression on the network. This reduces key key expression to a numerical id,
/// which allows to save the bandwith, when passing key expression between Zenoh entities.
///
/// @param this_: An uninitialized location in memory where key expression will be constructed.
/// @param session: Session on which to declare key expression.
/// @param key_expr: Key expression to declare on network.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_declare_keyexpr(
    this: &mut MaybeUninit<z_owned_keyexpr_t>,
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let key_expr = key_expr.as_rust_type_ref();
    let session = session.as_rust_type_ref();
    match session.declare_keyexpr(key_expr).wait() {
        Ok(id) => {
            this.write(Some(id.into_owned()));
            result::Z_OK
        }
        Err(e) => {
            tracing::debug!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
    }
}

/// Undeclares the key expression generated by a call to `z_declare_keyexpr()`.
/// The key expression is consumed.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_undeclare_keyexpr(
    this: &mut z_moved_keyexpr_t,
    session: &z_loaned_session_t,
) -> result::z_result_t {
    let Some(kexpr) = this.take_rust_type() else {
        tracing::debug!("Attempted to undeclare dropped keyexpr");
        return result::Z_EINVAL;
    };
    let session = session.as_rust_type_ref();
    match session.undeclare(kexpr).wait() {
        Ok(()) => result::Z_OK,
        Err(e) => {
            tracing::debug!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// Returns ``true`` if both ``left`` and ``right`` are equal, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_keyexpr_equals(left: &z_loaned_keyexpr_t, right: &z_loaned_keyexpr_t) -> bool {
    let l = left.as_rust_type_ref();
    let r = right.as_rust_type_ref();
    *l == *r
}

/// Returns ``true`` if the keyexprs intersect, i.e. there exists at least one key which is contained in both of the
/// sets defined by ``left`` and ``right``, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_keyexpr_intersects(
    left: &z_loaned_keyexpr_t,
    right: &z_loaned_keyexpr_t,
) -> bool {
    let l = left.as_rust_type_ref();
    let r = right.as_rust_type_ref();
    l.intersects(r)
}

/// Returns ``true`` if ``left`` includes ``right``, i.e. the set defined by ``left`` contains every key belonging to the set
/// defined by ``right``, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_keyexpr_includes(
    left: &z_loaned_keyexpr_t,
    right: &z_loaned_keyexpr_t,
) -> bool {
    let l = left.as_rust_type_ref();
    let r = right.as_rust_type_ref();
    l.includes(r)
}

/// Constructs key expression by concatenation of key expression in `left` with a string in `right`.
/// Returns 0 in case of success, negative error code otherwise.
///
/// You should probably prefer `z_keyexpr_join` as Zenoh may then take advantage of the hierachical separation it inserts.
/// To avoid odd behaviors, concatenating a key expression starting with `*` to one ending with `*` is forbidden by this operation,
/// as this would extremely likely cause bugs.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_keyexpr_concat(
    this: &mut MaybeUninit<z_owned_keyexpr_t>,
    left: &z_loaned_keyexpr_t,
    right_start: *const c_char,
    right_len: usize,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let left = left.as_rust_type_ref();
    let right = std::slice::from_raw_parts(right_start as _, right_len);
    let right = match std::str::from_utf8(right) {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(
                "Couldn't concatenate {:02x?} to {} because it is not valid UTF8: {}",
                right,
                left,
                e
            );
            this.write(None);
            return result::Z_EINVAL;
        }
    };
    match left.concat(right) {
        Ok(result) => {
            this.write(Some(result));
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
    }
}

/// Constructs key expression by performing path-joining (automatically inserting) of `left` with `right`.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_keyexpr_join(
    this: &mut MaybeUninit<z_owned_keyexpr_t>,
    left: &z_loaned_keyexpr_t,
    right: &z_loaned_keyexpr_t,
) -> result::z_result_t {
    let left = left.as_rust_type_ref();
    let right = right.as_rust_type_ref();
    let this = this.as_rust_type_mut_uninit();
    match left.join(right.as_str()) {
        Ok(result) => {
            this.write(Some(result));
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
    }
}
#[cfg(feature = "unstable")]
/// Intersection level of 2 key expressions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum z_keyexpr_intersection_level_t {
    /// 2 key expressions do not intersect.
    DISJOINT = 0,
    /// 2 key expressions intersect, i.e. there exists at least one key expression that is included by both.
    INTERSECTS = 1,
    /// First key expression is the superset of second one.
    INCLUDES = 2,
    /// 2 key expressions are equal.
    EQUALS = 3,
}
#[cfg(feature = "unstable")]
impl From<SetIntersectionLevel> for z_keyexpr_intersection_level_t {
    fn from(val: SetIntersectionLevel) -> Self {
        match val {
            SetIntersectionLevel::Disjoint => z_keyexpr_intersection_level_t::DISJOINT,
            SetIntersectionLevel::Intersects => z_keyexpr_intersection_level_t::INTERSECTS,
            SetIntersectionLevel::Includes => z_keyexpr_intersection_level_t::INCLUDES,
            SetIntersectionLevel::Equals => z_keyexpr_intersection_level_t::EQUALS,
        }
    }
}
#[cfg(feature = "unstable")]
#[no_mangle]
/// Returns the relation between `left` and `right` from `left`'s point of view.
///
/// Note that this is slower than `z_keyexpr_intersects` and `keyexpr_includes`, so you should favor these methods for most applications.
pub extern "C" fn z_keyexpr_relation_to(
    left: &z_loaned_keyexpr_t,
    right: &z_loaned_keyexpr_t,
) -> z_keyexpr_intersection_level_t {
    let l = left.as_rust_type_ref();
    let r = right.as_rust_type_ref();
    l.relation_to(r).into()
}
