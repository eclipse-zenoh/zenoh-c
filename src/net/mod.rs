//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use async_std::prelude::FutureExt;
use async_std::sync::{channel, Arc, Sender};
use async_std::task;
use futures::prelude::*;
use futures::select;
use libc::{c_char, c_int, c_uchar, c_uint, c_ulong};
use std::convert::TryFrom;
use std::ffi::{c_void, CStr, CString};
use std::slice;
use zenoh::net::*;
use zenoh_protocol::core::ZInt;
use zenoh_util::to_zint;

#[no_mangle]
pub static ROUTER: c_uint = whatami::ROUTER as c_uint;
#[no_mangle]
pub static PEER: c_uint = whatami::PEER as c_uint;
#[no_mangle]
pub static CLIENT: c_uint = whatami::CLIENT as c_uint;

// Flags used in Queryable declaration and in queries
#[no_mangle]
pub static ALL_KINDS: c_uint = queryable::ALL_KINDS as c_uint;
#[no_mangle]
pub static STORAGE: c_uint = queryable::STORAGE as c_uint;
#[no_mangle]
pub static EVAL: c_uint = queryable::EVAL as c_uint;

// Properties for zenoh net session configuration
#[no_mangle]
pub static ZN_MODE_KEY: c_uint = config::ZN_MODE_KEY as c_uint;
#[no_mangle]
pub static ZN_PEER_KEY: c_uint = config::ZN_PEER_KEY as c_uint;
#[no_mangle]
pub static ZN_LISTENER_KEY: c_uint = config::ZN_LISTENER_KEY as c_uint;
#[no_mangle]
pub static ZN_USER_KEY: c_uint = config::ZN_USER_KEY as c_uint;
#[no_mangle]
pub static ZN_PASSWORD_KEY: c_uint = config::ZN_PASSWORD_KEY as c_uint;
#[no_mangle]
pub static ZN_MULTICAST_SCOUTING_KEY: c_uint = config::ZN_MULTICAST_SCOUTING_KEY as c_uint;
#[no_mangle]
pub static ZN_MULTICAST_INTERFACE_KEY: c_uint = config::ZN_MULTICAST_INTERFACE_KEY as c_uint;
#[no_mangle]
pub static ZN_MULTICAST_ADDRESS_KEY: c_uint = config::ZN_MULTICAST_ADDRESS_KEY as c_uint;
#[no_mangle]
pub static ZN_SCOUTING_TIMEOUT_KEY: c_uint = config::ZN_SCOUTING_TIMEOUT_KEY as c_uint;
#[no_mangle]
pub static ZN_SCOUTING_DELAY_KEY: c_uint = config::ZN_SCOUTING_DELAY_KEY as c_uint;
#[no_mangle]
pub static ZN_ADD_TIMESTAMP_KEY: c_uint = config::ZN_ADD_TIMESTAMP_KEY as c_uint;
#[no_mangle]
pub static ZN_LOCAL_ROUTING_KEY: c_uint = config::ZN_LOCAL_ROUTING_KEY as c_uint;

// Properties returned by zn_info()
#[no_mangle]
pub static ZN_INFO_PID_KEY: c_uint = info::ZN_INFO_PID_KEY as c_uint;
#[no_mangle]
pub static ZN_INFO_PEER_PID_KEY: c_uint = info::ZN_INFO_PEER_PID_KEY as c_uint;
#[no_mangle]
pub static ZN_INFO_ROUTER_PID_KEY: c_uint = info::ZN_INFO_ROUTER_PID_KEY as c_uint;

pub struct ZNSession(zenoh::net::Session);

pub struct ZNResKey(zenoh::net::ResKey);

pub struct ZNProperties(zenoh::net::Properties);

enum ZnSubOps {
    Pull,
    Close,
}

pub struct ZNPublisher<'a>(zenoh::net::Publisher<'a>);

pub struct ZNSubscriber(Option<Arc<Sender<ZnSubOps>>>);

pub struct ZNQueryTarget(zenoh::net::QueryTarget);

