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
use prebindgen_proc_macro::prebindgen;
use std::{mem::MaybeUninit, num::TryFromIntError, sync::Arc};

use zenoh::shm::{AllocatedChunk, ChunkDescriptor, PtrInSegment};

use zenoh_ffi_opaque_types::opaque_types::{
    z_loaned_ptr_in_segment_t, z_moved_ptr_in_segment_t, z_owned_ptr_in_segment_t,
};

use crate::{
    context::{zc_threadsafe_context_t, ThreadsafeContext},
    shm::common::types::{z_chunk_id_t, z_segment_id_t},
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
};

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A ChunkDescriptor.
#[prebindgen]
#[repr(C)]
pub struct z_chunk_descriptor_t {
    segment: z_segment_id_t,
    chunk: z_chunk_id_t,
    len: usize,
}

impl TryFrom<z_chunk_descriptor_t> for ChunkDescriptor {
    type Error = TryFromIntError;
    fn try_from(value: z_chunk_descriptor_t) -> Result<Self, Self::Error> {
        Ok(Self::new(value.segment, value.chunk, value.len.try_into()?))
    }
}

impl From<&ChunkDescriptor> for z_chunk_descriptor_t {
    fn from(value: &ChunkDescriptor) -> Self {
        Self {
            segment: value.segment,
            chunk: value.chunk,
            len: value.len.get(),
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An AllocatedChunk.
#[prebindgen]
#[repr(C)]
pub struct z_allocated_chunk_t {
    descriptpr: z_chunk_descriptor_t,
    ptr: &'static mut z_moved_ptr_in_segment_t,
}

impl TryFrom<z_allocated_chunk_t> for AllocatedChunk {
    type Error = zenoh::Error;
    fn try_from(value: z_allocated_chunk_t) -> Result<Self, Self::Error> {
        let ptr = value.ptr.take_rust_type().ok_or("Ptr is not initialized")?;
        Ok(Self::new(value.descriptpr.try_into()?, ptr))
    }
}

decl_c_type!(
    owned(z_owned_ptr_in_segment_t, z_moved_ptr_in_segment_t,
        option PtrInSegment),
    loaned(z_loaned_ptr_in_segment_t),
);

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new data pointer in SHM Segment.
#[prebindgen]
pub fn z_ptr_in_segment_new(
    this: &mut MaybeUninit<z_owned_ptr_in_segment_t>,
    ptr: *mut u8,
    segment: zc_threadsafe_context_t,
) {
    let segment: ThreadsafeContext = segment.into();
    this.as_rust_type_mut_uninit()
        .write(Some(PtrInSegment::new(ptr, Arc::new(segment))));
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs data pointer in SHM Segment in its gravestone value.
#[prebindgen]
pub fn z_internal_ptr_in_segment_null(this_: &mut MaybeUninit<z_owned_ptr_in_segment_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if `this` is valid.
#[prebindgen]
pub fn z_internal_ptr_in_segment_check(this_: &z_owned_ptr_in_segment_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows data pointer in SHM Segment.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_ptr_in_segment_loan(this: &z_owned_ptr_in_segment_t) -> &z_loaned_ptr_in_segment_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Moves data pointer in SHM Segment.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_ptr_in_segment_move(
    this: &mut z_owned_ptr_in_segment_t,
) -> &mut z_moved_ptr_in_segment_t {
    std::mem::transmute(this)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deletes data pointer in SHM Segment.
#[prebindgen]
pub fn z_ptr_in_segment_drop(this_: &mut z_moved_ptr_in_segment_t) {
    let _ = this_.take_rust_type();
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Makes a shallow data pointer in SHM Segment copy.
#[prebindgen]
pub fn z_ptr_in_segment_clone(
    out: &mut MaybeUninit<z_owned_ptr_in_segment_t>,
    this_: &z_loaned_ptr_in_segment_t,
) {
    let this = this_.as_rust_type_ref();
    let copy = this.to_owned();
    out.as_rust_type_mut_uninit().write(Some(copy));
}
