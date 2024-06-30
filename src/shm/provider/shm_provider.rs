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
    BlockOn, Deallocate, Defragment, DynamicProtocolID, GarbageCollect, JustAlloc, ShmProvider,
    ShmProviderBuilder,
};

use crate::{
    context::{zc_context_t, zc_threadsafe_context_t, Context, ThreadsafeContext},
    errors::z_error_t,
    shm::{
        common::types::z_protocol_id_t,
        protocol_implementations::posix::posix_shm_provider::PosixShmProvider,
    },
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_loaned_shm_provider_t, z_moved_shm_provider_t, z_owned_buf_alloc_result_t, z_owned_shm_mut_t,
    z_owned_shm_provider_t,
};

use super::{
    chunk::z_allocated_chunk_t,
    shm_provider_backend::{zc_shm_provider_backend_callbacks_t, DynamicShmProviderBackend},
    shm_provider_impl::{alloc, alloc_async, available, defragment, garbage_collect, map},
    types::z_alloc_alignment_t,
};

pub type DynamicShmProvider = ShmProvider<DynamicProtocolID, DynamicShmProviderBackend<Context>>;

pub type DynamicShmProviderThreadsafe =
    ShmProvider<DynamicProtocolID, DynamicShmProviderBackend<ThreadsafeContext>>;

pub enum CSHMProvider {
    Posix(PosixShmProvider),
    Dynamic(DynamicShmProvider),
    DynamicThreadsafe(DynamicShmProviderThreadsafe),
}

decl_c_type!(
    owned(z_owned_shm_provider_t, z_moved_shm_provider_t, Option<CSHMProvider>),
    loaned(z_loaned_shm_provider_t, CSHMProvider),
);

/// Creates a new SHM Provider
#[no_mangle]
pub extern "C" fn z_shm_provider_new(
    this: &mut MaybeUninit<z_owned_shm_provider_t>,
    id: z_protocol_id_t,
    context: zc_context_t,
    callbacks: zc_shm_provider_backend_callbacks_t,
) {
    let backend = DynamicShmProviderBackend::new(context.into(), callbacks);
    let provider = ShmProviderBuilder::builder()
        .dynamic_protocol_id(id)
        .backend(backend)
        .res();

    this.as_rust_type_mut_uninit()
        .write(Some(CSHMProvider::Dynamic(provider)));
}

/// Creates a new threadsafe SHM Provider
#[no_mangle]
pub extern "C" fn z_shm_provider_threadsafe_new(
    this: &mut MaybeUninit<z_owned_shm_provider_t>,
    id: z_protocol_id_t,
    context: zc_threadsafe_context_t,
    callbacks: zc_shm_provider_backend_callbacks_t,
) {
    let backend = DynamicShmProviderBackend::new(context.into(), callbacks);
    let provider = ShmProviderBuilder::builder()
        .dynamic_protocol_id(id)
        .backend(backend)
        .res();

    this.as_rust_type_mut_uninit()
        .write(Some(CSHMProvider::DynamicThreadsafe(provider)));
}

/// Constructs SHM Provider in its gravestone value.
#[no_mangle]
pub extern "C" fn z_shm_provider_null(this: &mut MaybeUninit<z_owned_shm_provider_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_shm_provider_check(this: &z_owned_shm_provider_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Borrows SHM Provider
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

/// Deletes SHM Provider
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_shm_provider_drop(this: z_moved_shm_provider_t) {}

#[no_mangle]
pub extern "C" fn z_shm_provider_alloc(
    out_result: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<JustAlloc>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc(
    out_result: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<GarbageCollect>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag(
    out_result: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<Defragment<GarbageCollect>>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag_dealloc(
    out_result: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<Deallocate<100, Defragment<GarbageCollect>>>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag_blocking(
    out_result: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc::<BlockOn<Defragment<GarbageCollect>>>(out_result, provider, size, alignment)
}

#[no_mangle]
pub extern "C" fn z_shm_provider_alloc_gc_defrag_async(
    out_result: &'static mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &'static z_loaned_shm_provider_t,
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
pub extern "C" fn z_shm_provider_defragment(provider: &z_loaned_shm_provider_t) {
    defragment(provider);
}

#[no_mangle]
pub extern "C" fn z_shm_provider_garbage_collect(provider: &z_loaned_shm_provider_t) {
    garbage_collect(provider);
}

#[no_mangle]
pub extern "C" fn z_shm_provider_available(provider: &z_loaned_shm_provider_t) -> usize {
    available(provider)
}

#[no_mangle]
pub extern "C" fn z_shm_provider_map(
    out_result: &mut MaybeUninit<z_owned_shm_mut_t>,
    provider: &z_loaned_shm_provider_t,
    allocated_chunk: z_allocated_chunk_t,
    len: usize,
) {
    map(out_result, provider, allocated_chunk, len);
}
