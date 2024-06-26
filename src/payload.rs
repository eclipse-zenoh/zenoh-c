use crate::errors::{self, z_error_t, Z_EIO, Z_EPARSE, Z_OK};
use crate::transmute::{
    unwrap_ref_unchecked, unwrap_ref_unchecked_mut, Inplace, TransmuteFromHandle,
    TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
};
use crate::transmute2::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit};
use crate::{
    z_loaned_slice_map_t, z_owned_slice_map_t, z_owned_slice_t, z_owned_string_t, CSlice,
    CSliceOwned, CStringOwned, ZHashMap,
};
use core::fmt;
use std::any::Any;
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::MaybeUninit;
use std::os::raw::c_void;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use zenoh::bytes::{
    Deserialize, Serialize, ZBytes, ZBytesIterator, ZBytesReader, ZBytesWriter, ZSerde,
};
use zenoh::internal::buffers::{ZBuf, ZSlice, ZSliceBuffer};

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::errors::Z_ENULL;
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::{z_loaned_shm_t, z_owned_shm_mut_t, z_owned_shm_t};

pub use crate::opaque_types::z_loaned_bytes_t;
pub use crate::opaque_types::z_owned_bytes_t;
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
    this.as_rust_type_ref().as_loaned_ctype_ref()
}

/// Muatably borrows data.
#[no_mangle]
extern "C" fn z_bytes_loan_mut(this: &mut z_owned_bytes_t) -> &mut z_loaned_bytes_t {
    this.as_rust_type_mut().as_loaned_ctype_mut()
}

/// Returns ``true`` if `this_` is empty, ``false`` otherwise.
#[no_mangle]
extern "C" fn z_bytes_is_empty(this: &z_loaned_bytes_t) -> bool {
    this.as_rust_type_ref().is_empty()
}

/// Constructs an owned shallow copy of data in provided uninitialized memory location.
#[no_mangle]
extern "C" fn z_bytes_clone(this: &z_loaned_bytes_t, dst: &mut MaybeUninit<z_owned_bytes_t>) {
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
/// @param dst: An unitialized memory location where to construct a deserialized string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_string(
    this: &z_loaned_bytes_t,
    dst: &mut MaybeUninit<z_owned_string_t>,
) -> z_error_t {
    let payload = this.as_rust_type_ref();
    match payload.deserialize::<String>() {
        Ok(s) => {
            dst.as_rust_type_mut_uninit().write(s.into());
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to deserialize the payload: {}", e);
            dst.as_rust_type_mut_uninit().write(CStringOwned::default());
            errors::Z_EIO
        }
    }
}

/// Deserializes data into an owned bytes map.
///
/// @param this_: Data to deserialize.
/// @param dst: An unitialized memory location where to construct a deserialized map.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_slice_map(
    this: &z_loaned_bytes_t,
    dst: &mut MaybeUninit<z_owned_slice_map_t>,
) -> z_error_t {
    let dst = dst.as_rust_type_mut_uninit();
    let payload = this.as_rust_type_ref();
    let iter = payload.iter::<(Vec<u8>, Vec<u8>)>();
    let mut hm = ZHashMap::new();

    let iter = iter.filter_map(|val| val.ok());
    for (k, v) in iter {
        hm.insert(k.into(), v.into());
    }
    dst.write(Some(hm));
    errors::Z_OK
}

