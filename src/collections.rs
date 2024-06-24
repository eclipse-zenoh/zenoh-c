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

use crate::errors::{self, z_error_t};
use crate::transmute::{
    unwrap_ref_unchecked, unwrap_ref_unchecked_mut, Inplace, TransmuteFromHandle,
    TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
};

pub struct CSlice(*const u8, isize);
#[derive(Default)]
pub struct CSliceOwned(pub CSlice);
#[derive(Default)]
pub struct CSliceView(pub CSlice);

impl AsRef<CSlice> for CSliceOwned {
    fn as_ref(&self) -> &CSlice {
        &self.0
    }
}

impl AsRef<CSlice> for CSliceView {
    fn as_ref(&self) -> &CSlice {
        &self.0
    }
}

impl CSliceView {
    pub fn new_borrowed(data: *const u8, len: usize) -> Self {
        let len: isize = len as isize;
        Self(CSlice(data, -len))
    }

    pub fn new_borrowed_from_slice(slice: &[u8]) -> Self {
        Self::new_borrowed(slice.as_ptr(), slice.len())
    }
}

impl CSliceOwned {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_owned(data: *const u8, len: usize) -> Self {
        if len == 0 {
            return Self::default();
        }
        let b = unsafe { from_raw_parts(data, len).to_vec().into_boxed_slice() };
        let slice = Box::leak(b);
        Self(CSlice(slice.as_ptr(), slice.len() as isize))
    }
}

impl CSlice {
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

    pub unsafe fn clone_to_owned(&self) -> CSliceOwned {
        CSliceOwned::new_owned(self.data(), self.len())
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

impl Clone for CSliceOwned {
    fn clone(&self) -> Self {
        unsafe { Self::new_owned(self.0.data(), self.0.len()) }
    }
}

impl From<Vec<u8>> for CSlice {
    fn from(value: Vec<u8>) -> Self {
        let slice = Box::leak(value.into_boxed_slice());
        CSlice(slice.as_ptr(), slice.len() as isize)
    }
}

impl From<String> for CSlice {
    fn from(value: String) -> Self {
        let slice = Box::leak(value.into_boxed_str());
        CSlice(slice.as_ptr() as _, slice.len() as isize)
    }
}

impl std::cmp::Eq for CSlice {}

pub use crate::opaque_types::z_loaned_slice_t;
pub use crate::opaque_types::z_owned_slice_t;
pub use crate::opaque_types::z_view_slice_t;


// decl_c_type!(
//     owned(z_owned_slice_t, CSlice),
//     loaned(z_loaned_slice_t, CSlice),
// )

decl_transmute_owned!(CSliceOwned, z_owned_slice_t);
decl_transmute_owned!(CSliceView, z_view_slice_t);
decl_transmute_handle!(CSliceOwned, z_loaned_slice_t);
decl_transmute_handle!(CSliceView, z_loaned_slice_t);

validate_equivalence!(z_owned_slice_t, z_loaned_slice_t);
validate_equivalence!(z_view_slice_t, z_loaned_slice_t);

/// Constructs an empty view slice.
#[no_mangle]
pub extern "C" fn z_view_slice_empty(this: *mut MaybeUninit<z_view_slice_t>) {
    Inplace::init(this.transmute_uninit_ptr(), CSliceView::default())
}

/// Constructs an empty view slice.
#[no_mangle]
pub extern "C" fn z_view_slice_null(this: *mut MaybeUninit<z_view_slice_t>) {
    Inplace::init(this.transmute_uninit_ptr(), CSliceView::default())
}

/// Constructs a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// @return -1 if `str == NULL` (and creates an empty view slice), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_slice_from_str(
    this: *mut MaybeUninit<z_view_slice_t>,
    str: *const c_char,
) -> z_error_t {
    if str.is_null() {
        z_view_slice_empty(this);
        errors::Z_EINVAL
    } else {
        z_view_slice_wrap(this, str as *const u8, libc::strlen(str));
        errors::Z_OK
    }
}

/// Constructs a `len` bytes long view starting at `start`.
///
/// @return -1 if `start == NULL` and `len > 0` (and creates an empty view slice), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_slice_wrap(
    this: *mut MaybeUninit<z_view_slice_t>,
    start: *const u8,
    len: usize,
) -> z_error_t {
    if len == 0 {
        z_view_slice_empty(this);
        errors::Z_OK
    } else if start.is_null() {
        z_view_slice_empty(this);
        errors::Z_EINVAL
    } else {
        Inplace::init(
            this.transmute_uninit_ptr(),
            CSliceView::new_borrowed(start, len),
        );
        errors::Z_OK
    }
}

