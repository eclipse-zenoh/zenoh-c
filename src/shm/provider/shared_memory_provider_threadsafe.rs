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
    AllocPolicy, BlockOn, Deallocate, Defragment, DynamicProtocolID, GarbageCollect, JustAlloc,
    SharedMemoryProvider, SharedMemoryProviderBuilder,
};

use crate::{
    access_loaned_memory, access_owned_memory, common::types::z_protocol_id_t,
    decl_rust_new_owned_type, impl_guarded_transmute, zc_threadsafe_context_t, DroppableContext,
    GuardedTransmute, ThreadsafeContext,
};

use super::{
    alloc_layout_threadsafe::z_owned_alloc_layout_threadsafe_t,
    chunk::z_allocated_chunk_t,
    shared_memory_provider_backend::{
        zc_shared_memory_provider_backend_callbacks_t, DynamicSharedMemoryProviderBackend,
    },
    shared_memory_provider_impl::{alloc, alloc_async},
    types::{z_alloc_alignment_t, z_owned_buf_alloc_result_t},
    zsliceshm::z_owned_slice_shm_mut_t,
};

/// A loaned SharedMemoryProvider thread-safe specialization
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct z_loaned_shared_memory_provider_threadsafe_t<'a>(
    &'a z_owned_shared_memory_provider_threadsafe_t,
);

/// An owned SharedMemoryProvider thread-safe specialization
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_owned_shared_memory_provider_threadsafe_t([u64; 14]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_owned_shared_memory_provider_threadsafe_t([u64; 14]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_owned_shared_memory_provider_threadsafe_t([u64; 14]);

decl_rust_new_owned_type!(
    zenoh:(Option<SharedMemoryProvider<DynamicProtocolID, DynamicSharedMemoryProviderBackend<ThreadsafeContext>>>),
    c:(z_owned_shared_memory_provider_threadsafe_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_new(
    out_provider: &mut MaybeUninit<z_owned_shared_memory_provider_threadsafe_t>,
    id: z_protocol_id_t,
    context: zc_threadsafe_context_t,
    callbacks: zc_shared_memory_provider_backend_callbacks_t,
) {
    let backend = DynamicSharedMemoryProviderBackend::new(context.transmute(), callbacks);
    let provider = SharedMemoryProviderBuilder::builder()
        .dynamic_protocol_id(id)
        .backend(backend)
        .res();
    out_provider.write(Some(provider).transmute());
}

/// Initializes a null memory for safe-to-drop value of 'z_shared_memory_provider_threadsafe_' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_shared_memory_provider_threadsafe_null(
    val: &mut z_owned_shared_memory_provider_threadsafe_t,
) {
    val.make_null();
}

/// Returns ``true`` if `val` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_shared_memory_provider_threadsafe_check(
    val: &z_owned_shared_memory_provider_threadsafe_t,
) -> bool {
    val.check()
}

/// Returns a :c:type:`z_loaned_shared_memory_provider_threadsafe_t` loaned from `val`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_shared_memory_provider_threadsafe_loan(
    val: &z_owned_shared_memory_provider_threadsafe_t,
) -> z_loaned_shared_memory_provider_threadsafe_t {
    z_loaned_shared_memory_provider_threadsafe_t(val)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_shm_mut_delete(val: &mut z_owned_slice_shm_mut_t) {
    val.delete();
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_layout_new(
    out_layout: &mut MaybeUninit<z_owned_alloc_layout_threadsafe_t>,
    provider: z_loaned_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> i32 {
    access_loaned_memory!(provider, |val: &mut SharedMemoryProvider<_, _>| {
        match val
            .alloc_layout()
            .size(size)
            .alignment(alignment.transmute())
            .res()
        {
            Ok(layout) => {
                out_layout.write(Some(layout).transmute());
                0
            }
            Err(e) => {
                log::error!("{e}");
                return -5; // todo: E_ARGUMENT_INVALID
            }
        }
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc(
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: z_loaned_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> i32 {
    alloc_inner::<JustAlloc>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_gc(
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: z_loaned_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> i32 {
    alloc_inner::<GarbageCollect>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_gc_defrag(
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: z_loaned_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> i32 {
    alloc_inner::<Defragment<GarbageCollect>>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_gc_defrag_dealloc(
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: z_loaned_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> i32 {
    alloc_inner::<Deallocate<100, Defragment<GarbageCollect>>>(
        provider, size, alignment, out_buffer,
    )
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_gc_defrag_blocking(
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: z_loaned_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
) -> i32 {
    alloc_inner::<BlockOn<Defragment<GarbageCollect>>>(provider, size, alignment, out_buffer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_alloc_gc_defrag_async(
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
    provider: z_loaned_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    result_context: zc_threadsafe_context_t,
    result_callback: unsafe extern "C" fn(
        *mut c_void,
        &mut MaybeUninit<z_owned_buf_alloc_result_t>,
        i32,
    ),
) {
    let result_context = result_context.transmute();
    //todo: this should be ported to tokio with executor argument support
    async_std::task::spawn(async move {
        let result = match provider.0.transmute_ref() {
            Some(val) => {
                alloc_async::<BlockOn<Defragment<GarbageCollect>>>(val, size, alignment, out_buffer)
                    .await
            }
            None => -2, // todo: error type E_ACCESS_NULL
        };

        (result_callback)(result_context.get(), out_buffer, result);
    });
}

#[allow(clippy::missing_safety_doc)]
unsafe fn alloc_inner<Policy: AllocPolicy>(
    provider: z_loaned_shared_memory_provider_threadsafe_t,
    size: usize,
    alignment: z_alloc_alignment_t,
    out_buffer: &mut MaybeUninit<z_owned_buf_alloc_result_t>,
) -> i32 {
    access_loaned_memory!(provider, |val| {
        alloc::<Policy, ThreadsafeContext>(val, size, alignment, out_buffer)
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_defragment(
    provider: z_loaned_shared_memory_provider_threadsafe_t,
) -> i32 {
    access_loaned_memory!(provider, |val: &mut SharedMemoryProvider<_, _>| {
        let _ = val.defragment();
        0
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_garbage_collect(
    provider: z_loaned_shared_memory_provider_threadsafe_t,
) -> i32 {
    access_loaned_memory!(provider, |val: &mut SharedMemoryProvider<_, _>| {
        let _ = val.garbage_collect();
        0
    })
}

//#[no_mangle]
//#[allow(clippy::missing_safety_doc)]
//pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_available(
//    provider: z_loaned_shared_memory_provider_threadsafe_t,
//) -> i32 {
//    access_loaned_memory!(provider, |val: &mut SharedMemoryProvider<_, _>| {
//        val.available()
//    })
//}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_provider_threadsafe_map(
    provider: &z_owned_shared_memory_provider_threadsafe_t,
    allocated_chunk: z_allocated_chunk_t,
    len: usize,
    out_buffer: &mut MaybeUninit<z_owned_slice_shm_mut_t>,
) -> bool {
    let provider = provider.transmute_ref();
    match provider.map(allocated_chunk.transmute(), len) {
        Ok(buffer) => {
            out_buffer.write(buffer.transmute());
            true
        }
        Err(e) => {
            log::error!("{e}");
            false
        }
    }
}
