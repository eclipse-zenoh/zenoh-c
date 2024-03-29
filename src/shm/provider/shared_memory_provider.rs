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
    common::types::z_protocol_id_t, decl_rust_copy_type, impl_guarded_transmute, GuardedTransmute,
};

use super::{
    chunk::z_allocated_chunk_t,
    shared_memory_provider_backend::{
        zc_context_t, zc_shared_memory_provider_backend_callbacks_t,
        DynamicSharedMemoryProviderBackend,
    },
    types::{z_alloc_alignment_t, z_buf_alloc_result_t},
    zsliceshm::z_slice_shm_t,
};

/// An owned SharedMemoryProvider
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
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
    zenoh:(SharedMemoryProvider<DynamicProtocolID, DynamicSharedMemoryProviderBackend>),
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
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc::<JustAlloc>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_alloc_gc(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc::<GarbageCollect>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_alloc_gc_defrag(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc::<Defragment<GarbageCollect>>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_alloc_gc_defrag_dealloc(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc::<Deallocate<100, Defragment<GarbageCollect>>>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_alloc_gc_defrag_blocking(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    alloc::<BlockOn<Defragment<GarbageCollect>>>(provider, size, alignment, out_buffer)
}

#[allow(clippy::missing_safety_doc)]
unsafe fn alloc<Policy: AllocPolicy>(
    provider: &z_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool {
    let provider = provider.transmute_ref();
    match provider
        .alloc_layout()
        .size(size)
        .alignment(alignment.transmute())
        .res()
    {
        Ok(layout) => {
            let result = layout.alloc().with_policy::<Policy>().res().transmute();
            out_buffer.write(result);
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
