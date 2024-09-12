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
    z_id_t,
};
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
///
/// Closures are not guaranteed not to be called concurrently.
///
/// It is guaranteed that:
///   - `call` will never be called once `drop` has started.
///   - `drop` will only be called **once**, and **after every** `call` has ended.
///   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
#[repr(C)]
pub struct z_owned_closure_zid_t {
    /// An optional pointer to a closure state.
    context: *mut c_void,
    /// A callback function.
    call: Option<extern "C" fn(z_id: &mut z_id_t, context: *mut c_void)>,
    /// An optional function that will be called upon closure drop.
    drop: Option<extern "C" fn(context: *mut c_void)>,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Loaned closure.
#[repr(C)]
pub struct z_loaned_closure_zid_t {
    _0: [usize; 3],
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Moved closure.
#[repr(C)]
pub struct z_moved_closure_zid_t {
    pub _this: z_owned_closure_zid_t,
}

decl_c_type!(
    owned(z_owned_closure_zid_t),
    loaned(z_loaned_closure_zid_t),
    moved(z_moved_closure_zid_t),
);

impl Default for z_owned_closure_zid_t {
    fn default() -> Self {
        z_owned_closure_zid_t {
            context: std::ptr::null_mut(),
            call: None,
            drop: None,
        }
    }
}

impl z_owned_closure_zid_t {
    pub fn is_empty(&self) -> bool {
        self.call.is_none() && self.drop.is_none() && self.context.is_null()
    }
}
unsafe impl Send for z_owned_closure_zid_t {}
unsafe impl Sync for z_owned_closure_zid_t {}
impl Drop for z_owned_closure_zid_t {
    fn drop(&mut self) {
        if let Some(drop) = self.drop {
            drop(self.context)
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_internal_closure_zid_check(this_: &z_owned_closure_zid_t) -> bool {
    !this_.is_empty()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs a null closure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_internal_closure_zid_null(
    this_: &mut MaybeUninit<z_owned_closure_zid_t>,
) {
    this_.write(z_owned_closure_zid_t::default());
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Calls the closure. Calling an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn z_closure_zid_call(closure: &z_loaned_closure_zid_t, z_id: &z_id_t) {
    let closure = closure.as_owned_c_type_ref();
    match closure.call {
        Some(call) => call(z_id, closure.context),
        None => {
            tracing::error!("Attempted to call an uninitialized closure!");
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Drops the closure, resetting it to its gravestone state. Droping an uninitialized (null) closure is a no-op.
#[no_mangle]
pub extern "C" fn z_closure_zid_drop(closure_: &mut z_moved_closure_zid_t) {
    let _ = closure_.take_rust_type();
}

impl<F: Fn(&z_id_t)> From<F> for z_owned_closure_zid_t {
    fn from(f: F) -> Self {
        let this = Box::into_raw(Box::new(f)) as _;
        extern "C" fn call<F: Fn(&z_id_t)>(response: &z_id_t, this: *mut c_void) {
            let this = unsafe { &*(this as *const F) };
            this(response)
        }
        extern "C" fn drop<F>(this: *mut c_void) {
            std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
        }
        z_owned_closure_zid_t {
            context: this,
            call: Some(call::<F>),
            drop: Some(drop::<F>),
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows closure.
#[no_mangle]
pub extern "C" fn z_closure_zid_loan(closure: &z_owned_closure_zid_t) -> &z_loaned_closure_zid_t {
    closure.as_loaned_c_type_ref()
}