/// Deserializes data into an owned slice.
///
/// @param this_: Data to deserialize.
/// @param dst: An unitialized memory location where to construct a slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_slice(
    this: &z_loaned_bytes_t,
    dst: &mut MaybeUninit<z_owned_slice_t>,
) -> z_error_t {
    let payload = this.as_rust_type_ref();
    match payload.deserialize::<Vec<u8>>() {
        Ok(v) => {
            dst.as_rust_type_mut_uninit().write(v.into());
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to read the payload: {}", e);
            dst.as_rust_type_mut_uninit().write(CSliceOwned::default());
            errors::Z_EIO
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// Deserializes data into an owned SHM buffer by copying it's shared reference
///
/// @param this_: Data to deserialize.
/// @param dst: An unitialized memory location where to construct a deserialized string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_owned_shm(
    this: &z_loaned_bytes_t,
    dst: *mut MaybeUninit<z_owned_shm_t>,
) -> z_error_t {
    use zenoh::shm::zshm;

    let payload = this.as_rust_type_ref();
    match payload.deserialize::<&zshm>() {
        Ok(s) => {
            Inplace::init(dst.transmute_uninit_ptr(), Some(s.to_owned()));
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to deserialize the payload: {}", e);
            Inplace::empty(dst.transmute_uninit_ptr());
            errors::Z_EIO
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// Deserializes data into a loaned SHM buffer
///
/// @param this_: Data to deserialize.
/// @param dst: An unitialized memory location where to construct a deserialized string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_loaned_shm(
    this: &z_loaned_bytes_t,
    dst: *mut MaybeUninit<&'static z_loaned_shm_t>,
) -> z_error_t {
    use zenoh::shm::zshm;

    let payload = this.as_rust_type_ref();
    match payload.deserialize::<&zshm>() {
        Ok(s) => {
            (*dst).write(s.transmute_handle());
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to deserialize the payload: {}", e);
            errors::Z_EIO
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// Deserializes data into a mutably loaned SHM buffer
///
/// @param this_: Data to deserialize.
/// @param dst: An unitialized memory location where to construct a deserialized string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_deserialize_into_mut_loaned_shm(
    this: &mut z_loaned_bytes_t,
    dst: *mut MaybeUninit<&'static mut z_loaned_shm_t>,
) -> z_error_t {
    use zenoh::shm::zshm;

    let payload = this.as_rust_type_mut();
    match payload.deserialize_mut::<&mut zshm>() {
        Ok(s) => {
            (*dst).write(s.transmute_handle_mut());
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to deserialize the payload: {}", e);
            errors::Z_EIO
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
) -> z_error_t
where
    ZSerde: Deserialize<'a, T, Input = &'a ZBytes>,
    <ZSerde as Deserialize<'a, T>>::Error: fmt::Debug,
{
    match this.as_rust_type_ref().deserialize::<T>() {
        Ok(v) => {
            *val = v;
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to deserialize the payload: {}", e);
            errors::Z_EPARSE
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
) -> z_error_t {
    z_bytes_deserialize_into_arithmetic::<u8>(this, dst)
}

/// Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_uint16(
    this: &z_loaned_bytes_t,
    dst: &mut u16,
) -> z_error_t {
    z_bytes_deserialize_into_arithmetic::<u16>(this, dst)
}

/// Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_uint32(
    this: &z_loaned_bytes_t,
    dst: &mut u32,
) -> z_error_t {
    z_bytes_deserialize_into_arithmetic::<u32>(this, dst)
}

/// Deserializes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_uint64(
    this: &z_loaned_bytes_t,
    dst: &mut u64,
) -> z_error_t {
    z_bytes_deserialize_into_arithmetic::<u64>(this, dst)
}

/// Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_int8(
    this: &z_loaned_bytes_t,
    dst: &mut i8,
) -> z_error_t {
    z_bytes_deserialize_into_arithmetic::<i8>(this, dst)
}

/// Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_int16(
    this: &z_loaned_bytes_t,
    dst: &mut i16,
) -> z_error_t {
    z_bytes_deserialize_into_arithmetic::<i16>(this, dst)
}

/// Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_int32(
    this: &z_loaned_bytes_t,
    dst: &mut i32,
) -> z_error_t {
    z_bytes_deserialize_into_arithmetic::<i32>(this, dst)
}

/// Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_int64(
    this: &z_loaned_bytes_t,
    dst: &mut i64,
) -> z_error_t {
    z_bytes_deserialize_into_arithmetic::<i64>(this, dst)
}

/// Deserializes into a float.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_float(
    this: &z_loaned_bytes_t,
    dst: &mut f32,
) -> z_error_t {
    z_bytes_deserialize_into_arithmetic::<f32>(this, dst)
}

/// Deserializes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_deserialize_into_double(
    this: &z_loaned_bytes_t,
    dst: &mut f64,
) -> z_error_t {
    z_bytes_deserialize_into_arithmetic::<f64>(this, dst)
}

/// Serializes a slice by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_slice(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    data: *const u8,
    len: usize,
) {
    let s = CSlice::new_borrowed_unchecked(data, len);
    let payload = ZBytes::from(ZSlice::from(s));
    this.as_rust_type_mut_uninit().write(payload);
}

