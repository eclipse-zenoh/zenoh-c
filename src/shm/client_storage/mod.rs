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

use zenoh::shm::{ProtocolID, ShmClient, ShmClientStorage, GLOBAL_CLIENT_STORAGE};

use super::common::types::z_protocol_id_t;
use crate::{
    result::{z_result_t, Z_EINVAL, Z_OK},
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_loaned_shm_client_storage_t, z_owned_shm_client_storage_t, z_owned_shm_client_t,
    zc_loaned_shm_client_list_t, zc_owned_shm_client_list_t,
};

decl_c_type!(
    owned(zc_owned_shm_client_list_t, Option<Vec<(ProtocolID, Arc<dyn ShmClient>)>>),
    loaned(zc_loaned_shm_client_list_t, Vec<(ProtocolID, Arc<dyn ShmClient>)>)
);

/// Creates a new empty list of SHM Clients
#[no_mangle]
pub extern "C" fn zc_shm_client_list_new(this: &mut MaybeUninit<zc_owned_shm_client_list_t>) {
    let client_list: Vec<(ProtocolID, Arc<dyn ShmClient>)> = Vec::default();
    this.as_rust_type_mut_uninit().write(Some(client_list));
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
pub extern "C" fn zc_shm_client_list_drop(this: &mut zc_owned_shm_client_list_t) {
    *this.as_rust_type_mut() = None;
}

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
    client: &mut z_owned_shm_client_t,
    list: &mut zc_loaned_shm_client_list_t,
) -> z_result_t {
    match client.as_rust_type_mut().take() {
        Some(client) => {
            list.as_rust_type_mut().push((id, client));
            Z_OK
        }
        None => Z_EINVAL,
    }
}

decl_c_type!(
    owned(z_owned_shm_client_storage_t, Option<Arc<ShmClientStorage>>),
    loaned(z_loaned_shm_client_storage_t, Arc<ShmClientStorage>),
);

#[no_mangle]
pub extern "C" fn z_ref_shm_client_storage_global(
    this: &mut MaybeUninit<z_owned_shm_client_storage_t>,
) {
    let global_client_storage = &*(GLOBAL_CLIENT_STORAGE.read());
    this.as_rust_type_mut_uninit()
        .write(Some(global_client_storage.clone()));
}

#[no_mangle]
pub extern "C" fn z_shm_client_storage_new_default(
    this: &mut MaybeUninit<z_owned_shm_client_storage_t>,
) {
    this.as_rust_type_mut_uninit().write(Some(Arc::new(
        ShmClientStorage::builder()
            .with_default_client_set()
            .build(),
    )));
}

#[no_mangle]
pub extern "C" fn z_shm_client_storage_new(
    this: &mut MaybeUninit<z_owned_shm_client_storage_t>,
    clients: &zc_loaned_shm_client_list_t,
    add_default_client_set: bool,
) -> z_result_t {
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

/// Performs a shallow copy of SHM Client Storage
#[no_mangle]
pub extern "C" fn z_shm_client_storage_clone(
    this: &mut MaybeUninit<z_owned_shm_client_storage_t>,
    from: &z_loaned_shm_client_storage_t,
) {
    this.as_rust_type_mut_uninit()
        .write(Some(from.as_rust_type_ref().clone()));
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
pub extern "C" fn z_shm_client_storage_drop(this: &mut z_owned_shm_client_storage_t) {
    *this.as_rust_type_mut() = None;
}

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