pub struct ZNQueryConsolidation(zenoh::net::QueryConsolidation);

pub struct ZNQueryable(Option<Arc<Sender<bool>>>);

pub struct ZNQuery(zenoh::net::Query);

pub struct ZNSubInfo(zenoh::net::SubInfo);

pub struct ZNScout(std::vec::Vec<Hello>);

pub struct ZNLocators(std::vec::Vec<std::ffi::CString>);

/// A string.
///
/// Members:
///   const char *val: A pointer to the string.
///   unsigned int len: The length of the string.
///
#[repr(C)]
pub struct zn_string {
    val: *const c_char,
    len: c_uint,
}

/// An array of bytes.
///
/// Members:
///   const unsigned char *val: A pointer to the bytes array.
///   unsigned int len: The length of the bytes array.
///
#[repr(C)]
pub struct zn_bytes {
    val: *const c_uchar,
    len: c_uint,
}

/// A zenoh-net data sample.
///
/// A sample is the value associated to a given resource at a given point in time.
///
/// Members:
///   zn_string key: The resource key of this data sample.
///   zn_bytes value: The value of this data sample.
#[repr(C)]
pub struct zn_sample {
    key: zn_string,
    value: zn_bytes,
}

/// Information on the source of a reply.
///
/// Members:
///   unsigned int kind: The kind of source.
///   zn_bytes id: The unique id of the source.
#[repr(C)]
pub struct zn_source_info {
    kind: c_uint,
    id: zn_bytes,
}

/// Create a default :c:type:`ZNQueryTarget`.
#[no_mangle]
pub extern "C" fn zn_query_target_default() -> *mut ZNQueryTarget {
    Box::into_raw(Box::new(ZNQueryTarget(QueryTarget::default())))
}

/// Create a default :c:type:`ZNQueryConsolidation`.
#[no_mangle]
pub extern "C" fn zn_query_consolidation_default() -> *mut ZNQueryConsolidation {
    Box::into_raw(Box::new(
        ZNQueryConsolidation(QueryConsolidation::default()),
    ))
}

/// Create a no consolidation :c:type:`ZNQueryConsolidation`.
/// Same resource may be received multiple times from different queriable sources.
#[no_mangle]
pub extern "C" fn zn_query_consolidation_none() -> *mut ZNQueryConsolidation {
    Box::into_raw(Box::new(ZNQueryConsolidation(QueryConsolidation::None)))
}

/// Create an incremental consolidation :c:type:`ZNQueryConsolidation`.
/// Multiple occurences of the same resource will be filtered by all routers
/// along the replies path.
#[no_mangle]
pub extern "C" fn zn_query_consolidation_incremental() -> *mut ZNQueryConsolidation {
    Box::into_raw(Box::new(ZNQueryConsolidation(
        QueryConsolidation::Incremental,
    )))
}

/// Create an last_hop consolidation :c:type:`ZNQueryConsolidation`.
/// Multiple occurences of the same resource will be filtered by the last router
/// along the replies path.
#[no_mangle]
pub extern "C" fn zn_query_consolidation_last_hop() -> *mut ZNQueryConsolidation {
    Box::into_raw(Box::new(ZNQueryConsolidation(QueryConsolidation::LastHop)))
}

/// Create a resource key from a resource id.
///
/// Parameters:
///     id: The resource id.
///
/// Returns:
///     Return a new resource key.
#[no_mangle]
pub extern "C" fn zn_rid(id: c_ulong) -> *mut ZNResKey {
    Box::into_raw(Box::new(ZNResKey(zenoh::net::ResKey::RId(to_zint!(id)))))
}

/// Create a resource key from a resource id and a suffix.
///
/// Parameters:
///     id: The resource id.
///     suffix: The suffix.
///
/// Returns:
///     Return a new resource key.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_rid_with_suffix(id: c_ulong, suffix: *const c_char) -> *mut ZNResKey {
    Box::into_raw(Box::new(ZNResKey(zenoh::net::ResKey::RIdWithSuffix(
        to_zint!(id),
        CStr::from_ptr(suffix).to_str().unwrap().to_string(),
    ))))
}

