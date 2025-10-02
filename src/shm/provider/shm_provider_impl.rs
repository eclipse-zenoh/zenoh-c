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
        AllocPolicy, AsyncAllocPolicy, BlockOn, ConstBool, ConstPolicy, Deallocate, Defragment,
        GarbageCollect, JustAlloc, PolicyValue, PosixShmProviderBackend, ShmProvider,
        ShmProviderBackend,
    },
    Wait,
};

use super::{
    chunk::z_allocated_chunk_t, shm_provider_backend::DynamicShmProviderBackend,
    types::z_alloc_alignment_t,
};
use crate::{
    context::{Context, DroppableContext, ThreadsafeContext},
    result::{z_result_t, Z_EINVAL, Z_OK},
    shm::provider::types::z_buf_layout_alloc_result_t,
    transmute::{IntoRustType, RustTypeRef, RustTypeRefUninit},
    z_loaned_shm_provider_t, z_owned_shm_mut_t,
};

pub(crate) trait GenericAllocPolicy {
    type AllocPolicy<Backend: ShmProviderBackend>: AllocPolicy<Backend> + ConstPolicy + Send + Sync;
}

impl GenericAllocPolicy for JustAlloc {
    type AllocPolicy<Backend: ShmProviderBackend> = JustAlloc;
}
impl<InnerPolicy> GenericAllocPolicy for BlockOn<InnerPolicy>
where
    InnerPolicy: GenericAllocPolicy,
{
    type AllocPolicy<Backend: ShmProviderBackend> = BlockOn<InnerPolicy::AllocPolicy<Backend>>;
}
impl<InnerPolicy, AltPolicy, Safe> GenericAllocPolicy
    for GarbageCollect<InnerPolicy, AltPolicy, Safe>
where
    InnerPolicy: GenericAllocPolicy,
    AltPolicy: GenericAllocPolicy,
    Safe: PolicyValue<bool> + ConstPolicy + Send + Sync,
{
    type AllocPolicy<Backend: ShmProviderBackend> =
        GarbageCollect<InnerPolicy::AllocPolicy<Backend>, AltPolicy::AllocPolicy<Backend>, Safe>;
}
impl<InnerPolicy, AltPolicy> GenericAllocPolicy for Defragment<InnerPolicy, AltPolicy>
where
    InnerPolicy: GenericAllocPolicy,
    AltPolicy: GenericAllocPolicy,
{
    type AllocPolicy<Backend: ShmProviderBackend> =
        Defragment<InnerPolicy::AllocPolicy<Backend>, AltPolicy::AllocPolicy<Backend>>;
}
impl<Limit, InnerPolicy, AltPolicy> GenericAllocPolicy for Deallocate<Limit, InnerPolicy, AltPolicy>
where
    Limit: PolicyValue<usize> + ConstPolicy + Send + Sync,
    InnerPolicy: GenericAllocPolicy,
    AltPolicy: GenericAllocPolicy,
{
    type AllocPolicy<Backend: ShmProviderBackend> =
        Deallocate<Limit, InnerPolicy::AllocPolicy<Backend>, AltPolicy::AllocPolicy<Backend>>;
}

pub(crate) trait GenericAsyncAllocPolicy {
    type AsyncAllocPolicy<Backend: ShmProviderBackend + Sync>: AsyncAllocPolicy<Backend>
        + ConstPolicy
        + Sync;
}

impl<InnerPolicy> GenericAsyncAllocPolicy for BlockOn<InnerPolicy>
where
    InnerPolicy: GenericAllocPolicy,
{
    type AsyncAllocPolicy<Backend: ShmProviderBackend + Sync> =
        BlockOn<InnerPolicy::AllocPolicy<Backend>>;
}

pub(crate) type UnsafeGarbageCollect = GarbageCollect<JustAlloc, JustAlloc, ConstBool<false>>;

pub(crate) fn alloc<Policy: GenericAllocPolicy>(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) {
    match provider.as_rust_type_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => {
            alloc_impl::<Policy, PosixShmProviderBackend>(out_result, provider, size, alignment)
        }
        super::shm_provider::CSHMProvider::Dynamic(provider) => {
            alloc_impl::<Policy, DynamicShmProviderBackend<Context>>(
                out_result, provider, size, alignment,
            )
        }
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => {
            alloc_impl::<Policy, DynamicShmProviderBackend<ThreadsafeContext>>(
                out_result, provider, size, alignment,
            )
        }
    }
}

