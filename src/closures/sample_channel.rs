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
    transmute2::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_loaned_sample_t, z_owned_closure_sample_t, z_owned_sample_t,
};
use libc::c_void;
use std::{mem::MaybeUninit, sync::Arc};
use zenoh::{
    handlers::{self, IntoHandler, RingChannelHandler},
    sample::Sample,
};

pub use crate::opaque_types::z_loaned_fifo_handler_sample_t;
pub use crate::opaque_types::z_owned_fifo_handler_sample_t;
decl_c_type!(
    owned(z_owned_fifo_handler_sample_t, Option<flume::Receiver<Sample>>),
    loaned(z_loaned_fifo_handler_sample_t, flume::Receiver<Sample>)
);

/// Drops the handler and resets it to a gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_sample_drop(this: &mut z_owned_fifo_handler_sample_t) {
    *this.as_rust_type_mut() = None;
}

/// Constructs a handler in gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_sample_null(
    this: &mut MaybeUninit<z_owned_fifo_handler_sample_t>,
) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_sample_check(this: &z_owned_fifo_handler_sample_t) -> bool {
    this.as_rust_type_ref().is_some()
}

extern "C" fn __z_handler_sample_send(sample: &z_loaned_sample_t, context: *mut c_void) {
    unsafe {
        let f = (context as *mut std::sync::Arc<dyn Fn(Sample) + Send + Sync>)
            .as_mut()
            .unwrap_unchecked();
        (f)(sample.as_rust_type_ref().clone());
    }
}

extern "C" fn __z_handler_sample_drop(context: *mut c_void) {
    unsafe {
        let f = (context as *mut Arc<dyn Fn(Sample) + Send + Sync>).read();
        std::mem::drop(f);
    }
}

/// Constructs send and recieve ends of the fifo channel
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_fifo_channel_sample_new(
    callback: &mut MaybeUninit<z_owned_closure_sample_t>,
    handler: &mut MaybeUninit<z_owned_fifo_handler_sample_t>,
    capacity: usize,
) {
    let fifo = handlers::FifoChannel::new(capacity);
    let (cb, h) = fifo.into_handler();
    let cb_ptr = Box::into_raw(Box::new(cb)) as *mut libc::c_void;
    handler.as_rust_type_mut_uninit().write(Some(h));
    callback.write(z_owned_closure_sample_t {
        call: Some(__z_handler_sample_send),
        context: cb_ptr,
        drop: Some(__z_handler_sample_drop),
    });
}

/// Borrows handler.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_fifo_handler_sample_loan(
    this: &z_owned_fifo_handler_sample_t,
) -> &z_loaned_fifo_handler_sample_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_ctype_ref()
}

/// Returns sample from the fifo buffer. If there are no more pending replies will block until next sample is received, or until
/// the channel is dropped (normally when there are no more samples to receive). In the later case will return ``false`` and sample will be
/// in the gravestone state.
#[no_mangle]
pub extern "C" fn z_fifo_handler_sample_recv(
    this: &z_loaned_fifo_handler_sample_t,
    sample: &mut MaybeUninit<z_owned_sample_t>,
) -> bool {
    match this.as_rust_type_ref().recv() {
        Ok(q) => {
            sample.as_rust_type_mut_uninit().write(Some(q));
            true
        }
        Err(_) => {
            sample.as_rust_type_mut_uninit().write(None);
            false
        }
    }
}

/// Returns sample from the fifo buffer. If there are no more pending replies will return immediately (with sample set to its gravestone state).
/// Will return false if the channel is dropped (normally when there are no more samples to receive) and there are no more replies in the fifo.
#[no_mangle]
pub extern "C" fn z_fifo_handler_sample_try_recv(
    this: &z_loaned_fifo_handler_sample_t,
    sample: &mut MaybeUninit<z_owned_sample_t>,
) -> bool {
    match this.as_rust_type_ref().try_recv() {
        Ok(q) => {
            sample.as_rust_type_mut_uninit().write(Some(q));
            true
        }
        Err(e) => {
            sample.as_rust_type_mut_uninit().write(None);
            match e {
                flume::TryRecvError::Empty => true,
                flume::TryRecvError::Disconnected => false,
            }
        }
    }
}

pub use crate::opaque_types::z_loaned_ring_handler_sample_t;
pub use crate::opaque_types::z_owned_ring_handler_sample_t;

decl_transmute_owned!(
    Option<RingChannelHandler<Sample>>,
    z_owned_ring_handler_sample_t
);
decl_transmute_handle!(RingChannelHandler<Sample>, z_loaned_ring_handler_sample_t);
validate_equivalence!(
    z_owned_fifo_handler_sample_t,
    z_loaned_ring_handler_sample_t
);

/// Drops the handler and resets it to a gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_sample_drop(this: &mut z_owned_ring_handler_sample_t) {
    Inplace::drop(this.transmute_mut());
}

/// Constructs a handler in gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_sample_null(
    this: *mut MaybeUninit<z_owned_ring_handler_sample_t>,
) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_sample_check(this: &z_owned_ring_handler_sample_t) -> bool {
    this.transmute_ref().is_some()
}

/// Constructs send and recieve ends of the ring channel
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_ring_channel_sample_new(
    callback: *mut MaybeUninit<z_owned_closure_sample_t>,
    handler: *mut MaybeUninit<z_owned_ring_handler_sample_t>,
    capacity: usize,
) {
    let ring = handlers::RingChannel::new(capacity);
    let (cb, h) = ring.into_handler();
    let cb_ptr = Box::into_raw(Box::new(cb)) as *mut libc::c_void;
    Inplace::init(handler.transmute_uninit_ptr(), Some(h));
    (*callback).write(z_owned_closure_sample_t {
        call: Some(__z_handler_sample_send),
        context: cb_ptr,
        drop: Some(__z_handler_sample_drop),
    });
}

/// Borrows handler.
#[no_mangle]
pub extern "C" fn z_ring_handler_sample_loan(
    this: &z_owned_ring_handler_sample_t,
) -> &z_loaned_ring_handler_sample_t {
    unwrap_ref_unchecked(this.transmute_ref()).transmute_handle()
}

/// Returns sample from the ring buffer. If there are no more pending replies will block until next sample is received, or until
/// the channel is dropped (normally when there are no more samples to receive). In the later case will return ``false`` and sample will be
/// in the gravestone state.
#[no_mangle]
pub extern "C" fn z_ring_handler_sample_recv(
    this: &z_loaned_ring_handler_sample_t,
    sample: &mut MaybeUninit<z_owned_sample_t>,
) -> bool {
    match this.transmute_ref().recv() {
        Ok(q) => {
            sample.as_rust_type_mut_uninit().write(Some(q));
            true
        }
        Err(_) => {
            sample.as_rust_type_mut_uninit().write(None);
            false
        }
    }
}

/// Returns sample from the ring buffer. If there are no more pending replies will return immediately (with sample set to its gravestone state).
/// Will return false if the channel is dropped (normally when there are no more samples to receive) and there are no more replies in the fifo.
#[no_mangle]
pub extern "C" fn z_ring_handler_sample_try_recv(
    this: &z_loaned_ring_handler_sample_t,
    sample: &mut MaybeUninit<z_owned_sample_t>,
) -> bool {
    match this.transmute_ref().try_recv() {
        Ok(q) => {
            sample.as_rust_type_mut_uninit().write(q);
            true
        }
        Err(_) => {
            sample.as_rust_type_mut_uninit().write(None);
            false
        }
    }
}