/// Borrows view slice.
#[no_mangle]
pub extern "C" fn z_view_slice_loan(this: &z_view_slice_t) -> &z_loaned_slice_t {
    this.transmute_ref().transmute_handle()
}

/// @return ``true`` if the slice is not empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_view_slice_check(this: &z_view_slice_t) -> bool {
    !this.transmute_ref().as_ref().is_empty()
}

/// Constructs an empty `z_owned_slice_t`.
#[no_mangle]
pub extern "C" fn z_slice_empty(this: *mut MaybeUninit<z_owned_slice_t>) {
    Inplace::init(this.transmute_uninit_ptr(), CSliceOwned::default())
}

/// Constructs an empty `z_owned_slice_t`.
#[no_mangle]
pub extern "C" fn z_slice_null(this: *mut MaybeUninit<z_owned_slice_t>) {
    z_slice_empty(this);
}

/// Copies a string into `z_owned_slice_t` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// @return -1 if `str == NULL` (and creates an empty slice), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_from_str(
    this: *mut MaybeUninit<z_owned_slice_t>,
    str: *const c_char,
) -> z_error_t {
    if str.is_null() {
        z_slice_empty(this);
        errors::Z_EINVAL
    } else {
        z_slice_wrap(this, str as *const u8, libc::strlen(str));
        errors::Z_OK
    }
}

/// Constructs a slice by copying a `len` bytes long sequence starting at `start`.
///
/// @return -1 if `start == NULL` and `len > 0` (creating an empty slice), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_wrap(
    this: *mut MaybeUninit<z_owned_slice_t>,
    start: *const u8,
    len: usize,
) -> z_error_t {
    if len == 0 {
        z_slice_empty(this);
        errors::Z_OK
    } else if start.is_null() {
        z_slice_empty(this);
        errors::Z_EINVAL
    } else {
        Inplace::init(this.transmute_uninit_ptr(), CSliceOwned::new_owned(start, len));
        errors::Z_OK
    }
}

/// Frees the memory and invalidates the slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_drop(this: &mut z_owned_slice_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

/// Borrows slice.
#[no_mangle]
pub extern "C" fn z_slice_loan(this: &z_owned_slice_t) -> &z_loaned_slice_t {
    this.transmute_ref().transmute_handle()
}

/// Constructs an owned copy of a slice.
#[no_mangle]
pub extern "C" fn z_slice_clone(this: &z_loaned_slice_t, dst: *mut MaybeUninit<z_owned_slice_t>) {
    Inplace::init(dst.transmute_uninit_ptr(), this.transmute_ref().clone());
}

/// @return ``true`` if slice is not empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_slice_check(this: &z_owned_slice_t) -> bool {
    !this.transmute_ref().is_empty()
}

/// @return the length of the slice.
#[no_mangle]
pub extern "C" fn z_slice_len(this: &z_loaned_slice_t) -> usize {
    this.transmute_ref().len()
}

/// @return the pointer to the slice data.
#[no_mangle]
pub extern "C" fn z_slice_data(this: &z_loaned_slice_t) -> *const u8 {
    this.transmute_ref().data()
}

