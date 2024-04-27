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
    AllocPolicy, AsyncAllocPolicy, DynamicProtocolID, SharedMemoryProvider,
};

use crate::{DroppableContext, GuardedTransmute, ThreadsafeContext};

use super::{
    shared_memory_provider_backend::DynamicSharedMemoryProviderBackend,
    types::{z_alloc_alignment_t, z_owned_buf_alloc_result_t},
};

#[allow(clippy::missing_safety_doc)]
pub(crate) unsafe fn alloc<Policy: AllocPolicy, TAnyContext: DroppableContext>(
    provider: &SharedMemoryProvider<
        DynamicProtocolID,
        DynamicSharedMemoryProviderBackend<TAnyContext>,
    >,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
) -> i32 {
    match provider
        .alloc_layout()
        .size(size)
        .alignment(alignment.transmute())
        .res()
    {
        Ok(layout) => {
            let result = layout.alloc().with_policy::<Policy>().res();
            out_buffer.write(Some(result).transmute());
            0
        }
        Err(e) => {
            log::error!("{e}");
            -5 // todo: E_ARGUMENT_INVALID
        }
    }
}

#[allow(clippy::missing_safety_doc)]
pub(crate) async fn alloc_async<Policy: AsyncAllocPolicy>(
    provider: &SharedMemoryProvider<
        DynamicProtocolID,
        DynamicSharedMemoryProviderBackend<ThreadsafeContext>,
    >,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
) -> i32 {
    match provider
        .alloc_layout()
        .size(size)
        .alignment(alignment.transmute())
        .res()
    {
        Ok(layout) => {
            let result = layout.alloc().with_policy::<Policy>().res_async().await;
            out_buffer.write(Some(result).transmute());
            0
        }
        Err(e) => {
            log::error!("{e}");
            -5 // todo: E_ARGUMENT_INVALID
        }
    }
}
