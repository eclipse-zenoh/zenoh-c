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
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_loaned_shm_client_storage_t, z_moved_shm_client_storage_t, z_moved_shm_client_t,
    z_owned_shm_client_storage_t, zc_loaned_shm_client_list_t, zc_moved_shm_client_list_t,
    zc_owned_shm_client_list_t,
};

decl_c_type!(
    owned(zc_owned_shm_client_list_t, option Vec<(ProtocolID, Arc<dyn ShmClient>)>),
    loaned(zc_loaned_shm_client_list_t),
);

/// @attention Unstable feature.
/// @brief Creates a new empty list of SHM Clients.
#[no_mangle]
pub extern "C" fn zc_shm_client_list_new(this_: &mut MaybeUninit<zc_owned_shm_client_list_t>) {
    let client_list: Vec<(ProtocolID, Arc<dyn ShmClient>)> = Vec::default();
    this_.as_rust_type_mut_uninit().write(Some(client_list));
}

/// @attention Unstable feature.
/// @brief Constructs SHM client list in its gravestone value.
#[no_mangle]
pub extern "C" fn zc_internal_shm_client_list_null(
    this_: &mut MaybeUninit<zc_owned_shm_client_list_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @attention Unstable feature.
/// @brief Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn zc_internal_shm_client_list_check(this_: &zc_owned_shm_client_list_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @attention Unstable feature.
/// @brief Deletes list of SHM Clients.
#[no_mangle]
pub extern "C" fn zc_shm_client_list_drop(this_: &mut zc_moved_shm_client_list_t) {
    let _ = this_.take_rust_type();
}

/// @attention Unstable feature.
/// @brief Borrows list of SHM Clients.
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

/// @attention Unstable feature.
/// @brief Mutably borrows list of SHM Clients.
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

/// @attention Unstable feature.
#[no_mangle]
pub extern "C" fn zc_shm_client_list_add_client(
    id: z_protocol_id_t,
    client: &mut z_moved_shm_client_t,
    list: &mut zc_loaned_shm_client_list_t,
) -> z_result_t {
    let Some(client) = client.take_rust_type() else {
        return Z_EINVAL;
    };
    list.as_rust_type_mut().push((id, client));
    Z_OK
}

decl_c_type!(
    owned(z_owned_shm_client_storage_t, option Arc<ShmClientStorage> ),
    loaned(z_loaned_shm_client_storage_t),
);

/// @attention Unstable feature.
#[no_mangle]
pub extern "C" fn z_ref_shm_client_storage_global(
    this: &mut MaybeUninit<z_owned_shm_client_storage_t>,
) {
    this.as_rust_type_mut_uninit()
        .write(Some(Arc::clone(&GLOBAL_CLIENT_STORAGE.read())));
}

/// @attention Unstable feature.
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

/// @attention Unstable feature.
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

/// @attention Unstable feature.
/// @brief Performs a shallow copy of SHM Client Storage.
#[no_mangle]
pub extern "C" fn z_shm_client_storage_clone(
    this: &mut MaybeUninit<z_owned_shm_client_storage_t>,
    from: &z_loaned_shm_client_storage_t,
) {
    this.as_rust_type_mut_uninit()
        .write(Some(from.as_rust_type_ref().clone()));
}

/// @attention Unstable feature.
/// Constructs SHM Client Storage in its gravestone value.
#[no_mangle]
pub extern "C" fn z_internal_shm_client_storage_null(
    this_: &mut MaybeUninit<z_owned_shm_client_storage_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @attention Unstable feature.
/// @return ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_internal_shm_client_storage_check(
    this_: &z_owned_shm_client_storage_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @attention Unstable feature.
/// @brief Derefs SHM Client Storage.
#[no_mangle]
pub extern "C" fn z_shm_client_storage_drop(this_: &mut z_moved_shm_client_storage_t) {
    let _ = this_.take_rust_type();
}

/// @attention Unstable feature.
/// @brief Borrows SHM Client Storage.
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
