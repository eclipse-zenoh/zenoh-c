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
    transmute::{
        unwrap_ref_unchecked, unwrap_ref_unchecked_mut, Inplace, TransmuteFromHandle,
        TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
    },
    z_loaned_shm_mut_t, z_loaned_shm_t, z_owned_shm_mut_t, z_owned_shm_t,
};

decl_transmute_owned!(Option<ZShm>, z_owned_shm_t);

decl_transmute_handle!(zshm, z_loaned_shm_t);

/// Constructs ZShm slice from ZShmMut slice
#[no_mangle]
pub extern "C" fn z_shm_from_mut(
    this: *mut MaybeUninit<z_owned_shm_t>,
    that: &mut z_owned_shm_mut_t,
) {
    let shm: Option<ZShm> = that.transmute_mut().extract().map(|val| val.into());
    Inplace::init(this.transmute_uninit_ptr(), shm);
}

/// Constructs ZShm slice in its gravestone value.
#[no_mangle]
pub extern "C" fn z_shm_null(this: *mut MaybeUninit<z_owned_shm_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if `this` is valid.
#[no_mangle]
pub extern "C" fn z_shm_check(this: &z_owned_shm_t) -> bool {
    this.transmute_ref().is_some()
}

/// Converts borrowed ZShm slice as owned ZShm slice by performing shared memory handle copy
#[no_mangle]
pub extern "C" fn z_shm_copy(this: *mut MaybeUninit<z_owned_shm_t>, loaned: &z_loaned_shm_t) {
    let loaned = loaned.transmute_ref();
    let owned = loaned.to_owned();
    Inplace::init(this.transmute_uninit_ptr(), Some(owned));
}

/// Borrows ZShm slice
#[no_mangle]
pub extern "C" fn z_shm_loan(this: &z_owned_shm_t) -> &z_loaned_shm_t {
    let this = this.transmute_ref();
    let this: &ZShm = unwrap_ref_unchecked(this);
    let shm: &zshm = this.borrow();
    shm.transmute_handle()
}

/// Mutably borrows ZShm slice
#[no_mangle]
pub extern "C" fn z_shm_loan_mut(this: &mut z_owned_shm_t) -> &mut z_loaned_shm_t {
    let this = this.transmute_mut();
    let this: &mut ZShm = unwrap_ref_unchecked_mut(this);
    let shm: &mut zshm = this.borrow_mut();
    shm.transmute_handle_mut()
}

/// Mutably borrows ZShm slice as borrowed ZShmMut slice
#[no_mangle]
pub extern "C" fn z_shm_try_mut(this: &mut z_owned_shm_t) -> *mut z_loaned_shm_mut_t {
    let this = this.transmute_mut();
    let this: &mut ZShm = unwrap_ref_unchecked_mut(this);
    let shm: &mut zshm = this.borrow_mut();
    match shm.try_into() {
        Ok(val) => {
            let v: &mut zshmmut = val;
            v.transmute_handle_mut()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Deletes ZShm slice
#[no_mangle]
pub extern "C" fn z_shm_drop(this: &mut z_owned_shm_t) {
    let _ = this.transmute_mut().take();
}

/// Tries to reborrow mutably-borrowed ZShm slice as borrowed ZShmMut slice
#[no_mangle]
pub extern "C" fn z_shm_try_reloan_mut(this: &mut z_loaned_shm_t) -> *mut z_loaned_shm_mut_t {
    let this = this.transmute_mut();
    match this.try_into() {
        Ok(val) => {
            let v: &mut zshmmut = val;
            v.transmute_handle_mut()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// @return the length of the ZShm slice
#[no_mangle]
pub extern "C" fn z_shm_len(this: &z_loaned_shm_t) -> usize {
    this.transmute_ref().len()
}

/// @return the pointer of the ZShm slice
#[no_mangle]
pub extern "C" fn z_shm_data(this: &z_loaned_shm_t) -> *const libc::c_uchar {
    let s = this.transmute_ref();
    s.as_ref().as_ptr()
}
