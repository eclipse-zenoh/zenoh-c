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
use libc::{c_char, c_uint, c_ulong, size_t};
use std::ffi::{CStr, CString};
use zenoh::{
    buf::ZBuf,
    config::WhatAmI,
    info::{self, InfoProperties},
    net::protocol::{core::SubInfo, io::SplitBuffer},
    prelude::{KeyExpr, PeerId, Sample, ZInt},
    publication::CongestionControl,
    query::{
        ConsolidationMode, ConsolidationStrategy, QueryConsolidation, QueryTarget, Reply,
    },
    scouting::Hello,
    subscriber::{Reliability, SubMode},
    time::Period,
};

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
    unsafe { &*(b"add_timestamp\0".as_ptr() as *const c_char) };
#[no_mangle]
pub static Z_CONFIG_LOCAL_ROUTING_KEY: &c_char =
    unsafe { &*(b"local_routing\0".as_ptr() as *const c_char) };

// Properties returned by z_info()
#[no_mangle]
pub static Z_INFO_PID_KEY: c_uint = info::ZN_INFO_PID_KEY as c_uint;
#[no_mangle]
pub static Z_INFO_PEER_PID_KEY: c_uint = info::ZN_INFO_PEER_PID_KEY as c_uint;
#[no_mangle]
pub static Z_INFO_ROUTER_PID_KEY: c_uint = info::ZN_INFO_ROUTER_PID_KEY as c_uint;

pub trait FromRaw<T> {
    fn from_raw(r: T) -> Self;
    fn into_raw(self) -> T;
}

/// An owned, zenoh-allocated, null-terminated, string.  
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct z_owned_string_t {
    pub _loan: *const c_char,
}

/// A loaned null-terminated string.
#[allow(non_camel_case_types)]
pub type z_string_t = *const c_char;

/// Constructs a :c:type:`z_owned_string_t` from a NULL terminated string.
/// The contents of `s` are copied.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_new(s: *const c_char) -> z_owned_string_t {
    if s.is_null() {
        return z_owned_string_t { _loan: s };
    }
    let inner = CStr::from_ptr(s).to_owned();
    let start = inner.as_ptr();
    std::mem::forget(inner);
    z_owned_string_t { _loan: start }
}
/// Frees `s`'s memory, while invalidating `s` for double-free-safety.
#[no_mangle]
pub extern "C" fn z_string_free(s: &mut z_owned_string_t) {
    if !s._loan.is_null() {
        unsafe { CString::from_raw(s._loan as *mut c_char) };
        s._loan = std::ptr::null_mut();
    }
}
/// Returns `true` if `s` is valid
#[no_mangle]
pub extern "C" fn z_string_check(s: &z_owned_string_t) -> bool {
    !s._loan.is_null()
}

/// Returns a :c:type:`z_string_t` loaned from `s`.
#[no_mangle]
pub extern "C" fn z_string_loan(s: &z_owned_string_t) -> z_string_t {
    s._loan
}
impl From<String> for z_owned_string_t {
    fn from(s: String) -> Self {
        let inner = CString::new(s).unwrap();
        let start = inner.as_ptr();
        std::mem::forget(inner);
        z_owned_string_t { _loan: start }
    }
}
impl Default for z_owned_string_t {
    fn default() -> Self {
        Self {
            _loan: std::ptr::null(),
        }
    }
}
impl From<&str> for z_owned_string_t {
    fn from(s: &str) -> Self {
        let inner = CString::new(s).unwrap();
        let start = inner.as_ptr();
        std::mem::forget(inner);
        z_owned_string_t { _loan: start }
    }
}
impl From<z_owned_string_t> for String {
    fn from(s: z_owned_string_t) -> Self {
        if s._loan.is_null() {
            String::new()
        } else {
            unsafe { CString::from_raw(s._loan as *mut c_char) }
                .into_string()
                .unwrap()
        }
    }
}

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
pub extern "C" fn z_info_get(info: z_info_t, key: u64) -> z_owned_string_t {
    let info = info.0.as_ref();
    match info.as_ref().and_then(|i| i.get(&key)) {
        None => z_owned_string_t::default(),
        Some(s) => s.as_str().into(),
    }
}

/// An owned array of owned NULL terminated strings, allocated by zenoh.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C)]
pub struct z_owned_str_array_t {
    pub val: *const *const c_char,
    pub len: size_t,
}

impl<T> From<Vec<T>> for z_owned_str_array_t
where
    T: ToString,
{
    #[inline]
    fn from(v: Vec<T>) -> Self {
        let v = v
            .into_iter()
            .map(|t| {
                let s = CString::new(t.to_string()).unwrap();
                let res = s.as_ptr();
                std::mem::forget(s);
                res
            })
            .collect::<Vec<*const c_char>>();
        let res = z_owned_str_array_t {
            val: v.as_ptr(),
            len: v.len() as size_t,
        };
        std::mem::forget(v);
        res
    }
}

impl<T> From<Option<Vec<T>>> for z_owned_str_array_t
where
    T: ToString,
{
    #[inline]
    fn from(v: Option<Vec<T>>) -> Self {
        match v {
            Some(v) => v.into(),
            None => z_owned_str_array_t {
                val: std::ptr::null(),
                len: 0,
            },
        }
    }
}

