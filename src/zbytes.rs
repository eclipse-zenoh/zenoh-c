//
// Copyright (c) 2024 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use core::fmt;
use std::{
    any::Any,
    io::{Read, Seek, SeekFrom, Write},
    mem::MaybeUninit,
    os::raw::c_void,
    ptr::null_mut,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use zenoh::{
    bytes::{ZBytes, ZBytesReader, ZBytesSliceIterator, ZBytesWriter},
    internal::buffers::{ZBuf, ZSliceBuffer},
};

pub use crate::opaque_types::{z_loaned_bytes_t, z_owned_bytes_t};
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::result::Z_ENULL;
use crate::{
    result::{self, z_result_t, Z_EINVAL, Z_EIO, Z_OK},
    transmute::{Gravestone, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_loaned_slice_t, z_loaned_string_t, z_moved_bytes_t, z_moved_slice_t, z_moved_string_t,
    z_owned_slice_t, z_owned_string_t, z_view_slice_t, CSlice, CSliceOwned, CSliceView, CString,
    CStringOwned,
};
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::{z_loaned_shm_t, z_moved_shm_mut_t, z_moved_shm_t, z_owned_shm_t};
decl_c_type! {
    owned(z_owned_bytes_t, ZBytes),
    loaned(z_loaned_bytes_t),
}

impl Gravestone for ZBytes {
    fn gravestone() -> Self {
        ZBytes::default()
    }
    fn is_gravestone(&self) -> bool {
        self.is_empty()
    }
}

/// The gravestone value for `z_owned_bytes_t`.
#[no_mangle]
extern "C" fn z_internal_bytes_null(this: &mut MaybeUninit<z_owned_bytes_t>) {
    this.as_rust_type_mut_uninit().write(ZBytes::default());
}

/// Constructs an empty instance of `z_owned_bytes_t`.
#[no_mangle]
extern "C" fn z_bytes_empty(this: &mut MaybeUninit<z_owned_bytes_t>) {
    this.as_rust_type_mut_uninit().write(ZBytes::default());
}

/// Drops `this_`, resetting it to gravestone value. If there are any shallow copies
/// created by `z_bytes_clone()`, they would still stay valid.
#[no_mangle]
extern "C" fn z_bytes_drop(this_: &mut z_moved_bytes_t) {
    let _ = this_.take_rust_type();
}

/// Returns ``true`` if `this_` is in a valid state, ``false`` if it is in a gravestone state.
#[no_mangle]
extern "C" fn z_internal_bytes_check(this: &z_owned_bytes_t) -> bool {
    !this.as_rust_type_ref().is_empty()
}

/// Borrows data.
#[no_mangle]
unsafe extern "C" fn z_bytes_loan(this: &z_owned_bytes_t) -> &z_loaned_bytes_t {
    this.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Muatably borrows data.
#[no_mangle]
extern "C" fn z_bytes_loan_mut(this: &mut z_owned_bytes_t) -> &mut z_loaned_bytes_t {
    this.as_rust_type_mut().as_loaned_c_type_mut()
}

/// Returns ``true`` if `this_` is empty, ``false`` otherwise.
#[no_mangle]
extern "C" fn z_bytes_is_empty(this: &z_loaned_bytes_t) -> bool {
    this.as_rust_type_ref().is_empty()
}

/// Constructs an owned shallow copy of data in provided uninitialized memory location.
#[no_mangle]
extern "C" fn z_bytes_clone(dst: &mut MaybeUninit<z_owned_bytes_t>, this: &z_loaned_bytes_t) {
    dst.as_rust_type_mut_uninit()
        .write(this.as_rust_type_ref().clone());
}

/// Returns total number of bytes in the payload.
#[no_mangle]
extern "C" fn z_bytes_len(this: &z_loaned_bytes_t) -> usize {
    this.as_rust_type_ref().len()
}

/// Converts data into an owned non-null-terminated string.
///
/// @param this_: Data to convert.
/// @param dst: An uninitialized memory location where to construct a string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_to_string(
    this: &z_loaned_bytes_t,
    dst: &mut MaybeUninit<z_owned_string_t>,
) -> z_result_t {
    let payload = this.as_rust_type_ref();
    match payload.try_to_string() {
        Ok(s) => {
            dst.as_rust_type_mut_uninit().write(s.into_owned().into());
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("Failed to convert the payload: {}", e);
            dst.as_rust_type_mut_uninit()
                .write(CStringOwned::gravestone());
            result::Z_EINVAL
        }
    }
}

/// Converts data into an owned slice.
///
/// @param this_: Data to convert.
/// @param dst: An uninitialized memory location where to construct a slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_to_slice(
    this: &z_loaned_bytes_t,
    dst: &mut MaybeUninit<z_owned_slice_t>,
) -> z_result_t {
    let payload = this.as_rust_type_ref();
    dst.as_rust_type_mut_uninit()
        .write(payload.to_bytes().into_owned().into());
    result::Z_OK
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Converts data into an owned SHM buffer by copying it's shared reference.
///
/// @param this_: Data to convert.
/// @param dst: An uninitialized memory location where to construct an SHM buffer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_to_owned_shm(
    this: &z_loaned_bytes_t,
    dst: &mut MaybeUninit<z_owned_shm_t>,
) -> z_result_t {
    let payload = this.as_rust_type_ref();
    match payload.as_shm() {
        Some(s) => {
            dst.as_rust_type_mut_uninit().write(Some(s.to_owned()));
            result::Z_OK
        }
        None => {
            tracing::error!("Failed to convert the payload");
            dst.as_rust_type_mut_uninit().write(None);
            result::Z_EINVAL
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Converts data into a loaned SHM buffer.
///
/// @param this_: Data to convert.
/// @param dst: An uninitialized memory location where to construct an SHM buffer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_as_loaned_shm(
    this: &'static z_loaned_bytes_t,
    dst: &'static mut MaybeUninit<&'static z_loaned_shm_t>,
) -> z_result_t {
    let payload = this.as_rust_type_ref();
    match payload.as_shm() {
        Some(s) => {
            dst.write(s.as_loaned_c_type_ref());
            result::Z_OK
        }
        None => {
            tracing::error!("Failed to convert the payload");
            result::Z_EINVAL
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Converts data into a mutably loaned SHM buffer.
///
/// @param this_: Data to convert.
/// @param dst: An uninitialized memory location where to construct an SHM buffer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_as_mut_loaned_shm(
    this: &'static mut z_loaned_bytes_t,
    dst: &'static mut MaybeUninit<&'static mut z_loaned_shm_t>,
) -> z_result_t {
    let payload = this.as_rust_type_mut();
    match payload.as_shm_mut() {
        Some(s) => {
            dst.write(s.as_loaned_c_type_mut());
            result::Z_OK
        }
        None => {
            tracing::error!("Failed to convert the payload");
            result::Z_EINVAL
        }
    }
}

unsafe impl Send for CSlice {}
unsafe impl Sync for CSlice {}

impl fmt::Debug for CSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CSLice").field("_0", &self.slice()).finish()
    }
}

impl ZSliceBuffer for CSlice {
    fn as_slice(&self) -> &[u8] {
        self.slice()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl From<CSlice> for ZBytes {
    fn from(value: CSlice) -> Self {
        ZBytes::from(ZBuf::from(value))
    }
}

impl From<CSliceOwned> for ZBytes {
    fn from(value: CSliceOwned) -> Self {
        let value: CSlice = value.into();
        ZBytes::from(value)
    }
}
impl From<CString> for ZBytes {
    fn from(value: CString) -> Self {
        let value: CSlice = value.into();
        ZBytes::from(value)
    }
}

impl From<CStringOwned> for ZBytes {
    fn from(value: CStringOwned) -> Self {
        let value: CSlice = value.into();
        ZBytes::from(value)
    }
}

/// Converts a slice into `z_owned_bytes_t`.
/// The slice is consumed upon function return.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_slice(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    slice: &mut z_moved_slice_t,
) {
    let slice = slice.take_rust_type();
    let payload = ZBytes::from(slice);
    this.as_rust_type_mut_uninit().write(payload);
}

/// Converts a slice into `z_owned_bytes_t` by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_copy_from_slice(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    slice: &z_loaned_slice_t,
) {
    let slice = slice.as_rust_type_ref();
    let payload = ZBytes::from(slice.clone_to_owned());
    this.as_rust_type_mut_uninit().write(payload);
}

/// Converts buffer into `z_owned_bytes_t`.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param data: A pointer to the buffer containing data. `this_` will take ownership of the buffer.
/// @param len: Length of the buffer.
/// @param deleter: A thread-safe function, that will be called on `data` when `this_` is dropped. Can be `NULL` if `data` is located in static memory and does not require a drop.
/// @param context: An optional context to be passed to `deleter`.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_buf(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    data: *mut u8,
    len: usize,
    deleter: Option<extern "C" fn(data: *mut c_void, context: *mut c_void)>,
    context: *mut c_void,
) -> z_result_t {
    if let Ok(s) = CSliceOwned::wrap(data, len, deleter, context) {
        this.as_rust_type_mut_uninit().write(ZBytes::from(s));
        Z_OK
    } else {
        this.as_rust_type_mut_uninit().write(ZBytes::default());
        Z_EINVAL
    }
}

/// Converts a statically allocated constant buffer into `z_owned_bytes_t`.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param data: A pointer to the statically allocated constant data.
/// @param len: A length of the buffer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_static_buf(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    data: *mut u8,
    len: usize,
) -> z_result_t {
    if let Ok(s) = CSliceOwned::wrap(data as _, len, None, null_mut()) {
        this.as_rust_type_mut_uninit().write(ZBytes::from(s));
        Z_OK
    } else {
        this.as_rust_type_mut_uninit().write(ZBytes::default());
        Z_EINVAL
    }
}

/// Converts a data from buffer into `z_owned_bytes_t` by copying.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param data: A pointer to the buffer containing data.
/// @param len: Length of the buffer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_copy_from_buf(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    data: *const u8,
    len: usize,
) -> z_result_t {
    if let Ok(s) = CSliceOwned::new(data, len) {
        this.as_rust_type_mut_uninit().write(ZBytes::from(s));
        Z_OK
    } else {
        this.as_rust_type_mut_uninit().write(ZBytes::default());
        Z_EINVAL
    }
}

