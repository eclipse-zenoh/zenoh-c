use crate::{z_closure_reply_drop, z_owned_closure_reply_t, z_owned_reply_t};
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
    call: Option<extern "C" fn(&mut z_owned_reply_t, *mut c_void) -> bool>,
    drop: Option<extern "C" fn(*mut c_void)>,
}

/// A pair of closures, the `send` one accepting
#[repr(C)]
pub struct z_owned_reply_channel_t {
    pub send: z_owned_closure_reply_t,
    pub recv: z_owned_reply_channel_closure_t,
}
#[no_mangle]
pub extern "C" fn z_reply_channel_drop(channel: &mut z_owned_reply_channel_t) {
    z_closure_reply_drop(&mut channel.send);
    z_reply_channel_closure_drop(&mut channel.recv);
}
/// Constructs a null safe-to-drop value of 'z_owned_reply_channel_t' type
#[no_mangle]
pub extern "C" fn z_reply_channel_null() -> z_owned_reply_channel_t {
    z_owned_reply_channel_t {
        send: z_owned_closure_reply_t::empty(),
        recv: z_owned_reply_channel_closure_t::empty(),
    }
}

/// Creates a new blocking fifo channel, returned as a pair of closures.
///
/// If `bound` is different from 0, that channel will be bound and apply back-pressure when full.
///
/// The `send` end should be passed as callback to a `z_get` call.
///
/// The `recv` end is a synchronous closure that will block until either a `z_owned_reply_t` is available,
/// which it will then return; or until the `send` closure is dropped and all replies have been consumed,
/// at which point it will return an invalidated `z_owned_reply_t`, and so will further calls.
#[no_mangle]
pub extern "C" fn zc_reply_fifo_new(bound: usize) -> z_owned_reply_channel_t {
    let (send, rx) = if bound == 0 {
        let (tx, rx) = std::sync::mpsc::channel();
        (
            From::from(move |reply: &mut z_owned_reply_t| {
                if let Some(reply) = reply.take() {
                    if let Err(e) = tx.send(reply) {
                        log::error!("Attempted to push onto a closed reply_fifo: {}", e)
                    }
                }
            }),
            rx,
        )
    } else {
        let (tx, rx) = std::sync::mpsc::sync_channel(bound);
        (
            From::from(move |reply: &mut z_owned_reply_t| {
                if let Some(reply) = reply.take() {
                    if let Err(e) = tx.send(reply) {
                        log::error!("Attempted to push onto a closed reply_fifo: {}", e)
                    }
                }
            }),
            rx,
        )
    };
    z_owned_reply_channel_t {
        send,
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
/// If `bound` is different from 0, that channel will be bound and apply back-pressure when full.
///
/// The `send` end should be passed as callback to a `z_get` call.
///
/// The `recv` end is a synchronous closure that will block until either a `z_owned_reply_t` is available,
/// which it will then return; or until the `send` closure is dropped and all replies have been consumed,
/// at which point it will return an invalidated `z_owned_reply_t`, and so will further calls.
#[no_mangle]
pub extern "C" fn zc_reply_non_blocking_fifo_new(bound: usize) -> z_owned_reply_channel_t {
    let (send, rx) = if bound == 0 {
        let (tx, rx) = std::sync::mpsc::channel();
        (
            From::from(move |reply: &mut z_owned_reply_t| {
                if let Some(reply) = reply.take() {
                    if let Err(e) = tx.send(reply) {
                        log::error!("Attempted to push onto a closed reply_fifo: {}", e)
                    }
                }
            }),
            rx,
        )
    } else {
        let (tx, rx) = std::sync::mpsc::sync_channel(bound);
        (
            From::from(move |reply: &mut z_owned_reply_t| {
                if let Some(reply) = reply.take() {
                    if let Err(e) = tx.send(reply) {
                        log::error!("Attempted to push onto a closed reply_fifo: {}", e)
                    }
                }
            }),
            rx,
        )
    };

    z_owned_reply_channel_t {
        send,
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

/// Constructs a null safe-to-drop value of 'z_owned_reply_channel_closure_t' type
#[no_mangle]
pub extern "C" fn z_reply_channel_closure_null() -> z_owned_reply_channel_closure_t {
    z_owned_reply_channel_closure_t::empty()
}

/// Calls the closure. Calling an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn z_reply_channel_closure_call(
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
pub extern "C" fn z_reply_channel_closure_drop(closure: &mut z_owned_reply_channel_closure_t) {
    let mut empty_closure = z_owned_reply_channel_closure_t::empty();
    std::mem::swap(&mut empty_closure, closure);
}
impl<F: Fn(&mut z_owned_reply_t) -> bool> From<F> for z_owned_reply_channel_closure_t {
    fn from(f: F) -> Self {
        let this = Box::into_raw(Box::new(f)) as _;
        extern "C" fn call<F: Fn(&mut z_owned_reply_t) -> bool>(
            response: &mut z_owned_reply_t,
            this: *mut c_void,
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
