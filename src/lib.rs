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
use async_std::channel::{bounded, Sender};
use async_std::prelude::FutureExt;
use async_std::sync::Arc;
use async_std::task;
use futures::prelude::*;
use futures::select;
use libc::{c_char, c_int, c_uchar, c_uint, c_ulong, size_t};
use std::convert::TryFrom;
use std::ffi::{c_void, CStr, CString};
use std::mem::ManuallyDrop;
use std::slice;
use zenoh::config::whatami::WhatAmIMatcher;
use zenoh::config::{Config, ConfigProperties, IntKeyMapLike, WhatAmI};
use zenoh::prelude::{KeyedSelector, ResKey, Sample, ZFuture, ZInt};
use zenoh::queryable::Query;
use zenoh::scouting::Hello;
use zenoh::Session;

mod types;
pub use types::*;

#[allow(non_camel_case_types)]
pub struct z_session_t(Session);
impl From<Session> for z_session_t {
    fn from(session: Session) -> Self {
        z_session_t(session)
    }
}
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_owned_session_t {
    borrow: *mut z_session_t,
}
impl From<Session> for z_owned_session_t {
    fn from(s: Session) -> Self {
        z_owned_session_t {
            borrow: Box::into_raw(Box::new(s.into())),
        }
    }
}

#[allow(non_camel_case_types)]
pub struct z_config_t(Config);
#[repr(C)]
pub struct z_owned_config_t {
    borrow: *mut z_config_t,
}

enum SubOps {
    Pull,
    Close,
}
#[allow(non_camel_case_types)]
pub struct z_publisher_t<'a>(zenoh::publisher::Publisher<'a>);
#[allow(non_camel_case_types)]
pub struct z_subscriber_t<'a>(
    Option<Arc<Sender<SubOps>>>,
    std::marker::PhantomData<&'a Session>,
);
#[repr(C)]
pub struct z_owned_subscriber_t<'a> {
    borrow: *mut z_subscriber_t<'a>,
}
impl<'a> z_subscriber_t<'a> {
    fn new(inner: Option<Arc<Sender<SubOps>>>) -> Self {
        z_subscriber_t(inner, Default::default())
    }
}
#[allow(non_camel_case_types)]
pub struct z_queryable_t(Option<Arc<Sender<bool>>>);
#[allow(non_camel_case_types)]
pub struct z_query_t(Query);
#[allow(non_camel_case_types)]
pub struct z_scout_t(std::vec::Vec<Hello>);
#[allow(non_camel_case_types)]
pub struct z_locators_t(std::vec::Vec<std::ffi::CString>);

/// Create a resource key from a resource id.
///
/// Parameters:
///     id: The resource id.
///
/// Returns:
///     A new resource key.
#[no_mangle]
pub extern "C" fn z_rid(id: c_ulong) -> z_reskey_t {
    unsafe { z_reskey__new(id, std::ptr::null()) }
}

/// Create a resource key from a resource id and a suffix.
///
/// Parameters:
///     id: The resource id.
///     suffix: The suffix, a NULL terminated string, copied on construction.
///
/// Returns:
///     A new resource key.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_rid_with_suffix(id: c_ulong, suffix: *const c_char) -> z_reskey_t {
    z_reskey__new(id, suffix)
}

/// Create a resource key from a resource name.
///
/// Parameters:
///     id: The resource name, a NULL terminated string, copied on construction.
///
/// Returns:
///     A new resource key.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_rname(name: *const c_char) -> z_reskey_t {
    z_reskey__new(0, name)
}

/// Return a new empty configuration.
#[no_mangle]
pub extern "C" fn z_config__new() -> z_owned_config_t {
    z_owned_config_t {
        borrow: Box::into_raw(Box::new(z_config_t(Config::default()))),
    }
}

/// Get the length of the given properties map.
///
/// Parameters:
///     ps: A pointer to the properties map.
///
/// Returns:
///     The length of the given properties map.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config__len(ps: &z_config_t) -> c_uint {
    ps.0.ikeys().len() as c_uint
}

/// Get the property with the given key from a properties map.
///
/// Parameters:
///     ps: A pointer to properties map.
///     key: The key of the property.
///
/// Returns:
///     The value of the property with key ``key`` in properties map ``ps``.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config__get(ps: &z_config_t, key: c_uint) -> z_string_t {
    let val = ps.0.iget(key as u64);
    match val {
        Some(val) => val.into_owned().into(),
        None => z_string_t::default(),
    }
}

