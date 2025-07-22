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
use std::{mem::MaybeUninit, slice::from_raw_parts, str::from_utf8};

use libc::{c_char, c_uint};
use zenoh::config::{Config, WhatAmI};

use crate::{
    result::{self, Z_OK},
    strlen_or_zero,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_internal_string_null, z_owned_string_t, z_string_copy_from_substr, CStringView,
};

#[no_mangle]
pub static Z_ROUTER: c_uint = WhatAmI::Router as c_uint;
#[no_mangle]
pub static Z_PEER: c_uint = WhatAmI::Peer as c_uint;
#[no_mangle]
pub static Z_CLIENT: c_uint = WhatAmI::Client as c_uint;

pub use crate::opaque_types::{z_loaned_config_t, z_moved_config_t, z_owned_config_t};
decl_c_type!(
    owned(z_owned_config_t, option Config),
    loaned(z_loaned_config_t),
);

/// Borrows config.
#[no_mangle]
pub extern "C" fn z_config_loan(this_: &'static z_owned_config_t) -> &'static z_loaned_config_t {
    let this = this_.as_rust_type_ref();
    let this = unsafe { this.as_ref().unwrap_unchecked() };
    this.as_loaned_c_type_ref()
}

/// Mutably borrows config.
#[no_mangle]
pub extern "C" fn z_config_loan_mut(this_: &mut z_owned_config_t) -> &mut z_loaned_config_t {
    let this = this_.as_rust_type_mut();
    let this = unsafe { this.as_mut().unwrap_unchecked() };
    this.as_loaned_c_type_mut()
}

/// Constructs a new empty configuration.
#[no_mangle]
pub extern "C" fn z_config_default(
    this_: &mut MaybeUninit<z_owned_config_t>,
) -> result::z_result_t {
    this_
        .as_rust_type_mut_uninit()
        .write(Some(Config::default()));
    Z_OK
}

/// Constructs config in its gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_config_null(this_: &mut MaybeUninit<z_owned_config_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Clones the config into provided uninitialized memory location.
#[no_mangle]
pub extern "C" fn z_config_clone(
    dst: &mut MaybeUninit<z_owned_config_t>,
    this: &z_loaned_config_t,
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
) -> result::z_result_t {
    zc_config_get_from_substr(this, key, strlen_or_zero(key), out_value_string)
}

/// Gets the property with the given path key from the configuration, and constructs and owned string from it.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_config_get_from_substr(
    this: &z_loaned_config_t,
    key: *const c_char,
    key_len: usize,
    out_value_string: &mut MaybeUninit<z_owned_string_t>,
) -> result::z_result_t {
    let config = this.as_rust_type_ref();
    if key.is_null() {
        z_internal_string_null(out_value_string);
        crate::report_error!("Key should not be null");
        return result::Z_EINVAL;
    }

    let key = match from_utf8(from_raw_parts(key as _, key_len)) {
        Ok(s) => s,
        Err(e) => {
            crate::report_error!("Config key is not a valid utf-8 string: {}", e);
            z_internal_string_null(out_value_string);
            return result::Z_EINVAL;
        }
    };
    let val = config.get_json(key).ok();
    match val {
        Some(val) => {
            z_string_copy_from_substr(
                out_value_string,
                val.as_ptr() as *const libc::c_char,
                val.len(),
            );
            result::Z_OK
        }
        None => {
            crate::report_error!("No value was found in the config for key: '{}'", key);
            z_internal_string_null(out_value_string);
            result::Z_EUNAVAILABLE
        }
    }
}

/// Inserts a JSON-serialized `value` at the `key` position of the configuration.
///
/// Returns 0 if successful, a negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc, unused_must_use)]
pub unsafe extern "C" fn zc_config_insert_json5(
    this: &mut z_loaned_config_t,
    key: *const c_char,
    value: *const c_char,
) -> result::z_result_t {
    zc_config_insert_json5_from_substr(this, key, strlen_or_zero(key), value, strlen_or_zero(value))
}

/// Inserts a JSON-serialized `value` at the `key` position of the configuration.
///
/// Returns 0 if successful, a negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc, unused_must_use)]
pub unsafe extern "C" fn zc_config_insert_json5_from_substr(
    this: &mut z_loaned_config_t,
    key: *const c_char,
    key_len: usize,
    value: *const c_char,
    value_len: usize,
) -> result::z_result_t {
    let config = this.as_rust_type_mut();
    let csk = match CStringView::new_borrowed(key, key_len) {
        Ok(cs) => cs,
        Err(r) => return r,
    };
    let key = match (&csk).try_into() {
        Ok(s) => s,
        Err(e) => {
            crate::report_error!("Config key is not a valid utf-8 string: {}", e);
            return result::Z_EINVAL;
        }
    };
    let csv = match CStringView::new_borrowed(value, value_len) {
        Ok(cs) => cs,
        Err(r) => return r,
    };
    let value = match (&csv).try_into() {
        Ok(s) => s,
        Err(e) => {
            crate::report_error!("Config value is not a valid utf-8 string: {}", e);
            return result::Z_EINVAL;
        }
    };
    match config.insert_json5(key, value) {
        Ok(_) => 0,
        Err(e) => {
            crate::report_error!(
                "Failed to insert value '{}' for key '{}' into config: {}",
                value,
                key,
                e
            );
            result::Z_EGENERIC
        }
    }
}

