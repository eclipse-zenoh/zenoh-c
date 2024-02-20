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
use libc::{c_char, c_uint};
use std::ffi::CStr;
use zenoh::config::{Config, ValidatedMap, WhatAmI};

use crate::{impl_guarded_transmute, z_owned_str_t, z_str_null, GuardedTransmute};

#[no_mangle]
/// tags{ api.options.whatami}
/// tags{c.z_router, api.options.whatami.router}
pub static Z_ROUTER: c_uint = WhatAmI::Router as c_uint;
#[no_mangle]
/// tags{c.z_router, api.options.whatami.peer}
pub static Z_PEER: c_uint = WhatAmI::Peer as c_uint;
#[no_mangle]
/// tags{c.z_client, api.options.whatami.client}
pub static Z_CLIENT: c_uint = WhatAmI::Client as c_uint;

#[no_mangle]
/// tags{c.z_config_mode_key, api.config.mode}
pub static Z_CONFIG_MODE_KEY: &c_char = unsafe { &*(b"mode\0".as_ptr() as *const c_char) };
#[no_mangle]
/// tags{c.z_config_connect_key, api.config.connect}
pub static Z_CONFIG_CONNECT_KEY: &c_char =
    unsafe { &*(b"connect/endpoints\0".as_ptr() as *const c_char) };
#[no_mangle]
/// tags{c.z_config_listen_key, api.config.listen}
pub static Z_CONFIG_LISTEN_KEY: &c_char =
    unsafe { &*(b"listen/endpoints\0".as_ptr() as *const c_char) };
#[no_mangle]
/// tags{c.z_config_user_key, api.config.transport.auth.usrpwd.user}
pub static Z_CONFIG_USER_KEY: &c_char =
    unsafe { &*(b"transport/auth/usrpwd/user\0".as_ptr() as *const c_char) };
#[no_mangle]
/// tags{c.z_config_password_key, api.config.transport.auth.usrpwd.password}
pub static Z_CONFIG_PASSWORD_KEY: &c_char =
    unsafe { &*(b"transport/auth/usrpwd/password\0".as_ptr() as *const c_char) };
#[no_mangle]
/// tags{c.z_config_multicast_scouting_key, api.config.scouting.multicast.enabled}
pub static Z_CONFIG_MULTICAST_SCOUTING_KEY: &c_char =
    unsafe { &*(b"scouting/multicast/enabled\0".as_ptr() as *const c_char) };
#[no_mangle]
/// tags{c.z_config_multicast_interface_key, api.config.scouting.multicast.interface}
pub static Z_CONFIG_MULTICAST_INTERFACE_KEY: &c_char =
    unsafe { &*(b"scouting/multicast/interface\0".as_ptr() as *const c_char) };
#[no_mangle]
/// tags{c.z_config_multicast_ipv4_address_key, api.config.scouting.multicast.address}
pub static Z_CONFIG_MULTICAST_IPV4_ADDRESS_KEY: &c_char =
    unsafe { &*(b"scouting/multicast/address\0".as_ptr() as *const c_char) };
#[no_mangle]
/// tags{c.z_config_multicast_port_key, api.config.scouting.multicast.port}
pub static Z_CONFIG_SCOUTING_TIMEOUT_KEY: &c_char =
    unsafe { &*(b"scouting/timeout\0".as_ptr() as *const c_char) };
#[no_mangle]
/// tags{c.z_config_scouting_delay_key, api.config.scouting.delay}
pub static Z_CONFIG_SCOUTING_DELAY_KEY: &c_char =
    unsafe { &*(b"scouting/delay\0".as_ptr() as *const c_char) };
#[no_mangle]
/// tags{c.z_config_add_timestamp_key, api.config.timestamping.enabled}
pub static Z_CONFIG_ADD_TIMESTAMP_KEY: &c_char =
    unsafe { &*(b"timestamping/enabled\0".as_ptr() as *const c_char) };

/// A loaned zenoh configuration.
/// tags{c.z_config_t, api.config}
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_config_t(*const z_owned_config_t);

/// An owned zenoh configuration.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
/// tags{c.z_owned_config_t, api.config}
#[repr(C)]
pub struct z_owned_config_t(*mut ());
impl_guarded_transmute!(Option<Box<Config>>, z_owned_config_t);

