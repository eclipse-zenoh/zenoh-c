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

use zenoh::shm::{
    AllocPolicy, BlockOn, Deallocate, Defragment, DynamicProtocolID, GarbageCollect, JustAlloc,
    SharedMemoryProvider, SharedMemoryProviderBuilder,
};

use crate::{
    context::{zc_context_t, Context},
    errors::z_error_t,
    shm::common::types::z_protocol_id_t,
    transmute::{
        unwrap_ref_unchecked, Inplace, TransmuteFromHandle, TransmuteIntoHandle, TransmuteRef,
        TransmuteUninitPtr,
    },
    z_owned_buf_alloc_result_t, z_owned_shm_mut_t,
};

use super::{
    chunk::z_allocated_chunk_t,
    shared_memory_provider_backend::{
        zc_shared_memory_provider_backend_callbacks_t, DynamicSharedMemoryProviderBackend,
    },
    shared_memory_provider_impl::alloc,
    types::z_alloc_alignment_t,
};

/// A loaned SharedMemoryProvider specialization
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_loaned_shared_memory_provider_t([u64; 14]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_loaned_shared_memory_provider_t([u64; 14]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_loaned_shared_memory_provider_t([u64; 14]);

/// An owned SharedMemoryProvider specialization
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_owned_shared_memory_provider_t([u64; 14]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_owned_shared_memory_provider_t([u64; 14]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_owned_shared_memory_provider_t([u64; 14]);

decl_transmute_owned!(
    Option<SharedMemoryProvider<DynamicProtocolID, DynamicSharedMemoryProviderBackend<Context>>>,
    z_owned_shared_memory_provider_t
);

decl_transmute_handle!(
    SharedMemoryProvider<DynamicProtocolID, DynamicSharedMemoryProviderBackend<Context>>,
    z_loaned_shared_memory_provider_t
);

/// Creates a new SHM Provider
#[no_mangle]
pub extern "C" fn z_shared_memory_provider_new(
    this: *mut MaybeUninit<z_owned_shared_memory_provider_t>,
    id: z_protocol_id_t,
    context: zc_context_t,
    callbacks: zc_shared_memory_provider_backend_callbacks_t,
) {
    let backend = DynamicSharedMemoryProviderBackend::new(context.into(), callbacks);
    let provider = SharedMemoryProviderBuilder::builder()
        .dynamic_protocol_id(id)
        .backend(backend)
        .res();

    Inplace::init(this.transmute_uninit_ptr(), Some(provider));
}

/// Constructs SHM Provider in its gravestone value.
#[no_mangle]
pub extern "C" fn z_shared_memory_provider_null(
    this: *mut MaybeUninit<z_owned_shared_memory_provider_t>,
) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_shared_memory_provider_check(this: &z_owned_shared_memory_provider_t) -> bool {
    this.transmute_ref().is_some()
}

/// Borrows SHM Provider
#[no_mangle]
pub extern "C" fn z_shared_memory_provider_loan(
    this: &z_owned_shared_memory_provider_t,
) -> &z_loaned_shared_memory_provider_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

/// Deletes SHM Provider
#[no_mangle]
pub extern "C" fn z_shared_memory_provider_delete(this: &mut z_owned_shared_memory_provider_t) {
    let _ = this.transmute_mut().extract();
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc_inner::<JustAlloc>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc_gc(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc_inner::<GarbageCollect>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc_gc_defrag(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc_inner::<Defragment<GarbageCollect>>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc_gc_defrag_dealloc(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc_inner::<Deallocate<100, Defragment<GarbageCollect>>>(
        out_result, provider, size, alignment,
    )
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc_gc_defrag_blocking(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc_inner::<BlockOn<Defragment<GarbageCollect>>>(out_result, provider, size, alignment)
}

fn alloc_inner<Policy: AllocPolicy>(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<Policy, Context>(out_result, provider.transmute_ref(), size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_defragment(
    provider: &z_loaned_shared_memory_provider_t,
) {
    let _ = provider.transmute_ref().defragment();
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_garbage_collect(
    provider: &z_loaned_shared_memory_provider_t,
) {
    let _ = provider.transmute_ref().garbage_collect();
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_map(
    out_result: *mut MaybeUninit<z_owned_shm_mut_t>,
    provider: &z_loaned_shared_memory_provider_t,
    allocated_chunk: z_allocated_chunk_t,
    len: usize,
) {
    let provider = provider.transmute_ref();
    match provider.map(allocated_chunk.into(), len) {
        Ok(buffer) => Inplace::init(out_result.transmute_uninit_ptr(), Some(buffer)),
        Err(e) => {
            log::error!("{e}");
            Inplace::init(out_result.transmute_uninit_ptr(), None)
        }
    }
}
