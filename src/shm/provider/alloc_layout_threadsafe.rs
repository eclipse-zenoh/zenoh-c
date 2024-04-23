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

use libc::c_void;
use zenoh::shm::provider::shared_memory_provider::{
    AllocLayout, BlockOn, Deallocate, Defragment, DynamicProtocolID, GarbageCollect, JustAlloc,
};

use crate::{
    access_owned_memory, decl_rust_new_owned_type, impl_guarded_transmute, prepare_memory_to_init,
    zc_threadsafe_context_t, DroppableContext, GuardedTransmute, ThreadsafeContext,
};

use super::{
    shared_memory_provider_backend::DynamicSharedMemoryProviderBackend,
    types::z_owned_buf_alloc_result_t,
};

/// A loaned thread-safe SharedMemoryProvider's AllocLayout
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct z_alloc_layout_threadsafe_t<'a>(&'a z_owned_alloc_layout_threadsafe_t);

/// A thread-safe SharedMemoryProvider's AllocLayout
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_owned_alloc_layout_threadsafe_t([u64; 4]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_owned_alloc_layout_threadsafe_t([u64; 4]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_owned_alloc_layout_threadsafe_t([u64; 4]);

decl_rust_new_owned_type!(
    zenoh:(Option<AllocLayout<'static, DynamicProtocolID, DynamicSharedMemoryProviderBackend<ThreadsafeContext>>>),
    c:(z_owned_alloc_layout_threadsafe_t)
);

/// Initializes a null memory for safe-to-drop value of 'z_owned_alloc_layout_threadsafe_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_alloc_layout_threadsafe_null(
    out: &mut z_owned_alloc_layout_threadsafe_t,
) {
    out.make_null();
}

/// Returns ``true`` if `layout` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_alloc_layout_threadsafe_check(
    layout: &z_owned_alloc_layout_threadsafe_t,
) -> bool {
    layout.check()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_delete(
    layout: &mut z_owned_alloc_layout_threadsafe_t,
) {
    layout.delete();
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc(
    out_buffer: &mut z_owned_buf_alloc_result_t,
    layout: z_alloc_layout_threadsafe_t,
) -> i32 {
    access_owned_memory!(layout.0, |layout: &AllocLayout<_, _>| {
        let out_buffer = prepare_memory_to_init!(out_buffer);
        let buffer = layout.alloc().with_policy::<JustAlloc>().res();
        *out_buffer = Some(buffer);
        0
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc_gc(
    out_buffer: &mut z_owned_buf_alloc_result_t,
    layout: z_alloc_layout_threadsafe_t,
) -> i32 {
    access_owned_memory!(layout.0, |layout: &AllocLayout<_, _>| {
        let out_buffer = prepare_memory_to_init!(out_buffer);
        let buffer = layout.alloc().with_policy::<GarbageCollect>().res();
        *out_buffer = Some(buffer);
        0
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc_gc_defrag(
    out_buffer: &mut z_owned_buf_alloc_result_t,
    layout: z_alloc_layout_threadsafe_t,
) -> i32 {
    access_owned_memory!(layout.0, |layout: &AllocLayout<_, _>| {
        let out_buffer = prepare_memory_to_init!(out_buffer);
        let buffer = layout
            .alloc()
            .with_policy::<Defragment<GarbageCollect>>()
            .res();
        *out_buffer = Some(buffer);
        0
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc_gc_defrag_dealloc(
    out_buffer: &mut z_owned_buf_alloc_result_t,
    layout: z_alloc_layout_threadsafe_t,
) -> i32 {
    access_owned_memory!(layout.0, |layout: &AllocLayout<_, _>| {
        let out_buffer = prepare_memory_to_init!(out_buffer);
        let buffer = layout
            .alloc()
            .with_policy::<Deallocate<100, Defragment<GarbageCollect>>>()
            .res();
        *out_buffer = Some(buffer);
        0
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc_gc_defrag_blocking(
    out_buffer: &mut z_owned_buf_alloc_result_t,
    layout: z_alloc_layout_threadsafe_t,
) -> i32 {
    access_owned_memory!(layout.0, |layout: &AllocLayout<_, _>| {
        let out_buffer = prepare_memory_to_init!(out_buffer);
        let buffer = layout
            .alloc()
            .with_policy::<BlockOn<Defragment<GarbageCollect>>>()
            .res();
        *out_buffer = Some(buffer);
        0
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_alloc_layout_threadsafe_alloc_gc_defrag_async(
    out_buffer: &mut z_owned_buf_alloc_result_t,
    layout: z_alloc_layout_threadsafe_t,

    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(*mut c_void, &mut z_owned_buf_alloc_result_t),
) -> i32 {
    let transmuted_out_buffer = prepare_memory_to_init!(out_buffer);
    //todo: this should be ported to tokio with executor argument support
    access_owned_memory!(layout.0, |layout: &AllocLayout<_, _>| {
        async_std::task::spawn(async move {
            let buffer = layout
                .alloc()
                .with_policy::<BlockOn<Defragment<GarbageCollect>>>()
                .res_async()
                .await;
            *transmuted_out_buffer = Some(buffer);
            let result_context = result_context.transmute();
            (result_callback)(result_context.get(), out_buffer);
        });
        0
    })
}
