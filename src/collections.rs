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

use core::ffi::c_void;
use std::{
    hash::Hash,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::{null, null_mut, slice_from_raw_parts},
    slice::from_raw_parts,
};

use libc::strlen;

use crate::{
    result::{self, z_result_t},
    transmute::{Gravestone, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
};

pub struct CSlice {
    data: *const u8,
    len: usize,
    drop: Option<extern "C" fn(data: *mut c_void, context: *mut c_void)>,
    context: *mut c_void,
}

pub extern "C" fn _z_drop_c_slice_default(data: *mut c_void, context: *mut c_void) {
    let ptr = data as *const u8;
    let len = context as usize;
    let b = unsafe { Box::from_raw(slice_from_raw_parts(ptr, len).cast_mut()) };
    std::mem::drop(b);
}

#[derive(Clone)]
pub struct CSliceOwned(CSlice);
pub struct CSliceView(CSlice);

impl Gravestone for CSliceOwned {
    fn gravestone() -> Self {
        Self(CSlice::gravestone())
    }
    fn is_gravestone(&self) -> bool {
        self.0.is_gravestone()
    }
}

impl Gravestone for CSliceView {
    fn gravestone() -> Self {
        Self(CSlice::gravestone())
    }
    fn is_gravestone(&self) -> bool {
        self.0.is_gravestone()
    }
}

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
    pub fn new(data: *const u8, len: usize) -> Result<Self, z_result_t> {
        Ok(Self(CSlice::new_borrowed(data, len)?))
    }

    pub fn from_slice(s: &[u8]) -> CSliceView {
        CSliceView(CSlice::new_borrowed_from_slice(s))
    }
}

impl CSliceOwned {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new(data: *const u8, len: usize) -> Result<Self, z_result_t> {
        Ok(Self(CSlice::new_owned(data, len)?))
    }
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn wrap(
        data: *mut u8,
        len: usize,
        drop: Option<extern "C" fn(data: *mut c_void, context: *mut c_void)>,
        context: *mut c_void,
    ) -> Result<Self, z_result_t> {
        Ok(CSliceOwned(CSlice::new(data, len, drop, context)?))
    }
}

impl CSlice {
    pub fn new_unchecked(
        data: *const u8,
        len: usize,
        drop: Option<extern "C" fn(data: *mut c_void, context: *mut c_void)>,
        context: *mut c_void,
    ) -> Self {
        Self {
            data,
            len,
            drop,
            context,
        }
    }

    pub fn new_borrowed_unchecked(data: *const u8, len: usize) -> Self {
        Self::new_unchecked(data, len, None, null_mut())
    }

    pub fn new(
        data: *mut u8,
        len: usize,
        drop: Option<extern "C" fn(data: *mut c_void, context: *mut c_void)>,
        context: *mut c_void,
    ) -> Result<Self, z_result_t> {
        if data.is_null() && len > 0 {
            Err(result::Z_EINVAL)
        } else {
            Ok(Self::new_unchecked(data, len, drop, context))
        }
    }

    pub fn new_borrowed(data: *const u8, len: usize) -> Result<Self, z_result_t> {
        if data.is_null() && len > 0 {
            Err(result::Z_EINVAL)
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
            return Self::gravestone();
        }
        let b = unsafe { from_raw_parts(data, len).to_vec().into_boxed_slice() };
        let slice = Box::leak(b);
        CSlice::wrap(slice.as_ptr(), len)
    }

    pub fn wrap(data: *const u8, len: usize) -> Self {
        Self::new_unchecked(data, len, Some(_z_drop_c_slice_default), len as *mut c_void)
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_owned(data: *const u8, len: usize) -> Result<Self, z_result_t> {
        if data.is_null() && len > 0 {
            Err(result::Z_EINVAL)
        } else {
            Ok(Self::new_owned_unchecked(data, len))
        }
    }

    pub fn slice(&self) -> &'static [u8] {
        if self.len == 0 {
            return &[0u8; 0];
        }
        unsafe { from_raw_parts(self.data, self.len) }
    }

