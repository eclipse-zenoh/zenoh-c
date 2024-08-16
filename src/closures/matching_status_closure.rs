//
// Copyright (c) 2017, 2024 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//
use std::mem::MaybeUninit;

use libc::c_void;

use crate::{
    transmute::{LoanedCTypeRef, OwnedCTypeRef, TakeRustType},
    zc_matching_status_t,
};
/// A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
///
/// Closures are not guaranteed not to be called concurrently.
///
/// It is guaranteed that:
///   - `call` will never be called once `drop` has started.
///   - `drop` will only be called **once**, and **after every** `call` has ended.
///   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
#[repr(C)]
pub struct zc_owned_closure_matching_status_t {
    /// An optional pointer to a closure state.
    context: *mut c_void,
    /// A closure body.
    call: Option<extern "C" fn(matching_status: &zc_matching_status_t, context: *mut c_void)>,
    /// An optional drop function that will be called when the closure is dropped.
    drop: Option<extern "C" fn(context: *mut c_void)>,
}

/// Loaned closure.
#[repr(C)]
pub struct zc_loaned_closure_matching_status_t {
    _0: [usize; 3],
}

/// Moved closure.
#[repr(C)]
pub struct zc_moved_closure_matching_status_t {
    _this: zc_owned_closure_matching_status_t,
}

decl_c_type!(
    owned(zc_owned_closure_matching_status_t),
    loaned(zc_loaned_closure_matching_status_t),
    moved(zc_moved_closure_matching_status_t),
);

impl Default for zc_owned_closure_matching_status_t {
    fn default() -> Self {
        zc_owned_closure_matching_status_t {
            context: std::ptr::null_mut(),
            call: None,
            drop: None,
        }
    }
}

impl zc_owned_closure_matching_status_t {
    pub fn is_empty(&self) -> bool {
        self.call.is_none() && self.drop.is_none() && self.context.is_null()
    }
}
unsafe impl Send for zc_owned_closure_matching_status_t {}
unsafe impl Sync for zc_owned_closure_matching_status_t {}
impl Drop for zc_owned_closure_matching_status_t {
    fn drop(&mut self) {
        if let Some(drop) = self.drop {
            drop(self.context)
        }
    }
}
/// Constructs a null value of 'zc_owned_closure_matching_status_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_closure_matching_status_null(
    this: *mut MaybeUninit<zc_owned_closure_matching_status_t>,
) {
    (*this).write(zc_owned_closure_matching_status_t::default());
}

/// Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn zc_closure_matching_status_check(
    this: &zc_owned_closure_matching_status_t,
) -> bool {
    !this.is_empty()
}

/// Calls the closure. Calling an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn zc_closure_matching_status_call(
    closure: &zc_loaned_closure_matching_status_t,
    mathing_status: &zc_matching_status_t,
) {
    let closure = closure.as_owned_c_type_ref();
    match closure.call {
        Some(call) => call(mathing_status, closure.context),
        None => {
            tracing::error!("Attempted to call an uninitialized closure!");
        }
    }
}
/// Drops the closure, resetting it to its gravestone state. Droping an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn zc_closure_matching_status_drop(closure_: &mut zc_moved_closure_matching_status_t) {
    let _ = closure_.take_rust_type();
}

impl<F: Fn(&zc_matching_status_t)> From<F> for zc_owned_closure_matching_status_t {
    fn from(f: F) -> Self {
        let this = Box::into_raw(Box::new(f)) as _;
        extern "C" fn call<F: Fn(&zc_matching_status_t)>(
            response: &zc_matching_status_t,
            this: *mut c_void,
        ) {
            let this = unsafe { &*(this as *const F) };
            this(response)
        }
        extern "C" fn drop<F>(this: *mut c_void) {
            std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
        }
        zc_owned_closure_matching_status_t {
            context: this,
            call: Some(call::<F>),
            drop: Some(drop::<F>),
        }
    }
}

/// Borrows closure.
#[no_mangle]
pub extern "C" fn zc_closure_matching_status_loan(
    closure: &zc_owned_closure_matching_status_t,
) -> &zc_loaned_closure_matching_status_t {
    closure.as_loaned_c_type_ref()
}
