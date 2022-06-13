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
        context: *mut c_void,
        call: Option<extern "C" fn(&z_sample_t, context: *const c_void)>,
        drop: Option<extern "C" fn(*mut c_void)>,
    }
    impl z_owned_closure_sample_t {
        pub const fn empty() -> Self {
            z_owned_closure_sample_t {
                context: std::ptr::null_mut(),
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
                drop(self.context)
            }
        }
    }
    /// Calls the closure. Calling an uninitialized closure is a no-op.
    #[no_mangle]
    pub extern "C" fn z_owned_closure_sample_call(
        closure: &z_owned_closure_sample_t,
        sample: &z_sample_t,
    ) {
        match closure.call {
            Some(call) => call(sample, closure.context),
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
                context: this,
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
        context: *mut c_void,
        call: Option<extern "C" fn(z_query_t, context: *const c_void)>,
        drop: Option<extern "C" fn(*mut c_void)>,
    }
    impl z_owned_closure_query_t {
        pub const fn empty() -> Self {
            z_owned_closure_query_t {
                context: std::ptr::null_mut(),
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
                drop(self.context)
            }
        }
    }
    /// Calls the closure. Calling an uninitialized closure is a no-op.
    #[no_mangle]
    pub extern "C" fn z_owned_closure_query_call(
        closure: &z_owned_closure_query_t,
        query: z_query_t,
    ) {
        match closure.call {
            Some(call) => call(query, closure.context),
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
                context: this,
                call: Some(call::<F>),
                drop: Some(drop::<F>),
            }
        }
    }
}

pub use reply_closure::*;
mod reply_closure {
    use crate::z_owned_reply_t;
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
    pub struct z_owned_closure_reply_t {
        context: *mut c_void,
        call: Option<extern "C" fn(&mut z_owned_reply_t, *const c_void)>,
        drop: Option<extern "C" fn(*mut c_void)>,
    }

