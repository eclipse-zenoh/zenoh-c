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

use std::sync::Arc;

use zenoh::shm::{
    client::shared_memory_client::SharedMemoryClient,
    protocol_implementations::posix::posix_shared_memory_client::PosixSharedMemoryClient,
};

use crate::{
    client::shared_memory_client::z_owned_shared_memory_client_t, prepare_memory_to_init,
    GuardedTransmute,
};

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_posix_shared_memory_client_new(
    out_client: &mut z_owned_shared_memory_client_t,
) -> i32 {
    let out_client = prepare_memory_to_init!(out_client);
    let client = Arc::new(PosixSharedMemoryClient) as Arc<dyn SharedMemoryClient>;
    *out_client = Some(client);
    0
}
