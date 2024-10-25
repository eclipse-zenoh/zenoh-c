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
    transmute::{LoanedCTypeMut, LoanedCTypeRef, OwnedCTypeRef, TakeRustType},
    z_loaned_reply_t,
};

/// @brief A reply-processing closure.
///
/// A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
#[repr(C)]
pub struct z_owned_closure_reply_t {
    pub _context: *mut c_void,
    pub(crate) _call: Option<extern "C" fn(reply: &mut z_loaned_reply_t, context: *mut c_void)>,
    pub _drop: Option<extern "C" fn(context: *mut c_void)>,
}

/// Loaned closure.
#[repr(C)]
pub struct z_loaned_closure_reply_t {
    _0: [usize; 3],
}

/// Moved closure.
#[repr(C)]
#[derive(Default)]
pub struct z_moved_closure_reply_t {
    pub _this: z_owned_closure_reply_t,
}

decl_c_type!(
    owned(z_owned_closure_reply_t),
    loaned(z_loaned_closure_reply_t),
    moved(z_moved_closure_reply_t),
);

impl Default for z_owned_closure_reply_t {
    fn default() -> Self {
        z_owned_closure_reply_t {
            _context: std::ptr::null_mut(),
            _call: None,
            _drop: None,
        }
    }
}

impl z_owned_closure_reply_t {
    pub(crate) fn is_empty(&self) -> bool {
        self._call.is_none() && self._drop.is_none() && self._context.is_null()
    }
}
unsafe impl Send for z_owned_closure_reply_t {}
unsafe impl Sync for z_owned_closure_reply_t {}
impl Drop for z_owned_closure_reply_t {
    fn drop(&mut self) {
        if let Some(drop) = self._drop {
            drop(self._context)
        }
    }
}
/// Constructs a closure int its gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_internal_closure_reply_null(
    this_: *mut MaybeUninit<z_owned_closure_reply_t>,
) {
    (*this_).write(z_owned_closure_reply_t::default());
}

/// Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_closure_reply_check(this_: &z_owned_closure_reply_t) -> bool {
    !this_.is_empty()
}

/// Calls the closure. Calling an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn z_closure_reply_call(
    closure: &z_loaned_closure_reply_t,
    reply: &mut z_loaned_reply_t,
) {
    let closure = closure.as_owned_c_type_ref();
    match closure._call {
        Some(call) => call(reply, closure._context),
        None => {
            tracing::error!("Attempted to call an uninitialized closure!");
        }
    }
}
/// Drops the closure, resetting it to its gravestone state. Droping an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn z_closure_reply_drop(closure_: &mut z_moved_closure_reply_t) {
    let _ = closure_.take_rust_type();
}

impl<F: Fn(&mut z_loaned_reply_t)> From<F> for z_owned_closure_reply_t {
    fn from(f: F) -> Self {
        let this = Box::into_raw(Box::new(f)) as _;
        extern "C" fn call<F: Fn(&mut z_loaned_reply_t)>(
            response: &mut z_loaned_reply_t,
            this: *mut c_void,
        ) {
            let this = unsafe { &*(this as *const F) };
            this(response);
        }
        extern "C" fn drop<F>(this: *mut c_void) {
            std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
        }
        z_owned_closure_reply_t {
            _context: this,
            _call: Some(call::<F>),
            _drop: Some(drop::<F>),
        }
    }
}

/// Borrows closure.
#[no_mangle]
pub extern "C" fn z_closure_reply_loan(
    closure: &z_owned_closure_reply_t,
) -> &z_loaned_closure_reply_t {
    closure.as_loaned_c_type_ref()
}

/// Mutably borrows closure.
#[no_mangle]
pub extern "C" fn z_closure_reply_loan_mut(
    closure: &mut z_owned_closure_reply_t,
) -> &mut z_loaned_closure_reply_t {
    closure.as_loaned_c_type_mut()
}

/// @brief Constructs closure.
///
/// Closures are not guaranteed not to be called concurrently.
///
/// It is guaranteed that:
///   - `call` will never be called once `drop` has started.
///   - `drop` will only be called **once**, and **after every** `call` has ended.
///   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
/// @param this_: uninitialized memory location where new closure will be constructed.
/// @param call: a closure body.
/// @param drop: an optional function to be called once on closure drop.
/// @param context: closure context.
#[no_mangle]
pub extern "C" fn z_closure_reply(
    this: &mut MaybeUninit<z_owned_closure_reply_t>,
    call: Option<extern "C" fn(reply: &mut z_loaned_reply_t, context: *mut c_void)>,
    drop: Option<extern "C" fn(context: *mut c_void)>,
    context: *mut c_void,
) {
    this.write(z_owned_closure_reply_t {
        _context: context,
        _call: call,
        _drop: drop,
    });
}
