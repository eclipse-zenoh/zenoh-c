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

use std::{mem::MaybeUninit, sync::Arc};

use zenoh::shm::{PosixShmClient, ShmClient};

use crate::{
    errors::{z_error_t, Z_OK},
    transmute2::RustTypeRefUninit,
    z_owned_shm_client_t,
};

/// Creates a new POSIX SHM Client
#[no_mangle]
pub extern "C" fn z_posix_shm_client_new(
    this: &mut MaybeUninit<z_owned_shm_client_t>,
) -> z_error_t {
    let client = Arc::new(PosixShmClient) as Arc<dyn ShmClient>;
    this.as_rust_type_mut_uninit().write(Some(client));
    Z_OK
}