/// Serializes a slice by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_slice_copy(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    data: *const u8,
    len: usize,
) {
    let s = CSlice::new_owned_unchecked(data, len);
    let payload = ZBytes::from(ZSlice::from(s));
    this.as_rust_type_mut_uninit().write(payload);
}

/// Serializes slice map by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_slice_map(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    bytes_map: &z_loaned_slice_map_t,
) {
    let hm = bytes_map.as_rust_type_ref();
    let payload = ZBytes::from_iter(hm.iter().map(|(k, v)| {
        (
            CSlice::new_borrowed_unchecked(k.data(), k.len()),
            CSlice::new_borrowed_unchecked(v.data(), v.len()),
        )
    }));
    this.as_rust_type_mut_uninit().write(payload);
}

/// Serializes slice map by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_slice_map_copy(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    bytes_map: &z_loaned_slice_map_t,
) {
    let hm = bytes_map.as_rust_type_ref();
    let payload = ZBytes::from_iter(hm.iter().map(|(k, v)| {
        (
            CSlice::new_owned_unchecked(k.data(), k.len()),
            CSlice::new_owned_unchecked(v.data(), v.len()),
        )
    }));
    this.as_rust_type_mut_uninit().write(payload);
}

/// Serializes a null-terminated string by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_string(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    s: *const libc::c_char,
) {
    z_bytes_serialize_from_slice(this, s as *const u8, libc::strlen(s));
}

/// Serializes a null-terminated string by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_string_copy(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    s: *const libc::c_char,
) {
    z_bytes_serialize_from_slice_copy(this, s as *const u8, libc::strlen(s));
}

