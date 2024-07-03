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

use super::common::types::z_protocol_id_t;
use crate::{
    errors::{z_error_t, Z_EINVAL, Z_OK},
    transmute::{IntoRustType, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_loaned_shm_client_storage_t, z_moved_shm_client_storage_t, z_moved_shm_client_t,
    z_owned_shm_client_storage_t, z_owned_shm_client_t, zc_loaned_shm_client_list_t,
    zc_moved_shm_client_list_t, zc_owned_shm_client_list_t,
};
use std::{mem::MaybeUninit, sync::Arc};
use zenoh::shm::{ProtocolID, ShmClient, ShmClientStorage, GLOBAL_CLIENT_STORAGE};

decl_c_type!(
    owned(zc_owned_shm_client_list_t, option Vec<(ProtocolID, Arc<dyn ShmClient>)>),
    loaned(zc_loaned_shm_client_list_t),
    moved(zc_moved_shm_client_list_t)
);

/// Creates a new empty list of SHM Clients
#[no_mangle]
pub extern "C" fn zc_shm_client_list_new(
    this: &mut MaybeUninit<zc_owned_shm_client_list_t>,
) -> z_error_t {
    let client_list: Vec<(ProtocolID, Arc<dyn ShmClient>)> = Vec::default();
    this.as_rust_type_mut_uninit().write(Some(client_list));
    Z_OK
}

/// Constructs SHM client list in its gravestone value.
#[no_mangle]
pub extern "C" fn zc_shm_client_list_null(this: &mut MaybeUninit<zc_owned_shm_client_list_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn zc_shm_client_list_check(this: &zc_owned_shm_client_list_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Deletes list of SHM Clients
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn zc_shm_client_list_drop(this: zc_moved_shm_client_list_t) {}

/// Borrows list of SHM Clients
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_shm_client_list_loan(
    this: &zc_owned_shm_client_list_t,
) -> &zc_loaned_shm_client_list_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Mutably borrows list of SHM Clients
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_shm_client_list_loan_mut(
    this: &mut zc_owned_shm_client_list_t,
) -> &mut zc_loaned_shm_client_list_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

#[no_mangle]
pub extern "C" fn zc_shm_client_list_add_client(
    id: z_protocol_id_t,
    client: z_moved_shm_client_t,
    list: &mut zc_loaned_shm_client_list_t,
) -> z_error_t {
    let Some(client) = client.into_rust_type() else {
        return Z_EINVAL;
    };
    list.as_rust_type_mut().push((id, client));
    Z_OK
}

decl_c_type!(
    owned(z_owned_shm_client_storage_t, option Arc<ShmClientStorage> ),
    loaned(z_loaned_shm_client_storage_t),
    moved(z_moved_shm_client_storage_t)
);

#[no_mangle]
pub extern "C" fn z_ref_shm_client_storage_global(
    this: &mut MaybeUninit<z_owned_shm_client_storage_t>,
) -> z_error_t {
    this.as_rust_type_mut_uninit()
        .write(Some(GLOBAL_CLIENT_STORAGE.clone()));
    Z_OK
}

#[no_mangle]
pub extern "C" fn z_shm_client_storage_new_default(
    this: &mut MaybeUninit<z_owned_shm_client_storage_t>,
) -> z_error_t {
    this.as_rust_type_mut_uninit().write(Some(Arc::new(
        ShmClientStorage::builder()
            .with_default_client_set()
            .build(),
    )));
    Z_OK
}

#[no_mangle]
pub extern "C" fn z_shm_client_storage_new(
    this: &mut MaybeUninit<z_owned_shm_client_storage_t>,
    clients: &zc_loaned_shm_client_list_t,
    add_default_client_set: bool,
) -> z_error_t {
    let clients = clients.as_rust_type_ref();
    if clients.is_empty() {
        return Z_EINVAL;
    }

    let builder = match add_default_client_set {
        true => ShmClientStorage::builder()
            .with_default_client_set()
            .with_clients(clients),
        false => ShmClientStorage::builder().with_clients(clients),
    };
    this.as_rust_type_mut_uninit()
        .write(Some(Arc::new(builder.build())));
    Z_OK
}

/// Constructs SHM Client Storage in its gravestone value.
#[no_mangle]
pub extern "C" fn z_shm_client_storage_null(this: &mut MaybeUninit<z_owned_shm_client_storage_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_shm_client_storage_check(this: &z_owned_shm_client_storage_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Derefs SHM Client Storage
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_shm_client_storage_drop(this: z_moved_shm_client_storage_t) {}

/// Borrows SHM Client Storage
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shm_client_storage_loan(
    this: &z_owned_shm_client_storage_t,
) -> &z_loaned_shm_client_storage_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}
