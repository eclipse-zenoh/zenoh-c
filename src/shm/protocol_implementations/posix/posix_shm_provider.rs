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

use zenoh::{
    shm::{AllocLayout, PosixShmProviderBackend, ShmProvider, ShmProviderBuilder},
    Wait,
};

use crate::{
    result::{z_result_t, Z_EINVAL, Z_OK},
    shm::provider::shm_provider::CSHMProvider,
    transmute::{RustTypeRef, RustTypeRefUninit},
    z_loaned_memory_layout_t, z_owned_shm_provider_t,
};

pub type PosixShmProvider = ShmProvider<PosixShmProviderBackend>;

pub type PosixAllocLayout = AllocLayout<'static, PosixShmProviderBackend>;

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates a new POSIX SHM Provider.
#[prebindgen]
pub fn z_posix_shm_provider_new(
    this: &mut MaybeUninit<z_owned_shm_provider_t>,
    layout: &z_loaned_memory_layout_t,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    match PosixShmProviderBackend::builder()
        .with_layout(layout.as_rust_type_ref())
        .wait()
    {
        Ok(backend) => {
            let provider = ShmProviderBuilder::backend(backend).wait();
            this.write(Some(CSHMProvider::Posix(provider)));
            Z_OK
        }
        Err(e) => {
            this.write(None);
            tracing::error!("{}", e);
            Z_EINVAL
        }
    }
}
