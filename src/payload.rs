use crate::errors::{self, z_error_t, Z_EINVAL, Z_EIO, Z_EPARSE, Z_OK};
use crate::transmute::{
    unwrap_ref_unchecked, unwrap_ref_unchecked_mut, Inplace, TransmuteFromHandle,
    TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
};
use crate::{
    z_loaned_slice_map_t, z_owned_slice_map_t, z_owned_slice_t, z_owned_str_t, CSlice, ZHashMap,
};
use core::fmt;
use std::any::Any;
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::MaybeUninit;
use std::os::raw::c_void;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use zenoh::buffers::{ZBuf, ZSlice, ZSliceBuffer};
use zenoh::bytes::{
    Deserialize, Serialize, ZBytes, ZBytesIterator, ZBytesReader, ZBytesWriter, ZSerde,
};

pub use crate::opaque_types::z_owned_bytes_t;
decl_transmute_owned!(Option<ZBytes>, z_owned_bytes_t);

/// The gravestone value for `z_owned_bytes_t`.
#[no_mangle]
extern "C" fn z_bytes_null(this: *mut MaybeUninit<z_owned_bytes_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Constructs an empty instance of `z_owned_bytes_t`.
#[no_mangle]
extern "C" fn z_bytes_empty(this: *mut MaybeUninit<z_owned_bytes_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::init(this, Some(ZBytes::empty()));
}

/// Drops `this_`, resetting it to gravestone value. If there are any shallow copies
/// created by `z_bytes_clone()`, they would still stay valid.
#[no_mangle]
extern "C" fn z_bytes_drop(this: &mut z_owned_bytes_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

/// Returns ``true`` if `this_` is in a valid state, ``false`` if it is in a gravestone state.
#[no_mangle]
extern "C" fn z_bytes_check(this: &z_owned_bytes_t) -> bool {
    this.transmute_ref().is_some()
}

/// Borrows data.
#[no_mangle]
extern "C" fn z_bytes_loan(this: &z_owned_bytes_t) -> &z_loaned_bytes_t {
    let payload = this.transmute_ref();
    let payload = unwrap_ref_unchecked(payload);
    payload.transmute_handle()
}

/// Muatably borrows data.
#[no_mangle]
extern "C" fn z_bytes_loan_mut(this: &mut z_owned_bytes_t) -> &mut z_loaned_bytes_t {
    let payload = this.transmute_mut();
    let payload = unwrap_ref_unchecked_mut(payload);
    payload.transmute_handle_mut()
}

pub use crate::opaque_types::z_loaned_bytes_t;
decl_transmute_handle!(ZBytes, z_loaned_bytes_t);

validate_equivalence!(z_owned_bytes_t, z_loaned_bytes_t);

/// Returns ``true`` if `this_` is empty, ``false`` otherwise.
#[no_mangle]
extern "C" fn z_bytes_is_empty(this: &z_loaned_bytes_t) -> bool {
    this.transmute_ref().is_empty()
}

/// Constructs an owned shallow copy of data in provided uninitialized memory location.
#[no_mangle]
extern "C" fn z_bytes_clone(this: &z_loaned_bytes_t, dst: *mut MaybeUninit<z_owned_bytes_t>) {
    let dst = dst.transmute_uninit_ptr();
    let src = this.transmute_ref();
    let src = Some(src.clone());
    Inplace::init(dst, src);
}

/// Returns total number of bytes in the payload.
#[no_mangle]
extern "C" fn z_bytes_len(this: &z_loaned_bytes_t) -> usize {
    this.transmute_ref().len()
}

/// Decodes data into an owned non-null-terminated string.
///
/// @param this_: Data to decode.
/// @param dst: An unitialized memory location where to construct a decoded string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_decode_into_string(
    this: &z_loaned_bytes_t,
    dst: *mut MaybeUninit<z_owned_str_t>,
) -> z_error_t {
    let payload = this.transmute_ref();
    match payload.deserialize::<String>() {
        Ok(s) => {
            Inplace::init(dst.transmute_uninit_ptr(), s.into());
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to decode the payload: {}", e);
            Inplace::empty(dst.transmute_uninit_ptr());
            errors::Z_EIO
        }
    }
}

/// Decodes data into an owned bytes map.
///
/// @param this_: Data to decode.
/// @param dst: An unitialized memory location where to construct a decoded map.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_decode_into_slice_map(
    this: &z_loaned_bytes_t,
    dst: *mut MaybeUninit<z_owned_slice_map_t>,
) -> z_error_t {
    let dst = dst.transmute_uninit_ptr();
    let payload = this.transmute_ref();
    let iter = payload.iter::<(Vec<u8>, Vec<u8>)>();
    let mut hm = ZHashMap::new();
    for (k, v) in iter {
        hm.insert(k.into(), v.into());
    }
    Inplace::init(dst, Some(hm));
    errors::Z_OK
}

