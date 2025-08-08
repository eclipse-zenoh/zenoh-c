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
use prebindgen_proc_macro::prebindgen;

use libc::c_void;
use zenoh::{
    internal::bail,
    shm::{ProtocolID, SegmentID, ShmClient, ShmSegment, WithProtocolID},
    Result,
};

use super::shm_segment::{z_shm_segment_t, DynamicShmSegment};
pub use crate::opaque_types::{z_moved_shm_client_t, z_owned_shm_client_t};
use crate::{
    context::{zc_threadsafe_context_t, DroppableContext, ThreadsafeContext},
    shm::common::types::{z_protocol_id_t, z_segment_id_t},
    transmute::{RustTypeRef, RustTypeRefUninit, TakeRustType},
};

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Callback for ShmClient.
#[prebindgen]
#[derive(Debug)]
#[repr(C)]
pub struct zc_shm_client_callbacks_t {
    /// Attach to particular shared memory segment
    attach_fn: unsafe extern "C" fn(
        out_segment: &mut MaybeUninit<z_shm_segment_t>,
        segment_id: z_segment_id_t,
        context: *mut c_void,
    ) -> bool,

    /// ID of SHM Protocol this client implements
    id_fn: unsafe extern "C" fn(context: *mut c_void) -> z_protocol_id_t,
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

impl WithProtocolID for DynamicShmClient {
    fn id(&self) -> ProtocolID {
        unsafe { (self.callbacks.id_fn)(self.context.get()) }
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

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new SHM Client.
#[prebindgen]
pub fn z_shm_client_new(
    this: &mut MaybeUninit<z_owned_shm_client_t>,
    context: zc_threadsafe_context_t,
    callbacks: zc_shm_client_callbacks_t,
) {
    let client = Arc::new(DynamicShmClient::new(context.into(), callbacks)) as Arc<dyn ShmClient>;
    this.as_rust_type_mut_uninit().write(Some(client));
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs SHM client in its gravestone value.
#[prebindgen]
pub fn z_internal_shm_client_null(this_: &mut MaybeUninit<z_owned_shm_client_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @return Returns ``true`` if `this` is valid.
#[prebindgen]
pub fn z_internal_shm_client_check(this_: &z_owned_shm_client_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deletes SHM Client.
#[prebindgen]
pub fn z_shm_client_drop(this_: &mut z_moved_shm_client_t) {
    let _ = this_.take_rust_type();
}