/// Create a resource key from a resource name.
///
/// Parameters:
///     id: The resource name.
///
/// Returns:
///     Return a new resource key.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_rname(name: *const c_char) -> *mut ZNResKey {
    Box::into_raw(Box::new(ZNResKey(zenoh::net::ResKey::RName(
        CStr::from_ptr(name).to_str().unwrap().to_string(),
    ))))
}

/// Return a new empty set of properties.
#[no_mangle]
pub extern "C" fn zn_properties_make() -> *mut ZNProperties {
    Box::into_raw(Box::new(ZNProperties(zenoh::net::Properties::new())))
}

/// Get the length of the given properties.
///
/// Parameters:
///     ps: A pointer to the properties.
///
/// Returns:
///     The length of the given properties.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_properties_len(ps: *mut ZNProperties) -> c_uint {
    (*ps).0.len() as c_uint
}

/// Get the id of the property at a given index in a set of properties.
///
/// Parameters:
///     ps: A pointer to the properties.
///     n: The index of the property.
///
/// Returns:
///     The id of the property at index ``n`` in properties ``ps``.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_property_id(ps: *mut ZNProperties, n: c_uint) -> c_uint {
    (*ps).0[n as usize].0 as c_uint
}

/// Get the value of the property at a given index in a set of properties.
///
/// Parameters:
///     ps: A pointer to the properties.
///     n: The index of the property.
///
/// Returns:
///     The value of the property at index ``n`` in properties ``ps``.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_property_value(ps: *mut ZNProperties, n: c_uint) -> *const zn_bytes {
    let ptr = (*ps).0[n as usize].1.as_ptr();
    let value = Box::new(zn_bytes {
        val: ptr as *const c_uchar,
        len: (*ps).0[n as usize].1.len() as c_uint,
    });
    Box::into_raw(value)
}

/// Add a property to a set of properties.
///
/// Parameters:
///   ps: A pointer to the properties.
///   id: The id of the property to add.
///   value: The value of the property to add.
///
/// Returns:
///     A pointer to the updated properties.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_properties_add(
    ps: *mut ZNProperties,
    id: c_ulong,
    value: *const c_char,
) -> *mut ZNProperties {
    let bs = CStr::from_ptr(value).to_bytes();
    (*ps).0.push((to_zint!(id), Vec::from(bs)));
    ps
}

/// Free a set of properties.
///
/// Parameters:
///   ps: A pointer to the properties.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_properties_free(ps: *mut ZNProperties) {
    let bps = Box::from_raw(ps);
    drop(bps);
}

/// Create an empty set of properties for zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn zn_config_empty() -> *mut ZNProperties {
    Box::into_raw(Box::new(ZNProperties(config::empty())))
}

/// Create a default set of properties for zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn zn_config_default() -> *mut ZNProperties {
    Box::into_raw(Box::new(ZNProperties(config::default())))
}

/// Create a default set of properties for peer mode zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn zn_config_peer() -> *mut ZNProperties {
    Box::into_raw(Box::new(ZNProperties(config::peer())))
}

/// Create a default set of properties for client mode zenoh-net session configuration.
/// If peer is not null, it is added to the configuration as remote peer.
///
/// Parameters:
///   peer: An optional peer locator.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_config_client(peer: *mut c_char) -> *mut ZNProperties {
    let locator = if peer.is_null() {
        None
    } else if let Ok(locator) = CString::from_raw(peer).into_string() {
        Some(locator)
    } else {
        return std::ptr::null_mut();
    };
    Box::into_raw(Box::new(ZNProperties(config::client(locator))))
}

/// Return the resource name for this query
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_query_res_name(query: *mut ZNQuery) -> *const zn_string {
    let rn = zn_string {
        val: (*query).0.res_name.as_ptr() as *const c_char,
        len: (*query).0.res_name.len() as c_uint,
    };
    Box::into_raw(Box::new(rn))
}

