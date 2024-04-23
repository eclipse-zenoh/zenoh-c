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
use std::mem::MaybeUninit;
use zenoh::config::{Config, ValidatedMap, WhatAmI};

use crate::errors::{ZCError, Z_EPARSE};
use crate::transmute::{
    unwrap_ref_unchecked, Inplace, TransmuteFromHandle, TransmuteIntoHandle, TransmuteRef,
    TransmuteUninitPtr,
};
use crate::{errors, z_owned_str_t, z_str_null};

#[no_mangle]
pub static Z_ROUTER: c_uint = WhatAmI::Router as c_uint;
#[no_mangle]
pub static Z_PEER: c_uint = WhatAmI::Peer as c_uint;
#[no_mangle]
pub static Z_CLIENT: c_uint = WhatAmI::Client as c_uint;

#[no_mangle]
pub static Z_CONFIG_MODE_KEY: &c_char = unsafe { &*(b"mode\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_CONNECT_KEY: &c_char =
    unsafe { &*(b"connect/endpoints\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_LISTEN_KEY: &c_char =
    unsafe { &*(b"listen/endpoints\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_USER_KEY: &c_char =
    unsafe { &*(b"transport/auth/usrpwd/user\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_PASSWORD_KEY: &c_char =
    unsafe { &*(b"transport/auth/usrpwd/password\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_MULTICAST_SCOUTING_KEY: &c_char =
    unsafe { &*(b"scouting/multicast/enabled\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_MULTICAST_INTERFACE_KEY: &c_char =
    unsafe { &*(b"scouting/multicast/interface\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_MULTICAST_IPV4_ADDRESS_KEY: &c_char =
    unsafe { &*(b"scouting/multicast/address\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_SCOUTING_TIMEOUT_KEY: &c_char =
    unsafe { &*(b"scouting/timeout\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_SCOUTING_DELAY_KEY: &c_char =
    unsafe { &*(b"scouting/delay\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_ADD_TIMESTAMP_KEY: &c_char =
    unsafe { &*(b"timestamping/enabled\0".as_ptr() as *const c_char) };

pub use crate::opaque_types::z_config_t;
decl_transmute_handle!(Config, z_config_t);

pub use crate::opaque_types::z_owned_config_t;
decl_transmute_owned!(Option<Box<Config>>, z_owned_config_t);

/// Returns a :c:type:`z_config_t` loaned from `s`.
#[no_mangle]
pub extern "C" fn z_config_loan(s: &'static z_owned_config_t) -> z_config_t {
    let s = s.transmute_ref();
    let s = unwrap_ref_unchecked(s);
    let s = s.as_ref();
    s.transmute_handle()
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
#[no_mangle]
pub extern "C" fn z_config_new(this: *mut MaybeUninit<z_owned_config_t>) {
    let this = this.transmute_uninit_ptr();
    let config: Box<Config> = Box::default();
    Inplace::init(this, Some(config));
}

/// Constructs a null safe-to-drop value of 'z_owned_config_t' type
#[no_mangle]
pub extern "C" fn z_config_null(this: *mut MaybeUninit<z_owned_config_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Clones the config.
#[no_mangle]
pub extern "C" fn z_config_clone(src: &z_config_t, dst: *mut MaybeUninit<z_owned_config_t>) {
    let src = src.transmute_ref();
    let src = Box::new(src.clone());
    let dst = dst.transmute_uninit_ptr();
    Inplace::init(dst, Some(src));
}

/// Gets the property with the given path key from the configuration, returning an owned, null-terminated, JSON serialized string.
/// Use `z_drop` to safely deallocate this string
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_config_get(config: z_config_t, key: *const c_char) -> z_owned_str_t {
    let config = config.transmute_ref();
    let key = match CStr::from_ptr(key).to_str() {
        Ok(s) => s,
        Err(_) => return z_str_null(),
    };
    let val = config.get_json(key).ok();
    match val {
        Some(val) => val.as_bytes().into(),
        None => z_str_null(),
    }
}

/// Inserts a JSON-serialized `value` at the `key` position of the configuration.
///
/// Returns 0 if successful, a negative value otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc, unused_must_use)]
pub unsafe extern "C" fn zc_config_insert_json(
    config: z_config_t,
    key: *const c_char,
    value: *const c_char,
) -> i8 {
    let config = config.transmute_mut();
    let key = CStr::from_ptr(key);
    let value = CStr::from_ptr(value);
    match config.insert_json5(&key.to_string_lossy(), &value.to_string_lossy()) {
        Ok(_) => 0,
        Err(_) => i8::MIN,
    }
}

/// Frees `config`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_config_drop(config: &mut z_owned_config_t) {
    let config = config.transmute_mut();
    Inplace::drop(config);
}
/// Returns ``true`` if `config` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_config_check(config: &z_owned_config_t) -> bool {
    let config = config.transmute_ref();
    config.as_ref().is_some()
}

/// Reads a configuration from a JSON-serialized string, such as '{mode:"client",connect:{endpoints:["tcp/127.0.0.1:7447"]}}'.
///
/// Passing a null-ptr will result in a gravestone value (`z_check(x) == false`).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_config_from_str(
    s: *const c_char,
    this: *mut MaybeUninit<z_owned_config_t>,
) -> errors::ZCError {
    let mut res = errors::Z_OK;
    if s.is_null() {
        z_config_null(this);
        res = errors::Z_EINVAL;
    } else {
        let conf_str = CStr::from_ptr(s);
        let props: Option<Box<Config>> = json5::from_str(&conf_str.to_string_lossy())
            .ok()
            .map(|v| Box::new(v));
        if props.is_none() {
            res = Z_EPARSE;
        }
        Inplace::init(this.transmute_uninit_ptr(), props);
    }
    res
}

/// Converts `config` into a JSON-serialized string, such as '{"mode":"client","connect":{"endpoints":["tcp/127.0.0.1:7447"]}}'.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn zc_config_to_string(config: z_config_t) -> z_owned_str_t {
    let config: &Config = config.transmute_ref();
    match json5::to_string(config) {
        Ok(s) => s.as_bytes().into(),
        Err(_) => z_str_null(),
    }
}

/// Constructs a configuration by parsing a file at `path`. Currently supported format is JSON5, a superset of JSON.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_config_from_file(
    path: *const c_char,
    this: *mut MaybeUninit<z_owned_config_t>,
) -> errors::ZCError {
    let path_str = CStr::from_ptr(path);
    let mut res = errors::Z_OK;
    let config = match path_str.to_str() {
        Ok(path) => match zenoh::config::Config::from_file(path) {
            Ok(c) => Some(Box::new(c)),
            Err(e) => {
                log::error!("Couldn't read config from {}: {}", path, e);
                res = errors::Z_EPARSE;
                None
            }
        },
        Err(e) => {
            log::error!("Invalid path '{}': {}", path_str.to_string_lossy(), e);
            res = errors::Z_EIO;
            None
        }
    };
    Inplace::init(this.transmute_uninit_ptr(), config);
    res
}

/// Constructs a default, zenoh-allocated, peer mode configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config_peer(this: *mut MaybeUninit<z_owned_config_t>) {
    Inplace::init(
        this.transmute_uninit_ptr(),
        Some(Box::new(zenoh::config::peer())),
    );
}

/// Constructs a default, zenoh-allocated, client mode configuration.
/// If `peer` is not null, it is added to the configuration as remote peer.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config_client(
    peers: *const *const c_char,
    n_peers: usize,
    this: *mut MaybeUninit<z_owned_config_t>,
) -> ZCError {
    let mut res = errors::Z_OK;
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
                    res = errors::Z_EPARSE;
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
        z_config_null(this);
        return res;
    };
    Inplace::init(
        this.transmute_uninit_ptr(),
        Some(Box::new(zenoh::config::client(locators))),
    );
    res
}
