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
    transmute::{
        unwrap_ref_unchecked, Inplace, TransmuteFromHandle, TransmuteIntoHandle, TransmuteRef,
        TransmuteUninitPtr,
    },
    transmute2::{RustTypeRef, RustTypeRefUninit},
    z_loaned_query_t, z_owned_closure_query_t, z_owned_query_t,
};
use libc::c_void;
use std::{mem::MaybeUninit, sync::Arc};
use zenoh::{
    handlers::{self, IntoHandler, RingChannelHandler},
    query::Query,
};

pub use crate::opaque_types::z_loaned_fifo_handler_query_t;
pub use crate::opaque_types::z_owned_fifo_handler_query_t;

decl_transmute_owned!(Option<flume::Receiver<Query>>, z_owned_fifo_handler_query_t);
decl_transmute_handle!(flume::Receiver<Query>, z_loaned_fifo_handler_query_t);
validate_equivalence!(z_owned_fifo_handler_query_t, z_loaned_fifo_handler_query_t);

/// Drops the handler and resets it to a gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_query_drop(this: &mut z_owned_fifo_handler_query_t) {
    Inplace::drop(this.transmute_mut());
}

/// Constructs a handler in gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_query_null(this: *mut MaybeUninit<z_owned_fifo_handler_query_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_query_check(this: &z_owned_fifo_handler_query_t) -> bool {
    this.transmute_ref().is_some()
}

extern "C" fn __z_handler_query_send(query: &z_loaned_query_t, context: *mut c_void) {
    unsafe {
        let f = (context as *mut std::sync::Arc<dyn Fn(Query) + Send + Sync>)
            .as_mut()
            .unwrap_unchecked();
        (f)(query.as_rust_type_ref().clone());
    }
}

extern "C" fn __z_handler_query_drop(context: *mut c_void) {
    unsafe {
        let f = (context as *mut Arc<dyn Fn(Query) + Send + Sync>).read();
        std::mem::drop(f);
    }
}

/// Constructs send and recieve ends of the fifo channel
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_fifo_channel_query_new(
    callback: *mut MaybeUninit<z_owned_closure_query_t>,
    handler: *mut MaybeUninit<z_owned_fifo_handler_query_t>,
    capacity: usize,
) {
    let fifo = handlers::FifoChannel::new(capacity);
    let (cb, h) = fifo.into_handler();
    let cb_ptr = Box::into_raw(Box::new(cb)) as *mut libc::c_void;
    Inplace::init(handler.transmute_uninit_ptr(), Some(h));
    (*callback).write(z_owned_closure_query_t {
        call: Some(__z_handler_query_send),
        context: cb_ptr,
        drop: Some(__z_handler_query_drop),
    });
}

/// Borrows handler.
#[no_mangle]
pub extern "C" fn z_fifo_handler_query_loan(
    this: &z_owned_fifo_handler_query_t,
) -> &z_loaned_fifo_handler_query_t {
    unwrap_ref_unchecked(this.transmute_ref()).transmute_handle()
}

/// Returns query from the fifo buffer. If there are no more pending queries will block until next query is received, or until
/// the channel is dropped (normally when Queryable is dropped). In the later case will return ``false`` and query will be
/// in the gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_query_recv(
    this: &z_loaned_fifo_handler_query_t,
    query: &mut MaybeUninit<z_owned_query_t>,
) -> bool {
    match this.transmute_ref().recv() {
        Ok(q) => {
            query.as_rust_type_mut_uninit().write(Some(q));
            true
        }
        Err(_) => {
            query.as_rust_type_mut_uninit().write(None);
            false
        }
    }
}

/// Returns query from the fifo buffer. If there are no more pending queries will return immediately (with query set to its gravestone state).
/// Will return false if the channel is dropped (normally when Queryable is dropped) and there are no more queries in the fifo.
#[no_mangle]
pub extern "C" fn z_fifo_handler_query_try_recv(
    this: &z_loaned_fifo_handler_query_t,
    query: &mut MaybeUninit<z_owned_query_t>,
) -> bool {
    match this.transmute_ref().try_recv() {
        Ok(q) => {
            query.as_rust_type_mut_uninit().write(Some(q));
            true
        }
        Err(e) => {
            query.as_rust_type_mut_uninit().write(None);
            match e {
                flume::TryRecvError::Empty => true,
                flume::TryRecvError::Disconnected => false,
            }
        }
    }
}

pub use crate::opaque_types::z_loaned_ring_handler_query_t;
pub use crate::opaque_types::z_owned_ring_handler_query_t;

decl_transmute_owned!(
    Option<RingChannelHandler<Query>>,
    z_owned_ring_handler_query_t
);
decl_transmute_handle!(RingChannelHandler<Query>, z_loaned_ring_handler_query_t);
validate_equivalence!(z_owned_fifo_handler_query_t, z_loaned_ring_handler_query_t);

/// Drops the handler and resets it to a gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_query_drop(this: &mut z_owned_ring_handler_query_t) {
    Inplace::drop(this.transmute_mut());
}

/// Constructs a handler in gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_query_null(this: *mut MaybeUninit<z_owned_ring_handler_query_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_query_check(this: &z_owned_ring_handler_query_t) -> bool {
    this.transmute_ref().is_some()
}

/// Constructs send and recieve ends of the ring channel
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_ring_channel_query_new(
    callback: *mut MaybeUninit<z_owned_closure_query_t>,
    handler: *mut MaybeUninit<z_owned_ring_handler_query_t>,
    capacity: usize,
) {
    let ring = handlers::RingChannel::new(capacity);
    let (cb, h) = ring.into_handler();
    let cb_ptr = Box::into_raw(Box::new(cb)) as *mut libc::c_void;
    Inplace::init(handler.transmute_uninit_ptr(), Some(h));
    (*callback).write(z_owned_closure_query_t {
        call: Some(__z_handler_query_send),
        context: cb_ptr,
        drop: Some(__z_handler_query_drop),
    });
}

/// Borrows handler.
#[no_mangle]
pub extern "C" fn z_ring_handler_query_loan(
    this: &z_owned_ring_handler_query_t,
) -> &z_loaned_ring_handler_query_t {
    unwrap_ref_unchecked(this.transmute_ref()).transmute_handle()
}

/// Returns query from the ring buffer. If there are no more pending queries will block until next query is received, or until
/// the channel is dropped (normally when Queryable is dropped). In the later case will return ``false`` and query will be
/// in the gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_query_recv(
    this: &z_loaned_ring_handler_query_t,
    query: &mut MaybeUninit<z_owned_query_t>,
) -> bool {
    match this.transmute_ref().recv() {
        Ok(q) => {
            query.as_rust_type_mut_uninit().write(Some(q));
            true
        }
        Err(_) => {
            query.as_rust_type_mut_uninit().write(None);
            false
        }
    }
}

/// Returns query from the ring buffer. If there are no more pending queries will return immediately (with query set to its gravestone state).
/// Will return false if the channel is dropped (normally when Queryable is dropped) and there are no more queries in the fifo.
#[no_mangle]
pub extern "C" fn z_ring_handler_query_try_recv(
    this: &z_loaned_ring_handler_query_t,
    query: &mut MaybeUninit<z_owned_query_t>,
) -> bool {
    match this.transmute_ref().try_recv() {
        Ok(q) => {
            query.as_rust_type_mut_uninit().write(q);
            true
        }
        Err(_) => {
            query.as_rust_type_mut_uninit().write(None);
            false
        }
    }
}
