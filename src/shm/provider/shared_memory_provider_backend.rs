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

use std::fmt::Debug;

use libc::c_void;
use zenoh::core::Result;
use zenoh::shm::{ChunkAllocResult, ChunkDescriptor, MemoryLayout, SharedMemoryProviderBackend};
use zenoh_util::core::zerror;

use crate::context::DroppableContext;
use crate::transmute::{TransmuteIntoHandle, TransmuteRef};
use crate::{z_loaned_memory_layout_t, z_owned_chunk_alloc_result_t, z_owned_memory_layout_t};

use super::chunk::z_chunk_descriptor_t;

/// A callbacks for SharedMemoryProviderBackend
#[derive(Debug)]
#[repr(C)]
pub struct zc_shared_memory_provider_backend_callbacks_t {
    alloc_fn: unsafe extern "C" fn(
        *mut c_void,
        &z_loaned_memory_layout_t,
        *mut z_owned_chunk_alloc_result_t,
    ),
    free_fn: unsafe extern "C" fn(*mut c_void, *const z_chunk_descriptor_t),
    defragment_fn: unsafe extern "C" fn(*mut c_void) -> usize,
    available_fn: unsafe extern "C" fn(*mut c_void) -> usize,
    layout_for_fn: unsafe extern "C" fn(*mut c_void, *mut z_owned_memory_layout_t),
    drop_fn: unsafe extern "C" fn(*mut c_void),
}

#[derive(Debug)]
pub struct DynamicSharedMemoryProviderBackend<TContext>
where
    TContext: DroppableContext,
{
    context: TContext,
    callbacks: zc_shared_memory_provider_backend_callbacks_t,
}

impl<TContext> DynamicSharedMemoryProviderBackend<TContext>
where
    TContext: DroppableContext,
{
    pub fn new(
        context: TContext,
        callbacks: zc_shared_memory_provider_backend_callbacks_t,
    ) -> Self {
        Self { context, callbacks }
    }
}

impl<TContext> SharedMemoryProviderBackend for DynamicSharedMemoryProviderBackend<TContext>
where
    TContext: DroppableContext,
{
    fn alloc(&self, layout: &MemoryLayout) -> ChunkAllocResult {
        let mut result = std::mem::MaybeUninit::uninit();
        unsafe {
            (self.callbacks.alloc_fn)(
                self.context.get(),
                layout.transmute_handle(),
                result.as_mut_ptr(),
            );
            match result.assume_init().transmute_mut().take() {
                Some(val) => val,
                None => Err(zenoh::shm::ZAllocError::Other(
                    "Callback returned empty result".into(),
                )),
            }
        }
    }

    fn free(&self, chunk: &ChunkDescriptor) {
        unsafe { (self.callbacks.free_fn)(self.context.get(), &chunk.into()) };
    }

    fn defragment(&self) -> usize {
        unsafe { (self.callbacks.defragment_fn)(self.context.get()) }
    }

    fn available(&self) -> usize {
        unsafe { (self.callbacks.available_fn)(self.context.get()) }
    }

    fn layout_for(&self, layout: MemoryLayout) -> Result<MemoryLayout> {
        let mut layout = Some(layout);
        unsafe {
            (self.callbacks.layout_for_fn)(self.context.get(), layout.transmute_mut());
        }
        layout.ok_or_else(|| zerror!("{:?}: unsupported layout", self).into())
    }
}
