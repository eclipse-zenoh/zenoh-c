//
// Copyright (c) 2017, 2022 ZettaScale Technology.
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
pub use sample_closure::*;
mod sample_closure {
    use crate::z_sample_t;
    use libc::c_void;
    /// A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
    /// - `this` is a pointer to an arbitrary state.
    /// - `call` is the typical callback function. `this` will be passed as its last argument.
    /// - `drop` allows the callback's state to be freed.
    ///
    /// Closures are not guaranteed not to be called concurrently.
    ///
    /// We guarantee that:
    /// - `call` will never be called once `drop` has started.
    /// - `drop` will only be called ONCE, and AFTER EVERY `call` has ended.
    /// - The two previous guarantees imply that `call` and `drop` are never called concurrently.
    #[repr(C)]
    pub struct z_owned_closure_sample_t {
        this: *mut c_void,
        call: Option<extern "C" fn(&z_sample_t, *const c_void)>,
        drop: Option<extern "C" fn(*mut c_void)>,
    }
    impl z_owned_closure_sample_t {
        pub const fn empty() -> Self {
            z_owned_closure_sample_t {
                this: std::ptr::null_mut(),
                call: None,
                drop: None,
            }
        }
    }
    unsafe impl Send for z_owned_closure_sample_t {}
    unsafe impl Sync for z_owned_closure_sample_t {}
    impl Drop for z_owned_closure_sample_t {
        fn drop(&mut self) {
            if let Some(drop) = self.drop {
                drop(self.this)
            }
        }
    }
    /// Constructs a stateless closure from a pointer to function.
    /// The state pointer will always be a nullptr.
    #[no_mangle]
    pub extern "C" fn z_owned_closure_sample_new_stateless(
        call: extern "C" fn(&z_sample_t, *const c_void),
    ) -> z_owned_closure_sample_t {
        z_owned_closure_sample_t {
            this: std::ptr::null_mut(),
            call: Some(call),
            drop: None,
        }
    }
    /// Calls the closure. Calling an uninitialized closure is a no-op.
    #[no_mangle]
    pub extern "C" fn z_owned_closure_sample_call(
        closure: &z_owned_closure_sample_t,
        sample: &z_sample_t,
    ) {
        match closure.call {
            Some(call) => call(sample, closure.this),
            None => log::error!("Attempted to call an uninitialized closure!"),
        }
    }
    /// Drops the closure. Droping an uninitialized closure is a no-op.
    #[no_mangle]
    pub extern "C" fn z_owned_closure_sample_drop(closure: &mut z_owned_closure_sample_t) {
        let mut empty_closure = z_owned_closure_sample_t::empty();
        std::mem::swap(&mut empty_closure, closure);
    }
    impl<F: Fn(&z_sample_t)> From<F> for z_owned_closure_sample_t {
        fn from(f: F) -> Self {
            let this = Box::into_raw(Box::new(f)) as _;
            extern "C" fn call<F: Fn(&z_sample_t)>(sample: &z_sample_t, this: *const c_void) {
                let this = unsafe { &*(this as *const F) };
                this(sample)
            }
            extern "C" fn drop<F>(this: *mut c_void) {
                std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
            }
            z_owned_closure_sample_t {
                this,
                call: Some(call::<F>),
                drop: Some(drop::<F>),
            }
        }
    }
}

pub use query_closure::*;
mod query_closure {
    use crate::z_query_t;
    use libc::c_void;
    /// A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
    /// - `this` is a pointer to an arbitrary state.
    /// - `call` is the typical callback function. `this` will be passed as its last argument.
    /// - `drop` allows the callback's state to be freed.
    ///
    /// Closures are not guaranteed not to be called concurrently.
    ///
    /// We guarantee that:
    /// - `call` will never be called once `drop` has started.
    /// - `drop` will only be called ONCE, and AFTER EVERY `call` has ended.
    /// - The two previous guarantees imply that `call` and `drop` are never called concurrently.
    #[repr(C)]
    pub struct z_owned_closure_query_t {
        this: *mut c_void,
        call: Option<extern "C" fn(z_query_t, *const c_void)>,
        drop: Option<extern "C" fn(*mut c_void)>,
    }
    impl z_owned_closure_query_t {
        pub const fn empty() -> Self {
            z_owned_closure_query_t {
                this: std::ptr::null_mut(),
                call: None,
                drop: None,
            }
        }
    }
    unsafe impl Send for z_owned_closure_query_t {}
    unsafe impl Sync for z_owned_closure_query_t {}
    impl Drop for z_owned_closure_query_t {
        fn drop(&mut self) {
            if let Some(drop) = self.drop {
                drop(self.this)
            }
        }
    }
    /// Constructs a stateless closure from a pointer to function.
    /// The state pointer will always be a nullptr.
    #[no_mangle]
    pub extern "C" fn z_owned_closure_query_new_stateless(
        call: extern "C" fn(z_query_t, *const c_void),
    ) -> z_owned_closure_query_t {
        z_owned_closure_query_t {
            this: std::ptr::null_mut(),
            call: Some(call),
            drop: None,
        }
    }
    /// Calls the closure. Calling an uninitialized closure is a no-op.
    #[no_mangle]
    pub extern "C" fn z_owned_closure_query_call(
        closure: &z_owned_closure_query_t,
        sample: z_query_t,
    ) {
        match closure.call {
            Some(call) => call(sample, closure.this),
            None => log::error!("Attempted to call an uninitialized closure!"),
        }
    }
    /// Drops the closure. Droping an uninitialized closure is a no-op.
    #[no_mangle]
    pub extern "C" fn z_owned_closure_query_drop(closure: &mut z_owned_closure_query_t) {
        let mut empty_closure = z_owned_closure_query_t::empty();
        std::mem::swap(&mut empty_closure, closure);
    }
    impl<F: Fn(z_query_t)> From<F> for z_owned_closure_query_t {
        fn from(f: F) -> Self {
            let this = Box::into_raw(Box::new(f)) as _;
            extern "C" fn call<F: Fn(z_query_t)>(sample: z_query_t, this: *const c_void) {
                let this = unsafe { &*(this as *const F) };
                this(sample)
            }
            extern "C" fn drop<F>(this: *mut c_void) {
                std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
            }
            z_owned_closure_query_t {
                this,
                call: Some(call::<F>),
                drop: Some(drop::<F>),
            }
        }
    }
}