/// Decodes data into an owned slice.
///
/// @param this_: Data to decode.
/// @param dst: An unitialized memory location where to construct a slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_decode_into_slice(
    this: &z_loaned_bytes_t,
    dst: *mut MaybeUninit<z_owned_slice_t>,
) -> z_error_t {
    let payload = this.transmute_ref();
    match payload.deserialize::<Vec<u8>>() {
        Ok(v) => {
            Inplace::init(dst.transmute_uninit_ptr(), v.into());
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to read the payload: {}", e);
            Inplace::empty(dst.transmute_uninit_ptr());
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

fn z_bytes_encode_from_arithmetic<T>(this: *mut MaybeUninit<z_owned_bytes_t>, val: T)
where
    ZSerde: Serialize<T, Output = ZBytes>,
{
    let this = this.transmute_uninit_ptr();
    let payload = ZBytes::serialize(val);
    Inplace::init(this, Some(payload));
}

fn z_bytes_decode_into_arithmetic<T>(this: &z_loaned_bytes_t, val: &mut T) -> z_error_t
where
    ZSerde: Deserialize<'static, T, Input = &'static ZBytes>,
    <ZSerde as Deserialize<'static, T>>::Error: fmt::Debug,
{
    match this.transmute_ref().deserialize::<T>() {
        Ok(v) => {
            *val = v;
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Failed to decode the payload: {}", e);
            errors::Z_EPARSE
        }
    }
}

/// Encodes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_uint8(this: *mut MaybeUninit<z_owned_bytes_t>, val: u8) {
    z_bytes_encode_from_arithmetic::<u8>(this, val);
}

/// Encodes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_uint16(this: *mut MaybeUninit<z_owned_bytes_t>, val: u16) {
    z_bytes_encode_from_arithmetic::<u16>(this, val);
}

/// Encodes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_uint32(this: *mut MaybeUninit<z_owned_bytes_t>, val: u32) {
    z_bytes_encode_from_arithmetic::<u32>(this, val);
}

/// Encodes an unsigned integer.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_uint64(this: *mut MaybeUninit<z_owned_bytes_t>, val: u64) {
    z_bytes_encode_from_arithmetic::<u64>(this, val);
}

/// Encodes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_int8(this: *mut MaybeUninit<z_owned_bytes_t>, val: i8) {
    z_bytes_encode_from_arithmetic::<i8>(this, val);
}

/// Encodes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_int16(this: *mut MaybeUninit<z_owned_bytes_t>, val: i16) {
    z_bytes_encode_from_arithmetic::<i16>(this, val);
}

/// Encodes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_int32(this: *mut MaybeUninit<z_owned_bytes_t>, val: i32) {
    z_bytes_encode_from_arithmetic::<i32>(this, val);
}

/// Encodes a signed integer.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_int64(this: *mut MaybeUninit<z_owned_bytes_t>, val: i64) {
    z_bytes_encode_from_arithmetic::<i64>(this, val);
}

/// Encodes a float.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_float(this: *mut MaybeUninit<z_owned_bytes_t>, val: f32) {
    z_bytes_encode_from_arithmetic::<f32>(this, val);
}

/// Encodes a double.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_double(this: *mut MaybeUninit<z_owned_bytes_t>, val: f64) {
    z_bytes_encode_from_arithmetic::<f64>(this, val);
}
/// Decodes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_uint8(this: &z_loaned_bytes_t, dst: &mut u8) -> z_error_t {
    z_bytes_decode_into_arithmetic::<u8>(this, dst)
}

/// Decodes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_uint16(this: &z_loaned_bytes_t, dst: &mut u16) -> z_error_t {
    z_bytes_decode_into_arithmetic::<u16>(this, dst)
}

