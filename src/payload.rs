use crate::transmute::{
    unwrap_ref_unchecked, Inplace, TransmuteCopy, TransmuteRef, TransmuteUninitPtr,
};
use crate::{z_bytes_empty, z_bytes_t, z_owned_bytes_t, z_owned_str_t};
use core::slice;
use std::any::Any;
use std::mem::MaybeUninit;
use std::slice::from_raw_parts_mut;
use zenoh::buffers::HasReader;
use zenoh::buffers::Reader;
use zenoh::buffers::ZBufReader;
use zenoh::buffers::{SplitBuffer, ZBuf, ZSliceBuffer};

pub use crate::opaque_types::z_owned_buffer_t;
decl_transmute_owned!(Option<ZBuf>, z_owned_buffer_t);

/// The gravestone value for `z_owned_buffer_t`.
#[no_mangle]
extern "C" fn z_buffer_null(this: *mut MaybeUninit<z_owned_buffer_t>) {
    let this = z_owned_buffer_t::transmute_uninit_ptr(this);
    Inplace::empty(this);
}

/// Decrements the buffer's reference counter, destroying it if applicable.
///
/// `buffer` will be reset to `z_buffer_null`, preventing UB on double-frees.
#[no_mangle]
extern "C" fn z_buffer_drop(buffer: &mut z_owned_buffer_t) {
    let buffer = buffer.transmute_mut();
    Inplace::drop(buffer);
}

/// Returns `true` if the buffer is in a valid state.
#[no_mangle]
extern "C" fn z_buffer_check(buffer: &z_owned_buffer_t) -> bool {
    buffer.transmute_ref().is_some()
}

/// Loans the buffer, allowing you to call functions that only need a loan of it.
#[no_mangle]
extern "C" fn z_buffer_loan(buffer: &z_owned_buffer_t) -> z_buffer_t {
    let buffer = buffer.transmute_ref();
    let buffer = unwrap_ref_unchecked(buffer);
    buffer.transmute_copy()
}

