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

use std::mem::MaybeUninit;

use zenoh::shm::provider::chunk::{AllocatedChunk, ChunkDescriptor};

use crate::{
    common::types::{z_chunk_id_t, z_segment_id_t},
    decl_rust_copy_type, impl_guarded_transmute, GuardedTransmute,
};

#[repr(C)]
pub struct zc_chunk_descriptor_data_t {
    segment: z_segment_id_t,
    chunk: z_chunk_id_t,
    len: usize,
}

/// A ChunkDescriptor
#[repr(C)]
pub struct z_chunk_descriptor_t(zc_chunk_descriptor_data_t);

decl_rust_copy_type!(
    zenoh:(ChunkDescriptor),
    c:(z_chunk_descriptor_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_chunk_descriptor_new(
    data: zc_chunk_descriptor_data_t,
    out_val: &mut MaybeUninit<z_chunk_descriptor_t>,
) {
    let descriptor = ChunkDescriptor::new(data.segment, data.chunk, data.len);
    out_val.write(descriptor.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_chunk_descriptor_unwrap(
    descriptor: z_chunk_descriptor_t,
) -> zc_chunk_descriptor_data_t {
    let descriptor = descriptor.transmute();
    zc_chunk_descriptor_data_t {
        segment: descriptor.segment,
        chunk: descriptor.chunk,
        len: descriptor.len,
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_chunk_descriptor_delete(val: z_chunk_descriptor_t) {
    let _ = val.transmute();
}

/// An AllocatedChunk.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_allocated_chunk_t([u64; 3]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_allocated_chunk_t([u64; 4]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_allocated_chunk_t([u64; 3]);

decl_rust_copy_type!(
    zenoh:(AllocatedChunk),
    c:(z_allocated_chunk_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_allocated_chunk_new(
    descriptor: z_chunk_descriptor_t,
    data: *mut u8,
    out_val: &mut MaybeUninit<z_allocated_chunk_t>,
) {
    let chunk = AllocatedChunk::new(descriptor.transmute(), data.into());
    out_val.write(chunk.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_allocated_chunk_delete(val: z_allocated_chunk_t) {
    let _ = val.transmute();
}
