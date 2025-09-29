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
use zenoh::{
    shm::{
        BlockOn, ConstUsize, Deallocate, Defragment, GarbageCollect, JustAlloc, ShmProvider,
        ShmProviderBuilder,
    },
    Wait,
};

use super::{
    chunk::z_allocated_chunk_t,
    shm_provider_backend::{zc_shm_provider_backend_callbacks_t, DynamicShmProviderBackend},
    shm_provider_impl::{alloc, alloc_async, available, defragment, garbage_collect, map},
    types::z_alloc_alignment_t,
};
use crate::{
    context::{zc_context_t, zc_threadsafe_context_t, Context, ThreadsafeContext},
    result::{z_result_t, Z_EINVAL, Z_OK},
    shm::{
        protocol_implementations::posix::posix_shm_provider::PosixShmProvider,
        provider::types::z_buf_layout_alloc_result_t,
    },
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_loaned_shm_provider_t, z_moved_shm_provider_t, z_owned_shm_mut_t, z_owned_shm_provider_t,
};

pub type DynamicShmProvider = ShmProvider<DynamicShmProviderBackend<Context>>;

pub type DynamicShmProviderThreadsafe = ShmProvider<DynamicShmProviderBackend<ThreadsafeContext>>;

pub enum CSHMProvider {
    Posix(PosixShmProvider),
    Dynamic(DynamicShmProvider),
    DynamicThreadsafe(DynamicShmProviderThreadsafe),
}

