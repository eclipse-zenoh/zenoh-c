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
    transmute::{
        unwrap_ref_unchecked_mut, Inplace, TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
    },
    z_loaned_shm_mut_t, z_owned_shm_mut_t, z_owned_shm_t,
};

decl_transmute_owned!(Option<ZShmMut>, z_owned_shm_mut_t);

decl_transmute_handle!(zshmmut, z_loaned_shm_mut_t);

/// Tries to construct ZShmMut slice from ZShm slice
#[no_mangle]
pub extern "C" fn z_shm_mut_try_from_immut(
    this: *mut MaybeUninit<z_owned_shm_mut_t>,
    that: &mut z_owned_shm_t,
) {
    let shm: Option<ZShmMut> = that
        .transmute_mut()
        .extract()
        .and_then(|val| val.try_into().ok());
    Inplace::init(this.transmute_uninit_ptr(), shm);
}

/// Constructs ZShmMut slice in its gravestone value.
#[no_mangle]
pub extern "C" fn z_shm_mut_null(this: *mut MaybeUninit<z_owned_shm_mut_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_shm_mut_check(this: &z_owned_shm_mut_t) -> bool {
    this.transmute_ref().is_some()
}

/// Borrows ZShmMut slice
#[no_mangle]
pub extern "C" fn z_shm_mut_loan_mut(this: &mut z_owned_shm_mut_t) -> &mut z_loaned_shm_mut_t {
    let this = this.transmute_mut();
    let this = unwrap_ref_unchecked_mut(this);
    let shmmut: &mut zshmmut = this.borrow_mut();
    shmmut.transmute_handle_mut()
}

/// Deletes ZShm slice
#[no_mangle]
pub extern "C" fn z_shm_mut_drop(this: &mut z_owned_shm_mut_t) {
    let _ = this.transmute_mut().take(); 
}
