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
use zenoh::prelude::ZenohId;

/// An array of bytes.  
#[repr(C)]
#[derive(Clone, Copy, Debug)]
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

/// Returns `true` if `b` is initialized.
#[no_mangle]
pub extern "C" fn z_bytes_check(b: &z_bytes_t) -> bool {
    !b.start.is_null()
}

/// Frees `b` and invalidates it for double-free safety.
#[allow(clippy::missing_safety_doc)]
pub(crate) unsafe fn z_bytes_free(b: &mut z_bytes_t) {
    if !b.start.is_null() {
        std::mem::drop(Box::from_raw(
            std::slice::from_raw_parts(b.start, b.len) as *const [u8] as *mut [u8],
        ));
        b.start = std::ptr::null();
    }
}

impl From<ZenohId> for z_bytes_t {
    #[inline]
    fn from(pid: ZenohId) -> Self {
        let pid = pid.as_slice().to_vec().into_boxed_slice();
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

// impl<'a> From<&'a ZBuf> for z_bytes_t {
//     fn from(buf: &'a ZBuf) -> Self {
//         let data = buf.contiguous();
//         let res = z_bytes_t {
//             start: data.as_ptr(),
//             len: data.len(),
//         };
//         std::mem::forget(data);
//         res
//     }
// }

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