/// Frees `config`, and resets it to its gravestone state.
#[no_mangle]
pub extern "C" fn z_config_drop(this_: &mut z_moved_config_t) {
    let _ = this_.take_rust_type();
}

/// Returns ``true`` if config is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_internal_config_check(this_: &z_owned_config_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Reads a configuration from a JSON-serialized string, such as '{mode:"client",connect:{endpoints:["tcp/127.0.0.1:7447"]}}'.
///
/// Returns 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_config_from_str(
    this: &mut MaybeUninit<z_owned_config_t>,
    s: *const c_char,
) -> result::z_result_t {
    zc_config_from_substr(this, s, strlen_or_zero(s))
}

/// Reads a configuration from a JSON-serialized substring of specified lenght, such as '{mode:"client",connect:{endpoints:["tcp/127.0.0.1:7447"]}}'.
///
/// Returns 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_config_from_substr(
    this: &mut MaybeUninit<z_owned_config_t>,
    s: *const c_char,
    len: usize,
) -> result::z_result_t {
    z_internal_config_null(this);
    if s.is_null() {
        crate::report_error!("String should not be NULL");
        result::Z_EINVAL
    } else {
        let slice = std::slice::from_raw_parts(s as _, len);
        let conf_str = match std::str::from_utf8(slice) {
            Ok(cs) => cs,
            Err(e) => {
                crate::report_error!("Config should be a valid utf-8 string {}", e);
                return result::Z_EINVAL;
            }
        };
        match json5::from_str(conf_str) {
            Ok(props) => {
                this.as_rust_type_mut_uninit().write(Some(props));
                result::Z_OK
            }
            Err(e) => {
                crate::report_error!("Invalid config string: {}", e);
                result::Z_EPARSE
            }
        }
    }
}

/// Constructs a json string representation of the `config`, such as '{"mode":"client","connect":{"endpoints":["tcp/127.0.0.1:7447"]}}'.
///
/// Returns 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_config_to_string(
    config: &z_loaned_config_t,
    out_config_string: &mut MaybeUninit<z_owned_string_t>,
) -> result::z_result_t {
    let config = config.as_rust_type_ref();
    match json5::to_string(config) {
        Ok(s) => {
            unsafe {
                z_string_copy_from_substr(
                    out_config_string,
                    s.as_ptr() as *const libc::c_char,
                    s.len(),
                )
            };
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Config is not a valid json5: {}", e);
            z_internal_string_null(out_config_string);
            result::Z_EPARSE
        }
    }
}

/// Constructs a configuration by parsing a file at `path` null-terminated string. Currently supported format is JSON5, a superset of JSON.
///
/// Returns 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_config_from_file(
    this: &mut MaybeUninit<z_owned_config_t>,
    path: *const c_char,
) -> result::z_result_t {
    zc_config_from_file_substr(this, path, strlen_or_zero(path))
}

/// Constructs a configuration by parsing a file at `path` susbstring of specified length. Currently supported format is JSON5, a superset of JSON.
///
/// Returns 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_config_from_file_substr(
    this: &mut MaybeUninit<z_owned_config_t>,
    path: *const c_char,
    len: usize,
) -> result::z_result_t {
    z_internal_config_null(this);
    if path.is_null() {
        crate::report_error!("Path should be NULL");
        return result::Z_EINVAL;
    }
    let slice = std::slice::from_raw_parts(path as _, len);
    let path_str = match std::str::from_utf8(slice) {
        Ok(cs) => cs,
        Err(e) => {
            crate::report_error!("Path should be a valid utf-8 string {}", e);
            return result::Z_EINVAL;
        }
    };
    match zenoh::config::Config::from_file(path_str) {
        Ok(c) => {
            this.as_rust_type_mut_uninit().write(Some(c));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Failed to read config from {}: {}", path_str, e);
            result::Z_EPARSE
        }
    }
}

/// Constructs a configuration by parsing a file path stored in ZENOH_CONFIG environmental variable.
///
/// Returns 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zc_config_from_env(
    this: &mut MaybeUninit<z_owned_config_t>,
) -> result::z_result_t {
    match Config::from_env() {
        Ok(c) => {
            this.as_rust_type_mut_uninit().write(Some(c));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Close error: {}", e);
            result::Z_EIO
        }
    }
}