pub(crate) fn alloc_async<Policy: GenericAsyncAllocPolicy>(
    out_result: &'static mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &'static z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    result_context: ThreadsafeContext,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    ),
) -> z_result_t {
    match provider.as_rust_type_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => {
            alloc_async_impl::<Policy, PosixShmProviderBackend>(
                out_result,
                provider,
                size,
                alignment,
                result_context,
                result_callback,
            );
            Z_OK
        }
        super::shm_provider::CSHMProvider::Dynamic(_) => Z_EINVAL,
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => {
            alloc_async_impl::<Policy, DynamicShmProviderBackend<ThreadsafeContext>>(
                out_result,
                provider,
                size,
                alignment,
                result_context,
                result_callback,
            );
            Z_OK
        }
    }
}

pub(crate) fn defragment(provider: &z_loaned_shm_provider_t) -> usize {
    match provider.as_rust_type_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => provider.defragment(),
        super::shm_provider::CSHMProvider::Dynamic(provider) => provider.defragment(),
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => provider.defragment(),
    }
}

pub(crate) unsafe fn garbage_collect_unsafe(provider: &z_loaned_shm_provider_t) -> usize {
    match provider.as_rust_type_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => unsafe {
            provider.garbage_collect_unsafe()
        },
        super::shm_provider::CSHMProvider::Dynamic(provider) => unsafe {
            provider.garbage_collect_unsafe()
        },
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => unsafe {
            provider.garbage_collect_unsafe()
        },
    }
}

pub(crate) fn available(provider: &z_loaned_shm_provider_t) -> usize {
    match provider.as_rust_type_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => provider.available(),
        super::shm_provider::CSHMProvider::Dynamic(provider) => provider.available(),
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => provider.available(),
    }
}

#[no_mangle]
pub(crate) fn map(
    out_result: &mut MaybeUninit<z_owned_shm_mut_t>,
    provider: &z_loaned_shm_provider_t,
    allocated_chunk: z_allocated_chunk_t,
    len: usize,
) -> z_result_t {
    let chunk = match allocated_chunk.try_into() {
        Ok(val) => val,
        Err(_) => return Z_EINVAL,
    };

    let mapping = match provider.as_rust_type_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => provider.map(chunk, len),
        super::shm_provider::CSHMProvider::Dynamic(provider) => provider.map(chunk, len),
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => provider.map(chunk, len),
    };

    match mapping {
        Ok(buffer) => {
            out_result.as_rust_type_mut_uninit().write(Some(buffer));
            Z_OK
        }
        Err(e) => {
            crate::report_error!("{:?}", e);
            Z_EINVAL
        }
    }
}

fn alloc_impl<Policy: GenericAllocPolicy, TBackend: ShmProviderBackend>(
    out_result: &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &ShmProvider<TBackend>,
    size: usize,
    alignment: z_alloc_alignment_t,
) {
    let result = unsafe {
        provider
            .alloc((size, alignment.into_rust_type()))
            .with_unsafe_policy::<Policy::AllocPolicy<TBackend>>()
            .wait()
    };

    out_result.write(result.into());
}

pub(crate) fn alloc_async_impl<
    Policy: GenericAsyncAllocPolicy,
    TBackend: ShmProviderBackend + Send + Sync,
>(
    out_result: &'static mut MaybeUninit<z_buf_layout_alloc_result_t>,
    provider: &'static ShmProvider<TBackend>,
    size: usize,
    alignment: z_alloc_alignment_t,
    result_context: ThreadsafeContext,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        &mut MaybeUninit<z_buf_layout_alloc_result_t>,
    ),
) {
    zenoh_runtime::ZRuntime::Application.spawn(async move {
        let result = unsafe {
            provider
                .alloc((size, alignment.into_rust_type()))
                .with_unsafe_policy::<Policy::AsyncAllocPolicy<TBackend>>()
                .await
        };
        out_result.write(result.into());
        unsafe {
            (result_callback)(result_context.get(), out_result);
        }
    });
}