/// Insert a property with a given key to a properties map.
/// If a property with the same key already exists in the properties map, it is replaced.
///
/// Parameters:
///   ps: A pointer to the properties map.
///   key: The key of the property to add.
///   value: The value of the property to add.
///
/// Returns:
///     A pointer to the updated properties map.
#[allow(clippy::missing_safety_doc, unused_must_use)]
#[no_mangle]
pub unsafe extern "C" fn z_config__insert(ps: &mut z_config_t, key: c_ulong, value: z_string_t) {
    ps.0.iset(key as u64, String::from(value));
}

/// Free a set of properties.
///
/// Parameters:
///   ps: A pointer to the properties.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config__free(ps: z_owned_config_t) {
    drop(Box::from_raw(ps.borrow));
}

/// Create an empty set of properties for zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config__empty() -> z_owned_config_t {
    z_owned_config_t {
        borrow: Box::into_raw(Box::new(z_config_t(Config::default()))),
    }
}

/// Create a default set of properties for zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config__default() -> z_owned_config_t {
    z_owned_config_t {
        borrow: Box::into_raw(Box::new(z_config_t(Config::default()))),
    }
}

/// Create a set of properties for zenoh-net session configuration, parsing a string listing the properties
/// in such format: "mode=client;peer=tcp/127.0.0.1:7447".
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config__from_str(s: *const c_char) -> z_owned_config_t {
    if s.is_null() {
        z_config__empty()
    } else {
        let conf_str = CStr::from_ptr(s);
        let props = zenoh::config::ConfigProperties::from(conf_str.to_string_lossy().as_ref());
        z_owned_config_t {
            borrow: Box::into_raw(Box::new(z_config_t(Config::try_from(props).unwrap()))),
        }
    }
}

/// Convert a set of properties into a string.
///
/// Parameters:
///     config: The set of properties.
///
/// Returns:
///     A keys/values string containing with such format: "key1=value1;key2=value2;...".
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config__to_str(config: &z_config_t) -> z_string_t {
    ConfigProperties::from(&config.0).to_string().into()
}

/// Create a set of properties for zenoh-net session configuration, parsing a file listing the properties
/// (1 "key=value" per line, comments starting with '#' character are allowed).
/// Returns null if parsing fails.
///
/// Parameters:
///   path: The path to the file (must be in UTF-8).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config__from_file(path: *const c_char) -> z_owned_config_t {
    let path_str = CStr::from_ptr(path);
    z_owned_config_t {
        borrow: match path_str.to_str() {
            Ok(path) => match zenoh::config::Config::from_file(path) {
                Ok(c) => Box::into_raw(Box::new(z_config_t(c))),
                Err(e) => {
                    log::error!("Couldn't read config from {}: {}", path, e);
                    std::ptr::null_mut()
                }
            },
            Err(e) => {
                log::error!("Invalid path '{}': {}", path_str.to_string_lossy(), e);
                std::ptr::null_mut()
            }
        },
    }
}

/// Create a default set of properties for peer mode zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config__peer() -> z_owned_config_t {
    z_owned_config_t {
        borrow: Box::into_raw(Box::new(z_config_t(
            Config::try_from(zenoh::config::peer()).unwrap(),
        ))),
    }
}

/// Create a default set of properties for client mode zenoh-net session configuration.
/// If peer is not null, it is added to the configuration as remote peer.
///
/// Parameters:
///   peer: An optional peer locator.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config__client(peer: *mut c_char) -> z_owned_config_t {
    let locator = if peer.is_null() {
        None
    } else if let Ok(locator) = CString::from_raw(peer).into_string() {
        Some(locator)
    } else {
        return z_owned_config_t {
            borrow: std::ptr::null_mut(),
        };
    };
    z_owned_config_t {
        borrow: Box::into_raw(Box::new(z_config_t(
            Config::try_from(zenoh::config::client(locator)).unwrap(),
        ))),
    }
}

/// Get the resource name of a received query.
///
/// Parameters:
///     query: The query.
///
/// Returns:
///     The resource name of the query.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query__res_name(query: &z_query_t) -> z_string_t {
    query.0.selector().key_selector.into()
}

