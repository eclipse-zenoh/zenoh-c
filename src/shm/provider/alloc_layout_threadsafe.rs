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
use zenoh::shm::provider::shared_memory_provider::{
    AllocLayout, BlockOn, Deallocate, Defragment, DynamicProtocolID, GarbageCollect, JustAlloc,
};

use crate::{
    decl_rust_copy_type, impl_guarded_transmute, zc_threadsafe_context_t, DroppableContext,
    GuardedTransmute, ThreadsafeContext,
};

use super::{
    shared_memory_provider_backend::DynamicSharedMemoryProviderBackend, types::z_buf_alloc_result_t,
};

/// A thread-safe SharedMemoryProvider's AllocLayout
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_alloc_layout_threadsafe_t([u64; 4]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_alloc_layout_threadsafe_t([u64; 14]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_alloc_layout_threadsafe_t([u64; 14]);

decl_rust_copy_type!(
    zenoh:(AllocLayout<'static, DynamicProtocolID, DynamicSharedMemoryProviderBackend<ThreadsafeContext>>),
    c:(z_alloc_layout_threadsafe_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_delete(layout: z_alloc_layout_threadsafe_t) {
    let _ = layout.transmute();
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc(
    layout: &z_alloc_layout_threadsafe_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) {
    let layout = layout.transmute_ref();
    let buffer = layout.alloc().with_policy::<JustAlloc>().res();
    out_buffer.write(buffer.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc_gc(
    layout: &z_alloc_layout_threadsafe_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) {
    let layout = layout.transmute_ref();
    let buffer = layout.alloc().with_policy::<GarbageCollect>().res();
    out_buffer.write(buffer.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc_gc_defrag(
    layout: &z_alloc_layout_threadsafe_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) {
    let layout = layout.transmute_ref();
    let buffer = layout
        .alloc()
        .with_policy::<Defragment<GarbageCollect>>()
        .res();
    out_buffer.write(buffer.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc_gc_defrag_dealloc(
    layout: &z_alloc_layout_threadsafe_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) {
    let layout = layout.transmute_ref();
    let buffer = layout
        .alloc()
        .with_policy::<Deallocate<100, Defragment<GarbageCollect>>>()
        .res();
    out_buffer.write(buffer.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc_gc_defrag_blocking(
    layout: &z_alloc_layout_threadsafe_t,
    out_buffer: &mut MaybeUninit<z_buf_alloc_result_t>,
) {
    let layout = layout.transmute_ref();
    let buffer = layout
        .alloc()
        .with_policy::<BlockOn<Defragment<GarbageCollect>>>()
        .res();
    out_buffer.write(buffer.transmute());
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc_gc_defrag_async(
    layout: &'static z_alloc_layout_threadsafe_t,
    out_buffer: &'static mut MaybeUninit<z_buf_alloc_result_t>,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(*mut c_void, &mut MaybeUninit<z_buf_alloc_result_t>),
) {
    let result_context = result_context.transmute();
    //todo: this should be ported to tokio with executor argument support
    async_std::task::block_on(async move {
        let layout = layout.transmute_ref();
        let buffer = layout
            .alloc()
            .with_policy::<BlockOn<Defragment<GarbageCollect>>>()
            .res_async()
            .await;
        out_buffer.write(buffer.transmute());

        (result_callback)(result_context.get(), out_buffer);
    });
}