/// Return the predicate for this query
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_query_predicate(query: *mut ZNQuery) -> *const zn_string {
    let pred = zn_string {
        val: (*query).0.predicate.as_ptr() as *const c_char,
        len: (*query).0.predicate.len() as c_uint,
    };
    Box::into_raw(Box::new(pred))
}

/// Create a default subscription info.
#[no_mangle]
pub extern "C" fn zn_subinfo_default() -> *mut ZNSubInfo {
    Box::into_raw(Box::new(ZNSubInfo(SubInfo::default())))
}

/// Create a pull mode subscription info.
#[no_mangle]
pub extern "C" fn zn_subinfo_pull() -> *mut ZNSubInfo {
    let si = SubInfo {
        reliability: Reliability::Reliable,
        mode: SubMode::Pull,
        period: None,
    };
    Box::into_raw(Box::new(ZNSubInfo(si)))
}

/// Get the number of entities scouted  and available as part of
/// the ZNScout
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout_len(si: *mut ZNScout) -> c_uint {
    (*si).0.len() as c_uint
}

/// Get the whatami for the scouted entity at the given index
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout_whatami(si: *mut ZNScout, idx: c_uint) -> c_uint {
    match (*si).0[idx as usize].whatami {
        Some(w) => w as c_uint,
        None => ROUTER as c_uint,
    }
}

/// Get the peer-id for the scouted entity at the given index
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout_peerid(si: *mut ZNScout, idx: c_uint) -> *const c_uchar {
    match &(*si).0[idx as usize].pid {
        Some(v) => v.as_slice().as_ptr() as *const c_uchar,
        None => std::ptr::null(),
    }
}

/// Get the length of the peer-id for the scouted entity at the given index
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout_peerid_len(si: *mut ZNScout, idx: c_uint) -> c_uint {
    match &(*si).0[idx as usize].pid {
        Some(v) => v.as_slice().len() as c_uint,
        None => 0,
    }
}

/// Get the locators for the scouted.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout_locators(si: *mut ZNScout, idx: c_uint) -> *mut ZNLocators {
    let mut vs = vec![];
    match &(*si).0[idx as usize].locators {
        Some(ls) => {
            for l in ls {
                vs.push(CString::new(format!("{}", l)).unwrap())
            }
        }
        None => (),
    }
    Box::into_raw(Box::new(ZNLocators(vs)))
}

/// Get the number of locators for the scouted entity.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout_locators_len(ls: *mut ZNLocators) -> c_uint {
    (*ls).0.len() as c_uint
}

/// Get the locator at the given index.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout_locator_get(ls: *mut ZNLocators, idx: c_uint) -> *const c_char {
    (*ls).0[idx as usize].as_ptr()
}

/// Frees the locators
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout_locators_free(ls: *mut ZNLocators) {
    drop(Box::from_raw(ls))
}

/// The scout mask allows to specify what to scout for.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout(
    what: c_uint,
    config: *mut ZNProperties,
    scout_period: c_ulong,
) -> *mut ZNScout {
    let what = what as ZInt;
    let config = Box::from_raw(config);

    let hellos = task::block_on(async move {
        let mut hs = std::vec::Vec::<Hello>::new();
        let mut stream = zenoh::net::scout(what, (*config).0).await;
        let scout = async {
            while let Some(hello) = stream.next().await {
                hs.push(hello)
            }
        };
        let timeout = async_std::task::sleep(std::time::Duration::from_millis(scout_period as u64));
        FutureExt::race(scout, timeout).await;
        hs
    });
    Box::into_raw(Box::new(ZNScout(hellos)))
}

/// Frees the ZNScout by releasing its associated memory.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout_free(s: *mut ZNScout) {
    drop(Box::from_raw(s))
}

/// Initialise the zenoh runtime logger
///
#[no_mangle]
pub extern "C" fn zn_init_logger() {
    env_logger::init();
}

