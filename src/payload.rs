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
    bytes::{
        Deserialize, Serialize, ZBytes, ZBytesIterator, ZBytesReader, ZBytesSliceIterator,
        ZBytesWriter, ZSerde,
    },
    internal::buffers::{ZBuf, ZSliceBuffer},
};

pub use crate::opaque_types::{z_loaned_bytes_t, z_owned_bytes_t};
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::result::Z_ENULL;
use crate::{
    result::{self, z_result_t, Z_EINVAL, Z_EIO, Z_EPARSE, Z_OK},
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
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

/// Deserializes data into an owned non-null-terminated string.
///
/// @param this_: Data to deserialize.
/// @param dst: An uninitialized memory location where to construct a deserialized string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_string(
    this: &z_loaned_bytes_t,
    dst: &mut MaybeUninit<z_owned_string_t>,
) -> z_result_t {
    let payload = this.as_rust_type_ref();
    match payload.deserialize::<String>() {
        Ok(s) => {
            dst.as_rust_type_mut_uninit().write(s.into());
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("Failed to deserialize the payload: {}", e);
            dst.as_rust_type_mut_uninit().write(CStringOwned::default());
            result::Z_EIO
        }
    }
}

/// Deserializes data into an owned slice.
///
/// @param this_: Data to deserialize.
/// @param dst: An uninitialized memory location where to construct a slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_slice(
    this: &z_loaned_bytes_t,
    dst: &mut MaybeUninit<z_owned_slice_t>,
) -> z_result_t {
    let payload = this.as_rust_type_ref();
    match payload.deserialize::<Vec<u8>>() {
        Ok(v) => {
            dst.as_rust_type_mut_uninit().write(v.into());
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("Failed to read the payload: {}", e);
            dst.as_rust_type_mut_uninit().write(CSliceOwned::default());
            result::Z_EIO
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deserializes data into an owned SHM buffer by copying it's shared reference.
///
/// @param this_: Data to deserialize.
/// @param dst: An uninitialized memory location where to construct a deserialized string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_owned_shm(
    this: &z_loaned_bytes_t,
    dst: &mut MaybeUninit<z_owned_shm_t>,
) -> z_result_t {
    use zenoh::shm::zshm;

    let payload = this.as_rust_type_ref();
    match payload.deserialize::<&zshm>() {
        Ok(s) => {
            dst.as_rust_type_mut_uninit().write(Some(s.to_owned()));
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("Failed to deserialize the payload: {:?}", e);
            dst.as_rust_type_mut_uninit().write(None);
            result::Z_EIO
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deserializes data into a loaned SHM buffer.
///
/// @param this_: Data to deserialize.
/// @param dst: An uninitialized memory location where to construct a deserialized SHM buffer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_loaned_shm(
    this: &'static z_loaned_bytes_t,
    dst: &'static mut MaybeUninit<&'static z_loaned_shm_t>,
) -> z_result_t {
    use zenoh::shm::zshm;

    let payload = this.as_rust_type_ref();
    match payload.deserialize::<&zshm>() {
        Ok(s) => {
            dst.write(s.as_loaned_c_type_ref());
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("Failed to deserialize the payload: {:?}", e);
            result::Z_EIO
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deserializes data into a mutably loaned SHM buffer.
///
/// @param this_: Data to deserialize.
/// @param dst: An uninitialized memory location where to construct a deserialized SHM buffer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_mut_loaned_shm(
    this: &'static mut z_loaned_bytes_t,
    dst: &'static mut MaybeUninit<&'static mut z_loaned_shm_t>,
) -> z_result_t {
    use zenoh::shm::zshm;

    let payload = this.as_rust_type_mut();
    match payload.deserialize_mut::<&mut zshm>() {
        Ok(s) => {
            dst.write(s.as_loaned_c_type_mut());
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("Failed to deserialize the payload: {:?}", e);
            result::Z_EIO
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
        ZBytes::new(value)
    }
}

impl From<CSliceOwned> for ZBytes {
    fn from(value: CSliceOwned) -> Self {
        let value: CSlice = value.into();
        ZBytes::new(value)
    }
}
impl From<CString> for ZBytes {
    fn from(value: CString) -> Self {
        let value: CSlice = value.into();
        ZBytes::new(value)
    }
}

impl From<CStringOwned> for ZBytes {
    fn from(value: CStringOwned) -> Self {
        let value: CSlice = value.into();
        ZBytes::new(value)
    }
}

fn z_bytes_serialize_from_arithmetic<T>(this: &mut MaybeUninit<z_owned_bytes_t>, val: T)
where
    ZSerde: Serialize<T, Output = ZBytes>,
{
    this.as_rust_type_mut_uninit().write(ZBytes::serialize(val));
}

fn z_bytes_deserialize_into_arithmetic<'a, T>(
    this: &'a z_loaned_bytes_t,
    val: &'a mut T,
) -> z_result_t
where
    ZSerde: Deserialize<T, Input<'a> = &'a ZBytes>,
    <ZSerde as Deserialize<T>>::Error: fmt::Debug,
{
    match this.as_rust_type_ref().deserialize::<T>() {
        Ok(v) => {
            *val = v;
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("Failed to deserialize the payload: {:?}", e);
            result::Z_EPARSE
        }
    }
}

/// Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_uint8(this_: &mut MaybeUninit<z_owned_bytes_t>, val: u8) {
    z_bytes_serialize_from_arithmetic::<u8>(this_, val);
}

/// Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_uint16(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: u16,
) {
    z_bytes_serialize_from_arithmetic::<u16>(this_, val);
}

/// Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_uint32(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: u32,
) {
    z_bytes_serialize_from_arithmetic::<u32>(this_, val);
}

/// Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_uint64(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: u64,
) {
    z_bytes_serialize_from_arithmetic::<u64>(this_, val);
}

/// Serializes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_int8(this_: &mut MaybeUninit<z_owned_bytes_t>, val: i8) {
    z_bytes_serialize_from_arithmetic::<i8>(this_, val);
}

/// Serializes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_int16(this_: &mut MaybeUninit<z_owned_bytes_t>, val: i16) {
    z_bytes_serialize_from_arithmetic::<i16>(this_, val);
}