/// @return ``true`` if slice is empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_slice_is_empty(this: &z_loaned_slice_t) -> bool {
    this.transmute_ref().is_empty()
}

pub use crate::opaque_types::z_loaned_string_t;
pub use crate::opaque_types::z_owned_string_t;
pub use crate::opaque_types::z_view_string_t;

decl_transmute_owned!(custom_inplace_init CSlice, z_owned_string_t);
decl_transmute_owned!(custom_inplace_init CSlice, z_view_string_t);
decl_transmute_handle!(CSlice, z_loaned_string_t);

validate_equivalence!(z_owned_string_t, z_loaned_string_t);
validate_equivalence!(z_view_string_t, z_loaned_string_t);

/// Frees memory and invalidates `z_owned_string_t`, putting it in gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_drop(this: &mut z_owned_string_t) {
    z_slice_drop(this.transmute_mut().transmute_mut());
}

/// @return ``true`` if `this_` is a valid string, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_string_check(this: &z_owned_string_t) -> bool {
    z_slice_check(this.transmute_ref().transmute_ref())
}

/// Constructs owned string in a gravestone state.
#[no_mangle]
pub extern "C" fn z_string_null(this: *mut MaybeUninit<z_owned_string_t>) {
    z_slice_empty(this as *mut _)
}

/// @return ``true`` if view string is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
pub extern "C" fn z_view_string_check(this: &z_view_string_t) -> bool {
    z_view_slice_check(this.transmute_ref().transmute_ref())
}

/// Constructs view string in a gravestone state.
#[no_mangle]
pub extern "C" fn z_view_string_null(this: *mut MaybeUninit<z_view_string_t>) {
    z_view_slice_empty(this as *mut _)
}

/// Constructs an empty owned string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_empty(this: *mut MaybeUninit<z_owned_string_t>) {
    z_slice_empty(this as *mut _);
}

/// Constructs an empty view string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_string_empty(this: *mut MaybeUninit<z_view_string_t>) {
    z_view_slice_empty(this as *mut _);
}

/// Borrows string.
#[no_mangle]
pub extern "C" fn z_string_loan(this: &z_owned_string_t) -> &z_loaned_string_t {
    this.transmute_ref().transmute_handle()
}

/// Borrows view string.
#[no_mangle]
pub extern "C" fn z_view_string_loan(this: &z_view_string_t) -> &z_loaned_string_t {
    this.transmute_ref().transmute_handle()
}

/// Constructs an owned string by copying `str` into it (including terminating 0), using `strlen` (this should therefore not be used with untrusted inputs).
///
/// @return -1 if `str == NULL` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_wrap(
    this: *mut MaybeUninit<z_owned_string_t>,
    str: *const libc::c_char,
) -> z_error_t {
    z_slice_wrap(this as *mut _, str as _, strlen(str))
}

/// Constructs an owned string by copying a `str` substring of length `len`.
///
/// @return -1 if `str == NULL` and `len > 0` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_from_substring(
    this: *mut MaybeUninit<z_owned_string_t>,
    str: *const libc::c_char,
    len: usize,
) -> z_error_t {
    z_slice_wrap(this as *mut _, str as _, len)
}

/// Constructs a view string of `str`, using `strlen` (this should therefore not be used with untrusted inputs).
///
/// @return -1 if `str == NULL` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_string_wrap(
    this: *mut MaybeUninit<z_view_string_t>,
    str: *const libc::c_char,
) -> z_error_t {
    z_view_slice_wrap(this as *mut _, str as _, strlen(str))
}

/// Constructs a view string to a specified substring of length `len`.
///
/// @return -1 if `str == NULL` and `len > 0` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_string_from_substring(
    this: *mut MaybeUninit<z_view_string_t>,
    str: *const libc::c_char,
    len: usize,
) -> z_error_t {
    z_view_slice_wrap(this as *mut _, str as _, len)
}

