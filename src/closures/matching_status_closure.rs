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
use prebindgen_proc_macro::prebindgen;
use libc::c_void;

use crate::{
    transmute::{LoanedCTypeRef, OwnedCTypeRef, TakeRustType},
    z_matching_status_t,
};
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A matching status-processing closure.
///
/// A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
#[prebindgen]
#[repr(C)]
pub struct z_owned_closure_matching_status_t {
    _context: *mut c_void,
    _call: Option<extern "C" fn(matching_status: &z_matching_status_t, context: *mut c_void)>,
    _drop: Option<extern "C" fn(context: *mut c_void)>,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Loaned closure.
#[prebindgen]
#[repr(C)]
pub struct z_loaned_closure_matching_status_t {
    _0: usize,
    _1: usize,
    _2: usize,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Moved closure.
#[prebindgen]
#[repr(C)]
pub struct z_moved_closure_matching_status_t {
    _this: z_owned_closure_matching_status_t,
}

decl_c_type!(
    owned(z_owned_closure_matching_status_t),
    loaned(z_loaned_closure_matching_status_t),
    moved(z_moved_closure_matching_status_t),
);

impl Default for z_owned_closure_matching_status_t {
    fn default() -> Self {
        z_owned_closure_matching_status_t {
            _context: std::ptr::null_mut(),
            _call: None,
            _drop: None,
        }
    }
}

impl z_owned_closure_matching_status_t {
    pub fn is_empty(&self) -> bool {
        self._call.is_none() && self._drop.is_none() && self._context.is_null()
    }
}
unsafe impl Send for z_owned_closure_matching_status_t {}
unsafe impl Sync for z_owned_closure_matching_status_t {}
impl Drop for z_owned_closure_matching_status_t {
    fn drop(&mut self) {
        if let Some(drop) = self._drop {
            drop(self._context)
        }
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs a null value of 'z_owned_closure_matching_status_t' type
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_internal_closure_matching_status_null(
    this: &mut MaybeUninit<z_owned_closure_matching_status_t>,
) {
    this.write(z_owned_closure_matching_status_t::default());
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
#[prebindgen]
pub fn z_internal_closure_matching_status_check(
    this: &z_owned_closure_matching_status_t,
) -> bool {
    !this.is_empty()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Calls the closure. Calling an uninitialized closure is a no-op.
#[prebindgen]
pub fn z_closure_matching_status_call(
    closure: &z_loaned_closure_matching_status_t,
    mathing_status: &z_matching_status_t,
) {
    let closure = closure.as_owned_c_type_ref();
    match closure._call {
        Some(call) => call(mathing_status, closure._context),
        None => {
            crate::report_error!("Attempted to call an uninitialized closure!");
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Drops the closure, resetting it to its gravestone state. Droping an uninitialized closure is a no-op.
#[prebindgen]
pub fn z_closure_matching_status_drop(closure_: &mut z_moved_closure_matching_status_t) {
    let _ = closure_.take_rust_type();
}

impl<F: Fn(&z_matching_status_t)> From<F> for z_owned_closure_matching_status_t {
    fn from(f: F) -> Self {
        let this = Box::into_raw(Box::new(f)) as _;
        extern "C" fn call<F: Fn(&z_matching_status_t)>(
            response: &z_matching_status_t,
            this: *mut c_void,
        ) {
            let this = unsafe { &*(this as *const F) };
            this(response)
        }
        extern "C" fn drop<F>(this: *mut c_void) {
            std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
        }
        z_owned_closure_matching_status_t {
            _context: this,
            _call: Some(call::<F>),
            _drop: Some(drop::<F>),
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows closure.
#[prebindgen]
pub fn z_closure_matching_status_loan(
    closure: &z_owned_closure_matching_status_t,
) -> &z_loaned_closure_matching_status_t {
    closure.as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Moves closure.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_closure_matching_status_move(
    closure: &mut z_owned_closure_matching_status_t,
) -> &mut z_moved_closure_matching_status_t {
    std::mem::transmute(closure)
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
///
/// Closures are not guaranteed not to be called concurrently.
///
/// It is guaranteed that:
///   - `call` will never be called once `drop` has started.
///   - `drop` will only be called **once**, and **after every** `call` has ended.
///   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
///
/// @brief Constructs closure.
/// @param this_: uninitialized memory location where new closure will be constructed.
/// @param call: a closure body.
/// @param drop: an optional function to be called once on closure drop.
/// @param context: closure context.
#[prebindgen]
pub fn z_closure_matching_status(
    this: &mut MaybeUninit<z_owned_closure_matching_status_t>,
    call: Option<extern "C" fn(matching_status: &z_matching_status_t, context: *mut c_void)>,
    drop: Option<extern "C" fn(context: *mut c_void)>,
    context: *mut c_void,
) {
    this.write(z_owned_closure_matching_status_t {
        _context: context,
        _call: call,
        _drop: drop,
    });
}
