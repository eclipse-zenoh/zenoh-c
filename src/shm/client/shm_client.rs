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
    internal::bail,
    shm::{SegmentID, ShmClient, ShmSegment},
    Result,
};

use super::shm_segment::{z_shm_segment_t, DynamicShmSegment};
pub use crate::opaque_types::{z_moved_shm_client_t, z_owned_shm_client_t};
use crate::{
    context::{zc_threadsafe_context_t, DroppableContext, ThreadsafeContext},
    shm::common::types::z_segment_id_t,
    transmute::{RustTypeRef, RustTypeRefUninit, TakeRustType},
};

/// @attention Unstable feature.
/// @brief Callback for ShmClient.
#[derive(Debug)]
#[repr(C)]
pub struct zc_shm_client_callbacks_t {
    attach_fn: unsafe extern "C" fn(
        out_segment: &mut MaybeUninit<z_shm_segment_t>,
        segment_id: z_segment_id_t,
        context: *mut c_void,
    ) -> bool,
}

decl_c_type!(
    owned(z_owned_shm_client_t, option Arc<dyn ShmClient>),
);

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

/// @attention Unstable feature.
/// @brief Creates a new SHM Client.
#[no_mangle]
pub extern "C" fn z_shm_client_new(
    this: &mut MaybeUninit<z_owned_shm_client_t>,
    context: zc_threadsafe_context_t,
    callbacks: zc_shm_client_callbacks_t,
) {
    let client = Arc::new(DynamicShmClient::new(context.into(), callbacks)) as Arc<dyn ShmClient>;
    this.as_rust_type_mut_uninit().write(Some(client));
}

/// @attention Unstable feature.
/// @brief Constructs SHM client in its gravestone value.
#[no_mangle]
pub extern "C" fn z_internal_shm_client_null(this_: &mut MaybeUninit<z_owned_shm_client_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @attention Unstable feature.
/// @return Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_internal_shm_client_check(this_: &z_owned_shm_client_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @attention Unstable feature.
/// @brief Deletes SHM Client.
#[no_mangle]
pub extern "C" fn z_shm_client_drop(this_: &mut z_moved_shm_client_t) {
    let _ = this_.take_rust_type();
}
