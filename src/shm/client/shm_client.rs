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
use zenoh::internal::bail;
use zenoh::shm::{SegmentID, ShmClient, ShmSegment};

use crate::context::DroppableContext;
use crate::transmute::TransmuteRef;
use crate::{
    context::{zc_threadsafe_context_t, ThreadsafeContext},
    errors,
    shm::common::types::z_segment_id_t,
    transmute::{Inplace, TransmuteUninitPtr},
};

pub use crate::opaque_types::z_owned_shm_client_t;

use super::shm_segment::{z_shm_segment_t, DynamicShmSegment};

/// A callbacks for ShmClient
#[derive(Debug)]
#[repr(C)]
pub struct zc_shm_client_callbacks_t {
    attach_fn: unsafe extern "C" fn(
        out_segment: &mut MaybeUninit<z_shm_segment_t>,
        segment_id: z_segment_id_t,
        context: *mut c_void,
    ) -> bool,
}

decl_transmute_owned!(Option<Arc<dyn ShmClient>>, z_owned_shm_client_t);

#[derive(Debug)]
pub struct DynamicShmClient {
    context: ThreadsafeContext,
    callbacks: zc_shm_client_callbacks_t,
}

impl DynamicShmClient {
    pub fn new(context: ThreadsafeContext, callbacks: zc_shm_client_callbacks_t) -> Self {
        Self { context, callbacks }
    }
}

impl ShmClient for DynamicShmClient {
    fn attach(&self, segment: SegmentID) -> Result<Arc<dyn ShmSegment>> {
        let mut segment_data = MaybeUninit::uninit();
        unsafe {
            match (self.callbacks.attach_fn)(&mut segment_data, segment, self.context.get()) {
                true => Ok(Arc::new(DynamicShmSegment::new(segment_data.assume_init()))),
                false => bail!("C Callback returned error"),
            }
        }
    }
}

/// Creates a new SHM Client
#[no_mangle]
pub extern "C" fn z_shm_client_new(
    this: *mut MaybeUninit<z_owned_shm_client_t>,
    context: zc_threadsafe_context_t,
    callbacks: zc_shm_client_callbacks_t,
) -> errors::z_error_t {
    let client = Arc::new(DynamicShmClient::new(context.into(), callbacks)) as Arc<dyn ShmClient>;

    Inplace::init(this.transmute_uninit_ptr(), Some(client));
    errors::Z_OK
}

/// Constructs SHM client in its gravestone value.
#[no_mangle]
pub extern "C" fn z_shm_client_null(this: *mut MaybeUninit<z_owned_shm_client_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_shm_client_check(this: &z_owned_shm_client_t) -> bool {
    this.transmute_ref().is_some()
}

/// Deletes SHM Client
#[no_mangle]
pub extern "C" fn z_shm_client_drop(this: &mut z_owned_shm_client_t) {
    let _ = this.transmute_mut().take();
}
