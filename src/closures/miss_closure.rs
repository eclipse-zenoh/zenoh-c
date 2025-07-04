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
    ze_miss_t,
};
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A sample miss-processing closure.
///
/// A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
#[repr(C)]
pub struct ze_owned_closure_miss_t {
    _context: *mut c_void,
    _call: Option<extern "C" fn(matching_status: &ze_miss_t, context: *mut c_void)>,
    _drop: Option<extern "C" fn(context: *mut c_void)>,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Loaned closure.
#[repr(C)]
pub struct ze_loaned_closure_miss_t {
    _0: [usize; 3],
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Moved closure.
#[repr(C)]
pub struct ze_moved_closure_miss_t {
    _this: ze_owned_closure_miss_t,
}

decl_c_type!(
    owned(ze_owned_closure_miss_t),
    loaned(ze_loaned_closure_miss_t),
    moved(ze_moved_closure_miss_t),
);

impl Default for ze_owned_closure_miss_t {
    fn default() -> Self {
        ze_owned_closure_miss_t {
            _context: std::ptr::null_mut(),
            _call: None,
            _drop: None,
        }
    }
}

impl ze_owned_closure_miss_t {
    pub fn is_empty(&self) -> bool {
        self._call.is_none() && self._drop.is_none() && self._context.is_null()
    }
}
unsafe impl Send for ze_owned_closure_miss_t {}
unsafe impl Sync for ze_owned_closure_miss_t {}
impl Drop for ze_owned_closure_miss_t {
    fn drop(&mut self) {
        if let Some(drop) = self._drop {
            drop(self._context)
        }
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs a null value of 'ze_owned_closure_miss_t' type
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn ze_internal_closure_miss_null(
    this: *mut MaybeUninit<ze_owned_closure_miss_t>,
) {
    (*this).write(ze_owned_closure_miss_t::default());
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
#[prebindgen]
pub fn ze_internal_closure_miss_check(this: &ze_owned_closure_miss_t) -> bool {
    !this.is_empty()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Calls the closure. Calling an uninitialized closure is a no-op.
#[prebindgen]
pub fn ze_closure_miss_call(
    closure: &ze_loaned_closure_miss_t,
    mathing_status: &ze_miss_t,
) {
    let closure = closure.as_owned_c_type_ref();
    match closure._call {
        Some(call) => call(mathing_status, closure._context),
        None => {
            tracing::error!("Attempted to call an uninitialized closure!");
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Drops the closure, resetting it to its gravestone state. Droping an uninitialized closure is a no-op.
#[prebindgen]
pub fn ze_closure_miss_drop(closure_: &mut ze_moved_closure_miss_t) {
    let _ = closure_.take_rust_type();
}

impl<F: Fn(&ze_miss_t)> From<F> for ze_owned_closure_miss_t {
    fn from(f: F) -> Self {
        let this = Box::into_raw(Box::new(f)) as _;
        extern "C" fn call<F: Fn(&ze_miss_t)>(response: &ze_miss_t, this: *mut c_void) {
            let this = unsafe { &*(this as *const F) };
            this(response)
        }
        extern "C" fn drop<F>(this: *mut c_void) {
            std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
        }
        ze_owned_closure_miss_t {
            _context: this,
            _call: Some(call::<F>),
            _drop: Some(drop::<F>),
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows closure.
#[prebindgen]
pub fn ze_closure_miss_loan(
    closure: &ze_owned_closure_miss_t,
) -> &ze_loaned_closure_miss_t {
    closure.as_loaned_c_type_ref()
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
pub fn ze_closure_miss(
    this: &mut MaybeUninit<ze_owned_closure_miss_t>,
    call: Option<extern "C" fn(matching_status: &ze_miss_t, context: *mut c_void)>,
    drop: Option<extern "C" fn(context: *mut c_void)>,
    context: *mut c_void,
) {
    this.write(ze_owned_closure_miss_t {
        _context: context,
        _call: call,
        _drop: drop,
    });
}
