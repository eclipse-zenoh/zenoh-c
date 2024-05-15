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
use zenoh::shm::{AllocLayout, AllocPolicy};

use crate::context::DroppableContext;
use crate::{
    transmute::{Inplace, TransmuteUninitPtr},
    z_owned_buf_alloc_result_t,
};

use super::shared_memory_provider_backend::DynamicSharedMemoryProviderBackend;

pub(crate) fn alloc<Policy: AllocPolicy, TContext: DroppableContext>(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    layout: &AllocLayout<'static, DynamicProtocolID, DynamicSharedMemoryProviderBackend<TContext>>,
) {
    let result = layout.alloc().with_policy::<Policy>().wait();
    Inplace::init(out_result.transmute_uninit_ptr(), Some(result));
}
