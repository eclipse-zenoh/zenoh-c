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

use std::borrow::Cow;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::ptr::null;
use std::slice::from_raw_parts;

use libc::{c_char, c_void, strlen};

use crate::errors;
use crate::transmute::{
    unwrap_ref_unchecked, unwrap_ref_unchecked_mut, Inplace, TransmuteFromHandle,
    TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
};

pub use crate::opaque_types::z_loaned_slice_t;
pub use crate::opaque_types::z_owned_slice_t;
pub use crate::opaque_types::z_view_slice_t;

decl_transmute_owned!(Option<Box<[u8]>>, z_owned_slice_t);
decl_transmute_owned!(Option<&'static [u8]>, z_view_slice_t);
decl_transmute_handle!(&'static [u8], z_loaned_slice_t);

/// Returns an empty `z_view_slice_t`
#[no_mangle]
pub extern "C" fn z_view_slice_empty(this: *mut MaybeUninit<z_view_slice_t>) {
    let slice: &'static [u8] = &[];
    Inplace::init(this.transmute_uninit_ptr(), Some(slice))
}

#[no_mangle]
pub extern "C" fn z_view_slice_null(this: *mut MaybeUninit<z_view_slice_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// Calling this with `str == NULL` is equivalent to `z_view_slice_null`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_slice_from_str(
    this: *mut MaybeUninit<z_view_slice_t>,
    str: *const c_char,
) {
    if str.is_null() {
        z_view_slice_null(this)
    } else {
        z_view_slice_wrap(this, str as *const u8, libc::strlen(str))
    }
}

/// Constructs a `len` bytes long view starting at `start`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_slice_wrap(
    this: *mut MaybeUninit<z_view_slice_t>,
    start: *const u8,
    len: usize,
) {
    if len == 0 {
        z_view_slice_empty(this)
    } else if start.is_null() {
        z_view_slice_null(this)
    } else {
        let slice: &'static [u8] = from_raw_parts(start, len);
        Inplace::init(this.transmute_uninit_ptr(), Some(slice))
    }
}

#[no_mangle]
pub extern "C" fn z_view_slice_loan(this: &z_view_slice_t) -> *const z_loaned_slice_t {
    match this.transmute_ref() {
        Some(s) => s.transmute_handle(),
        None => null(),
    }
}

/// Returns an empty `z_owned_slice_t`
#[no_mangle]
pub extern "C" fn z_slice_empty(this: *mut MaybeUninit<z_owned_slice_t>) {
    let slice = Box::new([]);
    Inplace::init(this.transmute_uninit_ptr(), Some(slice))
}

#[no_mangle]
pub extern "C" fn z_slice_null(this: *mut MaybeUninit<z_owned_slice_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Copies a string into `z_owned_slice_t` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// Calling this with `str == NULL` is equivalent to `z_slice_null`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_from_str(
    this: *mut MaybeUninit<z_owned_slice_t>,
    str: *const c_char,
) {
    if str.is_null() {
        z_slice_null(this)
    } else {
        z_slice_wrap(this, str as *const u8, libc::strlen(str))
    }
}

/// Constructs a `len` bytes long view starting at `start`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_wrap(
    this: *mut MaybeUninit<z_owned_slice_t>,
    start: *const u8,
    len: usize,
) {
    if len == 0 {
        z_slice_empty(this)
    } else if start.is_null() {
        z_slice_null(this)
    } else {
        let slice = from_raw_parts(start, len).to_owned().into_boxed_slice();
        Inplace::init(this.transmute_uninit_ptr(), Some(slice))
    }
}

/// Frees `this` and invalidates it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_drop(this: &mut z_owned_slice_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

#[no_mangle]
pub extern "C" fn z_slice_loan(this: &z_owned_slice_t) -> *const z_loaned_slice_t {
    match this.transmute_ref() {
        Some(s) => (&s.as_ref()) as *const &[u8] as *const z_loaned_slice_t,
        None => null(),
    }
}

#[no_mangle]
pub extern "C" fn z_slice_clone(this: &z_loaned_slice_t, dst: *mut MaybeUninit<z_owned_slice_t>) {
    let slice = this.transmute_ref().to_vec().into_boxed_slice();
    Inplace::init(dst.transmute_uninit_ptr(), Some(slice));
}

/// Returns ``true`` if `this` is initialized.
#[no_mangle]
pub extern "C" fn z_owned_slice_check(this: &z_owned_slice_t) -> bool {
    this.transmute_ref().is_some()
}

/// Returns ``true`` if `this` is initialized.
#[no_mangle]
pub extern "C" fn z_view_slice_check(this: &z_view_slice_t) -> bool {
    this.transmute_ref().is_some()
}

#[no_mangle]
pub extern "C" fn z_slice_len(this: &z_loaned_slice_t) -> usize {
    this.transmute_ref().len()
}

