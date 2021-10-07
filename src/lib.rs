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
use zenoh::info::InfoProperties;
use zenoh::prelude::{
    Encoding, KeyedSelector, Priority, ResKey, Sample, SampleKind, ZFuture, ZInt,
};
use zenoh::publisher::{CongestionControl, Publisher};
use zenoh::queryable::Query;
use zenoh::scouting::Hello;
use zenoh::Session;

mod types;
pub use types::*;

pub const Z_SESSION_PADDING_U64: usize = 3;
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_owned_session_t([u64; Z_SESSION_PADDING_U64]);
impl From<Session> for z_owned_session_t {
    fn from(session: Session) -> Self {
        unsafe { z_owned_session_t(std::mem::transmute(Some(session))) }
    }
}
impl AsRef<Option<Session>> for z_owned_session_t {
    fn as_ref(&self) -> &Option<Session> {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Option<Session>> for z_owned_session_t {
    fn as_mut(&mut self) -> &mut Option<Session> {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsRef<Option<Session>> for z_session_t {
    fn as_ref(&self) -> &Option<Session> {
        unsafe { (&*self.0).as_ref() }
    }
}
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_session_t(*const z_owned_session_t);
#[no_mangle]
pub extern "C" fn z_session_borrow(s: &z_owned_session_t) -> z_session_t {
    z_session_t(s)
}

pub const Z_CONFIG_PADDING_U64: usize = 66;
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_config_t(*const z_owned_config_t);
#[repr(C)]
pub struct z_owned_config_t([u64; Z_CONFIG_PADDING_U64]);
#[no_mangle]
pub extern "C" fn z_config_borrow(s: &z_owned_config_t) -> z_config_t {
    z_config_t(s)
}
impl AsRef<Option<Config>> for z_config_t {
    fn as_ref(&self) -> &Option<Config> {
        unsafe { (&*self.0).as_ref() }
    }
}
impl AsMut<Option<Config>> for z_config_t {
    fn as_mut(&mut self) -> &mut Option<Config> {
        unsafe { (&mut *(self.0 as *mut z_owned_config_t)).as_mut() }
    }
}
impl AsRef<Option<Config>> for z_owned_config_t {
    fn as_ref(&self) -> &Option<Config> {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Option<Config>> for z_owned_config_t {
    fn as_mut(&mut self) -> &mut Option<Config> {
        unsafe { std::mem::transmute(self) }
    }
}

enum SubOps {
    Pull,
    Close,
}

pub const Z_PUBLISHER_PADDING_U64: usize = 3;
#[repr(C)]
pub struct z_owned_publisher_t([u64; Z_PUBLISHER_PADDING_U64]);
impl AsRef<Option<Publisher<'static>>> for z_owned_publisher_t {
    fn as_ref(&self) -> &Option<Publisher<'static>> {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Option<Publisher<'static>>> for z_owned_publisher_t {
    fn as_mut(&mut self) -> &mut Option<Publisher<'static>> {
        unsafe { std::mem::transmute(self) }
    }
}
type Subscriber = Option<Arc<Sender<SubOps>>>;
pub const Z_SUBSCRIBER_PADDING_U64: usize = 1;
#[allow(non_camel_case_types)]
// pub struct z_subscriber_t<'a>();
#[repr(C)]
pub struct z_owned_subscriber_t([u64; Z_SUBSCRIBER_PADDING_U64]);
impl AsRef<Subscriber> for z_owned_subscriber_t {
    fn as_ref(&self) -> &Subscriber {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Subscriber> for z_owned_subscriber_t {
    fn as_mut(&mut self) -> &mut Subscriber {
        unsafe { std::mem::transmute(self) }
    }
}
// impl<'a> z_subscriber_t<'a> {
//     fn new(inner: Option<Arc<Sender<SubOps>>>) -> Self {
//         z_subscriber_t(inner, Default::default())
//     }
// }
type Queryable = Option<Arc<Sender<bool>>>;
// pub struct z_queryable_t(Option<Arc<Sender<bool>>>);
pub const Z_QUERYABLE_PADDING_U64: usize = 1;
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_owned_queryable_t([u64; Z_QUERYABLE_PADDING_U64]);
impl AsRef<Queryable> for z_owned_queryable_t {
    fn as_ref(&self) -> &Queryable {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Queryable> for z_owned_queryable_t {
    fn as_mut(&mut self) -> &mut Queryable {
        unsafe { std::mem::transmute(self) }
    }
}
#[allow(non_camel_case_types)]
pub struct z_query_t(Query);

/// Create a resource key from a resource id.
///
/// Parameters:
///     id: The resource id.
///
/// Returns:
///     A new resource key.
#[no_mangle]
pub extern "C" fn z_rid(id: c_ulong) -> z_owned_reskey_t {
    unsafe { z_reskey_new(id, std::ptr::null()) }
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
pub unsafe extern "C" fn z_rid_with_suffix(id: c_ulong, suffix: *const c_char) -> z_owned_reskey_t {
    z_reskey_new(id, suffix)
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
pub unsafe extern "C" fn z_rname(name: *const c_char) -> z_owned_reskey_t {
    z_reskey_new(0, name)
}

/// Return a new empty configuration.
#[no_mangle]
pub extern "C" fn z_config_new() -> z_owned_config_t {
    unsafe { z_owned_config_t(std::mem::transmute(Some(Config::default()))) }
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
pub unsafe extern "C" fn z_config_len(ps: &z_config_t) -> c_uint {
    ps.as_ref()
        .as_ref()
        .map(|c| c.ikeys().len() as c_uint)
        .unwrap_or(0)
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
pub unsafe extern "C" fn z_config_get(ps: &z_config_t, key: c_uint) -> z_owned_string_t {
    let val = ps.as_ref().as_ref().map(|c| c.iget(key as u64)).flatten();
    match val {
        Some(val) => val.into_owned().into(),
        None => z_owned_string_t::default(),
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
pub unsafe extern "C" fn z_config_set(mut ps: z_config_t, key: c_ulong, value: z_owned_string_t) {
    ps.as_mut()
        .as_mut()
        .unwrap()
        .iset(key as u64, String::from(value));
}

/// Free a set of properties.
///
/// Parameters:
///   ps: A pointer to the properties.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config_free(ps: &mut z_owned_config_t) {
    std::mem::drop(ps.as_mut().take())
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config_check(ps: &z_owned_config_t) -> bool {
    ps.as_ref().is_some()
}

/// Create an empty set of properties for zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config_empty() -> z_owned_config_t {
    z_config_new()
}

/// Create a default set of properties for zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config_default() -> z_owned_config_t {
    z_config_new()
}

/// Create a set of properties for zenoh-net session configuration, parsing a string listing the properties
/// in such format: "mode=client;peer=tcp/127.0.0.1:7447".
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config_from_str(s: *const c_char) -> z_owned_config_t {
    if s.is_null() {
        z_config_empty()
    } else {
        let conf_str = CStr::from_ptr(s);
        let props = zenoh::config::ConfigProperties::from(conf_str.to_string_lossy().as_ref());
        z_owned_config_t(std::mem::transmute(Config::try_from(props).ok()))
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
pub extern "C" fn z_config_to_str(config: &z_config_t) -> z_owned_string_t {
    let config = match config.as_ref() {
        Some(c) => c,
        None => return z_owned_string_t::default(),
    };
    ConfigProperties::from(config).to_string().into()
}

/// Create a set of properties for zenoh-net session configuration, parsing a file listing the properties
/// (1 "key=value" per line, comments starting with '#' character are allowed).
/// Returns null if parsing fails.
///
/// Parameters:
///   path: The path to the file (must be in UTF-8).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config_from_file(path: *const c_char) -> z_owned_config_t {
    let path_str = CStr::from_ptr(path);
    z_owned_config_t(std::mem::transmute(match path_str.to_str() {
        Ok(path) => match zenoh::config::Config::from_file(path) {
            Ok(c) => Some(c),
            Err(e) => {
                log::error!("Couldn't read config from {}: {}", path, e);
                None
            }
        },
        Err(e) => {
            log::error!("Invalid path '{}': {}", path_str.to_string_lossy(), e);
            None
        }
    }))
}

/// Create a default set of properties for peer mode zenoh-net session configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config_peer() -> z_owned_config_t {
    unsafe {
        z_owned_config_t(std::mem::transmute(
            Config::try_from(zenoh::config::peer()).ok(),
        ))
    }
}

/// Create a default set of properties for client mode zenoh-net session configuration.
/// If peer is not null, it is added to the configuration as remote peer.
///
/// Parameters:
///   peer: An optional peer locator.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config_client(peer: *mut c_char) -> z_owned_config_t {
    let locator = if peer.is_null() {
        None
    } else if let Ok(locator) = CString::from_raw(peer).into_string() {
        Some(locator)
    } else {
        return z_owned_config_t(std::mem::transmute(None::<Config>));
    };
    z_owned_config_t(std::mem::transmute(
        Config::try_from(zenoh::config::client(locator)).ok(),
    ))
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
pub extern "C" fn z_query_res_name(query: &z_query_t) -> z_owned_string_t {
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
pub extern "C" fn z_query_predicate(query: &z_query_t) -> z_owned_string_t {
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
    config: &mut z_owned_config_t,
    scout_period: c_ulong,
) -> z_owned_hello_array_t {
    let what = WhatAmIMatcher::try_from(what as ZInt).unwrap_or(WhatAmI::Router | WhatAmI::Peer);
    let config = config.as_mut().take().unwrap();

    let hellos = task::block_on(async move {
        let mut hs = std::vec::Vec::<Hello>::new();
        let mut stream = zenoh::scout(what, config).wait().unwrap();
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
pub unsafe extern "C" fn z_open(config: &mut z_owned_config_t) -> z_owned_session_t {
    let config = match config.as_mut().take() {
        Some(c) => c,
        None => return z_owned_session_t(std::mem::transmute(None::<Session>)),
    };
    let s = task::block_on(async move { zenoh::open(config).await });
    match s {
        Ok(v) => v.into(),
        Err(e) => {
            log::error!("Error opening session: {}", e);
            z_owned_session_t(std::mem::transmute(None::<Session>))
        }
    }
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_session_check(config: &z_owned_session_t) -> bool {
    config.as_ref().is_some()
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
pub unsafe extern "C" fn z_info(session: z_session_t) -> z_owned_info_t {
    let session = (&*session.0).as_ref();
    match session {
        Some(s) => z_owned_info_t(std::mem::transmute(task::block_on(s.info()))),
        None => z_owned_info_t(std::mem::transmute(None::<InfoProperties>)),
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
pub unsafe extern "C" fn z_info_as_str(session: z_session_t) -> z_owned_string_t {
    let session = (&*session.0).as_ref();
    match session {
        Some(s) => task::block_on(s.info()).to_string().into(),
        None => z_owned_string_t::default(),
    }
}

/// Close a zenoh-net session.
///
/// Parameters:
///     session: A zenoh-net session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_close(session: &mut z_owned_session_t) {
    session.as_mut().take().map(|s| task::block_on(s.close()));
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
    session: z_session_t,
    reskey: z_owned_reskey_t,
) -> z_owned_reskey_t {
    let result = session
        .as_ref()
        .as_ref()
        .unwrap()
        .register_resource(ResKey::from_raw(reskey))
        .wait()
        .unwrap() as c_ulong;
    z_rid(result)
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
    session: z_session_t,
    reskey: z_reskey_t,
    payload: *const u8,
    len: c_uint,
) -> c_int {
    let r = session
        .as_ref()
        .as_ref()
        .unwrap()
        .put(
            reskey,
            slice::from_raw_parts(payload as *const u8, len as usize),
        )
        .wait();

    match r {
        Ok(()) => 0,
        _ => 1,
    }
}

#[allow(non_camel_case_types)]
#[derive(Default)]
pub struct z_write_options_t {
    encoding: Encoding,
    congestion_control: CongestionControl,
    kind: SampleKind,
    priority: Priority,
}

#[repr(C)]
pub enum z_write_options_field_t {
    ENCODING,
    CONGESTION_CONTROL,
    KIND,
    PRIORITY,
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_write_options_set(
    options: &mut z_write_options_t,
    key: z_write_options_field_t,
    value: c_uint,
) {
    match key {
        z_write_options_field_t::ENCODING => options.encoding = Encoding::from(value as ZInt),
        z_write_options_field_t::CONGESTION_CONTROL => {
            if value < 2 {
                options.congestion_control = std::mem::transmute(value as u8)
            }
        }
        z_write_options_field_t::KIND => options.kind = (value as ZInt).into(),
        z_write_options_field_t::PRIORITY => {
            if 0 < value && value < 8 {
                options.priority = std::mem::transmute(value as u8)
            }
        }
    };
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
    session: z_session_t,
    reskey: z_reskey_t,
    payload: *const u8,
    len: c_uint,
    options: &z_write_options_t,
) -> c_int {
    let result = match session
        .as_ref()
        .as_ref()
        .unwrap()
        .put(
            reskey,
            slice::from_raw_parts(payload as *const u8, len as usize),
        )
        .encoding(options.encoding.clone())
        .kind(options.kind)
        .congestion_control(options.congestion_control)
        .priority(options.priority)
        .wait()
    {
        Ok(()) => 0,
        _ => 1,
    };
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
    session: z_session_t,
    reskey: z_reskey_t,
) -> z_owned_publisher_t {
    let publisher = std::mem::transmute::<_, &'static z_session_t>(session)
        .as_ref()
        .as_ref()
        .map(|s| s.publishing(reskey).wait().ok())
        .flatten();
    z_owned_publisher_t(std::mem::transmute(publisher))
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_publisher_check(publ: &z_owned_publisher_t) -> bool {
    publ.as_ref().is_some()
}

/// Undeclare a :c:type:`zn_publisher_t`.
///
/// Parameters:
///     sub: The :c:type:`zn_publisher_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_unregister_publisher(publ: &mut z_owned_publisher_t) {
    std::mem::drop(publ.as_mut().take())
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
    session: z_session_t,
    reskey: z_reskey_t,
    sub_info: z_subinfo_t,
    callback: extern "C" fn(*const z_sample_t, *const c_void),
    arg: *mut c_void,
) -> z_owned_subscriber_t {
    let arg = Box::from_raw(arg);
    let (tx, rx) = bounded::<SubOps>(8);
    let rsub = z_owned_subscriber_t(std::mem::transmute(Some(Arc::new(tx))));
    let sub = session
        .as_ref()
        .as_ref()
        .unwrap()
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
            let mut key = vec![0u8];
            let mut sample = z_sample_t {
                key: key.as_ptr() as *const i8,
                value: z_bytes_t {
                    val: std::ptr::null(),
                    len: 0,
                },
            };
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
                        key.clear();
                        key.extend(us.res_name.bytes());
                        key.push(0);
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
    rsub
}

/// Pull data for a pull mode :c:type:`zn_subscriber_t`. The pulled data will be provided
/// by calling the **callback** function provided to the :c:func:`zn_declare_subscriber` function.
///
/// Parameters:
///     sub: The :c:type:`zn_subscriber_t` to pull from.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_pull(sub: &z_owned_subscriber_t) {
    match sub.as_ref() {
        Some(tx) => {
            let _ = async_std::task::block_on(tx.send(SubOps::Pull));
        }
        None => (),
    }
}

/// Undeclare a :c:type:`zn_subscriber_t`.
///
/// Parameters:
///     sub: The :c:type:`zn_subscriber_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_unregister_subscriber(sub: &mut z_owned_subscriber_t) {
    let sub = sub.as_mut();
    match sub {
        Some(tx) => {
            let _ = async_std::task::block_on(tx.send(SubOps::Close));
            *sub = None;
        }
        None => (),
    }
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_subscriber_check(sub: &z_owned_subscriber_t) -> bool {
    sub.as_ref().is_some()
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
    session: z_session_t,
    reskey: z_reskey_t,
    predicate: *const c_char,
    target: z_query_target_t,
    consolidation: z_query_consolidation_t,
    callback: extern "C" fn(z_owned_reply_t, *const c_void),
    arg: *mut c_void,
) {
    let p = CStr::from_ptr(predicate).to_str().unwrap();
    let arg = Box::from_raw(arg);
    let mut q = session
        .as_ref()
        .as_ref()
        .unwrap()
        .get(KeyedSelector {
            key_selector: reskey.into(),
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
                    z_owned_reply_t {
                        tag: z_reply_t_Tag::DATA,
                        data: reply.into(),
                    },
                    arg,
                )
            }
            callback(
                z_owned_reply_t {
                    tag: z_reply_t_Tag::FINAL,
                    data: z_owned_reply_data_t::empty(),
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
    session: z_session_t,
    reskey: z_reskey_t,
    predicate: *const c_char,
    target: z_query_target_t,
    consolidation: z_query_consolidation_t,
) -> z_owned_reply_data_array_t {
    let p = CStr::from_ptr(predicate).to_str().unwrap();
    let mut replies = task::block_on(async {
        let q = session
            .as_ref()
            .as_ref()
            .unwrap()
            .get(KeyedSelector {
                key_selector: reskey.into(),
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
    .collect::<Vec<z_owned_reply_data_t>>();

    replies.shrink_to_fit();
    //TODO replace when stable https://github.com/rust-lang/rust/issues/65816
    let (val, len, _cap) = vec_into_raw_parts(replies);
    z_owned_reply_data_array_t { val, len }
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
    session: z_session_t,
    reskey: z_reskey_t,
    kind: c_uint,
    callback: extern "C" fn(&z_query_t, *const c_void),
    arg: *mut c_void,
) -> z_owned_queryable_t {
    let arg = Box::from_raw(arg);
    let (tx, rx) = bounded::<bool>(1);
    let r = z_owned_queryable_t(std::mem::transmute(Some(Arc::new(tx))));
    let queryable = session
        .as_ref()
        .as_ref()
        .unwrap()
        .register_queryable(reskey)
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
                  let query = z_query_t(query.unwrap());
                  callback(&query, arg);
                },
                _ = rx.recv().fuse() => {
                    let _ = queryable.unregister().await;
                    return
                })
            }
        })
    });
    r
}

/// Undeclare a :c:type:`zn_queryable_t`.
///
/// Parameters:
///     qable: The :c:type:`zn_queryable_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_unregister_queryable(qable: &mut z_owned_queryable_t) {
    let qable = qable.as_mut();
    match qable {
        Some(tx) => {
            let _ = async_std::task::block_on(tx.send(true));
            *qable = None;
        }
        None => (),
    }
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_queryable_check(qable: &z_owned_queryable_t) -> bool {
    qable.as_ref().is_some()
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
    query: &z_query_t,
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
