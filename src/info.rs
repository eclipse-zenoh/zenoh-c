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
use zenoh::config::WhatAmI;
#[cfg(feature = "unstable")]
use zenoh::session::{
    Link, LinkEvent, LinkEventsListener, Transport, TransportEvent, TransportEventsListener,
};
use zenoh::{session::ZenohId, Wait};

pub use crate::opaque_types::z_id_t;
#[cfg(feature = "unstable")]
use crate::transmute::Gravestone;
use crate::{
    result,
    transmute::{CTypeRef, IntoCType, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_zid_call, z_closure_zid_loan, z_loaned_session_t, z_moved_closure_zid_t,
    z_owned_string_t,
};
#[cfg(feature = "unstable")]
use crate::{z_owned_string_array_t, z_reliability_t, z_whatami_t, CStringOwned};
#[cfg(feature = "unstable")]
use crate::{CStringInner, ZVector};
decl_c_type!(copy(z_id_t, ZenohId));

#[cfg(feature = "unstable")]
use crate::{
    transmute::LoanedCTypeRef, z_closure_link_call, z_closure_link_loan, z_closure_transport_call,
    z_closure_transport_loan, z_loaned_link_event_t, z_loaned_link_events_listener_t,
    z_loaned_link_t, z_loaned_transport_event_t, z_loaned_transport_events_listener_t,
    z_loaned_transport_t, z_moved_closure_link_t, z_moved_closure_transport_t,
    z_moved_transport_t, z_owned_link_event_t, z_owned_link_events_listener_t, z_owned_link_t,
    z_owned_transport_event_t, z_owned_transport_events_listener_t, z_owned_transport_t,
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

/// @brief Get the whatami from the `z_loaned_transport_t`.
///
/// Returns the whatami (node type) of the remote node.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_transport_whatami(transport: &z_loaned_transport_t) -> z_whatami_t {
    let transport = transport.as_rust_type_ref();
    match transport.whatami() {
        WhatAmI::Router => z_whatami_t::ROUTER,
        WhatAmI::Peer => z_whatami_t::PEER,
        WhatAmI::Client => z_whatami_t::CLIENT,
    }
}

/// @brief Check if the transport supports QoS.
///
/// Returns true if the transport supports QoS, false otherwise.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_transport_is_qos(transport: &z_loaned_transport_t) -> bool {
    let transport = transport.as_rust_type_ref();
    transport.is_qos()
}

/// @brief Check if the transport is multicast.
///
/// Returns true if the transport is multicast, false otherwise.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_transport_is_multicast(transport: &z_loaned_transport_t) -> bool {
    let transport = transport.as_rust_type_ref();
    transport.is_multicast()
}

/// @brief Check if the transport supports shared memory.
///
/// Returns true if the transport supports shared memory, false otherwise.
#[cfg(all(feature = "unstable", feature = "shared-memory"))]
#[no_mangle]
pub extern "C" fn z_transport_is_shm(transport: &z_loaned_transport_t) -> bool {
    let transport = transport.as_rust_type_ref();
    transport.is_shm()
}

/// @brief Clones the transport.
///
/// Creates an owned copy of the transport that can be used independently.
/// The caller is responsible for dropping the cloned transport using `z_drop`.
///
/// @param this_: The destination for the cloned transport.
/// @param transport: The transport to clone.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_transport_clone(
    this_: &mut MaybeUninit<z_owned_transport_t>,
    transport: &z_loaned_transport_t,
) {
    let transport = transport.as_rust_type_ref();
    this_.as_rust_type_mut_uninit().write(Some(transport.clone()));
}

/// @brief Drops the owned transport.
///
/// @param this_: The transport to drop.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_transport_drop(this_: &mut z_moved_transport_t) {
    let _ = this_.take_rust_type();
}

/// Constructs a transport in its gravestone state.
#[cfg(feature = "unstable")]
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_internal_transport_null(this_: &mut MaybeUninit<z_owned_transport_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if transport is valid, ``false`` if it is in gravestone state.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_internal_transport_check(this_: &z_owned_transport_t) -> bool {
    this_.as_rust_type_ref().is_some()
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
            transport.as_loaned_c_type_mut(),
        );
    }
    result::Z_OK
}

/// Options for `z_info_links()`.
#[cfg(feature = "unstable")]
#[repr(C)]
pub struct z_info_links_options_t {
    /// Optional transport to filter links by.
    /// If NULL, returns all links (default behavior).
    /// If provided, ownership of the transport is taken and it will be dropped after filtering.
    pub transport: Option<&'static mut z_moved_transport_t>,
}

#[cfg(feature = "unstable")]
impl Default for z_info_links_options_t {
    fn default() -> Self {
        Self { transport: None }
    }
}

/// Constructs the default value for `z_info_links_options_t`.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_info_links_options_default(
    this_: &mut MaybeUninit<z_info_links_options_t>,
) {
    this_.write(z_info_links_options_t::default());
}

