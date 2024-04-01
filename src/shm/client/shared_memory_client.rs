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
use std::{mem::MaybeUninit, sync::Arc};

use libc::c_void;
use zenoh::{
    shm::{
        client::{
            shared_memory_client::SharedMemoryClient, shared_memory_segment::SharedMemorySegment,
        },
        common::types::SegmentID,
    },
    Result,
};
use zenoh_util::core::bail;

use crate::{
    common::types::z_segment_id_t, decl_rust_copy_type, impl_guarded_transmute,
    zc_threadsafe_context_t, DroppableContext, GuardedTransmute, ThreadsafeContext,
};

use super::shared_memory_segment::{z_shared_memory_segment_t, DynamicSharedMemorySegment};

/// A callbacks for SharedMemoryClient
#[derive(Debug)]
#[repr(C)]
pub struct zc_shared_memory_client_callbacks_t {
    attach_fn: unsafe extern "C" fn(
        *mut c_void,
        z_segment_id_t,
        &mut MaybeUninit<z_shared_memory_segment_t>,
    ) -> bool,
}

/// A SharedMemoryClient
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_shared_memory_client_t([u64; 2]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_shared_memory_client_t([u64; 2]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_shared_memory_client_t([u64; 2]);

decl_rust_copy_type!(
    zenoh:(Box<dyn SharedMemoryClient>),
    c:(z_shared_memory_client_t)
);

#[derive(Debug)]
pub struct DynamicSharedMemoryClient {
    context: ThreadsafeContext,
    callbacks: zc_shared_memory_client_callbacks_t,
}

impl DynamicSharedMemoryClient {
    pub fn new(context: ThreadsafeContext, callbacks: zc_shared_memory_client_callbacks_t) -> Self {
        Self { context, callbacks }
    }
}

impl SharedMemoryClient for DynamicSharedMemoryClient {
    fn attach(&self, segment: SegmentID) -> Result<Arc<dyn SharedMemorySegment>> {
        let mut segment_data = MaybeUninit::uninit();
        unsafe {
            match (self.callbacks.attach_fn)(self.context.get(), segment, &mut segment_data) {
                true => Ok(Arc::new(DynamicSharedMemorySegment::new(
                    segment_data.assume_init(),
                ))),
                false => bail!("C Callback returned error"),
            }
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_client_new(
    context: zc_threadsafe_context_t,
    callbacks: zc_shared_memory_client_callbacks_t,
    out_client: &mut MaybeUninit<z_shared_memory_client_t>,
) {
    let client = Box::new(DynamicSharedMemoryClient::new(
        context.transmute(),
        callbacks,
    )) as Box<dyn SharedMemoryClient>;

    out_client.write(client.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_client_delete(client: z_shared_memory_client_t) {
    let _ = client.transmute();
}
