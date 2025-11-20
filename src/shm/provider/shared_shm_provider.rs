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

use crate::{
    shm::provider::shm_provider::CSHMProvider,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_loaned_shared_shm_provider_t, z_loaned_shm_provider_t, z_moved_shared_shm_provider_t,
    z_owned_shared_shm_provider_t,
};

pub struct SharedShmProvider(pub CSHMProvider);

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

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// Constructs a shallow copy of shared SHM provider.
#[no_mangle]
extern "C" fn z_shared_shm_provider_clone(
    dst: &mut MaybeUninit<z_owned_shared_shm_provider_t>,
    this: &z_loaned_shared_shm_provider_t,
) {
    match &this.as_rust_type_ref().0 {
        CSHMProvider::SharedPosix(provider) => {
            dst.as_rust_type_mut_uninit().write(Some(SharedShmProvider(
                CSHMProvider::SharedPosix(provider.clone()),
            )));
        }
        _ => {
            unreachable!("Unsupported SHM provider variant");
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Loan as SHM Provider. Provides access to the underlying SHM Provider to be used where SHM Provider is expected.
#[no_mangle]
pub extern "C" fn z_shared_shm_provider_loan_as(
    this: &z_loaned_shared_shm_provider_t,
) -> &z_loaned_shm_provider_t {
    &this.as_rust_type_ref().0.as_loaned_c_type_ref()
}
