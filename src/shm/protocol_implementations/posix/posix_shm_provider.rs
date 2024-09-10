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
    AllocLayout, PosixShmProviderBackend, ShmProvider, ShmProviderBuilder, StaticProtocolID,
    POSIX_PROTOCOL_ID,
};

use crate::{
    result::{z_result_t, Z_EINVAL, Z_OK},
    shm::provider::shm_provider::CSHMProvider,
    transmute::{RustTypeRef, RustTypeRefUninit},
    z_loaned_memory_layout_t, z_owned_shm_provider_t,
};

pub type PosixShmProvider =
    ShmProvider<StaticProtocolID<POSIX_PROTOCOL_ID>, PosixShmProviderBackend>;

pub type PosixAllocLayout =
    AllocLayout<'static, StaticProtocolID<POSIX_PROTOCOL_ID>, PosixShmProviderBackend>;

/// @attention Unstable feature.
/// @brief Creates a new POSIX SHM Provider.
#[no_mangle]
pub extern "C" fn z_posix_shm_provider_new(
    this: &mut MaybeUninit<z_owned_shm_provider_t>,
    layout: &z_loaned_memory_layout_t,
) -> z_result_t {
    match PosixShmProviderBackend::builder()
        .with_layout(layout.as_rust_type_ref())
        .res()
    {
        Ok(backend) => {
            let provider = ShmProviderBuilder::builder()
                .protocol_id::<POSIX_PROTOCOL_ID>()
                .backend(backend)
                .res();
            this.as_rust_type_mut_uninit()
                .write(Some(CSHMProvider::Posix(provider)));
            Z_OK
        }
        Err(e) => {
            tracing::error!("{}", e);
            Z_EINVAL
        }
    }
}
