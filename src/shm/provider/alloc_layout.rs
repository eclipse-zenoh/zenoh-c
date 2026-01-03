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

use libc::c_void;
use prebindgen_proc_macro::prebindgen;

use crate::{
    context::zc_threadsafe_context_t,
    result::z_result_t,
    shm::provider::{
        precomputed_layout::{
            z_internal_precomputed_layout_check, z_internal_precomputed_layout_null,
            z_precomputed_layout_alloc, z_precomputed_layout_alloc_gc,
            z_precomputed_layout_alloc_gc_defrag, z_precomputed_layout_alloc_gc_defrag_blocking,
            z_precomputed_layout_alloc_gc_defrag_dealloc, z_precomputed_layout_drop,
            z_precomputed_layout_loan, z_precomputed_layout_threadsafe_alloc_gc_defrag_async,
            z_shm_provider_alloc_layout, z_shm_provider_alloc_layout_aligned,
        },
        types::{z_alloc_alignment_t, z_buf_alloc_result_t},
    },
    z_loaned_precomputed_layout_t, z_loaned_shm_provider_t, z_moved_precomputed_layout_t,
    z_owned_precomputed_layout_t,
};

pub type z_owned_alloc_layout_t = z_owned_precomputed_layout_t;
pub type z_loaned_alloc_layout_t = z_loaned_precomputed_layout_t;
pub type z_moved_alloc_layout_t = z_moved_precomputed_layout_t;

/// @warning This API has been marked as deprecated, use `z_shm_provider_alloc_layout` instead.
#[prebindgen]
pub fn z_alloc_layout_new(
    this: &mut MaybeUninit<z_owned_alloc_layout_t>,
    provider: &'static z_loaned_shm_provider_t,
    size: usize,
) -> z_result_t {
    z_shm_provider_alloc_layout(this, provider, size)
}

/// @warning This API has been marked as deprecated, use `z_shm_provider_alloc_layout_aligned` instead.
#[prebindgen]
pub fn z_alloc_layout_with_alignment_new(
    this: &mut MaybeUninit<z_owned_alloc_layout_t>,
    provider: &'static z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_result_t {
    z_shm_provider_alloc_layout_aligned(this, provider, size, alignment)
}

/// @warning This API has been marked as deprecated, use `z_internal_precomputed_layout_null` instead.
#[prebindgen]
pub fn z_internal_alloc_layout_null(this_: &mut MaybeUninit<z_owned_alloc_layout_t>) {
    z_internal_precomputed_layout_null(this_)
}

/// @warning This API has been marked as deprecated, use `z_internal_precomputed_layout_check` instead.
#[prebindgen]
pub fn z_internal_alloc_layout_check(this_: &z_owned_alloc_layout_t) -> bool {
    z_internal_precomputed_layout_check(this_)
}

/// @warning This API has been marked as deprecated, use `z_precomputed_layout_loan` instead.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_alloc_layout_loan(this: &z_owned_alloc_layout_t) -> &z_loaned_alloc_layout_t {
    unsafe { z_precomputed_layout_loan(this) }
}

/// @warning This API has been marked as deprecated, use `z_precomputed_layout_drop` instead.
#[prebindgen]
pub fn z_alloc_layout_drop(this_: &mut z_moved_alloc_layout_t) {
    z_precomputed_layout_drop(this_)
}

/// @warning This API has been marked as deprecated, use `z_precomputed_layout_alloc` instead.
#[prebindgen]
pub fn z_alloc_layout_alloc(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    z_precomputed_layout_alloc(out_result, layout)
}

/// @warning This API has been marked as deprecated, use `z_precomputed_layout_alloc_gc` instead.
#[prebindgen]
pub fn z_alloc_layout_alloc_gc(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    z_precomputed_layout_alloc_gc(out_result, layout)
}

/// @warning This API has been marked as deprecated, use `z_precomputed_layout_alloc_gc_defrag` instead.
#[prebindgen]
pub fn z_alloc_layout_alloc_gc_defrag(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    z_precomputed_layout_alloc_gc_defrag(out_result, layout)
}

/// @warning This API has been marked as deprecated, use `z_precomputed_layout_alloc_gc_defrag_dealloc` instead.
#[prebindgen]
pub fn z_alloc_layout_alloc_gc_defrag_dealloc(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    z_precomputed_layout_alloc_gc_defrag_dealloc(out_result, layout)
}

/// @warning This API has been marked as deprecated, use `z_precomputed_layout_alloc_gc_defrag_blocking` instead.
#[prebindgen]
pub fn z_alloc_layout_alloc_gc_defrag_blocking(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    z_precomputed_layout_alloc_gc_defrag_blocking(out_result, layout)
}

/// @warning This API has been marked as deprecated, use `z_precomputed_layout_threadsafe_alloc_gc_defrag_async` instead.
/// @brief Make allocation performing garbage collection and/or defragmentation in async manner. Will return Z_EINVAL
/// if used with non-threadsafe SHM Provider.
#[prebindgen]
pub fn z_alloc_layout_threadsafe_alloc_gc_defrag_async(
    out_result: &'static mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &'static z_loaned_alloc_layout_t,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(*mut c_void, &mut MaybeUninit<z_buf_alloc_result_t>),
) -> z_result_t {
    z_precomputed_layout_threadsafe_alloc_gc_defrag_async(
        out_result,
        layout,
        result_context,
        result_callback,
    )
}
