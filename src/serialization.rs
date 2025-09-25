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

use core::str;
use std::{mem::MaybeUninit, slice::from_raw_parts};

use zenoh::bytes::ZBytes;
use zenoh_ext::{
    z_deserialize, z_serialize, Deserialize, Serialize, VarInt, ZDeserializer, ZSerializer,
};

pub use crate::opaque_types::{
    ze_deserializer_t, ze_loaned_serializer_t, ze_moved_serializer_t, ze_owned_serializer_t,
};
use crate::{
    result::{self, z_result_t},
    strlen_or_zero,
    transmute::{Gravestone, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_loaned_bytes_t, z_loaned_slice_t, z_loaned_string_t, z_owned_bytes_t, z_owned_slice_t,
    z_owned_string_t, CSliceOwned, CStringOwned,
};

decl_c_type! {
    owned(ze_owned_serializer_t, option ZSerializer),
    loaned(ze_loaned_serializer_t),
}

/// @brief Constructs a serializer with empty payload.
/// @param this_: An uninitialized memory location where serializer is to be constructed.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
extern "C" fn ze_serializer_empty(this: &mut MaybeUninit<ze_owned_serializer_t>) -> z_result_t {
    this.as_rust_type_mut_uninit()
        .write(Some(ZSerializer::new()));
    result::Z_OK
}

/// @brief Drops `this_`, resetting it to gravestone value.
#[no_mangle]
extern "C" fn ze_serializer_drop(this_: &mut ze_moved_serializer_t) {
    let _ = this_.take_rust_type();
}

/// @brief Returns ``true`` if `this_` is in a valid state, ``false`` if it is in a gravestone state.
#[no_mangle]
extern "C" fn ze_internal_serializer_check(this: &ze_owned_serializer_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// @brief Borrows serializer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_serializer_loan(
    this: &ze_owned_serializer_t,
) -> &ze_loaned_serializer_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @brief Muatably borrows serializer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_serializer_loan_mut(
    this: &mut ze_owned_serializer_t,
) -> &mut ze_loaned_serializer_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// @brief Constructs a serializer in a gravestone state.
#[no_mangle]
pub extern "C" fn ze_internal_serializer_null(this_: &mut MaybeUninit<ze_owned_serializer_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @brief Drop serializer and extract underlying `bytes` object it was writing to.
/// @param this_: A serializer instance.
/// @param bytes: An uninitialized memory location where `bytes` object` will be written to.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn ze_serializer_finish(
    this: &mut ze_moved_serializer_t,
    bytes: &mut MaybeUninit<z_owned_bytes_t>,
) {
    bytes
        .as_rust_type_mut_uninit()
        .write(this.take_rust_type().unwrap_unchecked().finish());
}

decl_c_type! {loaned(ze_deserializer_t, ZDeserializer<'static>)}

fn ze_serialize_arithmetic<T>(this: &mut MaybeUninit<z_owned_bytes_t>, val: &T)
where
    T: Serialize,
{
    this.as_rust_type_mut_uninit().write(z_serialize(val));
}

fn ze_deserialize_arithmetic<'a, T>(this: &'a z_loaned_bytes_t, val: &'a mut T) -> z_result_t
where
    T: Deserialize,
{
    match z_deserialize(this.as_rust_type_ref()) {
        Ok(v) => {
            *val = v;
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to deserialize the payload: {:?}", e);
            result::Z_EPARSE
        }
    }
}

/// @brief Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn ze_serialize_uint8(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: u8,
) -> z_result_t {
    ze_serialize_arithmetic::<u8>(this_, &val);
    result::Z_OK
}

/// @brief Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn ze_serialize_uint16(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: u16,
) -> z_result_t {
    ze_serialize_arithmetic::<u16>(this_, &val);
    result::Z_OK
}

/// @brief Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn ze_serialize_uint32(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: u32,
) -> z_result_t {
    ze_serialize_arithmetic::<u32>(this_, &val);
    result::Z_OK
}