/// Get the predicate of a received query.
///
/// Parameters:
///     query: The query.
///
/// Returns:
///     The predicate of the query.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query__predicate(query: &z_query_t) -> z_string_t {
    query.0.selector().value_selector.into()
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
pub unsafe extern "C" fn z_scout(
    what: c_uint,
    config: z_owned_config_t,
    scout_period: c_ulong,
) -> z_hello_array_t {
    let what = WhatAmIMatcher::try_from(what as ZInt).unwrap_or(WhatAmI::Router | WhatAmI::Peer);
    let config = Box::from_raw(config.borrow);

    let hellos = task::block_on(async move {
        let mut hs = std::vec::Vec::<Hello>::new();
        let mut stream = zenoh::scout(what, config.0).wait().unwrap();
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
pub extern "C" fn z_init_logger() {
    env_logger::init();
}

/// Open a zenoh-net session
///
/// Parameters:
///     config: A set of properties.
///
/// Returns:
///     The created zenoh-net session or null if the creation did not succeed, wrapped in the z_owned_session_t type.
///     Later functions do not check for null pointers, so you should do it to prevent segfaults.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_open(config: z_owned_config_t) -> z_owned_session_t {
    let config = Box::from_raw(config.borrow);
    let s = task::block_on(async move { zenoh::open(config.0).await });
    match s {
        Ok(v) => v.into(),
        Err(e) => {
            log::error!("Error opening session: {}", e);
            z_owned_session_t {
                borrow: std::ptr::null_mut(),
            }
        }
    }
}

/// Get informations about an zenoh-net session.
///
/// Parameters:
///     session: A zenoh-net session.
///
/// Returns:
///     A :c:type:`zn_properties_t` map containing informations on the given zenoh-net session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info(session: &z_session_t) -> z_info_t {
    let ps = task::block_on(session.0.info());
    let bps = Box::new(z_info_inner_t(ps));
    z_info_t {
        borrow: Box::into_raw(bps),
    }
}

/// Get informations about an zenoh-net session.
///
/// Parameters:
///     session: A zenoh-net session.
///
/// Returns:
///     A keys/values string containing informations on the given zenoh-net session.
///     The format of the string is: "key1=value1;key2=value2;...".
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_info_as_str(session: &z_session_t) -> z_string_t {
    let ps = task::block_on(session.0.info());
    ps.to_string().into()
}

/// Close a zenoh-net session.
///
/// Parameters:
///     session: A zenoh-net session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_close(session: z_owned_session_t) {
    task::block_on((*Box::from_raw(session.borrow)).0.close()).unwrap();
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
pub unsafe extern "C" fn z_register_resource(
    session: &mut z_session_t,
    reskey: z_reskey_t,
) -> c_ulong {
    // if session.is_null() {
    //     return 0;
    // }
    let reskey = ResKey::from_raw(reskey);
    let result = session.0.register_resource(&reskey).wait().unwrap() as c_ulong;
    ResKey::into_raw(reskey);
    result
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
pub unsafe extern "C" fn z_write(
    session: &mut z_session_t,
    reskey: z_reskey_t,
    payload: *const u8,
    len: c_uint,
) -> c_int {
    let reskey = ResKey::from_raw(reskey);
    let r = session
        .0
        .put(
            &reskey,
            slice::from_raw_parts(payload as *const u8, len as usize),
        )
        .wait();

    ResKey::into_raw(reskey);
    match r {
        Ok(()) => 0,
        _ => 1,
    }
}

/// Write data with extended options.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key to write.
///     payload: The value to write.
///     len: The length of the value to write.
///     encoding: The encoding of the value.
///     kind: The kind of value.
///     congestion_control: The behavior to adopt in case of congestion while routing some data.
/// Returns:
///     ``0`` in case of success, ``1`` in case of failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_write_ext(
    session: &mut z_session_t,
    reskey: z_reskey_t,
    payload: *const u8,
    len: c_uint,
    encoding: c_uint,
    kind: c_uint,
    congestion_control: zn_congestion_control_t,
) -> c_int {
    // if session.is_null() {
    //     return 1;
    // }

    let reskey = ResKey::from_raw(reskey);
    let result = match session
        .0
        .put(
            &reskey,
            slice::from_raw_parts(payload as *const u8, len as usize),
        )
        .encoding(encoding as ZInt)
        .kind((kind as ZInt).into())
        .congestion_control(std::mem::transmute(congestion_control as u8))
        .wait()
    {
        Ok(()) => 0,
        _ => 1,
    };
    ResKey::into_raw(reskey);
    result
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
pub unsafe extern "C" fn z_register_publisher(
    session: &mut z_session_t,
    reskey: z_reskey_t,
) -> *mut z_publisher_t<'_> {
    // if session.is_null() {
    //     return std::ptr::null_mut();
    // }

    let reskey = ResKey::from_raw(reskey);
    let result = Box::into_raw(Box::new(z_publisher_t(
        (*session).0.publishing(reskey).wait().unwrap(),
    )));
    result
}

/// Undeclare a :c:type:`zn_publisher_t`.
///
/// Parameters:
///     sub: The :c:type:`zn_publisher_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_unregister_publisher(publ: *mut z_publisher_t) {
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
pub unsafe extern "C" fn z_register_subscriber(
    session: &mut z_session_t,
    reskey: z_reskey_t,
    sub_info: z_subinfo_t,
    callback: extern "C" fn(*const z_sample_t, *const c_void),
    arg: *mut c_void,
) -> z_owned_subscriber_t<'_> {
    // if session.is_null() {
    //     return std::ptr::null_mut();
    // }

    // let s = Box::from_raw(session);
    let reskey = ResKey::from_raw(reskey);
    let arg = Box::from_raw(arg);
    let (tx, rx) = bounded::<SubOps>(8);
    let rsub = z_subscriber_t::new(Some(Arc::new(tx)));
    let sub = session
        .0
        .subscribe(reskey)
        .period(sub_info.period.into())
        .reliability(sub_info.reliability.into())
        .mode(sub_info.mode.into());
    let sub = sub.wait().unwrap();
    let mut sub = std::mem::transmute::<_, zenoh::subscriber::Subscriber<'static>>(sub);

    // Note: This is done to ensure that even if the call-back into C
    // does any blocking call we do not incur the risk of blocking
    // any of the task resolving futures.
    task::spawn_blocking(move || {
        task::block_on(async move {
            let key = z_string_t::default();
            let value = z_bytes_t {
                val: std::ptr::null(),
                len: 0,
            };
            let mut sample = z_sample_t { key, value };
            let arg = Box::into_raw(arg);
            loop {
                select!(
                    s = sub.receiver().next().fuse() => {
                        // This is a bit brutal but avoids an allocation and
                        // a copy that would be otherwise required to add the
                        // C string terminator. See the test_sub.c to find out how to deal
                        // with non null terminated strings.
                        let us = s.unwrap();
                        let data = us.value.payload.to_vec();
                        z_string__free(sample.key);
                        sample.key = us.res_name.into();
                        sample.value.val = data.as_ptr() as *const c_uchar;
                        sample.value.len = data.len() as size_t;
                        callback(&sample, arg)
                    },
                    op = rx.recv().fuse() => {
                        match op {
                            Ok(SubOps::Pull) => {
                                let _ = sub.pull().await;
                            },

                            Ok(SubOps::Close) => {
                                let _ = sub.unregister().await;
                                return
                            },
                            _ => return
                        }
                    }
                )
            }
        })
    });
    z_owned_subscriber_t {
        borrow: Box::into_raw(Box::new(rsub)),
    }
}