decl_c_type!(
    owned(z_owned_shm_provider_t, option CSHMProvider),
    loaned(z_loaned_shm_provider_t),
);

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new SHM Provider ith default backend.
#[no_mangle]
pub extern "C" fn z_shm_provider_default_new(
    this: &mut MaybeUninit<z_owned_shm_provider_t>,
    size: usize,
) -> z_result_t {
    match ShmProviderBuilder::default_backend(size).wait() {
        Ok(provider) => {
            this.as_rust_type_mut_uninit()
                .write(Some(CSHMProvider::Posix(provider)));
            Z_OK
        }
        Err(e) => {
            crate::report_error!("{:?}", e);
            Z_EINVAL
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new SHM Provider.
#[no_mangle]
pub extern "C" fn z_shm_provider_new(
    this: &mut MaybeUninit<z_owned_shm_provider_t>,
    context: zc_context_t,
    callbacks: zc_shm_provider_backend_callbacks_t,
) {
    let backend = DynamicShmProviderBackend::new(context.into(), callbacks);
    let provider = ShmProviderBuilder::backend(backend).wait();

    this.as_rust_type_mut_uninit()
        .write(Some(CSHMProvider::Dynamic(provider)));
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new threadsafe SHM Provider.
#[no_mangle]
pub extern "C" fn z_shm_provider_threadsafe_new(
    this: &mut MaybeUninit<z_owned_shm_provider_t>,
    context: zc_threadsafe_context_t,
    callbacks: zc_shm_provider_backend_callbacks_t,
) {
    let backend = DynamicShmProviderBackend::new(context.into(), callbacks);
    let provider = ShmProviderBuilder::backend(backend).wait();

    this.as_rust_type_mut_uninit()
        .write(Some(CSHMProvider::DynamicThreadsafe(provider)));
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs SHM Provider in its gravestone value.
#[no_mangle]
pub extern "C" fn z_internal_shm_provider_null(this_: &mut MaybeUninit<z_owned_shm_provider_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_internal_shm_provider_check(this_: &z_owned_shm_provider_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows SHM Provider.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shm_provider_loan(
    this: &z_owned_shm_provider_t,
) -> &z_loaned_shm_provider_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deletes SHM Provider.
#[no_mangle]
pub extern "C" fn z_shm_provider_drop(this_: &mut z_moved_shm_provider_t) {
    let _ = this_.take_rust_type();
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation without any additional actions.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
) {
    alloc::<JustAlloc>(out_result, provider, size, z_alloc_alignment_t { pow: 0 })
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation performing garbage collection if needed.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
) {
    alloc::<GarbageCollect>(out_result, provider, size, z_alloc_alignment_t { pow: 0 })
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation performing garbage collection and/or defragmentation if needed.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
) {
    alloc::<Defragment<GarbageCollect>>(out_result, provider, size, z_alloc_alignment_t { pow: 0 })
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation performing garbage collection and/or defragmentation and/or forced deallocation if needed.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag_dealloc(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
) {
    alloc::<Deallocate<ConstUsize<100>, Defragment<GarbageCollect>>>(
        out_result,
        provider,
        size,
        z_alloc_alignment_t { pow: 0 },
    )
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation performing garbage collection and/or defragmentation and/or blocking if needed.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag_blocking(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
) {
    alloc::<BlockOn<Defragment<GarbageCollect>>>(
        out_result,
        provider,
        size,
        z_alloc_alignment_t { pow: 0 },
    )
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation performing garbage collection and/or defragmentation in async manner. Will return Z_EINVAL
/// if used with non-threadsafe SHM Provider.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag_async(
    out_result: &'static mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &'static z_loaned_shm_provider_t,
    size: usize,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    ),
) -> z_result_t {
    alloc_async::<BlockOn<Defragment<GarbageCollect>>>(
        out_result,
        provider,
        size,
        z_alloc_alignment_t { pow: 0 },
        result_context.into(),
        result_callback,
    )
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make aligned allocation without any additional actions.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_aligned(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) {
    alloc::<JustAlloc>(out_result, provider, size, alignment)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make aligned allocation performing garbage collection if needed.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_aligned(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) {
    alloc::<GarbageCollect>(out_result, provider, size, alignment)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make aligned allocation performing garbage collection and/or defragmentation if needed.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag_aligned(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) {
    alloc::<Defragment<GarbageCollect>>(out_result, provider, size, alignment)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make aligned allocation performing garbage collection and/or defragmentation and/or forced deallocation if needed.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag_dealloc_aligned(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) {
    alloc::<Deallocate<ConstUsize<100>, Defragment<GarbageCollect>>>(
        out_result, provider, size, alignment,
    )
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make aligned allocation performing garbage collection and/or defragmentation and/or blocking if needed.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag_blocking_aligned(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) {
    alloc::<BlockOn<Defragment<GarbageCollect>>>(out_result, provider, size, alignment)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make aligned allocation performing garbage collection and/or defragmentation in async manner. Will return Z_EINVAL
/// if used with non-threadsafe SHM Provider.
#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag_aligned_async(
    out_result: &'static mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &'static z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    ),
) -> z_result_t {
    alloc_async::<BlockOn<Defragment<GarbageCollect>>>(
        out_result,
        provider,
        size,
        alignment,
        result_context.into(),
        result_callback,
    )
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Perform memory defragmentation. The real operations taken depend on the provider's backend allocator
/// implementation.
#[no_mangle]
pub extern "C" fn z_shm_provider_defragment(provider: &z_loaned_shm_provider_t) -> usize {
    defragment(provider)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Perform memory garbage collection and reclaim all dereferenced SHM buffers.
#[no_mangle]
pub extern "C" fn z_shm_provider_garbage_collect(provider: &z_loaned_shm_provider_t) -> usize {
    garbage_collect(provider)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Return the memory size available in the provider.
#[no_mangle]
pub extern "C" fn z_shm_provider_available(provider: &z_loaned_shm_provider_t) -> usize {
    available(provider)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Map the preallocated data chunk into SHM buffer.
#[no_mangle]
pub extern "C" fn z_shm_provider_map(
    out_result: &mut MaybeUninit<z_owned_shm_mut_t>,
    provider: &z_loaned_shm_provider_t,
    allocated_chunk: z_allocated_chunk_t,
    len: usize,
) -> z_result_t {
    map(out_result, provider, allocated_chunk, len)
}
