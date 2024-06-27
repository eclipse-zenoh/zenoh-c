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
use std::mem::MaybeUninit;
use zenoh::prelude::*;
use zenoh::shm::{
    AllocPolicy, AsyncAllocPolicy, BufLayoutAllocResult, DynamicProtocolID,
    PosixShmProviderBackend, ProtocolIDSource, ShmProvider, ShmProviderBackend, StaticProtocolID,
    ZLayoutAllocError, POSIX_PROTOCOL_ID,
};

use crate::context::{Context, DroppableContext, ThreadsafeContext};
use crate::errors::{z_error_t, Z_EINVAL, Z_OK};
use crate::transmute::{TransmuteCopy, TransmuteFromHandle};
use crate::transmute2::RustTypeRefUninit;
use crate::{z_loaned_shm_provider_t, z_owned_buf_alloc_result_t, z_owned_shm_mut_t};

use super::chunk::z_allocated_chunk_t;
use super::shm_provider_backend::DynamicShmProviderBackend;
use super::types::z_alloc_alignment_t;

pub(crate) fn alloc<Policy: AllocPolicy>(
    out_result: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    match provider.transmute_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => {
            alloc_impl::<Policy, StaticProtocolID<POSIX_PROTOCOL_ID>, PosixShmProviderBackend>(
                out_result, provider, size, alignment,
            )
        }
        super::shm_provider::CSHMProvider::Dynamic(provider) => {
            alloc_impl::<Policy, DynamicProtocolID, DynamicShmProviderBackend<Context>>(
                out_result, provider, size, alignment,
            )
        }
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => {
            alloc_impl::<Policy, DynamicProtocolID, DynamicShmProviderBackend<ThreadsafeContext>>(
                out_result, provider, size, alignment,
            )
        }
    }
}

pub(crate) fn alloc_async<Policy: AsyncAllocPolicy>(
    out_result: &'static mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &'static z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    result_context: ThreadsafeContext,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        z_error_t,
        *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    ),
) -> z_error_t {
    match provider.transmute_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => {
            alloc_async_impl::<Policy, StaticProtocolID<POSIX_PROTOCOL_ID>, PosixShmProviderBackend>(
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
            alloc_async_impl::<
                Policy,
                DynamicProtocolID,
                DynamicShmProviderBackend<ThreadsafeContext>,
            >(
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

pub(crate) fn defragment(provider: &z_loaned_shm_provider_t) {
    match provider.transmute_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => {
            provider.defragment();
        }
        super::shm_provider::CSHMProvider::Dynamic(provider) => {
            provider.defragment();
        }
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => {
            provider.defragment();
        }
    }
}

pub(crate) fn garbage_collect(provider: &z_loaned_shm_provider_t) {
    match provider.transmute_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => {
            provider.garbage_collect();
        }
        super::shm_provider::CSHMProvider::Dynamic(provider) => {
            provider.garbage_collect();
        }
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => {
            provider.garbage_collect();
        }
    }
}

pub(crate) fn available(provider: &z_loaned_shm_provider_t) -> usize {
    match provider.transmute_ref() {
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
) {
    let mapping = match provider.transmute_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => {
            provider.map(allocated_chunk.into(), len)
        }
        super::shm_provider::CSHMProvider::Dynamic(provider) => {
            provider.map(allocated_chunk.into(), len)
        }
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => {
            provider.map(allocated_chunk.into(), len)
        }
    };

    match mapping {
        Ok(buffer) => {
            out_result.as_rust_type_mut_uninit().write(Some(buffer));
        }
        Err(e) => {
            log::error!("{e}");
            out_result.as_rust_type_mut_uninit().write(None);
        }
    }
}

fn alloc_impl<Policy: AllocPolicy, TProtocolID: ProtocolIDSource, TBackend: ShmProviderBackend>(
    out_result: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &ShmProvider<TProtocolID, TBackend>,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    let result = provider
        .alloc(size)
        .with_alignment(alignment.transmute_copy())
        .with_policy::<Policy>()
        .wait();

    parse_result(out_result, result)
}

pub(crate) fn alloc_async_impl<
    Policy: AsyncAllocPolicy,
    TProtocolID: ProtocolIDSource,
    TBackend: ShmProviderBackend + Send + Sync,
>(
    out_result: &'static mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &'static ShmProvider<TProtocolID, TBackend>,
    size: usize,
    alignment: z_alloc_alignment_t,
    result_context: ThreadsafeContext,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        z_error_t,
        *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    ),
) {
    //todo: this should be ported to tokio with executor argument support
    async_std::task::spawn(async move {
        let result = provider
            .alloc(size)
            .with_alignment(alignment.transmute_copy())
            .with_policy::<Policy>()
            .await;
        let error = parse_result(out_result, result);
        unsafe {
            (result_callback)(result_context.get(), error, out_result);
        }
    });
}

fn parse_result(
    out_result: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    result: BufLayoutAllocResult,
) -> z_error_t {
    match result {
        Ok(buf) => {
            out_result.as_rust_type_mut_uninit().write(Some(Ok(buf)));
            Z_OK
        }
        Err(ZLayoutAllocError::Alloc(e)) => {
            out_result.as_rust_type_mut_uninit().write(Some(Err(e)));
            Z_OK
        }
        Err(ZLayoutAllocError::Layout(_)) => Z_EINVAL,
    }
}
