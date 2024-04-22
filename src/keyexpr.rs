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
use crate::errors::ZCError;
use crate::errors::Z_OK;
use crate::transmute::unwrap_ref_unchecked;
use crate::transmute::Inplace;
use crate::transmute::TransmuteCopy;
use crate::transmute::TransmuteFromHandle;
use crate::transmute::TransmuteIntoHandle;
use crate::transmute::TransmuteRef;
use crate::transmute::TransmuteUninitPtr;
use crate::z_slice_t;
use crate::z_owned_str_t;
use crate::z_session_t;
use libc::c_char;
use zenoh::core::SyncResolve;
use zenoh::key_expr::SetIntersectionLevel;
use zenoh::prelude::keyexpr;
use zenoh::prelude::KeyExpr;
use std::error::Error;

pub use crate::opaque_types::z_owned_keyexpr_t;
decl_transmute_owned!(Option<KeyExpr<'static>>, z_owned_keyexpr_t);

/// Constructs a null safe-to-drop value of 'z_owned_keyexpr_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_keyexpr_null(this: *mut MaybeUninit<z_owned_keyexpr_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

fn keyexpr_create_inner(name: &'static mut str, should_auto_canonize: bool, should_copy: bool) -> Result<KeyExpr<'static>, Box<dyn Error + Send + Sync>> {
    if should_copy {
        let s = name.to_owned();
        match should_auto_canonize {
            true => KeyExpr::<'static>::autocanonize(s),
            false => KeyExpr::<'static>::try_from(s),
        }
    } else {
        match should_auto_canonize {
            true => KeyExpr::<'static>::autocanonize(name),
            false => KeyExpr::<'static>::try_from(name),
        }
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
unsafe fn keyexpr_create(name: &'static mut [u8], should_auto_canonize: bool, should_copy: bool) -> Result<KeyExpr<'static>, errors::ZCError> {
    match std::str::from_utf8_mut(name) {
        Ok(name) => {
            match keyexpr_create_inner(name, should_auto_canonize, should_copy) {
                Ok(v) => {
                    Ok(v)
                }
                Err(e) => {
                    log::error!("Couldn't construct a keyexpr: {}", e);
                    Err(errors::Z_EINVAL)
                }
            }
        },
        Err(e) => {
            log::error!("{}", e);
            Err(errors::Z_EPARSE)
        }
    }
}
   


/// Constructs a :c:type:`z_owned_keyexpr_t` departing from a string, copying the passed string.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_new(
    name: *const c_char,
    this: *mut MaybeUninit<z_owned_keyexpr_t>,
) -> errors::ZCError {
    if name.is_null() {
        return errors::Z_EINVAL;
    }
    let name = std::slice::from_raw_parts_mut(name as _, libc::strlen(name));
    let this = this.transmute_uninit_ptr();
    match keyexpr_create(name, false, true) {
        Ok(ke) => {
            Inplace::init(this, Some(ke));
            errors::Z_OK
        },
        Err(e) => {
            Inplace::empty(this);
            e
        }
    }
}

/// Constructs a :c:type:`z_owned_keyexpr_t` departing from a string, copying the passed string. The copied string is canonized.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_new_autocanonize(
    name: *const c_char,
    this: *mut MaybeUninit<z_owned_keyexpr_t>,
) -> ZCError {
    if name.is_null() {
        return errors::Z_EINVAL;
    }
    let name = std::slice::from_raw_parts_mut(name as _, libc::strlen(name));
    let this = this.transmute_uninit_ptr();
    match keyexpr_create(name, true, true) {
        Ok(ke) => {
            Inplace::init(this, Some(ke));
            errors::Z_OK
        },
        Err(e) => {
            Inplace::empty(this);
            e
        }
    }
}

/// Returns a :c:type:`z_keyexpr_t` loaned from :c:type:`z_owned_keyexpr_t`.
#[no_mangle]
pub extern "C" fn z_keyexpr_loan(key_expr: &'static z_owned_keyexpr_t) -> z_keyexpr_t {
    unwrap_ref_unchecked(key_expr.transmute_ref()).transmute_handle()
}

/// Frees `keyexpr` and invalidates it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_keyexpr_drop(keyexpr: &mut z_owned_keyexpr_t) {
    Inplace::drop(keyexpr.transmute_mut());
}

/// Returns ``true`` if `keyexpr` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_keyexpr_check(keyexpr: &z_owned_keyexpr_t) -> bool {
    keyexpr.transmute_ref().is_some()
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
pub use crate::opaque_types::z_keyexpr_t;
decl_transmute_handle!(KeyExpr<'static>, z_keyexpr_t);

#[derive(Debug, Clone, Copy)]
pub struct UninitializedKeyExprError;
impl std::fmt::Display for UninitializedKeyExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Uninitialized Key Expression detected, make sure you use `z_keyexpr_check` or `z_loaned_keyexpr_check` after constructing your key expressions")
    }
}
impl std::error::Error for UninitializedKeyExprError {}

/// Returns ``0`` if the passed string is a valid (and canon) key expression.
/// Otherwise returns error value
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_is_canon(start: *const c_char, len: usize) -> ZCError {
    let name = std::slice::from_raw_parts_mut(start as _, len);
    match keyexpr_create(name, false, false) {
        Ok(_) => errors::Z_OK,
        Err(e) => e,
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
pub unsafe extern "C" fn z_keyexpr_canonize_null_terminated(start: *mut c_char) -> ZCError {
    let mut len = libc::strlen(start);
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
/// Returns ``0`` upon success, negative values upon failure.  
/// Returns a negative value if canonization failed, which indicates that the passed string was an invalid
/// key expression for reasons other than a non-canon form.
///
/// May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_canonize(start: *mut c_char, len: &mut usize) -> ZCError {
    let name = std::slice::from_raw_parts_mut(start as _, *len);
    match keyexpr_create(name, true, false) {
        Ok(ke) => {
            *len = ke.len();
            errors::Z_OK
        },
        Err(e) => e,
    }
}

/// Constructs a :c:type:`z_keyexpr_t` by aliasing a string.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_keyexpr_from_slice(this: *mut MaybeUninit<z_owned_keyexpr_t>, name: *const c_char, len: usize) -> ZCError {
    let this = this.transmute_uninit_ptr();
    if name.is_null() {
        Inplace::empty(this);
        return errors::Z_EINVAL;
    }
    let name = std::slice::from_raw_parts_mut(name as _, len);
   
    match keyexpr_create(name, false, false) {
        Ok(ke) => {
            Inplace::init(this, Some(ke));
            errors::Z_OK
        },
        Err(e) => {
            Inplace::empty(this);
            e
        }
    }
}

/// Constructs a :c:type:`z_owned_keyexpr_t` by aliasing a string.
/// The string is canonized in-place before being passed to keyexpr.
/// May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_keyexpr_from_slice_autocanonize(
    this: *mut MaybeUninit<z_owned_keyexpr_t>,
    name: *mut c_char,
    len: &mut usize,
) -> ZCError {
    let this = this.transmute_uninit_ptr();
    if name.is_null() {
        Inplace::empty(this);
        return errors::Z_EINVAL;
    }
    let name = std::slice::from_raw_parts_mut(name as _, libc::strlen(name));
    
    match keyexpr_create(name, true, false) {
        Ok(ke) => {
            *len = ke.len();
            Inplace::init(this, Some(ke));
            errors::Z_OK
        },
        Err(e) => {
            Inplace::empty(this);
            e
        }
    }
}

/// Constructs a :c:type:`z_keyexpr_t` departing from a string.
/// It is a loaned key expression that aliases `name`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr(this: *mut MaybeUninit<z_owned_keyexpr_t>, name: *const c_char) -> ZCError {
    if name.is_null() {
        Inplace::empty(this.transmute_uninit_ptr());
        return errors::Z_EINVAL;
    } else {
        let len = libc::strlen(name);
        zc_keyexpr_from_slice(this, name, len)
    }
}

