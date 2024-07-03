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
use std::ops::{Deref, DerefMut};
use std::ptr::{null, slice_from_raw_parts};
use std::slice::from_raw_parts;

use libc::{c_char, c_void, strlen};

use crate::errors::{self, z_error_t};
use crate::transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit};

pub struct CSlice(*const u8, isize);
#[derive(Default, Clone)]
pub struct CSliceOwned(CSlice);
#[derive(Default)]
pub struct CSliceView(CSlice);

impl Deref for CSliceOwned {
    type Target = CSlice;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for CSliceView {
    type Target = CSlice;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CSliceOwned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for CSliceView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<CSliceOwned> for CSlice {
    fn from(value: CSliceOwned) -> Self {
        value.0
    }
}

impl From<CSliceView> for CSlice {
    fn from(value: CSliceView) -> Self {
        value.0
    }
}

impl CSliceView {
    pub fn new(data: *const u8, len: usize) -> Result<Self, z_error_t> {
        Ok(Self(CSlice::new_borrowed(data, len)?))
    }
}

impl CSliceOwned {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new(data: *const u8, len: usize) -> Result<Self, z_error_t> {
        Ok(Self(CSlice::new_owned(data, len)?))
    }
}

impl CSlice {
    pub fn new_borrowed_unchecked(data: *const u8, len: usize) -> Self {
        let len: isize = len as isize;
        Self(data, -len)
    }

    pub fn new_borrowed(data: *const u8, len: usize) -> Result<Self, z_error_t> {
        if data.is_null() && len > 0 {
            Err(errors::Z_EINVAL)
        } else {
            Ok(Self::new_borrowed_unchecked(data, len))
        }
    }

    pub fn new_borrowed_from_slice(slice: &[u8]) -> Self {
        Self::new_borrowed_unchecked(slice.as_ptr(), slice.len())
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_owned_unchecked(data: *const u8, len: usize) -> Self {
        if len == 0 {
            return Self::default();
        }
        let b = unsafe { from_raw_parts(data, len).to_vec().into_boxed_slice() };
        let slice = Box::leak(b);
        Self(slice.as_ptr(), slice.len() as isize)
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_owned(data: *const u8, len: usize) -> Result<Self, z_error_t> {
        if data.is_null() && len > 0 {
            Err(errors::Z_EINVAL)
        } else {
            Ok(Self::new_owned_unchecked(data, len))
        }
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

    pub fn clone_to_owned(&self) -> CSliceOwned {
        CSliceOwned(unsafe { Self::new_owned_unchecked(self.data(), self.len()) })
    }
}

impl Clone for CSlice {
    fn clone(&self) -> Self {
        unsafe { Self::new_owned_unchecked(self.data(), self.len()) }
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

impl From<Vec<u8>> for CSliceOwned {
    fn from(value: Vec<u8>) -> Self {
        let slice = Box::leak(value.into_boxed_slice());
        CSliceOwned(CSlice(slice.as_ptr(), slice.len() as isize))
    }
}

impl From<Vec<u8>> for CSlice {
    fn from(value: Vec<u8>) -> Self {
        let slice: CSliceOwned = value.into();
        slice.0
    }
}

impl std::cmp::Eq for CSlice {}

pub use crate::opaque_types::z_loaned_slice_t;
pub use crate::opaque_types::z_moved_slice_t;
pub use crate::opaque_types::z_owned_slice_t;
pub use crate::opaque_types::z_view_slice_t;

decl_c_type!(
    owned(z_owned_slice_t, CSliceOwned),
    loaned(z_loaned_slice_t, CSlice),
    view(z_view_slice_t, CSliceView),
    moved(z_moved_slice_t)
);

/// Constructs an empty view slice.
#[no_mangle]
pub extern "C" fn z_view_slice_empty(this: &mut MaybeUninit<z_view_slice_t>) {
    this.as_rust_type_mut_uninit().write(CSliceView::default());
}

/// Constructs an empty view slice.
#[no_mangle]
pub extern "C" fn z_view_slice_null(this: &mut MaybeUninit<z_view_slice_t>) {
    this.as_rust_type_mut_uninit().write(CSliceView::default());
}

/// Constructs a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// @return -1 if `str == NULL` (and creates an empty view slice), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_slice_from_str(
    this: &mut MaybeUninit<z_view_slice_t>,
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
    this: &mut MaybeUninit<z_view_slice_t>,
    start: *const u8,
    len: usize,
) -> z_error_t {
    let this = this.as_rust_type_mut_uninit();
    match CSliceView::new(start, len) {
        Ok(slice) => {
            this.write(slice);
            errors::Z_OK
        }
        Err(e) => {
            this.write(CSliceView::default());
            e
        }
    }
}

/// Borrows view slice.
#[no_mangle]
pub extern "C" fn z_view_slice_loan(this: &z_view_slice_t) -> &z_loaned_slice_t {
    this.as_rust_type_ref().as_loaned_c_type_ref()
}

/// @return ``true`` if the slice is not empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_view_slice_check(this: &z_view_slice_t) -> bool {
    !this.as_rust_type_ref().is_empty()
}

/// Constructs an empty `z_owned_slice_t`.
#[no_mangle]
pub extern "C" fn z_slice_empty(this: &mut MaybeUninit<z_owned_slice_t>) {
    this.as_rust_type_mut_uninit().write(CSliceOwned::default());
}

/// Constructs an empty `z_owned_slice_t`.
#[no_mangle]
pub extern "C" fn z_slice_null(this: &mut MaybeUninit<z_owned_slice_t>) {
    z_slice_empty(this);
}

/// Copies a string into `z_owned_slice_t` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// @return -1 if `str == NULL` (and creates an empty slice), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_from_str(
    this: &mut MaybeUninit<z_owned_slice_t>,
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
    this: &mut MaybeUninit<z_owned_slice_t>,
    start: *const u8,
    len: usize,
) -> z_error_t {
    let this = this.as_rust_type_mut_uninit();
    match CSliceOwned::new(start, len) {
        Ok(slice) => {
            this.write(slice);
            errors::Z_OK
        }
        Err(e) => {
            this.write(CSliceOwned::default());
            e
        }
    }
}

/// Frees the memory and invalidates the slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
#[allow(unused_variables)]
pub unsafe extern "C" fn z_slice_drop(this: z_moved_slice_t) {}

/// Borrows slice.
#[no_mangle]
pub extern "C" fn z_slice_loan(this: &z_owned_slice_t) -> &z_loaned_slice_t {
    this.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Constructs an owned copy of a slice.
#[no_mangle]
pub extern "C" fn z_slice_clone(this: &z_loaned_slice_t, dst: &mut MaybeUninit<z_owned_slice_t>) {
    dst.as_rust_type_mut_uninit()
        .write(this.as_rust_type_ref().clone_to_owned());
}

/// @return ``true`` if slice is not empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_slice_check(this: &z_owned_slice_t) -> bool {
    !this.as_rust_type_ref().is_empty()
}

/// @return the length of the slice.
#[no_mangle]
pub extern "C" fn z_slice_len(this: &z_loaned_slice_t) -> usize {
    this.as_rust_type_ref().len()
}

/// @return the pointer to the slice data.
#[no_mangle]
pub extern "C" fn z_slice_data(this: &z_loaned_slice_t) -> *const u8 {
    this.as_rust_type_ref().data()
}

/// @return ``true`` if slice is empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_slice_is_empty(this: &z_loaned_slice_t) -> bool {
    this.as_rust_type_ref().is_empty()
}

pub use crate::opaque_types::z_loaned_string_t;
pub use crate::opaque_types::z_moved_string_t;
pub use crate::opaque_types::z_owned_string_t;
pub use crate::opaque_types::z_view_string_t;

#[derive(Default)]
pub struct CString(CSlice);
#[derive(Default)]
pub struct CStringOwned(CString);
#[derive(Default)]
pub struct CStringView(CString);

impl CString {
    pub fn new_borrowed_from_slice(slice: &[u8]) -> Self {
        CString(CSlice::new_borrowed_from_slice(slice))
    }
}

impl CStringOwned {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_owned(data: *const libc::c_char, len: usize) -> Result<Self, z_error_t> {
        Ok(CStringOwned(CString(CSlice::new_owned(data as _, len)?)))
    }
}

impl CStringView {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_borrowed(data: *const libc::c_char, len: usize) -> Result<Self, z_error_t> {
        Ok(CStringView(CString(CSlice::new_borrowed(data as _, len)?)))
    }
    pub fn new_borrowed_from_slice(slice: &[u8]) -> Self {
        CStringView(CString::new_borrowed_from_slice(slice))
    }
}

impl Deref for CString {
    type Target = CSlice;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for CStringOwned {
    type Target = CString;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for CStringView {
    type Target = CString;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<CSlice> for CString {
    fn as_ref(&self) -> &CSlice {
        &self.0
    }
}

impl AsRef<CSlice> for CStringOwned {
    fn as_ref(&self) -> &CSlice {
        &self.0
    }
}

impl AsRef<CSlice> for CStringView {
    fn as_ref(&self) -> &CSlice {
        &self.0
    }
}

impl From<String> for CStringOwned {
    fn from(value: String) -> Self {
        let slice = Box::leak(value.into_boxed_str());
        CStringOwned(CString(CSlice(slice.as_ptr(), slice.len() as isize)))
    }
}

decl_c_type!(
    owned(z_owned_string_t, CStringOwned),
    loaned(z_loaned_string_t, CString),
    view(z_view_string_t, CStringView),
    moved(z_moved_string_t)
);

/// Frees memory and invalidates `z_owned_string_t`, putting it in gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
#[allow(unused_variables)]
pub unsafe extern "C" fn z_string_drop(this: z_moved_string_t) {}

/// @return ``true`` if `this_` is a valid string, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_string_check(this: &z_owned_string_t) -> bool {
    !this.as_rust_type_ref().is_empty()
}

/// Constructs owned string in a gravestone state.
#[no_mangle]
pub extern "C" fn z_string_null(this: &mut MaybeUninit<z_owned_string_t>) {
    this.as_rust_type_mut_uninit()
        .write(CStringOwned::default());
}

/// @return ``true`` if view string is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
pub extern "C" fn z_view_string_check(this: &z_view_string_t) -> bool {
    !this.as_rust_type_ref().is_empty()
}

/// Constructs view string in a gravestone state.
#[no_mangle]
pub extern "C" fn z_view_string_null(this: &mut MaybeUninit<z_view_string_t>) {
    this.as_rust_type_mut_uninit().write(CStringView::default());
}

/// Constructs an empty owned string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_empty(this: &mut MaybeUninit<z_owned_string_t>) {
    this.as_rust_type_mut_uninit()
        .write(CStringOwned::default());
}

/// Constructs an empty view string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_string_empty(this: &mut MaybeUninit<z_view_string_t>) {
    this.as_rust_type_mut_uninit().write(CStringView::default());
}

/// Borrows string.
#[no_mangle]
pub extern "C" fn z_string_loan(this: &z_owned_string_t) -> &z_loaned_string_t {
    this.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Borrows view string.
#[no_mangle]
pub extern "C" fn z_view_string_loan(this: &z_view_string_t) -> &z_loaned_string_t {
    this.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Constructs an owned string by copying `str` into it (including terminating 0), using `strlen` (this should therefore not be used with untrusted inputs).
///
/// @return -1 if `str == NULL` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_from_str(
    this: &mut MaybeUninit<z_owned_string_t>,
    str: *const libc::c_char,
) -> z_error_t {
    z_string_from_substr(this, str, strlen(str))
}

/// Constructs an owned string by copying a `str` substring of length `len`.
///
/// @return -1 if `str == NULL` and `len > 0` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_from_substr(
    this: &mut MaybeUninit<z_owned_string_t>,
    str: *const libc::c_char,
    len: usize,
) -> z_error_t {
    let this = this.as_rust_type_mut_uninit();
    match CStringOwned::new_owned(str, len) {
        Ok(slice) => {
            this.write(slice);
            errors::Z_OK
        }
        Err(e) => {
            this.write(CStringOwned::default());
            e
        }
    }
}

/// Constructs a view string of `str`, using `strlen` (this should therefore not be used with untrusted inputs).
///
/// @return -1 if `str == NULL` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_string_wrap(
    this: &mut MaybeUninit<z_view_string_t>,
    str: *const libc::c_char,
) -> z_error_t {
    let this = this.as_rust_type_mut_uninit();
    match CStringView::new_borrowed(str, strlen(str)) {
        Ok(slice) => {
            this.write(slice);
            errors::Z_OK
        }
        Err(e) => {
            this.write(CStringView::default());
            e
        }
    }
}

/// Constructs a view string to a specified substring of length `len`.
///
/// @return -1 if `str == NULL` and `len > 0` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_string_from_substr(
    this: &mut MaybeUninit<z_view_string_t>,
    str: *const libc::c_char,
    len: usize,
) -> z_error_t {
    let this = this.as_rust_type_mut_uninit();
    match CStringView::new_borrowed(str, len) {
        Ok(slice) => {
            this.write(slice);
            errors::Z_OK
        }
        Err(e) => {
            this.write(CStringView::default());
            e
        }
    }
}

/// @return the length of the string (without terminating 0 character).
#[no_mangle]
pub extern "C" fn z_string_len(this: &z_loaned_string_t) -> usize {
    this.as_rust_type_ref().len()
}

/// @return the pointer of the string data.
#[no_mangle]
pub extern "C" fn z_string_data(this: &z_loaned_string_t) -> *const libc::c_char {
    this.as_rust_type_ref().data() as _
}

/// Constructs an owned copy of a string.
#[no_mangle]
pub extern "C" fn z_string_clone(
    this: &z_loaned_string_t,
    dst: &mut MaybeUninit<z_owned_string_t>,
) {
    let slice = this.as_rust_type_ref().clone_to_owned();
    dst.as_rust_type_mut_uninit()
        .write(CStringOwned(CString(slice.0)));
}

// Converts loaned string into loaned slice (with terminating 0 character).
#[no_mangle]
pub extern "C" fn z_string_as_slice(this: &z_loaned_string_t) -> &z_loaned_slice_t {
    this.as_rust_type_ref().as_ref().as_loaned_c_type_ref()
}

/// @return ``true`` if string is empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_string_is_empty(this: &z_loaned_string_t) -> bool {
    this.as_rust_type_ref().is_empty()
}

