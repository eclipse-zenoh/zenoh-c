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

use zenoh::shm::{
    ProtocolID, SharedMemoryClient, SharedMemoryClientStorage, GLOBAL_CLIENT_STORAGE,
};

use crate::{
    errors::{z_error_t, Z_EINVAL, Z_OK},
    transmute::{
        unwrap_ref_unchecked, unwrap_ref_unchecked_mut, Inplace, TransmuteFromHandle,
        TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
    },
    z_loaned_shared_memory_client_storage_t, z_owned_shared_memory_client_storage_t,
    z_owned_shared_memory_client_t, zc_loaned_shared_memory_client_list_t,
    zc_owned_shared_memory_client_list_t,
};

use super::common::types::z_protocol_id_t;

decl_transmute_owned!(
    Option<Vec<(ProtocolID, Arc<dyn SharedMemoryClient>)>>,
    zc_owned_shared_memory_client_list_t
);

decl_transmute_handle!(
    Vec<(ProtocolID, Arc<dyn SharedMemoryClient>)>,
    zc_loaned_shared_memory_client_list_t
);

/// Creates a new empty list of SHM Clients
#[no_mangle]
pub extern "C" fn zc_shared_memory_client_list_new(
    this: *mut MaybeUninit<zc_owned_shared_memory_client_list_t>,
) -> z_error_t {
    let client_list: Vec<(ProtocolID, Arc<dyn SharedMemoryClient>)> = Vec::default();
    Inplace::init(this.transmute_uninit_ptr(), Some(client_list));
    Z_OK
}

/// Constructs SHM client list in its gravestone value.
#[no_mangle]
pub extern "C" fn zc_shared_memory_client_list_null(
    this: *mut MaybeUninit<zc_owned_shared_memory_client_list_t>,
) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn zc_shared_memory_client_list_check(
    this: &zc_owned_shared_memory_client_list_t,
) -> bool {
    this.transmute_ref().is_some()
}

/// Deletes list of SHM Clients
#[no_mangle]
pub extern "C" fn zc_shared_memory_client_list_drop(
    this: &mut zc_owned_shared_memory_client_list_t,
) {
    let _ = this.transmute_mut().take();
}

/// Borrows list of SHM Clients
#[no_mangle]
pub extern "C" fn zc_shared_memory_client_list_loan(
    this: &zc_owned_shared_memory_client_list_t,
) -> &zc_loaned_shared_memory_client_list_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

/// Mutably borrows list of SHM Clients
#[no_mangle]
pub extern "C" fn zc_shared_memory_client_list_loan_mut(
    this: &mut zc_owned_shared_memory_client_list_t,
) -> &mut zc_loaned_shared_memory_client_list_t {
    let this = this.transmute_mut();
    let this = unwrap_ref_unchecked_mut(this);
    this.transmute_handle_mut()
}

#[no_mangle]
pub extern "C" fn zc_shared_memory_client_list_add_client(
    id: z_protocol_id_t,
    client: &mut z_owned_shared_memory_client_t,
    list: &mut zc_loaned_shared_memory_client_list_t,
) -> z_error_t {
    match client.transmute_mut().extract() {
        Some(client) => {
            list.transmute_mut().push((id, client));
            Z_OK
        }
        None => Z_EINVAL,
    }
}

decl_transmute_owned!(
    Option<Arc<SharedMemoryClientStorage>>,
    z_owned_shared_memory_client_storage_t
);

decl_transmute_handle!(
    Arc<SharedMemoryClientStorage>,
    z_loaned_shared_memory_client_storage_t
);

#[no_mangle]
pub extern "C" fn z_ref_shared_memory_client_storage_global(
    this: *mut MaybeUninit<z_owned_shared_memory_client_storage_t>,
) -> z_error_t {
    Inplace::init(
        this.transmute_uninit_ptr(),
        Some(GLOBAL_CLIENT_STORAGE.clone()),
    );
    Z_OK
}

#[no_mangle]
pub extern "C" fn z_shared_memory_client_storage_new_default(
    this: *mut MaybeUninit<z_owned_shared_memory_client_storage_t>,
) -> z_error_t {
    Inplace::init(
        this.transmute_uninit_ptr(),
        Some(Arc::new(
            SharedMemoryClientStorage::builder()
                .with_default_client_set()
                .build(),
        )),
    );
    Z_OK
}

#[no_mangle]
pub extern "C" fn z_shared_memory_client_storage_new(
    this: *mut MaybeUninit<z_owned_shared_memory_client_storage_t>,
    clients: &zc_loaned_shared_memory_client_list_t,
    add_default_client_set: bool,
) -> z_error_t {
    let clients = clients.transmute_ref();
    if clients.is_empty() {
        return Z_EINVAL;
    }

    let builder = match add_default_client_set {
        true => SharedMemoryClientStorage::builder()
            .with_default_client_set()
            .with_clients(clients),
        false => SharedMemoryClientStorage::builder().with_clients(clients),
    };
    Inplace::init(this.transmute_uninit_ptr(), Some(Arc::new(builder.build())));
    Z_OK
}

/// Constructs SHM Client Storage in its gravestone value.
#[no_mangle]
pub extern "C" fn z_shared_memory_client_storage_null(
    this: *mut MaybeUninit<z_owned_shared_memory_client_storage_t>,
) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_shared_memory_client_storage_check(
    this: &z_owned_shared_memory_client_storage_t,
) -> bool {
    this.transmute_ref().is_some()
}

/// Derefs SHM Client Storage
#[no_mangle]
pub extern "C" fn z_shared_memory_client_storage_drop(
    this: &mut z_owned_shared_memory_client_storage_t,
) {
    let _ = this.transmute_mut().take();
}

/// Borrows SHM Client Storage
#[no_mangle]
pub extern "C" fn z_shared_memory_client_storage_loan(
    this: &z_owned_shared_memory_client_storage_t,
) -> &z_loaned_shared_memory_client_storage_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}
