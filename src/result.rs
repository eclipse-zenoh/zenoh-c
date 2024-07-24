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

#[allow(non_camel_case_types)]
pub type z_result_t = i8;

pub const Z_OK: z_result_t = 0;
pub const Z_EINVAL: z_result_t = -1;
pub const Z_EPARSE: z_result_t = -2;
pub const Z_EIO: z_result_t = -3;
pub const Z_ENETWORK: z_result_t = -4;
pub const Z_ENULL: z_result_t = -5;
pub const Z_EUNAVAILABLE: z_result_t = -6;
// negative pthread error codes (due to convention to return negative values on error)
pub const Z_EBUSY_MUTEX: z_result_t = -16;
pub const Z_EINVAL_MUTEX: z_result_t = -22;
pub const Z_EAGAIN_MUTEX: z_result_t = -11;
pub const Z_EPOISON_MUTEX: z_result_t = -22; // same as Z_EINVAL_MUTEX
pub const Z_EGENERIC: z_result_t = i8::MIN;
