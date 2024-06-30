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

use crate::{
    transmute::{LoanedCTypeRef, OwnedCTypeRef},
    z_loaned_reply_t,
};
use libc::c_void;
use std::mem::MaybeUninit;
/// A structure that contains all the elements for stateful, memory-leak-free callbacks.
///
/// Closures are not guaranteed not to be called concurrently.
///
/// It is guaranteed that:
///   - `call` will never be called once `drop` has started.
///   - `drop` will only be called **once**, and **after every** `call` has ended.
///   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
#[repr(C)]
pub struct z_owned_closure_reply_t {
    /// An optional pointer to a context representing a closure state.
    pub context: *mut c_void,
    /// A closure body.
    pub(crate) call: Option<extern "C" fn(reply: &z_loaned_reply_t, context: *mut c_void)>,
    /// An optional drop function that will be called when the closure is dropped.
    pub drop: Option<extern "C" fn(context: *mut c_void)>,
}

/// Loaned closure.
#[repr(C)]
pub struct z_loaned_closure_reply_t {
    _0: [usize; 3],
}

/// Moved closure.
#[repr(C)]
pub struct z_moved_closure_reply_t<'a> {
    pub ptr: &'a z_owned_closure_reply_t,
}

decl_c_type!(
    owned(z_owned_closure_reply_t, z_moved_closure_reply_t),
    loaned(z_loaned_closure_reply_t)
);

impl z_owned_closure_reply_t {
    pub(crate) fn empty() -> Self {
        z_owned_closure_reply_t {
            context: std::ptr::null_mut(),
            call: None,
            drop: None,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.call.is_none() && self.drop.is_none() && self.context.is_null()
    }
}
unsafe impl Send for z_owned_closure_reply_t {}
unsafe impl Sync for z_owned_closure_reply_t {}
impl Drop for z_owned_closure_reply_t {
    fn drop(&mut self) {
        if let Some(drop) = self.drop {
            drop(self.context)
        }
    }
}
/// Constructs a closure int its gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_closure_reply_null(this: *mut MaybeUninit<z_owned_closure_reply_t>) {
    (*this).write(z_owned_closure_reply_t::empty());
}

/// Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_closure_reply_check(this: &z_owned_closure_reply_t) -> bool {
    !this.is_empty()
}

/// Calls the closure. Calling an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn z_closure_reply_call(
    closure: &z_loaned_closure_reply_t,
    reply: &z_loaned_reply_t,
) {
    let closure = closure.as_owned_c_type_ref();
    match closure.call {
        Some(call) => call(reply, closure.context),
        None => {
            log::error!("Attempted to call an uninitialized closure!");
        }
    }
}
/// Drops the closure, resetting it to its gravestone state. Droping an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn z_closure_reply_drop(closure: &mut z_owned_closure_reply_t) {
    let mut empty_closure = z_owned_closure_reply_t::empty();
    std::mem::swap(&mut empty_closure, closure);
}
impl<F: Fn(&z_loaned_reply_t)> From<F> for z_owned_closure_reply_t {
    fn from(f: F) -> Self {
        let this = Box::into_raw(Box::new(f)) as _;
        extern "C" fn call<F: Fn(&z_loaned_reply_t)>(
            response: &z_loaned_reply_t,
            this: *mut c_void,
        ) {
            let this = unsafe { &*(this as *const F) };
            this(response);
        }
        extern "C" fn drop<F>(this: *mut c_void) {
            std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
        }
        z_owned_closure_reply_t {
            context: this,
            call: Some(call::<F>),
            drop: Some(drop::<F>),
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