#[no_mangle]
pub extern "C" fn z_slice_data(this: &z_loaned_slice_t) -> *const u8 {
    this.transmute_ref().as_ptr()
}

pub use crate::opaque_types::z_loaned_str_t;
pub use crate::opaque_types::z_owned_str_t;
pub use crate::opaque_types::z_view_str_t;

decl_transmute_owned!(custom_inplace_init Option<Box<[u8]>>, z_owned_str_t);
decl_transmute_owned!(custom_inplace_init Option<&'static [u8]>, z_view_str_t);
decl_transmute_handle!(&'static [u8], z_loaned_str_t);

/// Frees `z_owned_str_t`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_str_drop(this: &mut z_owned_str_t) {
    z_slice_drop(this.transmute_mut().transmute_mut());
}

/// Returns ``true`` if `s` is a valid string
#[no_mangle]
pub extern "C" fn z_str_check(this: &z_owned_str_t) -> bool {
    z_owned_slice_check(this.transmute_ref().transmute_ref())
}

/// Returns undefined `z_owned_str_t`
#[no_mangle]
pub extern "C" fn z_str_null(this: *mut MaybeUninit<z_owned_str_t>) {
    z_slice_null(this as *mut _)
}

/// Returns undefined `z_owned_str_t`
#[no_mangle]
pub extern "C" fn z_view_str_null(this: *mut MaybeUninit<z_view_str_t>) {
    z_slice_null(this as *mut _)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_str_empty(this: *mut MaybeUninit<z_owned_str_t>) {
    z_slice_wrap(this as *mut _, [0u8].as_ptr(), 1)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_str_empty(this: *mut MaybeUninit<z_view_str_t>) {
    z_view_slice_wrap(this as *mut _, [0u8].as_ptr(), 1)
}

/// Returns :c:type:`z_loaned_str_t` structure loaned from :c:type:`z_owned_str_t`.
#[no_mangle]
pub extern "C" fn z_str_loan(this: &z_owned_str_t) -> *const z_loaned_str_t {
    z_slice_loan(this.transmute_ref().transmute_ref()) as _
}

/// Returns :c:type:`z_loaned_str_t` structure loaned from :c:type:`z_view_str_t`.
#[no_mangle]
pub extern "C" fn z_view_str_loan(this: &z_view_str_t) -> *const z_loaned_str_t {
    z_view_slice_loan(this.transmute_ref().transmute_ref()) as _
}

/// Copies a string into `z_owned_str_t` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// Calling this with `str == NULL` is equivalent to `z_str_null`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_str_wrap(
    this: *mut MaybeUninit<z_owned_slice_t>,
    str: *const libc::c_char,
) {
    z_slice_wrap(this as *mut _, str as _, strlen(str) + 1)
}

/// Copies a a substring of length `len`into `z_owned_str_t`.
///
/// Calling this with `str == NULL` is equivalent to `z_str_null`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_str_from_substring(
    this: *mut MaybeUninit<z_owned_str_t>,
    str: *const libc::c_char,
    len: usize,
) {
    let mut v = vec![0u8; len + 1];
    v[0..len].copy_from_slice(from_raw_parts(str as *const u8, len));
    let b = v.into_boxed_slice();
    Inplace::init(this.transmute_uninit_ptr(), Some(b));
}

/// Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// Calling this with `str == NULL` is equivalent to `z_view_str_null`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_str_wrap(
    this: *mut MaybeUninit<z_view_slice_t>,
    str: *const libc::c_char,
) {
    z_view_slice_wrap(this as *mut _, str as _, strlen(str) + 1)
}