/// Serializes a pair of `z_owned_bytes_t` objects which are consumed in the process.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_pair(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    first: &mut z_owned_bytes_t,
    second: &mut z_owned_bytes_t,
) -> z_error_t {
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
) -> z_error_t {
    match this.as_rust_type_ref().deserialize::<(ZBytes, ZBytes)>() {
        Ok((a, b)) => {
            first.as_rust_type_mut_uninit().write(a);
            second.as_rust_type_mut_uninit().write(b);
            Z_OK
        }
        Err(e) => {
            log::error!("Failed to deserialize the payload: {}", e);
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
/// @param this_: An uninitialized location in memery for `z_owned_bytes_t` will be constructed.
/// @param iterator_body: Iterator body function, providing data items. Returning false is treated as iteration end.
/// @param context: Arbitrary context that will be passed to iterator_body.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_serialize_from_iter(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    iterator_body: extern "C" fn(
        data: &mut MaybeUninit<z_owned_bytes_t>,
        context: *mut c_void,
    ) -> bool,
    context: *mut c_void,
) -> z_error_t {
    let it = ZBytesInIterator {
        body: iterator_body,
        context,
    };

    let b = ZBytes::from_iter(it);
    this.as_rust_type_mut_uninit().write(b);
    Z_OK
}

pub use crate::z_bytes_iterator_t;
decl_transmute_handle!(ZBytesIterator<'static, ZBytes>, z_bytes_iterator_t);
/// Returns an iterator for multi-element serialized data.
///
/// The `data` should outlive the iterator.
#[no_mangle]
pub extern "C" fn z_bytes_get_iterator(data: &'static z_loaned_bytes_t) -> z_bytes_iterator_t {
    *data.as_rust_type_ref().iter::<ZBytes>().transmute_handle()
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
    match iter.transmute_mut().next() {
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

/// Returns an iterator for multi-element serialized data.
/// @param this_: Data to deserialize.
#[no_mangle]
pub extern "C" fn z_bytes_iter(
    this: &z_loaned_bytes_t,
    iterator_body: extern "C" fn(data: &z_loaned_bytes_t, context: *mut c_void) -> z_error_t,
    context: *mut c_void,
) -> z_error_t {
    let mut res = Z_OK;
    for zb in this.as_rust_type_ref().iter::<ZBytes>() {
        // this is safe because literally any payload is convertable into ZBuf
        let b = unsafe { zb.unwrap_unchecked() };
        res = iterator_body(b.as_loaned_ctype_ref(), context);
        if res != Z_OK {
            break;
        }
    }

    res
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// Serializes from an immutable SHM buffer consuming it
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_shm(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    shm: &mut z_owned_shm_t,
) -> z_error_t {
    match shm.transmute_mut().take() {
        Some(shm) => {
            this.as_rust_type_mut_uninit().write(shm.into());
            Z_OK
        }
        None => Z_ENULL,
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// Serializes from an immutable SHM buffer copying it
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_shm_copy(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    shm: &z_loaned_shm_t,
) {
    this.as_rust_type_mut_uninit()
        .write(shm.transmute_ref().to_owned().into());
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// Serializes from a mutable SHM buffer consuming it
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_serialize_from_shm_mut(
    this: &mut MaybeUninit<z_owned_bytes_t>,
    shm: &mut z_owned_shm_mut_t,
) -> z_error_t {
    match shm.transmute_mut().take() {
        Some(shm) => {
            this.as_rust_type_mut_uninit().write(shm.into());
            Z_OK
        }
        None => Z_ENULL,
    }
}

pub use crate::z_bytes_reader_t;
decl_transmute_handle!(ZBytesReader<'static>, z_bytes_reader_t);
/// Returns a reader for the data.
///
/// The `data` should outlive the reader.
#[no_mangle]
pub extern "C" fn z_bytes_get_reader(data: &'static z_loaned_bytes_t) -> z_bytes_reader_t {
    *data.as_rust_type_ref().reader().transmute_handle()
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
    let reader = this.transmute_mut();
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
) -> z_error_t {
    let reader = this.transmute_mut();
    let pos = match origin {
        libc::SEEK_SET => match offset.try_into() {
            Ok(o) => SeekFrom::Start(o),
            Err(_) => {
                return errors::Z_EINVAL;
            }
        },
        libc::SEEK_CUR => SeekFrom::Current(offset),
        libc::SEEK_END => SeekFrom::End(offset),
        _ => {
            return errors::Z_EINVAL;
        }
    };

    match reader.seek(pos) {
        Ok(_) => errors::Z_OK,
        Err(_) => errors::Z_EINVAL,
    }
}

/// Gets the read position indicator.
/// @return read position indicator on success or -1L if failure occurs.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_tell(this: &mut z_bytes_reader_t) -> i64 {
    let reader = this.transmute_mut();
    reader.stream_position().map(|p| p as i64).unwrap_or(-1)
}

pub use crate::opaque_types::z_loaned_bytes_writer_t;
pub use crate::opaque_types::z_owned_bytes_writer_t;

decl_transmute_owned!(Option<ZBytesWriter<'static>>, z_owned_bytes_writer_t);
decl_transmute_handle!(ZBytesWriter<'static>, z_loaned_bytes_writer_t);
validate_equivalence!(z_loaned_bytes_writer_t, z_owned_bytes_writer_t);

/// The gravestone value for `z_owned_bytes_reader_t`.
#[no_mangle]
extern "C" fn z_bytes_writer_null(this: *mut MaybeUninit<z_owned_bytes_writer_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Drops `this_`, resetting it to gravestone value.
#[no_mangle]
extern "C" fn z_bytes_writer_drop(this: &mut z_owned_bytes_writer_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

/// Returns ``true`` if `this_` is in a valid state, ``false`` if it is in a gravestone state.
#[no_mangle]
extern "C" fn z_bytes_writer_check(this: &z_owned_bytes_writer_t) -> bool {
    this.transmute_ref().is_some()
}

/// Borrows writer.
#[no_mangle]
extern "C" fn z_bytes_writer_loan(this: &z_owned_bytes_writer_t) -> &z_loaned_bytes_writer_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

/// Muatably borrows writer.
#[no_mangle]
extern "C" fn z_bytes_writer_loan_mut(
    this: &mut z_owned_bytes_writer_t,
) -> &mut z_loaned_bytes_writer_t {
    let this = this.transmute_mut();
    let this = unwrap_ref_unchecked_mut(this);
    this.transmute_handle_mut()
}

/// Gets writer for `this_`.
#[no_mangle]
extern "C" fn z_bytes_get_writer(
    this: &'static mut z_loaned_bytes_t,
    out: *mut MaybeUninit<z_owned_bytes_writer_t>,
) {
    let out = out.transmute_uninit_ptr();
    Inplace::init(out, Some(this.as_rust_type_mut().writer()));
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
) -> z_error_t {
    match this.transmute_mut().write(from_raw_parts(src, len)) {
        Ok(_) => Z_OK,
        Err(_) => Z_EIO,
    }
}
