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
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_loaned_reply_t, z_owned_closure_reply_t, z_owned_reply_t,
};
use libc::c_void;
use std::{mem::MaybeUninit, sync::Arc};
use zenoh::{
    handlers::{self, IntoHandler, RingChannelHandler},
    query::Reply,
};

pub use crate::opaque_types::z_loaned_fifo_handler_reply_t;
pub use crate::opaque_types::z_moved_fifo_handler_reply_t;
pub use crate::opaque_types::z_owned_fifo_handler_reply_t;
decl_c_type!(
    owned(
        z_owned_fifo_handler_reply_t,
        Option<flume::Receiver<Reply>>,
    ),
    loaned(z_loaned_fifo_handler_reply_t, flume::Receiver<Reply>),
    moved z_moved_fifo_handler_reply_t
);

/// Drops the handler and resets it to a gravestone state.
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_fifo_handler_reply_drop(this: z_moved_fifo_handler_reply_t) {}

/// Constructs a handler in gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_reply_null(this: &mut MaybeUninit<z_owned_fifo_handler_reply_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_reply_check(this: &z_owned_fifo_handler_reply_t) -> bool {
    this.as_rust_type_ref().is_some()
}

extern "C" fn __z_handler_reply_send(reply: &z_loaned_reply_t, context: *mut c_void) {
    unsafe {
        let f = (context as *mut std::sync::Arc<dyn Fn(Reply) + Send + Sync>)
            .as_mut()
            .unwrap_unchecked();
        (f)(reply.as_rust_type_ref().clone());
    }
}

extern "C" fn __z_handler_reply_drop(context: *mut c_void) {
    unsafe {
        let f = (context as *mut Arc<dyn Fn(Reply) + Send + Sync>).read();
        std::mem::drop(f);
    }
}

/// Constructs send and recieve ends of the fifo channel
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_fifo_channel_reply_new(
    callback: &mut MaybeUninit<z_owned_closure_reply_t>,
    handler: &mut MaybeUninit<z_owned_fifo_handler_reply_t>,
    capacity: usize,
) {
    let fifo = handlers::FifoChannel::new(capacity);
    let (cb, h) = fifo.into_handler();
    let cb_ptr = Box::into_raw(Box::new(cb)) as *mut libc::c_void;
    handler.as_rust_type_mut_uninit().write(Some(h));
    callback.write(z_owned_closure_reply_t {
        call: Some(__z_handler_reply_send),
        context: cb_ptr,
        drop: Some(__z_handler_reply_drop),
    });
}

/// Borrows handler.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_fifo_handler_reply_loan(
    this: &z_owned_fifo_handler_reply_t,
) -> &z_loaned_fifo_handler_reply_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Returns reply from the fifo buffer. If there are no more pending replies will block until next reply is received, or until
/// the channel is dropped (normally when all replies are received). In the later case will return ``false`` and reply will be
/// in the gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_reply_recv(
    this: &z_loaned_fifo_handler_reply_t,
    reply: &mut MaybeUninit<z_owned_reply_t>,
) -> bool {
    match this.as_rust_type_ref().recv() {
        Ok(q) => {
            reply.as_rust_type_mut_uninit().write(Some(q));
            true
        }
        Err(_) => {
            reply.as_rust_type_mut_uninit().write(None);
            false
        }
    }
}

/// Returns reply from the fifo buffer. If there are no more pending replies will return immediately (with reply set to its gravestone state).
/// Will return false if the channel is dropped (normally when all replies are received) and there are no more replies in the fifo.
#[no_mangle]
pub extern "C" fn z_fifo_handler_reply_try_recv(
    this: &z_loaned_fifo_handler_reply_t,
    reply: &mut MaybeUninit<z_owned_reply_t>,
) -> bool {
    match this.as_rust_type_ref().try_recv() {
        Ok(q) => {
            reply.as_rust_type_mut_uninit().write(Some(q));
            true
        }
        Err(e) => {
            reply.as_rust_type_mut_uninit().write(None);
            match e {
                flume::TryRecvError::Empty => true,
                flume::TryRecvError::Disconnected => false,
            }
        }
    }
}

pub use crate::opaque_types::z_loaned_ring_handler_reply_t;
pub use crate::opaque_types::z_moved_ring_handler_reply_t;
pub use crate::opaque_types::z_owned_ring_handler_reply_t;
decl_c_type!(
    owned(
        z_owned_ring_handler_reply_t,
        Option<RingChannelHandler<Reply>>,
    ),
    loaned(z_loaned_ring_handler_reply_t, RingChannelHandler<Reply>),
    moved z_moved_ring_handler_reply_t
);

/// Drops the handler and resets it to a gravestone state.
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_ring_handler_reply_drop(this: z_moved_ring_handler_reply_t) {}

/// Constructs a handler in gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_reply_null(this: &mut MaybeUninit<z_owned_ring_handler_reply_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_reply_check(this: &z_owned_ring_handler_reply_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Constructs send and recieve ends of the ring channel
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_ring_channel_reply_new(
    callback: &mut MaybeUninit<z_owned_closure_reply_t>,
    handler: &mut MaybeUninit<z_owned_ring_handler_reply_t>,
    capacity: usize,
) {
    let ring = handlers::RingChannel::new(capacity);
    let (cb, h) = ring.into_handler();
    let cb_ptr = Box::into_raw(Box::new(cb)) as *mut libc::c_void;
    handler.as_rust_type_mut_uninit().write(Some(h));
    callback.write(z_owned_closure_reply_t {
        call: Some(__z_handler_reply_send),
        context: cb_ptr,
        drop: Some(__z_handler_reply_drop),
    });
}

/// Borrows handler.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_ring_handler_reply_loan(
    this: &z_owned_ring_handler_reply_t,
) -> &z_loaned_ring_handler_reply_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Returns reply from the ring buffer. If there are no more pending replies will block until next reply is received, or until
/// the channel is dropped (normally when all replies are received). In the later case will return ``false`` and reply will be
/// in the gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_reply_recv(
    this: &z_loaned_ring_handler_reply_t,
    reply: &mut MaybeUninit<z_owned_reply_t>,
) -> bool {
    match this.as_rust_type_ref().recv() {
        Ok(q) => {
            reply.as_rust_type_mut_uninit().write(Some(q));
            true
        }
        Err(_) => {
            reply.as_rust_type_mut_uninit().write(None);
            false
        }
    }
}

/// Returns reply from the ring buffer. If there are no more pending replies will return immediately (with reply set to its gravestone state).
/// Will return false if the channel is dropped (normally when all replies are received) and there are no more replies in the fifo.
#[no_mangle]
pub extern "C" fn z_ring_handler_reply_try_recv(
    this: &z_loaned_ring_handler_reply_t,
    reply: &mut MaybeUninit<z_owned_reply_t>,
) -> bool {
    match this.as_rust_type_ref().try_recv() {
        Ok(q) => {
            reply.as_rust_type_mut_uninit().write(q);
            true
        }
        Err(_) => {
            reply.as_rust_type_mut_uninit().write(None);
            false
        }
    }
}