    impl z_owned_closure_reply_t {
        pub fn empty() -> Self {
            z_owned_closure_reply_t {
                context: std::ptr::null_mut(),
                call: None,
                drop: None,
            }
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
    /// Calls the closure. Calling an uninitialized closure is a no-op.
    #[no_mangle]
    pub extern "C" fn z_owned_closure_reply_call(
        closure: &z_owned_closure_reply_t,
        sample: &mut z_owned_reply_t,
    ) {
        match closure.call {
            Some(call) => call(sample, closure.context),
            None => {
                log::error!("Attempted to call an uninitialized closure!");
            }
        }
    }
    /// Drops the closure. Droping an uninitialized closure is a no-op.
    #[no_mangle]
    pub extern "C" fn z_owned_closure_reply_drop(closure: &mut z_owned_closure_reply_t) {
        let mut empty_closure = z_owned_closure_reply_t::empty();
        std::mem::swap(&mut empty_closure, closure);
    }
    impl<F: Fn(&mut z_owned_reply_t)> From<F> for z_owned_closure_reply_t {
        fn from(f: F) -> Self {
            let this = Box::into_raw(Box::new(f)) as _;
            extern "C" fn call<F: Fn(&mut z_owned_reply_t)>(
                response: &mut z_owned_reply_t,
                this: *const c_void,
            ) {
                let this = unsafe { &*(this as *const F) };
                this(response)
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
}

pub use response_channel::*;
mod response_channel {
    use crate::{z_owned_closure_reply_t, z_owned_reply_t};
    use libc::c_void;
    use std::sync::mpsc::TryRecvError;
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
    pub struct z_owned_reply_channel_closure_t {
        context: *mut c_void,
        call: Option<extern "C" fn(&mut z_owned_reply_t, *const c_void) -> bool>,
        drop: Option<extern "C" fn(*mut c_void)>,
    }

    /// A pair of closures, the `send` one accepting
    #[repr(C)]
    pub struct z_coowned_reply_channel_t {
        pub send: z_owned_closure_reply_t,
        pub recv: z_owned_reply_channel_closure_t,
    }

    /// Creates a new blocking fifo channel, returned as a pair of closures.
    ///
    /// The `send` end should be passed as callback to a `z_get` call.
    ///
    /// The `recv` end is a synchronous closure that will block until either a `z_owned_reply_t` is available,
    /// which it will then return; or until the `send` closure is dropped and all replies have been consumed,
    /// at which point it will return an invalidated `z_owned_reply_t`, and so will further calls.
    #[no_mangle]
    pub extern "C" fn z_reply_fifo_new() -> z_coowned_reply_channel_t {
        let (tx, rx) = std::sync::mpsc::channel();
        z_coowned_reply_channel_t {
            send: From::from(move |reply: &mut z_owned_reply_t| {
                if let Some(reply) = reply.take() {
                    if let Err(e) = tx.send(reply) {
                        log::error!("Attempted to push onto a closed reply_fifo: {}", e)
                    }
                }
            }),
            recv: From::from(move |receptacle: &mut z_owned_reply_t| {
                *receptacle = match rx.recv() {
                    Ok(val) => val.into(),
                    Err(_) => None.into(),
                };
                true
            }),
        }
    }

    /// Creates a new non-blocking fifo channel, returned as a pair of closures.
    ///
    /// The `send` end should be passed as callback to a `z_get` call.
    ///
    /// The `recv` end is a synchronous closure that will block until either a `z_owned_reply_t` is available,
    /// which it will then return; or until the `send` closure is dropped and all replies have been consumed,
    /// at which point it will return an invalidated `z_owned_reply_t`, and so will further calls.
    #[no_mangle]
    pub extern "C" fn z_reply_non_blocking_fifo_new() -> z_coowned_reply_channel_t {
        let (tx, rx) = std::sync::mpsc::channel();
        z_coowned_reply_channel_t {
            send: From::from(move |reply: &mut z_owned_reply_t| {
                if let Some(reply) = reply.take() {
                    if let Err(e) = tx.send(reply) {
                        log::error!("Attempted to push onto a closed reply_fifo: {}", e)
                    }
                }
            }),
            recv: From::from(
                move |receptacle: &mut z_owned_reply_t| match rx.try_recv() {
                    Ok(val) => {
                        let mut tmp = z_owned_reply_t::from(val);
                        std::mem::swap(&mut tmp, receptacle);
                        true
                    }
                    Err(TryRecvError::Disconnected) => {
                        receptacle.take();
                        true
                    }
                    Err(TryRecvError::Empty) => {
                        receptacle.take();
                        false
                    }
                },
            ),
        }
    }

    impl z_owned_reply_channel_closure_t {
        pub fn empty() -> Self {
            z_owned_reply_channel_closure_t {
                context: std::ptr::null_mut(),
                call: None,
                drop: None,
            }
        }
    }
    unsafe impl Send for z_owned_reply_channel_closure_t {}
    unsafe impl Sync for z_owned_reply_channel_closure_t {}
    impl Drop for z_owned_reply_channel_closure_t {
        fn drop(&mut self) {
            if let Some(drop) = self.drop {
                drop(self.context)
            }
        }
    }
    /// Calls the closure. Calling an uninitialized closure is a no-op.
    #[no_mangle]
    pub extern "C" fn z_owned_reply_channel_closure_call(
        closure: &z_owned_reply_channel_closure_t,
        sample: &mut z_owned_reply_t,
    ) -> bool {
        match closure.call {
            Some(call) => call(sample, closure.context),
            None => {
                log::error!("Attempted to call an uninitialized closure!");
                true
            }
        }
    }
    /// Drops the closure. Droping an uninitialized closure is a no-op.
    #[no_mangle]
    pub extern "C" fn z_owned_reply_channel_closure_drop(
        closure: &mut z_owned_reply_channel_closure_t,
    ) {
        let mut empty_closure = z_owned_reply_channel_closure_t::empty();
        std::mem::swap(&mut empty_closure, closure);
    }
    impl<F: Fn(&mut z_owned_reply_t) -> bool> From<F> for z_owned_reply_channel_closure_t {
        fn from(f: F) -> Self {
            let this = Box::into_raw(Box::new(f)) as _;
            extern "C" fn call<F: Fn(&mut z_owned_reply_t) -> bool>(
                response: &mut z_owned_reply_t,
                this: *const c_void,
            ) -> bool {
                let this = unsafe { &*(this as *const F) };
                this(response)
            }
            extern "C" fn drop<F>(this: *mut c_void) {
                std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
            }
            z_owned_reply_channel_closure_t {
                context: this,
                call: Some(call::<F>),
                drop: Some(drop::<F>),
            }
        }
    }
}
