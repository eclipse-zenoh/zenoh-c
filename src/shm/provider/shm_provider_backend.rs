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
use zenoh::shm::{
    ChunkAllocResult, ChunkDescriptor, MemoryLayout, ShmProviderBackend, ZLayoutError,
};

use super::chunk::z_chunk_descriptor_t;
use crate::{
    context::DroppableContext,
    transmute::{LoanedCTypeRef, OwnedCTypeRef, RustTypeRef},
    z_loaned_memory_layout_t, z_owned_chunk_alloc_result_t, z_owned_memory_layout_t,
};

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Callbacks for ShmProviderBackend.
#[derive(Debug)]
#[repr(C)]
pub struct zc_shm_provider_backend_callbacks_t {
    alloc_fn: unsafe extern "C" fn(
        out_result: *mut z_owned_chunk_alloc_result_t,
        layout: &z_loaned_memory_layout_t,
        context: *mut c_void,
    ),
    free_fn: unsafe extern "C" fn(chunk: *const z_chunk_descriptor_t, context: *mut c_void),
    defragment_fn: unsafe extern "C" fn(context: *mut c_void) -> usize,
    available_fn: unsafe extern "C" fn(context: *mut c_void) -> usize,
    layout_for_fn: unsafe extern "C" fn(layout: *mut z_owned_memory_layout_t, context: *mut c_void),
}

#[derive(Debug)]
pub struct DynamicShmProviderBackend<TContext>
where
    TContext: DroppableContext,
{
    context: TContext,
    callbacks: zc_shm_provider_backend_callbacks_t,
}

impl<TContext> DynamicShmProviderBackend<TContext>
where
    TContext: DroppableContext,
{
    pub fn new(context: TContext, callbacks: zc_shm_provider_backend_callbacks_t) -> Self {
        Self { context, callbacks }
    }
}

impl<TContext> ShmProviderBackend for DynamicShmProviderBackend<TContext>
where
    TContext: DroppableContext,
{
    fn alloc(&self, layout: &MemoryLayout) -> ChunkAllocResult {
        let mut result = std::mem::MaybeUninit::uninit();
        unsafe {
            (self.callbacks.alloc_fn)(
                result.as_mut_ptr(),
                layout.as_loaned_c_type_ref(),
                self.context.get(),
            );
            match result.assume_init().as_rust_type_mut().take() {
                Some(val) => val,
                None => Err(zenoh::shm::ZAllocError::Other),
            }
        }
    }

    fn free(&self, chunk: &ChunkDescriptor) {
        unsafe { (self.callbacks.free_fn)(&chunk.into(), self.context.get()) };
    }

    fn defragment(&self) -> usize {
        unsafe { (self.callbacks.defragment_fn)(self.context.get()) }
    }

    fn available(&self) -> usize {
        unsafe { (self.callbacks.available_fn)(self.context.get()) }
    }

    fn layout_for(&self, layout: MemoryLayout) -> Result<MemoryLayout, ZLayoutError> {
        let mut layout = Some(layout);
        unsafe {
            (self.callbacks.layout_for_fn)(layout.as_owned_c_type_mut(), self.context.get());
        }
        layout.ok_or(ZLayoutError::ProviderIncompatibleLayout)
    }
}