/// Open a zenoh-net session
///
/// Parameters:
///     config: A set of properties.
///
/// Returns:
///     The created zenoh-net session or null if the creation did not succeed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_open(config: *mut ZNProperties) -> *mut ZNSession {
    let config = Box::from_raw(config);
    let s = task::block_on(async move { open((*config).0).await });
    match s {
        Ok(v) => Box::into_raw(Box::new(ZNSession(v))),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Get informations about an zenoh-net session.
///
/// Parameters:
///     session: A zenoh-net session.
///
/// Returns:
///     A set of properties containing informations on the given zenoh-net session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_info(session: *mut ZNSession) -> *mut ZNProperties {
    let ps = task::block_on((*session).0.info());
    let bps = Box::new(ZNProperties(ps));
    Box::into_raw(bps)
}

/// Close a zenoh-net session.
///
/// Parameters:
///     session: A zenoh-net session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_close(session: *mut ZNSession) {
    task::block_on((*Box::from_raw(session)).0.close()).unwrap();
}

/// Associate a numerical id with the given resource key.
///
/// This numerical id will be used on the network to save bandwidth and
/// ease the retrieval of the concerned resource in the routing tables.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key to map to a numerical id.
///
/// Returns:
///     A numerical id.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_declare_resource(
    session: *mut ZNSession,
    reskey: *mut ZNResKey,
) -> c_ulong {
    if session.is_null() || reskey.is_null() {
        return 0;
    }

    task::block_on((*session).0.declare_resource(&(*reskey).0)).unwrap() as c_ulong
}

/// Write data.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key to write.
///     payload: The value to write.
///     len: The length of the value to write.
/// Returns:
///     ``0`` in case of success, ``1`` in case of failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_write(
    session: *mut ZNSession,
    reskey: *mut ZNResKey,
    payload: *const c_char,
    len: c_uint,
) -> c_int {
    if session.is_null() || reskey.is_null() {
        return 1;
    }

    match task::block_on((*session).0.write(
        &(*reskey).0,
        slice::from_raw_parts(payload as *const u8, len as usize).into(),
    )) {
        Ok(()) => 0,
        _ => 1,
    }
}

/// Declare a :c:type:`ZNPublisher` for the given resource key.
///
/// Written resources that match the given key will only be sent on the network
/// if matching subscribers exist in the system.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key to publish.
///
/// Returns:
///    The created :c:type:`ZNPublisher` or null if the declaration failed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_declare_publisher<'a>(
    session: *mut ZNSession,
    reskey: *mut ZNResKey,
) -> *mut ZNPublisher<'a> {
    if session.is_null() || reskey.is_null() {
        return std::ptr::null_mut();
    }

    Box::into_raw(Box::new(ZNPublisher(
        task::block_on((*session).0.declare_publisher(&(*reskey).0)).unwrap(),
    )))
}