/// A loan of a `z_owned_buffer_t`.
///
/// As it is a split buffer, it may contain more than one slice. It's number of slices is returned by `z_buffer_slice_count`.
pub use crate::opaque_types::z_buffer_t;
decl_transmute_copy!(&'static ZBuf, z_buffer_t);

/// Increments the buffer's reference count, returning an owned version of the buffer.
#[no_mangle]
extern "C" fn z_buffer_clone(dst: *mut MaybeUninit<z_owned_buffer_t>, buffer: &z_owned_buffer_t) {
    let dst = dst.transmute_uninit_ptr();
    let buffer = buffer.transmute_ref();
    let buffer = buffer.as_ref().map(Clone::clone);
    Inplace::init(dst, buffer);
}

/// Returns the number of slices in the buffer.
///
/// If the return value is 0 or 1, then the buffer's data is contiguous in memory.
#[no_mangle]
extern "C" fn z_buffer_slice_count(buffer: z_buffer_t) -> usize {
    ZBuf::slices(buffer.transmute_copy()).len()
}

/// Returns total number bytes in the buffer.
#[no_mangle]
extern "C" fn z_buffer_len(buffer: z_buffer_t) -> usize {
    ZBuf::slices(buffer.transmute_copy()).fold(0, |acc, s| acc + s.len())
}

/// Returns the `index`th slice of the buffer, aliasing it.
///
/// Out of bounds accesses will return `z_bytes_empty`.
#[no_mangle]
extern "C" fn z_buffer_slice_at(buffer: z_buffer_t, index: usize) -> z_bytes_t {
    let buf = buffer.transmute_copy();
    ZBuf::slices(buf)
        .nth(index)
        .map_or(z_bytes_empty(), |slice| slice.into())
}

/// An owned payload, backed by a reference counted owner.
///
/// The `payload` field may be modified, and Zenoh will take the new values into account.
#[allow(non_camel_case_types)]
pub type zc_owned_payload_t = z_owned_buffer_t;

/// Clones the `payload` by incrementing its reference counter.
#[no_mangle]
pub extern "C" fn zc_payload_rcinc(
    dst: *mut MaybeUninit<zc_owned_payload_t>,
    payload: &zc_owned_payload_t,
) {
    z_buffer_clone(dst, payload)
}
/// Returns `false` if `payload` is the gravestone value.
#[no_mangle]
pub extern "C" fn zc_payload_check(payload: &zc_owned_payload_t) -> bool {
    z_buffer_check(payload)
}
/// Decrements `payload`'s backing refcount, releasing the memory if appropriate.
#[no_mangle]
pub extern "C" fn zc_payload_drop(payload: &mut zc_owned_payload_t) {
    z_buffer_drop(payload)
}
/// Constructs `zc_owned_payload_t`'s gravestone value.
#[no_mangle]
pub extern "C" fn zc_payload_null(this: *mut MaybeUninit<zc_owned_payload_t>) {
    z_buffer_null(this);
}

/// Returns a :c:type:`zc_payload_t` loaned from `payload`.
#[no_mangle]
pub extern "C" fn zc_payload_loan(payload: &zc_owned_payload_t) -> zc_payload_t {
    z_buffer_loan(payload)
}

#[allow(non_camel_case_types)]
pub type zc_payload_t = z_buffer_t;

/// Increments internal payload reference count, returning owned payload.
#[no_mangle]
pub extern "C" fn zc_payload_clone(
    dst: *mut MaybeUninit<zc_owned_payload_t>,
    payload: &zc_owned_payload_t,
) {
    z_buffer_clone(dst, payload)
}

/// Decodes payload into null-terminated string
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_decode_into_string(
    payload: zc_payload_t,
    cstr: &mut z_owned_str_t,
) -> i8 {
    *cstr = z_owned_str_t::preallocate(zc_payload_len(payload));
    let payload = payload.transmute_copy();
    let mut pos = 0;
    for s in payload.slices() {
        cstr.insert_unchecked(pos, s);
        pos += s.len();
    }
    0
}

/// Decodes payload into null-terminated string
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_decode_into_bytes(
    payload: zc_payload_t,
    b: &mut z_owned_bytes_t,
) -> i8 {
    *b = z_owned_bytes_t::preallocate(zc_payload_len(payload));
    let payload = payload.transmute_copy();
    let mut pos = 0;
    for s in payload.slices() {
        b.insert_unchecked(pos, s);
        pos += s.len();
    }
    0
}

unsafe impl Send for z_bytes_t {}
unsafe impl Sync for z_bytes_t {}

impl ZSliceBuffer for z_bytes_t {
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
pub unsafe extern "C" fn zc_payload_encode_from_bytes(
    dst: *mut MaybeUninit<zc_owned_payload_t>,
    bytes: z_bytes_t,
) {
    let dst = dst.transmute_uninit_ptr();
    let buf = ZBuf::from(bytes);
    Inplace::init(dst, Some(buf));
}

/// Encodes a null-terminated string by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_encode_from_string(
    dst: *mut MaybeUninit<zc_owned_payload_t>,
    cstr: *const libc::c_char,
) {
    let bytes = z_bytes_t {
        start: cstr as *const u8,
        len: libc::strlen(cstr),
    };
    zc_payload_encode_from_bytes(dst, bytes);
}

/// Returns total number bytes in the payload.
#[no_mangle]
pub extern "C" fn zc_payload_len(payload: zc_payload_t) -> usize {
    z_buffer_len(payload)
}

pub use crate::opaque_types::zc_owned_payload_reader;
decl_transmute_owned!(Option<ZBufReader<'static>>, zc_owned_payload_reader);

pub use crate::opaque_types::zc_payload_reader;
decl_transmute_copy!(&'static ZBufReader<'static>, zc_payload_reader);

/// Creates a reader for the specified `payload`.
///
/// Returns 0 in case of success, -1 if `payload` is not valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_reader_init(
    this: *mut MaybeUninit<zc_owned_payload_reader>,
    payload: zc_payload_t,
) {
    let this = this.transmute_uninit_ptr();
    let payload = payload.transmute_copy();
    let reader = payload.reader();
    Inplace::init(this, Some(reader));
}

/// Reads data into specified destination.
///
/// Will read at most `len` bytes.
/// Returns number of bytes read. If return value is smaller than `len`, it means that end of the payload was reached.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_reader_read(
    reader: zc_payload_reader,
    dest: *mut u8,
    len: usize,
) -> usize {
    let reader = reader.transmute_copy();
    let buf = unsafe { from_raw_parts_mut(dest, len) };
    reader.read(buf).map(|n| n.get()).unwrap_or(0)
}

/// Returns number of the remaining bytes in the payload
///
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_reader_remaining(reader: zc_payload_reader) -> usize {
    let reader = reader.transmute_copy();
    reader.remaining()
}