/// @brief Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn ze_serialize_uint64(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: u64,
) -> z_result_t {
    ze_serialize_arithmetic::<u64>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a signed integer.
#[no_mangle]
pub extern "C" fn ze_serialize_int8(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: i8,
) -> z_result_t {
    ze_serialize_arithmetic::<i8>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a signed integer.
#[no_mangle]
pub extern "C" fn ze_serialize_int16(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: i16,
) -> z_result_t {
    ze_serialize_arithmetic::<i16>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a signed integer.
#[no_mangle]
pub extern "C" fn ze_serialize_int32(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: i32,
) -> z_result_t {
    ze_serialize_arithmetic::<i32>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a signed integer.
#[no_mangle]
pub extern "C" fn ze_serialize_int64(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: i64,
) -> z_result_t {
    ze_serialize_arithmetic::<i64>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a float.
#[no_mangle]
pub extern "C" fn ze_serialize_float(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: f32,
) -> z_result_t {
    ze_serialize_arithmetic::<f32>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a double.
#[no_mangle]
pub extern "C" fn ze_serialize_double(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: f64,
) -> z_result_t {
    ze_serialize_arithmetic::<f64>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a bool.
#[no_mangle]
pub extern "C" fn ze_serialize_bool(
    this_: &mut MaybeUninit<z_owned_bytes_t>,
    val: bool,
) -> z_result_t {
    ze_serialize_arithmetic::<bool>(this_, &val);
    result::Z_OK
}

/// @brief Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_uint8(this: &z_loaned_bytes_t, dst: &mut u8) -> z_result_t {
    ze_deserialize_arithmetic::<u8>(this, dst)
}

/// @brief Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_uint16(this: &z_loaned_bytes_t, dst: &mut u16) -> z_result_t {
    ze_deserialize_arithmetic::<u16>(this, dst)
}

/// @brief Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_uint32(this: &z_loaned_bytes_t, dst: &mut u32) -> z_result_t {
    ze_deserialize_arithmetic::<u32>(this, dst)
}

/// @brief Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_uint64(this: &z_loaned_bytes_t, dst: &mut u64) -> z_result_t {
    ze_deserialize_arithmetic::<u64>(this, dst)
}

/// @brief Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_int8(this: &z_loaned_bytes_t, dst: &mut i8) -> z_result_t {
    ze_deserialize_arithmetic::<i8>(this, dst)
}

/// @brief Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_int16(this: &z_loaned_bytes_t, dst: &mut i16) -> z_result_t {
    ze_deserialize_arithmetic::<i16>(this, dst)
}

/// @brief Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_int32(this: &z_loaned_bytes_t, dst: &mut i32) -> z_result_t {
    ze_deserialize_arithmetic::<i32>(this, dst)
}

/// @brief Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_int64(this: &z_loaned_bytes_t, dst: &mut i64) -> z_result_t {
    ze_deserialize_arithmetic::<i64>(this, dst)
}

/// @brief Deserializes into a float.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_float(this: &z_loaned_bytes_t, dst: &mut f32) -> z_result_t {
    ze_deserialize_arithmetic::<f32>(this, dst)
}

/// @brief Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_double(this: &z_loaned_bytes_t, dst: &mut f64) -> z_result_t {
    ze_deserialize_arithmetic::<f64>(this, dst)
}

/// @brief Deserializes into a bool.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserialize_bool(this: &z_loaned_bytes_t, dst: &mut bool) -> z_result_t {
    ze_deserialize_arithmetic::<bool>(this, dst)
}

/// @brief Serializes a slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_serialize_slice(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    slice: &z_loaned_slice_t,
) -> z_result_t {
    let cslice = slice.as_rust_type_ref().slice();
    this.as_rust_type_mut_uninit().write(z_serialize(cslice));
    result::Z_OK
}

/// @brief Serializes a data from buffer by.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param data: A pointer to the buffer containing data.
/// @param len: Length of the buffer.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_serialize_buf(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    data: *const u8,
    len: usize,
) -> z_result_t {
    let slice = unsafe { from_raw_parts(data, len) };
    this.as_rust_type_mut_uninit().write(z_serialize(slice));
    result::Z_OK
}

/// @brief Deserializes into a slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_deserialize_slice(
    this: &z_loaned_bytes_t,
    slice: &mut MaybeUninit<z_owned_slice_t>,
) -> z_result_t {
    let payload = this.as_rust_type_ref();
    match z_deserialize::<Vec<u8>>(payload) {
        Ok(s) => {
            slice.as_rust_type_mut_uninit().write(s.into());
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to deserialize the payload: {}", e);
            slice
                .as_rust_type_mut_uninit()
                .write(CSliceOwned::gravestone());
            result::Z_EDESERIALIZE
        }
    }
}

/// @brief Serializes a string.
/// The string should be a valid UTF-8.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param str: a string to serialize.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_serialize_string(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    str: &z_loaned_string_t,
) -> z_result_t {
    match str::from_utf8(str.as_rust_type_ref().slice()) {
        Ok(s) => {
            this.as_rust_type_mut_uninit().write(z_serialize(s));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            this.as_rust_type_mut_uninit().write(ZBytes::new());
            result::Z_EUTF8
        }
    }
}

/// @brief Serializes a substring.
/// The substring should be a valid UTF-8.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param start: a pointer to the the start of the substring.
/// @param len: the length of the substring.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_serialize_substr(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    start: *const libc::c_char,
    len: usize,
) -> z_result_t {
    let slice = unsafe { from_raw_parts(start as *const u8, len) };
    match str::from_utf8(slice) {
        Ok(s) => {
            this.as_rust_type_mut_uninit().write(z_serialize(s));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            this.as_rust_type_mut_uninit().write(ZBytes::new());
            result::Z_EUTF8
        }
    }
}

