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
    access_owned_memory, client::shared_memory_client::z_owned_shared_memory_client_t,
    common::types::z_protocol_id_t, decl_rust_new_owned_type, impl_guarded_transmute,
    move_owned_memory, prepare_memory_to_init, GuardedTransmute,
};
use std::sync::Arc;

/// A loaned list of SharedMemoryClients
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct zc_loaned_shared_memory_client_list_t<'a>(&'a zc_owned_shared_memory_client_list_t);

/// An owned list of SharedMemoryClients
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(8))]
pub struct zc_owned_shared_memory_client_list_t([u64; 3]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct zc_owned_shared_memory_client_list_t([u64; 4]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct zc_owned_shared_memory_client_list_t([u64; 3]);

decl_rust_new_owned_type!(
    zenoh:(Option<Vec<(ProtocolID, Arc<dyn SharedMemoryClient>)>>),
    c:(zc_owned_shared_memory_client_list_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_shared_memory_client_list_new(
    out: &mut zc_owned_shared_memory_client_list_t,
) -> i32 {
    let out = prepare_memory_to_init!(out);
    *out = Some(Vec::default());
    0
}

/// Initializes a null memory for safe-to-drop value of 'zc_owned_shared_memory_client_list_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn zc_shared_memory_client_list_null(
    val: &mut zc_owned_shared_memory_client_list_t,
) {
    val.make_null();
}

/// Returns ``true`` if `val` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn zc_shared_memory_client_list_check(
    val: &zc_owned_shared_memory_client_list_t,
) -> bool {
    val.check()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_shared_memory_client_list_delete(
    val: &mut zc_owned_shared_memory_client_list_t,
) {
    val.delete();
}

/// Returns a :c:type:`zc_loaned_shared_memory_client_list_t` loaned from `list`.
#[no_mangle]
pub extern "C" fn zc_shared_memory_client_list_loan(
    list: &zc_owned_shared_memory_client_list_t,
) -> zc_loaned_shared_memory_client_list_t {
    zc_loaned_shared_memory_client_list_t(list)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_shared_memory_client_list_add_client(
    id: z_protocol_id_t,
    client: &mut z_owned_shared_memory_client_t,
    list: zc_loaned_shared_memory_client_list_t,
) -> i32 {
    access_owned_memory!(list.0, |list: &mut Vec<_>| {
        move_owned_memory!(client, |client| {
            list.push((id, client));
            0
        })
    })
}

/// A SharedMemoryClientStorage.
#[repr(C)]
pub struct z_owned_shared_memory_client_storage_t(usize);

decl_rust_new_owned_type!(
    zenoh:(Option<Arc<SharedMemoryClientStorage>>),
    c:(z_owned_shared_memory_client_storage_t)
);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_client_storage_global(
    out: &mut z_owned_shared_memory_client_storage_t,
) -> i32 {
    let out = prepare_memory_to_init!(out);
    *out = Some(GLOBAL_CLIENT_STORAGE.clone());
    0
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_owned_shared_memory_client_storage_new(
    out: &mut z_owned_shared_memory_client_storage_t,
    clients: zc_loaned_shared_memory_client_list_t,
) -> i32 {
    let out = prepare_memory_to_init!(out);
    access_owned_memory!(clients.0, |list: &Vec<_>| {
        if list.is_empty() {
            return -5; // todo: E_ARGUMENT_INVALID
        }
        let builder = SharedMemoryClientStorage::builder().with_clients(list);
        *out = Some(Arc::new(builder.build()));
        0
    })
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_owned_shared_memory_client_storage_new_with_default_client_set(
    out: &mut z_owned_shared_memory_client_storage_t,
    clients: zc_loaned_shared_memory_client_list_t,
) -> i32 {
    let out = prepare_memory_to_init!(out);
    access_owned_memory!(clients.0, |list: &Vec<_>| {
        if list.is_empty() {
            return -5; // todo: E_ARGUMENT_INVALID
        }
        let builder = SharedMemoryClientStorage::builder()
            .with_default_client_set()
            .with_clients(list);
        *out = Some(Arc::new(builder.build()));
        0
    })
}

/// Initializes a null memory for safe-to-drop value of 'z_owned_shared_memory_client_storage_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_shared_memory_client_storage_null(
    out: &mut z_owned_shared_memory_client_storage_t,
) {
    out.make_null();
}

/// Returns ``true`` if `storage` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_shared_memory_client_storage_check(
    storage: &z_owned_shared_memory_client_storage_t,
) -> bool {
    storage.check()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_memory_client_storage_deref(
    storage: &mut z_owned_shared_memory_client_storage_t,
) {
    storage.delete();
}
