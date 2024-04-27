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

use zenoh::shm::slice::zsliceshmmut::ZSliceShmMut;

use crate::{decl_rust_new_owned_type, impl_guarded_transmute, GuardedTransmute};


/// A loaned ZSliceShmMut
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct z_loaned_slice_shm_mut_t<'a>(&'a z_owned_slice_shm_mut_t);

/// An owned ZSliceShmMut
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
pub struct z_owned_slice_shm_mut_t([u64; 10]);

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct z_owned_slice_shm_mut_t([u64; 10]);

#[cfg(target_arch = "arm")]
#[repr(C, align(8))]
pub struct z_owned_slice_shm_mut_t([u64; 10]);

decl_rust_new_owned_type!(
    zenoh:(Option<ZSliceShmMut>),
    c:(z_owned_slice_shm_mut_t)
);

/// Initializes a null memory for safe-to-drop value of 'z_owned_slice_shm_mut_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_slice_shm_mut_null(
    val: &mut z_owned_slice_shm_mut_t,
) {
    val.make_null();
}

/// Returns ``true`` if `val` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_slice_shm_mut_check(
    val: &z_owned_slice_shm_mut_t,
) -> bool {
    val.check()
}

/// Returns a :c:type:`z_loaned_slice_shm_mut_t` loaned from `val`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_slice_shm_mut_loan(
    val: &z_owned_slice_shm_mut_t,
) -> z_loaned_slice_shm_mut_t {
    z_loaned_slice_shm_mut_t(val)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_slice_shm_mut_delete(
    val: &mut z_owned_slice_shm_mut_t,
) {
    val.delete();
}