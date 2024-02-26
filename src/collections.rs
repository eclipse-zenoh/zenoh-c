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

/// A buffer owned by Zenoh.
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

#[no_mangle]
pub extern "C" fn z_buffer_null() -> z_owned_buffer_t {
    unsafe { core::mem::transmute(None::<ZBuf>) }
}
#[no_mangle]
pub extern "C" fn z_buffer_drop(buffer: &mut z_owned_buffer_t) {
    core::mem::drop(buffer.take())
}

#[no_mangle]
pub extern "C" fn z_buffer_check(buffer: &z_owned_buffer_t) -> bool {
    buffer.is_some()
}
#[no_mangle]
pub extern "C" fn z_buffer_loan(buffer: &z_owned_buffer_t) -> z_buffer_t {
    buffer.as_ref().into()
}

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

#[no_mangle]
pub extern "C" fn z_buffer_clone(buffer: z_buffer_t) -> z_owned_buffer_t {
    unsafe { Some(core::mem::transmute::<_, &ZBuf>(buffer).clone()).into() }
}

#[no_mangle]
pub extern "C" fn z_buffer_payload(buffer: z_buffer_t) -> z_bytes_t {
    let Some(buffer): Option<&ZBuf> = buffer.into() else {
        return z_bytes_null();
    };
    match buffer.contiguous() {
        std::borrow::Cow::Borrowed(buffer) => buffer.into(),
        std::borrow::Cow::Owned(_) => {
            log::error!("A non-contiguous buffer reached user code, this is definitely a bug, please inform us at https://github.com/eclipse-zenoh/zenoh-c/issues/new");
            z_bytes_null()
        }
    }
}
