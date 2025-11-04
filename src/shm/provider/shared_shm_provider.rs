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
use std::sync::Arc;

use crate::{
    shm::protocol_implementations::posix::posix_shm_provider::PosixShmProvider,
    shm::provider::shm_provider::CSHMProvider,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_loaned_shared_shm_provider_t, z_moved_shared_shm_provider_t, z_owned_shared_shm_provider_t,
    z_owned_shm_provider_t,
};

pub type SharedShmProvider = Arc<PosixShmProvider>;
decl_c_type!(
    owned(z_owned_shared_shm_provider_t, option SharedShmProvider),
    loaned(z_loaned_shared_shm_provider_t),
);

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs Shared SHM Provider in its gravestone value.
#[no_mangle]
pub extern "C" fn z_internal_shared_shm_provider_null(
    this_: &mut MaybeUninit<z_owned_shared_shm_provider_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_internal_shared_shm_provider_check(
    this_: &z_owned_shared_shm_provider_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows Shared SHM Provider.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shared_shm_provider_loan(
    this: &z_owned_shared_shm_provider_t,
) -> &z_loaned_shared_shm_provider_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deletes Shared SHM Provider.
#[no_mangle]
pub extern "C" fn z_shared_shm_provider_drop(this_: &mut z_moved_shared_shm_provider_t) {
    let _ = this_.take_rust_type();
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Performs a shallow copy of contained SHM provider. The resulting provider object is **semantically**
/// similar to threadsafe provider wrapped into C++'s shared_ptr.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_inner_shm_provider(
    out: &mut MaybeUninit<z_owned_shm_provider_t>,
    this: &z_loaned_shared_shm_provider_t,
) {
    let shared_provider = this.as_rust_type_ref().clone();
    out.as_rust_type_mut_uninit()
        .write(Some(CSHMProvider::SharedPosix(shared_provider)));
}