impl From<Option<Box<Config>>> for z_owned_config_t {
    fn from(v: Option<Box<Config>>) -> Self {
        v.transmute()
    }
}
/// Returns a :c:type:`z_config_t` loaned from `s`.
/// tags{c.z_config_loan} 
#[no_mangle]
pub extern "C" fn z_config_loan(s: &z_owned_config_t) -> z_config_t {
    z_config_t(s)
}
impl AsRef<Option<Box<Config>>> for z_config_t {
    fn as_ref(&self) -> &Option<Box<Config>> {
        unsafe { (*self.0).as_ref() }
    }
}
impl AsMut<Option<Box<Config>>> for z_config_t {
    fn as_mut(&mut self) -> &mut Option<Box<Config>> {
        unsafe { (*(self.0 as *mut z_owned_config_t)).as_mut() }
    }
}
impl AsRef<Option<Box<Config>>> for z_owned_config_t {
    fn as_ref(&self) -> &Option<Box<Config>> {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Option<Box<Config>>> for z_owned_config_t {
    fn as_mut(&mut self) -> &mut Option<Box<Config>> {
        unsafe { std::mem::transmute(self) }
    }
}
impl z_owned_config_t {
    // tags{}
    pub fn null() -> Self {
        None.into()
    }
}

/// Return a new, zenoh-allocated, empty configuration.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
/// tags{c.z_config_new, api.config.create.empty}
#[no_mangle]
pub extern "C" fn z_config_new() -> z_owned_config_t {
    let config: Box<Config> = Box::default();
    unsafe { z_owned_config_t(std::mem::transmute(Some(config))) }
}

/// Constructs a null safe-to-drop value of 'z_owned_config_t' type
// tags{c.z_config_null}
#[no_mangle]
pub extern "C" fn z_config_null() -> z_owned_config_t {
    z_owned_config_t::null()
}

/// Gets the property with the given path key from the configuration, returning an owned, null-terminated, JSON serialized string.
/// Use `z_drop` to safely deallocate this string
/// tags{c.z_config_get}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_config_get(config: z_config_t, key: *const c_char) -> z_owned_str_t {
    let key = match CStr::from_ptr(key).to_str() {
        Ok(s) => s,
        Err(_) => return z_str_null(),
    };

    let val = config.as_ref().as_ref().and_then(|c| c.get_json(key).ok());
    match val {
        Some(val) => val.as_bytes().into(),
        None => z_str_null(),
    }
}

/// Inserts a JSON-serialized `value` at the `key` position of the configuration.
///
/// Returns 0 if successful, a negative value otherwise.
/// tags{c.z_config_insert_json}
#[no_mangle]
#[allow(clippy::missing_safety_doc, unused_must_use)]
pub unsafe extern "C" fn zc_config_insert_json(
    mut config: z_config_t,
    key: *const c_char,
    value: *const c_char,
) -> i8 {
    let key = CStr::from_ptr(key);
    let value = CStr::from_ptr(value);
    match config
        .as_mut()
        .as_mut()
        .expect("uninitialized config")
        .insert_json5(&key.to_string_lossy(), &value.to_string_lossy())
    {
        Ok(_) => 0,
        Err(_) => i8::MIN,
    }
}

/// Frees `config`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_config_drop, api.config.drop}
pub extern "C" fn z_config_drop(config: &mut z_owned_config_t) {
    std::mem::drop(config.as_mut().take())
}
/// Returns ``true`` if `config` is valid.
/// tags{c.z_config_check, api.config.check}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_config_check(config: &z_owned_config_t) -> bool {
    config.as_ref().is_some()
}

/// Creates a default, zenoh-allocated, configuration.
/// tags{c.z_config_default, api.config.create.default}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_config_default() -> z_owned_config_t {
    z_config_new()
}

/// Reads a configuration from a JSON-serialized string, such as '{mode:"client",connect:{endpoints:["tcp/127.0.0.1:7447"]}}'.
///
/// Passing a null-ptr will result in a gravestone value (`z_check(x) == false`).
/// tags{c.z_config_from_str, api.config.create.from_str}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_config_from_str(s: *const c_char) -> z_owned_config_t {
    if s.is_null() {
        z_config_null()
    } else {
        let conf_str = CStr::from_ptr(s);
        let props: Option<Config> = json5::from_str(&conf_str.to_string_lossy()).ok();
        z_owned_config_t(std::mem::transmute(props.map(Box::new)))
    }
}

/// Converts `config` into a JSON-serialized string, such as '{"mode":"client","connect":{"endpoints":["tcp/127.0.0.1:7447"]}}'.
/// tags{c.z_config_to_string, api.config.to_string}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn zc_config_to_string(config: z_config_t) -> z_owned_str_t {
    let config: &Config = match config.as_ref() {
        Some(c) => c,
        None => return z_str_null(),
    };
    match json5::to_string(config) {
        Ok(s) => s.as_bytes().into(),
        Err(_) => z_str_null(),
    }
}

/// Constructs a configuration by parsing a file at `path`. Currently supported format is JSON5, a superset of JSON.
/// tags{c.z_config_from_file, api.config.create.from_file}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_config_from_file(path: *const c_char) -> z_owned_config_t {
    let path_str = CStr::from_ptr(path);
    z_owned_config_t(std::mem::transmute(match path_str.to_str() {
        Ok(path) => match zenoh::config::Config::from_file(path) {
            Ok(c) => Some(Box::new(c)),
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

/// Constructs a default, zenoh-allocated, peer mode configuration.
/// tags{c.z_config_peer, api.config.create.peer}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config_peer() -> z_owned_config_t {
    unsafe { z_owned_config_t(std::mem::transmute(Some(Box::new(zenoh::config::peer())))) }
}

/// Constructs a default, zenoh-allocated, client mode configuration.
/// If `peer` is not null, it is added to the configuration as remote peer.
/// tags{c.z_config_client, api.config.create.client}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config_client(
    peers: *const *const c_char,
    n_peers: usize,
) -> z_owned_config_t {
    let locators = if peers.is_null() {
        Vec::new()
    } else if let Ok(locators) = std::slice::from_raw_parts(peers, n_peers)
        .iter()
        .map(|&s| CStr::from_ptr(s).to_string_lossy().parse())
        .try_fold(
            Vec::<zenoh::prelude::Locator>::new(),
            |mut acc, it| match it {
                Err(e) => {
                    log::error!("Error parsing peer address: {}", e);
                    Err(())
                }
                Ok(loc) => {
                    acc.push(loc);
                    Ok(acc)
                }
            },
        )
    {
        locators
    } else {
        return z_owned_config_t(std::mem::transmute(None::<Box<Config>>));
    };
    z_owned_config_t(std::mem::transmute(Some(Box::new(zenoh::config::client(
        locators,
    )))))
}
