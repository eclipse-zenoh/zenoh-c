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
use std::ops::Deref;
use std::ptr::NonNull;
use zenoh::{
    buffers::{buffer::SplitBuffer, ZBuf},
    prelude::ZenohId,
};

use crate::{impl_guarded_transmute, GuardedTransmute};

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

#[repr(C)]
#[derive(Clone, Debug)]
pub struct z_owned_bytes_t {
    pub start: *mut u8,
    pub len: size_t,
}

impl Drop for z_owned_bytes_t {
    fn drop(&mut self) {
        unsafe { z_bytes_drop(self) }
    }
}

impl z_owned_bytes_t {
    pub fn new(data: &[u8]) -> z_owned_bytes_t {
        if data.is_empty() {
            return z_bytes_null();
        }
        let data = data.to_vec().into_boxed_slice();
        z_owned_bytes_t {
            len: data.len(),
            start: Box::leak(data).as_mut_ptr(),
        }
    }

    pub fn preallocate(len: usize) -> z_owned_bytes_t {
        let data = vec![0u8; len].into_boxed_slice();
        z_owned_bytes_t {
            len,
            start: Box::leak(data).as_mut_ptr(),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn insert_unchecked(&mut self, start: usize, value: &[u8]) {
        std::ptr::copy_nonoverlapping(value.as_ptr(), self.start.add(start), value.len());
    }
}

/// Returns ``true`` if `b` is initialized.
#[no_mangle]
pub extern "C" fn z_bytes_is_initialized(b: &z_bytes_t) -> bool {
    !b.start.is_null()
}

/// Returns the gravestone value for `z_bytes_t`
#[no_mangle]
pub const extern "C" fn z_bytes_empty() -> z_bytes_t {
    z_bytes_t {
        len: 0,
        start: core::ptr::null(),
    }
}

/// Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// `str == NULL` will cause this to return `z_bytes_empty()`
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_from_str(str: *const c_char) -> z_bytes_t {
    if str.is_null() {
        z_bytes_empty()
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
/// `str == NULL` will cause this to return `z_bytes_empty()`
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
        z_bytes_empty()
    } else {
        z_bytes_t { len, start }
    }
}

/// Frees `b` and invalidates it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_drop(b: &mut z_owned_bytes_t) {
    if !b.start.is_null() {
        std::mem::drop(Box::from_raw(
            core::ptr::slice_from_raw_parts(b.start, b.len).cast_mut(),
        ));
        b.start = std::ptr::null_mut();
        b.len = 0;
    }
}

/// Returns the gravestone value for `z_owned_bytes_t`
#[no_mangle]
pub const extern "C" fn z_bytes_null() -> z_owned_bytes_t {
    z_owned_bytes_t {
        len: 0,
        start: core::ptr::null_mut(),
    }
}

#[no_mangle]
pub const extern "C" fn z_bytes_loan(b: &z_owned_bytes_t) -> z_bytes_t {
    z_bytes_t {
        len: b.len,
        start: b.start,
    }
}

#[no_mangle]
pub extern "C" fn z_bytes_clone(b: &z_bytes_t) -> z_owned_bytes_t {
    if !z_bytes_is_initialized(b) {
        z_bytes_null()
    } else {
        z_owned_bytes_t::new(unsafe { std::slice::from_raw_parts(b.start, b.len) })
    }
}

/// Returns ``true`` if `b` is initialized.
#[no_mangle]
pub extern "C" fn z_bytes_check(b: &z_owned_bytes_t) -> bool {
    !b.start.is_null()
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

pub use crate::z_owned_buffer_t;
impl_guarded_transmute!(noderefs Option<ZBuf>, z_owned_buffer_t);
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
#[derive(Clone, Copy, Default)]
pub struct z_buffer_t {
    _inner: Option<NonNull<z_owned_buffer_t>>,
}

impl From<Option<&ZBuf>> for z_buffer_t {
    fn from(value: Option<&ZBuf>) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<z_buffer_t> for Option<&'static ZBuf> {
    fn from(value: z_buffer_t) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

/// Increments the buffer's reference count, returning an owned version of the buffer.
#[no_mangle]
pub extern "C" fn z_buffer_clone(buffer: z_buffer_t) -> z_owned_buffer_t {
    match buffer._inner {
        Some(b) => unsafe { b.as_ref().deref().clone().transmute() },
        None => ZBuf::empty().into(),
    }
}

/// Returns the number of slices in the buffer.
///
/// If the return value is 0 or 1, then the buffer's data is contiguous in memory.
#[no_mangle]
pub extern "C" fn z_buffer_slice_count(buffer: z_buffer_t) -> usize {
    match buffer.into() {
        None => 0,
        Some(buf) => ZBuf::slices(buf).len(),
    }
}

/// Returns total number bytes in the buffer.
#[no_mangle]
pub extern "C" fn z_buffer_len(buffer: z_buffer_t) -> usize {
    match buffer.into() {
        None => 0,
        Some(buf) => ZBuf::slices(buf).fold(0, |acc, s| acc + s.len()),
    }
}

/// Returns the `index`th slice of the buffer, aliasing it.
///
/// Out of bounds accesses will return `z_bytes_empty`.
#[no_mangle]
pub extern "C" fn z_buffer_slice_at(buffer: z_buffer_t, index: usize) -> z_bytes_t {
    match buffer.into() {
        None => z_bytes_empty(),
        Some(buf) => ZBuf::slices(buf)
            .nth(index)
            .map_or(z_bytes_empty(), |slice| slice.into()),
    }
}