/// @brief Get the links `z_loaned_link_t` used by the session.
///
/// The link represents a concrete network connection used by a transport.
/// Each transport may have multiple links associated with it.
///
/// Callback will be called once for each link, is guaranteed to never be called concurrently,
/// and is guaranteed to be dropped before this function exits.
///
/// @param session The session to query links from.
/// @param callback The callback to be called for each link.
/// @param options Optional parameters to filter links. If NULL, returns all links.
///
/// Returns 0 on success, negative values on failure.
#[cfg(feature = "unstable")]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_links(
    session: &z_loaned_session_t,
    callback: &mut z_moved_closure_link_t,
    options: Option<&mut z_info_links_options_t>,
) -> result::z_result_t {
    let session = session.as_rust_type_ref();
    let callback = callback.take_rust_type();

    // Build the links query with optional transport filter
    let session_info = session.info();
    let mut links_builder = session_info.links();

    // Apply transport filter if provided
    if let Some(opts) = options {
        if let Some(moved_transport) = opts.transport.take() {
            // Take ownership of the transport
            let owned_transport = moved_transport.take_rust_type();
            if let Some(transport) = owned_transport {
                // Use the transport for filtering
                links_builder = links_builder.transport(transport);
            }
        }
    }

    // Execute the query and call callback for each link
    for mut link in links_builder.wait() {
        z_closure_link_call(z_closure_link_loan(&callback), link.as_loaned_c_type_mut());
    }

    result::Z_OK
}

/// @brief Get the zenoh id from the `z_loaned_link_t`.
///
/// Returns the zenoh id of the transport this link belongs to.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_link_zid(link: &z_loaned_link_t) -> z_id_t {
    let link = link.as_rust_type_ref();
    link.zid().into_c_type()
}

/// @brief Get the source locator (local endpoint) from the `z_loaned_link_t`.
///
/// Stores the source locator string in `str_out`.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_link_src(link: &z_loaned_link_t, str_out: &mut MaybeUninit<z_owned_string_t>) {
    let link = link.as_rust_type_ref();
    str_out
        .as_rust_type_mut_uninit()
        .write(link.src().to_string().into());
}

/// @brief Get the destination locator (remote endpoint) from the `z_loaned_link_t`.
///
/// Stores the destination locator string in `str_out`.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_link_dst(link: &z_loaned_link_t, str_out: &mut MaybeUninit<z_owned_string_t>) {
    let link = link.as_rust_type_ref();
    str_out
        .as_rust_type_mut_uninit()
        .write(link.dst().to_string().into());
}

/// @brief Get the group locator from the `z_loaned_link_t` (for multicast links).
///
/// Stores the group locator string in `str_out` if present, or initializes a null string otherwise.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_link_group(
    link: &z_loaned_link_t,
    str_out: &mut MaybeUninit<z_owned_string_t>,
) {
    let link = link.as_rust_type_ref();
    str_out.as_rust_type_mut_uninit().write(
        link.group()
            .map(|g| g.to_string().into())
            .unwrap_or_else(|| CStringOwned::gravestone()),
    );
}

/// @brief Get the MTU (maximum transmission unit) from the `z_loaned_link_t`.
///
/// Returns the MTU in bytes.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_link_mtu(link: &z_loaned_link_t) -> u16 {
    let link = link.as_rust_type_ref();
    link.mtu()
}

/// @brief Check if the link is streamed.
///
/// Returns true if the link is streamed, false otherwise.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_link_is_streamed(link: &z_loaned_link_t) -> bool {
    let link = link.as_rust_type_ref();
    link.is_streamed()
}

/// @brief Get the network interfaces associated with the `z_loaned_link_t`.
///
/// Stores an array of interface name strings in `interfaces_out`.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_link_interfaces(
    link: &z_loaned_link_t,
    interfaces_out: &mut MaybeUninit<z_owned_string_array_t>,
) {
    let link = link.as_rust_type_ref();
    let mut interfaces = ZVector::with_capacity(link.interfaces().len());
    for iface in link.interfaces().iter() {
        interfaces.push(CStringInner::new_borrowed_from_slice(iface.as_bytes()));
    }
    interfaces_out.as_rust_type_mut_uninit().write(interfaces);
}

/// @brief Get the authentication identifier from the `z_loaned_link_t`.
///
/// Stores the authentication identifier string in `str_out` if present, or initializes a null string otherwise.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_link_auth_identifier(
    link: &z_loaned_link_t,
    str_out: &mut MaybeUninit<z_owned_string_t>,
) {
    let link = link.as_rust_type_ref();
    str_out.as_rust_type_mut_uninit().write(
        link.auth_identifier()
            .map(|s| s.to_string().into())
            .unwrap_or_else(|| CStringOwned::gravestone()),
    );
}

/// @brief Get the priority range from the `z_loaned_link_t` (if QoS is supported).
///
/// Returns true if priorities are supported and stores the min and max priorities in `min_out` and `max_out`.
/// Returns false if the link does not support QoS.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_link_priorities(
    link: &z_loaned_link_t,
    min_out: &mut u8,
    max_out: &mut u8,
) -> bool {
    let link = link.as_rust_type_ref();
    if let Some((min, max)) = link.priorities() {
        *min_out = min;
        *max_out = max;
        true
    } else {
        false
    }
}

/// @brief Get the reliability from the `z_loaned_link_t` (if QoS is supported).
///
/// Returns true if reliability information is available and stores the reliability level in `reliability_out`.
/// Returns false if the link does not support QoS.
#[cfg(feature = "unstable")]
#[no_mangle]
pub extern "C" fn z_link_reliability(
    link: &z_loaned_link_t,
    reliability_out: &mut z_reliability_t,
) -> bool {
    let link = link.as_rust_type_ref();
    if let Some(reliability) = link.reliability() {
        *reliability_out = reliability.into();
        true
    } else {
        false
    }
}
