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
use zenoh::shm::provider::{
    chunk::ChunkDescriptor,
    shared_memory_provider_backend::SharedMemoryProviderBackend,
    types::{ChunkAllocResult, MemoryLayout},
};
use zenoh::Result;
use zenoh_util::core::bail;

use crate::{decl_rust_copy_type, impl_guarded_transmute, GuardedTransmute};

use super::{
    chunk::z_chunk_descriptor_t,
    types::{z_chunk_alloc_result_t, z_memory_layout_t},
};

/// A droppable context
#[derive(Debug)]
#[repr(C)]
pub struct zc_context_t {
    context: *mut c_void,
    drop_fn: unsafe extern "C" fn(*mut c_void),
}

decl_rust_copy_type!(
    zenoh:(Context),
    c:(zc_context_t)
);

#[derive(Debug)]
pub struct Context(zc_context_t);

impl Context {
    pub fn get(&self) -> *mut c_void {
        self.0.context
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            (self.0.drop_fn)(self.0.context);
        }
    }
}

/// A callbacks for SharedMemoryProviderBackend
#[derive(Debug)]
#[repr(C)]
pub struct zc_shared_memory_provider_backend_callbacks_t {
    alloc_fn:
        unsafe extern "C" fn(*mut c_void, *const z_memory_layout_t, *mut z_chunk_alloc_result_t),
    free_fn: unsafe extern "C" fn(*mut c_void, *const z_chunk_descriptor_t),
    defragment_fn: unsafe extern "C" fn(*mut c_void) -> usize,
    available_fn: unsafe extern "C" fn(*mut c_void) -> usize,
    layout_for_fn:
        unsafe extern "C" fn(*mut c_void, *const z_memory_layout_t, *mut z_memory_layout_t) -> bool,
    drop_fn: unsafe extern "C" fn(*mut c_void),
}

#[derive(Debug)]
pub struct DynamicSharedMemoryProviderBackend {
    context: Context,
    callbacks: zc_shared_memory_provider_backend_callbacks_t,
}

impl DynamicSharedMemoryProviderBackend {
    pub fn new(context: Context, callbacks: zc_shared_memory_provider_backend_callbacks_t) -> Self {
        Self { context, callbacks }
    }
}

impl SharedMemoryProviderBackend for DynamicSharedMemoryProviderBackend {
    fn alloc(&self, layout: &MemoryLayout) -> ChunkAllocResult {
        let mut result = std::mem::MaybeUninit::uninit();
        unsafe {
            (self.callbacks.alloc_fn)(
                self.context.get(),
                layout.transmute_ref(),
                result.as_mut_ptr(),
            );
            result.assume_init().transmute()
        }
    }

    fn free(&self, chunk: &ChunkDescriptor) {
        unsafe { (self.callbacks.free_fn)(self.context.get(), chunk.transmute_ref()) };
    }

    fn defragment(&self) -> usize {
        unsafe { (self.callbacks.defragment_fn)(self.context.get()) }
    }

    fn available(&self) -> usize {
        unsafe { (self.callbacks.available_fn)(self.context.get()) }
    }

    fn layout_for(&self, layout: MemoryLayout) -> Result<MemoryLayout> {
        let mut result = std::mem::MaybeUninit::uninit();
        unsafe {
            match (self.callbacks.layout_for_fn)(
                self.context.get(),
                layout.transmute_ref(),
                result.as_mut_ptr(),
            ) {
                true => Ok(result.assume_init().transmute()),
                false => bail!("{:?}: unsupported layout: {:?}", self, layout),
            }
        }
    }
}