/// Undeclare a :c:type:`ZNPublisher`.
///
/// Parameters:
///     sub: The :c:type:`ZNPublisher` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_undeclare_publisher(publ: *mut ZNPublisher) {
    Box::from_raw(publ);
}
/// Declare a :c:type:`ZNSubscriber` for the given resource key.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key to subscribe.
///     sub_info: The :c:type:`ZNSubInfo` to configure the :c:type:`ZNSubscriber`.
///     callback: The callback function that will be called each time a data matching the subscribed resource is received.
///     arg: A pointer that will be passed to the **callback** on each call.
///
/// Returns:
///    The created :c:type:`ZNSubscriber` or null if the declaration failed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_declare_subscriber(
    session: *mut ZNSession,
    reskey: *mut ZNResKey,
    sub_info: *mut ZNSubInfo,
    callback: extern "C" fn(*const zn_sample, *const c_void),
    arg: *mut c_void,
) -> *mut ZNSubscriber {
    if session.is_null() || reskey.is_null() || sub_info.is_null() {
        return std::ptr::null_mut();
    }

    let s = Box::from_raw(session);
    let si = Box::from_raw(sub_info);
    let arg = Box::from_raw(arg);
    let (tx, rx) = channel::<ZnSubOps>(8);
    let rsub = ZNSubscriber(Some(Arc::new(tx)));
    let mut sub: Subscriber =
        task::block_on((*session).0.declare_subscriber(&(*reskey).0, &si.0)).unwrap();
    // Note: This is done to ensure that even if the call-back into C
    // does any blocking call we do not incour the risk of blocking
    // any of the task resolving futures.
    task::spawn_blocking(move || {
        task::block_on(async move {
            let key = zn_string {
                val: std::ptr::null(),
                len: 0,
            };
            let value = zn_bytes {
                val: std::ptr::null(),
                len: 0,
            };
            let mut sample = zn_sample { key, value };
            let arg = Box::into_raw(arg);

            loop {
                select!(
                    s = sub.stream().next().fuse() => {
                        // This is a bit brutal but avoids an allocation and
                        // a copy that would be otherwise required to add the
                        // C string terminator. See the test_sub.c to find out how to deal
                        // with non null terminated strings.
                        let us = s.unwrap();
                        let data = us.payload.to_vec();
                        sample.key.val = us.res_name.as_ptr() as *const c_char;
                        sample.key.len = us.res_name.len() as c_uint;
                        sample.value.val = data.as_ptr() as *const c_uchar;
                        sample.value.len = data.len() as c_uint;
                        callback(&sample, arg)
                    },
                    op = rx.recv().fuse() => {
                        match op {
                            Ok(ZnSubOps::Pull) => {
                                let _ = sub.pull().await;
                                ()
                            },

                            Ok(ZnSubOps::Close) => {
                                let _ = sub.undeclare().await;
                                Box::into_raw(s);
                                return ()
                            },
                            _ => return ()
                        }
                    }
                )
            }
        })
    });
    Box::into_raw(Box::new(rsub))
}

/// Pull data for a pull mode :c:type:`ZNSubscriber`. The pulled data will be provided
/// by calling the **callback** function provided to the :c:func:`zn_declare_subscriber` function.
///
/// Parameters:
///     sub: The :c:type:`ZNSubscriber` to pull from.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_pull(sub: *mut ZNSubscriber) {
    let sub = Box::from_raw(sub);
    match *sub {
        ZNSubscriber(Some(ref tx)) => smol::block_on(tx.send(ZnSubOps::Pull)),
        ZNSubscriber(None) => (),
    }
    Box::into_raw(sub);
}

/// Undeclare a :c:type:`ZNSubscriber`.
///
/// Parameters:
///     sub: The :c:type:`ZNSubscriber` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_undeclare_subscriber(sub: *mut ZNSubscriber) {
    match *Box::from_raw(sub) {
        ZNSubscriber(Some(tx)) => smol::block_on(tx.send(ZnSubOps::Close)),
        ZNSubscriber(None) => (),
    }
}

/// Query data from the matching queryables in the system.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key to query.
///     predicate: An indication to matching queryables about the queried data.
///     target: The kind of queryables that should be target of this query.
///     consolidation: The kind of consolidation that should be applied on replies.
///     callback: The callback function that will be called on reception of replies for this query.
///     arg: A pointer that will be passed to the **callback** on each call.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_query(
    session: *mut ZNSession,
    reskey: *mut ZNResKey,
    predicate: *const c_char,
    target: *mut ZNQueryTarget,
    consolidation: *mut ZNQueryConsolidation,
    callback: extern "C" fn(*const zn_source_info, *const zn_sample, *const c_void),
    arg: *mut c_void,
) {
    let p = CStr::from_ptr(predicate).to_str().unwrap();
    let qt = Box::from_raw(target);
    let qc = Box::from_raw(consolidation);
    let arg = Box::from_raw(arg);
    let mut q = task::block_on((*session).0.query(&(*reskey).0, p, qt.0, qc.0)).unwrap();

    task::spawn_blocking(move || {
        task::block_on(async move {
            let key = zn_string {
                val: std::ptr::null(),
                len: 0,
            };
            let value = zn_bytes {
                val: std::ptr::null(),
                len: 0,
            };
            let mut sample = zn_sample { key, value };
            let id = zn_bytes {
                val: std::ptr::null(),
                len: 0,
            };
            let mut source_info = zn_source_info { kind: 0, id };
            let arg = Box::into_raw(arg);

            while let Some(reply) = q.next().await {
                source_info.kind = reply.source_kind as c_uint;
                source_info.id.val = reply.replier_id.as_slice().as_ptr() as *const c_uchar;
                source_info.id.len = reply.replier_id.as_slice().len() as c_uint;
                sample.key.val = reply.data.res_name.as_ptr() as *const c_char;
                sample.key.len = reply.data.res_name.len() as c_uint;
                let data = reply.data.payload.to_vec();
                sample.value.val = data.as_ptr() as *const c_uchar;
                sample.value.len = data.len() as c_uint;

                callback(&source_info, &sample, arg)
            }
        })
    });
}

