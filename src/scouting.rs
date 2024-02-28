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
use crate::{
    z_closure_hello_call, z_config_check, z_config_default, z_config_null, z_config_t, z_id_t,
    z_owned_closure_hello_t, z_owned_config_t, zc_init_logger,
};
use async_std::task;
use libc::{c_char, c_uint, c_ulong, size_t};
use std::ffi::CString;
use zenoh::scouting::Hello;
use zenoh_protocol::core::{whatami::WhatAmIMatcher, WhatAmI};
use zenoh_util::core::AsyncResolve;

/// An owned array of owned, zenoh allocated, NULL terminated strings.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
/// tags{c.z_owned_str_array_t}
pub struct z_owned_str_array_t {
    pub val: *mut *mut c_char,
    pub len: size_t,
}

/// Frees `strs` and invalidates it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// tags{c.z_str_array_drop}
pub unsafe extern "C" fn z_str_array_drop(strs: &mut z_owned_str_array_t) {
    let locators = Vec::from_raw_parts(strs.val as *mut *const c_char, strs.len, strs.len);
    for locator in locators {
        std::mem::drop(CString::from_raw(locator as *mut c_char));
    }
    strs.val = std::ptr::null_mut();
    strs.len = 0;
}

/// Returns ``true`` if `strs` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// tags{c.z_str_array_check}
pub extern "C" fn z_str_array_check(strs: &z_owned_str_array_t) -> bool {
    !strs.val.is_null()
}

/// An borrowed array of borrowed, zenoh allocated, NULL terminated strings.
#[repr(C)]
/// tags{c.z_str_array_t}
pub struct z_str_array_t {
    pub len: size_t,
    pub val: *const *const c_char,
}

/// Returns a :c:type:`z_str_array_t` loaned from :c:type:`z_owned_str_array_t`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// tags{c.z_str_array_loan}
pub extern "C" fn z_str_array_loan(strs: &z_owned_str_array_t) -> z_str_array_t {
    z_str_array_t {
        val: strs.val as *const _,
        len: strs.len,
    }
}
/// A zenoh-allocated hello message returned by a zenoh entity to a scout message sent with `z_scout`.
///
/// Members:
///   unsigned int whatami: The kind of zenoh entity.
///   z_owned_bytes_t pid: The peer id of the scouted entity (empty if absent).
///   z_owned_str_array_t locators: The locators of the scouted entity.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[repr(C)]
/// tags{c.z_owned_hello_t, api.hello}
pub struct z_owned_hello_t {
    pub _whatami: c_uint,
    pub _pid: z_id_t,
    pub _locators: z_owned_str_array_t,
}
/// A reference-type hello message returned by a zenoh entity to a scout message sent with `z_scout`.
///
/// Members:
///   unsigned int whatami: The kind of zenoh entity.
///   z_owned_bytes_t pid: The peer id of the scouted entity (empty if absent).
///   z_owned_str_array_t locators: The locators of the scouted entity.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[repr(C)]
/// tags{z.z_hello_t, api.hello}
pub struct z_hello_t {
    pub whatami: c_uint,
    pub pid: z_id_t,
    pub locators: z_str_array_t,
}

impl From<Hello> for z_owned_hello_t {
    fn from(h: Hello) -> Self {
        z_owned_hello_t {
            _whatami: h.whatami as c_uint,
            _pid: unsafe { std::mem::transmute(h.zid) },
            _locators: if !h.locators.is_empty() {
                let mut locators = h
                    .locators
                    .into_iter()
                    .map(|l| CString::new(l.to_string()).unwrap().into_raw())
                    .collect::<Vec<_>>();
                let val = locators.as_mut_ptr();
                let len = locators.len();
                std::mem::forget(locators);
                z_owned_str_array_t { val, len }
            } else {
                z_owned_str_array_t {
                    val: std::ptr::null_mut(),
                    len: 0,
                }
            },
        }
    }
}

/// Frees `hello`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_hello_drop}
pub unsafe extern "C" fn z_hello_drop(hello: &mut z_owned_hello_t) {
    z_str_array_drop(&mut hello._locators);
    hello._whatami = 0;
}

