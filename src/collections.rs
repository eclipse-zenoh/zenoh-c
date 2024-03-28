//
// Copyright (c) 2017, 2022 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//

use libc::{c_char, size_t};
use zenoh::{
    buffers::{buffer::SplitBuffer, ZBuf},
    prelude::ZenohId,
};

use crate::impl_guarded_transmute;

/// A contiguous view of bytes owned by some other entity.
///
/// `start` being `null` is considered a gravestone value,
/// and empty slices are represented using a possibly dangling pointer for `start`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct z_bytes_t {
    pub start: *const u8,
    pub len: size_t,
}

impl z_bytes_t {
    pub fn as_slice(&self) -> Option<&[u8]> {
        if self.start.is_null() {
            return None;
        }
        Some(unsafe { core::slice::from_raw_parts(self.start, self.len) })
    }
    pub fn empty() -> Self {
        z_bytes_t {
            start: std::ptr::null(),
            len: 0,
        }
    }
}

impl Default for z_bytes_t {
    fn default() -> Self {
        Self::empty()
    }
}

/// Returns ``true`` if `b` is initialized.
#[no_mangle]
pub extern "C" fn z_bytes_check(b: &z_bytes_t) -> bool {
    !b.start.is_null()
}

/// Returns the gravestone value for `z_bytes_t`
#[no_mangle]
pub const extern "C" fn z_bytes_null() -> z_bytes_t {
    z_bytes_t {
        len: 0,
        start: core::ptr::null(),
    }
}

/// Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// `str == NULL` will cause this to return `z_bytes_null()`
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_str(str: *const c_char) -> z_bytes_t {
    if str.is_null() {
        z_bytes_null()
    } else {
        let len = unsafe { libc::strlen(str) };
        z_bytes_t {
            len,
            start: str.cast(),
        }
    }
}

#[deprecated = "Renamed to z_bytes_from_str"]
/// Deprecated in favor of `z_bytes_from_str`: Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// `str == NULL` will cause this to return `z_bytes_null()`
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_new(str: *const c_char) -> z_bytes_t {
    z_bytes_from_str(str)
}

/// Constructs a `len` bytes long view starting at `start`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_wrap(start: *const u8, len: usize) -> z_bytes_t {
    if start.is_null() {
        z_bytes_null()
    } else {
        z_bytes_t { len, start }
    }
}

/// Frees `b` and invalidates it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
pub(crate) unsafe fn z_bytes_drop(b: &mut z_bytes_t) {
    if !b.start.is_null() {
        std::mem::drop(Box::from_raw(
            core::ptr::slice_from_raw_parts(b.start, b.len).cast_mut(),
        ));
        b.start = std::ptr::null();
    }
}

impl From<ZenohId> for z_bytes_t {
    #[inline]
    fn from(pid: ZenohId) -> Self {
        let pid = pid.to_le_bytes().to_vec().into_boxed_slice();
        let res = z_bytes_t {
            start: pid.as_ptr(),
            len: pid.len() as size_t,
        };
        std::mem::forget(pid);
        res
    }
}

impl From<Option<ZenohId>> for z_bytes_t {
    #[inline]
    fn from(pid: Option<ZenohId>) -> Self {
        match pid {
            Some(pid) => pid.into(),
            None => z_bytes_t {
                start: std::ptr::null(),
                len: 0,
            },
        }
    }
}

impl From<z_bytes_t> for String {
    fn from(s: z_bytes_t) -> Self {
        unsafe {
            String::from_utf8(
                Box::from_raw(std::slice::from_raw_parts_mut(s.start as *mut u8, s.len)).into(),
            )
            .unwrap()
        }
    }
}

impl From<&[u8]> for z_bytes_t {
    fn from(s: &[u8]) -> Self {
        z_bytes_t {
            start: s.as_ptr(),
            len: s.len(),
        }
    }
}