/// Converts a string into `z_owned_bytes_t`.
/// The string is consumed upon function return.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_string(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    s: &mut z_moved_string_t,
) {
    // TODO: verify that string is a valid utf-8 string ?
    let cs = s.take_rust_type();
    let payload = ZBytes::from(cs);
    this.as_rust_type_mut_uninit().write(payload);
}

/// Converts a string into `z_owned_bytes_t` by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_copy_from_string(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    str: &z_loaned_string_t,
) {
    let s = str.as_rust_type_ref();
    let payload = ZBytes::from(s.clone_to_owned());
    this.as_rust_type_mut_uninit().write(payload);
}

/// Converts a null-terminated string into `z_owned_bytes_t`.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param str: a pointer to the string. `this_` will take ownership of the string.
/// @param deleter: A thread-safe function, that will be called on `str` when `this_` is dropped. Can be `NULL` if `str` is located in static memory and does not require a drop.
/// @param context: An optional context to be passed to `deleter`.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_str(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    str: *mut libc::c_char,
    deleter: Option<extern "C" fn(data: *mut c_void, context: *mut c_void)>,
    context: *mut c_void,
) -> z_result_t {
    if let Ok(s) = CStringOwned::wrap(str, libc::strlen(str), deleter, context) {
        this.as_rust_type_mut_uninit().write(ZBytes::from(s));
        Z_OK
    } else {
        this.as_rust_type_mut_uninit().write(ZBytes::default());
        Z_EINVAL
    }
}