/// Serializes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_int32(this_: &mut MaybeUninit<z_owned_bytes_t>, val: i32) {
    z_bytes_serialize_from_arithmetic::<i32>(this_, val);
}

/// Serializes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_int64(this_: &mut MaybeUninit<z_owned_bytes_t>, val: i64) {
    z_bytes_serialize_from_arithmetic::<i64>(this_, val);
}

/// Serializes a float.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_float(this_: &mut MaybeUninit<z_owned_bytes_t>, val: f32) {
    z_bytes_serialize_from_arithmetic::<f32>(this_, val);
}

/// Serializes a double.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_double(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: f64,
) {
    z_bytes_serialize_from_arithmetic::<f64>(this_, val);
}
/// Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_uint8(
    this: &z_loaned_bytes_t,
    dst: &mut u8,
) -> z_result_t {
    z_bytes_deserialize_into_arithmetic::<u8>(this, dst)
}

/// Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_uint16(
    this: &z_loaned_bytes_t,
    dst: &mut u16,
) -> z_result_t {
    z_bytes_deserialize_into_arithmetic::<u16>(this, dst)
}

/// Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_uint32(
    this: &z_loaned_bytes_t,
    dst: &mut u32,
) -> z_result_t {
    z_bytes_deserialize_into_arithmetic::<u32>(this, dst)
}

/// Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_uint64(
    this: &z_loaned_bytes_t,
    dst: &mut u64,
) -> z_result_t {
    z_bytes_deserialize_into_arithmetic::<u64>(this, dst)
}

/// Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_int8(
    this: &z_loaned_bytes_t,
    dst: &mut i8,
) -> z_result_t {
    z_bytes_deserialize_into_arithmetic::<i8>(this, dst)
}

/// Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_int16(
    this: &z_loaned_bytes_t,
    dst: &mut i16,
) -> z_result_t {
    z_bytes_deserialize_into_arithmetic::<i16>(this, dst)
}

/// Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_int32(
    this: &z_loaned_bytes_t,
    dst: &mut i32,
) -> z_result_t {
    z_bytes_deserialize_into_arithmetic::<i32>(this, dst)
}

/// Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_int64(
    this: &z_loaned_bytes_t,
    dst: &mut i64,
) -> z_result_t {
    z_bytes_deserialize_into_arithmetic::<i64>(this, dst)
}

/// Deserializes into a float.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_float(
    this: &z_loaned_bytes_t,
    dst: &mut f32,
) -> z_result_t {
    z_bytes_deserialize_into_arithmetic::<f32>(this, dst)
}

/// Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_double(
    this: &z_loaned_bytes_t,
    dst: &mut f64,
) -> z_result_t {
    z_bytes_deserialize_into_arithmetic::<f64>(this, dst)
}

/// Serializes a slice.
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

/// Serializes a slice by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_slice(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    slice: &z_loaned_slice_t,
) {
    let slice = slice.as_rust_type_ref();
    let payload = ZBytes::from(slice.clone_to_owned());
    this.as_rust_type_mut_uninit().write(payload);
}

/// Serializes a data from buffer.
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

/// Serializes a statically allocated constant data.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param data: A pointer to the statically allocated constant data.
/// @param len: Number of bytes to serialize.
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

/// Serializes a data from buffer by copying.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param data: A pointer to the buffer containing data.
/// @param len: Length of the buffer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_buf(
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

/// Serializes a string.
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

/// Serializes a string by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_string(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    str: &z_loaned_string_t,
) {
    let s = str.as_rust_type_ref();
    let payload = ZBytes::from(s.clone_to_owned());
    this.as_rust_type_mut_uninit().write(payload);
}

/// Serializes a null-terminated string.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param str: a pointer to the string. `this_` will take ownership of the buffer.
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

/// Serializes a statically allocated constant null-terminated string by aliasing.
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

/// Serializes a null-terminated string by copying.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param str: a pointer to the null-terminated string. `this_` will take ownership of the string.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_str(
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

/// Serializes a pair of `z_owned_bytes_t` objects which are consumed in the process.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_from_pair(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    first: &mut z_moved_bytes_t,
    second: &mut z_moved_bytes_t,
) -> z_result_t {
    let payload = ZBytes::serialize((first.take_rust_type(), second.take_rust_type()));
    this.as_rust_type_mut_uninit().write(payload);
    Z_OK
}

/// Deserializes into a pair of `z_owned_bytes_t` objects.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_pair(
    this: &z_loaned_bytes_t,
    first: &mut MaybeUninit<z_owned_bytes_t>,
    second: &mut MaybeUninit<z_owned_bytes_t>,
) -> z_result_t {
    match this.as_rust_type_ref().deserialize::<(ZBytes, ZBytes)>() {
        Ok((a, b)) => {
            first.as_rust_type_mut_uninit().write(a);
            second.as_rust_type_mut_uninit().write(b);
            Z_OK
        }
        Err(e) => {
            first.as_rust_type_mut_uninit().write(ZBytes::default());
            second.as_rust_type_mut_uninit().write(ZBytes::default());
            tracing::error!("Failed to deserialize the payload: {:?}", e);
            Z_EPARSE
        }
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

struct ZBytesInIterator {
    body: extern "C" fn(data: &mut MaybeUninit<z_owned_bytes_t>, context: *mut c_void) -> bool,
    context: *mut c_void,
}

impl Iterator for ZBytesInIterator {
    type Item = ZBuf;

    fn next(&mut self) -> Option<ZBuf> {
        let mut data = MaybeUninit::<z_owned_bytes_t>::uninit();

        if !(self.body)(&mut data, self.context) {
            return None;
        }

        let mut data = unsafe { data.assume_init() };
        let buf = std::mem::take(data.as_rust_type_mut());
        Some(buf.into())
    }
}

/// Constructs payload from an iterator to `z_owned_bytes_t`.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param iterator_body: Iterator body function, providing data items. Returning false is treated as iteration end.
/// @param context: Arbitrary context that will be passed to iterator_body.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_from_iter(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    iterator_body: extern "C" fn(
        data: &mut MaybeUninit<z_owned_bytes_t>,
        context: *mut c_void,
    ) -> bool,
    context: *mut c_void,
) -> z_result_t {
    let it = ZBytesInIterator {
        body: iterator_body,
        context,
    };

    let b = ZBytes::from_iter(it);
    this.as_rust_type_mut_uninit().write(b);
    Z_OK
}