/// Frees `strs` and invalidates it for double-free safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_str_array_free(strs: &mut z_owned_str_array_t) {
    let locators = Vec::from_raw_parts(
        strs.val as *mut *const c_char,
        strs.len as usize,
        strs.len as usize,
    );
    for locator in locators {
        std::mem::drop(CString::from_raw(locator as *mut c_char));
    }
    strs.val = std::ptr::null();
    strs.len = 0;
}

/// Returns `true` if `strs` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_str_array_check(strs: &z_owned_str_array_t) -> bool {
    !strs.val.is_null() || strs.len == 0
}

/// A zenoh-allocated array of bytes.   
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
pub struct z_owned_bytes_t {
    pub start: *const u8,
    pub len: size_t,
}

/// A loaned array of bytes.  
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_bytes_t {
    pub start: *const u8,
    pub len: size_t,
}
impl Default for z_owned_bytes_t {
    fn default() -> Self {
        z_owned_bytes_t {
            start: std::ptr::null(),
            len: 0,
        }
    }
}

/// Constructs a :c:type:`z_owned_bytes_t` of lengh `len` from the bytes
/// starting at address `start`.
/// The bytes from `start` are copied.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_new(start: *const u8, len: usize) -> z_owned_bytes_t {
    if start.is_null() {
        z_owned_bytes_t { start, len: 0 }
    } else {
        let slice = std::slice::from_raw_parts(start, len);
        let boxed = Box::<[u8]>::from(slice);
        let start = Box::into_raw(boxed);
        z_owned_bytes_t {
            start: (*start).as_ptr(),
            len,
        }
    }
}
/// Frees `b` and invalidates it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_free(b: &mut z_owned_bytes_t) {
    if !b.start.is_null() {
        std::mem::drop(Box::from_raw(
            std::slice::from_raw_parts(b.start, b.len) as *const [u8] as *mut [u8],
        ));
        b.start = std::ptr::null_mut();
    }
}
/// Returns `true` if `b` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_check(b: &z_owned_bytes_t) -> bool {
    !b.start.is_null()
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_bytes_loan(b: &z_owned_bytes_t) -> z_bytes_t {
    z_bytes_t {
        start: b.start,
        len: b.len,
    }
}
impl From<PeerId> for z_owned_bytes_t {
    #[inline]
    fn from(pid: PeerId) -> Self {
        let pid = pid.as_slice().to_vec().into_boxed_slice();
        let res = z_owned_bytes_t {
            start: pid.as_ptr(),
            len: pid.len() as size_t,
        };
        std::mem::forget(pid);
        res
    }
}
impl From<Option<PeerId>> for z_owned_bytes_t {
    #[inline]
    fn from(pid: Option<PeerId>) -> Self {
        match pid {
            Some(pid) => pid.into(),
            None => z_owned_bytes_t {
                start: std::ptr::null(),
                len: 0,
            },
        }
    }
}
impl From<ZBuf> for z_owned_bytes_t {
    fn from(buf: ZBuf) -> Self {
        let data = buf.contiguous().into_owned().into_boxed_slice();
        let res = z_owned_bytes_t {
            start: data.as_ptr(),
            len: data.len(),
        };
        std::mem::forget(data);
        res
    }
}
impl From<z_owned_bytes_t> for String {
    fn from(s: z_owned_bytes_t) -> Self {
        unsafe {
            String::from_utf8(
                Box::from_raw(std::slice::from_raw_parts_mut(s.start as *mut u8, s.len)).into(),
            )
            .unwrap()
        }
    }
}

/// A zenoh-allocated key expression.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"/key/expression"``.
///    - ``"/key/ex*"``.
///
/// Key expressions can be mapped to numerical ids through :c:func:`z_declare_expr`
/// for wire and computation efficiency.
///
/// A key expression can be either:
///   - A plain string expression.
///   - A pure numerical id.
///   - The combination of a numerical prefix and a string suffix.
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
#[derive(Default)]
pub struct z_owned_keyexpr_t {
    pub id: c_ulong,
    pub suffix: z_owned_bytes_t,
}
/// A loaned key expression.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"/key/expression"``.
///    - ``"/key/ex*"``.
///
/// Key expressions can be mapped to numerical ids through :c:func:`z_declare_expr`
/// for wire and computation efficiency.
///
/// A key expression can be either:
///   - A plain string expression.
///   - A pure numerical id.
///   - The combination of a numerical prefix and a string suffix.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_keyexpr_t {
    pub id: c_ulong,
    pub suffix: z_bytes_t,
}

/// Constructs a zenoh-owned key expression. `suffix`'s contents will be copied.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_keyexpr_new(id: c_ulong, suffix: *const c_char) -> z_owned_keyexpr_t {
    z_owned_keyexpr_t {
        id,
        suffix: z_bytes_new(
            suffix as *const _,
            if suffix.is_null() {
                0
            } else {
                libc::strlen(suffix)
            },
        ),
    }
}
/// Constructs a loaned key expression. The constructed value is valid as long as `suffix` is.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_keyexpr_new_loaned(id: c_ulong, suffix: *const c_char) -> z_keyexpr_t {
    z_keyexpr_t {
        id,
        suffix: z_bytes_t {
            start: suffix as *const _,
            len: if suffix.is_null() {
                0
            } else {
                libc::strlen(suffix)
            },
        },
    }
}
/// Frees `keyexpr` and invalidates it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_keyexpr_free(keyexpr: &mut z_owned_keyexpr_t) {
    z_bytes_free(&mut keyexpr.suffix);
    keyexpr.id = 0;
}
/// Returns `true` if `keyexpr` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_keyexpr_check(keyexpr: &z_owned_keyexpr_t) -> bool {
    keyexpr.id != 0 || z_bytes_check(&keyexpr.suffix)
}

