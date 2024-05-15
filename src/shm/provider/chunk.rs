use std::sync::atomic::AtomicPtr;

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
use libc::c_void;
use zenoh::shm::{AllocatedChunk, ChunkDescriptor};

use crate::shm::common::types::{z_chunk_id_t, z_segment_id_t};

/// A ChunkDescriptor
#[repr(C)]
pub struct z_chunk_descriptor_t {
    segment: z_segment_id_t,
    chunk: z_chunk_id_t,
    len: usize,
}

impl From<z_chunk_descriptor_t> for ChunkDescriptor {
    fn from(value: z_chunk_descriptor_t) -> Self {
        Self::new(value.segment, value.chunk, value.len)
    }
}

impl From<&ChunkDescriptor> for z_chunk_descriptor_t {
    fn from(value: &ChunkDescriptor) -> Self {
        Self {
            segment: value.segment,
            chunk: value.chunk,
            len: value.len,
        }
    }
}

/// An AllocatedChunk
#[repr(C)]
pub struct z_allocated_chunk_t {
    descriptpr: z_chunk_descriptor_t,
    data: *mut c_void,
}

impl From<z_allocated_chunk_t> for AllocatedChunk {
    fn from(value: z_allocated_chunk_t) -> Self {
        Self::new(
            value.descriptpr.into(),
            AtomicPtr::from(value.data as *mut u8),
        )
    }
}