/// @return the length of the string (without terminating 0 character).
#[no_mangle]
pub extern "C" fn z_string_len(this: &z_loaned_string_t) -> usize {
    z_slice_len(this.transmute_ref().transmute_handle())
}

/// @return the pointer of the string data.
#[no_mangle]
pub extern "C" fn z_string_data(this: &z_loaned_string_t) -> *const libc::c_char {
    z_slice_data(this.transmute_ref().transmute_handle()) as _
}

/// Constructs an owned copy of a string.
#[no_mangle]
pub extern "C" fn z_string_clone(
    this: &z_loaned_string_t,
    dst: *mut MaybeUninit<z_owned_string_t>,
) {
    z_slice_clone(this.transmute_ref().transmute_handle(), dst as *mut _);
}

// Converts loaned string into loaned slice (with terminating 0 character).
#[no_mangle]
pub extern "C" fn z_string_as_slice(this: &z_loaned_string_t) -> &z_loaned_slice_t {
    this.transmute_ref().transmute_handle()
}

/// @return ``true`` if string is empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_string_is_empty(this: &z_loaned_string_t) -> bool {
    z_string_len(this) == 0
}

pub use crate::opaque_types::z_loaned_slice_map_t;
pub use crate::opaque_types::z_owned_slice_map_t;

pub type ZHashMap = HashMap<CSlice, CSlice>;
decl_transmute_handle!(HashMap<CSlice, CSlice>, z_loaned_slice_map_t);
decl_transmute_owned!(Option<HashMap<CSlice, CSlice>>, z_owned_slice_map_t);

validate_equivalence!(z_owned_slice_map_t, z_loaned_slice_map_t);

/// Constructs a new empty map.
#[no_mangle]
pub extern "C" fn z_slice_map_new(this: *mut MaybeUninit<z_owned_slice_map_t>) {
    let this = this.transmute_uninit_ptr();
    let map = ZHashMap::new();
    Inplace::init(this, Some(map));
}

/// Constructs the gravestone value for `z_owned_slice_map_t`.
#[no_mangle]
pub extern "C" fn z_slice_map_null(this: *mut MaybeUninit<z_owned_slice_map_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// @return ``true`` if the map is not in its gravestone state, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_slice_map_check(map: &z_owned_slice_map_t) -> bool {
    let map = map.transmute_ref();
    map.as_ref().is_some()
}

/// Destroys the map, resetting it to its gravestone value.
#[no_mangle]
pub extern "C" fn z_slice_map_drop(this: &mut z_owned_slice_map_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

/// Borrows slice map.
#[no_mangle]
pub extern "C" fn z_slice_map_loan(this: &z_owned_slice_map_t) -> &z_loaned_slice_map_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

/// Mutably borrows slice map.
#[no_mangle]
pub extern "C" fn z_slice_map_loan_mut(
    this: &mut z_owned_slice_map_t,
) -> &mut z_loaned_slice_map_t {
    let this = this.transmute_mut();
    let this = unwrap_ref_unchecked_mut(this);
    this.transmute_handle_mut()
}

/// @return number of key-value pairs in the map.
#[no_mangle]
pub extern "C" fn z_slice_map_len(this: &z_loaned_slice_map_t) -> usize {
    this.transmute_ref().len()
}

/// @return ``true`` if the map is empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_slice_map_is_empty(this: &z_loaned_slice_map_t) -> bool {
    z_slice_map_len(this) == 0
}

/// Iterates over key-value pairs of a slice map.
///
/// @param this_: Slice map to iterate over.
/// @param body: Iterator body function. Returning `true` is treated as iteration loop `break`.
/// @param context: Some data passed to every body invocation.
#[no_mangle]
pub extern "C" fn z_slice_map_iterate(
    this: &z_loaned_slice_map_t,
    body: extern "C" fn(
        key: &z_loaned_slice_t,
        value: &z_loaned_slice_t,
        context: *mut c_void,
    ) -> bool,
    context: *mut c_void,
) {
    let this = this.transmute_ref();
    for (key, value) in this {
        if body(key.transmute_handle(), value.transmute_handle(), context) {
            break;
        }
    }
}

