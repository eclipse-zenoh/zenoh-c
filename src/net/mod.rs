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

mod types;
pub use types::*;

#[allow(non_camel_case_types)]
pub struct zn_session_t(zenoh::net::Session);
#[allow(non_camel_case_types)]
pub struct zn_reskey_t(zenoh::net::ResKey);
#[allow(non_camel_case_types)]
pub struct zn_properties_t(zenoh::net::Properties);

enum ZnSubOps {
    Pull,
    Close,
}
#[allow(non_camel_case_types)]
pub struct zn_publisher_t<'a>(zenoh::net::Publisher<'a>);
#[allow(non_camel_case_types)]
pub struct zn_subscriber_t(Option<Arc<Sender<ZnSubOps>>>);
#[allow(non_camel_case_types)]
pub struct zn_queryable_t(Option<Arc<Sender<bool>>>);
#[allow(non_camel_case_types)]
pub struct zn_query_t(zenoh::net::Query);
#[allow(non_camel_case_types)]
pub struct zn_scout_t(std::vec::Vec<Hello>);
#[allow(non_camel_case_types)]
pub struct zn_locators_t(std::vec::Vec<std::ffi::CString>);

/// Create a resource key from a resource id.
///
/// Parameters:
///     id: The resource id.
///
/// Returns:
///     Return a new resource key.
#[no_mangle]
pub extern "C" fn zn_rid(id: c_ulong) -> *mut zn_reskey_t {
    Box::into_raw(Box::new(zn_reskey_t(zenoh::net::ResKey::RId(to_zint!(id)))))
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
pub unsafe extern "C" fn zn_rid_with_suffix(
    id: c_ulong,
    suffix: *const c_char,
) -> *mut zn_reskey_t {
    Box::into_raw(Box::new(zn_reskey_t(zenoh::net::ResKey::RIdWithSuffix(
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
pub unsafe extern "C" fn zn_rname(name: *const c_char) -> *mut zn_reskey_t {
    Box::into_raw(Box::new(zn_reskey_t(zenoh::net::ResKey::RName(
        CStr::from_ptr(name).to_str().unwrap().to_string(),
    ))))
}

/// Return a new empty set of properties.
#[no_mangle]
pub extern "C" fn zn_properties_make() -> *mut zn_properties_t {
    Box::into_raw(Box::new(zn_properties_t(zenoh::net::Properties::new())))
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
pub unsafe extern "C" fn zn_properties_len(ps: *mut zn_properties_t) -> c_uint {
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
pub unsafe extern "C" fn zn_property_id(ps: *mut zn_properties_t, n: c_uint) -> c_uint {
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
pub unsafe extern "C" fn zn_property_value(
    ps: *mut zn_properties_t,
    n: c_uint,
) -> *const zn_bytes_t {
    let ptr = (*ps).0[n as usize].1.as_ptr();
    let value = Box::new(zn_bytes_t {
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
    ps: *mut zn_properties_t,
    id: c_ulong,
    value: *const c_char,
) -> *mut zn_properties_t {
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
pub unsafe extern "C" fn zn_properties_free(ps: *mut zn_properties_t) {
    let bps = Box::from_raw(ps);
    drop(bps);
}

/// Create an empty set of properties for zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn zn_config_empty() -> *mut zn_properties_t {
    Box::into_raw(Box::new(zn_properties_t(config::empty())))
}

/// Create a default set of properties for zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn zn_config_default() -> *mut zn_properties_t {
    Box::into_raw(Box::new(zn_properties_t(config::default())))
}

/// Create a default set of properties for peer mode zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn zn_config_peer() -> *mut zn_properties_t {
    Box::into_raw(Box::new(zn_properties_t(config::peer())))
}

/// Create a default set of properties for client mode zenoh-net session configuration.
/// If peer is not null, it is added to the configuration as remote peer.
///
/// Parameters:
///   peer: An optional peer locator.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_config_client(peer: *mut c_char) -> *mut zn_properties_t {
    let locator = if peer.is_null() {
        None
    } else if let Ok(locator) = CString::from_raw(peer).into_string() {
        Some(locator)
    } else {
        return std::ptr::null_mut();
    };
    Box::into_raw(Box::new(zn_properties_t(config::client(locator))))
}

/// Return the resource name for this query
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_query_res_name(query: *mut zn_query_t) -> *const zn_string_t {
    let rn = zn_string_t {
        val: (*query).0.res_name.as_ptr() as *const c_char,
        len: (*query).0.res_name.len() as c_uint,
    };
    Box::into_raw(Box::new(rn))
}

/// Return the predicate for this query
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_query_predicate(query: *mut zn_query_t) -> *const zn_string_t {
    let pred = zn_string_t {
        val: (*query).0.predicate.as_ptr() as *const c_char,
        len: (*query).0.predicate.len() as c_uint,
    };
    Box::into_raw(Box::new(pred))
}

/// Scout for routers and/or peers.
///
/// Parameters:
///     what: A whatami bitmask of zenoh entities kind to scout for.
///     config: A set of properties to configure the scouting.
///     scout_period: The time that should be spent scouting before returnng the results.
///
/// Returns:
///     An array of :c:struct:`zn_hello_t` messages.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_scout(
    what: c_uint,
    config: *mut zn_properties_t,
    scout_period: c_ulong,
) -> zn_hello_array_t {
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
    hellos.into()
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
pub unsafe extern "C" fn zn_open(config: *mut zn_properties_t) -> *mut zn_session_t {
    let config = Box::from_raw(config);
    let s = task::block_on(async move { open((*config).0).await });
    match s {
        Ok(v) => Box::into_raw(Box::new(zn_session_t(v))),
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
pub unsafe extern "C" fn zn_info(session: *mut zn_session_t) -> *mut zn_properties_t {
    let ps = task::block_on((*session).0.info());
    let bps = Box::new(zn_properties_t(ps));
    Box::into_raw(bps)
}

/// Close a zenoh-net session.
///
/// Parameters:
///     session: A zenoh-net session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_close(session: *mut zn_session_t) {
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
    session: *mut zn_session_t,
    reskey: *mut zn_reskey_t,
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
    session: *mut zn_session_t,
    reskey: *mut zn_reskey_t,
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

/// Declare a :c:type:`zn_publisher_t` for the given resource key.
///
/// Written resources that match the given key will only be sent on the network
/// if matching subscribers exist in the system.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key to publish.
///
/// Returns:
///    The created :c:type:`zn_publisher_t` or null if the declaration failed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_declare_publisher<'a>(
    session: *mut zn_session_t,
    reskey: *mut zn_reskey_t,
) -> *mut zn_publisher_t<'a> {
    if session.is_null() || reskey.is_null() {
        return std::ptr::null_mut();
    }

    Box::into_raw(Box::new(zn_publisher_t(
        task::block_on((*session).0.declare_publisher(&(*reskey).0)).unwrap(),
    )))
}

/// Undeclare a :c:type:`zn_publisher_t`.
///
/// Parameters:
///     sub: The :c:type:`zn_publisher_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_undeclare_publisher(publ: *mut zn_publisher_t) {
    Box::from_raw(publ);
}
/// Declare a :c:type:`zn_subscriber_t` for the given resource key.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key to subscribe.
///     sub_info: The :c:type:`zn_subinfo_t` to configure the :c:type:`zn_subscriber_t`.
///     callback: The callback function that will be called each time a data matching the subscribed resource is received.
///     arg: A pointer that will be passed to the **callback** on each call.
///
/// Returns:
///    The created :c:type:`zn_subscriber_t` or null if the declaration failed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_declare_subscriber(
    session: *mut zn_session_t,
    reskey: *mut zn_reskey_t,
    sub_info: zn_subinfo_t,
    callback: extern "C" fn(*const zn_sample_t, *const c_void),
    arg: *mut c_void,
) -> *mut zn_subscriber_t {
    if session.is_null() || reskey.is_null() {
        return std::ptr::null_mut();
    }

    let s = Box::from_raw(session);
    let arg = Box::from_raw(arg);
    let (tx, rx) = channel::<ZnSubOps>(8);
    let rsub = zn_subscriber_t(Some(Arc::new(tx)));
    let mut sub: Subscriber = task::block_on(
        (*session)
            .0
            .declare_subscriber(&(*reskey).0, &sub_info.into()),
    )
    .unwrap();
    // Note: This is done to ensure that even if the call-back into C
    // does any blocking call we do not incour the risk of blocking
    // any of the task resolving futures.
    task::spawn_blocking(move || {
        task::block_on(async move {
            let key = zn_string_t {
                val: std::ptr::null(),
                len: 0,
            };
            let value = zn_bytes_t {
                val: std::ptr::null(),
                len: 0,
            };
            let mut sample = zn_sample_t { key, value };
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

/// Pull data for a pull mode :c:type:`zn_subscriber_t`. The pulled data will be provided
/// by calling the **callback** function provided to the :c:func:`zn_declare_subscriber` function.
///
/// Parameters:
///     sub: The :c:type:`zn_subscriber_t` to pull from.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_pull(sub: *mut zn_subscriber_t) {
    let sub = Box::from_raw(sub);
    match *sub {
        zn_subscriber_t(Some(ref tx)) => smol::block_on(tx.send(ZnSubOps::Pull)),
        zn_subscriber_t(None) => (),
    }
    Box::into_raw(sub);
}

/// Undeclare a :c:type:`zn_subscriber_t`.
///
/// Parameters:
///     sub: The :c:type:`zn_subscriber_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_undeclare_subscriber(sub: *mut zn_subscriber_t) {
    match *Box::from_raw(sub) {
        zn_subscriber_t(Some(tx)) => smol::block_on(tx.send(ZnSubOps::Close)),
        zn_subscriber_t(None) => (),
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
    session: *mut zn_session_t,
    reskey: *mut zn_reskey_t,
    predicate: *const c_char,
    target: zn_query_target_t,
    consolidation: zn_query_consolidation_t,
    callback: extern "C" fn(*const zn_source_info_t, *const zn_sample_t, *const c_void),
    arg: *mut c_void,
) {
    let p = CStr::from_ptr(predicate).to_str().unwrap();
    let arg = Box::from_raw(arg);
    let mut q = task::block_on((*session).0.query(
        &(*reskey).0,
        p,
        target.into(),
        consolidation.into(),
    ))
    .unwrap();

    task::spawn_blocking(move || {
        task::block_on(async move {
            let key = zn_string_t {
                val: std::ptr::null(),
                len: 0,
            };
            let value = zn_bytes_t {
                val: std::ptr::null(),
                len: 0,
            };
            let mut sample = zn_sample_t { key, value };
            let id = zn_bytes_t {
                val: std::ptr::null(),
                len: 0,
            };
            let mut source_info = zn_source_info_t { kind: 0, id };
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

/// Declare a :c:type:`zn_queryable_t` for the given resource key.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key the :c:type:`zn_queryable_t` will reply to.
///     kind: The kind of :c:type:`zn_queryable_t`.
///     callback: The callback function that will be called each time a matching query is received.
///     arg: A pointer that will be passed to the **callback** on each call.
///
/// Returns:
///    The created :c:type:`zn_queryable_t` or null if the declaration failed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_declare_queryable(
    session: *mut zn_session_t,
    reskey: *mut zn_reskey_t,
    kind: c_uint,
    callback: extern "C" fn(*mut zn_query_t, *const c_void),
    arg: *mut c_void,
) -> *mut zn_queryable_t {
    if session.is_null() || reskey.is_null() {
        return std::ptr::null_mut();
    }

    let arg = Box::from_raw(arg);
    let (tx, rx) = channel::<bool>(1);
    let r = zn_queryable_t(Some(Arc::new(tx)));
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
                  let bquery = Box::new(zn_query_t(query.unwrap()));
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

/// Undeclare a :c:type:`zn_queryable_t`.
///
/// Parameters:
///     sub: The :c:type:`zn_queryable_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_undeclare_queryable(sub: *mut zn_queryable_t) {
    match *Box::from_raw(sub) {
        zn_queryable_t(Some(tx)) => smol::block_on(tx.send(true)),
        zn_queryable_t(None) => (),
    }
}

/// Sends a reply to a query.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_send_reply(
    query: *mut zn_query_t,
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
pub unsafe extern "C" fn zn_close_query(query: *mut zn_query_t) {
    let bq = Box::from_raw(query);
    std::mem::drop(bq);
}
