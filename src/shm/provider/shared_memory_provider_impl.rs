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
use zenoh::prelude::*;
use zenoh::shm::{AllocPolicy, AsyncAllocPolicy, DynamicProtocolID, SharedMemoryProvider};

use crate::errors::{z_error_t, Z_EINVAL, Z_OK};
use crate::transmute::{Inplace, TransmuteUninitPtr};
use crate::{
    context::{DroppableContext, ThreadsafeContext},
    z_owned_buf_alloc_result_t,
};

use super::{
    shared_memory_provider_backend::DynamicSharedMemoryProviderBackend, types::z_alloc_alignment_t,
};

pub(crate) fn alloc<Policy: AllocPolicy, TAnyContext: DroppableContext>(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &SharedMemoryProvider<
        DynamicProtocolID,
        DynamicSharedMemoryProviderBackend<TAnyContext>,
    >,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    let result = provider
        .alloc(size)
        .with_alignment(alignment.into())
        .with_policy::<Policy>()
        .wait();

    parse_result(out_result, result)
}

pub(crate) async fn alloc_async<Policy: AsyncAllocPolicy>(
    out_result: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: &SharedMemoryProvider<
        DynamicProtocolID,
        DynamicSharedMemoryProviderBackend<ThreadsafeContext>,
    >,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    let result = provider
        .alloc(size)
        .with_alignment(alignment.into())
        .with_policy::<Policy>()
        .await;

    parse_result(out_result, result)
}

fn parse_result(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    result: BufLayoutAllocResult,
) -> z_error_t {
    match result {
        Ok(buf) => {
            Inplace::init(out_result.transmute_uninit_ptr(), Some(Ok(buf)));
            Z_OK
        }
        Err(ZLayoutAllocError::Alloc(e)) => {
            Inplace::init(out_result.transmute_uninit_ptr(), Some(Err(e)));
            Z_OK
        }
        Err(ZLayoutAllocError::Layout(_)) => Z_EINVAL,
    }
}
