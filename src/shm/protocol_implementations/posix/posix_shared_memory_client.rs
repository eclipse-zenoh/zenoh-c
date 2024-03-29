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

use zenoh::shm::{
    client::shared_memory_client::SharedMemoryClient,
    protocol_implementations::posix::posix_shared_memory_client::PosixSharedMemoryClient,
};

use crate::{client::shared_memory_client::z_shared_memory_client_t, GuardedTransmute};

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_posix_shared_memory_client_new() -> z_shared_memory_client_t {
    let client = Box::new(PosixSharedMemoryClient) as Box<dyn SharedMemoryClient>;
    client.transmute()
}
