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
    common::types::z_segment_id_t, decl_rust_new_owned_type, impl_guarded_transmute,
    prepare_memory_to_init, zc_threadsafe_context_t, DroppableContext, GuardedTransmute,
    ThreadsafeContext,
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

/// A loaned SharedMemoryClient
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct z_shared_memory_client_t<'a>(&'a z_owned_shared_memory_client_t);

/// An owned SharedMemoryClient
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_owned_shared_memory_client_t([u64; 2]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_owned_shared_memory_client_t([u64; 2]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_owned_shared_memory_client_t([u64; 2]);

decl_rust_new_owned_type!(
    zenoh:(Option<Arc<dyn SharedMemoryClient>>),
    c:(z_owned_shared_memory_client_t)
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
pub unsafe extern "C" fn z_owned_shared_memory_client_new(
    out_client: &mut z_owned_shared_memory_client_t,
    context: zc_threadsafe_context_t,
    callbacks: zc_shared_memory_client_callbacks_t,
) -> i32 {
    let out_client = prepare_memory_to_init!(out_client);

    let client = Arc::new(DynamicSharedMemoryClient::new(
        context.transmute(),
        callbacks,
    )) as Arc<dyn SharedMemoryClient>;

    *out_client = Some(client);
    0
}

/// Initializes a null memory for safe-to-drop value of 'z_owned_shared_memory_client_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_owned_shared_memory_client_null(
    out_client: &mut z_owned_shared_memory_client_t,
) {
    out_client.make_null();
}

/// Returns ``true`` if `client` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_owned_shared_memory_client_check(
    client: &z_owned_shared_memory_client_t,
) -> bool {
    client.check()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_owned_shared_memory_client_delete(
    out_client: &mut z_owned_shared_memory_client_t,
) {
    out_client.delete();
}

/// Returns a :c:type:`z_shared_memory_client_t` loaned from `client`.
#[no_mangle]
pub extern "C" fn z_owned_shared_memory_client_loan(
    client: &z_owned_shared_memory_client_t,
) -> z_shared_memory_client_t {
    z_shared_memory_client_t(client)
}
