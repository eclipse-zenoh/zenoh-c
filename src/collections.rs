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
use libc::size_t;
use zenoh::{buf::ZBuf, net::protocol::io::SplitBuffer, prelude::ZenohId};

/// A zenoh-allocated array of bytes.   
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
pub struct z_owned_bytes_t {
    pub start: *const u8,
    pub len: size_t,
}

impl z_owned_bytes_t {
    pub fn empty() -> Self {
        z_owned_bytes_t {
            start: std::ptr::null(),
            len: 0,
        }
    }
}

impl Default for z_owned_bytes_t {
    fn default() -> Self {
        Self::empty()
    }
}

/// A loaned array of bytes.  
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_bytes_t {
    pub start: *const u8,
    pub len: size_t,
}

impl z_bytes_t {
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

/// Constructs a :c:type:`z_bytes_t` of length `len` from the bytes
/// starting at address `start`.
/// The bytes from `start` are NOT copied.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes(start: *const u8, len: usize) -> z_bytes_t {
    z_bytes_t { start, len }
}

/// Constructs a :c:type:`z_owned_bytes_t` of length `len` from the bytes
/// starting at address `start`.
/// The bytes from `start` are copied.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_new(start: *const u8, len: usize) -> z_owned_bytes_t {
    let slice = std::slice::from_raw_parts(start, len);
    let boxed = Box::<[u8]>::from(slice);
    let start = Box::into_raw(boxed);
    z_owned_bytes_t {
        start: (*start).as_ptr(),
        len,
    }
}

/// Constructs a :c:type:`z_owned_bytes_t` of length `len` from the loaned z_bytes_t
/// starting at address `start`.
/// The bytes from `start` are copied.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_clone(bytes: z_bytes_t) -> z_owned_bytes_t {
    z_bytes_new(bytes.start, bytes.len)
}

/// Frees `b` and invalidates it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_free(b: &mut z_owned_bytes_t) {
    if !b.start.is_null() {
        std::mem::drop(Box::from_raw(
            std::slice::from_raw_parts(b.start, b.len) as *const [u8] as *mut [u8],
        ));
        b.start = std::ptr::null_mut();
    }
}

/// Returns `true` if `b` is valid.
#[no_mangle]
pub extern "C" fn z_bytes_check(b: &z_owned_bytes_t) -> bool {
    !b.start.is_null()
}

#[no_mangle]
pub extern "C" fn z_bytes_loan(b: &z_owned_bytes_t) -> z_bytes_t {
    z_bytes_t {
        start: b.start,
        len: b.len,
    }
}

impl From<ZenohId> for z_owned_bytes_t {
    #[inline]
    fn from(pid: ZenohId) -> Self {
        let pid = pid.as_slice().to_vec().into_boxed_slice();
        let res = z_owned_bytes_t {
            start: pid.as_ptr(),
            len: pid.len() as size_t,
        };
        std::mem::forget(pid);
        res
    }
}

impl From<Option<ZenohId>> for z_owned_bytes_t {
    #[inline]
    fn from(pid: Option<ZenohId>) -> Self {
        match pid {
            Some(pid) => pid.into(),
            None => z_owned_bytes_t {
                start: std::ptr::null(),
                len: 0,
            },
        }
    }
}

impl From<ZBuf> for z_owned_bytes_t {
    fn from(buf: ZBuf) -> Self {
        let data = buf.contiguous().into_owned().into_boxed_slice();
        let res = z_owned_bytes_t {
            start: data.as_ptr(),
            len: data.len(),
        };
        std::mem::forget(data);
        res
    }
}

impl From<z_owned_bytes_t> for String {
    fn from(s: z_owned_bytes_t) -> Self {
        unsafe {
            String::from_utf8(
                Box::from_raw(std::slice::from_raw_parts_mut(s.start as *mut u8, s.len)).into(),
            )
            .unwrap()
        }
    }
}
