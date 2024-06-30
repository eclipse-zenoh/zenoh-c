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

use std::{
    borrow::{Borrow, BorrowMut},
    mem::MaybeUninit,
};

use zenoh::shm::{zshm, zshmmut, ZShm};

use crate::{
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_loaned_shm_mut_t, z_loaned_shm_t, z_moved_shm_t, z_owned_shm_mut_t, z_owned_shm_t,
};

decl_c_type!(
    owned(z_owned_shm_t, z_moved_shm_t, Option<ZShm>),
    loaned(z_loaned_shm_t, zshm),
);

/// Constructs ZShm slice from ZShmMut slice
#[no_mangle]
pub extern "C" fn z_shm_from_mut(
    this: &mut MaybeUninit<z_owned_shm_t>,
    that: &mut z_owned_shm_mut_t,
) {
    let shm: Option<ZShm> = that.as_rust_type_mut().take().map(|val| val.into());
    this.as_rust_type_mut_uninit().write(shm);
}

/// Constructs ZShm slice in its gravestone value.
#[no_mangle]
pub extern "C" fn z_shm_null(this: &mut MaybeUninit<z_owned_shm_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_shm_check(this: &z_owned_shm_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Converts borrowed ZShm slice to owned ZShm slice by performing a shallow SHM reference copy
#[no_mangle]
pub extern "C" fn z_shm_clone(this: &z_loaned_shm_t, out: &mut MaybeUninit<z_owned_shm_t>) {
    let this = this.as_rust_type_ref();
    let copy = this.to_owned();
    out.as_rust_type_mut_uninit().write(Some(copy));
}

/// Borrows ZShm slice
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shm_loan(this: &z_owned_shm_t) -> &z_loaned_shm_t {
    let this: &zshm = this.as_rust_type_ref().as_ref().unwrap_unchecked().borrow();
    this.as_loaned_c_type_ref()
}

/// Mutably borrows ZShm slice
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shm_loan_mut(this: &mut z_owned_shm_t) -> &mut z_loaned_shm_t {
    let this: &mut zshm = this
        .as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .borrow_mut();
    this.as_loaned_c_type_mut()
}

/// Mutably borrows ZShm slice as borrowed ZShmMut slice
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_shm_try_mut(this: &mut z_owned_shm_t) -> *mut z_loaned_shm_mut_t {
    let this = this.as_rust_type_mut();
    let this: &mut ZShm = this.as_mut().unwrap_unchecked();
    let shm: &mut zshm = this.borrow_mut();
    match shm.try_into() {
        Ok(val) => {
            let v: &mut zshmmut = val;
            v.as_loaned_c_type_mut()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Deletes ZShm slice
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_shm_drop(this: z_moved_shm_t) {}

/// Tries to reborrow mutably-borrowed ZShm slice as borrowed ZShmMut slice
#[no_mangle]
pub extern "C" fn z_shm_try_reloan_mut(this: &mut z_loaned_shm_t) -> *mut z_loaned_shm_mut_t {
    let this = this.as_rust_type_mut();
    match this.try_into() {
        Ok(val) => {
            let v: &mut zshmmut = val;
            v.as_loaned_c_type_mut()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// @return the length of the ZShm slice
#[no_mangle]
pub extern "C" fn z_shm_len(this: &z_loaned_shm_t) -> usize {
    this.as_rust_type_ref().len()
}

/// @return the pointer of the ZShm slice
#[no_mangle]
pub extern "C" fn z_shm_data(this: &z_loaned_shm_t) -> *const libc::c_uchar {
    let s = this.as_rust_type_ref();
    s.as_ref().as_ptr()
}