/// Returns a :c:type:`z_keyexpr_t` loaned from `keyexpr`.
#[no_mangle]
pub extern "C" fn z_keyexpr_loan(keyexpr: &z_owned_keyexpr_t) -> z_keyexpr_t {
    z_keyexpr_t {
        id: keyexpr.id,
        suffix: unsafe { z_bytes_loan(&keyexpr.suffix) },
    }
}

impl<'a> From<&'a z_owned_keyexpr_t> for KeyExpr<'a> {
    fn from(r: &'a z_owned_keyexpr_t) -> Self {
        unsafe {
            let len = r.suffix.len;
            match (r.id, len) {
                (id, 0) => KeyExpr::from(id as ZInt),
                (0, _) => KeyExpr::from(
                    std::str::from_utf8(std::slice::from_raw_parts(
                        r.suffix.start as *const _,
                        len,
                    ))
                    .unwrap(),
                ),
                (id, _) => KeyExpr::from(id as ZInt).with_suffix(
                    std::str::from_utf8(std::slice::from_raw_parts(
                        r.suffix.start as *const _,
                        len,
                    ))
                    .unwrap(),
                ),
            }
        }
    }
}

impl<'a> From<z_keyexpr_t> for KeyExpr<'a> {
    fn from(r: z_keyexpr_t) -> Self {
        unsafe {
            let len = r.suffix.len;
            match (r.id, len) {
                (id, 0) => KeyExpr::from(id as ZInt),
                (0, _) => {
                    std::str::from_utf8(std::slice::from_raw_parts(r.suffix.start as *const _, len))
                        .unwrap()
                        .into()
                }
                (id, _) => KeyExpr::from(id as ZInt).with_suffix(
                    std::str::from_utf8(std::slice::from_raw_parts(
                        r.suffix.start as *const _,
                        len,
                    ))
                    .unwrap(),
                ),
            }
        }
    }
}

impl<'a> From<&KeyExpr<'a>> for z_keyexpr_t {
    fn from(key: &KeyExpr<'a>) -> Self {
        let (id, suffix) = key.as_id_and_suffix();
        z_keyexpr_t {
            id: id as c_ulong,
            suffix: z_bytes_t {
                start: suffix.as_ptr() as *const _,
                len: suffix.len(),
            },
        }
    }
}

impl<'a> From<KeyExpr<'a>> for z_owned_keyexpr_t {
    fn from(key: KeyExpr<'a>) -> Self {
        let (id, suffix) = key.as_id_and_suffix();
        z_owned_keyexpr_t {
            id: id as c_ulong,
            suffix: unsafe { z_bytes_new(suffix.as_ptr() as *const _, suffix.len()) },
        }
    }
}

/// A zenoh-allocated data sample.
///
/// A sample is the value associated to a given resource at a given point in time.
///
/// Members:
///   `z_owned_string_t key`: The resource key of this data sample.
///   `z_owned_bytes_t value`: The value of this data sample.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[repr(C)]
pub struct z_owned_sample_t {
    pub key: z_owned_keyexpr_t,
    pub value: z_owned_bytes_t,
    pub encoding: z_owned_encoding_t,
}

/// A loaned data sample.
///
/// A sample is the value associated to a given resource at a given point in time.
///
/// Members:
///   `z_string_t key`: The resource key of this data sample.
///   `z_bytes_t value`: The value of this data sample.
#[repr(C)]
pub struct z_sample_t {
    pub key: z_keyexpr_t,
    pub value: z_bytes_t,
    pub encoding: z_encoding_t,
}

impl From<Sample> for z_owned_sample_t {
    #[inline]
    fn from(s: Sample) -> Self {
        z_owned_sample_t {
            key: s.key_expr.into(),
            value: s.value.payload.into(),
            encoding: z_encoding_t::from(&s.value.encoding).into(),
        }
    }
}

/// Frees `sample`, invalidating it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_sample_free(sample: &mut z_owned_sample_t) {
    z_keyexpr_free(&mut sample.key);
    z_bytes_free(&mut sample.value);
}
/// Returns `true` if `sample` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_sample_check(sample: &z_owned_sample_t) -> bool {
    z_keyexpr_check(&sample.key) && z_bytes_check(&sample.value)
}

/// Returns a :c:type:`z_sample_t` loaned from `sample`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_sample_loan(sample: &z_owned_sample_t) -> z_sample_t {
    z_sample_t {
        key: z_keyexpr_loan(&sample.key),
        value: z_bytes_loan(&sample.value),
        encoding: z_encoding_loan(&sample.encoding),
    }
}

/// A zenoh-allocated hello message returned by a zenoh entity to a scout message sent with `z_scout`.
///
/// Members:
///   `unsigned int whatami`: The kind of zenoh entity.
///   `z_owned_bytes_t pid`: The peer id of the scouted entity (empty if absent).
///   `z_owned_str_array_t locators`: The locators of the scouted entity.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[repr(C)]
pub struct z_owned_hello_t {
    pub whatami: c_uint,
    pub pid: z_owned_bytes_t,
    pub locators: z_owned_str_array_t,
}
impl From<Hello> for z_owned_hello_t {
    #[inline]
    fn from(h: Hello) -> Self {
        z_owned_hello_t {
            whatami: match h.whatami {
                Some(whatami) => whatami as c_uint,
                None => Z_ROUTER,
            },
            pid: h.pid.into(),
            locators: h.locators.into(),
        }
    }
}