    pub fn data(&self) -> *const u8 {
        self.data
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_owned(&self) -> bool {
        self.drop.is_some()
    }

    pub fn clone_to_borrowed(&self) -> Self {
        Self::new_borrowed_unchecked(self.data, self.len)
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

impl Gravestone for CSlice {
    fn gravestone() -> Self {
        Self {
            data: null(),
            len: 0,
            drop: None,
            context: null_mut(),
        }
    }
    fn is_gravestone(&self) -> bool {
        self.data.is_null()
    }
}

impl Drop for CSlice {
    fn drop(&mut self) {
        if let Some(drop) = self.drop {
            drop(self.data as *mut c_void, self.context);
        }
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
        CSliceOwned(CSlice::wrap(slice.as_ptr(), slice.len()))
    }
}

impl From<Vec<u8>> for CSlice {
    fn from(value: Vec<u8>) -> Self {
        let slice: CSliceOwned = value.into();
        slice.0
    }
}

impl std::cmp::Eq for CSlice {}

pub use crate::opaque_types::{z_loaned_slice_t, z_moved_slice_t, z_owned_slice_t, z_view_slice_t};

decl_c_type!(
    owned(z_owned_slice_t, CSliceOwned),
    loaned(z_loaned_slice_t, CSlice),
    view(z_view_slice_t, CSliceView),
);

/// Constructs an empty view slice.
#[no_mangle]
pub extern "C" fn z_view_slice_empty(this_: &mut MaybeUninit<z_view_slice_t>) {
    this_
        .as_rust_type_mut_uninit()
        .write(CSliceView::gravestone());
}

/// Constructs a `len` bytes long view starting at `start`.
///
/// @return -1 if `start == NULL` and `len > 0` (and creates an empty view slice), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_slice_from_buf(
    this: &mut MaybeUninit<z_view_slice_t>,
    start: *const u8,
    len: usize,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    match CSliceView::new(start, len) {
        Ok(slice) => {
            this.write(slice);
            result::Z_OK
        }
        Err(e) => {
            this.write(CSliceView::gravestone());
            e
        }
    }
}

/// Borrows view slice.
#[no_mangle]
pub extern "C" fn z_view_slice_loan(this_: &z_view_slice_t) -> &z_loaned_slice_t {
    this_.as_rust_type_ref().as_loaned_c_type_ref()
}

/// @return ``true`` if the slice is not empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_view_slice_is_empty(this_: &z_view_slice_t) -> bool {
    this_.as_rust_type_ref().is_empty()
}

/// Constructs an empty `z_owned_slice_t`.
#[no_mangle]
pub extern "C" fn z_slice_empty(this_: &mut MaybeUninit<z_owned_slice_t>) {
    this_
        .as_rust_type_mut_uninit()
        .write(CSliceOwned::gravestone());
}

/// Constructs an empty `z_owned_slice_t`.
#[no_mangle]
pub extern "C" fn z_internal_slice_null(this_: &mut MaybeUninit<z_owned_slice_t>) {
    z_slice_empty(this_);
}

/// Frees the memory and invalidates the slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_drop(this_: &mut z_moved_slice_t) {
    let _ = this_.take_rust_type();
}

/// Borrows slice.
#[no_mangle]
pub extern "C" fn z_slice_loan(this_: &z_owned_slice_t) -> &z_loaned_slice_t {
    this_.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Constructs an owned copy of a slice.
#[no_mangle]
pub extern "C" fn z_slice_clone(dst: &mut MaybeUninit<z_owned_slice_t>, this_: &z_loaned_slice_t) {
    dst.as_rust_type_mut_uninit()
        .write(this_.as_rust_type_ref().clone_to_owned());
}

/// @return ``true`` if slice is not empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_internal_slice_check(this_: &z_owned_slice_t) -> bool {
    !this_.as_rust_type_ref().is_empty()
}

/// @return the length of the slice.
#[no_mangle]
pub extern "C" fn z_slice_len(this_: &z_loaned_slice_t) -> usize {
    this_.as_rust_type_ref().len()
}

/// @return the pointer to the slice data.
#[no_mangle]
pub extern "C" fn z_slice_data(this_: &z_loaned_slice_t) -> *const u8 {
    this_.as_rust_type_ref().data()
}

