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

use std::{borrow::BorrowMut, mem::MaybeUninit};

use zenoh::shm::{zshmmut, ZShmMut};

use crate::{
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_loaned_shm_mut_t, z_owned_shm_mut_t, z_owned_shm_t,
};

decl_c_type!(
    owned(z_owned_shm_mut_t, Option<ZShmMut>),
    loaned(z_loaned_shm_mut_t, zshmmut)
);

/// Tries to construct ZShmMut slice from ZShm slice
#[no_mangle]
pub extern "C" fn z_shm_mut_try_from_immut(
    this: &mut MaybeUninit<z_owned_shm_mut_t>,
    that: &mut z_owned_shm_t,
) {
    let shm: Option<ZShmMut> = that
        .as_rust_type_mut()
        .take()
        .and_then(|val| val.try_into().ok());
    this.as_rust_type_mut_uninit().write(shm);
}

/// Constructs ZShmMut slice in its gravestone value.
#[no_mangle]
pub extern "C" fn z_shm_mut_null(this: &mut MaybeUninit<z_owned_shm_mut_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_shm_mut_check(this: &z_owned_shm_mut_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Borrows ZShmMut slice
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shm_mut_loan_mut(
    this: &mut z_owned_shm_mut_t,
) -> &mut z_loaned_shm_mut_t {
    let shmmut: &mut zshmmut = this
        .as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .borrow_mut();
    shmmut.as_loaned_c_type_mut()
}

/// Deletes ZShmMut slice
#[no_mangle]
pub extern "C" fn z_shm_mut_drop(this: &mut z_owned_shm_mut_t) {
    *this.as_rust_type_mut() = None;
}

/// @return the length of the ZShmMut slice
#[no_mangle]
pub extern "C" fn z_shm_mut_len(this: &z_loaned_shm_mut_t) -> usize {
    this.as_rust_type_ref().len()
}

/// @return the mutable pointer of the ZShmMut slice
#[no_mangle]
pub extern "C" fn z_shm_mut_data_mut(this: &mut z_loaned_shm_mut_t) -> *mut libc::c_uchar {
    this.as_rust_type_mut().as_mut().as_mut_ptr()
}