/// Pull data for a pull mode :c:type:`zn_subscriber_t`. The pulled data will be provided
/// by calling the **callback** function provided to the :c:func:`zn_declare_subscriber` function.
///
/// Parameters:
///     sub: The :c:type:`zn_subscriber_t` to pull from.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_pull(sub: &mut z_subscriber_t) {
    match sub {
        z_subscriber_t(Some(tx), _) => {
            let _ = async_std::task::block_on(tx.send(SubOps::Pull));
        }
        z_subscriber_t(None, _) => (),
    }
}

/// Undeclare a :c:type:`zn_subscriber_t`.
///
/// Parameters:
///     sub: The :c:type:`zn_subscriber_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_unregister_subscriber(sub: z_owned_subscriber_t) {
    match *Box::from_raw(sub.borrow) {
        z_subscriber_t(Some(tx), _) => {
            let _ = async_std::task::block_on(tx.send(SubOps::Close));
        }
        z_subscriber_t(None, _) => (),
    }
}

/// Query data from the matching queryables in the system.
/// Replies are provided through a callback function.
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
pub unsafe extern "C" fn z_query(
    session: &mut z_session_t,
    reskey: z_reskey_t,
    predicate: *const c_char,
    target: z_query_target_t,
    consolidation: z_query_consolidation_t,
    callback: extern "C" fn(z_reply_t, *const c_void),
    arg: *mut c_void,
) {
    let p = CStr::from_ptr(predicate).to_str().unwrap();
    let reskey = ResKey::from_raw(reskey);
    let arg = Box::from_raw(arg);
    let mut q = session
        .0
        .get(KeyedSelector {
            key_selector: reskey.clone(),
            value_selector: p,
        })
        .target(target.into())
        .consolidation(consolidation.into())
        .wait()
        .unwrap();

    task::spawn_blocking(move || {
        task::block_on(async move {
            let arg = Box::into_raw(arg);
            while let Some(reply) = q.next().await {
                callback(
                    z_reply_t {
                        tag: z_reply_t_Tag::DATA,
                        data: reply.into(),
                    },
                    arg,
                )
            }
            callback(
                z_reply_t {
                    tag: z_reply_t_Tag::FINAL,
                    data: z_reply_data_t::empty(),
                },
                arg,
            )
            // while let Some(reply) = q.next().await {
            //     callback(zn_reply_t::DATA { data: reply.into() }, arg)
            // }
            // callback(zn_reply_t::FINAL, arg)
        })
    });
}

