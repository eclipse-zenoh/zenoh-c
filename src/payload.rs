use core::slice;
use std::mem::MaybeUninit;
use std::slice::from_raw_parts_mut;
use std::{any::Any, ops::Deref, ptr::NonNull};

use zenoh::buffers::HasReader;
use zenoh::buffers::Reader;
use zenoh::buffers::ZBufReader;
use zenoh::buffers::{SplitBuffer, ZBuf, ZSliceBuffer};

use crate::{
    impl_guarded_transmute, z_bytes_empty, z_bytes_null, z_bytes_t, z_owned_bytes_t, z_owned_str_t,
    z_str_null, GuardedTransmute,
};

pub use crate::opaque_types::z_owned_buffer_t;
impl_guarded_transmute!(Option<ZBuf>, z_owned_buffer_t);

impl Default for z_owned_buffer_t {
    fn default() -> Self {
        z_buffer_null()
    }
}
impl From<ZBuf> for z_owned_buffer_t {
    fn from(value: ZBuf) -> Self {
        Some(value).transmute()
    }
}

/// The gravestone value for `z_owned_buffer_t`.
#[no_mangle]
extern "C" fn z_buffer_null() -> z_owned_buffer_t {
    None::<ZBuf>.transmute()
}

/// Decrements the buffer's reference counter, destroying it if applicable.
///
/// `buffer` will be reset to `z_buffer_null`, preventing UB on double-frees.
#[no_mangle]
extern "C" fn z_buffer_drop(buffer: &mut z_owned_buffer_t) {
    core::mem::drop(buffer.take())
}

/// Returns `true` if the buffer is in a valid state.
#[no_mangle]
extern "C" fn z_buffer_check(buffer: &z_owned_buffer_t) -> bool {
    buffer.is_some()
}

/// Loans the buffer, allowing you to call functions that only need a loan of it.
#[no_mangle]
extern "C" fn z_buffer_loan(buffer: &z_owned_buffer_t) -> z_buffer_t {
    buffer.as_ref().into()
}

/// A loan of a `z_owned_buffer_t`.
///
/// As it is a split buffer, it may contain more than one slice. It's number of slices is returned by `z_buffer_slice_count`.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct z_buffer_t {
    _inner: Option<NonNull<z_owned_buffer_t>>,
}

impl_guarded_transmute!(noderefs Option<&ZBuf>, z_buffer_t);
impl_guarded_transmute!(noderefs z_buffer_t, Option<&'static ZBuf>);

impl From<Option<&ZBuf>> for z_buffer_t {
    fn from(value: Option<&ZBuf>) -> Self {
        value.transmute()
    }
}

impl From<z_buffer_t> for Option<&'static ZBuf> {
    fn from(value: z_buffer_t) -> Self {
        value.transmute()
    }
}

/// Increments the buffer's reference count, returning an owned version of the buffer.
#[no_mangle]
extern "C" fn z_buffer_clone(buffer: z_buffer_t) -> z_owned_buffer_t {
    match buffer._inner {
        Some(b) => unsafe { b.as_ref().deref().clone().transmute() },
        None => ZBuf::empty().into(),
    }
}

/// Returns the number of slices in the buffer.
///
/// If the return value is 0 or 1, then the buffer's data is contiguous in memory.
#[no_mangle]
extern "C" fn z_buffer_slice_count(buffer: z_buffer_t) -> usize {
    match buffer.into() {
        None => 0,
        Some(buf) => ZBuf::slices(buf).len(),
    }
}

/// Returns total number bytes in the buffer.
#[no_mangle]
extern "C" fn z_buffer_len(buffer: z_buffer_t) -> usize {
    match buffer.into() {
        None => 0,
        Some(buf) => ZBuf::slices(buf).fold(0, |acc, s| acc + s.len()),
    }
}

/// Returns the `index`th slice of the buffer, aliasing it.
///
/// Out of bounds accesses will return `z_bytes_empty`.
#[no_mangle]
extern "C" fn z_buffer_slice_at(buffer: z_buffer_t, index: usize) -> z_bytes_t {
    match buffer.into() {
        None => z_bytes_empty(),
        Some(buf) => ZBuf::slices(buf)
            .nth(index)
            .map_or(z_bytes_empty(), |slice| slice.into()),
    }
}

/// An owned payload, backed by a reference counted owner.
///
/// The `payload` field may be modified, and Zenoh will take the new values into account.
#[allow(non_camel_case_types)]
pub type zc_owned_payload_t = z_owned_buffer_t;

/// Clones the `payload` by incrementing its reference counter.
#[no_mangle]
pub extern "C" fn zc_payload_rcinc(payload: &zc_owned_payload_t) -> zc_owned_payload_t {
    z_buffer_clone(z_buffer_loan(payload))
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
pub extern "C" fn zc_payload_null() -> zc_owned_payload_t {
    z_buffer_null()
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
pub extern "C" fn zc_payload_clone(payload: zc_payload_t) -> zc_owned_payload_t {
    z_buffer_clone(payload)
}

/// Decodes payload into null-terminated string
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_decode_into_string(
    payload: zc_payload_t,
    cstr: &mut z_owned_str_t,
) -> i8 {
    let payload: Option<&ZBuf> = payload.into();
    if payload.is_none() {
        *cstr = z_str_null();
        return 0;
    }
    *cstr = z_owned_str_t::preallocate(zc_payload_len(payload.into()));
    let payload = payload.unwrap();

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
    let payload: Option<&ZBuf> = payload.into();
    if payload.is_none() {
        *b = z_bytes_null();
        return 0;
    }
    *b = z_owned_bytes_t::preallocate(zc_payload_len(payload.into()));
    let payload = payload.unwrap();

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
pub unsafe extern "C" fn zc_payload_encode_from_bytes(bytes: z_bytes_t) -> zc_owned_payload_t {
    ZBuf::from(bytes).into()
}

/// Encodes a null-terminated string by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_encode_from_string(
    cstr: *const libc::c_char,
) -> zc_owned_payload_t {
    let bytes = z_bytes_t {
        start: cstr as *const u8,
        len: libc::strlen(cstr),
    };
    zc_payload_encode_from_bytes(bytes)
}

/// Returns total number bytes in the payload.
#[no_mangle]
pub extern "C" fn zc_payload_len(payload: zc_payload_t) -> usize {
    z_buffer_len(payload)
}

pub use crate::opaque_types::zc_payload_reader;
decl_transmute_copy!(ZBufReader<'static>, zc_payload_reader);

/// Creates a reader for the specified `payload`.
///
/// Returns 0 in case of success, -1 if `payload` is not valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_reader_init(
    payload: zc_payload_t,
    reader: *mut MaybeUninit<zc_payload_reader>,
) -> i8 {
    if payload._inner.is_none() {
        return -1;
    }
    *reader = payload.transmute().unwrap().reader().transmute();
    0
}

/// Reads data into specified destination.
///
/// Will read at most `len` bytes.
/// Returns number of bytes read. If return value is smaller than `len`, it means that end of the payload was reached.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_reader_read(
    reader: *mut zc_payload_reader,
    dest: *mut u8,
    len: usize,
) -> usize {
    let buf = unsafe { from_raw_parts_mut(dest, len) };
    reader
        .as_mut()
        .unwrap()
        .read(buf)
        .map(|n| n.get())
        .unwrap_or(0)
}

/// Returns number of the remaining bytes in the payload
///
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_payload_reader_remaining(reader: *const zc_payload_reader) -> usize {
    reader.as_ref().unwrap().remaining()
}