/// Returns a :c:type:`z_hello_t` loaned from :c:type:`z_owned_hello_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_hello_loan}
pub extern "C" fn z_hello_loan(hello: &z_owned_hello_t) -> z_hello_t {
    z_hello_t {
        whatami: hello._whatami,
        pid: hello._pid,
        locators: z_str_array_loan(&hello._locators),
    }
}

/// Constructs a gravestone value for hello, useful to steal one from a callback
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_hello_null}
pub extern "C" fn z_hello_null() -> z_owned_hello_t {
    z_owned_hello_t {
        _whatami: 0,
        _pid: z_id_t { id: [0; 16] },
        _locators: z_owned_str_array_t {
            val: std::ptr::null_mut(),
            len: 0,
        },
    }
}
impl Drop for z_owned_hello_t {
    fn drop(&mut self) {
        unsafe { z_hello_drop(self) };
    }
}
/// Returns ``true`` if `hello` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_hello_check}
pub extern "C" fn z_hello_check(hello: &z_owned_hello_t) -> bool {
    hello._whatami != 0 && z_str_array_check(&hello._locators)
}

#[repr(C)]
/// tags{c.z_owned_scouting_config, api.scouting_config}
pub struct z_owned_scouting_config_t {
    _config: z_owned_config_t,
    pub zc_timeout_ms: c_ulong,
    pub zc_what: u8,
}

const DEFAULT_SCOUTING_WHAT: u8 = WhatAmI::Router as u8 | WhatAmI::Peer as u8;
const DEFAULT_SCOUTING_TIMEOUT: c_ulong = 1000;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// tags{c.z_scouting_config_null}
pub extern "C" fn z_scouting_config_null() -> z_owned_scouting_config_t {
    z_owned_scouting_config_t {
        _config: z_config_null(),
        zc_timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
        zc_what: DEFAULT_SCOUTING_WHAT,
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_scouting_config_default, api.scouting_config.create.default}
pub extern "C" fn z_scouting_config_default() -> z_owned_scouting_config_t {
    z_owned_scouting_config_t {
        _config: z_config_default(),
        zc_timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
        zc_what: DEFAULT_SCOUTING_WHAT,
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_scouting_config_from, api.scouting_config.create.from_config}
pub extern "C" fn z_scouting_config_from(config: z_config_t) -> z_owned_scouting_config_t {
    z_owned_scouting_config_t {
        _config: config.as_ref().clone().into(),
        zc_timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
        zc_what: DEFAULT_SCOUTING_WHAT,
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_scouting_config_check}
pub extern "C" fn z_scouting_config_check(config: &z_owned_scouting_config_t) -> bool {
    z_config_check(&config._config)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_scouting_config_drop}
pub extern "C" fn z_scouting_config_drop(config: &mut z_owned_scouting_config_t) {
    std::mem::drop(std::mem::replace(config, z_scouting_config_null()));
}

/// Scout for routers and/or peers.
///
/// Parameters:
///     what: A whatami bitmask of zenoh entities kind to scout for.
///     config: A set of properties to configure the scouting.
///     timeout: The time (in milliseconds) that should be spent scouting.
///
/// Returns 0 if successful, negative values upon failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
/// tags{c.z_scout, api.scout}
pub extern "C" fn z_scout(
    config: &mut z_owned_scouting_config_t,
    callback: &mut z_owned_closure_hello_t,
) -> i8 {
    if cfg!(feature = "logger-autoinit") {
        zc_init_logger();
    }
    let config = std::mem::replace(config, z_scouting_config_null());
    let what = WhatAmIMatcher::try_from(config.zc_what).unwrap_or(WhatAmI::Router | WhatAmI::Peer);
    #[allow(clippy::unnecessary_cast)] // Required for multi-target
    let timeout = config.zc_timeout_ms as u64;
    let mut config = config._config;
    let config = config.as_mut().take().expect("invalid config");
    let mut closure = z_owned_closure_hello_t::empty();
    std::mem::swap(&mut closure, callback);

    task::block_on(async move {
        let scout = zenoh::scout(what, *config)
            .callback(move |h| {
                let mut hello = h.into();
                z_closure_hello_call(&closure, &mut hello)
            })
            .res_async()
            .await
            .unwrap();
        async_std::task::sleep(std::time::Duration::from_millis(timeout)).await;
        std::mem::drop(scout);
    });
    0
}