/// Decodes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_uint32(this: &z_loaned_bytes_t, dst: &mut u32) -> z_error_t {
    z_bytes_decode_into_arithmetic::<u32>(this, dst)
}

/// Decodes into an unsigned integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_uint64(this: &z_loaned_bytes_t, dst: &mut u64) -> z_error_t {
    z_bytes_decode_into_arithmetic::<u64>(this, dst)
}

/// Decodes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_int8(this: &z_loaned_bytes_t, dst: &mut i8) -> z_error_t {
    z_bytes_decode_into_arithmetic::<i8>(this, dst)
}

/// Decodes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_int16(this: &z_loaned_bytes_t, dst: &mut i16) -> z_error_t {
    z_bytes_decode_into_arithmetic::<i16>(this, dst)
}

/// Decodes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_int32(this: &z_loaned_bytes_t, dst: &mut i32) -> z_error_t {
    z_bytes_decode_into_arithmetic::<i32>(this, dst)
}

/// Decodes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_int64(this: &z_loaned_bytes_t, dst: &mut i64) -> z_error_t {
    z_bytes_decode_into_arithmetic::<i64>(this, dst)
}

/// Decodes into a float.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_float(this: &z_loaned_bytes_t, dst: &mut f32) -> z_error_t {
    z_bytes_decode_into_arithmetic::<f32>(this, dst)
}

/// Decodes into a signed integer.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_double(this: &z_loaned_bytes_t, dst: &mut f64) -> z_error_t {
    z_bytes_decode_into_arithmetic::<f64>(this, dst)
}

/// Encodes a slice by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_slice(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    data: *const u8,
    len: usize,
) {
    let s = CSlice::new_borrowed(data, len);
    let this = this.transmute_uninit_ptr();
    let payload = ZBytes::from(ZSlice::from(s));
    Inplace::init(this, Some(payload));
}

/// Encodes a slice by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_slice_copy(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    data: *const u8,
    len: usize,
) {
    let s = CSlice::new_borrowed(data, len).clone();
    let this = this.transmute_uninit_ptr();
    let payload = ZBytes::from(ZSlice::from(s));
    Inplace::init(this, Some(payload));
}

/// Encodes slice map by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_slice_map(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    bytes_map: &z_loaned_slice_map_t,
) {
    let dst = this.transmute_uninit_ptr();
    let hm = bytes_map.transmute_ref();
    let payload = ZBytes::from_iter(hm.iter().map(|(k, v)| {
        (
            CSlice::new_borrowed(k.data(), k.len()),
            CSlice::new_borrowed(v.data(), v.len()),
        )
    }));
    Inplace::init(dst, Some(payload));
}

/// Encodes slice map by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_slice_map_copy(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    bytes_map: &z_loaned_slice_map_t,
) {
    let dst = this.transmute_uninit_ptr();
    let hm = bytes_map.transmute_ref();
    let payload = ZBytes::from_iter(hm.iter().map(|(k, v)| {
        (
            CSlice::new_borrowed(k.data(), k.len()).clone(),
            CSlice::new_borrowed(v.data(), v.len()).clone(),
        )
    }));
    Inplace::init(dst, Some(payload));
}

/// Encodes a null-terminated string by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_string(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    s: *const libc::c_char,
) {
    z_bytes_encode_from_slice(this, s as *const u8, libc::strlen(s));
}

/// Encodes a null-terminated string by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_string_copy(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    s: *const libc::c_char,
) {
    z_bytes_encode_from_slice_copy(this, s as *const u8, libc::strlen(s));
}

/// Encodes a pair of `z_owned_bytes` objects which are consumed in the process.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_pair(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    first: &mut z_owned_bytes_t,
    second: &mut z_owned_bytes_t,
) -> z_error_t {
    let first = match first.transmute_mut().extract() {
        Some(z) => z,
        None => return Z_EINVAL,
    };
    let second = match second.transmute_mut().extract() {
        Some(z) => z,
        None => return Z_EINVAL,
    };
    let b = ZBytes::serialize((first, second));
    Inplace::init(this.transmute_uninit_ptr(), Some(b));
    Z_OK
}

