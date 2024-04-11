use crate::{z_closure_reply_drop, z_owned_closure_reply_t, z_owned_reply_t};

/// A pair of closures, the `send` one accepting
#[repr(C)]
pub struct z_owned_reply_fifo_channel_t {
    pub send: z_owned_closure_reply_t,
    pub recv: z_owned_closure_reply_t,
    pub try_recv: z_owned_closure_reply_t,
}
#[no_mangle]
pub extern "C" fn z_reply_fifo_channel_drop(channel: &mut z_owned_reply_fifo_channel_t) {
    z_closure_reply_drop(&mut channel.send);
    z_closure_reply_drop(&mut channel.recv);
    z_closure_reply_drop(&mut channel.try_recv);
}
/// Constructs a null safe-to-drop value of 'z_owned_reply_fifo_channel_t' type
#[no_mangle]
pub extern "C" fn z_reply_fifo_channel_null() -> z_owned_reply_fifo_channel_t {
    z_owned_reply_fifo_channel_t {
        send: z_owned_closure_reply_t::empty(),
        recv: z_owned_closure_reply_t::empty(),
        try_recv: z_owned_closure_reply_t::empty(),
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
pub extern "C" fn z_reply_fifo_channel_new(bound: usize) -> z_owned_reply_fifo_channel_t {
    let (tx, rx) = if bound == 0 {
        crossbeam_channel::unbounded()
    } else {
        crossbeam_channel::bounded(bound)
    };
    let rx_clone = rx.clone();
    z_owned_reply_fifo_channel_t {
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
        }),
        try_recv: From::from(move |receptacle: &mut z_owned_reply_t| {
            *receptacle = match rx_clone.try_recv() {
                Ok(val) => val.into(),
                Err(_) => None.into(),
            };
        }),
    }
}
