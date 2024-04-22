use crate::errors::{self, ZCError};
use crate::transmute::{
    unwrap_ref_unchecked, Inplace, TransmuteFromHandle, TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr
};
use crate::{z_owned_slice_map_t, z_owned_slice_t, z_owned_str_t, z_slice_map_t, z_slice_t, ZHashMap};
use core::slice;
use std::any::Any;
use std::io::{Read, Seek, SeekFrom};
use std::mem::MaybeUninit;
use std::slice::from_raw_parts_mut;
use zenoh::buffers::{ZSlice, ZSliceBuffer};
use zenoh::bytes::{ZBytes, ZBytesReader};

pub use crate::opaque_types::z_owned_bytes_t;
decl_transmute_owned!(Option<ZBytes>, z_owned_bytes_t);

/// The gravestone value for `z_owned_bytes_t`.
#[no_mangle]
extern "C" fn z_bytes_null(this: *mut MaybeUninit<z_owned_bytes_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Decrements the payload's reference counter, destroying it if applicable.
///
/// `this` will be reset to `z_buffer_null`, preventing UB on double-frees.
#[no_mangle]
extern "C" fn z_bytes_drop(this: &mut z_owned_bytes_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

/// Returns `true` if the payload is in a valid state.
#[no_mangle]
extern "C" fn z_bytes_check(payload: &z_owned_bytes_t) -> bool {
    payload.transmute_ref().is_some()
}

/// Loans the payload, allowing you to call functions that only need a loan of it.
#[no_mangle]
extern "C" fn z_bytes_loan(payload: &'static z_owned_bytes_t) -> z_bytes_t {
    let payload = payload.transmute_ref();
    let payload = unwrap_ref_unchecked(payload);
    payload.transmute_handle()
}

pub use crate::opaque_types::z_bytes_t;
decl_transmute_handle!(ZBytes, z_bytes_t);

/// Increments the payload's reference count, returning an owned version of it.
#[no_mangle]
extern "C" fn z_bytes_clone(src: &z_owned_bytes_t, dst: *mut MaybeUninit<z_owned_bytes_t>) {
    let dst = dst.transmute_uninit_ptr();
    let src = src.transmute_ref();
    let src = src.as_ref().map(Clone::clone);
    Inplace::init(dst, src);
}

/// Returns total number bytes in the payload.
#[no_mangle]
extern "C" fn z_bytes_len(payload: z_bytes_t) -> usize {
    payload.transmute_ref().len()
}

/// Decodes payload into null-terminated string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_decode_into_string(
    payload: z_bytes_t,
    dst: *mut MaybeUninit<z_owned_str_t>,
) -> ZCError {
    let len = z_bytes_len(payload);
    let cstr = z_owned_str_t::preallocate(len);
    let payload = payload.transmute_ref();
    payload.reader().read(from_raw_parts_mut(cstr._cstr as *mut u8, len));
    Inplace::init(dst, cstr);
    errors::Z_OK
}

/// Decodes payload into bytes map.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_decode_into_bytes_map(
    payload: z_bytes_t,
    dst: *mut MaybeUninit<z_owned_slice_map_t>,
) -> ZCError {
    let dst = dst.transmute_uninit_ptr();
    let payload = payload.transmute_ref();
    let iter = payload.iter::<(Vec<u8>, Vec<u8>)>();
    let mut hm = ZHashMap::new();
    for (k, v) in iter {
        hm.insert(k.into(), v.into());
    }
    Inplace::init(dst, Some(hm));
    errors::Z_OK
}

/// Decodes payload into owned bytes
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_decode_into_bytes(
    payload: z_bytes_t,
    dst: *mut MaybeUninit<z_owned_slice_t>,
) -> ZCError {
    let len = z_bytes_len(payload);
    let b = z_owned_slice_t::preallocate(len);
    let payload = payload.transmute_ref();
    payload.reader().read(from_raw_parts_mut(b.start, len));
    Inplace::init(dst, b);
    errors::Z_OK
}

unsafe impl Send for z_slice_t {}
unsafe impl Sync for z_slice_t {}