/// Constructs a :c:type:`z_owned_keyexpr_t` by aliasing a string.
/// The string is canonized in-place before being passed to keyexpr.
/// May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_autocanonize(this: *mut MaybeUninit<z_owned_keyexpr_t>, name: *mut c_char) -> ZCError {
    if name.is_null() {
        Inplace::empty(this.transmute_uninit_ptr());
        return errors::Z_EINVAL;
    } else {
        let mut len = libc::strlen(name);
        let res = zc_keyexpr_from_slice_autocanonize(this, name, &mut len);
        if res == errors::Z_OK {
            *name.add(len) = 0;
        }
        res
    }
}

/// Constructs a :c:type:`z_owned_eyexpr_t` by aliasing a string without checking any of `z_keyexpr_t`'s assertions:
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
    this: *mut MaybeUninit<z_owned_keyexpr_t>,
    start: *const c_char,
    len: usize,
) {
    let name = std::slice::from_raw_parts(start as _, len);
    let name = std::str::from_utf8_unchecked(name);
    let name: KeyExpr = keyexpr::from_str_unchecked(name).into();
    Inplace::init(this.transmute_uninit_ptr(), Some(name));
}

/// Constructs a :c:type:`z_owned_keyexpr_t` by aliasing a string without checking any of `z_keyexpr_t`'s assertions:
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
pub unsafe extern "C" fn z_keyexpr_unchecked(this: *mut MaybeUninit<z_owned_keyexpr_t>, name: *const c_char) {
    zc_keyexpr_from_slice_unchecked(this, name, libc::strlen(name))
}

/// Constructs a null-terminated string departing from a :c:type:`z_keyexpr_t`.
/// The user is responsible of droping the returned string using `z_drop`
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_keyexpr_to_string(ke: z_keyexpr_t) -> z_owned_str_t {
    ke.transmute_ref().as_bytes().into()
}

/// Returns the key expression's internal string by aliasing it.
///
/// Currently exclusive to zenoh-c
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_keyexpr_as_bytes(ke: z_keyexpr_t) -> z_slice_t {
    let ke = ke.transmute_ref();
    z_slice_t {
        start: ke.as_ptr(),
        len: ke.len()
    }
}