/// Declare a :c:type:`ZNQueryable` for the given resource key.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key the :c:type:`ZNQueryable` will reply to.
///     kind: The kind of :c:type:`ZNQueryable`.
///     callback: The callback function that will be called each time a matching query is received.
///     arg: A pointer that will be passed to the **callback** on each call.
///
/// Returns:
///    The created :c:type:`ZNQueryable` or null if the declaration failed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_declare_queryable(
    session: *mut ZNSession,
    reskey: *mut ZNResKey,
    kind: c_uint,
    callback: extern "C" fn(*mut ZNQuery, *const c_void),
    arg: *mut c_void,
) -> *mut ZNQueryable {
    if session.is_null() || reskey.is_null() {
        return std::ptr::null_mut();
    }

    let arg = Box::from_raw(arg);
    let (tx, rx) = channel::<bool>(1);
    let r = ZNQueryable(Some(Arc::new(tx)));
    let mut queryable: zenoh::net::Queryable =
        task::block_on((*session).0.declare_queryable(&(*reskey).0, kind as ZInt)).unwrap();

    // Note: This is done to ensure that even if the call-back into C
    // does any blocking call we do not incour the risk of blocking
    // any of the task resolving futures.
    task::spawn_blocking(move || {
        task::block_on(async move {
            let arg = Box::into_raw(arg);
            loop {
                select!(
                query = queryable.stream().next().fuse() => {
                  // This is a bit brutal but avoids an allocation and
                  // a copy that would be otherwise required to add the
                  // C string terminator. See the test_sub.c to find out how to deal
                  // with non null terminated strings.
                  let bquery = Box::new(ZNQuery(query.unwrap()));
                  let rbquery = Box::into_raw(bquery);
                  callback(rbquery, arg);
                  Box::from_raw(rbquery);
                },
                _ = rx.recv().fuse() => {
                    let _ = queryable.undeclare().await;
                    return ()
                })
            }
        })
    });
    Box::into_raw(Box::new(r))
}

/// Undeclare a :c:type:`ZNQueryable`.
///
/// Parameters:
///     sub: The :c:type:`ZNQueryable` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_undeclare_queryable(sub: *mut ZNQueryable) {
    match *Box::from_raw(sub) {
        ZNQueryable(Some(tx)) => smol::block_on(tx.send(true)),
        ZNQueryable(None) => (),
    }
}

/// Sends a reply to a query.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_send_reply(
    query: *mut ZNQuery,
    key: *const c_char,
    payload: *const c_uchar,
    len: c_uint,
) {
    let name = CStr::from_ptr(key).to_str().unwrap();
    let s = Sample {
        res_name: name.to_string(),
        payload: slice::from_raw_parts(payload as *const u8, len as usize).into(),
        data_info: None,
    };
    task::block_on((*query).0.replies_sender.send(s));
}

/// Notifies the zenoh runtime that there won't be any more replies sent for this
/// query.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_close_query(query: *mut ZNQuery) {
    let bq = Box::from_raw(query);
    std::mem::drop(bq);
}
