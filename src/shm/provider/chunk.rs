use std::{num::TryFromIntError, sync::atomic::AtomicPtr};

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

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A ChunkDescriptor.
#[repr(C)]
pub struct z_chunk_descriptor_t {
    segment: z_segment_id_t,
    chunk: z_chunk_id_t,
    len: usize,
}

impl TryFrom<z_chunk_descriptor_t> for ChunkDescriptor {
    type Error = TryFromIntError;
    fn try_from(value: z_chunk_descriptor_t) -> Result<Self, Self::Error> {
        Ok(Self::new(value.segment, value.chunk, value.len.try_into()?))
    }
}

impl From<&ChunkDescriptor> for z_chunk_descriptor_t {
    fn from(value: &ChunkDescriptor) -> Self {
        Self {
            segment: value.segment,
            chunk: value.chunk,
            len: value.len.get(),
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An AllocatedChunk.
#[repr(C)]
pub struct z_allocated_chunk_t {
    descriptpr: z_chunk_descriptor_t,
    data: *mut c_void,
}

impl TryFrom<z_allocated_chunk_t> for AllocatedChunk {
    type Error = TryFromIntError;
    fn try_from(value: z_allocated_chunk_t) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.descriptpr.try_into()?,
            AtomicPtr::from(value.data as *mut u8),
        ))
    }
}
