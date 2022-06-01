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
use crate::session::*;
use libc::{c_char, c_uint};
use std::ffi::CString;
use zenoh::info::InfoProperties;
use zenoh::prelude::sync::SyncResolve;

// Properties returned by z_info()
#[no_mangle]
pub static Z_INFO_PID_KEY: c_uint = zenoh::info::ZN_INFO_PID_KEY as c_uint;
#[no_mangle]
pub static Z_INFO_PEER_PID_KEY: c_uint = zenoh::info::ZN_INFO_PEER_PID_KEY as c_uint;
#[no_mangle]
pub static Z_INFO_ROUTER_PID_KEY: c_uint = zenoh::info::ZN_INFO_ROUTER_PID_KEY as c_uint;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_info_t<'a>(&'a z_owned_info_t);

/// A map of integers to strings providing informations on the zenoh session.  
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_owned_info_t {
    _align: [u64; 2],
    _pad: [usize; 4],
}

impl AsRef<Option<InfoProperties>> for z_owned_info_t {
    fn as_ref(&self) -> &Option<InfoProperties> {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Option<InfoProperties>> for z_owned_info_t {
    fn as_mut(&mut self) -> &mut Option<InfoProperties> {
        unsafe { std::mem::transmute(self) }
    }
}

/// Frees `info`'s memory, while invalidating `info` for double-free-safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_info_free(info: &mut z_owned_info_t) {
    std::mem::drop(info.as_mut().take())
}
/// Returns `true` if `info` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_info_check(info: &z_owned_info_t) -> bool {
    info.as_ref().is_some()
}

/// Returns a :c:type:`z_info_t` loaned from `info`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_info_loan(info: &z_owned_info_t) -> z_info_t {
    z_info_t(info)
}

/// Returns the information associated with `key` if it exists.  
/// If it doesn't, the returned value is invalid, and doesn't need freeing.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_info_get(info: z_info_t, key: u64) -> *mut c_char {
    let info = info.0.as_ref();
    match info.as_ref().and_then(|i| i.get(&key)) {
        None => std::ptr::null_mut(),
        Some(s) => CString::from_vec_unchecked(s.as_str().as_bytes().to_vec()).into_raw(),
    }
}

/// Gets informations about an zenoh session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info(session: z_session_t) -> z_owned_info_t {
    match session.as_ref() {
        Some(s) => std::mem::transmute(s.info().res()),
        None => std::mem::transmute(None::<InfoProperties>),
    }
}

/// Gets informations about an zenoh session as a properties-formatted string.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_as_str(session: z_session_t) -> *mut c_char {
    match session.as_ref() {
        Some(s) => match CString::new(s.info().res().to_string()) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        None => std::ptr::null_mut(),
    }
}
