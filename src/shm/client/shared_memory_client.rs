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
use zenoh::core::Result;
use zenoh::shm::{SegmentID, SharedMemoryClient, SharedMemorySegment};
use zenoh_util::core::bail;

use crate::context::DroppableContext;
use crate::transmute::TransmuteRef;
use crate::{
    context::{zc_threadsafe_context_t, ThreadsafeContext},
    errors,
    shm::common::types::z_segment_id_t,
    transmute::{Inplace, TransmuteUninitPtr},
};

pub use crate::opaque_types::z_owned_shared_memory_client_t;

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

decl_transmute_owned!(
    Option<Arc<dyn SharedMemoryClient>>,
    z_owned_shared_memory_client_t
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

/// Creates a new SHM Client
#[no_mangle]
pub extern "C" fn z_owned_shared_memory_client_new(
    this: *mut MaybeUninit<z_owned_shared_memory_client_t>,
    context: zc_threadsafe_context_t,
    callbacks: zc_shared_memory_client_callbacks_t,
) -> errors::z_error_t {
    let client = Arc::new(DynamicSharedMemoryClient::new(
        context.into(),
        callbacks,
    )) as Arc<dyn SharedMemoryClient>;

    Inplace::init(this.transmute_uninit_ptr(), Some(client));
    errors::Z_OK
}

/// Constructs SHM client in its gravestone value.
#[no_mangle]
pub extern "C" fn z_shared_memory_client_null(
    this: *mut MaybeUninit<z_owned_shared_memory_client_t>,
) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_owned_shared_memory_client_check(
    this: &z_owned_shared_memory_client_t,
) -> bool {
    this.transmute_ref().is_some()
}

/// Deletes SHM Client
#[no_mangle]
pub extern "C" fn z_shared_memory_client_delete(this: &mut z_owned_shared_memory_client_t) {
    let _ = this.transmute_mut().extract();
}