/// Query data from the matching queryables in the system.
/// Replies are collected in an array.
///
/// Parameters:
///     session: The zenoh-net session.
///     resource: The resource key to query.
///     predicate: An indication to matching queryables about the queried data.
///     target: The kind of queryables that should be target of this query.
///     consolidation: The kind of consolidation that should be applied on replies.
///
/// Returns:
///    An array containing all the replies for this query.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_collect(
    session: &mut z_session_t,
    reskey: z_reskey_t,
    predicate: *const c_char,
    target: z_query_target_t,
    consolidation: z_query_consolidation_t,
) -> z_reply_data_array_t {
    let p = CStr::from_ptr(predicate).to_str().unwrap();
    let reskey = ResKey::from_raw(reskey);
    let mut replies = task::block_on(async {
        let q = session
            .0
            .get(KeyedSelector {
                key_selector: reskey.clone(),
                value_selector: p,
            })
            .target(target.into())
            .consolidation(consolidation.into())
            .await
            .unwrap();
        q.collect::<Vec<_>>().await
    })
    .into_iter()
    .map(|r| r.into())
    .collect::<Vec<z_reply_data_t>>();

    replies.shrink_to_fit();
    //TODO replace when stable https://github.com/rust-lang/rust/issues/65816
    let (val, len, _cap) = vec_into_raw_parts(replies);
    z_reply_data_array_t { val, len }
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
pub unsafe extern "C" fn z_register_queryable(
    session: &mut z_session_t,
    reskey: z_reskey_t,
    kind: c_uint,
    callback: extern "C" fn(&mut z_query_t, *const c_void),
    arg: *mut c_void,
) -> *mut z_queryable_t {
    let arg = Box::from_raw(arg);
    let reskey = ResKey::from_raw(reskey);
    let (tx, rx) = bounded::<bool>(1);
    let r = z_queryable_t(Some(Arc::new(tx)));
    let queryable = session
        .0
        .register_queryable(&reskey)
        .kind(kind as ZInt)
        .wait()
        .unwrap();
    let mut queryable: zenoh::queryable::Queryable<'static> = std::mem::transmute(queryable);

    // Note: This is done to ensure that even if the call-back into C
    // does any blocking call we do not incour the risk of blocking
    // any of the task resolving futures.
    task::spawn_blocking(move || {
        task::block_on(async move {
            let arg = Box::into_raw(arg);
            loop {
                select!(
                query = queryable.receiver().next().fuse() => {
                  // This is a bit brutal but avoids an allocation and
                  // a copy that would be otherwise required to add the
                  // C string terminator. See the test_sub.c to find out how to deal
                  // with non null terminated strings.
                  let mut query = z_query_t(query.unwrap());
                  callback(&mut query, arg);
                },
                _ = rx.recv().fuse() => {
                    let _ = queryable.unregister().await;
                    return
                })
            }
        })
    });
    Box::into_raw(Box::new(r))
}

/// Undeclare a :c:type:`zn_queryable_t`.
///
/// Parameters:
///     qable: The :c:type:`zn_queryable_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_unregister_queryable(qable: *mut z_queryable_t) {
    match *Box::from_raw(qable) {
        z_queryable_t(Some(tx)) => {
            let _ = async_std::task::block_on(tx.send(true));
        }
        z_queryable_t(None) => (),
    }
}

/// Send a reply to a query.
///
/// This function must be called inside of a Queryable callback passing the
/// query received as parameters of the callback function. This function can
/// be called multiple times to send multiple replies to a query. The reply
/// will be considered complete when the Queryable callback returns.
///
/// Parameters:
///     query: The query to reply to.
///     key: The resource key of this reply.
///     payload: The value of this reply.
///     len: The length of the value of this reply.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_send_reply(
    query: &mut z_query_t,
    key: *const c_char,
    payload: *const u8,
    len: c_uint,
) {
    let name = CStr::from_ptr(key).to_str().unwrap();
    let s = Sample::new(
        name.to_string(),
        slice::from_raw_parts(payload as *const u8, len as usize),
    );
    query.0.replies_sender.send(s);
}

//TODO replace when stable https://github.com/rust-lang/rust/issues/65816
#[inline]
pub(crate) fn vec_into_raw_parts<T>(v: Vec<T>) -> (*mut T, usize, usize) {
    let mut me = ManuallyDrop::new(v);
    (me.as_mut_ptr(), me.len(), me.capacity())
}