/// @brief Serializes a null-terminated string.
/// The string should be a valid UTF-8.
/// @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
/// @param str: a pointer to the null-terminated string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_serialize_str(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    str: *const libc::c_char,
) -> z_result_t {
    unsafe { ze_serialize_substr(this, str, strlen_or_zero(str)) }
}

/// @brief Deserializes into a UTF-8 string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_deserialize_string(
    this: &z_loaned_bytes_t,
    str: &mut MaybeUninit<z_owned_string_t>,
) -> z_result_t {
    let payload = this.as_rust_type_ref();
    match z_deserialize::<String>(payload) {
        Ok(s) => {
            str.as_rust_type_mut_uninit().write(s.into());
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to deserialize the payload: {}", e);
            str.as_rust_type_mut_uninit()
                .write(CStringOwned::gravestone());
            result::Z_EDESERIALIZE
        }
    }
}

/// @brief Gets deserializer for`this_`.
#[no_mangle]
extern "C" fn ze_deserializer_from_bytes(this: &'static z_loaned_bytes_t) -> ze_deserializer_t {
    *ZDeserializer::new(this.as_rust_type_ref()).as_loaned_c_type_ref()
}

/// @brief Checks if deserializer parsed all of its data.
/// @return `true` if there is no more data to parse, `false` otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ze_deserializer_is_done(this_: &ze_deserializer_t) -> bool {
    let deserializer = this_.as_rust_type_ref();
    deserializer.done()
}

fn ze_serializer_serialize_arithmetic<T>(this: &mut ze_loaned_serializer_t, val: &T)
where
    T: Serialize,
{
    this.as_rust_type_mut().serialize(val);
}

fn ze_deserializer_deserialize_arithmetic<'a, T>(
    this: &'a mut ze_deserializer_t,
    val: &'a mut T,
) -> z_result_t
where
    T: Deserialize,
{
    match this.as_rust_type_mut().deserialize::<T>() {
        Ok(v) => {
            *val = v;
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to deserialize the payload: {:?}", e);
            result::Z_EDESERIALIZE
        }
    }
}

/// @brief Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_uint8(
    this_: &mut ze_loaned_serializer_t,
    val: u8,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<u8>(this_, &val);
    result::Z_OK
}

/// @brief Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_uint16(
    this_: &mut ze_loaned_serializer_t,
    val: u16,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<u16>(this_, &val);
    result::Z_OK
}

/// @brief Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_uint32(
    this_: &mut ze_loaned_serializer_t,
    val: u32,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<u32>(this_, &val);
    result::Z_OK
}

/// @brief Serializes an unsigned integer.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_uint64(
    this_: &mut ze_loaned_serializer_t,
    val: u64,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<u64>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a signed integer.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_int8(
    this_: &mut ze_loaned_serializer_t,
    val: i8,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<i8>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a signed integer.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_int16(
    this_: &mut ze_loaned_serializer_t,
    val: i16,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<i16>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a signed integer.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_int32(
    this_: &mut ze_loaned_serializer_t,
    val: i32,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<i32>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a signed integer.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_int64(
    this_: &mut ze_loaned_serializer_t,
    val: i64,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<i64>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a float.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_float(
    this_: &mut ze_loaned_serializer_t,
    val: f32,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<f32>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a double.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_double(
    this_: &mut ze_loaned_serializer_t,
    val: f64,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<f64>(this_, &val);
    result::Z_OK
}

/// @brief Serializes a bool.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_bool(
    this_: &mut ze_loaned_serializer_t,
    val: bool,
) -> z_result_t {
    ze_serializer_serialize_arithmetic::<bool>(this_, &val);
    result::Z_OK
}

/// @brief Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_uint8(
    this: &mut ze_deserializer_t,
    dst: &mut u8,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<u8>(this, dst)
}

/// @brief Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_uint16(
    this: &mut ze_deserializer_t,
    dst: &mut u16,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<u16>(this, dst)
}

/// @brief Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_uint32(
    this: &mut ze_deserializer_t,
    dst: &mut u32,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<u32>(this, dst)
}

/// @brief Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_uint64(
    this: &mut ze_deserializer_t,
    dst: &mut u64,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<u64>(this, dst)
}

/// @brief Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_int8(
    this: &mut ze_deserializer_t,
    dst: &mut i8,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<i8>(this, dst)
}

/// @brief Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_int16(
    this: &mut ze_deserializer_t,
    dst: &mut i16,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<i16>(this, dst)
}

/// @brief Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_int32(
    this: &mut ze_deserializer_t,
    dst: &mut i32,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<i32>(this, dst)
}

