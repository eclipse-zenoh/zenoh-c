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

use std::collections::HashMap;
use std::hash::Hash;
use std::mem::MaybeUninit;
use std::ptr::{null, slice_from_raw_parts};
use std::slice::from_raw_parts;

use libc::{c_char, c_void, strlen};

use crate::errors;
use crate::transmute::{
    unwrap_ref_unchecked, unwrap_ref_unchecked_mut, Inplace, TransmuteFromHandle,
    TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
};

pub struct CSlice(*const u8, isize);

impl CSlice {
    pub fn new_borrowed(data: *const u8, len: usize) -> Self {
        let len: isize = len as isize;
        CSlice(data, -len)
    }

    pub fn new_borrowed_from_slice(slice: &[u8]) -> Self {
        Self::new_borrowed(slice.as_ptr(), slice.len())
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_owned(data: *const u8, len: usize) -> Self {
        if len == 0 {
            return CSlice::default();
        }
        let b = unsafe { from_raw_parts(data, len).to_vec().into_boxed_slice() };
        let slice = Box::leak(b);
        CSlice(slice.as_ptr(), slice.len() as isize)
    }

    pub fn slice(&self) -> &'static [u8] {
        if self.1 == 0 {
            return &[0u8; 0];
        }
        unsafe { from_raw_parts(self.0, self.1.unsigned_abs()) }
    }

    pub fn data(&self) -> *const u8 {
        self.0
    }

    pub fn len(&self) -> usize {
        self.1.unsigned_abs()
    }

    pub fn is_empty(&self) -> bool {
        self.1 == 0
    }

    pub fn is_owned(&self) -> bool {
        self.1 > 0
    }