/// Converts a statically allocated constant null-terminated string into `z_owned_bytes_t` by aliasing.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param str: a pointer to the statically allocated constant string.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_static_str(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    str: *const libc::c_char,
) -> z_result_t {
    if let Ok(s) = CStringOwned::wrap(str as _, libc::strlen(str), None, null_mut()) {
        this.as_rust_type_mut_uninit().write(ZBytes::from(s));
        Z_OK
    } else {
        this.as_rust_type_mut_uninit().write(ZBytes::default());
        Z_EINVAL
    }
}

/// Converts a null-terminated string into `z_owned_bytes_t` by copying.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param str: a pointer to the null-terminated string.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_copy_from_str(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    str: *const libc::c_char,
) -> z_result_t {
    if let Ok(s) = CStringOwned::new(str, libc::strlen(str)) {
        this.as_rust_type_mut_uninit().write(ZBytes::from(s));
        Z_OK
    } else {
        this.as_rust_type_mut_uninit().write(ZBytes::default());
        Z_EINVAL
    }
}

pub use crate::z_bytes_slice_iterator_t;
decl_c_type!(loaned(z_bytes_slice_iterator_t, ZBytesSliceIterator<'static>));

/// Returns an iterator on raw bytes slices contained in the `z_loaned_bytes_t`.
///
/// Zenoh may store data in non-contiguous regions of memory, this iterator
/// then allows to access raw data directly without any attempt of deserializing it.
/// Please note that no guarantee is provided on the internal memory layout.
/// The only provided guarantee is on the bytes order that is preserved.
#[no_mangle]
pub extern "C" fn z_bytes_get_slice_iterator(
    this: &'static z_loaned_bytes_t,
) -> z_bytes_slice_iterator_t {
    *this.as_rust_type_ref().slices().as_loaned_c_type_ref()
}

/// Gets next slice.
/// @param this_: Slice iterator.
/// @param slice: An unitialized memory location where the view for the next slice will be constructed.
/// @return `false` if there are no more slices (in this case slice will stay unchanged), `true` otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_slice_iterator_next(
    this: &mut z_bytes_slice_iterator_t,
    slice: &mut MaybeUninit<z_view_slice_t>,
) -> bool {
    match this.as_rust_type_mut().next() {
        Some(s) => {
            slice
                .as_rust_type_mut_uninit()
                .write(CSliceView::from_slice(s));
            true
        }
        None => false,
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Attempts to get a contiguous view to the underlying bytes.
/// This is only possible if data is not fragmented, otherwise the function will fail.
/// In case of fragmented data, consider using `z_bytes_get_slice_iterator()`.
///
/// @param this_: An instance of Zenoh data.
/// @param view: An uninitialized memory location where a contiguous view on data will be constructed.
/// @return  ​0​ upon success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_get_contiguous_view(
    this: &'static z_loaned_bytes_t,
    view: &mut MaybeUninit<z_view_slice_t>,
) -> result::z_result_t {
    let payload = this.as_rust_type_ref();
    match payload.to_bytes() {
        std::borrow::Cow::Borrowed(s) => {
            view.as_rust_type_mut_uninit()
                .write(CSliceView::from_slice(s));
            result::Z_OK
        }
        std::borrow::Cow::Owned(_) => result::Z_EINVAL,
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Converts from an immutable SHM buffer consuming it.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_shm(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    shm: &mut z_moved_shm_t,
) -> z_result_t {
    let Some(shm) = shm.take_rust_type() else {
        this.as_rust_type_mut_uninit().write(ZBytes::default());
        return Z_ENULL;
    };
    this.as_rust_type_mut_uninit().write(shm.into());
    Z_OK
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Converts a mutable SHM buffer consuming it.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_shm_mut(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    shm: &mut z_moved_shm_mut_t,
) -> z_result_t {
    let Some(shm) = shm.take_rust_type() else {
        this.as_rust_type_mut_uninit().write(ZBytes::default());
        return Z_ENULL;
    };
    this.as_rust_type_mut_uninit().write(shm.into());
    Z_OK
}

pub use crate::z_bytes_reader_t;
decl_c_type!(loaned(z_bytes_reader_t, ZBytesReader<'static>));

/// Returns a reader for the data.
///
/// The `data` should outlive the reader.
#[no_mangle]
pub extern "C" fn z_bytes_get_reader(data: &'static z_loaned_bytes_t) -> z_bytes_reader_t {
    *data.as_rust_type_ref().reader().as_loaned_c_type_ref()
}

/// Reads data into specified destination.
///
/// @param this_: Data reader to read from.
/// @param dst: Buffer where the read data is written.
/// @param len: Maximum number of bytes to read.
/// @return number of bytes read. If return value is smaller than `len`, it means that  theend of the data was reached.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_read(
    this: &mut z_bytes_reader_t,
    dst: *mut u8,
    len: usize,
) -> usize {
    let reader = this.as_rust_type_mut();
    let buf = unsafe { from_raw_parts_mut(dst, len) };
    reader.read(buf).unwrap_or(0)
}

/// Sets the `reader` position indicator for the payload to the value pointed to by offset.
/// The new position is exactly `offset` bytes measured from the beginning of the payload if origin is `SEEK_SET`,
/// from the current reader position if origin is `SEEK_CUR`, and from the end of the payload if origin is `SEEK_END`.
/// @return ​0​ upon success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_seek(
    this: &mut z_bytes_reader_t,
    offset: i64,
    origin: libc::c_int,
) -> z_result_t {
    let reader = this.as_rust_type_mut();
    let pos = match origin {
        libc::SEEK_SET => match offset.try_into() {
            Ok(o) => SeekFrom::Start(o),
            Err(_) => {
                return result::Z_EINVAL;
            }
        },
        libc::SEEK_CUR => SeekFrom::Current(offset),
        libc::SEEK_END => SeekFrom::End(offset),
        _ => {
            return result::Z_EINVAL;
        }
    };

    match reader.seek(pos) {
        Ok(_) => result::Z_OK,
        Err(_) => result::Z_EINVAL,
    }
}

/// Gets the read position indicator.
/// @return read position indicator on success or -1L if failure occurs.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_tell(this_: &mut z_bytes_reader_t) -> i64 {
    let reader = this_.as_rust_type_mut();
    reader.stream_position().map(|p| p as i64).unwrap_or(-1)
}

/// Gets the number of bytes that can still be read.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_remaining(this_: &z_bytes_reader_t) -> usize {
    let reader = this_.as_rust_type_ref();
    reader.remaining()
}

pub use crate::opaque_types::{
    z_loaned_bytes_writer_t, z_moved_bytes_writer_t, z_owned_bytes_writer_t,
};

decl_c_type! {
    owned(z_owned_bytes_writer_t, option ZBytesWriter),
    loaned(z_loaned_bytes_writer_t),
}

/// @brief Constructs a data writer with empty payload.
/// @param this_: An uninitialized memory location where writer is to be constructed.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
extern "C" fn z_bytes_writer_empty(this: &mut MaybeUninit<z_owned_bytes_writer_t>) -> z_result_t {
    this.as_rust_type_mut_uninit().write(Some(ZBytes::writer()));
    result::Z_OK
}

/// Drops `this_`, resetting it to gravestone value.
#[no_mangle]
extern "C" fn z_bytes_writer_drop(this_: &mut z_moved_bytes_writer_t) {
    let _ = this_.take_rust_type();
}

/// Returns ``true`` if `this_` is in a valid state, ``false`` if it is in a gravestone state.
#[no_mangle]
extern "C" fn z_internal_bytes_writer_check(this: &z_owned_bytes_writer_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Borrows writer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_writer_loan(
    this: &z_owned_bytes_writer_t,
) -> &z_loaned_bytes_writer_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Muatably borrows writer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_writer_loan_mut(
    this: &mut z_owned_bytes_writer_t,
) -> &mut z_loaned_bytes_writer_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// Constructs a writer in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_bytes_writer_null(this_: &mut MaybeUninit<z_owned_bytes_writer_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @brief Drop writer and extract underlying `bytes` object it was writing to.
/// @param this_: A writer instance.
/// @param bytes: An uninitialized memory location where `bytes` object` will be written to.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_bytes_writer_finish(
    this: &mut z_moved_bytes_writer_t,
    bytes: &mut MaybeUninit<z_owned_bytes_t>,
) {
    bytes
        .as_rust_type_mut_uninit()
        .write(this.take_rust_type().unwrap_unchecked().finish());
}

/// Writes `len` bytes from `src` into underlying data.
///
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
unsafe extern "C" fn z_bytes_writer_write_all(
    this: &mut z_loaned_bytes_writer_t,
    src: *const u8,
    len: usize,
) -> z_result_t {
    match this.as_rust_type_mut().write_all(from_raw_parts(src, len)) {
        Ok(_) => Z_OK,
        Err(_) => Z_EIO,
    }
}

/// Appends bytes.     
/// This allows to compose a serialized data out of multiple `z_owned_bytes_t` that may point to different memory regions.
/// Said in other terms, it allows to create a linear view on different memory regions without copy.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
extern "C" fn z_bytes_writer_append(
    this: &mut z_loaned_bytes_writer_t,
    bytes: &mut z_moved_bytes_t,
) -> z_result_t {
    this.as_rust_type_mut().append(bytes.take_rust_type());
    result::Z_OK
}
