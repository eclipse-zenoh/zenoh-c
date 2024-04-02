//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use zenoh::shm::provider::zsliceshm::ZSliceShm;

use crate::{decl_rust_copy_type, impl_guarded_transmute, GuardedTransmute};

// A ZSliceSHM
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_slice_shm_t([u64; 10]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_slice_shm_t([u64; 10]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_slice_shm_t([u64; 10]);

decl_rust_copy_type!(
    zenoh:(ZSliceShm),
    c:(z_slice_shm_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_shm_delete(slice: z_slice_shm_t) {
    let _ = slice.transmute();
}
