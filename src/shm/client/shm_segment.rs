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
    internal::zerror,
    shm::{ChunkID, ShmSegment},
    Result,
};

use crate::{
    context::{zc_threadsafe_context_t, DroppableContext, ThreadsafeContext},
    shm::common::types::z_chunk_id_t,
};

/// A callbacks for ShmSegment
#[derive(Debug)]
#[repr(C)]
pub struct zc_shm_segment_callbacks_t {
    map_fn: unsafe extern "C" fn(chunk_id: z_chunk_id_t, context: *mut c_void) -> *mut u8,
}

/// A ShmSegment
#[derive(Debug)]
#[repr(C)]
pub struct z_shm_segment_t {
    context: zc_threadsafe_context_t,
    callbacks: zc_shm_segment_callbacks_t,
}

#[derive(Debug)]
pub struct DynamicShmSegment {
    context: ThreadsafeContext,
    callbacks: zc_shm_segment_callbacks_t,
}

impl DynamicShmSegment {
    pub fn new(data: z_shm_segment_t) -> Self {
        Self {
            context: data.context.into(),
            callbacks: data.callbacks,
        }
    }
}

impl ShmSegment for DynamicShmSegment {
    fn map(&self, chunk: ChunkID) -> Result<AtomicPtr<u8>> {
        unsafe {
            let cb_result = (self.callbacks.map_fn)(chunk, self.context.get());
            cb_result
                .as_mut()
                .map(|p| AtomicPtr::new(p))
                .ok_or_else(|| zerror!("C callback returned null pointer!").into())
        }
    }
}
