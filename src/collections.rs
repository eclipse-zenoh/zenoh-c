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

use std::borrow::Cow;
use std::collections::HashMap;
use std::mem::MaybeUninit;

use libc::{c_char, c_void, size_t};
use zenoh::prelude::ZenohId;

use crate::errors;
use crate::transmute::{
    unwrap_ref_unchecked, Inplace, InplaceDefault, TransmuteFromHandle, TransmuteIntoHandle,
    TransmuteRef, TransmuteUninitPtr,
};

/// A contiguous view of bytes owned by some other entity.
///
/// `start` being `null` is considered a gravestone value,
/// and empty slices are represented using a possibly dangling pointer for `start`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct z_slice_t {
    pub start: *const u8,
    pub len: size_t,
}

impl z_slice_t {
    pub fn as_slice(&self) -> Option<&'static [u8]> {
        if self.start.is_null() {
            return None;
        }
        Some(unsafe { core::slice::from_raw_parts(self.start, self.len) })
    }
    pub fn empty() -> Self {
        z_slice_t {
            start: std::ptr::null(),
            len: 0,
        }
    }
}

impl Default for z_slice_t {
    fn default() -> Self {
        Self::empty()
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct z_owned_slice_t {
    pub start: *mut u8,
    pub len: size_t,
}

impl Drop for z_owned_slice_t {
    fn drop(&mut self) {
        unsafe { z_slice_drop(self) }
    }
}

impl z_owned_slice_t {
    pub fn new(data: &[u8]) -> z_owned_slice_t {
        if data.is_empty() {
            return z_slice_null();
        }
        let data = data.to_vec().into_boxed_slice();
        z_owned_slice_t {
            len: data.len(),
            start: Box::leak(data).as_mut_ptr(),
        }
    }

    pub fn preallocate(len: usize) -> z_owned_slice_t {
        let data = vec![0u8; len].into_boxed_slice();
        z_owned_slice_t {
            len,
            start: Box::leak(data).as_mut_ptr(),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn insert_unchecked(&mut self, start: usize, value: &[u8]) {
        std::ptr::copy_nonoverlapping(value.as_ptr(), self.start.add(start), value.len());
    }
}

impl Default for z_owned_slice_t {
    fn default() -> Self {
        z_slice_null()
    }
}

/// Returns ``true`` if `b` is initialized.
#[no_mangle]
pub extern "C" fn z_slice_is_initialized(b: &z_slice_t) -> bool {
    !b.start.is_null()
}

/// Returns the gravestone value for `z_slice_t`
#[no_mangle]
pub const extern "C" fn z_slice_empty() -> z_slice_t {
    z_slice_t {
        len: 0,
        start: core::ptr::null(),
    }
}

/// Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// `str == NULL` will cause this to return `z_slice_empty()`
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_from_str(str: *const c_char) -> z_slice_t {
    if str.is_null() {
        z_slice_empty()
    } else {
        let len = unsafe { libc::strlen(str) };
        z_slice_t {
            len,
            start: str.cast(),
        }
    }
}

#[deprecated = "Renamed to z_slice_from_str"]
/// Deprecated in favor of `z_slice_from_str`: Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
///
/// `str == NULL` will cause this to return `z_slice_empty()`
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_new(str: *const c_char) -> z_slice_t {
    z_slice_from_str(str)
}

/// Constructs a `len` bytes long view starting at `start`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_wrap(start: *const u8, len: usize) -> z_slice_t {
    if start.is_null() {
        z_slice_empty()
    } else {
        z_slice_t { len, start }
    }
}

/// Frees `b` and invalidates it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_drop(b: &mut z_owned_slice_t) {
    if !b.start.is_null() {
        std::mem::drop(Box::from_raw(
            core::ptr::slice_from_raw_parts(b.start, b.len).cast_mut(),
        ));
        b.start = std::ptr::null_mut();
        b.len = 0;
    }
}

/// Returns the gravestone value for `z_owned_slice_t`
#[no_mangle]
pub const extern "C" fn z_slice_null() -> z_owned_slice_t {
    z_owned_slice_t {
        len: 0,
        start: core::ptr::null_mut(),
    }
}

#[no_mangle]
pub const extern "C" fn z_slice_loan(b: &z_owned_slice_t) -> z_slice_t {
    z_slice_t {
        len: b.len,
        start: b.start,
    }
}

#[no_mangle]
pub extern "C" fn z_slice_clone(b: &z_slice_t) -> z_owned_slice_t {
    if !z_slice_is_initialized(b) {
        z_slice_null()
    } else {
        z_owned_slice_t::new(unsafe { std::slice::from_raw_parts(b.start, b.len) })
    }
}

/// Returns ``true`` if `b` is initialized.
#[no_mangle]
pub extern "C" fn z_slice_check(b: &z_owned_slice_t) -> bool {
    !b.start.is_null()
}

impl From<ZenohId> for z_slice_t {
    #[inline]
    fn from(pid: ZenohId) -> Self {
        let pid = pid.to_le_bytes().to_vec().into_boxed_slice();
        let res = z_slice_t {
            start: pid.as_ptr(),
            len: pid.len() as size_t,
        };
        std::mem::forget(pid);
        res
    }
}

impl From<Option<ZenohId>> for z_slice_t {
    #[inline]
    fn from(pid: Option<ZenohId>) -> Self {
        match pid {
            Some(pid) => pid.into(),
            None => z_slice_t {
                start: std::ptr::null(),
                len: 0,
            },
        }
    }
}

impl From<z_slice_t> for String {
    fn from(s: z_slice_t) -> Self {
        unsafe {
            String::from_utf8(
                Box::from_raw(std::slice::from_raw_parts_mut(s.start as *mut u8, s.len)).into(),
            )
            .unwrap()
        }
    }
}

impl From<&[u8]> for z_slice_t {
    fn from(s: &[u8]) -> Self {
        z_slice_t {
            start: s.as_ptr(),
            len: s.len(),
        }
    }
}

impl InplaceDefault for z_owned_slice_t {}

/// The wrapper type for null-terminated string values allocated by zenoh. The instances of `z_owned_str_t`
/// should be released with `z_drop` macro or with `z_str_drop` function and checked to validity with
/// `z_check` and `z_str_check` correspondently
#[repr(C)]
pub struct z_owned_str_t {
    pub _cstr: *mut libc::c_char,
}

impl z_owned_str_t {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn preallocate(len: usize) -> z_owned_str_t {
        let cstr = libc::malloc(len + 1) as *mut libc::c_char;
        *cstr.add(len) = 0;
        z_owned_str_t { _cstr: cstr }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn insert_unchecked(&mut self, start: usize, value: &[u8]) {
        std::ptr::copy_nonoverlapping(
            value.as_ptr(),
            (self._cstr as *mut u8).add(start),
            value.len(),
        );
    }
}

impl From<&[u8]> for z_owned_str_t {
    fn from(value: &[u8]) -> Self {
        unsafe {
            let mut cstr = Self::preallocate(value.len());
            cstr.insert_unchecked(0, value);
            cstr
        }
    }
}

impl Drop for z_owned_str_t {
    fn drop(&mut self) {
        unsafe { z_str_drop(self) }
    }
}

impl Default for z_owned_str_t {
    fn default() -> Self {
        z_str_null()
    }
}

/// Frees `z_owned_str_t`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_str_drop(s: &mut z_owned_str_t) {
    if s._cstr.is_null() {
        return;
    }
    libc::free(std::mem::transmute(s._cstr));
    s._cstr = std::ptr::null_mut();
}

/// Returns ``true`` if `s` is a valid string
#[no_mangle]
pub extern "C" fn z_str_check(s: &z_owned_str_t) -> bool {
    !s._cstr.is_null()
}

/// Returns undefined `z_owned_str_t`
#[no_mangle]
pub extern "C" fn z_str_null() -> z_owned_str_t {
    z_owned_str_t {
        _cstr: std::ptr::null_mut(),
    }
}

/// Returns :c:type:`z_str_t` structure loaned from :c:type:`z_owned_str_t`.
#[no_mangle]
pub extern "C" fn z_str_loan(s: &z_owned_str_t) -> *const libc::c_char {
    s._cstr
}

impl InplaceDefault for z_owned_str_t {}

pub use crate::opaque_types::z_owned_slice_map_t;
pub use crate::opaque_types::z_slice_map_t;

pub type ZHashMap = HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>;
pub use crate::opaque_types::z_config_t;
decl_transmute_handle!(
    HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>,
    z_slice_map_t
);

pub use crate::opaque_types::z_owned_config_t;
decl_transmute_owned!(
    Option<HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>>,
    z_owned_slice_map_t
);

/// Constructs a new empty map.
#[no_mangle]
pub extern "C" fn z_slice_map_new(this: *mut MaybeUninit<z_owned_slice_map_t>) {
    let this = this.transmute_uninit_ptr();
    let map = ZHashMap::new();
    Inplace::init(this, Some(map));
}

/// Constructs the gravestone value for `z_owned_slice_map_t`
#[no_mangle]
pub extern "C" fn z_slice_map_null(this: *mut MaybeUninit<z_owned_slice_map_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Returns `true` if the map is not in its gravestone state
#[no_mangle]
pub extern "C" fn z_slice_map_check(map: &z_owned_slice_map_t) -> bool {
    let map = map.transmute_ref();
    map.as_ref().is_some()
}

/// Destroys the map, resetting `this` to its gravestone value.
///
/// This function is double-free safe, passing a pointer to the gravestone value will have no effect.
#[no_mangle]
pub extern "C" fn z_slice_map_drop(this: &mut z_owned_slice_map_t) {
    let this = this.transmute_mut();
    Inplace::drop(this);
}

#[no_mangle]
pub extern "C" fn z_slice_map_loan(this: &z_owned_slice_map_t) -> z_slice_map_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

/// Returns number of key-value pairs in the map.
#[no_mangle]
pub extern "C" fn z_slice_map_len(this: z_slice_map_t) -> usize {
    this.transmute_ref().len()
}

/// Returns true if the map is empty, false otherwise.
#[no_mangle]
pub extern "C" fn z_slice_map_is_empty(this: z_slice_map_t) -> bool {
    z_slice_map_len(this) == 0
}

/// The body of a loop over a z_slice_map's key-value pairs.
///
/// `key` and `value` are loaned to the body for the duration of a single call.
/// `context` is passed transparently through the iteration driver.
///
/// Returning `true` is treated as `continue`.
#[allow(non_camel_case_types)]
pub type z_slice_map_iter_body_t =
    extern "C" fn(key: z_slice_t, value: z_slice_t, context: *mut c_void) -> bool;

#[no_mangle]
pub extern "C" fn z_slice_map_iterate(
    this: &z_slice_map_t,
    body: z_slice_map_iter_body_t,
    context: *mut c_void,
) {
    let this = this.transmute_ref();
    for (key, value) in this {
        if !body(key.as_ref().into(), value.as_ref().into(), context) {
            break;
        }
    }
}

/// Returns the value associated with `key`, returning a gravestone value if:
/// - `key` is in gravestone state.
#[no_mangle]
pub extern "C" fn z_slice_map_get(this: z_slice_map_t, key: z_slice_t) -> z_slice_t {
    if !z_slice_is_initialized(&key) {
        return z_slice_empty();
    }
    let m = this.transmute_mut();
    let key = key.as_slice().unwrap();
    m.get(key)
        .map(|s| s.as_ref().into())
        .unwrap_or(z_slice_empty())
}

/// Associates `value` to `key` in the map, copying them to obtain ownership: `key` and `value` are not aliased past the function's return.
///
/// Returns 0 in case of success, -1 if one of the arguments were in gravestone state.
#[no_mangle]
pub extern "C" fn z_slice_map_insert_by_copy(
    this: z_slice_map_t,
    key: z_slice_t,
    value: z_slice_t,
) -> errors::z_error_t {
    let this = this.transmute_mut();
    if let (Some(key), Some(value)) = (key.as_slice(), value.as_slice()) {
        this.insert(Cow::Owned(key.to_owned()), Cow::Owned(value.to_owned()));
        errors::Z_OK
    } else {
        errors::Z_EINVAL
    }
}

/// Associates `value` to `key` in the map, aliasing them.
///
/// Note that once `key` is aliased, reinserting at the same key may alias the previous instance, or the new instance of `key`.
///
/// Returns 0 in case of success, -1 if one of the arguments were in gravestone state.
#[no_mangle]
pub extern "C" fn z_slice_map_insert_by_alias(
    this: z_slice_map_t,
    key: z_slice_t,
    value: z_slice_t,
) -> errors::z_error_t {
    let this = this.transmute_mut();
    if let (Some(key), Some(value)) = (key.as_slice(), value.as_slice()) {
        this.insert(Cow::Borrowed(key), Cow::Borrowed(value));
        errors::Z_OK
    } else {
        errors::Z_EINVAL
    }
}