/// A split buffer that owns all of its data.
///
/// To minimize copies and reallocations, Zenoh may provide you data in split buffers.
///
/// You can use `z_buffer_contiguous` to obtain a contiguous version of a buffer.
/// If the buffer was already contiguous, the reference count will simply be increased.
/// Otherwise, the split buffer's entire content will be copied in a newly allocated buffer.
#[repr(C)]
pub struct z_owned_buffer_t {
    _inner: [usize; 5],
}
impl_guarded_transmute!(noderefs Option<ZBuf>, z_owned_buffer_t);
impl Default for z_owned_buffer_t {
    fn default() -> Self {
        z_buffer_null()
    }
}
impl From<ZBuf> for z_owned_buffer_t {
    fn from(value: ZBuf) -> Self {
        let value = match value.contiguous() {
            std::borrow::Cow::Borrowed(_) => value,
            std::borrow::Cow::Owned(value) => value.into(),
        };
        unsafe { core::mem::transmute(Some(value)) }
    }
}
impl From<Option<ZBuf>> for z_owned_buffer_t {
    fn from(value: Option<ZBuf>) -> Self {
        match value {
            Some(value) => value.into(),
            None => z_buffer_null(),
        }
    }
}
impl core::ops::Deref for z_owned_buffer_t {
    type Target = Option<ZBuf>;

    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl core::ops::DerefMut for z_owned_buffer_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

/// The gravestone value for `z_owned_buffer_t`.
#[no_mangle]
pub extern "C" fn z_buffer_null() -> z_owned_buffer_t {
    unsafe { core::mem::transmute(None::<ZBuf>) }
}

/// Decrements the buffer's reference counter, destroying it if applicable.
///
/// `buffer` will be reset to `z_buffer_null`, preventing UB on double-frees.
#[no_mangle]
pub extern "C" fn z_buffer_drop(buffer: &mut z_owned_buffer_t) {
    core::mem::drop(buffer.take())
}

/// Returns `true` if the buffer is in a valid state.
#[no_mangle]
pub extern "C" fn z_buffer_check(buffer: &z_owned_buffer_t) -> bool {
    buffer.is_some()
}

/// Loans the buffer, allowing you to call functions that only need a loan of it.
#[no_mangle]
pub extern "C" fn z_buffer_loan(buffer: &z_owned_buffer_t) -> z_buffer_t {
    buffer.as_ref().into()
}

/// A loan of a `z_owned_buffer_t`.
///
/// As it is a split buffer, it may contain more than one slice. It's number of slices is returned by `z_buffer_slice_count`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_buffer_t<'a> {
    _inner: &'a (),
}
impl_guarded_transmute!(Option<&'a ZBuf>, z_buffer_t<'a>, 'a);
impl<'a> From<z_buffer_t<'a>> for Option<&'a ZBuf> {
    fn from(value: z_buffer_t) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

/// Increments the buffer's reference count, returning an owned version of the buffer.
#[no_mangle]
pub extern "C" fn z_buffer_clone(buffer: z_buffer_t) -> z_owned_buffer_t {
    unsafe { Some(core::mem::transmute::<_, &ZBuf>(buffer).clone()).into() }
}

/// Returns the payload of the buffer if it is contiguous, aliasling it.
///
/// If the payload was not contiguous in memory, `z_bytes_null` will be returned instead.
#[no_mangle]
pub extern "C" fn z_buffer_payload(buffer: z_buffer_t) -> z_bytes_t {
    let Some(buffer): Option<&ZBuf> = buffer.into() else {
        return z_bytes_null();
    };
    match buffer.contiguous() {
        std::borrow::Cow::Borrowed(buffer) => buffer.into(),
        std::borrow::Cow::Owned(_) => z_bytes_null(),
    }
}

/// Returns an owned version of this buffer whose data is guaranteed to be contiguous in memory.
///
/// This is achieved by increasing the reference count if the buffer is already contiguous, and by copying its data in a new contiguous buffer if it wasn't.
#[no_mangle]
pub extern "C" fn z_buffer_contiguous(buffer: z_buffer_t) -> z_owned_buffer_t {
    let Some(buf): Option<&ZBuf> = buffer.into() else {
        return z_buffer_null();
    };
    match buf.contiguous() {
        std::borrow::Cow::Borrowed(_) => buf.clone().into(),
        std::borrow::Cow::Owned(buf) => ZBuf::from(buf).into(),
    }
}

/// Returns the number of slices in the buffer.
///
/// If the return value is 0 or 1, then the buffer's data is contiguous in memory and `z_buffer_contiguous` will succeed.
#[no_mangle]
pub extern "C" fn z_buffer_slice_count(buffer: z_buffer_t) -> usize {
    match buffer.into() {
        None => 0,
        Some(buf) => ZBuf::slices(buf).len(),
    }
}

/// Returns the `index`th slice of the buffer, aliasing it.
///
/// Out of bounds accesses will return `z_bytes_null`.
#[no_mangle]
pub extern "C" fn z_buffer_slice_at(buffer: z_buffer_t, index: usize) -> z_bytes_t {
    match buffer.into() {
        None => z_bytes_null(),
        Some(buf) => ZBuf::slices(buf)
            .nth(index)
            .map_or(z_bytes_null(), |slice| slice.into()),
    }
}