    pub fn shallow_copy(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl Default for CSlice {
    fn default() -> Self {
        Self(null(), 0)
    }
}

impl Drop for CSlice {
    fn drop(&mut self) {
        if !self.is_owned() {
            return;
        }
        let b = unsafe { Box::from_raw(slice_from_raw_parts(self.data(), self.len()).cast_mut()) };
        std::mem::drop(b);
    }
}

impl Hash for CSlice {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.slice().hash(state);
    }
}

impl PartialEq for CSlice {
    fn eq(&self, other: &Self) -> bool {
        self.slice() == other.slice()
    }
}

impl Clone for CSlice {
    fn clone(&self) -> Self {
        unsafe { Self::new_owned(self.data(), self.len()) }
    }
}

impl From<Vec<u8>> for CSlice {
    fn from(value: Vec<u8>) -> Self {
        let slice = Box::leak(value.into_boxed_slice());
        CSlice(slice.as_ptr(), slice.len() as isize)
    }
}

impl std::cmp::Eq for CSlice {}

pub use crate::opaque_types::z_loaned_slice_t;
pub use crate::opaque_types::z_owned_slice_t;
pub use crate::opaque_types::z_view_slice_t;

decl_transmute_owned!(CSlice, z_owned_slice_t);
decl_transmute_owned!(custom_inplace_init CSlice, z_view_slice_t);
decl_transmute_handle!(CSlice, z_loaned_slice_t);

/// Returns an empty `z_view_slice_t`
#[no_mangle]
pub extern "C" fn z_view_slice_empty(this: *mut MaybeUninit<z_view_slice_t>) {
    Inplace::init(this.transmute_uninit_ptr(), CSlice::default())
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
        z_view_slice_empty(this)
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
    if len == 0 || start.is_null() {
        z_view_slice_empty(this)
    } else {
        Inplace::init(
            this.transmute_uninit_ptr(),
            CSlice::new_borrowed(start, len),
        )
    }
}

#[no_mangle]
pub extern "C" fn z_view_slice_loan(this: &z_view_slice_t) -> &z_loaned_slice_t {
    this.transmute_ref().transmute_handle()
}

/// Returns ``true`` if `this` is initialized.
#[no_mangle]
pub extern "C" fn z_view_slice_check(this: &z_view_slice_t) -> bool {
    !this.transmute_ref().is_empty()
}

/// Returns an empty `z_owned_slice_t`
#[no_mangle]
pub extern "C" fn z_slice_empty(this: *mut MaybeUninit<z_owned_slice_t>) {
    Inplace::init(this.transmute_uninit_ptr(), CSlice::default())
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
        z_slice_empty(this)
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
    if len == 0 || start.is_null() {
        z_slice_empty(this)
    } else {
        Inplace::init(this.transmute_uninit_ptr(), CSlice::new_owned(start, len))
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
pub extern "C" fn z_slice_loan(this: &z_owned_slice_t) -> &z_loaned_slice_t {
    this.transmute_ref().transmute_handle()
}

#[no_mangle]
pub extern "C" fn z_slice_clone(this: &z_loaned_slice_t, dst: *mut MaybeUninit<z_owned_slice_t>) {
    Inplace::init(dst.transmute_uninit_ptr(), this.transmute_ref().clone());
}

/// Returns ``true`` if `this` is initialized.
#[no_mangle]
pub extern "C" fn z_slice_check(this: &z_owned_slice_t) -> bool {
    !this.transmute_ref().is_empty()
}

#[no_mangle]
pub extern "C" fn z_slice_len(this: &z_loaned_slice_t) -> usize {
    this.transmute_ref().len()
}

#[no_mangle]
pub extern "C" fn z_slice_data(this: &z_loaned_slice_t) -> *const u8 {
    this.transmute_ref().data()
}

pub use crate::opaque_types::z_loaned_str_t;
pub use crate::opaque_types::z_owned_str_t;
pub use crate::opaque_types::z_view_str_t;

decl_transmute_owned!(custom_inplace_init CSlice, z_owned_str_t);
decl_transmute_owned!(custom_inplace_init CSlice, z_view_str_t);
decl_transmute_handle!(CSlice, z_loaned_str_t);

/// Frees `z_owned_str_t`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_str_drop(this: &mut z_owned_str_t) {
    z_slice_drop(this.transmute_mut().transmute_mut());
}

/// Returns ``true`` if `s` is a valid string
#[no_mangle]
pub extern "C" fn z_str_check(this: &z_owned_str_t) -> bool {
    z_slice_check(this.transmute_ref().transmute_ref())
}

/// Returns undefined `z_owned_str_t`
#[no_mangle]
pub extern "C" fn z_str_null(this: *mut MaybeUninit<z_owned_str_t>) {
    z_slice_empty(this as *mut _)
}

/// Returns ``true`` if `s` is a valid string
#[no_mangle]
pub extern "C" fn z_view_str_check(this: &z_view_str_t) -> bool {
    z_view_slice_check(this.transmute_ref().transmute_ref())
}

/// Returns undefined `z_owned_str_t`
#[no_mangle]
pub extern "C" fn z_view_str_null(this: *mut MaybeUninit<z_view_str_t>) {
    z_view_slice_empty(this as *mut _)
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
pub extern "C" fn z_str_loan(this: &z_owned_str_t) -> Option<&z_loaned_str_t> {
    if !z_str_check(this) {
        return None;
    }
    Some(
        z_slice_loan(this.transmute_ref().transmute_ref())
            .transmute_ref()
            .transmute_handle(),
    )
}

/// Returns :c:type:`z_loaned_str_t` structure loaned from :c:type:`z_view_str_t`.
#[no_mangle]
pub extern "C" fn z_view_str_loan(this: &z_view_str_t) -> Option<&z_loaned_str_t> {
    if !z_view_str_check(this) {
        return None;
    }
    Some(
        z_view_slice_loan(this.transmute_ref().transmute_ref())
            .transmute_ref()
            .transmute_handle(),
    )
}

/// Copies a string into `z_owned_str_t` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// Calling this with `str == NULL` is equivalent to `z_str_null`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_str_wrap(
    this: *mut MaybeUninit<z_owned_str_t>,
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
    Inplace::init(this.transmute_uninit_ptr(), v.into());
}

/// Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// Calling this with `str == NULL` is equivalent to `z_view_str_null`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_str_wrap(
    this: *mut MaybeUninit<z_view_str_t>,
    str: *const libc::c_char,
) {
    z_view_slice_wrap(this as *mut _, str as _, strlen(str) + 1)
}

#[no_mangle]
pub extern "C" fn z_view_str_len(this: &z_loaned_str_t) -> usize {
    z_slice_len(this.transmute_ref().transmute_handle()) - 1
}

#[no_mangle]
pub extern "C" fn z_str_data(this: &z_loaned_str_t) -> *const libc::c_char {
    z_slice_data(this.transmute_ref().transmute_handle()) as _
}

#[no_mangle]
pub extern "C" fn z_str_clone(this: &z_loaned_str_t, dst: *mut MaybeUninit<z_owned_str_t>) {
    z_slice_clone(this.transmute_ref().transmute_handle(), dst as *mut _);
}

// returns string as slice (with null-terminating character)
#[no_mangle]
pub extern "C" fn z_str_as_slice(this: &z_loaned_str_t) -> &z_loaned_slice_t {
    this.transmute_ref().transmute_handle()
}

pub use crate::opaque_types::z_loaned_slice_map_t;
pub use crate::opaque_types::z_owned_slice_map_t;

pub type ZHashMap = HashMap<CSlice, CSlice>;
decl_transmute_handle!(
    HashMap<CSlice, CSlice>,
    z_loaned_slice_map_t
);

decl_transmute_owned!(Option<HashMap<CSlice, CSlice>>, z_owned_slice_map_t);

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
/// Returning `true` is treated as `break`.
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
        if body(key.transmute_handle(), value.transmute_handle(), context) {
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
) -> Option<&'static z_loaned_slice_t> {
    let m = this.transmute_ref();
    let key = key.transmute_ref();
    m.get(key).map(|s| s.transmute_handle())
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
    let key = key.transmute_ref();
    let value = value.transmute_ref();
    match this.insert(key.clone(), value.clone()) {
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
    match this.insert(key.shallow_copy(), value.shallow_copy()) {
        Some(_) => 1,
        None => 0,
    }
}

pub use crate::opaque_types::z_loaned_slice_array_t;
pub use crate::opaque_types::z_owned_slice_array_t;

pub type ZVector = Vec<CSlice>;
decl_transmute_handle!(Vec<CSlice>, z_loaned_slice_array_t);

decl_transmute_owned!(Option<Vec<CSlice>>, z_owned_slice_array_t);

/// Constructs a new empty array.
#[no_mangle]
pub extern "C" fn z_slice_array_new(this: *mut MaybeUninit<z_owned_slice_array_t>) {
    let this = this.transmute_uninit_ptr();
    let a = ZVector::new();
    Inplace::init(this, Some(a));
}

/// Constructs the gravestone value for `z_owned_slice_array_t`
#[no_mangle]
pub extern "C" fn z_slice_array_null(this: *mut MaybeUninit<z_owned_slice_array_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Returns `true` if the array is not in its gravestone state
#[no_mangle]
pub extern "C" fn z_slice_array_check(this: &z_owned_slice_array_t) -> bool {
    let this = this.transmute_ref();
    this.as_ref().is_some()
}

/// Destroys the array, resetting `this` to its gravestone value.
///
/// This function is double-free safe, passing a pointer to the gravestone value will have no effect.
#[no_mangle]
pub extern "C" fn z_slice_array_drop(this: &mut z_owned_slice_array_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

#[no_mangle]
pub extern "C" fn z_slice_array_loan(this: &z_owned_slice_array_t) -> &z_loaned_slice_array_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

#[no_mangle]
pub extern "C" fn z_slice_array_loan_mut(
    this: &mut z_owned_slice_array_t,
) -> &mut z_loaned_slice_array_t {
    let this = this.transmute_mut();
    let this = unwrap_ref_unchecked_mut(this);
    this.transmute_handle_mut()
}

/// Returns number of key-value pairs in the map.
#[no_mangle]
pub extern "C" fn z_slice_array_len(this: &z_loaned_slice_array_t) -> usize {
    this.transmute_ref().len()
}

/// Returns true if the array is empty, false otherwise.
#[no_mangle]
pub extern "C" fn z_slice_array_is_empty(this: &z_loaned_slice_array_t) -> bool {
    z_slice_array_len(this) == 0
}

/// Returns the value at the position of index in the array.
///
/// Will return NULL if the index is out of bounds.
#[no_mangle]
pub extern "C" fn z_slice_array_get(
    this: &z_loaned_slice_array_t,
    index: usize,
) -> Option<&z_loaned_slice_t> {
    let a = this.transmute_ref();
    if index >= a.len() {
        return None;
    }

    Some(a[index].transmute_handle())
}

/// Appends specified value to the end of the array by copying.
///
/// Returns the new length of the array.
#[no_mangle]
pub extern "C" fn z_slice_array_push_by_copy(
    this: &mut z_loaned_slice_array_t,
    value: &z_loaned_slice_t,
) -> usize {
    let this = this.transmute_mut();
    let v = value.transmute_ref();
    this.push(v.clone());

    this.len()
}

/// Appends specified value to the end of the array by aliasing.
///
/// Returns the new length of the array.
#[no_mangle]
pub extern "C" fn z_slice_array_push_by_alias(
    this: &mut z_loaned_slice_array_t,
    value: &z_loaned_slice_t,
) -> usize {
    let this = this.transmute_mut();
    let v = value.transmute_ref();
    this.push(v.shallow_copy());

    this.len()
}