/// @return ``true`` if slice is empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_slice_is_empty(this_: &z_loaned_slice_t) -> bool {
    this_.as_rust_type_ref().is_empty()
}

/// Constructs a slice by copying a `len` bytes long sequence starting at `start`.
///
/// @return -1 if `start == NULL` and `len > 0` (creating an empty slice), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_copy_from_buf(
    this: &mut MaybeUninit<z_owned_slice_t>,
    start: *const u8,
    len: usize,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    match CSliceOwned::new(start, len) {
        Ok(slice) => {
            this.write(slice);
            result::Z_OK
        }
        Err(e) => {
            this.write(CSliceOwned::gravestone());
            e
        }
    }
}

/// Constructs a slice by transferring ownership of `data` to it.
/// @param this_: Pointer to an uninitialized memoery location where slice will be constructed.
/// @param data: Pointer to the data to be owned by `this_`.
/// @param len: Number of bytes in `data`.
/// @param drop: A thread-safe delete function to free the `data`. Will be called once when `this_` is dropped. Can be NULL, in case if `data` is allocated in static memory.
/// @param context: An optional context to be passed to the `deleter`.
///
/// @return -1 if `start == NULL` and `len > 0` (creating an empty slice), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_from_buf(
    this: &mut MaybeUninit<z_owned_slice_t>,
    data: *mut u8,
    len: usize,
    drop: Option<extern "C" fn(data: *mut c_void, context: *mut c_void)>,
    context: *mut c_void,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    match CSliceOwned::wrap(data, len, drop, context) {
        Ok(slice) => {
            this.write(slice);
            result::Z_OK
        }
        Err(e) => {
            this.write(CSliceOwned::gravestone());
            e
        }
    }
}

pub use crate::opaque_types::{
    z_loaned_string_t, z_moved_string_t, z_owned_string_t, z_view_string_t,
};

// The wrappers which provides string-related interfaces to memory slice `CSlice`
// Unlike the standard `std:ffi::CString` these structures doesn't provide
// any guarantees about null-termination

#[derive(Clone)]
pub struct CStringInner(CSlice);
pub struct CStringOwned(CStringInner);
pub struct CStringView(CStringInner);

impl Gravestone for CStringInner {
    fn gravestone() -> Self {
        Self(CSlice::gravestone())
    }
    fn is_gravestone(&self) -> bool {
        self.0.is_gravestone()
    }
}

impl Gravestone for CStringOwned {
    fn gravestone() -> Self {
        Self(CStringInner::gravestone())
    }
    fn is_gravestone(&self) -> bool {
        self.0.is_gravestone()
    }
}

impl Gravestone for CStringView {
    fn gravestone() -> Self {
        Self(CStringInner::gravestone())
    }
    fn is_gravestone(&self) -> bool {
        self.0.is_gravestone()
    }
}

impl CStringInner {
    pub fn new_borrowed_from_slice(slice: &[u8]) -> Self {
        CStringInner(CSlice::new_borrowed_from_slice(slice))
    }
}

impl CStringOwned {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new(data: *const libc::c_char, len: usize) -> Result<Self, z_result_t> {
        Ok(CStringOwned(CStringInner(CSlice::new_owned(
            data as _, len,
        )?)))
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn wrap(
        data: *mut libc::c_char,
        len: usize,
        drop: Option<extern "C" fn(data: *mut c_void, context: *mut c_void)>,
        context: *mut c_void,
    ) -> Result<Self, z_result_t> {
        Ok(CStringOwned(CStringInner(CSlice::new(
            data as _, len, drop, context,
        )?)))
    }
}

impl CStringView {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new_borrowed(data: *const libc::c_char, len: usize) -> Result<Self, z_result_t> {
        Ok(CStringView(CStringInner(CSlice::new_borrowed(
            data as _, len,
        )?)))
    }
    pub fn new_borrowed_from_slice(slice: &[u8]) -> Self {
        CStringView(CStringInner::new_borrowed_from_slice(slice))
    }
}

impl Deref for CStringInner {
    type Target = CSlice;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for CStringOwned {
    type Target = CStringInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for CStringView {
    type Target = CStringInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<CSlice> for CStringInner {
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
        CStringOwned(CStringInner(CSlice::wrap(slice.as_ptr(), slice.len())))
    }
}

impl From<CStringInner> for CSlice {
    fn from(value: CStringInner) -> Self {
        value.0
    }
}

impl From<CStringOwned> for CSlice {
    fn from(value: CStringOwned) -> Self {
        value.0 .0
    }
}

decl_c_type!(
    owned(z_owned_string_t, CStringOwned),
    loaned(z_loaned_string_t, CStringInner),
    view(z_view_string_t, CStringView),
);

/// Frees memory and invalidates `z_owned_string_t`, putting it in gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_drop(this_: &mut z_moved_string_t) {
    let _ = this_.take_rust_type();
}

/// @return ``true`` if `this_` is a valid string, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_string_check(this_: &z_owned_string_t) -> bool {
    !this_.as_rust_type_ref().is_empty()
}

/// Constructs owned string in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_string_null(this_: &mut MaybeUninit<z_owned_string_t>) {
    this_
        .as_rust_type_mut_uninit()
        .write(CStringOwned::gravestone());
}

