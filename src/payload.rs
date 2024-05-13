use crate::errors::{self, z_error_t};
use crate::transmute::{
    unwrap_ref_unchecked, unwrap_ref_unchecked_mut, Inplace, TransmuteFromHandle,
    TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
};
use crate::{
    z_loaned_slice_map_t, z_loaned_slice_t, z_loaned_str_t, z_owned_slice_map_t, z_owned_slice_t,
    z_owned_str_t, CSlice, ZHashMap,
};
use core::fmt;
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

/// Drops `this_`, resetting it to gravestone value. If there are any shallow copies
/// created by `z_bytes_clone()`, they would still stay valid.
#[no_mangle]
extern "C" fn z_bytes_drop(this: &mut z_owned_bytes_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

/// Returns ``true`` if `this_` in a valid state, ``false`` if it is in a gravestone state.
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

pub use crate::opaque_types::z_loaned_bytes_t;
decl_transmute_handle!(ZBytes, z_loaned_bytes_t);

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

/// Decodes data into an owned null-terminated string.
///
/// @param this_: Data to decode.
/// @param dst: An unitialized memory location where to construct a decoded string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_decode_into_string(
    this: &z_loaned_bytes_t,
    dst: *mut MaybeUninit<z_owned_str_t>,
) -> z_error_t {
    let len = z_bytes_len(this);
    let payload = this.transmute_ref();
    let mut out = vec![0u8; len + 1];
    if let Err(e) = payload.reader().read(out.as_mut_slice()) {
        log::error!("Failed to read the payload: {}", e);
        Inplace::empty(dst.transmute_uninit_ptr());
        errors::Z_EIO
    } else if let Err(e) = std::str::from_utf8(out.as_slice()) {
        log::error!("Payload is not a valid utf-8 string: {}", e);
        Inplace::empty(dst.transmute_uninit_ptr());
        errors::Z_EPARSE
    } else {
        Inplace::init(dst.transmute_uninit_ptr(), out.into());
        errors::Z_OK
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

unsafe impl Send for z_loaned_slice_t {}
unsafe impl Sync for z_loaned_slice_t {}

impl fmt::Debug for z_loaned_slice_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.transmute_ref();
        f.debug_struct("z_loaned_slice_t")
            .field("_0", &s.slice())
            .finish()
    }
}

impl ZSliceBuffer for z_loaned_slice_t {
    fn as_slice(&self) -> &[u8] {
        self.transmute_ref().slice()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Encodes a slice by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_slice(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    bytes: &z_loaned_slice_t,
) {
    let this = this.transmute_uninit_ptr();
    let payload = ZBytes::from(ZSlice::from(*bytes));
    Inplace::init(this, Some(payload));
}

/// Encodes slice map by copying.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_slice_map(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    bytes_map: &z_loaned_slice_map_t,
) {
    let dst = this.transmute_uninit_ptr();
    let hm = bytes_map.transmute_ref();
    let payload = ZBytes::from_iter(hm.iter().map(|(k, v)| (k.slice(), v.slice())));
    Inplace::init(dst, Some(payload));
}

/// Encodes string by aliasing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_encode_from_string(
    this: *mut MaybeUninit<z_owned_bytes_t>,
    s: &z_loaned_str_t,
) {
    let s = s.transmute_ref();
    let s_without_terminating_0 = &s.slice()[0..s.len() - 1];
    let s_without_terminating_0 = CSlice::new_borrowed_from_slice(s_without_terminating_0);
    z_bytes_encode_from_slice(this, s_without_terminating_0.transmute_handle());
}

pub use crate::opaque_types::z_owned_bytes_reader_t;
decl_transmute_owned!(Option<ZBytesReader<'static>>, z_owned_bytes_reader_t);

pub use crate::opaque_types::z_loaned_bytes_reader_t;
decl_transmute_handle!(ZBytesReader<'static>, z_loaned_bytes_reader_t);

/// Creates a reader for the specified data.
///
/// The `data` should outlive the reader.
#[no_mangle]
pub extern "C" fn z_bytes_reader_new(
    this: *mut MaybeUninit<z_owned_bytes_reader_t>,
    data: &z_loaned_bytes_t,
) {
    let this = this.transmute_uninit_ptr();
    let payload = data.transmute_ref();
    let reader = payload.reader();
    Inplace::init(this, Some(reader));
}

/// Constructs data reader in a gravestone state.
#[no_mangle]
pub extern "C" fn z_bytes_reader_null(this: *mut MaybeUninit<z_owned_bytes_reader_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Returns ``true`` if `this_` in a valid state, ``false`` if it is in a gravestone state.
#[no_mangle]
pub extern "C" fn z_bytes_reader_check(this: &z_owned_bytes_reader_t) -> bool {
    this.transmute_ref().is_some()
}

/// Frees memory and resets data reader to its gravestone state.
#[no_mangle]
extern "C" fn z_bytes_reader_drop(this: &mut z_owned_bytes_reader_t) {
    let reader = this.transmute_mut();
    Inplace::drop(reader);
}

/// Borrows data reader.
#[no_mangle]
extern "C" fn z_bytes_reader_loan(reader: &z_owned_bytes_reader_t) -> &z_loaned_bytes_reader_t {
    let reader = reader.transmute_ref();
    let reader = unwrap_ref_unchecked(reader);
    reader.transmute_handle()
}

/// Mutably borrows data reader.
#[no_mangle]
extern "C" fn z_bytes_reader_loan_mut(
    reader: &mut z_owned_bytes_reader_t,
) -> &mut z_loaned_bytes_reader_t {
    let reader = reader.transmute_mut();
    let reader = unwrap_ref_unchecked_mut(reader);
    reader.transmute_handle_mut()
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
    this: &mut z_loaned_bytes_reader_t,
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
    this: &mut z_loaned_bytes_reader_t,
    offset: i64,
    origin: libc::c_int,
) -> z_error_t {
    let reader = this.transmute_mut();
    let pos = match origin {
        libc::SEEK_SET => offset.try_into().map(SeekFrom::Start),
        libc::SEEK_CUR => Ok(SeekFrom::Current(offset)),
        libc::SEEK_END => Ok(SeekFrom::End(offset)),
        _ => {
            return errors::Z_EINVAL;
        }
    };
    match pos.map(|p| reader.seek(p)) {
        Ok(_) => 0,
        Err(_) => errors::Z_EINVAL,
    }
}

/// Gets the read position indicator.
/// @return read position indicator on success or -1L if failure occurs.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_reader_tell(this: &mut z_loaned_bytes_reader_t) -> i64 {
    let reader = this.transmute_mut();
    reader.stream_position().map(|p| p as i64).unwrap_or(-1)
}
