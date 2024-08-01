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
    bytes::{Deserialize, Serialize, ZBytes, ZBytesIterator, ZBytesReader, ZBytesWriter, ZSerde},
    internal::buffers::{ZBuf, ZSlice, ZSliceBuffer},
};

pub use crate::opaque_types::{z_loaned_bytes_t, z_owned_bytes_t};
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::result::Z_ENULL;
use crate::{
    result::{self, z_result_t, Z_EINVAL, Z_EIO, Z_EPARSE, Z_OK},
    transmute::{LoanedCTypeRef, OwnedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_loaned_slice_t, z_loaned_string_t, z_moved_bytes_t, z_owned_slice_t, z_owned_string_t,
    z_slice_clone, z_string_clone, CSlice, CSliceOwned, CStringOwned,
};
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::{z_loaned_shm_t, z_moved_shm_mut_t, z_moved_shm_t, z_owned_shm_t};
decl_c_type! {
    owned(z_owned_bytes_t, ZBytes),
    loaned(z_loaned_bytes_t),
    moved(z_moved_bytes_t)
}

/// The gravestone value for `z_owned_bytes_t`.
#[no_mangle]
extern "C" fn z_bytes_null(this: &mut MaybeUninit<z_owned_bytes_t>) {
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
#[allow(unused_variables)]
extern "C" fn z_bytes_drop(this: z_moved_bytes_t) {}

/// Returns ``true`` if `this_` is in a valid state, ``false`` if it is in a gravestone state.
#[no_mangle]
extern "C" fn z_bytes_check(this: &z_owned_bytes_t) -> bool {
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
/// Deserializes data into an owned SHM buffer by copying it's shared reference
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
/// Deserializes data into a loaned SHM buffer
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
/// Deserializes data into a mutably loaned SHM buffer
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
pub extern "C" fn z_bytes_serialize_from_uint8(this: &mut MaybeUninit<z_owned_bytes_t>, val: u8) {
    z_bytes_serialize_from_arithmetic::<u8>(this, val);
}

/// Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_uint16(this: &mut MaybeUninit<z_owned_bytes_t>, val: u16) {
    z_bytes_serialize_from_arithmetic::<u16>(this, val);
}

/// Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_uint32(this: &mut MaybeUninit<z_owned_bytes_t>, val: u32) {
    z_bytes_serialize_from_arithmetic::<u32>(this, val);
}

/// Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_uint64(this: &mut MaybeUninit<z_owned_bytes_t>, val: u64) {
    z_bytes_serialize_from_arithmetic::<u64>(this, val);
}

/// Serializes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_int8(this: &mut MaybeUninit<z_owned_bytes_t>, val: i8) {
    z_bytes_serialize_from_arithmetic::<i8>(this, val);
}

/// Serializes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_int16(this: &mut MaybeUninit<z_owned_bytes_t>, val: i16) {
    z_bytes_serialize_from_arithmetic::<i16>(this, val);
}

/// Serializes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_int32(this: &mut MaybeUninit<z_owned_bytes_t>, val: i32) {
    z_bytes_serialize_from_arithmetic::<i32>(this, val);
}

/// Serializes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_int64(this: &mut MaybeUninit<z_owned_bytes_t>, val: i64) {
    z_bytes_serialize_from_arithmetic::<i64>(this, val);
}

/// Serializes a float.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_float(this: &mut MaybeUninit<z_owned_bytes_t>, val: f32) {
    z_bytes_serialize_from_arithmetic::<f32>(this, val);
}

/// Serializes a double.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_double(this: &mut MaybeUninit<z_owned_bytes_t>, val: f64) {
    z_bytes_serialize_from_arithmetic::<f64>(this, val);
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

fn _z_bytes_serialize_from_cslice(this: &mut MaybeUninit<z_owned_bytes_t>, s: CSlice) {
    let payload = ZBytes::from(ZSlice::from(s));
    this.as_rust_type_mut_uninit().write(payload);
}

/// Serializes a slice.
/// The slice is consumed upon function return.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_slice(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    slice: &mut z_owned_slice_t,
) {
    _z_bytes_serialize_from_cslice(this, std::mem::take(slice.as_rust_type_mut()))
}