impl<'a> From<&'static KeyExpr<'static>> for z_keyexpr_t {
    fn from(key: &'static KeyExpr<'static>) -> Self {
        key.transmute_handle()
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
    this: *mut MaybeUninit<z_owned_keyexpr_t>,
    session: z_session_t,
    key_expr: z_keyexpr_t,
) -> ZCError {
    let this = this.transmute_uninit_ptr();
    let key_expr = key_expr.transmute_ref();
    let session = session.transmute_copy();
    match session.declare_keyexpr(key_expr).res_sync() {
        Ok(id) => {
            Inplace::init(this, Some(id.into_owned()));
            errors::Z_OK
        }
        Err(e) => {
            log::debug!("{}", e);
            Inplace::empty(this);
            errors::Z_EGENERIC
        }
    }
}

/// Undeclare the key expression generated by a call to :c:func:`z_declare_keyexpr`.
/// The keyxpr is consumed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_undeclare_keyexpr(session: z_session_t, kexpr: &mut z_owned_keyexpr_t) -> i8 {
    let Some(kexpr) = kexpr.transmute_mut().take() else {
        log::debug!("Attempted to undeclare dropped keyexpr");
        return i8::MIN;
    };
    let session = session.transmute_copy();
    match session.undeclare(kexpr).res() {
        Ok(()) => errors::Z_OK,
        Err(e) => {
            log::debug!("{}", e);
            errors::Z_EGENERIC
        }
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// Returns ``0`` if both ``left`` and ``right`` are equal.
pub extern "C" fn z_keyexpr_equals(left: z_keyexpr_t, right: z_keyexpr_t) -> bool {
    let l = left.transmute_ref();
    let r = right.transmute_ref();
    *l == *r
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// Returns ``0`` if the keyexprs intersect, i.e. there exists at least one key which is contained in both of the
/// sets defined by ``left`` and ``right``.
pub extern "C" fn z_keyexpr_intersects(left: z_keyexpr_t, right: z_keyexpr_t) -> bool {
    let l = left.transmute_ref();
    let r = right.transmute_ref();
    l.intersects(r)
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// Returns ``0`` if ``left`` includes ``right``, i.e. the set defined by ``left`` contains every key belonging to the set
/// defined by ``right``.
pub extern "C" fn z_keyexpr_includes(left: z_keyexpr_t, right: z_keyexpr_t) -> bool {
    let l = left.transmute_ref();
    let r = right.transmute_ref();
    l.includes(r)
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
    this: *mut MaybeUninit<z_owned_keyexpr_t>
) -> errors::ZCError {
    let this = this.transmute_uninit_ptr();
    let left = left.transmute_ref();
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
            Inplace::empty(this);
            return errors::Z_EINVAL;
        }
    };
    match left.concat(right) {
        Ok(result) => {
            Inplace::init(this, Some(result));
            errors::Z_OK
        }
        Err(e) => {
            log::error!("{}", e);
            Inplace::empty(this);
            errors::Z_EGENERIC
        }
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// Performs path-joining (automatically inserting) and returns the result as a `z_owned_keyexpr_t`.
/// In case of error, the return value will be set to its invalidated state.
pub extern "C" fn z_keyexpr_join(left: z_keyexpr_t, right: z_keyexpr_t, this: *mut MaybeUninit<z_owned_keyexpr_t>) -> errors::ZCError {
    let left = left.transmute_ref();
    let right = right.transmute_ref();
    let this = this.transmute_uninit_ptr();
    match left.join(right.as_str()) {
        Ok(result) => {
            Inplace::init(this, Some(result));
            errors::Z_OK
        }
        Err(e) => {
            log::error!("{}", e);
            Inplace::empty(this);
            errors::Z_EGENERIC
        }
    }
}

/// A :c:type:`z_keyexpr_intersection_level_t`.
///
///     - **Z_KEYEXPR_INTERSECTION_LEVEL_DISJOINT**
///     - **Z_KEYEXPR_INTERSECTION_LEVEL_INTERSECTS**
///     - **Z_KEYEXPR_INTERSECTION_LEVEL_INCLUDES**
///     - **Z_KEYEXPR_INTERSECTION_LEVEL_EQUALS**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum z_keyexpr_intersection_level_t {
    DISJOINT = 0,
    INTERSECTS = 1,
    INCLUDES = 2,
    EQUALS = 3,
}

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

#[no_mangle]
/// Returns the relation between `left` and `right` from `left`'s point of view.
///
/// Note that this is slower than `z_keyexpr_intersects` and `keyexpr_includes`, so you should favor these methods for most applications.
pub extern "C" fn z_keyexpr_relation_to(
    left: z_keyexpr_t,
    right: z_keyexpr_t,
) -> z_keyexpr_intersection_level_t {
    let l = left.transmute_ref();
    let r = right.transmute_ref();
    l.relation_to(r).into()
}
