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
use prebindgen_proc_macro::prebindgen;

use libc::c_void;
use zenoh::shm::{AllocLayout, BlockOn, Deallocate, Defragment, GarbageCollect, JustAlloc};

use super::{
    alloc_layout_impl::{alloc, alloc_async, alloc_layout_new, alloc_layout_with_alignment_new},
    shm_provider_backend::DynamicShmProviderBackend,
    types::{z_alloc_alignment_t, z_buf_alloc_result_t},
};
use crate::{
    context::{zc_threadsafe_context_t, Context, ThreadsafeContext},
    result::z_result_t,
    shm::protocol_implementations::posix::posix_shm_provider::PosixAllocLayout,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_loaned_alloc_layout_t, z_loaned_shm_provider_t, z_moved_alloc_layout_t,
    z_owned_alloc_layout_t,
};

pub type DynamicAllocLayout = AllocLayout<'static, DynamicShmProviderBackend<Context>>;

pub type DynamicAllocLayoutThreadsafe =
    AllocLayout<'static, DynamicShmProviderBackend<ThreadsafeContext>>;

pub enum CSHMLayout {
    Posix(PosixAllocLayout),
    Dynamic(DynamicAllocLayout),
    DynamicThreadsafe(DynamicAllocLayoutThreadsafe),
}

decl_c_type!(
    owned(z_owned_alloc_layout_t, z_moved_alloc_layout_t,
        option CSHMLayout),
    loaned(z_loaned_alloc_layout_t),
);

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new Alloc Layout for SHM Provider.
#[prebindgen]
pub fn z_alloc_layout_new(
    this: &mut MaybeUninit<z_owned_alloc_layout_t>,
    provider: &'static z_loaned_shm_provider_t,
    size: usize,
) -> z_result_t {
    alloc_layout_new(this, provider, size)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new Alloc Layout for SHM Provider specifying the exact alignment.
#[prebindgen]
pub fn z_alloc_layout_with_alignment_new(
    this: &mut MaybeUninit<z_owned_alloc_layout_t>,
    provider: &'static z_loaned_shm_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_result_t {
    alloc_layout_with_alignment_new(this, provider, size, alignment)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs Alloc Layout in its gravestone value.
#[prebindgen]
pub fn z_internal_alloc_layout_null(this_: &mut MaybeUninit<z_owned_alloc_layout_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if `this` is valid.
#[prebindgen]
pub fn z_internal_alloc_layout_check(this_: &z_owned_alloc_layout_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows Alloc Layout.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_alloc_layout_loan(
    this: &z_owned_alloc_layout_t,
) -> &z_loaned_alloc_layout_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Moves Alloc Layout.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_alloc_layout_move(
    this: &mut z_owned_alloc_layout_t,
) -> &mut z_moved_alloc_layout_t {
    std::mem::transmute(this)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deletes Alloc Layout.
#[prebindgen]
pub fn z_alloc_layout_drop(this_: &mut z_moved_alloc_layout_t) {
    let _ = this_.take_rust_type();
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation without any additional actions.
#[prebindgen]
pub fn z_alloc_layout_alloc(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    alloc::<JustAlloc>(out_result, layout);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation performing garbage collection if needed.
#[prebindgen]
pub fn z_alloc_layout_alloc_gc(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    alloc::<GarbageCollect>(out_result, layout);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation performing garbage collection and/or defragmentation if needed.
#[prebindgen]
pub fn z_alloc_layout_alloc_gc_defrag(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    alloc::<Defragment<GarbageCollect>>(out_result, layout);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation performing garbage collection and/or defragmentation and/or forced deallocation if needed.
#[prebindgen]
pub fn z_alloc_layout_alloc_gc_defrag_dealloc(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    alloc::<Deallocate<100, Defragment<GarbageCollect>>>(out_result, layout);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation performing garbage collection and/or defragmentation and/or blocking if needed.
#[prebindgen]
pub fn z_alloc_layout_alloc_gc_defrag_blocking(
    out_result: &mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    alloc::<BlockOn<Defragment<GarbageCollect>>>(out_result, layout);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Make allocation performing garbage collection and/or defragmentation in async manner. Will return Z_EINVAL
/// if used with non-threadsafe SHM Provider.
#[prebindgen]
pub fn z_alloc_layout_threadsafe_alloc_gc_defrag_async(
    out_result: &'static mut MaybeUninit<z_buf_alloc_result_t>,
    layout: &'static z_loaned_alloc_layout_t,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(*mut c_void, &mut MaybeUninit<z_buf_alloc_result_t>),
) -> z_result_t {
    alloc_async::<BlockOn<Defragment<GarbageCollect>>>(
        out_result,
        layout,
        result_context,
        result_callback,
    )
}
