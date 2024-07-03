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
use std::slice::from_raw_parts;
use std::str::from_utf8;
use zenoh::config::{Config, ValidatedMap, WhatAmI};

use crate::errors::z_error_t;
use crate::transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit};
use crate::{errors, z_owned_string_t, z_string_from_substr, z_string_null};

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

pub use crate::opaque_types::z_loaned_config_t;
pub use crate::opaque_types::z_owned_config_t;
decl_c_type!(
    owned(z_owned_config_t, Option<Config>),
    loaned(z_loaned_config_t, Config)
);

/// Borrows config.
#[no_mangle]
pub extern "C" fn z_config_loan(this: &'static z_owned_config_t) -> &z_loaned_config_t {
    let this = this.as_rust_type_ref();
    let this = unsafe { this.as_ref().unwrap_unchecked() };
    this.as_loaned_c_type_ref()
}

/// Mutably borrows config.
#[no_mangle]
pub extern "C" fn z_config_loan_mut(this: &mut z_owned_config_t) -> &mut z_loaned_config_t {
    let this = this.as_rust_type_mut();
    let this = unsafe { this.as_mut().unwrap_unchecked() };
    this.as_loaned_c_type_mut()
}

/// Constructs a new empty configuration.
#[no_mangle]
pub extern "C" fn z_config_default(this: &mut MaybeUninit<z_owned_config_t>) {
    this.as_rust_type_mut_uninit()
        .write(Some(Config::default()));
}

/// Constructs config in its gravestone state.
#[no_mangle]
pub extern "C" fn z_config_null(this: &mut MaybeUninit<z_owned_config_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Clones the config into provided uninitialized memory location.
#[no_mangle]
pub extern "C" fn z_config_clone(
    this: &z_loaned_config_t,
    dst: &mut MaybeUninit<z_owned_config_t>,
) {
    let src = Some(this.as_rust_type_ref().clone());
    let dst = dst.as_rust_type_mut_uninit();
    dst.write(src);
}

/// Gets the property with the given path key from the configuration, and constructs and owned string from it.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_config_get_from_str(
    this: &z_loaned_config_t,
    key: *const c_char,
    out_value_string: &mut MaybeUninit<z_owned_string_t>,
) -> errors::z_error_t {
    zc_config_get_from_substr(this, key, libc::strlen(key), out_value_string)
}

/// Gets the property with the given path key from the configuration, and constructs and owned string from it.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_config_get_from_substr(
    this: &z_loaned_config_t,
    key: *const c_char,
    key_len: usize,
    out_value_string: &mut MaybeUninit<z_owned_string_t>,
) -> errors::z_error_t {
    let config = this.as_rust_type_ref();
    if key.is_null() {
        z_string_null(out_value_string);
        return errors::Z_EINVAL;
    }

    let key = match from_utf8(from_raw_parts(key as _, key_len)) {
        Ok(s) => s,
        Err(_) => {
            z_string_null(out_value_string);
            return errors::Z_EINVAL;
        }
    };
    let val = config.get_json(key).ok();
    match val {
        Some(val) => {
            z_string_from_substr(
                out_value_string,
                val.as_ptr() as *const libc::c_char,
                val.len(),
            );
            errors::Z_OK
        }
        None => {
            z_string_null(out_value_string);
            errors::Z_EUNAVAILABLE
        }
    }
}

/// Inserts a JSON-serialized `value` at the `key` position of the configuration.
///
/// Returns 0 if successful, a negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc, unused_must_use)]
pub unsafe extern "C" fn zc_config_insert_json(
    this: &mut z_loaned_config_t,
    key: *const c_char,
    value: *const c_char,
) -> errors::z_error_t {
    zc_config_insert_json_from_substr(this, key, libc::strlen(key), value, libc::strlen(value))
}

