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

use std::sync::atomic::AtomicPtr;

use libc::c_void;
use zenoh::{
    shm::{client::shared_memory_segment::SharedMemorySegment, common::types::ChunkID},
    Result,
};
use zenoh_util::core::zerror;

use crate::{
    common::types::z_chunk_id_t, zc_threadsafe_context_t, DroppableContext, GuardedTransmute,
    ThreadsafeContext,
};

/// A callbacks for SharedMemorySegment
#[derive(Debug)]
#[repr(C)]
pub struct zc_shared_memory_segment_callbacks_t {
    map_fn: unsafe extern "C" fn(*mut c_void, z_chunk_id_t) -> *mut u8,
}

/// A SharedMemorySegment
#[derive(Debug)]
#[repr(C)]
pub struct z_shared_memory_segment_t {
    context: zc_threadsafe_context_t,
    callbacks: zc_shared_memory_segment_callbacks_t,
}

#[derive(Debug)]
pub struct DynamicSharedMemorySegment {
    context: ThreadsafeContext,
    callbacks: zc_shared_memory_segment_callbacks_t,
}

impl DynamicSharedMemorySegment {
    pub fn new(data: z_shared_memory_segment_t) -> Self {
        Self {
            context: data.context.transmute(),
            callbacks: data.callbacks,
        }
    }
}

impl SharedMemorySegment for DynamicSharedMemorySegment {
    fn map(&self, chunk: ChunkID) -> Result<AtomicPtr<u8>> {
        unsafe {
            let cb_result = (self.callbacks.map_fn)(self.context.get(), chunk);
            cb_result
                .as_mut()
                .map(|p| AtomicPtr::new(p))
                .ok_or_else(|| zerror!("C callback returned null pointer!").into())
        }
    }
}
