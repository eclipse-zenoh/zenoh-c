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

use prebindgen_proc_macro::prebindgen;

#[prebindgen("types")]
#[allow(non_camel_case_types)]
pub type z_result_t = i8;

#[prebindgen("types")]
pub const Z_CHANNEL_DISCONNECTED: z_result_t = 1;
#[prebindgen("types")]
pub const Z_CHANNEL_NODATA: z_result_t = 2;
#[prebindgen("types")]
pub const Z_OK: z_result_t = 0;
#[prebindgen("types")]
pub const Z_EINVAL: z_result_t = -1;
#[prebindgen("types")]
pub const Z_EPARSE: z_result_t = -2;
#[prebindgen("types")]
pub const Z_EIO: z_result_t = -3;
#[prebindgen("types")]
pub const Z_ENETWORK: z_result_t = -4;
#[prebindgen("types")]
pub const Z_ENULL: z_result_t = -5;
#[prebindgen("types")]
pub const Z_EUNAVAILABLE: z_result_t = -6;
#[prebindgen("types")]
pub const Z_EDESERIALIZE: z_result_t = -7;
#[prebindgen("types")]
pub const Z_ESESSION_CLOSED: z_result_t = -8;
#[prebindgen("types")]
pub const Z_EUTF8: z_result_t = -9;
// negative pthread error codes (due to convention to return negative values on error)
#[prebindgen("types")]
pub const Z_EBUSY_MUTEX: z_result_t = -16;
#[prebindgen("types")]
pub const Z_EINVAL_MUTEX: z_result_t = -22;
#[prebindgen("types")]
pub const Z_EAGAIN_MUTEX: z_result_t = -11;
#[prebindgen("types")]
pub const Z_EPOISON_MUTEX: z_result_t = -22; // same as Z_EINVAL_MUTEX
#[prebindgen("types")]
pub const Z_EGENERIC: z_result_t = i8::MIN;
