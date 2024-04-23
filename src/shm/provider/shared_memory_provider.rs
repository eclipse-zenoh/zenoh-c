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

use zenoh::shm::provider::shared_memory_provider::{
    AllocPolicy, BlockOn, Deallocate, Defragment, DynamicProtocolID, GarbageCollect, JustAlloc,
    SharedMemoryProvider, SharedMemoryProviderBuilder,
};

use crate::{
    common::types::z_protocol_id_t, decl_rust_copy_type, impl_guarded_transmute, zc_context_t,
    Context, GuardedTransmute,
};

use super::{
    chunk::z_allocated_chunk_t,
    shared_memory_provider_backend::{
        zc_shared_memory_provider_backend_callbacks_t, DynamicSharedMemoryProviderBackend,
    },
    shared_memory_provider_impl::alloc,
    types::{z_alloc_alignment_t, z_owned_buf_alloc_result_t},
    zsliceshm::z_slice_shm_mut_t,
};

/// A non-thread-safe SharedMemoryProvider specialization
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_shared_memory_provider_t([u64; 14]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_shared_memory_provider_t([u64; 14]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_shared_memory_provider_t([u64; 14]);

decl_rust_copy_type!(
    zenoh:(SharedMemoryProvider<DynamicProtocolID, DynamicSharedMemoryProviderBackend<Context>>),
    c:(z_shared_memory_provider_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_new(
    id: z_protocol_id_t,
    context: zc_context_t,
    callbacks: zc_shared_memory_provider_backend_callbacks_t,
    out_provider: &mut MaybeUninit<z_shared_memory_provider_t>,
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
pub unsafe extern "C" fn z_shared_memory_provider_delete(provider: z_shared_memory_provider_t) {
    let _ = provider.transmute();
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_alloc(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
) -> bool {
    alloc_inner::<JustAlloc>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_alloc_gc(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
) -> bool {
    alloc_inner::<GarbageCollect>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_alloc_gc_defrag(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
) -> bool {
    alloc_inner::<Defragment<GarbageCollect>>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_alloc_gc_defrag_dealloc(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
) -> bool {
    alloc_inner::<Deallocate<100, Defragment<GarbageCollect>>>(
        provider, size, alignment, out_buffer,
    )
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_alloc_gc_defrag_blocking(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
) -> bool {
    alloc_inner::<BlockOn<Defragment<GarbageCollect>>>(provider, size, alignment, out_buffer)
}

#[allow(clippy::missing_safety_doc)]
unsafe fn alloc_inner<Policy: AllocPolicy>(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
) -> bool {
    alloc::<Policy, z_shared_memory_provider_t, Context>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_defragment(
    provider: &z_shared_memory_provider_t,
) -> usize {
    provider.transmute_ref().defragment()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_garbage_collect(
    provider: &z_shared_memory_provider_t,
) -> usize {
    provider.transmute_ref().garbage_collect()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_available(
    provider: &z_shared_memory_provider_t,
) -> usize {
    provider.transmute_ref().available()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_map(
    provider: &z_shared_memory_provider_t,
    allocated_chunk: z_allocated_chunk_t,
    len: usize,
    out_buffer: &mut MaybeUninit<z_slice_shm_mut_t>,
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