/// Frees `hello`, invalidating it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_free(hello: &mut z_owned_hello_t) {
    z_bytes_free(&mut hello.pid);
    z_str_array_free(&mut hello.locators);
    hello.whatami = 0;
}
/// Returns `true` if `hello` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_check(hello: &z_owned_hello_t) -> bool {
    hello.whatami != 0 && z_bytes_check(&hello.pid) && z_str_array_check(&hello.locators)
}

/// A zenoh-allocated array of `z_hello_t` messages.
///
/// Members:
///   const z_hello_t *val: A pointer to the array.
///   unsigned int len: The length of the array.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[repr(C)]
pub struct z_owned_hello_array_t {
    pub val: *const z_owned_hello_t,
    pub len: size_t,
}
impl From<Vec<Hello>> for z_owned_hello_array_t {
    #[inline]
    fn from(hvec: Vec<Hello>) -> Self {
        let mut hvec = hvec
            .into_iter()
            .map(|h| h.into())
            .collect::<Vec<z_owned_hello_t>>();
        hvec.shrink_to_fit();
        let res = z_owned_hello_array_t {
            val: hvec.as_ptr(),
            len: hvec.len() as size_t,
        };
        std::mem::forget(hvec);
        res
    }
}
/// Frees `hellos`, invalidating it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_array_free(hellos: &mut z_owned_hello_array_t) {
    let hellos_vec = Vec::from_raw_parts(
        hellos.val as *mut z_owned_hello_t,
        hellos.len as usize,
        hellos.len as usize,
    );
    for mut hello in hellos_vec {
        z_hello_free(&mut hello);
    }
    hellos.val = std::ptr::null_mut();
    hellos.len = 0;
}
/// Returns `true` if `hellos` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_array_check(hellos: &z_owned_hello_array_t) -> bool {
    !hellos.val.is_null() || hellos.len == 0
}

/// The behavior to adopt in case of congestion while routing some data.
///
///     - **z_congestion_control_t_BLOCK**
///     - **z_congestion_control_t_DROP**
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum z_congestion_control_t {
    BLOCK,
    DROP,
}

impl From<z_congestion_control_t> for CongestionControl {
    fn from(val: z_congestion_control_t) -> Self {
        match val {
            z_congestion_control_t::BLOCK => CongestionControl::Block,
            z_congestion_control_t::DROP => CongestionControl::Drop,
        }
    }
}

/// The subscription reliability.
///
///     - **z_reliability_t_BEST_EFFORT**
///     - **z_reliability_t_RELIABLE**
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum z_reliability_t {
    BEST_EFFORT,
    RELIABLE,
}

impl From<Reliability> for z_reliability_t {
    #[inline]
    fn from(r: Reliability) -> Self {
        match r {
            Reliability::BestEffort => z_reliability_t::BEST_EFFORT,
            Reliability::Reliable => z_reliability_t::RELIABLE,
        }
    }
}

impl From<z_reliability_t> for Reliability {
    #[inline]
    fn from(val: z_reliability_t) -> Self {
        match val {
            z_reliability_t::BEST_EFFORT => Reliability::BestEffort,
            z_reliability_t::RELIABLE => Reliability::Reliable,
        }
    }
}

/// The subscription mode.
///
///     - **z_submode_t_PUSH**
///     - **z_submode_t_PULL**
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum z_submode_t {
    PUSH,
    PULL,
}

impl From<SubMode> for z_submode_t {
    #[inline]
    fn from(sm: SubMode) -> Self {
        match sm {
            SubMode::Push => z_submode_t::PUSH,
            SubMode::Pull => z_submode_t::PULL,
        }
    }
}

impl From<z_submode_t> for SubMode {
    #[inline]
    fn from(val: z_submode_t) -> Self {
        match val {
            z_submode_t::PUSH => SubMode::Push,
            z_submode_t::PULL => SubMode::Pull,
        }
    }
}

/// The subscription period.
/// Equivalent of the rust `Option<zenoh::time::Period>` type, where `None` is represented by the `period` field being 0-valued.
///
/// Members:
///     `unsigned int origin`
///     `unsigned int period`
///     `unsigned int duration`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct z_period_t {
    pub origin: c_uint,
    pub period: c_uint,
    pub duration: c_uint,
}
#[allow(non_upper_case_globals)]
pub const z_period_NONE: z_period_t = z_period_t {
    origin: 0,
    period: 0,
    duration: 0,
};
impl From<Period> for z_period_t {
    #[inline]
    fn from(p: Period) -> Self {
        z_period_t {
            origin: p.origin as c_uint,
            period: p.period as c_uint,
            duration: p.duration as c_uint,
        }
    }
}
impl From<Option<Period>> for z_period_t {
    fn from(p: Option<Period>) -> Self {
        match p {
            Some(p) => p.into(),
            None => z_period_t {
                duration: 0,
                origin: 0,
                period: 0,
            },
        }
    }
}
impl From<z_period_t> for Period {
    #[inline]
    fn from(val: z_period_t) -> Self {
        Period {
            origin: val.origin as ZInt,
            period: val.period as ZInt,
            duration: val.duration as ZInt,
        }
    }
}
impl From<z_period_t> for Option<Period> {
    #[inline]
    fn from(val: z_period_t) -> Self {
        if val.period == 0 {
            None
        } else {
            Some(val.into())
        }
    }
}