/// @brief Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_int64(
    this: &mut ze_deserializer_t,
    dst: &mut i64,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<i64>(this, dst)
}

/// @brief Deserializes into a float.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_float(
    this: &mut ze_deserializer_t,
    dst: &mut f32,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<f32>(this, dst)
}

/// @brief Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_double(
    this: &mut ze_deserializer_t,
    dst: &mut f64,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<f64>(this, dst)
}

/// @brief Deserializes into a bool.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_bool(
    this: &mut ze_deserializer_t,
    dst: &mut bool,
) -> z_result_t {
    ze_deserializer_deserialize_arithmetic::<bool>(this, dst)
}

/// @brief Serializes a slice.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_slice(
    this: &mut ze_loaned_serializer_t,
    slice: &z_loaned_slice_t,
) -> z_result_t {
    let cslice = slice.as_rust_type_ref().slice();
    this.as_rust_type_mut().serialize(cslice);
    result::Z_OK
}

/// @brief Serializes a data from buffer.
/// @param this_: A serializer instance.
/// @param data: A pointer to the buffer containing data.
/// @param len: Length of the buffer.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_buf(
    this: &mut ze_loaned_serializer_t,
    data: *const u8,
    len: usize,
) -> z_result_t {
    let slice = unsafe { from_raw_parts(data, len) };
    this.as_rust_type_mut().serialize(slice);
    result::Z_OK
}

/// @brief Deserializes into a slice.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_slice(
    this: &mut ze_deserializer_t,
    slice: &mut MaybeUninit<z_owned_slice_t>,
) -> z_result_t {
    match this.as_rust_type_mut().deserialize::<Vec<u8>>() {
        Ok(s) => {
            slice.as_rust_type_mut_uninit().write(s.into());
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to deserialize the payload: {}", e);
            slice
                .as_rust_type_mut_uninit()
                .write(CSliceOwned::gravestone());
            result::Z_EDESERIALIZE
        }
    }
}

/// @brief Serializes a string.
/// The string should be a valid UTF-8.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_string(
    this: &mut ze_loaned_serializer_t,
    str: &z_loaned_string_t,
) -> z_result_t {
    match str::from_utf8(str.as_rust_type_ref().slice()) {
        Ok(s) => {
            this.as_rust_type_mut().serialize(s);
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EUTF8
        }
    }
}

/// @brief Serializes a substring of specified length.
/// The subsstring should be a valid UTF-8.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_substr(
    this: &mut ze_loaned_serializer_t,
    start: *const libc::c_char,
    len: usize,
) -> z_result_t {
    let slice = unsafe { from_raw_parts(start as *const u8, len) };
    match str::from_utf8(slice) {
        Ok(s) => {
            this.as_rust_type_mut().serialize(s);
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EUTF8
        }
    }
}

/// @brief Serializes a null-terminated string.
/// The string should be a valid UTF-8.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_str(
    this: &mut ze_loaned_serializer_t,
    str: *const libc::c_char,
) -> z_result_t {
    ze_serializer_serialize_substr(this, str, unsafe { strlen_or_zero(str) })
}

/// @brief Deserializes into a string.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_string(
    this: &mut ze_deserializer_t,
    str: &mut MaybeUninit<z_owned_string_t>,
) -> z_result_t {
    match this.as_rust_type_mut().deserialize::<String>() {
        Ok(s) => {
            str.as_rust_type_mut_uninit().write(s.into());
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to deserialize the payload: {}", e);
            str.as_rust_type_mut_uninit()
                .write(CStringOwned::gravestone());
            result::Z_EDESERIALIZE
        }
    }
}

/// @brief Initiates serialization of a sequence of multiple elements.
/// @param this_: A serializer instance.
/// @param len: Length of the sequence. Could be read during deserialization using `ze_deserializer_deserialize_sequence_length`.
#[no_mangle]
pub extern "C" fn ze_serializer_serialize_sequence_length(
    this: &mut ze_loaned_serializer_t,
    len: usize,
) -> z_result_t {
    this.as_rust_type_mut().serialize(VarInt::<usize>(len));
    result::Z_OK
}

/// @brief Initiates deserialization of a sequence of multiple elements.
/// @param this_: A serializer instance.
/// @param len:  pointer where the length of the sequence (previously passed via `z_bytes_writer_serialize_sequence_begin`) will be written.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn ze_deserializer_deserialize_sequence_length(
    this: &mut ze_deserializer_t,
    len: &mut usize,
) -> z_result_t {
    match this.as_rust_type_mut().deserialize::<VarInt<usize>>() {
        Ok(l) => {
            *len = l.0;
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to read the sequence length: {}", e);
            *len = 0;
            result::Z_EDESERIALIZE
        }
    }
}
