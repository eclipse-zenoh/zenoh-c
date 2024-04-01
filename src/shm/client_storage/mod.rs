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

use zenoh::{
    shm::{
        client::shared_memory_client::SharedMemoryClient, client_storage::GLOBAL_CLIENT_STORAGE,
        common::types::ProtocolID,
    },
    SharedMemoryClientStorage,
};

use crate::{
    client::shared_memory_client::z_shared_memory_client_t, common::types::z_protocol_id_t,
    decl_rust_copy_type, impl_guarded_transmute, GuardedTransmute,
};
use std::{mem::MaybeUninit, sync::Arc};

/// A list of SharedMemoryClients.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct zc_shared_memory_client_list_t([u64; 3]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct zc_shared_memory_client_list_t([u64; 4]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct zc_shared_memory_client_list_t([u64; 3]);

decl_rust_copy_type!(
    zenoh:(Vec<(ProtocolID, Box<dyn SharedMemoryClient>)>),
    c:(zc_shared_memory_client_list_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_shared_memory_client_list_new() -> zc_shared_memory_client_list_t {
    let result = Vec::default();
    result.transmute()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_shared_memory_client_list_add_client(
    id: z_protocol_id_t,
    client: z_shared_memory_client_t,
    list: &mut zc_shared_memory_client_list_t,
) {
    list.transmute_mut().push((id, client.transmute()));
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_shared_memory_client_list_delete(list: zc_shared_memory_client_list_t) {
    let _ = list.transmute();
}

/// A SharedMemoryClientStorage.
#[repr(C)]
pub struct z_shared_memory_client_storage_t(usize);

decl_rust_copy_type!(
    zenoh:(Arc<SharedMemoryClientStorage>),
    c:(z_shared_memory_client_storage_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_client_storage_global() -> z_shared_memory_client_storage_t
{
    let result = GLOBAL_CLIENT_STORAGE.clone();
    result.transmute()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_client_storage_deref(
    storage: z_shared_memory_client_storage_t,
) {
    let _ = storage.transmute();
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_client_storage_new(
    clients: zc_shared_memory_client_list_t,
    out_storage: &mut MaybeUninit<z_shared_memory_client_storage_t>,
) -> bool {
    let mut clients = clients.transmute();

    if let Some((id, client)) = clients.pop() {
        let mut builder = SharedMemoryClientStorage::builder().with_client(id, client);

        for (id, client) in clients.drain(0..) {
            match builder.with_client(id, client) {
                Ok(b) => builder = b,
                Err(_) => return false,
            }
        }
        out_storage.write(Arc::new(builder.build()).transmute());
        return true;
    }
    false
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_client_storage_new_with_default_client_set(
    clients: zc_shared_memory_client_list_t,
    out_storage: &mut MaybeUninit<z_shared_memory_client_storage_t>,
) -> bool {
    let mut clients = clients.transmute();
    let mut builder = SharedMemoryClientStorage::builder().with_default_client_set();

    for (id, client) in clients.drain(0..) {
        match builder.with_client(id, client) {
            Ok(b) => builder = b,
            Err(_) => return false,
        }
    }
    out_storage.write(Arc::new(builder.build()).transmute());
    true
}