pub use crate::z_bytes_iterator_t;
decl_c_type!(loaned(z_bytes_iterator_t, ZBytesIterator<'static, ZBytes>));

/// Returns an iterator for multi-element serialized data.
///
/// The `data` should outlive the iterator.
#[no_mangle]
pub extern "C" fn z_bytes_get_iterator(data: &'static z_loaned_bytes_t) -> z_bytes_iterator_t {
    *data
        .as_rust_type_ref()
        .iter::<ZBytes>()
        .as_loaned_c_type_ref()
}

/// Constructs `z_owned_bytes_t` object corresponding to the next element of serialized data.
///
/// Will construct null-state `z_owned_bytes_t` when iterator reaches the end.
/// @return ``false`` when iterator reaches the end,  ``true`` otherwise
#[no_mangle]
pub extern "C" fn z_bytes_iterator_next(
    iter: &mut z_bytes_iterator_t,
    out: &mut MaybeUninit<z_owned_bytes_t>,
) -> bool {
    match iter.as_rust_type_mut().next() {
        Some(buf) => {
            // this is safe because anything is convertible to ZBytes
            out.as_rust_type_mut_uninit()
                .write(unsafe { buf.unwrap_unchecked() });
            true
        }
        None => {
            out.as_rust_type_mut_uninit().write(ZBytes::default());
            false
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Serializes from an immutable SHM buffer consuming it.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_shm(
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
/// @brief Serializes from a mutable SHM buffer consuming it.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_shm_mut(
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

/// Reads data into specified destination.
///
/// @param this_: Data reader to read from.
/// @param dst: An uninitialized memory location where a new piece of data will be read. Note that it does not involve a copy, but only increases reference count.
/// @return ​0​ upon success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_read_bounded(
    this: &mut z_bytes_reader_t,
    dst: &mut MaybeUninit<z_owned_bytes_t>,
) -> z_result_t {
    let reader = this.as_rust_type_mut();
    match reader.deserialize::<ZBytes>() {
        Ok(b) => {
            dst.as_rust_type_mut_uninit().write(b);
            result::Z_OK
        }
        Err(e) => {
            dst.as_rust_type_mut_uninit().write(ZBytes::empty());
            tracing::error!("{}", e);
            result::Z_EPARSE
        }
    }
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

pub use crate::opaque_types::z_bytes_writer_t;

decl_c_type! {loaned(z_bytes_writer_t, ZBytesWriter<'static>)}

/// @brief Gets writer for`this_`.
/// @note Creating another writer while previous one is still in use is undefined behaviour.
#[no_mangle]
extern "C" fn z_bytes_get_writer(this: &'static mut z_loaned_bytes_t) -> z_bytes_writer_t {
    *this.as_rust_type_mut().writer().as_loaned_c_type_ref()
}

/// Writes `len` bytes from `src` into underlying data.
///
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
unsafe extern "C" fn z_bytes_writer_write_all(
    this: &mut z_bytes_writer_t,
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
/// @return 0 in case of success, negative error code otherwise
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
unsafe extern "C" fn z_bytes_writer_append(
    this: &mut z_bytes_writer_t,
    bytes: &mut z_moved_bytes_t,
) -> z_result_t {
    this.as_rust_type_mut().append(bytes.take_rust_type());
    result::Z_OK
}

/// Appends bytes, with boundaries information. It would allow to read the same piece of data using `z_bytes_reader_read_bounded()`.    
///
/// @return 0 in case of success, negative error code otherwise
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
unsafe extern "C" fn z_bytes_writer_append_bounded(
    this: &mut z_bytes_writer_t,
    bytes: &mut z_moved_bytes_t,
) -> z_result_t {
    this.as_rust_type_mut().serialize(bytes.take_rust_type());
    result::Z_OK
}
