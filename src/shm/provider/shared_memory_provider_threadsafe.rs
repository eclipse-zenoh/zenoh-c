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

use libc::c_void;
use zenoh::shm::provider::shared_memory_provider::{
    AllocPolicy, BlockOn, Deallocate, Defragment, DynamicProtocolID, GarbageCollect, JustAlloc,
    SharedMemoryProvider, SharedMemoryProviderBuilder,
};

use crate::{
    common::types::z_protocol_id_t, decl_rust_copy_type, impl_guarded_transmute,
    zc_threadsafe_context_t, DroppableContext, GuardedTransmute, ThreadsafeContext,
};

use super::{
    alloc_layout_threadsafe::z_alloc_layout_threadsafe_t,
    chunk::z_allocated_chunk_t,
    shared_memory_provider_backend::{
        zc_shared_memory_provider_backend_callbacks_t, DynamicSharedMemoryProviderBackend,
    },
    shared_memory_provider_impl::{alloc, alloc_async},
    types::{z_alloc_alignment_t, z_buf_alloc_result_t},
    zsliceshm::z_slice_shm_t,
};

/// A thread-safe SharedMemoryProvider specialization
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_shared_memory_provider_threadsafe_t([u64; 14]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_shared_memory_provider_threadsafe_t([u64; 14]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_shared_memory_provider_threadsafe_t([u64; 14]);

decl_rust_copy_type!(
    zenoh:(SharedMemoryProvider<DynamicProtocolID, DynamicSharedMemoryProviderBackend<ThreadsafeContext>>),
    c:(z_shared_memory_provider_threadsafe_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_new(
    id: z_protocol_id_t,
    context: zc_threadsafe_context_t,
    callbacks: zc_shared_memory_provider_backend_callbacks_t,
    out_provider: &mut MaybeUninit<z_shared_memory_provider_threadsafe_t>,
) {
    let backend = DynamicSharedMemoryProviderBackend::new(context.transmute(), callbacks);
    let provider = SharedMemoryProviderBuilder::builder()
        .dynamic_protocol_id(id)
        .backend(backend)
        .res();
    out_provider.write(provider.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_delete(
    provider: z_shared_memory_provider_threadsafe_t,
) {
    let _ = provider.transmute();
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_layout(
    provider: &'static z_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_layout: &mut MaybeUninit<z_alloc_layout_threadsafe_t>,
) -> bool {
    let provider = provider.transmute_ref();
    match provider
        .alloc_layout()
        .size(size)
        .alignment(alignment.transmute())
        .res()
    {
        Ok(layout) => {
            out_layout.write(layout.transmute());
            return true;
        }
        Err(e) => {
            log::error!("{e}");
        }
    };
    false
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc(
    provider: &z_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc_inner::<JustAlloc>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_gc(
    provider: &z_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc_inner::<GarbageCollect>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_gc_defrag(
    provider: &z_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc_inner::<Defragment<GarbageCollect>>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_gc_defrag_dealloc(
    provider: &z_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc_inner::<Deallocate<100, Defragment<GarbageCollect>>>(
        provider, size, alignment, out_buffer,
    )
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_gc_defrag_blocking(
    provider: &z_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc_inner::<BlockOn<Defragment<GarbageCollect>>>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_gc_defrag_async(
    provider: &z_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        bool,
        &mut MaybeUninit<z_buf_alloc_result_t>,
    ),
) {
    let result_context = result_context.transmute();
    //todo: this should be ported to tokio with executor argument support
    async_std::task::block_on(async move {
        let result = alloc_async::<
            BlockOn<Defragment<GarbageCollect>>,
            z_shared_memory_provider_threadsafe_t,
        >(provider, size, alignment, out_buffer)
        .await;
        (result_callback)(result_context.get(), result, out_buffer);
    });
}

#[allow(clippy::missing_safety_doc)]
unsafe fn alloc_inner<Policy: AllocPolicy>(
    provider: &z_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc::<Policy, z_shared_memory_provider_threadsafe_t, ThreadsafeContext>(
        provider, size, alignment, out_buffer,
    )
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_defragment(
    provider: &z_shared_memory_provider_threadsafe_t,
) -> usize {
    provider.transmute_ref().defragment()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_garbage_collect(
    provider: &z_shared_memory_provider_threadsafe_t,
) -> usize {
    provider.transmute_ref().garbage_collect()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_available(
    provider: &z_shared_memory_provider_threadsafe_t,
) -> usize {
    provider.transmute_ref().available()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_map(
    provider: &z_shared_memory_provider_threadsafe_t,
    allocated_chunk: z_allocated_chunk_t,
    len: usize,
    out_buffer: &mut MaybeUninit<z_slice_shm_t>,
) -> bool {
    let provider = provider.transmute_ref();
    match provider.map(allocated_chunk.transmute(), len) {
        Ok(buffer) => {
            out_buffer.write(buffer.transmute());
            true
        }
        Err(e) => {
            log::error!("{e}");
            false
        }
    }
}
