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
    AllocLayout, PosixSharedMemoryProviderBackend, SharedMemoryProvider,
    SharedMemoryProviderBuilder, StaticProtocolID, POSIX_PROTOCOL_ID,
};

use crate::{
    errors::{z_error_t, Z_EINVAL, Z_OK},
    shm::provider::shared_memory_provider::{z_owned_shared_memory_provider_t, CSHMProvider},
    transmute::{Inplace, TransmuteFromHandle, TransmuteUninitPtr},
    z_loaned_memory_layout_t,
};

pub type PosixSharedMemoryProvider =
    SharedMemoryProvider<StaticProtocolID<POSIX_PROTOCOL_ID>, PosixSharedMemoryProviderBackend>;

pub type PosixAllocLayout =
    AllocLayout<'static, StaticProtocolID<POSIX_PROTOCOL_ID>, PosixSharedMemoryProviderBackend>;

/// Creates a new threadsafe SHM Provider
#[no_mangle]
pub extern "C" fn z_posix_shared_memory_provider_new(
    this: *mut MaybeUninit<z_owned_shared_memory_provider_t>,
    layout: &z_loaned_memory_layout_t,
) -> z_error_t {
    match PosixSharedMemoryProviderBackend::builder()
        .with_layout(layout.transmute_ref())
        .res()
    {
        Ok(backend) => {
            let provider = SharedMemoryProviderBuilder::builder()
                .protocol_id::<POSIX_PROTOCOL_ID>()
                .backend(backend)
                .res();
            Inplace::init(
                this.transmute_uninit_ptr(),
                Some(CSHMProvider::Posix(provider)),
            );
            Z_OK
        }
        Err(e) => {
            log::error!("{}", e);
            Z_EINVAL
        }
    }
}