/// Informations to be passed to :c:func:`z_subscribe` to configure the created :c:type:`z_owned_subscriber_t`.
///
/// Members:
///     `z_reliability_t reliability`: The subscription reliability.
///     `z_submode_t mode`: The subscription mode.
///     `z_period_t *period`: The subscription period.
#[repr(C)]
pub struct z_subinfo_t {
    pub reliability: z_reliability_t,
    pub mode: z_submode_t,
    pub period: z_period_t,
}

/// Returns the subscription period from `info`.
#[no_mangle]
pub extern "C" fn z_subinfo_period(info: &z_subinfo_t) -> *const z_period_t {
    if info.period.period != 0 {
        &info.period
    } else {
        std::ptr::null()
    }
}
impl From<SubInfo> for z_subinfo_t {
    #[inline]
    fn from(si: SubInfo) -> Self {
        z_subinfo_t {
            reliability: si.reliability.into(),
            mode: si.mode.into(),
            period: si.period.into(),
        }
    }
}

impl From<z_subinfo_t> for SubInfo {
    #[inline]
    fn from(val: z_subinfo_t) -> Self {
        SubInfo {
            reliability: val.reliability.into(),
            mode: val.mode.into(),
            period: val.period.into(),
        }
    }
}

/// Create a default subscription info.
#[no_mangle]
pub extern "C" fn z_subinfo_default() -> z_subinfo_t {
    SubInfo::default().into()
}

/// An owned reply to a `z_get` (or `z_get_collect`).
///
/// Members:
///   `z_owned_sample_t sample`: a :c:type:`z_sample_t` containing the key and value of the reply.
///   `z_owned_bytes_t replier_id`: The id of the replier that sent this reply.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_owned_reply_data_t {
    sample: z_owned_sample_t,
    replier_id: z_owned_bytes_t,
}
impl z_owned_reply_data_t {
    #[inline]
    pub(crate) fn empty() -> Self {
        z_owned_reply_data_t {
            sample: z_owned_sample_t {
                key: z_owned_keyexpr_t::default(),
                value: z_owned_bytes_t::default(),
                encoding: z_owned_encoding_t {
                    prefix: z_known_encoding_t::Empty,
                    suffix: z_owned_bytes_t {
                        start: std::ptr::null(),
                        len: 0,
                    },
                    _freed: false,
                },
            },
            replier_id: z_owned_bytes_t::default(),
        }
    }
}
impl From<Reply> for z_owned_reply_data_t {
    #[inline]
    fn from(r: Reply) -> Self {
        z_owned_reply_data_t {
            sample: r.sample.into(),
            replier_id: r.replier_id.into(),
        }
    }
}

/// Frees `reply_data`, invalidating it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_data_free(reply_data: &mut z_owned_reply_data_t) {
    z_sample_free(&mut reply_data.sample);
    z_bytes_free(&mut reply_data.replier_id);
}
/// Returns `true` if `reply_data` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_data_check(reply_data: &z_owned_reply_data_t) -> bool {
    z_sample_check(&reply_data.sample) && z_bytes_check(&reply_data.replier_id)
}

/// A zenoh-allocated array of :c:type:`z_owned_reply_data_t`.
///
/// Members:
///   `char *const *val`: A pointer to the array.
///   `unsigned int len`: The length of the array.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[repr(C)]
pub struct z_owned_reply_data_array_t {
    pub val: *const z_owned_reply_data_t,
    pub len: size_t,
}

/// Free a :c:type:`z_owned_reply_data_array_t` and it's contained replies.
///
/// Parameters:
///     replies: The :c:type:`z_owned_reply_data_array_t` to free.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_reply_data_array_free(replies: &mut z_owned_reply_data_array_t) {
    let vec = Vec::from_raw_parts(
        replies.val as *mut z_owned_reply_data_t,
        replies.len,
        replies.len,
    );
    for mut rd in vec {
        z_reply_data_free(&mut rd);
    }
    replies.val = std::ptr::null();
    replies.len = 0;
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_reply_data_array_check(replies: &z_owned_reply_data_array_t) -> bool {
    !replies.val.is_null() || replies.len == 0
}

/// The possible values of :c:member:`z_owned_reply_t.tag`
///
///     - **z_reply_t_Tag_DATA**: The reply contains some data.
///     - **z_reply_t_Tag_FINAL**: The reply does not contain any data and indicates that there will be no more replies for this query.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum z_reply_t_Tag {
    DATA,
    FINAL,
}

/// An owned reply to a :c:func:`z_get`.
///
/// Members:
///   `z_reply_t_Tag tag`: Indicates if the reply contains data or if it's a FINAL reply.
///   `z_owned_reply_data_t data`: The reply data if :c:member:`z_owned_reply_t.tag` equals :c:member:`z_reply_t_Tag.z_reply_t_Tag_DATA`.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_owned_reply_t {
    pub tag: z_reply_t_Tag,
    pub data: z_owned_reply_data_t,
}
/// Frees `reply`, invalidating it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_free(reply: &mut z_owned_reply_t) {
    if reply.tag == z_reply_t_Tag::DATA {
        z_reply_data_free(&mut reply.data)
    }
}
/// Returns `true` if `reply` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_check(reply: &z_owned_reply_t) -> bool {
    z_reply_t_Tag::FINAL == reply.tag
        || (z_reply_t_Tag::DATA == reply.tag && z_reply_data_check(&reply.data))
}

