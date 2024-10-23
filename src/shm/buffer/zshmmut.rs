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

use std::{borrow::Borrow, mem::MaybeUninit};

use zenoh::shm::{zshmmut, ZShmMut};

use crate::{
    result,
    transmute::{
        LoanedCTypeMut, LoanedCTypeRef, RustTypeMut, RustTypeMutUninit, RustTypeRef, TakeRustType,
    },
    z_loaned_shm_mut_t, z_moved_shm_mut_t, z_moved_shm_t, z_owned_shm_mut_t, z_owned_shm_t,
};

decl_c_type!(
    owned(z_owned_shm_mut_t, Option<ZShmMut>),
    loaned(z_loaned_shm_mut_t, zshmmut),
);

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Tries to obtain mutable SHM buffer instead of immutable one.
/// @param this_: mutable SHM buffer to be initialized upon success
/// @param that: immutable SHM buffer
/// @param immut: immutable SHM buffer returned back to caller's side
/// ONLY in case of Z_EUNAVAILABLE failure
/// @return Z_OK in case of success, Z_EUNAVAILABLE in case of unsuccessful write access,
/// Z_EINVAL if moved value is incorrect.
#[no_mangle]
pub extern "C" fn z_shm_mut_try_from_immut(
    this: &mut MaybeUninit<z_owned_shm_mut_t>,
    that: &mut z_moved_shm_t,
    immut: &mut MaybeUninit<z_owned_shm_t>,
) -> result::z_result_t {
    if let Some(shm) = that.take_rust_type() {
        return match ZShmMut::try_from(shm) {
            Ok(val) => {
                this.as_rust_type_mut_uninit().write(Some(val));
                result::Z_OK
            }
            Err(old) => {
                immut.as_rust_type_mut_uninit().write(Some(old));
                result::Z_EUNAVAILABLE
            }
        };
    }
    result::Z_EINVAL
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs ZShmMut slice in its gravestone value.
#[no_mangle]
pub extern "C" fn z_internal_shm_mut_null(this_: &mut MaybeUninit<z_owned_shm_mut_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @return ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_internal_shm_mut_check(this_: &z_owned_shm_mut_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows ZShmMut slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shm_mut_loan(this_: &z_owned_shm_mut_t) -> &z_loaned_shm_mut_t {
    let shmmut: &zshmmut = this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .borrow();
    shmmut.as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Mutably borrows ZShmMut slice.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shm_mut_loan_mut(
    this: &mut z_owned_shm_mut_t,
) -> &mut z_loaned_shm_mut_t {
    this.as_rust_type_mut().as_loaned_c_type_mut()
}

/// Takes ownership of the mutably borrowed shm_mut
#[no_mangle]
pub extern "C" fn z_shm_mut_take_loaned(
    dst: &mut MaybeUninit<z_owned_shm_mut_t>,
    src: &mut z_loaned_shm_mut_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(std::mem::take(src.as_rust_type_mut()));
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Deletes ZShmMut slice.
#[no_mangle]
pub extern "C" fn z_shm_mut_drop(this_: &mut z_moved_shm_mut_t) {
    let _ = this_.take_rust_type();
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @return the length of the ZShmMut slice.
#[no_mangle]
pub extern "C" fn z_shm_mut_len(this_: &z_loaned_shm_mut_t) -> usize {
    this_.as_rust_type_ref().len()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @return the immutable pointer to the underlying data.
#[no_mangle]
pub extern "C" fn z_shm_mut_data(this_: &z_loaned_shm_mut_t) -> *const libc::c_uchar {
    let s = this_.as_rust_type_ref();
    s.as_ref().as_ptr()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @return the mutable pointer to the underlying data.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shm_mut_data_mut(this_: &mut z_loaned_shm_mut_t) -> *mut libc::c_uchar {
    this_
        .as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_mut_ptr()
}
