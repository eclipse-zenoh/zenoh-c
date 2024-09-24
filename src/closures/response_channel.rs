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

use std::{mem::MaybeUninit, sync::Arc};

use libc::c_void;
use zenoh::{
    handlers::{self, IntoHandler, RingChannelHandler},
    query::Reply,
};

pub use crate::opaque_types::{
    z_loaned_fifo_handler_reply_t, z_moved_fifo_handler_reply_t, z_owned_fifo_handler_reply_t,
};
use crate::{
    result::{self, z_result_t},
    transmute::{LoanedCTypeRef, RustTypeMutUninit, RustTypeRef, TakeRustType},
    z_loaned_reply_t, z_owned_closure_reply_t, z_owned_reply_t,
};
decl_c_type!(
    owned(z_owned_fifo_handler_reply_t, option flume::Receiver<Reply>),
    loaned(z_loaned_fifo_handler_reply_t),
);

/// Drops the handler and resets it to a gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_reply_drop(this_: &mut z_moved_fifo_handler_reply_t) {
    let _ = this_.take_rust_type();
}

/// Constructs a handler in gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_fifo_handler_reply_null(
    this_: &mut MaybeUninit<z_owned_fifo_handler_reply_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_fifo_handler_reply_check(
    this_: &z_owned_fifo_handler_reply_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

extern "C" fn __z_handler_reply_send(reply: &mut z_loaned_reply_t, context: *mut c_void) {
    unsafe {
        let f = (context as *mut std::sync::Arc<dyn Fn(Reply) + Send + Sync>)
            .as_mut()
            .unwrap_unchecked();
        let owned_ref: &mut Option<Reply> = std::mem::transmute(reply);
        (f)(std::mem::take(owned_ref).unwrap_unchecked());
    }
}

extern "C" fn __z_handler_reply_drop(context: *mut c_void) {
    unsafe {
        let f = Box::from_raw(context as *mut Arc<dyn Fn(Reply) + Send + Sync>);
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

/// Mutably borrows handler.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_fifo_handler_reply_loan_mut(
    this: &mut z_owned_fifo_handler_reply_t,
) -> &mut z_loaned_fifo_handler_reply_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}


/// Returns reply from the fifo buffer. If there are no more pending replies will block until next reply is received, or until
/// the channel is dropped (normally when all replies are received).
/// @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the reply will be in the gravestone state).
#[no_mangle]
pub extern "C" fn z_fifo_handler_reply_recv(
    this: &z_loaned_fifo_handler_reply_t,
    reply: &mut MaybeUninit<z_owned_reply_t>,
) -> z_result_t {
    match this.as_rust_type_ref().recv() {
        Ok(q) => {
            reply.as_rust_type_mut_uninit().write(Some(q));
            result::Z_OK
        }
        Err(_) => {
            reply.as_rust_type_mut_uninit().write(None);
            result::Z_CHANNEL_DISCONNECTED
        }
    }
}

/// Returns reply from the fifo buffer. If there are no more pending replies will return immediately (with reply set to its gravestone state).
/// @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the reply will be in the gravestone state),
/// `Z_CHANNEL_NODATA` if the channel is still alive, but its buffer is empty (the reply will be in the gravestone state).
#[no_mangle]
pub extern "C" fn z_fifo_handler_reply_try_recv(
    this: &z_loaned_fifo_handler_reply_t,
    reply: &mut MaybeUninit<z_owned_reply_t>,
) -> z_result_t {
    match this.as_rust_type_ref().try_recv() {
        Ok(q) => {
            reply.as_rust_type_mut_uninit().write(Some(q));
            result::Z_OK
        }
        Err(e) => {
            reply.as_rust_type_mut_uninit().write(None);
            match e {
                flume::TryRecvError::Empty => result::Z_CHANNEL_NODATA,
                flume::TryRecvError::Disconnected => result::Z_CHANNEL_DISCONNECTED,
            }
        }
    }
}

pub use crate::opaque_types::{
    z_loaned_ring_handler_reply_t, z_moved_ring_handler_reply_t, z_owned_ring_handler_reply_t,
};
decl_c_type!(
    owned(z_owned_ring_handler_reply_t, option RingChannelHandler<Reply>),
    loaned(z_loaned_ring_handler_reply_t),
);

/// Drops the handler and resets it to a gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_reply_drop(this_: &mut z_moved_ring_handler_reply_t) {
    let _ = this_.take_rust_type();
}

/// Constructs a handler in gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_ring_handler_reply_null(
    this_: &mut MaybeUninit<z_owned_ring_handler_reply_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_ring_handler_reply_check(
    this_: &z_owned_ring_handler_reply_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
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

/// Mutably borrows handler.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_ring_handler_reply_loan_mut(
    this: &mut z_owned_ring_handler_reply_t,
) -> &mut z_loaned_ring_handler_reply_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// Returns reply from the ring buffer. If there are no more pending replies will block until next reply is received, or until
/// the channel is dropped (normally when all replies are received).
/// @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the reply will be in the gravestone state).
#[no_mangle]
pub extern "C" fn z_ring_handler_reply_recv(
    this: &z_loaned_ring_handler_reply_t,
    reply: &mut MaybeUninit<z_owned_reply_t>,
) -> z_result_t {
    match this.as_rust_type_ref().recv() {
        Ok(q) => {
            reply.as_rust_type_mut_uninit().write(Some(q));
            result::Z_OK
        }
        Err(_) => {
            reply.as_rust_type_mut_uninit().write(None);
            result::Z_CHANNEL_DISCONNECTED
        }
    }
}

/// Returns reply from the ring buffer. If there are no more pending replies will return immediately (with reply set to its gravestone state).
/// @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the reply will be in the gravestone state),
/// `Z_CHANNEL_NODATA` if the channel is still alive, but its buffer is empty (the reply will be in the gravestone state).
#[no_mangle]
pub extern "C" fn z_ring_handler_reply_try_recv(
    this: &z_loaned_ring_handler_reply_t,
    reply: &mut MaybeUninit<z_owned_reply_t>,
) -> z_result_t {
    match this.as_rust_type_ref().try_recv() {
        Ok(q) => {
            let r = if q.is_some() {
                result::Z_OK
            } else {
                result::Z_CHANNEL_NODATA
            };
            reply.as_rust_type_mut_uninit().write(q);
            r
        }
        Err(_) => {
            reply.as_rust_type_mut_uninit().write(None);
            result::Z_CHANNEL_DISCONNECTED
        }
    }
}
