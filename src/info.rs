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
use std::mem::MaybeUninit;

#[cfg(feature = "unstable")]
use zenoh::session::{
    Link, LinkEvent, LinkEventsListener, Transport, TransportEvent, TransportEventsListener,
};
use zenoh::{session::ZenohId, Wait};

pub use crate::opaque_types::z_id_t;
use crate::{
    result,
    transmute::{CTypeRef, IntoCType, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_zid_call, z_closure_zid_loan, z_loaned_session_t, z_moved_closure_zid_t,
    z_owned_string_t,
};
decl_c_type!(copy(z_id_t, ZenohId));

#[cfg(feature = "unstable")]
use crate::{
    transmute::LoanedCTypeRef, z_closure_transport_call, z_closure_transport_loan,
    z_loaned_link_event_t, z_loaned_link_events_listener_t, z_loaned_link_t,
    z_loaned_transport_event_t, z_loaned_transport_events_listener_t, z_loaned_transport_t,
    z_moved_closure_transport_t, z_owned_link_event_t, z_owned_link_events_listener_t,
    z_owned_link_t, z_owned_transport_event_t, z_owned_transport_events_listener_t,
    z_owned_transport_t,
};

#[cfg(feature = "unstable")]
decl_c_type!(owned(z_owned_transport_t, option Transport), loaned(z_loaned_transport_t, Transport));
#[cfg(feature = "unstable")]
decl_c_type!(owned(z_owned_link_t, option Link), loaned(z_loaned_link_t, Link));
#[cfg(feature = "unstable")]
decl_c_type!(owned(z_owned_transport_event_t, option TransportEvent), loaned(z_loaned_transport_event_t, TransportEvent));
#[cfg(feature = "unstable")]
decl_c_type!(owned(z_owned_link_event_t, option LinkEvent), loaned(z_loaned_link_event_t, LinkEvent));
#[cfg(feature = "unstable")]
decl_c_type!(owned(z_owned_transport_events_listener_t, option TransportEventsListener<()>), loaned(z_loaned_transport_events_listener_t, TransportEventsListener<()>));
#[cfg(feature = "unstable")]
decl_c_type!(owned(z_owned_link_events_listener_t, option LinkEventsListener<()>), loaned(z_loaned_link_events_listener_t, LinkEventsListener<()>));

impl From<[u8; 16]> for z_id_t {
    fn from(value: [u8; 16]) -> Self {
        z_id_t { id: value }
    }
}

/// @brief Formats the `z_id_t` into 16-digit hex string (LSB-first order)
#[no_mangle]
pub extern "C" fn z_id_to_string(zid: &z_id_t, dst: &mut MaybeUninit<z_owned_string_t>) {
    let zid = zid.as_rust_type_ref();
    dst.as_rust_type_mut_uninit().write(zid.to_string().into());
}

/// @brief Returns the session's Zenoh ID.
///
/// Unless the `session` is invalid, that ID is guaranteed to be non-zero.
/// In other words, this function returning an array of 16 zeros means you failed
/// to pass it a valid session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_zid(session: &z_loaned_session_t) -> z_id_t {
    let session = session.as_rust_type_ref();
    session.info().zid().wait().into_c_type()
}

/// @brief Fetches the Zenoh IDs of all connected peers.
///
/// `callback` will be called once for each ID, is guaranteed to never be called concurrently,
/// and is guaranteed to be dropped before this function exits.
///
/// Retuns 0 on success, negative values on failure
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_peers_zid(
    session: &z_loaned_session_t,
    callback: &mut z_moved_closure_zid_t,
) -> result::z_result_t {
    let session = session.as_rust_type_ref();
    let callback = callback.take_rust_type();
    for mut id in session.info().peers_zid().wait() {
        z_closure_zid_call(z_closure_zid_loan(&callback), id.as_ctype_mut());
    }
    result::Z_OK
}

/// @brief Fetches the Zenoh IDs of all connected routers.
///
/// `callback` will be called once for each ID, is guaranteed to never be called concurrently,
/// and is guaranteed to be dropped before this function exits.
///
/// Retuns 0 on success, negative values on failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_routers_zid(
    session: &z_loaned_session_t,
    callback: &mut z_moved_closure_zid_t,
) -> result::z_result_t {
    let session = session.as_rust_type_ref();
    let callback = callback.take_rust_type();
    for mut id in session.info().routers_zid().wait() {
        z_closure_zid_call(z_closure_zid_loan(&callback), id.as_ctype_mut());
    }
    result::Z_OK
}

/// @brief Get the zenoh id from the `z_loaned_transport_t`.
///
/// Returns the zenoh id of the transport.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_transport_zid(transport: &z_loaned_transport_t) -> z_id_t {
    let transport = transport.as_rust_type_ref();
    transport.zid().into_c_type()
}

/// @brief Get the transports `z_loaned_transport_t` used by the session.
///
/// The tranport is a connection to another zenoh node. The `z_owned_transport_t`
/// contains the common information related to that connection. The information specific
/// to concrete network protocols are in the muttiple `z_owned_link_t` associated to each transport
/// rereieved by `z_info_links`.
///
/// Callback will be called once for each transport, is guaranteed to never be called concurrently,
/// and is guaranteed to be dropped before this function exits.
///
/// Returns 0 on success, negative values on failure.
#[cfg(feature = "unstable")]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_transports(
    session: &z_loaned_session_t,
    callback: &mut z_moved_closure_transport_t,
) -> result::z_result_t {
    let session = session.as_rust_type_ref();
    let callback = callback.take_rust_type();
    for mut transport in session.info().transports().wait() {
        z_closure_transport_call(
            z_closure_transport_loan(&callback),
            transport.as_loaned_ctype_mut(),
        );
    }
    result::Z_OK
}