impl ZSliceBuffer for z_slice_t {
    fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.start, self.len) }
    }
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.start as *mut u8, self.len) }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Encodes byte sequence by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_bytes(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    bytes: z_slice_t,
) {
    let this = this.transmute_uninit_ptr();
    let payload = ZBytes::from(ZSlice::from(bytes));
    Inplace::init(this, Some(payload));
}

/// Encodes bytes map by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_bytes_map(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    bytes_map: z_slice_map_t,
) {
    let dst = this.transmute_uninit_ptr();
    let hm = bytes_map.transmute_ref();
    let payload = ZBytes::from_iter(hm.iter());
    Inplace::init(dst, Some(payload));
}

/// Encodes a null-terminated string by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_string(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    cstr: *const libc::c_char,
) {
    let bytes = z_slice_t {
        start: cstr as *const u8,
        len: libc::strlen(cstr),
    };
    z_bytes_encode_from_bytes(this, bytes);
}

pub use crate::opaque_types::z_owned_bytes_t_reader_t;
decl_transmute_owned!(Option<ZBytesReader<'static>>, z_owned_bytes_t_reader_t);

pub use crate::opaque_types::z_bytes_reader_t;
decl_transmute_handle!(ZBytesReader<'static>, z_bytes_reader_t);

/// Creates a reader for the specified `payload`.
///
/// Returns 0 in case of success, -1 if `payload` is not valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_new(
    payload: z_bytes_t,
    this: *mut MaybeUninit<z_owned_bytes_t_reader_t>,
) {
    let this = this.transmute_uninit_ptr();
    let payload = payload.transmute_ref();
    let reader = payload.reader();
    Inplace::init(this, Some(reader));
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_null(this: *mut MaybeUninit<z_owned_bytes_t_reader_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_check(reader: &z_owned_bytes_t_reader_t) -> bool {
    let reader = reader.transmute_ref();
    reader.as_ref().is_some()
}

#[no_mangle]
extern "C" fn z_bytes_reader_drop(this: &mut z_owned_bytes_t_reader_t) {
    let reader = this.transmute_mut();
    Inplace::drop(reader);
}

#[no_mangle]
extern "C" fn z_bytes_reader_loan(reader: &'static z_owned_bytes_t_reader_t) -> z_bytes_reader_t {
    let reader = reader.transmute_ref();
    let reader = unwrap_ref_unchecked(reader);
    reader.transmute_handle()
}

/// Reads data into specified destination.
///
/// Will read at most `len` bytes.
/// Returns number of bytes read. If return value is smaller than `len`, it means that end of the payload was reached.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_read(
    reader: z_bytes_reader_t,
    dest: *mut u8,
    len: usize,
) -> usize {
    let reader = reader.transmute_mut();
    let buf = unsafe { from_raw_parts_mut(dest, len) };
    reader.read(buf).unwrap_or(0)
}

/// Sets the `reader` position indicator for the payload to the value pointed to by offset.
/// The new position is exactly offset bytes measured from the beginning of the payload if origin is SEEK_SET,
/// from the current reader position if origin is SEEK_CUR, and from the end of the payload if origin is SEEK_END.
/// Return ​0​ upon success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_seek(reader: z_bytes_reader_t, offset: i64, origin: libc::c_int) -> ZCError {
    let reader = reader.transmute_mut();
    let pos = match origin {
        libc::SEEK_SET => offset.try_into().map(|r| SeekFrom::Start(r)),
        libc::SEEK_CUR => Ok(SeekFrom::Current(offset)),
        libc::SEEK_END => Ok(SeekFrom::End(offset)),
        _ => { return errors::Z_EINVAL; }
    };
    match pos.map(|p| reader.seek(p)) {
        Ok(_) => 0,
        Err(_) => errors::Z_EINVAL
    }
}

/// Returns the read position indicator.
/// Returns read position indicator on success or -1L if failure occurs.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_tell(reader: z_bytes_reader_t) -> i64 {
    let reader = reader.transmute_mut();
    reader.stream_position().map(|p| p as i64).unwrap_or(-1)
}

