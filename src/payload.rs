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
    bytes::{ZBytes, ZBytesIterator, ZBytesReader, ZBytesWriter},
    internal::buffers::{ZBuf, ZSlice, ZSliceBuffer},
};

pub use crate::opaque_types::{z_loaned_bytes_t, z_owned_bytes_t};
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::result::Z_ENULL;
use crate::{
    result::{self, z_result_t, Z_EINVAL, Z_EIO, Z_EPARSE, Z_OK},
    transmute::{LoanedCTypeRef, OwnedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_loaned_slice_t, z_loaned_string_t, z_owned_slice_t, z_owned_string_t, z_slice_clone,
    z_string_clone, CSlice, CSliceOwned, CStringOwned,
};
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::{z_loaned_shm_t, z_owned_shm_mut_t, z_owned_shm_t};
decl_c_type! {
    owned(z_owned_bytes_t, ZBytes),
    loaned(z_loaned_bytes_t, ZBytes),
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
extern "C" fn z_bytes_drop(this: &mut z_owned_bytes_t) {
    *this.as_rust_type_mut() = ZBytes::default();
}

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

macro_rules! impl_serialize_and_deserialize_for_num {
    ($rust_type:ty, $c_type:ty, $desc:tt) => {
        paste::paste! {
            #[doc = "Serialization from " $desc "."]
            #[no_mangle]
            pub extern "C" fn [<z_bytes_serialize_from_ $c_type>](this: &mut MaybeUninit<z_owned_bytes_t>, val: $rust_type) {
                this.as_rust_type_mut_uninit().write(ZBytes::serialize(val));
            }

            #[doc = "Deserialization into " $desc "."]
            #[doc = "\n"]
            #[doc = "@return 0 in case of success, negative error code otherwise."]
            #[no_mangle]
            pub extern "C" fn [< z_bytes_deserialize_into_ $c_type>](
                this: &z_loaned_bytes_t,
                dst: &mut $rust_type,
            ) -> z_result_t {
                match this.as_rust_type_ref().deserialize::<$rust_type>() {
                    Ok(v) => {
                        *dst = v;
                        result::Z_OK
                    }
                    Err(e) => {
                        tracing::error!("Failed to deserialize the payload: {:?}", e);
                        result::Z_EPARSE
                    }
                }
            }
        }
    }
}

impl_serialize_and_deserialize_for_num!(u8, uint8, "unsigned integer");
impl_serialize_and_deserialize_for_num!(u16, uint16, "unsigned integer");
impl_serialize_and_deserialize_for_num!(u32, uint32, "unsigned integer");
impl_serialize_and_deserialize_for_num!(u64, uint64, "unsigned integer");
impl_serialize_and_deserialize_for_num!(i8, int8, "signed integer");
impl_serialize_and_deserialize_for_num!(i16, int16, "signed integer");
impl_serialize_and_deserialize_for_num!(i32, int32, "signed integer");
impl_serialize_and_deserialize_for_num!(i64, int64, "signed integer");
impl_serialize_and_deserialize_for_num!(f32, float, "float");
impl_serialize_and_deserialize_for_num!(f64, double, "double");

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
    first: &mut z_owned_bytes_t,
    second: &mut z_owned_bytes_t,
) -> z_result_t {
    let first = std::mem::take(first.as_rust_type_mut());
    let second = std::mem::take(second.as_rust_type_mut());
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
    shm: &mut z_owned_shm_t,
) -> z_result_t {
    match shm.as_rust_type_mut().take() {
        Some(shm) => {
            this.as_rust_type_mut_uninit().write(shm.into());
            Z_OK
        }
        None => Z_ENULL,
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// Serializes from a mutable SHM buffer consuming it
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_shm_mut(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    shm: &mut z_owned_shm_mut_t,
) -> z_result_t {
    match shm.as_rust_type_mut().take() {
        Some(shm) => {
            this.as_rust_type_mut_uninit().write(shm.into());
            Z_OK
        }
        None => Z_ENULL,
    }
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
pub unsafe extern "C" fn z_bytes_reader_tell(this: &mut z_bytes_reader_t) -> i64 {
    let reader = this.as_rust_type_mut();
    reader.stream_position().map(|p| p as i64).unwrap_or(-1)
}

pub use crate::opaque_types::z_bytes_writer_t;

decl_c_type! {loaned(z_bytes_writer_t, ZBytesWriter<'static>)}

/// Gets writer for `this_`.
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
    bytes: &mut z_owned_bytes_t,
) -> z_result_t {
    this.as_rust_type_mut()
        .append(std::mem::take(bytes.as_rust_type_mut()));
    result::Z_OK
}

/// Appends bytes, with boundaries information. It would allow to read the same piece of data using `z_bytes_reader_read_bounded()`.    
///
/// @return 0 in case of success, negative error code otherwise
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
unsafe extern "C" fn z_bytes_writer_append_bounded(
    this: &mut z_bytes_writer_t,
    bytes: &mut z_owned_bytes_t,
) -> z_result_t {
    this.as_rust_type_mut()
        .serialize(std::mem::take(bytes.as_rust_type_mut()));
    result::Z_OK
}