/// @return the value associated with `key` (`NULL` if the key is not present in the map.).
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
/// If the `key` was already present in the map, its value is updated.
/// @return 1 if there was already an entry associated with the key, 0 otherwise.
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
/// If the `key` was already present in the map, its value is updated.
/// @return 1 if there was already an entry associated with the key, 0 otherwise.
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

pub use crate::opaque_types::z_loaned_string_array_t;
pub use crate::opaque_types::z_owned_string_array_t;

pub type ZVector = Vec<CSlice>;
decl_transmute_handle!(Vec<CSlice>, z_loaned_string_array_t);
decl_transmute_owned!(Option<Vec<CSlice>>, z_owned_string_array_t);

validate_equivalence!(z_owned_string_array_t, z_loaned_string_array_t);

/// Constructs a new empty string array.
#[no_mangle]
pub extern "C" fn z_string_array_new(this: *mut MaybeUninit<z_owned_string_array_t>) {
    let this: *mut MaybeUninit<Option<Vec<CSlice>>> = this.transmute_uninit_ptr();
    let a = ZVector::new();
    Inplace::init(this, Some(a));
}

/// Constructs string array in its gravestone state.
#[no_mangle]
pub extern "C" fn z_string_array_null(this: *mut MaybeUninit<z_owned_string_array_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// @return ``true`` if the string array is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
pub extern "C" fn z_string_array_check(this: &z_owned_string_array_t) -> bool {
    let this = this.transmute_ref();
    this.as_ref().is_some()
}

/// Destroys the string array, resetting it to its gravestone value.
#[no_mangle]
pub extern "C" fn z_string_array_drop(this: &mut z_owned_string_array_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

/// Borrows string array.
#[no_mangle]
pub extern "C" fn z_string_array_loan(this: &z_owned_string_array_t) -> &z_loaned_string_array_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

/// Mutably borrows string array.
#[no_mangle]
pub extern "C" fn z_string_array_loan_mut(
    this: &mut z_owned_string_array_t,
) -> &mut z_loaned_string_array_t {
    let this = this.transmute_mut();
    let this = unwrap_ref_unchecked_mut(this);
    this.transmute_handle_mut()
}

/// @return number of elements in the array.
#[no_mangle]
pub extern "C" fn z_string_array_len(this: &z_loaned_string_array_t) -> usize {
    this.transmute_ref().len()
}

/// @return ``true`` if the array is empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_string_array_is_empty(this: &z_loaned_string_array_t) -> bool {
    z_string_array_len(this) == 0
}

/// @return the value at the position of index in the string array.
///
/// Will return `NULL` if the index is out of bounds.
#[no_mangle]
pub extern "C" fn z_string_array_get(
    this: &z_loaned_string_array_t,
    index: usize,
) -> Option<&z_loaned_string_t> {
    let a = this.transmute_ref();
    if index >= a.len() {
        return None;
    }

    Some(a[index].transmute_handle())
}

/// Appends specified value to the end of the string array by copying.
///
/// @return the new length of the array.
#[no_mangle]
pub extern "C" fn z_string_array_push_by_copy(
    this: &mut z_loaned_string_array_t,
    value: &z_loaned_string_t,
) -> usize {
    let this = this.transmute_mut();
    let v = value.transmute_ref();
    this.push(v.clone());

    this.len()
}

/// Appends specified value to the end of the string array by alias.
///
/// @return the new length of the array.
#[no_mangle]
pub extern "C" fn z_string_array_push_by_alias(
    this: &mut z_loaned_string_array_t,
    value: &z_loaned_string_t,
) -> usize {
    let this = this.transmute_mut();
    let v = value.transmute_ref();
    this.push(v.shallow_copy());

    this.len()
}