/// Decodes into a pair of `z_owned_bytes` objects.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_decode_into_pair(
    this: &z_loaned_bytes_t,
    first: *mut MaybeUninit<z_owned_bytes_t>,
    second: *mut MaybeUninit<z_owned_bytes_t>,
) -> z_error_t {
    match this.transmute_ref().deserialize::<(ZBytes, ZBytes)>() {
        Ok((a, b)) => {
            Inplace::init(first.transmute_uninit_ptr(), Some(a));
            Inplace::init(second.transmute_uninit_ptr(), Some(b));
            Z_OK
        }
        Err(e) => {
            log::error!("Failed to decode the payload: {}", e);
            Z_EPARSE
        }
    }
}

struct ZBytesInIterator {
    body: extern "C" fn(data: &mut MaybeUninit<z_owned_bytes_t>, context: *mut c_void),
    context: *mut c_void,
}

impl Iterator for ZBytesInIterator {
    type Item = ZBuf;

    fn next(&mut self) -> Option<ZBuf> {
        let mut data = MaybeUninit::<z_owned_bytes_t>::uninit();

        (self.body)(&mut data, self.context);
        unsafe { data.assume_init().transmute_mut().extract() }.map(|b| b.into())
    }
}

/// Constructs payload from an iterator to `z_owned_bytes_t`.
/// @param this_: An uninitialized location in memery for `z_owned_bytes_t` will be constructed.
/// @param iterator_body: Iterator body function, providing data items. Returning NULL
/// @param context: Arbitrary context that will be passed to iterator_body.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_bytes_encode_from_iter(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    iterator_body: extern "C" fn(data: &mut MaybeUninit<z_owned_bytes_t>, context: *mut c_void),
    context: *mut c_void,
) -> z_error_t {
    let it = ZBytesInIterator {
        body: iterator_body,
        context,
    };

    let b = ZBytes::from_iter(it);
    Inplace::init(this.transmute_uninit_ptr(), Some(b));
    Z_OK
}

pub use crate::z_bytes_iterator_t;
decl_transmute_handle!(ZBytesIterator<'static, ZBuf>, z_bytes_iterator_t);
/// Returns an iterator for multi-piece serialized data.
///
/// The `data` should outlive the iterator.
#[no_mangle]
pub extern "C" fn z_bytes_get_iterator(data: &z_loaned_bytes_t) -> z_bytes_iterator_t {
    *data.transmute_ref().iter::<ZBuf>().transmute_handle()
}

/// Constructs `z_owned_bytes` object corresponding to the next element of encoded data.
///
/// Will construct `z_owned_bytes` when iterator reaches the end.
/// @return ``false`` when iterator reaches the end,  ``true`` otherwise
#[no_mangle]
pub extern "C" fn z_bytes_iterator_next(
    iter: &mut z_bytes_iterator_t,
    out: *mut MaybeUninit<z_owned_bytes_t>,
) -> bool {
    let res = iter.transmute_mut().next().map(|z| z.into());
    if res.is_none() {
        Inplace::empty(out.transmute_uninit_ptr());
        false
    } else {
        Inplace::init(out.transmute_uninit_ptr(), res);
        true
    }
}

/// Returns an iterator for multi-piece serialized data.
/// @param this_: Data to decode.
#[no_mangle]
pub extern "C" fn z_bytes_iter(
    this: &z_loaned_bytes_t,
    iterator_body: extern "C" fn(data: &z_loaned_bytes_t, context: *mut c_void) -> z_error_t,
    context: *mut c_void,
) -> z_error_t {
    let mut res = Z_OK;
    for zb in this.transmute_ref().iter::<ZBuf>() {
        let b = ZBytes::new(zb);
        res = iterator_body(b.transmute_handle(), context);
        if res != Z_OK {
            break;
        }
    }

    res
}

pub use crate::z_bytes_reader_t;
decl_transmute_handle!(ZBytesReader<'static>, z_bytes_reader_t);
/// Returns a reader for the data.
///
/// The `data` should outlive the reader.
#[no_mangle]
pub extern "C" fn z_bytes_get_reader(data: &z_loaned_bytes_t) -> z_bytes_reader_t {
    *data.transmute_ref().reader().transmute_handle()
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
    this: &mut z_loaned_bytes_t,
    out: *mut MaybeUninit<z_owned_bytes_writer_t>,
) {
    let out = out.transmute_uninit_ptr();
    Inplace::init(out, Some(this.transmute_mut().writer()));
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