/// @return ``true`` if view string is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
pub extern "C" fn z_view_string_is_empty(this_: &z_view_string_t) -> bool {
    this_.as_rust_type_ref().is_empty()
}

/// Constructs an empty owned string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_empty(this_: &mut MaybeUninit<z_owned_string_t>) {
    this_
        .as_rust_type_mut_uninit()
        .write(CStringOwned::gravestone());
}

/// Constructs an empty view string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_string_empty(this_: &mut MaybeUninit<z_view_string_t>) {
    this_
        .as_rust_type_mut_uninit()
        .write(CStringView::gravestone());
}

/// Borrows string.
#[no_mangle]
pub extern "C" fn z_string_loan(this_: &z_owned_string_t) -> &z_loaned_string_t {
    this_.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Borrows view string.
#[no_mangle]
pub extern "C" fn z_view_string_loan(this_: &z_view_string_t) -> &z_loaned_string_t {
    this_.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Constructs an owned string by copying `str` into it (including terminating 0), using `strlen` (this should therefore not be used with untrusted inputs).
///
/// @return -1 if `str == NULL` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_copy_from_str(
    this_: &mut MaybeUninit<z_owned_string_t>,
    str: *const libc::c_char,
) -> z_result_t {
    z_string_copy_from_substr(this_, str, strlen(str))
}

/// Constructs an owned string by copying a `str` substring of length `len`.
///
/// @return -1 if `str == NULL` and `len > 0` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_copy_from_substr(
    this: &mut MaybeUninit<z_owned_string_t>,
    str: *const libc::c_char,
    len: usize,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    match CStringOwned::new(str, len) {
        Ok(slice) => {
            this.write(slice);
            result::Z_OK
        }
        Err(e) => {
            this.write(CStringOwned::gravestone());
            e
        }
    }
}

/// Constructs an owned string by transferring ownership of a null-terminated string `str` to it.
/// @param this_: Pointer to an uninitialized memory location where an owned string will be constructed.
/// @param str: Pointer to a null terminated string to be owned by `this_`.
/// @param drop: A thread-safe delete function to free the `str`. Will be called once when `str` is dropped. Can be NULL, in case if `str` is allocated in static memory.
/// @param context: An optional context to be passed to the `deleter`.
/// @return -1 if `str == NULL` and `len > 0` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_from_str(
    this: &mut MaybeUninit<z_owned_string_t>,
    str: *mut libc::c_char,
    drop: Option<extern "C" fn(value: *mut c_void, context: *mut c_void)>,
    context: *mut c_void,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    match CStringOwned::wrap(str, libc::strlen(str), drop, context) {
        Ok(slice) => {
            this.write(slice);
            result::Z_OK
        }
        Err(e) => {
            this.write(CStringOwned::gravestone());
            e
        }
    }
}

