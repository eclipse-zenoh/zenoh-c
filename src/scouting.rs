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
    errors::{self, Z_OK}, transmute::{Inplace, TransmuteRef}, z_closure_hello_call, z_config_check, z_config_clone, z_config_drop, z_config_new, z_config_null, z_config_t, z_id_t, z_owned_closure_hello_t, z_owned_config_t, zc_init_logger, CopyableToCArray
};
use async_std::task;
use libc::{c_char, c_uint, c_ulong, size_t};
use std::{ffi::CString, mem::MaybeUninit, os::raw::c_void};
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
pub struct z_owned_str_array_t {
    pub val: *mut *mut c_char,
    pub len: size_t,
}

/// Frees `strs` and invalidates it for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
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
pub extern "C" fn z_str_array_check(strs: &z_owned_str_array_t) -> bool {
    !strs.val.is_null()
}

/// An borrowed array of borrowed, zenoh allocated, NULL terminated strings.
#[repr(C)]
pub struct z_str_array_t {
    pub len: size_t,
    pub val: *const *const c_char,
}

/// Returns a :c:type:`z_str_array_t` loaned from :c:type:`z_owned_str_array_t`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
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
pub unsafe extern "C" fn z_hello_drop(hello: &mut z_owned_hello_t) {
    z_str_array_drop(&mut hello._locators);
    hello._whatami = 0;
}

/// Returns a :c:type:`z_hello_t` loaned from :c:type:`z_owned_hello_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
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
pub extern "C" fn z_hello_null() -> z_owned_hello_t {
    z_owned_hello_t {
        _whatami: 0,
        _pid: [0; 16].into(),
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
pub extern "C" fn z_hello_check(hello: &z_owned_hello_t) -> bool {
    hello._whatami != 0 && z_str_array_check(&hello._locators)
}

#[repr(C)]
pub struct z_owned_scouting_config_t {
    _config: z_owned_config_t,
    pub zc_timeout_ms: c_ulong,
    pub zc_what: u8,
}

pub const DEFAULT_SCOUTING_WHAT: u8 = WhatAmI::Router as u8 | WhatAmI::Peer as u8;
pub const DEFAULT_SCOUTING_TIMEOUT: c_ulong = 1000;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_scouting_config_null(this: *mut MaybeUninit<z_owned_scouting_config_t>) {
    let mut _config = MaybeUninit::<z_owned_config_t>::uninit();
    z_config_null(&mut _config as *mut MaybeUninit<z_owned_config_t>);
    let _config = unsafe { _config.assume_init() };

    let config = z_owned_scouting_config_t {
        _config,
        zc_timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
        zc_what: DEFAULT_SCOUTING_WHAT,
    };
    (unsafe { &mut *this }).write(config);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_scouting_config_default(this: *mut MaybeUninit<z_owned_scouting_config_t>) {
    let mut _config =  MaybeUninit::<z_owned_config_t>::uninit();
    z_config_new(&mut _config as *mut MaybeUninit<z_owned_config_t>);
    let _config = unsafe { _config.assume_init() };

    let config = z_owned_scouting_config_t {
        _config,
        zc_timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
        zc_what: DEFAULT_SCOUTING_WHAT,
    };
    (unsafe { &mut *this }).write(config);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_scouting_config_from(config: z_config_t, this: *mut MaybeUninit<z_owned_scouting_config_t>) {
    let mut dst = MaybeUninit::uninit();
    z_config_clone(&config, &mut dst as *mut _);
    let _config = unsafe { dst.assume_init() };

    let config = z_owned_scouting_config_t {
        _config,
        zc_timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
        zc_what: DEFAULT_SCOUTING_WHAT,
    };
    (unsafe { &mut *this }).write(config);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_scouting_config_check(config: &z_owned_scouting_config_t) -> bool {
    z_config_check(&config._config)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_scouting_config_drop(config: &mut z_owned_scouting_config_t) {
    z_config_drop(&mut config._config)
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
pub extern "C" fn z_scout(
    config: &mut z_owned_scouting_config_t,
    callback: &mut z_owned_closure_hello_t,
) -> errors::ZCError {
    if cfg!(feature = "logger-autoinit") {
        zc_init_logger();
    }
    let what = WhatAmIMatcher::try_from(config.zc_what).unwrap_or(WhatAmI::Router | WhatAmI::Peer);
    #[allow(clippy::unnecessary_cast)] // Required for multi-target
    let timeout = config.zc_timeout_ms as u64;
    let config = match config._config.transmute_mut().extract().take() {
        Some(c) => c,
        None => { return errors::Z_EINVAL ;}
    };
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
    Z_OK
}

/// Converts the kind of zenoh entity into a string.
///
/// Parameters:
///     whatami: A whatami bitmask of zenoh entity kind.
///     buf: Buffer to write a null-terminated string to.
///     len: Maximum number of bytes that can be written to the `buf`.
///
/// Returns 0 if successful, negative values if whatami contains an invalid bitmask or `buf` is null,
/// or number of remaining bytes, if the null-terminated string size exceeds `len`.
#[no_mangle]
pub extern "C" fn z_whatami_to_str(whatami: u8, buf: *mut c_char, len: usize) -> i8 {
    if buf.is_null() || len == 0 {
        return -1;
    }
    match WhatAmIMatcher::try_from(whatami) {
        Err(_) => -1,
        Ok(w) => {
            let s = w.to_str();
            let res = s.copy_to_c_array(buf as *mut c_void, len - 1);
            unsafe {
                *((buf as usize + res) as *mut c_char) = 0;
            }
            (s.len() - res) as i8
        }
    }
}