pub use crate::opaque_types::z_loaned_slice_map_t;
pub use crate::opaque_types::z_moved_slice_map_t;
pub use crate::opaque_types::z_owned_slice_map_t;
pub type ZHashMap = HashMap<CSlice, CSlice>;
decl_c_type!(
    owned(z_owned_slice_map_t, Option<ZHashMap>),
    loaned(z_loaned_slice_map_t, ZHashMap),
    moved(z_moved_slice_map_t)
);

/// Constructs a new empty map.
#[no_mangle]
pub extern "C" fn z_slice_map_new(this: &mut MaybeUninit<z_owned_slice_map_t>) {
    this.as_rust_type_mut_uninit().write(Some(ZHashMap::new()));
}

/// Constructs the gravestone value for `z_owned_slice_map_t`.
#[no_mangle]
pub extern "C" fn z_slice_map_null(this: &mut MaybeUninit<z_owned_slice_map_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// @return ``true`` if the map is not in its gravestone state, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_slice_map_check(map: &z_owned_slice_map_t) -> bool {
    map.as_rust_type_ref().is_some()
}

/// Destroys the map, resetting it to its gravestone value.
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_slice_map_drop(this: z_moved_slice_map_t) {}

/// Borrows slice map.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_map_loan(this: &z_owned_slice_map_t) -> &z_loaned_slice_map_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Mutably borrows slice map.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_map_loan_mut(
    this: &mut z_owned_slice_map_t,
) -> &mut z_loaned_slice_map_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// @return number of key-value pairs in the map.
#[no_mangle]
pub extern "C" fn z_slice_map_len(this: &z_loaned_slice_map_t) -> usize {
    this.as_rust_type_ref().len()
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
    let this = this.as_rust_type_ref();
    for (key, value) in this {
        if body(
            key.as_loaned_c_type_ref(),
            value.as_loaned_c_type_ref(),
            context,
        ) {
            break;
        }
    }
}

