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

use zenoh::shm::provider::types::{
    AllocAlignment, BufAllocResult, ChunkAllocResult, MemoryLayout, ZAllocError,
};
use zenoh_util::core::zerror;

use crate::{decl_rust_copy_type, decl_rust_new_owned_type, impl_guarded_transmute, move_owned_memory, GuardedTransmute};

use super::{chunk::z_allocated_chunk_t, zsliceshm::z_owned_slice_shm_mut_t};

/// Allocation errors
///
///     - **NEED_DEFRAGMENT**: defragmentation needed
///     - **OUT_OF_MEMORY**: the provider is out of memory
///     - **OTHER**: other error
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_alloc_error_t {
    NEED_DEFRAGMENT,
    OUT_OF_MEMORY,
    OTHER,
}

impl From<ZAllocError> for z_alloc_error_t {
    #[inline]
    fn from(value: ZAllocError) -> Self {
        match value {
            ZAllocError::NeedDefragment => z_alloc_error_t::NEED_DEFRAGMENT,
            ZAllocError::OutOfMemory => z_alloc_error_t::OUT_OF_MEMORY,
            ZAllocError::Other(_) => z_alloc_error_t::OTHER,
        }
    }
}

impl From<z_alloc_error_t> for ZAllocError {
    #[inline]
    fn from(value: z_alloc_error_t) -> Self {
        match value {
            z_alloc_error_t::NEED_DEFRAGMENT => ZAllocError::NeedDefragment,
            z_alloc_error_t::OUT_OF_MEMORY => ZAllocError::OutOfMemory,
            z_alloc_error_t::OTHER => ZAllocError::Other(zerror!("other error").into()),
        }
    }
}

// An AllocAlignment.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(4))]
pub struct z_alloc_alignment_t([u32; 1]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(4))]
pub struct z_alloc_alignment_t([u32; 1]);

#[cfg(target_arch = "arm")]
#[repr(C, align(4))]
pub struct z_alloc_alignment_t([u32; 1]);

decl_rust_copy_type!(
    zenoh:(AllocAlignment),
    c:(z_alloc_alignment_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_alignment_delete(alignment: z_alloc_alignment_t) {
    let _ = alignment.transmute();
}

// An owned MemoryLayout.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_memory_layout_t([u64; 2]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_memory_layout_t([u64; 2]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_memory_layout_t([u64; 2]);
decl_rust_copy_type!(
    zenoh:(MemoryLayout),
    c:(z_memory_layout_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_memory_layout_delete(layout: z_memory_layout_t) {
    let _ = layout.transmute();
}

// An owned ChunkAllocResult
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_chunk_alloc_result_t([u64; 4]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_chunk_alloc_result_t([u64; 4]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_chunk_alloc_result_t([u64; 4]);
decl_rust_copy_type!(
    zenoh:(ChunkAllocResult),
    c:(z_chunk_alloc_result_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_chunk_alloc_result_new_ok(
    allocated_chunk: z_allocated_chunk_t,
    out_result: &mut MaybeUninit<z_chunk_alloc_result_t>,
) {
    let allocated_chunk = allocated_chunk.transmute();
    let result = Ok(allocated_chunk);
    out_result.write(result.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_chunk_alloc_result_new_error(
    alloc_error: z_alloc_error_t,
    out_result: &mut MaybeUninit<z_chunk_alloc_result_t>,
) {
    let alloc_error = alloc_error.into();
    let result = Err(alloc_error);
    out_result.write(result.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_chunk_alloc_result_delete(result: z_chunk_alloc_result_t) {
    let _ = result.transmute();
}

/// A loaned BufAllocResult
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct z_buf_alloc_result_t<'a>(&'a z_owned_buf_alloc_result_t);

// A BufAllocResult
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_owned_buf_alloc_result_t([u64; 11]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_owned_buf_alloc_result_t([u64; 12]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_owned_buf_alloc_result_t([u64; 11]);
decl_rust_new_owned_type!(
    zenoh:(Option<BufAllocResult>),
    c:(z_owned_buf_alloc_result_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_owned_buf_alloc_result_unwrap(
    alloc_result: &mut z_owned_buf_alloc_result_t,
    out_buf: &mut MaybeUninit<z_owned_slice_shm_mut_t>,
    out_error: &mut MaybeUninit<z_alloc_error_t>,
) -> i32 {
    move_owned_memory!(alloc_result, |result: BufAllocResult| {
        match result {
            Ok(val) => {
                out_buf.write(Some(val).transmute());
                0
            }
            Err(e) => {
                out_error.write(e.into());
                -5 // todo: E_ARGUMENT_INVALID
            }
        }
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_owned_buf_alloc_result_delete(result: z_owned_buf_alloc_result_t) {
    let _ = result.transmute();
}