/// Inserts a JSON-serialized `value` at the `key` position of the configuration.
///
/// Returns 0 if successful, a negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc, unused_must_use)]
pub unsafe extern "C" fn zc_config_insert_json_from_substr(
    this: &mut z_loaned_config_t,
    key: *const c_char,
    key_len: usize,
    value: *const c_char,
    value_len: usize,
) -> errors::z_error_t {
    let config = this.as_rust_type_mut();
    let key = match from_utf8(from_raw_parts(key as _, key_len)) {
        Ok(s) => s,
        Err(_) => return errors::Z_EINVAL,
    };
    let value = match from_utf8(from_raw_parts(value as _, value_len)) {
        Ok(s) => s,
        Err(_) => return errors::Z_EINVAL,
    };
    match config.insert_json5(key, value) {
        Ok(_) => 0,
        Err(_) => errors::Z_EGENERIC,
    }
}

/// Frees `config`, and resets it to its gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_config_drop(this: &mut z_owned_config_t) {
    *this.as_rust_type_mut() = None;
}
/// Returns ``true`` if config is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_config_check(this: &z_owned_config_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Reads a configuration from a JSON-serialized string, such as '{mode:"client",connect:{endpoints:["tcp/127.0.0.1:7447"]}}'.
///
/// Returns 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_config_from_str(
    this: &mut MaybeUninit<z_owned_config_t>,
    s: *const c_char,
) -> errors::z_error_t {
    let mut res = errors::Z_OK;
    if s.is_null() {
        z_config_null(this);
        res = errors::Z_EINVAL;
    } else {
        let conf_str = CStr::from_ptr(s);
        let props: Option<Config> = json5::from_str(&conf_str.to_string_lossy()).ok();
        if props.is_none() {
            res = errors::Z_EPARSE;
        }
        this.as_rust_type_mut_uninit().write(props);
    }
    res
}

/// Constructs a json string representation of the `config`, such as '{"mode":"client","connect":{"endpoints":["tcp/127.0.0.1:7447"]}}'.
///
/// Returns 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_config_to_string(
    config: &z_loaned_config_t,
    out_config_string: &mut MaybeUninit<z_owned_string_t>,
) -> errors::z_error_t {
    let config = config.as_rust_type_ref();
    match json5::to_string(config) {
        Ok(s) => {
            unsafe {
                z_string_from_substr(
                    out_config_string,
                    s.as_ptr() as *const libc::c_char,
                    s.len(),
                )
            };
            errors::Z_OK
        }
        Err(_) => {
            z_string_null(out_config_string);
            errors::Z_EPARSE
        }
    }
}

/// Constructs a configuration by parsing a file at `path`. Currently supported format is JSON5, a superset of JSON.
///
/// Returns 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_config_from_file(
    this: &mut MaybeUninit<z_owned_config_t>,
    path: *const c_char,
) -> errors::z_error_t {
    let path_str = CStr::from_ptr(path);
    let mut res = errors::Z_OK;
    let config = match path_str.to_str() {
        Ok(path) => match zenoh::config::Config::from_file(path) {
            Ok(c) => Some(c),
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
    this.as_rust_type_mut_uninit().write(config);
    res
}

/// Constructs a default peer mode configuration.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_config_peer(this: &mut MaybeUninit<z_owned_config_t>) {
    this.as_rust_type_mut_uninit()
        .write(Some(zenoh::config::peer()));
}

/// Constructs a default, zenoh-allocated, client mode configuration.
///
/// @param peers: Array with `size >= n_peers`, containing peer locators to add to the config.
/// @param n_peers: Number of peers to add to the config.
///
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_config_client(
    this: &mut MaybeUninit<z_owned_config_t>,
    peers: *const *const c_char,
    n_peers: usize,
) -> z_error_t {
    let mut res = errors::Z_OK;
    let locators = if peers.is_null() {
        Vec::new()
    } else if let Ok(locators) = std::slice::from_raw_parts(peers, n_peers)
        .iter()
        .map(|&s| CStr::from_ptr(s).to_string_lossy().parse())
        .try_fold(
            Vec::<zenoh_protocol::core::Locator>::new(),
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
    this.as_rust_type_mut_uninit()
        .write(Some(zenoh::config::client(locators)));
    res
}
