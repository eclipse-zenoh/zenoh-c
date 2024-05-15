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

use zenoh::shm::{
    protocol_implementations::posix::{
        posix_shared_memory_provider_backend::PosixSharedMemoryProviderBackend,
        protocol_id::POSIX_PROTOCOL_ID,
    },
    provider::shared_memory_provider::{
        SharedMemoryProvider, SharedMemoryProviderBuilder, StaticProtocolID,
    },
};

use crate::{
    decl_rust_copy_type, impl_guarded_transmute, provider::types::z_memory_layout_t,
    GuardedTransmute,
};

/// An owned SharedMemoryProvider with POSIX backend
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct z_posix_shared_memory_provider_t([u64; 26]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_posix_shared_memory_provider_t([u64; 26]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_posix_shared_memory_provider_t([u64; 26]);

type PosixSharedMemoryProvider =
    SharedMemoryProvider<StaticProtocolID<POSIX_PROTOCOL_ID>, PosixSharedMemoryProviderBackend>;

decl_rust_copy_type!(
    zenoh:(PosixSharedMemoryProvider),
    c:(z_posix_shared_memory_provider_t));

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_posix_shared_memory_provider_new(
    layout: &z_memory_layout_t,
    out_provider: &mut MaybeUninit<z_posix_shared_memory_provider_t>,
) -> bool {
    match PosixSharedMemoryProviderBackend::builder()
        .with_layout(layout.transmute_ref())
        .res()
    {
        Ok(backend) => {
            let provider = SharedMemoryProviderBuilder::builder()
                .protocol_id::<POSIX_PROTOCOL_ID>()
                .backend(backend)
                .res();
            out_provider.write(provider.transmute());
            true
        }
        Err(e) => {
            log::error!("{}", e);
            false
        }
    }
}