/// @return the value associated with `key` (`NULL` if the key is not present in the map.).
#[no_mangle]
pub extern "C" fn z_slice_map_get<'a>(
    this: &'a z_loaned_slice_map_t,
    key: &z_loaned_slice_t,
) -> Option<&'a z_loaned_slice_t> {
    let m = this.as_rust_type_ref();
    let key = key.as_rust_type_ref();
    m.get(key).map(|s| s.as_loaned_c_type_ref())
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
    let this = this.as_rust_type_mut();
    let key = key.as_rust_type_ref();
    let value = value.as_rust_type_ref();
    match this.insert(key.clone_to_owned().into(), value.clone_to_owned().into()) {
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
    let this = this.as_rust_type_mut();
    let key = key.as_rust_type_ref();
    let value = value.as_rust_type_ref();
    match this.insert(key.shallow_copy(), value.shallow_copy()) {
        Some(_) => 1,
        None => 0,
    }
}

pub use crate::opaque_types::z_loaned_string_array_t;
pub use crate::opaque_types::z_moved_string_array_t;
pub use crate::opaque_types::z_owned_string_array_t;
pub type ZVector = Vec<CString>;
decl_c_type!(
    owned(z_owned_string_array_t, Option<ZVector>),
    loaned(z_loaned_string_array_t, ZVector),
    moved(z_moved_string_array_t)
);

