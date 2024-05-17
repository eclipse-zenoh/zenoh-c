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

use crate::{
    context::ThreadsafeContext,
    errors::z_error_t,
    shm::protocol_implementations::posix::posix_shared_memory_provider::PosixAllocLayout,
    transmute::{
        unwrap_ref_unchecked, Inplace, TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
    },
    z_owned_buf_alloc_result_t,
};
use zenoh::shm::{
    AllocLayout, BlockOn, Deallocate, Defragment, DynamicProtocolID, GarbageCollect, JustAlloc,
};

use crate::context::Context;

use super::{
    alloc_layout_impl::{alloc, alloc_layout_new},
    shared_memory_provider::z_loaned_shared_memory_provider_t,
    shared_memory_provider_backend::DynamicSharedMemoryProviderBackend,
    types::z_alloc_alignment_t,
};

/// A loaned SharedMemoryProvider's AllocLayout
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_loaned_alloc_layout_t([u64; 5]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_loaned_alloc_layout_t([u64; 6]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_loaned_alloc_layout_t([u64; 5]);

/// An owned SharedMemoryProvider's AllocLayout
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_owned_alloc_layout_t([u64; 5]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_owned_alloc_layout_t([u64; 6]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_owned_alloc_layout_t([u64; 5]);

pub type DynamicAllocLayout =
    AllocLayout<'static, DynamicProtocolID, DynamicSharedMemoryProviderBackend<Context>>;

pub type DynamicAllocLayoutThreadsafe =
    AllocLayout<'static, DynamicProtocolID, DynamicSharedMemoryProviderBackend<ThreadsafeContext>>;

pub enum CSHMLayout {
    Posix(PosixAllocLayout),
    Dynamic(DynamicAllocLayout),
    DynamicThreadsafe(DynamicAllocLayoutThreadsafe),
}

decl_transmute_owned!(Option<CSHMLayout>, z_owned_alloc_layout_t);

decl_transmute_handle!(CSHMLayout, z_loaned_alloc_layout_t);

/// Creates a new Alloc Layout for SHM Provider
#[no_mangle]
pub extern "C" fn z_alloc_layout_new(
    this: *mut MaybeUninit<z_owned_alloc_layout_t>,
    provider: &z_loaned_shared_memory_provider_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> z_error_t {
    alloc_layout_new(this, provider, size, alignment)
}

/// Constructs Alloc Layout in its gravestone value.
#[no_mangle]
pub extern "C" fn z_alloc_layout_null(this: *mut MaybeUninit<z_owned_alloc_layout_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_alloc_layout_check(this: &z_owned_alloc_layout_t) -> bool {
    this.transmute_ref().is_some()
}

/// Borrows Alloc Layout
#[no_mangle]
pub extern "C" fn z_alloc_layout_loan(this: &z_owned_alloc_layout_t) -> &z_loaned_alloc_layout_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

/// Deletes Alloc Layout
#[no_mangle]
pub extern "C" fn z_alloc_layout_drop(this: &mut z_owned_alloc_layout_t) {
    let _ = this.transmute_mut().extract();
}

#[no_mangle]
pub extern "C" fn z_alloc_layout_alloc(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    alloc::<JustAlloc>(out_result, layout);
}

#[no_mangle]
pub extern "C" fn z_alloc_layout_alloc_gc(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    alloc::<GarbageCollect>(out_result, layout);
}

#[no_mangle]
pub extern "C" fn z_alloc_layout_alloc_gc_defrag(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    alloc::<Defragment<GarbageCollect>>(out_result, layout);
}

#[no_mangle]
pub extern "C" fn z_alloc_layout_alloc_gc_defrag_dealloc(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    alloc::<Deallocate<100, Defragment<GarbageCollect>>>(out_result, layout);
}

#[no_mangle]
pub extern "C" fn z_alloc_layout_alloc_gc_defrag_blocking(
    out_result: *mut MaybeUninit<z_owned_buf_alloc_result_t>,
    layout: &z_loaned_alloc_layout_t,
) {
    alloc::<BlockOn<Defragment<GarbageCollect>>>(out_result, layout);
}

/*
#[no_mangle]
pub extern "C" fn z_alloc_layout_threadsafe_alloc_gc_defrag_async(
    out_result: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    layout: &'static z_loaned_alloc_layout_threadsafe_t,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    ),
) {
    let layout = layout.transmute_ref();
    let a = layout
    .alloc()
    .with_policy::<BlockOn<Defragment<GarbageCollect>>>();

    let result_context: ThreadsafeContext = result_context.into();
    //todo: this should be ported to tokio with executor argument support
    async_std::task::spawn(async move {
        let result = a.await;

        Inplace::init(
            (out_result as *mut MaybeUninit<z_owned_buf_alloc_result_t>).transmute_uninit_ptr(),
            Some(result),
        );

        (result_callback)(result_context.get(), out_result);
    });
}
*/
