use crate::{
    z_closure_owned_query_drop, z_closure_query_drop, z_owned_closure_owned_query_t,
    z_owned_closure_query_t, z_owned_query_t, z_query_clone, z_query_t,
};

#[repr(C)]
pub struct z_owned_query_fifo_channel_t {
    pub send: z_owned_closure_query_t,
    pub recv: z_owned_closure_owned_query_t,
    pub try_recv: z_owned_closure_owned_query_t,
}
#[no_mangle]
pub extern "C" fn z_query_fifo_channel_drop(channel: &mut z_owned_query_fifo_channel_t) {
    z_closure_query_drop(&mut channel.send);
    z_closure_owned_query_drop(&mut channel.recv);
    z_closure_owned_query_drop(&mut channel.try_recv);
}
/// Constructs a null safe-to-drop value of 'z_owned_query_fifo_channel_t' type
#[no_mangle]
pub extern "C" fn z_query_fifo_channel_null() -> z_owned_query_fifo_channel_t {
    z_owned_query_fifo_channel_t {
        send: z_owned_closure_query_t::empty(),
        recv: z_owned_closure_owned_query_t::empty(),
        try_recv: z_owned_closure_owned_query_t::empty(),
    }
}

/// Creates a new blocking fifo channel, returned as a pair of closures.
///
/// If `bound` is different from 0, that channel will be bound and apply back-pressure when full.
///
/// The `send` end should be passed as callback to a `z_get` call.
///
/// The `recv` end is a synchronous closure that will block until either a `z_owned_query_t` is available,
/// which it will then return; or until the `send` closure is dropped and all replies have been consumed,
/// at which point it will return an invalidated `z_owned_query_t`, and so will further calls.
#[no_mangle]
pub extern "C" fn z_query_fifo_channel_new(bound: usize) -> z_owned_query_fifo_channel_t {
    // TODO(sashacmc): switch to handlers::FifoChannel
    let (tx, rx) = if bound == 0 {
        crossbeam_channel::unbounded()
    } else {
        crossbeam_channel::bounded(bound)
    };
    let rx_clone = rx.clone();
    z_owned_query_fifo_channel_t {
        send: From::from(move |query: &z_query_t| {
            let mut oquery = z_query_clone(Some(query));
            if let Some(oquery) = oquery.take() {
                if let Err(e) = tx.send(oquery) {
                    log::error!("Attempted to push onto a closed query_ring: {}", e)
                }
            }
        }),
        recv: From::from(move |receptacle: &mut z_owned_query_t| {
            *receptacle = match rx.recv() {
                Ok(val) => val.into(),
                Err(_) => None.into(),
            };
        }),
        try_recv: From::from(move |receptacle: &mut z_owned_query_t| {
            *receptacle = match rx_clone.try_recv() {
                Ok(val) => val.into(),
                Err(_) => None.into(),
            };
        }),
    }
}

#[repr(C)]
pub struct z_owned_query_ring_channel_t {
    pub send: z_owned_closure_query_t,
    pub recv: z_owned_closure_owned_query_t,
    pub try_recv: z_owned_closure_owned_query_t,
}
#[no_mangle]
pub extern "C" fn z_query_ring_channel_drop(channel: &mut z_owned_query_ring_channel_t) {
    z_closure_query_drop(&mut channel.send);
    z_closure_owned_query_drop(&mut channel.recv);
    z_closure_owned_query_drop(&mut channel.try_recv);
}
/// Constructs a null safe-to-drop value of 'z_owned_query_ring_channel_t' type
#[no_mangle]
pub extern "C" fn z_query_ring_channel_null() -> z_owned_query_ring_channel_t {
    z_owned_query_ring_channel_t {
        send: z_owned_closure_query_t::empty(),
        recv: z_owned_closure_owned_query_t::empty(),
        try_recv: z_owned_closure_owned_query_t::empty(),
    }
}

/// Creates a new blocking ring channel, returned as a pair of closures.
///
/// If `bound` is different from 0, that channel will be bound and apply back-pressure when full.
///
/// The `send` end should be passed as callback to a `z_get` call.
///
/// The `recv` end is a synchronous closure that will block until either a `z_owned_query_t` is available,
/// which it will then return; or until the `send` closure is dropped and all replies have been consumed,
/// at which point it will return an invalidated `z_owned_query_t`, and so will further calls.
#[no_mangle]
pub extern "C" fn z_query_ring_channel_new(bound: usize) -> z_owned_query_ring_channel_t {
    // TODO(sashacmc): switch to handlers::RingChannel
    let (tx, rx) = if bound == 0 {
        crossbeam_channel::unbounded()
    } else {
        crossbeam_channel::bounded(bound)
    };
    let rx_clone = rx.clone();
    z_owned_query_ring_channel_t {
        send: From::from(move |query: &z_query_t| {
            let mut oquery = z_query_clone(Some(query));
            if let Some(oquery) = oquery.take() {
                if let Err(e) = tx.send(oquery) {
                    log::error!("Attempted to push onto a closed query_ring: {}", e)
                }
            }
        }),
        recv: From::from(move |receptacle: &mut z_owned_query_t| {
            *receptacle = match rx.recv() {
                Ok(val) => val.into(),
                Err(_) => None.into(),
            };
        }),
        try_recv: From::from(move |receptacle: &mut z_owned_query_t| {
            *receptacle = match rx_clone.try_recv() {
                Ok(val) => val.into(),
                Err(_) => None.into(),
            };
        }),
    }
}