/// The possible values of :c:member:`z_query_target_t.tag`.
///
///     - **z_query_target_t_BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
///     - **z_query_target_t_COMPLETE**: A set of complete queryables.
///     - **z_query_target_t_ALL**: All matching queryables.
///     - **z_query_target_t_NONE**: No queryables.
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum z_query_target_t {
    BEST_MATCHING,
    ALL,
    NONE,
    ALL_COMPLETE,
    // #[cfg(feature = "complete_n")]
    // COMPLETE {
    //     n: c_uint,
    // },
}

impl From<QueryTarget> for z_query_target_t {
    #[inline]
    fn from(t: QueryTarget) -> Self {
        match t {
            QueryTarget::BestMatching => z_query_target_t::BEST_MATCHING,
            QueryTarget::All => z_query_target_t::ALL,
            QueryTarget::None => z_query_target_t::NONE,
            QueryTarget::AllComplete => z_query_target_t::ALL_COMPLETE,
            // #[cfg(feature = "complete_n")]
            // QueryTarget::Complete(n) => z_query_target_t::COMPLETE { n: n as c_uint },
        }
    }
}

impl From<z_query_target_t> for QueryTarget {
    #[inline]
    fn from(val: z_query_target_t) -> Self {
        match val {
            z_query_target_t::BEST_MATCHING => QueryTarget::BestMatching,
            z_query_target_t::ALL => QueryTarget::All,
            z_query_target_t::NONE => QueryTarget::None,
            z_query_target_t::ALL_COMPLETE => QueryTarget::AllComplete,
            // #[cfg(feature = "complete_n")]
            // z_query_target_t::COMPLETE { n } => QueryTarget::Complete(n as ZInt),
        }
    }
}

/// Create a default :c:type:`z_query_target_t`.
#[no_mangle]
pub extern "C" fn z_query_target_default() -> z_query_target_t {
    QueryTarget::default().into()
}

/// The kind of consolidation that should be applied on replies to a :c:func:`z_get`.
///
///     - **z_consolidation_mode_t_FULL**: Guaranties unicity of replies. Optimizes bandwidth.
///     - **z_consolidation_mode_t_LAZY**: Does not garanty unicity. Optimizes latency.
///     - **z_consolidation_mode_t_NONE**: No consolidation.
#[repr(C)]
pub enum z_consolidation_mode_t {
    FULL,
    LAZY,
    NONE,
}

impl From<ConsolidationMode> for z_consolidation_mode_t {
    #[inline]
    fn from(cm: ConsolidationMode) -> Self {
        match cm {
            ConsolidationMode::Full => z_consolidation_mode_t::FULL,
            ConsolidationMode::Lazy => z_consolidation_mode_t::LAZY,
            ConsolidationMode::None => z_consolidation_mode_t::NONE,
        }
    }
}

impl From<z_consolidation_mode_t> for ConsolidationMode {
    #[inline]
    fn from(val: z_consolidation_mode_t) -> Self {
        match val {
            z_consolidation_mode_t::NONE => ConsolidationMode::None,
            z_consolidation_mode_t::LAZY => ConsolidationMode::Lazy,
            z_consolidation_mode_t::FULL => ConsolidationMode::Full,
        }
    }
}

/// The kind of consolidation that should be applied on replies to a :c:func:`z_get`
/// at the different stages of the reply process.
///
/// Members:
///   z_consolidation_mode_t first_routers: The consolidation mode to apply on first routers of the replies routing path.
///   z_consolidation_mode_t last_router: The consolidation mode to apply on last router of the replies routing path.
///   z_consolidation_mode_t reception: The consolidation mode to apply at reception of the replies.
#[repr(C)]
pub struct z_consolidation_strategy_t {
    pub first_routers: z_consolidation_mode_t,
    pub last_router: z_consolidation_mode_t,
    pub reception: z_consolidation_mode_t,
}

impl From<ConsolidationStrategy> for z_consolidation_strategy_t {
    #[inline]
    fn from(cs: ConsolidationStrategy) -> Self {
        z_consolidation_strategy_t {
            first_routers: cs.first_routers.into(),
            last_router: cs.last_router.into(),
            reception: cs.reception.into(),
        }
    }
}

impl From<z_consolidation_strategy_t> for ConsolidationStrategy {
    #[inline]
    fn from(val: z_consolidation_strategy_t) -> Self {
        ConsolidationStrategy {
            first_routers: val.first_routers.into(),
            last_router: val.last_router.into(),
            reception: val.reception.into(),
        }
    }
}

/// The replies consolidation strategy to apply on replies to a :c:func:`z_get`.
#[repr(C)]
pub enum z_query_consolidation_t {
    AUTO,
    MANUAL(z_consolidation_strategy_t),
}

impl From<QueryConsolidation> for z_query_consolidation_t {
    #[inline]
    fn from(qc: QueryConsolidation) -> Self {
        match qc {
            QueryConsolidation::Auto => z_query_consolidation_t::AUTO,
            QueryConsolidation::Manual(strategy) => {
                z_query_consolidation_t::MANUAL(strategy.into())
            }
        }
    }
}

