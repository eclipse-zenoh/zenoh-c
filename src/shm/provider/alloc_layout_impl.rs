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
use zenoh::prelude::*;
use zenoh::shm::AllocPolicy;

use crate::{
    context::{zc_threadsafe_context_t, DroppableContext, ThreadsafeContext},
    errors::{z_error_t, Z_EINVAL, Z_OK},
    transmute::{Inplace, TransmuteCopy, TransmuteFromHandle, TransmuteUninitPtr},
    z_owned_buf_alloc_result_t,
};

use super::{
    alloc_layout::{z_loaned_alloc_layout_t, z_owned_alloc_layout_t, CSHMLayout},
    shared_memory_provider::z_loaned_shared_memory_provider_t,
    shared_memory_provider_backend::DynamicSharedMemoryProviderBackend,
    types::z_alloc_alignment_t,
};

pub(crate) fn alloc_layout_new(
    this: *mut MaybeUninit<z_owned_alloc_layout_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    let layout = match provider.transmute_ref() {
        super::shared_memory_provider::CSHMProvider::Posix(provider) => {
            match provider
                .alloc(size)
                .with_alignment(alignment.transmute_copy())
                .into_layout()
            {
                Ok(layout) => CSHMLayout::Posix(layout),
                Err(e) => {
                    log::error!("{:?}", e);
                    return Z_EINVAL;
                }
            }
        }
        super::shared_memory_provider::CSHMProvider::Dynamic(provider) => {
            match provider
                .alloc(size)
                .with_alignment(alignment.transmute_copy())
                .into_layout()
            {
                Ok(layout) => CSHMLayout::Dynamic(layout),
                Err(e) => {
                    log::error!("{:?}", e);
                    return Z_EINVAL;
                }
            }
        }
        super::shared_memory_provider::CSHMProvider::DynamicThreadsafe(provider) => {
            match provider
                .alloc(size)
                .with_alignment(alignment.transmute_copy())
                .into_layout()
            {
                Ok(layout) => CSHMLayout::DynamicThreadsafe(layout),
                Err(e) => {
                    log::error!("{:?}", e);
                    return Z_EINVAL;
                }
            }
        }
    };

    Inplace::init(this.transmute_uninit_ptr(), Some(layout));
    Z_OK
}

pub(crate) fn alloc<Policy: AllocPolicy>(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    let result = match layout.transmute_ref() {
        super::alloc_layout::CSHMLayout::Posix(layout) => {
            layout.alloc().with_policy::<Policy>().wait()
        }
        super::alloc_layout::CSHMLayout::Dynamic(layout) => {
            layout.alloc().with_policy::<Policy>().wait()
        }
        super::alloc_layout::CSHMLayout::DynamicThreadsafe(layout) => {
            layout.alloc().with_policy::<Policy>().wait()
        }
    };
    Inplace::init(out_result.transmute_uninit_ptr(), Some(result));
}

pub(crate) fn alloc_async<Policy: AsyncAllocPolicy>(
    out_result: &'static mut MaybeUninit<z_owned_buf_alloc_result_t>,
    layout: &'static z_loaned_alloc_layout_t,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    ),
) -> z_error_t {
    match layout.transmute_ref() {
        super::alloc_layout::CSHMLayout::Posix(layout) => {
            alloc_async_impl::<
                Policy,
                StaticProtocolID<POSIX_PROTOCOL_ID>,
                PosixSharedMemoryProviderBackend,
            >(out_result, layout, result_context, result_callback);
            Z_OK
        }
        super::alloc_layout::CSHMLayout::Dynamic(_) => Z_EINVAL,
        super::alloc_layout::CSHMLayout::DynamicThreadsafe(layout) => {
            alloc_async_impl::<
                Policy,
                DynamicProtocolID,
                DynamicSharedMemoryProviderBackend<ThreadsafeContext>,
            >(out_result, layout, result_context, result_callback);
            Z_OK
        }
    }
}

pub fn alloc_async_impl<
    Policy: AsyncAllocPolicy,
    IDSource: ProtocolIDSource,
    Backend: SharedMemoryProviderBackend + Send + Sync,
>(
    out_result: &'static mut MaybeUninit<z_owned_buf_alloc_result_t>,
    layout: &'static AllocLayout<'static, IDSource, Backend>,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    ),
) {
    let result_context: ThreadsafeContext = result_context.into();
    //todo: this should be ported to tokio with executor argument support
    async_std::task::spawn(async move {
        let result = layout.alloc().with_policy::<Policy>().await;
        Inplace::init(
            (out_result as *mut MaybeUninit<z_owned_buf_alloc_result_t>).transmute_uninit_ptr(),
            Some(result),
        );
        unsafe { (result_callback)(result_context.get(), out_result) };
    });
}
