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
    types::{z_alloc_alignment_t, z_buf_alloc_result_t},
};

#[allow(clippy::missing_safety_doc)]
pub(crate) unsafe fn alloc<Policy: AllocPolicy, TAnyProvider, TAnyContext>(
    provider: &TAnyProvider,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool
where
    TAnyContext: DroppableContext,
    TAnyProvider: GuardedTransmute<
        SharedMemoryProvider<DynamicProtocolID, DynamicSharedMemoryProviderBackend<TAnyContext>>,
    >,
{
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

#[allow(clippy::missing_safety_doc)]
pub(crate) async fn alloc_async<Policy: AsyncAllocPolicy, TThreadsafeProvider>(
    provider: &TThreadsafeProvider,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) -> bool
where
    TThreadsafeProvider: GuardedTransmute<
        SharedMemoryProvider<
            DynamicProtocolID,
            DynamicSharedMemoryProviderBackend<ThreadsafeContext>,
        >,
    >,
{
    let provider = provider.transmute_ref();
    match provider
        .alloc_layout()
        .size(size)
        .alignment(alignment.transmute())
        .res()
    {
        Ok(layout) => {
            let result = layout
                .alloc()
                .with_policy::<Policy>()
                .res_async()
                .await
                .transmute();
            out_buffer.write(result);
            return true;
        }
        Err(e) => {
            log::error!("{e}");
        }
    };
    false
}