impl From<z_query_consolidation_t> for QueryConsolidation {
    #[inline]
    fn from(val: z_query_consolidation_t) -> Self {
        match val {
            z_query_consolidation_t::AUTO => QueryConsolidation::Auto,
            z_query_consolidation_t::MANUAL(strategy) => {
                QueryConsolidation::Manual(strategy.into())
            }
        }
    }
}

/// Automatic query consolidation strategy selection.
///
/// A query consolidation strategy will automatically be selected depending
/// the query selector. If the selector contains time range properties,
/// no consolidation is performed. Otherwise the
/// :c:func:`z_query_consolidation_reception` strategy is used.
#[no_mangle]
pub extern "C" fn z_query_consolidation_auto() -> z_query_consolidation_t {
    QueryConsolidation::auto().into()
}

/// No consolidation performed.
///
/// This is usefull when querying timeseries data bases or
/// when using quorums.
#[no_mangle]
pub extern "C" fn z_query_consolidation_none() -> z_query_consolidation_t {
    QueryConsolidation::none().into()
}

/// Lazy consolidation performed at all stages.
///
/// This strategy offers the best latency. Replies are directly
/// transmitted to the application when received without needing
/// to wait for all replies.
///
/// This mode does not garantie that there will be no duplicates.
#[no_mangle]
pub extern "C" fn z_query_consolidation_lazy() -> z_query_consolidation_t {
    QueryConsolidation::lazy().into()
}

/// Full consolidation performed at reception.
///
/// This is the default strategy. It offers the best latency while
/// garantying that there will be no duplicates.
#[no_mangle]
pub extern "C" fn z_query_consolidation_reception() -> z_query_consolidation_t {
    QueryConsolidation::reception().into()
}

/// Full consolidation performed on last router and at reception.
///
/// This mode offers a good latency while optimizing bandwidth on
/// the last transport link between the router and the application.
#[no_mangle]
pub extern "C" fn z_query_consolidation_last_router() -> z_query_consolidation_t {
    QueryConsolidation::last_router().into()
}

/// Full consolidation performed everywhere.
///
/// This mode optimizes bandwidth on all links in the system
/// but will provide a very poor latency.
#[no_mangle]
pub extern "C" fn z_query_consolidation_full() -> z_query_consolidation_t {
    QueryConsolidation::full().into()
}