/// Constructs a view string of `str`, using `strlen` (this should therefore not be used with untrusted inputs).
///
/// @return -1 if `str == NULL` (and creates a string in a gravestone state), 0 otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_view_string_from_str(
    this: &mut MaybeUninit<z_view_string_t>,
    str: *const libc::c_char,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    match CStringView::new_borrowed(str, strlen(str)) {
        Ok(slice) => {
            this.write(slice);
            result::Z_OK
        }
        Err(e) => {
            this.write(CStringView::gravestone());
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
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    match CStringView::new_borrowed(str, len) {
        Ok(slice) => {
            this.write(slice);
            result::Z_OK
        }
        Err(e) => {
            this.write(CStringView::gravestone());
            e
        }
    }
}

/// @return the length of the string (without terminating 0 character).
#[no_mangle]
pub extern "C" fn z_string_len(this_: &z_loaned_string_t) -> usize {
    this_.as_rust_type_ref().len()
}

/// @return the pointer of the string data.
#[no_mangle]
pub extern "C" fn z_string_data(this_: &z_loaned_string_t) -> *const libc::c_char {
    this_.as_rust_type_ref().data() as _
}

/// Constructs an owned copy of a string.
#[no_mangle]
pub extern "C" fn z_string_clone(
    dst: &mut MaybeUninit<z_owned_string_t>,
    this: &z_loaned_string_t,
) {
    let slice = this.as_rust_type_ref().clone_to_owned();
    dst.as_rust_type_mut_uninit()
        .write(CStringOwned(CStringInner(slice.0)));
}

// Converts loaned string into loaned slice (with terminating 0 character).
#[no_mangle]
pub extern "C" fn z_string_as_slice(this_: &z_loaned_string_t) -> &z_loaned_slice_t {
    this_.as_rust_type_ref().as_ref().as_loaned_c_type_ref()
}

/// @return ``true`` if string is empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_string_is_empty(this_: &z_loaned_string_t) -> bool {
    this_.as_rust_type_ref().is_empty()
}

pub use crate::opaque_types::{
    z_loaned_string_array_t, z_moved_string_array_t, z_owned_string_array_t,
};
pub type ZVector = Vec<CStringInner>;
decl_c_type!(
    owned(z_owned_string_array_t, ZVector),
    loaned(z_loaned_string_array_t),
);

impl Gravestone for ZVector {
    fn gravestone() -> Self {
        Vec::new()
    }
    fn is_gravestone(&self) -> bool {
        self.is_empty()
    }
}

/// Constructs a new empty string array.
#[no_mangle]
pub extern "C" fn z_string_array_new(this_: &mut MaybeUninit<z_owned_string_array_t>) {
    this_.as_rust_type_mut_uninit().write(ZVector::gravestone());
}

/// Constructs string array in its gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_string_array_null(this_: &mut MaybeUninit<z_owned_string_array_t>) {
    z_string_array_new(this_)
}

/// @return ``true`` if the string array is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_string_array_check(this_: &z_owned_string_array_t) -> bool {
    !this_.as_rust_type_ref().is_empty()
}

/// Destroys the string array, resetting it to its gravestone value.
#[no_mangle]
pub extern "C" fn z_string_array_drop(this_: &mut z_moved_string_array_t) {
    let _ = this_.take_rust_type();
}

/// Borrows string array.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_array_loan(
    this: &z_owned_string_array_t,
) -> &z_loaned_string_array_t {
    this.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Mutably borrows string array.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_array_loan_mut(
    this: &mut z_owned_string_array_t,
) -> &mut z_loaned_string_array_t {
    this.as_rust_type_mut().as_loaned_c_type_mut()
}

/// @return number of elements in the array.
#[no_mangle]
pub extern "C" fn z_string_array_len(this_: &z_loaned_string_array_t) -> usize {
    this_.as_rust_type_ref().len()
}

/// @return ``true`` if the array is empty, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_string_array_is_empty(this_: &z_loaned_string_array_t) -> bool {
    this_.as_rust_type_ref().is_empty()
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
    this.push(CStringInner(v.clone_to_owned().into()));

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
    this.push(CStringInner(v.clone_to_borrowed()));

    this.len()
}

/// Constructs an owned copy of a string array.
#[no_mangle]
pub extern "C" fn z_string_array_clone(
    dst: &mut MaybeUninit<z_owned_string_array_t>,
    this_: &z_loaned_string_array_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(this_.as_rust_type_ref().clone());
}
