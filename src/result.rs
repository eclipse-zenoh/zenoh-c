//
// Copyright (c) 2017, 2024 ZettaScale Technology.
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

#[cfg(feature = "unstable")]
use std::{cell::RefCell, mem::MaybeUninit, str};

#[cfg(feature = "unstable")]
use crate::{z_view_string_from_substr, z_view_string_t};

use prebindgen_proc_macro::prebindgen;

#[prebindgen]
#[allow(non_camel_case_types)]
pub type z_result_t = i8;

#[prebindgen]
pub const Z_CHANNEL_DISCONNECTED: z_result_t = 1;
#[prebindgen]
pub const Z_CHANNEL_NODATA: z_result_t = 2;
#[prebindgen]
pub const Z_OK: z_result_t = 0;
#[prebindgen]
pub const Z_EINVAL: z_result_t = -1;
#[prebindgen]
pub const Z_EPARSE: z_result_t = -2;
#[prebindgen]
pub const Z_EIO: z_result_t = -3;
#[prebindgen]
pub const Z_ENETWORK: z_result_t = -4;
#[prebindgen]
pub const Z_ENULL: z_result_t = -5;
#[prebindgen]
pub const Z_EUNAVAILABLE: z_result_t = -6;
#[prebindgen]
pub const Z_EDESERIALIZE: z_result_t = -7;
#[prebindgen]
pub const Z_ESESSION_CLOSED: z_result_t = -8;
#[prebindgen]
pub const Z_EUTF8: z_result_t = -9;
// negative pthread error codes (due to convention to return negative values on error)
#[prebindgen]
pub const Z_EBUSY_MUTEX: z_result_t = -16;
#[prebindgen]
pub const Z_EINVAL_MUTEX: z_result_t = -22;
#[prebindgen]
pub const Z_EAGAIN_MUTEX: z_result_t = -11;
#[prebindgen]
pub const Z_EPOISON_MUTEX: z_result_t = -22; // same as Z_EINVAL_MUTEX
#[prebindgen]
pub const Z_EGENERIC: z_result_t = i8::MIN;

#[cfg(feature = "unstable")]
pub struct Buffer<const N: usize> {
    buffer: [u8; N],
    len: usize,
}

#[cfg(feature = "unstable")]
impl<const N: usize> Default for Buffer<N> {
    fn default() -> Self {
        Self {
            buffer: [0; N],
            len: Default::default(),
        }
    }
}

#[cfg(feature = "unstable")]
impl<const N: usize> Buffer<N> {
    pub fn update(&mut self, error: &str) {
        self.len = error.len().min(N);
        (self.buffer[0..self.len]).copy_from_slice(&error.as_bytes()[0..self.len]);
    }
}

#[cfg(feature = "unstable")]
thread_local! {
    pub static ERROR_DESCRIPTION: RefCell<Buffer<1024>> = RefCell::default();
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs a view string on last error message.
/// The view string only remains valid until next faillable zenoh API call from the same thread.
#[no_mangle]
extern "C" fn zc_get_last_error(out: &mut MaybeUninit<z_view_string_t>) {
    ERROR_DESCRIPTION.with_borrow(|b| unsafe {
        z_view_string_from_substr(out, b.buffer.as_ptr() as *const libc::c_char, b.len)
    });
}

#[macro_export]
macro_rules! report_error{
    ($($t: tt)*) => {
        {
            tracing::error!($($t)*);
            #[cfg(feature = "unstable")]
            $crate::result::ERROR_DESCRIPTION.with_borrow_mut(|b| b.update(&format!($($t)*)));
        }
    };
}
