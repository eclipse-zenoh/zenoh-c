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
use zenoh::{
    shm::{ChunkID, ShmSegment},
    Result,
};

use crate::{
    context::{zc_threadsafe_context_t, DroppableContext, ThreadsafeContext},
    shm::common::types::z_chunk_id_t,
};

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Callbacks for ShmSegment.
#[prebindgen]
#[derive(Debug)]
#[repr(C)]
pub struct zc_shm_segment_callbacks_t {
    /// Obtain the actual region of memory identified by it's id.
    map_fn: unsafe extern "C" fn(chunk_id: z_chunk_id_t, context: *mut c_void) -> *mut u8,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An ShmSegment.
#[prebindgen]
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
    fn map(&self, chunk: ChunkID) -> Result<*mut u8> {
        unsafe {
            let cb_result = (self.callbacks.map_fn)(chunk, self.context.get());
            if cb_result.is_null() {
                Err("C callback returned null pointer!".into())
            } else {
                Ok(cb_result)
            }
        }
    }
}