/// Creates a default :c:type:`z_query_consolidation_t`.
#[no_mangle]
pub extern "C" fn z_query_consolidation_default() -> z_query_consolidation_t {
    QueryConsolidation::default().into()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum z_known_encoding_t {
    Empty = 0,
    AppOctetStream = 1,
    AppCustom = 2,
    TextPlain = 3,
    AppProperties = 4,
    AppJson = 5,
    AppSql = 6,
    AppInteger = 7,
    AppFloat = 8,
    AppXml = 9,
    AppXhtmlXml = 10,
    AppXWwwFormUrlencoded = 11,
    TextJson = 12,
    TextHtml = 13,
    TextXml = 14,
    TextCss = 15,
    TextCsv = 16,
    TextJavascript = 17,
    ImageJpeg = 18,
    ImagePng = 19,
    ImageGif = 20,
}
impl From<z_known_encoding_t> for zenoh_protocol_core::KnownEncoding {
    fn from(val: z_known_encoding_t) -> Self {
        if cfg!(debug_assertions) {
            match val {
                z_known_encoding_t::Empty => zenoh_protocol_core::KnownEncoding::Empty,
                z_known_encoding_t::AppOctetStream => {
                    zenoh_protocol_core::KnownEncoding::AppOctetStream
                }
                z_known_encoding_t::AppCustom => zenoh_protocol_core::KnownEncoding::AppCustom,
                z_known_encoding_t::TextPlain => zenoh_protocol_core::KnownEncoding::TextPlain,
                z_known_encoding_t::AppProperties => {
                    zenoh_protocol_core::KnownEncoding::AppProperties
                }
                z_known_encoding_t::AppJson => zenoh_protocol_core::KnownEncoding::AppJson,
                z_known_encoding_t::AppSql => zenoh_protocol_core::KnownEncoding::AppSql,
                z_known_encoding_t::AppInteger => zenoh_protocol_core::KnownEncoding::AppInteger,
                z_known_encoding_t::AppFloat => zenoh_protocol_core::KnownEncoding::AppFloat,
                z_known_encoding_t::AppXml => zenoh_protocol_core::KnownEncoding::AppXml,
                z_known_encoding_t::AppXhtmlXml => zenoh_protocol_core::KnownEncoding::AppXhtmlXml,
                z_known_encoding_t::AppXWwwFormUrlencoded => {
                    zenoh_protocol_core::KnownEncoding::AppXWwwFormUrlencoded
                }
                z_known_encoding_t::TextJson => zenoh_protocol_core::KnownEncoding::TextJson,
                z_known_encoding_t::TextHtml => zenoh_protocol_core::KnownEncoding::TextHtml,
                z_known_encoding_t::TextXml => zenoh_protocol_core::KnownEncoding::TextXml,
                z_known_encoding_t::TextCss => zenoh_protocol_core::KnownEncoding::TextCss,
                z_known_encoding_t::TextCsv => zenoh_protocol_core::KnownEncoding::TextCsv,
                z_known_encoding_t::TextJavascript => {
                    zenoh_protocol_core::KnownEncoding::TextJavascript
                }
                z_known_encoding_t::ImageJpeg => zenoh_protocol_core::KnownEncoding::ImageJpeg,
                z_known_encoding_t::ImagePng => zenoh_protocol_core::KnownEncoding::ImagePng,
                z_known_encoding_t::ImageGif => zenoh_protocol_core::KnownEncoding::ImageGif,
            }
        } else {
            unsafe { std::mem::transmute(val as u8) }
        }
    }
}
impl From<zenoh_protocol_core::KnownEncoding> for z_known_encoding_t {
    fn from(val: zenoh_protocol_core::KnownEncoding) -> Self {
        if cfg!(debug_assertions) {
            match val {
                zenoh_protocol_core::KnownEncoding::Empty => z_known_encoding_t::Empty,
                zenoh_protocol_core::KnownEncoding::AppOctetStream => {
                    z_known_encoding_t::AppOctetStream
                }
                zenoh_protocol_core::KnownEncoding::AppCustom => z_known_encoding_t::AppCustom,
                zenoh_protocol_core::KnownEncoding::TextPlain => z_known_encoding_t::TextPlain,
                zenoh_protocol_core::KnownEncoding::AppProperties => {
                    z_known_encoding_t::AppProperties
                }
                zenoh_protocol_core::KnownEncoding::AppJson => z_known_encoding_t::AppJson,
                zenoh_protocol_core::KnownEncoding::AppSql => z_known_encoding_t::AppSql,
                zenoh_protocol_core::KnownEncoding::AppInteger => z_known_encoding_t::AppInteger,
                zenoh_protocol_core::KnownEncoding::AppFloat => z_known_encoding_t::AppFloat,
                zenoh_protocol_core::KnownEncoding::AppXml => z_known_encoding_t::AppXml,
                zenoh_protocol_core::KnownEncoding::AppXhtmlXml => z_known_encoding_t::AppXhtmlXml,
                zenoh_protocol_core::KnownEncoding::AppXWwwFormUrlencoded => {
                    z_known_encoding_t::AppXWwwFormUrlencoded
                }
                zenoh_protocol_core::KnownEncoding::TextJson => z_known_encoding_t::TextJson,
                zenoh_protocol_core::KnownEncoding::TextHtml => z_known_encoding_t::TextHtml,
                zenoh_protocol_core::KnownEncoding::TextXml => z_known_encoding_t::TextXml,
                zenoh_protocol_core::KnownEncoding::TextCss => z_known_encoding_t::TextCss,
                zenoh_protocol_core::KnownEncoding::TextCsv => z_known_encoding_t::TextCsv,
                zenoh_protocol_core::KnownEncoding::TextJavascript => {
                    z_known_encoding_t::TextJavascript
                }
                zenoh_protocol_core::KnownEncoding::ImageJpeg => z_known_encoding_t::ImageJpeg,
                zenoh_protocol_core::KnownEncoding::ImagePng => z_known_encoding_t::ImagePng,
                zenoh_protocol_core::KnownEncoding::ImageGif => z_known_encoding_t::ImageGif,
            }
        } else {
            unsafe { std::mem::transmute(val as u32) }
        }
    }
}

/// The encoding of a payload, in a MIME-like format.
///
/// For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
///
/// `suffix` MUST be a valid UTF-8 string.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_encoding_t {
    pub prefix: z_known_encoding_t,
    pub suffix: z_bytes_t,
}
impl From<z_encoding_t> for zenoh_protocol_core::Encoding {
    fn from(enc: z_encoding_t) -> Self {
        if enc.suffix.len == 0 {
            zenoh_protocol_core::Encoding::Exact(enc.prefix.into())
        } else {
            let suffix = unsafe {
                let slice: &'static [u8] =
                    std::slice::from_raw_parts(enc.suffix.start, enc.suffix.len);
                std::str::from_utf8_unchecked(slice)
            };
            zenoh_protocol_core::Encoding::WithSuffix(enc.prefix.into(), suffix.into())
        }
    }
}
impl From<&zenoh_protocol_core::Encoding> for z_encoding_t {
    fn from(val: &zenoh_protocol_core::Encoding) -> Self {
        let suffix = val.suffix();
        z_encoding_t {
            prefix: (*val.prefix()).into(),
            suffix: z_bytes_t {
                start: suffix.as_ptr(),
                len: suffix.len(),
            },
        }
    }
}

#[repr(C)]
pub struct z_owned_encoding_t {
    pub prefix: z_known_encoding_t,
    pub suffix: z_owned_bytes_t,
    pub _freed: bool,
}

/// Frees `encoding`, invalidating it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_default() -> z_encoding_t {
    (&zenoh_protocol_core::Encoding::default()).into()
}
/// Frees `encoding`, invalidating it for double-free safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_free(encoding: &mut z_owned_encoding_t) {
    z_bytes_free(&mut encoding.suffix);
    encoding._freed = true
}
/// Returns `true` if `encoding` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_check(encoding: &z_owned_encoding_t) -> bool {
    !encoding._freed
}

/// Returns a :c:type:`z_encoding_t` loaned from `encoding`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_loan(encoding: &z_owned_encoding_t) -> z_encoding_t {
    z_encoding_t {
        prefix: encoding.prefix,
        suffix: z_bytes_loan(&encoding.suffix),
    }
}
impl From<z_encoding_t> for z_owned_encoding_t {
    fn from(val: z_encoding_t) -> Self {
        let suffix = unsafe { z_bytes_new(val.suffix.start, val.suffix.len) };
        z_owned_encoding_t {
            prefix: val.prefix,
            suffix,
            _freed: false,
        }
    }
}
