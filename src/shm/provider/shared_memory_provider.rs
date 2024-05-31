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
use zenoh::shm::{
    BlockOn, Deallocate, Defragment, DynamicProtocolID, GarbageCollect, JustAlloc,
    SharedMemoryProvider, SharedMemoryProviderBuilder,
};

use crate::{
    context::{zc_context_t, zc_threadsafe_context_t, Context, ThreadsafeContext},
    errors::z_error_t,
    shm::{
        common::types::z_protocol_id_t,
        protocol_implementations::posix::posix_shared_memory_provider::PosixSharedMemoryProvider,
    },
    transmute::{
        unwrap_ref_unchecked, Inplace, TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
    },
    z_owned_buf_alloc_result_t, z_owned_shm_mut_t,
};

use super::{
    chunk::z_allocated_chunk_t,
    shared_memory_provider_backend::{
        zc_shared_memory_provider_backend_callbacks_t, DynamicSharedMemoryProviderBackend,
    },
    shared_memory_provider_impl::{
        alloc, alloc_async, available, defragment, garbage_collect, map,
    },
    types::z_alloc_alignment_t,
};

/// A loaned SharedMemoryProvider specialization
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[repr(C, align(8))]
pub struct z_loaned_shared_memory_provider_t([u64; 26]);

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
#[repr(C, align(8))]
pub struct z_loaned_shared_memory_provider_t([u64; 32]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(8))]
pub struct z_loaned_shared_memory_provider_t([u64; 26]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_loaned_shared_memory_provider_t([u64; 26]);

/// An owned SharedMemoryProvider specialization
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[repr(C, align(8))]
pub struct z_owned_shared_memory_provider_t([u64; 26]);

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
#[repr(C, align(8))]
pub struct z_owned_shared_memory_provider_t([u64; 32]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(8))]
pub struct z_owned_shared_memory_provider_t([u64; 26]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_owned_shared_memory_provider_t([u64; 26]);

pub type DynamicSharedMemoryProvider =
    SharedMemoryProvider<DynamicProtocolID, DynamicSharedMemoryProviderBackend<Context>>;

pub type DynamicSharedMemoryProviderThreadsafe =
    SharedMemoryProvider<DynamicProtocolID, DynamicSharedMemoryProviderBackend<ThreadsafeContext>>;

pub enum CSHMProvider {
    Posix(PosixSharedMemoryProvider),
    Dynamic(DynamicSharedMemoryProvider),
    DynamicThreadsafe(DynamicSharedMemoryProviderThreadsafe),
}

decl_transmute_owned!(Option<CSHMProvider>, z_owned_shared_memory_provider_t);

decl_transmute_handle!(CSHMProvider, z_loaned_shared_memory_provider_t);

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

    Inplace::init(
        this.transmute_uninit_ptr(),
        Some(CSHMProvider::Dynamic(provider)),
    );
}

/// Creates a new threadsafe SHM Provider
#[no_mangle]
pub extern "C" fn z_shared_memory_provider_threadsafe_new(
    this: *mut MaybeUninit<z_owned_shared_memory_provider_t>,
    id: z_protocol_id_t,
    context: zc_threadsafe_context_t,
    callbacks: zc_shared_memory_provider_backend_callbacks_t,
) {
    let backend = DynamicSharedMemoryProviderBackend::new(context.into(), callbacks);
    let provider = SharedMemoryProviderBuilder::builder()
        .dynamic_protocol_id(id)
        .backend(backend)
        .res();

    Inplace::init(
        this.transmute_uninit_ptr(),
        Some(CSHMProvider::DynamicThreadsafe(provider)),
    );
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
pub extern "C" fn z_shared_memory_provider_drop(this: &mut z_owned_shared_memory_provider_t) {
    let _ = this.transmute_mut().take();
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<JustAlloc>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc_gc(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<GarbageCollect>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc_gc_defrag(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<Defragment<GarbageCollect>>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc_gc_defrag_dealloc(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<Deallocate<100, Defragment<GarbageCollect>>>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc_gc_defrag_blocking(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<BlockOn<Defragment<GarbageCollect>>>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_alloc_gc_defrag_async(
    out_result: &'static mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &'static z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        z_error_t,
        *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    ),
) -> z_error_t {
    alloc_async::<BlockOn<Defragment<GarbageCollect>>>(
        out_result,
        provider,
        size,
        alignment,
        result_context.into(),
        result_callback,
    )
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_defragment(
    provider: &z_loaned_shared_memory_provider_t,
) {
    defragment(provider);
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_garbage_collect(
    provider: &z_loaned_shared_memory_provider_t,
) {
    garbage_collect(provider);
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_available(
    provider: &z_loaned_shared_memory_provider_t,
) -> usize {
    available(provider)
}

#[no_mangle]
pub extern "C" fn z_shared_memory_provider_map(
    out_result: *mut MaybeUninit<z_owned_shm_mut_t>,
    provider: &z_loaned_shared_memory_provider_t,
    allocated_chunk: z_allocated_chunk_t,
    len: usize,
) {
    map(out_result, provider, allocated_chunk, len);
}
