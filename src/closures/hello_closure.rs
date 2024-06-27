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

/// A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
///
/// Closures are not guaranteed not to be called concurrently.
///
/// It is guaranteed that:
///
///   - `call` will never be called once `drop` has started.
///   - `drop` will only be called **once**, and **after every** `call` has ended.
///   - The two previous guarantees imply that `call` and `drop` are never called concurrently.

use std::mem::MaybeUninit;
use crate::{
    transmute2::{LoanedCTypeRef, OwnedCTypeRef},
    z_loaned_hello_t,
};
use libc::c_void;

#[repr(C)]
pub struct z_owned_closure_hello_t {
    /// An optional pointer to a closure state.
    context: *mut c_void,
    /// A closure body.
    call: Option<extern "C" fn(hello: *const z_loaned_hello_t, context: *mut c_void)>,
    /// An optional drop function that will be called when the closure is dropped.
    drop: Option<extern "C" fn(context: *mut c_void)>,
}

/// Loaned closure.
#[repr(C)]
pub struct z_loaned_closure_hello_t {
    _0: [usize; 3],
}

decl_c_type!(
    owned(z_owned_closure_hello_t),
    loaned(z_loaned_closure_hello_t)
);

impl z_owned_closure_hello_t {
    pub fn empty() -> Self {
        z_owned_closure_hello_t {
            context: std::ptr::null_mut(),
            call: None,
            drop: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.call.is_none() && self.drop.is_none() && self.context.is_null()
    }
}
unsafe impl Send for z_owned_closure_hello_t {}
unsafe impl Sync for z_owned_closure_hello_t {}
impl Drop for z_owned_closure_hello_t {
    fn drop(&mut self) {
        if let Some(drop) = self.drop {
            drop(self.context)
        }
    }
}
/// Constructs a closure in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_closure_hello_null(this: *mut MaybeUninit<z_owned_closure_hello_t>) {
    (*this).write(z_owned_closure_hello_t::empty());
}
/// Calls the closure. Calling an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn z_closure_hello_call(
    closure: &z_loaned_closure_hello_t,
    hello: &z_loaned_hello_t,
) {
    let closure = closure.as_owned_ctype_ref();
    match closure.call {
        Some(call) => call(hello, closure.context),
        None => {
            log::error!("Attempted to call an uninitialized closure!");
        }
    }
}
/// Drops the closure. Droping an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn z_closure_hello_drop(closure: &mut z_owned_closure_hello_t) {
    let mut empty_closure = z_owned_closure_hello_t::empty();
    std::mem::swap(&mut empty_closure, closure);
}
impl<F: Fn(&z_loaned_hello_t)> From<F> for z_owned_closure_hello_t {
    fn from(f: F) -> Self {
        let this = Box::into_raw(Box::new(f)) as _;
        extern "C" fn call<F: Fn(&z_loaned_hello_t)>(
            response: *const z_loaned_hello_t,
            this: *mut c_void,
        ) {
            let this = unsafe { &*(this as *const F) };
            unsafe { this(response.as_ref().unwrap()) }
        }
        extern "C" fn drop<F>(this: *mut c_void) {
            std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
        }
        z_owned_closure_hello_t {
            context: this,
            call: Some(call::<F>),
            drop: Some(drop::<F>),
        }
    }
}

/// Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_closure_hello_check(this: &z_owned_closure_hello_t) -> bool {
    !this.is_empty()
}

/// Borrows closure.
#[no_mangle]
pub extern "C" fn z_closure_hello_loan(
    closure: &z_owned_closure_hello_t,
) -> &z_loaned_closure_hello_t {
    closure.as_loaned_ctype_ref()
}