/// Constructs a new empty string array.
#[no_mangle]
pub extern "C" fn z_string_array_new(this: &mut MaybeUninit<z_owned_string_array_t>) {
    this.as_rust_type_mut_uninit().write(Some(ZVector::new()));
}

/// Constructs string array in its gravestone state.
#[no_mangle]
pub extern "C" fn z_string_array_null(this: &mut MaybeUninit<z_owned_string_array_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// @return ``true`` if the string array is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
pub extern "C" fn z_string_array_check(this: &z_owned_string_array_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Destroys the string array, resetting it to its gravestone value.
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_string_array_drop(this: z_moved_string_array_t) {}

/// Borrows string array.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_array_loan(
    this: &z_owned_string_array_t,
) -> &z_loaned_string_array_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Mutably borrows string array.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_array_loan_mut(
    this: &mut z_owned_string_array_t,
) -> &mut z_loaned_string_array_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// @return number of elements in the array.
#[no_mangle]
pub extern "C" fn z_string_array_len(this: &z_loaned_string_array_t) -> usize {
    this.as_rust_type_ref().len()
}

/// @return ``true`` if the array is empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_string_array_is_empty(this: &z_loaned_string_array_t) -> bool {
    this.as_rust_type_ref().is_empty()
}

/// @return the value at the position of index in the string array.
///
/// Will return `NULL` if the index is out of bounds.
#[no_mangle]
pub extern "C" fn z_string_array_get(
    this: &z_loaned_string_array_t,
    index: usize,
) -> Option<&z_loaned_string_t> {
    let a = this.as_rust_type_ref();
    if index >= a.len() {
        return None;
    }

    Some(a[index].as_loaned_c_type_ref())
}

/// Appends specified value to the end of the string array by copying.
///
/// @return the new length of the array.
#[no_mangle]
pub extern "C" fn z_string_array_push_by_copy(
    this: &mut z_loaned_string_array_t,
    value: &z_loaned_string_t,
) -> usize {
    let this = this.as_rust_type_mut();
    let v = value.as_rust_type_ref();
    this.push(CString(v.clone_to_owned().into()));

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
    let this = this.as_rust_type_mut();
    let v = value.as_rust_type_ref();
    this.push(CString(v.shallow_copy()));

    this.len()
}