#[no_mangle]
pub extern "C" fn z_view_str_len(this: &z_loaned_str_t) -> usize {
    z_slice_len(this.transmute_ref().transmute_handle()) - 1
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_view_str_data(this: &z_loaned_str_t) -> *const libc::c_char {
    z_slice_data(this.transmute_ref().transmute_handle()) as _
}

#[no_mangle]
pub extern "C" fn z_str_clone(this: &z_loaned_str_t, dst: *mut MaybeUninit<z_owned_str_t>) {
    let slice = this.transmute_ref().to_vec().into_boxed_slice();
    Inplace::init(dst.transmute_uninit_ptr(), Some(slice));
}

pub use crate::opaque_types::z_loaned_slice_map_t;
pub use crate::opaque_types::z_owned_slice_map_t;

pub type ZHashMap = HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>;
pub use crate::opaque_types::z_loaned_config_t;
decl_transmute_handle!(
    HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>,
    z_loaned_slice_map_t
);

pub use crate::opaque_types::z_owned_config_t;
decl_transmute_owned!(
    Option<HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>>,
    z_owned_slice_map_t
);

/// Constructs a new empty map.
#[no_mangle]
pub extern "C" fn z_slice_map_new(this: *mut MaybeUninit<z_owned_slice_map_t>) {
    let this = this.transmute_uninit_ptr();
    let map = ZHashMap::new();
    Inplace::init(this, Some(map));
}

/// Constructs the gravestone value for `z_owned_slice_map_t`
#[no_mangle]
pub extern "C" fn z_slice_map_null(this: *mut MaybeUninit<z_owned_slice_map_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Returns `true` if the map is not in its gravestone state
#[no_mangle]
pub extern "C" fn z_slice_map_check(map: &z_owned_slice_map_t) -> bool {
    let map = map.transmute_ref();
    map.as_ref().is_some()
}

/// Destroys the map, resetting `this` to its gravestone value.
///
/// This function is double-free safe, passing a pointer to the gravestone value will have no effect.
#[no_mangle]
pub extern "C" fn z_slice_map_drop(this: &mut z_owned_slice_map_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

#[no_mangle]
pub extern "C" fn z_slice_map_loan(this: &z_owned_slice_map_t) -> &z_loaned_slice_map_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

#[no_mangle]
pub extern "C" fn z_slice_map_loan_mut(
    this: &mut z_owned_slice_map_t,
) -> &mut z_loaned_slice_map_t {
    let this = this.transmute_mut();
    let this = unwrap_ref_unchecked_mut(this);
    this.transmute_handle_mut()
}

/// Returns number of key-value pairs in the map.
#[no_mangle]
pub extern "C" fn z_slice_map_len(this: &z_loaned_slice_map_t) -> usize {
    this.transmute_ref().len()
}

/// Returns true if the map is empty, false otherwise.
#[no_mangle]
pub extern "C" fn z_slice_map_is_empty(this: &z_loaned_slice_map_t) -> bool {
    z_slice_map_len(this) == 0
}

/// The body of a loop over a z_slice_map's key-value pairs.
///
/// `key` and `value` are loaned to the body for the duration of a single call.
/// `context` is passed transparently through the iteration driver.
///
/// Returning `true` is treated as `continue`.
#[allow(non_camel_case_types)]
pub type z_slice_map_iter_body_t =
    extern "C" fn(key: &z_loaned_slice_t, value: &z_loaned_slice_t, context: *mut c_void) -> bool;

#[no_mangle]
pub extern "C" fn z_slice_map_iterate(
    this: &z_loaned_slice_map_t,
    body: z_slice_map_iter_body_t,
    context: *mut c_void,
) {
    let this = this.transmute_ref();
    for (key, value) in this {
        let key_slice = key.as_ref();
        let value_slice = value.as_ref();
        if !body(
            key_slice.transmute_handle(),
            value_slice.transmute_handle(),
            context,
        ) {
            break;
        }
    }
}

/// Returns the value associated with `key`.
///
/// Will return NULL if the key is not present in the map.
#[no_mangle]
pub extern "C" fn z_slice_map_get(
    this: &z_loaned_slice_map_t,
    key: &z_loaned_slice_t,
) -> *const z_loaned_slice_t {
    let m = this.transmute_ref();
    let key = *key.transmute_ref();
    let k = Cow::Borrowed(key);
    m.get(&k)
        .map(|s| s.as_ref().transmute_handle() as *const _)
        .unwrap_or(null())
}

/// Associates `value` to `key` in the map, copying them to obtain ownership: `key` and `value` are not aliased past the function's return.
///
/// Returns 1 if there was already an entry associated with the key, 0 otherwise.
#[no_mangle]
pub extern "C" fn z_slice_map_insert_by_copy(
    this: &mut z_loaned_slice_map_t,
    key: &z_loaned_slice_t,
    value: &z_loaned_slice_t,
) -> u8 {
    let this = this.transmute_mut();
    let key = *key.transmute_ref();
    let value = *value.transmute_ref();
    match this.insert(Cow::Owned(key.to_owned()), Cow::Owned(value.to_owned())) {
        Some(_) => 1,
        None => 0,
    }
}

/// Associates `value` to `key` in the map, aliasing them.
///
/// Note that once `key` is aliased, reinserting at the same key may alias the previous instance, or the new instance of `key`.
///
/// Returns 1 if there was already an entry associated with the key, 0 otherwise.
#[no_mangle]
pub extern "C" fn z_slice_map_insert_by_alias(
    this: &mut z_loaned_slice_map_t,
    key: &z_loaned_slice_t,
    value: &z_loaned_slice_t,
) -> errors::z_error_t {
    let this = this.transmute_mut();
    let key = key.transmute_ref();
    let value = value.transmute_ref();
    match this.insert(Cow::Borrowed(key), Cow::Borrowed(value)) {
        Some(_) => 1,
        None => 0,
    }
}
