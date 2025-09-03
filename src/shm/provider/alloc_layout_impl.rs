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
        AllocLayout, AllocPolicy, AsyncAllocPolicy, MemoryLayout, PosixShmProviderBackend,
        ShmProviderBackend,
    },
    Wait,
};

use super::{
    alloc_layout::CSHMLayout, shm_provider_backend::DynamicShmProviderBackend,
    types::z_alloc_alignment_t,
};
use crate::{
    context::{zc_threadsafe_context_t, DroppableContext, ThreadsafeContext},
    result::{z_result_t, Z_EINVAL, Z_OK},
    shm::provider::types::z_buf_alloc_result_t,
    transmute::{IntoRustType, RustTypeRef, RustTypeRefUninit},
    z_loaned_alloc_layout_t, z_loaned_shm_provider_t, z_owned_alloc_layout_t,
};

pub(crate) fn alloc_layout_new(
    this: &mut MaybeUninit<z_owned_alloc_layout_t>,
    provider: &'static z_loaned_shm_provider_t,
    size: usize,
) -> z_result_t {
    let mem_layout: MemoryLayout = match size.try_into() {
        Ok(mem_layout) => mem_layout,
        Err(e) => {
            crate::report_error!("{:?}", e);
            return Z_EINVAL;
        }
    };

    let layout = match provider.as_rust_type_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => {
            match provider.alloc(mem_layout).into_layout() {
                Ok(layout) => CSHMLayout::Posix(layout),
                Err(e) => {
                    crate::report_error!("{:?}", e);
                    return Z_EINVAL;
                }
            }
        }
        super::shm_provider::CSHMProvider::Dynamic(provider) => {
            match provider.alloc(mem_layout).into_layout() {
                Ok(layout) => CSHMLayout::Dynamic(layout),
                Err(e) => {
                    crate::report_error!("{:?}", e);
                    return Z_EINVAL;
                }
            }
        }
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => {
            match provider.alloc(mem_layout).into_layout() {
                Ok(layout) => CSHMLayout::DynamicThreadsafe(layout),
                Err(e) => {
                    crate::report_error!("{:?}", e);
                    return Z_EINVAL;
                }
            }
        }
    };
    this.as_rust_type_mut_uninit().write(Some(layout));
    Z_OK
}

pub(crate) fn alloc_layout_with_alignment_new(
    this: &mut MaybeUninit<z_owned_alloc_layout_t>,
    provider: &'static z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_result_t {
    let mem_layout = match MemoryLayout::new(size, alignment.into_rust_type()) {
        Ok(layout) => layout,
        Err(e) => {
            crate::report_error!("{:?}", e);
            return Z_EINVAL;
        }
    };

    let layout = match provider.as_rust_type_ref() {
        super::shm_provider::CSHMProvider::Posix(provider) => {
            match provider.alloc(mem_layout).into_layout() {
                Ok(layout) => CSHMLayout::Posix(layout),
                Err(e) => {
                    crate::report_error!("{:?}", e);
                    return Z_EINVAL;
                }
            }
        }
        super::shm_provider::CSHMProvider::Dynamic(provider) => {
            match provider.alloc(mem_layout).into_layout() {
                Ok(layout) => CSHMLayout::Dynamic(layout),
                Err(e) => {
                    crate::report_error!("{:?}", e);
                    return Z_EINVAL;
                }
            }
        }
        super::shm_provider::CSHMProvider::DynamicThreadsafe(provider) => {
            match provider.alloc(mem_layout).into_layout() {
                Ok(layout) => CSHMLayout::DynamicThreadsafe(layout),
                Err(e) => {
                    crate::report_error!("{:?}", e);
                    return Z_EINVAL;
                }
            }
        }
    };
    this.as_rust_type_mut_uninit().write(Some(layout));
    Z_OK
}

pub(crate) fn alloc<Policy: AllocPolicy>(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    let result = match layout.as_rust_type_ref() {
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
    out_result.write(result.into());
}

pub(crate) fn alloc_async<Policy: AsyncAllocPolicy>(
    out_result: &'static mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &'static z_loaned_alloc_layout_t,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(*mut c_void, &mut MaybeUninit<z_buf_alloc_result_t>),
) -> z_result_t {
    match layout.as_rust_type_ref() {
        super::alloc_layout::CSHMLayout::Posix(layout) => {
            alloc_async_impl::<Policy, PosixShmProviderBackend>(
                out_result,
                layout,
                result_context,
                result_callback,
            );
            Z_OK
        }
        super::alloc_layout::CSHMLayout::Dynamic(_) => Z_EINVAL,
        super::alloc_layout::CSHMLayout::DynamicThreadsafe(layout) => {
            alloc_async_impl::<Policy, DynamicShmProviderBackend<ThreadsafeContext>>(
                out_result,
                layout,
                result_context,
                result_callback,
            );
            Z_OK
        }
    }
}

pub fn alloc_async_impl<Policy: AsyncAllocPolicy, Backend: ShmProviderBackend + Send + Sync>(
    out_result: &'static mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &'static AllocLayout<'static, Backend, MemoryLayout>,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(*mut c_void, &mut MaybeUninit<z_buf_alloc_result_t>),
) {
    let result_context: ThreadsafeContext = result_context.into();
    zenoh_runtime::ZRuntime::Application.spawn(async move {
        let result = layout.alloc().with_policy::<Policy>().await;
        out_result.write(result.into());
        unsafe { (result_callback)(result_context.get(), out_result) };
    });
}