/// Serializes a slice by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_slice(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    slice: &z_loaned_slice_t,
) {
    let mut s = MaybeUninit::<z_owned_slice_t>::uninit();
    z_slice_clone(&mut s, slice);
    let mut s_clone = s.assume_init();
    z_bytes_from_slice(this, &mut s_clone)
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
    if let Ok(mut s) = CSliceOwned::wrap(data, len, deleter, context) {
        z_bytes_from_slice(this, s.as_owned_c_type_mut());
        Z_OK
    } else {
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
    if let Ok(mut s) = CSliceOwned::wrap(data as _, len, None, null_mut()) {
        z_bytes_from_slice(this, s.as_owned_c_type_mut());
        Z_OK
    } else {
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
    if let Ok(mut s) = CSliceOwned::new(data, len) {
        z_bytes_from_slice(this, s.as_owned_c_type_mut());
        Z_OK
    } else {
        Z_EINVAL
    }
}

/// Serializes a string.
/// The string is consumed upon function return.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_string(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    s: &mut z_owned_string_t,
) {
    // TODO: verify that string is a valid utf-8 string ?
    let cs = std::mem::take(&mut s.as_rust_type_mut().0 .0);
    _z_bytes_serialize_from_cslice(this, cs)
}

/// Serializes a string by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_string(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    str: &z_loaned_string_t,
) {
    let mut s = MaybeUninit::<z_owned_string_t>::uninit();
    z_string_clone(&mut s, str);
    let mut s_clone = s.assume_init();
    z_bytes_from_string(this, &mut s_clone)
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
    if let Ok(mut s) = CStringOwned::wrap(str, libc::strlen(str), deleter, context) {
        z_bytes_from_string(this, s.as_owned_c_type_mut());
        Z_OK
    } else {
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
    if let Ok(mut s) = CStringOwned::wrap(str as _, libc::strlen(str), None, null_mut()) {
        z_bytes_from_string(this, s.as_owned_c_type_mut());
        Z_OK
    } else {
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
    if let Ok(mut s) = CStringOwned::new(str, libc::strlen(str)) {
        z_bytes_from_string(this, s.as_owned_c_type_mut());
        Z_OK
    } else {
        Z_EINVAL
    }
}

/// Serializes a pair of `z_owned_bytes_t` objects which are consumed in the process.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_from_pair(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    first: z_moved_bytes_t,
    second: z_moved_bytes_t,
) -> z_result_t {
    let Some(first) = first.into_rust_type() else {
        return result::Z_EINVAL;
    };
    let Some(second) = second.into_rust_type() else {
        return result::Z_EINVAL;
    };
    let b = ZBytes::serialize((first, second));
    this.as_rust_type_mut_uninit().write(b);
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
            tracing::error!("Failed to deserialize the payload: {:?}", e);
            Z_EPARSE
        }
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
/// Serializes from an immutable SHM buffer consuming it
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_shm(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    shm: z_moved_shm_t,
) -> z_result_t {
    let Some(shm) = shm.into_rust_type() else {
        return Z_ENULL;
    };
    this.as_rust_type_mut_uninit().write(shm.into());
    Z_OK
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// Serializes from a mutable SHM buffer consuming it
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_shm_mut(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    shm: z_moved_shm_mut_t,
) -> z_result_t {
    let Some(shm) = shm.into_rust_type() else {
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
/// Return ​0​ upon success, negative error code otherwise.
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
pub unsafe extern "C" fn z_bytes_reader_tell(this: &mut z_bytes_reader_t) -> i64 {
    let reader = this.as_rust_type_mut();
    reader.stream_position().map(|p| p as i64).unwrap_or(-1)
}

pub use crate::opaque_types::{
    z_loaned_bytes_writer_t, z_moved_bytes_writer_t, z_owned_bytes_writer_t,
};

decl_c_type! {
    owned(z_owned_bytes_writer_t,
        Option<ZBytesWriter<'static>>),
    loaned(z_loaned_bytes_writer_t, ZBytesWriter<'static>),
moved(z_moved_bytes_writer_t)
}

/// The gravestone value for `z_owned_bytes_reader_t`.
#[no_mangle]
extern "C" fn z_bytes_writer_null(this: &mut MaybeUninit<z_owned_bytes_writer_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Drops `this_`, resetting it to gravestone value.
#[no_mangle]
#[allow(unused_variables)]
extern "C" fn z_bytes_writer_drop(this: z_moved_bytes_writer_t) {}

/// Returns ``true`` if `this_` is in a valid state, ``false`` if it is in a gravestone state.
#[no_mangle]
extern "C" fn z_bytes_writer_check(this: &z_owned_bytes_writer_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Borrows writer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
unsafe extern "C" fn z_bytes_writer_loan(
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
unsafe extern "C" fn z_bytes_writer_loan_mut(
    this: &mut z_owned_bytes_writer_t,
) -> &mut z_loaned_bytes_writer_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// Gets writer for `this_`.
#[no_mangle]
extern "C" fn z_bytes_get_writer(
    this: &'static mut z_loaned_bytes_t,
    out: &mut MaybeUninit<z_owned_bytes_writer_t>,
) {
    out.as_rust_type_mut_uninit()
        .write(Some(this.as_rust_type_mut().writer()));
}

/// Writes `len` bytes from `src` into underlying data
///
/// @return 0 in case of success, negative error code otherwise
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
unsafe extern "C" fn z_bytes_writer_write(
    this: &mut z_loaned_bytes_writer_t,
    src: *const u8,
    len: usize,
) -> z_result_t {
    match this.as_rust_type_mut().write(from_raw_parts(src, len)) {
        Ok(_) => Z_OK,
        Err(_) => Z_EIO,
    }
}
