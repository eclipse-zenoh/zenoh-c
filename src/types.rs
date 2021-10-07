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
use libc::{c_char, c_uint, c_ulong, size_t};
use std::ffi::{CStr, CString};
use zenoh::{
    buf::ZBuf,
    config,
    config::WhatAmI,
    info::{self, InfoProperties},
    net::protocol::core::SubInfo,
    prelude::{PeerId, ResKey, Sample, ZInt},
    publisher::CongestionControl,
    query::{ConsolidationMode, QueryConsolidation, QueryTarget, Reply, Target},
    queryable,
    scouting::Hello,
    subscriber::{Reliability, SubMode},
    time::Period,
};

#[no_mangle]
pub static ZN_ROUTER: c_uint = WhatAmI::Router as c_uint;
#[no_mangle]
pub static ZN_PEER: c_uint = WhatAmI::Peer as c_uint;
#[no_mangle]
pub static ZN_CLIENT: c_uint = WhatAmI::Client as c_uint;

// Flags used in Queryable declaration and in queries
#[no_mangle]
pub static ZN_QUERYABLE_ALL_KINDS: c_uint = queryable::ALL_KINDS as c_uint;
#[no_mangle]
pub static ZN_QUERYABLE_STORAGE: c_uint = queryable::STORAGE as c_uint;
#[no_mangle]
pub static ZN_QUERYABLE_EVAL: c_uint = queryable::EVAL as c_uint;

// Properties for zenoh net session configuration
#[no_mangle]
pub static ZN_CONFIG_MODE_KEY: c_uint = config::ZN_MODE_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_PEER_KEY: c_uint = config::ZN_PEER_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_LISTENER_KEY: c_uint = config::ZN_LISTENER_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_USER_KEY: c_uint = config::ZN_USER_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_PASSWORD_KEY: c_uint = config::ZN_PASSWORD_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_MULTICAST_SCOUTING_KEY: c_uint = config::ZN_MULTICAST_SCOUTING_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_MULTICAST_INTERFACE_KEY: c_uint = config::ZN_MULTICAST_INTERFACE_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_MULTICAST_IPV4_ADDRESS_KEY: c_uint =
    config::ZN_MULTICAST_IPV4_ADDRESS_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_SCOUTING_TIMEOUT_KEY: c_uint = config::ZN_SCOUTING_TIMEOUT_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_SCOUTING_DELAY_KEY: c_uint = config::ZN_SCOUTING_DELAY_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_ADD_TIMESTAMP_KEY: c_uint = config::ZN_ADD_TIMESTAMP_KEY as c_uint;
#[no_mangle]
pub static ZN_CONFIG_LOCAL_ROUTING_KEY: c_uint = config::ZN_LOCAL_ROUTING_KEY as c_uint;

// Properties returned by zn_info()
#[no_mangle]
pub static ZN_INFO_PID_KEY: c_uint = info::ZN_INFO_PID_KEY as c_uint;
#[no_mangle]
pub static ZN_INFO_PEER_PID_KEY: c_uint = info::ZN_INFO_PEER_PID_KEY as c_uint;
#[no_mangle]
pub static ZN_INFO_ROUTER_PID_KEY: c_uint = info::ZN_INFO_ROUTER_PID_KEY as c_uint;

pub trait FromRaw<T> {
    fn from_raw(r: T) -> Self;
    fn into_raw(self) -> T;
}

/// An owned, zenoh-allocated, null-terminated, string.  
/// Use `z_string_new` to construct and `z_string_free` to destroy.
///
/// Members:  
///     `start`: the start of the held null-terminated string. `nullptr` is a legal value for `start`
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct z_owned_string_t {
    pub _borrow: *const c_char,
}
#[allow(non_camel_case_types)]
pub type z_string_t = *const c_char;
/// Constructs a :c:type:`z_string_t` from a NULL terminated string.  
/// The contents of `s` is copied.
///
/// Parameters:  
///     s: The NULL terminated string.
///
/// Returns:  
///     A new :c:type:`z_string_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_string_new(s: *const c_char) -> z_owned_string_t {
    if s.is_null() {
        return z_owned_string_t { _borrow: s };
    }
    let inner = CStr::from_ptr(s).to_owned();
    let start = inner.as_ptr();
    std::mem::forget(inner);
    z_owned_string_t { _borrow: start }
}
/// Frees the passed z_string_t.
#[no_mangle]
pub extern "C" fn z_string_free(s: &mut z_owned_string_t) {
    if !s._borrow.is_null() {
        unsafe { CString::from_raw(s._borrow as *mut c_char) };
        s._borrow = std::ptr::null_mut();
    }
}
#[no_mangle]
pub extern "C" fn z_string_check(s: &z_owned_string_t) -> bool {
    !s._borrow.is_null()
}
#[no_mangle]
pub extern "C" fn z_string_borrow(s: &z_owned_string_t) -> *const c_char {
    s._borrow
}
impl From<String> for z_owned_string_t {
    fn from(s: String) -> Self {
        let inner = CString::new(s).unwrap();
        let start = inner.as_ptr();
        std::mem::forget(inner);
        z_owned_string_t { _borrow: start }
    }
}
impl Default for z_owned_string_t {
    fn default() -> Self {
        Self {
            _borrow: std::ptr::null(),
        }
    }
}
impl From<&str> for z_owned_string_t {
    fn from(s: &str) -> Self {
        let inner = CString::new(s).unwrap();
        let start = inner.as_ptr();
        std::mem::forget(inner);
        z_owned_string_t { _borrow: start }
    }
}
impl From<z_owned_string_t> for String {
    fn from(s: z_owned_string_t) -> Self {
        if s._borrow.is_null() {
            String::new()
        } else {
            unsafe { CString::from_raw(s._borrow as *mut c_char) }
                .into_string()
                .unwrap()
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_info_t<'a>(&'a z_owned_info_t);
pub const Z_INFO_PADDING_U64: usize = 6;
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_owned_info_t(pub(crate) [u64; Z_INFO_PADDING_U64]);
impl AsRef<Option<InfoProperties>> for z_owned_info_t {
    fn as_ref(&self) -> &Option<InfoProperties> {
        unsafe { std::mem::transmute(&self.0) }
    }
}
impl AsMut<Option<InfoProperties>> for z_owned_info_t {
    fn as_mut(&mut self) -> &mut Option<InfoProperties> {
        unsafe { std::mem::transmute(&mut self.0) }
    }
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_info_free(info: &mut z_owned_info_t) {
    std::mem::drop(info.as_mut().take())
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_info_check(info: &z_owned_info_t) -> bool {
    info.as_ref().is_some()
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_info_borrow(info: &z_owned_info_t) -> z_info_t {
    z_info_t(info)
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_info_get(info: z_info_t, key: u64) -> z_owned_string_t {
    let info = info.0.as_ref();
    match info.as_ref().map(|i| i.get(&key)).flatten() {
        None => z_owned_string_t::default(),
        Some(s) => s.as_str().into(),
    }
}

/// An owned array of owned NULL terminated strings, allocated by zenoh.
/// Use `z_str_array_free` to destroy.
///
/// Members:
///   char *const *val: A pointer to the array.
///   unsigned int len: The length of the array.
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

/// Free an array of NULL terminated strings and it's contained NULL terminated strings recursively.
///
/// Parameters:
///     strs: The array of NULL terminated strings to free.
///
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
}

/// An owned, zenoh-allocated, array of bytes.
///
/// Members:
///   const unsigned char *val: A pointer to the bytes array.
///   unsigned int len: The length of the bytes array.
///
#[repr(C)]
pub struct z_owned_bytes_t {
    pub val: *const u8,
    pub len: size_t,
}
#[repr(C)]
pub struct z_bytes_t {
    pub val: *const u8,
    pub len: size_t,
}
impl Default for z_owned_bytes_t {
    fn default() -> Self {
        z_owned_bytes_t {
            val: std::ptr::null(),
            len: 0,
        }
    }
}
/// Free a :c:type:`z_bytes_t`.
///
/// Parameters:
///    b : The array to free.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_bytes_free(b: &mut z_owned_bytes_t) {
    if !b.val.is_null() {
        std::mem::drop(Box::from_raw(
            std::slice::from_raw_parts(b.val, b.len) as *const [u8] as *mut [u8],
        ));
        b.val = std::ptr::null_mut();
    }
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_bytes_check(b: &z_owned_bytes_t) -> bool {
    !b.val.is_null()
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_bytes_borrow(b: &z_owned_bytes_t) -> z_bytes_t {
    z_bytes_t {
        val: b.val,
        len: b.len,
    }
}
impl From<PeerId> for z_owned_bytes_t {
    #[inline]
    fn from(pid: PeerId) -> Self {
        let pid = pid.as_slice().to_vec().into_boxed_slice();
        let res = z_owned_bytes_t {
            val: pid.as_ptr(),
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
                val: std::ptr::null(),
                len: 0,
            },
        }
    }
}
impl From<ZBuf> for z_owned_bytes_t {
    fn from(buf: ZBuf) -> Self {
        let data = buf.to_vec().into_boxed_slice();
        let res = z_owned_bytes_t {
            val: data.as_ptr(),
            len: data.len(),
        };
        std::mem::forget(data);
        res
    }
}

/// A resource key.
///
/// Resources are identified by URI like string names.  
/// Examples : ``"/some/resource/key"``.
/// Resource names can be mapped to numerical ids through :c:func:`zn_declare_resource`
/// for wire and computation efficiency.
///
/// A resource key can be either:
///   - A plain string resource name.
///   - A pure numerical id.
///   - The combination of a numerical prefix and a string suffix.
///
/// Members:
///   unsigned long id: The id or prefix of this resource key. ``0`` if empty.
///   z_string_t suffix: The suffix of the ressource key. May be an empty string.
#[repr(C)]
pub struct z_owned_reskey_t {
    pub id: c_ulong,
    pub suffix: z_owned_string_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_reskey_t {
    pub id: c_ulong,
    pub suffix: *const c_char,
}

/// Free a :c:type:`z_owned_reskey_t`.
///
/// Parameters:
///    b : The array to free.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_reskey_new(id: c_ulong, suffix: *const c_char) -> z_owned_reskey_t {
    z_owned_reskey_t {
        id,
        suffix: z_string_new(suffix),
    }
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_reskey_new_borrowed(id: c_ulong, suffix: *const c_char) -> z_reskey_t {
    z_reskey_t { id, suffix }
}
/// Free a :c:type:`z_owned_reskey_t`.
///
/// Parameters:
///    b : The array to free.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_reskey_free(reskey: &mut z_owned_reskey_t) {
    z_string_free(&mut reskey.suffix)
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_reskey_check(reskey: &z_owned_reskey_t) -> bool {
    reskey.id != 0 || z_string_check(&reskey.suffix)
}
#[no_mangle]
pub extern "C" fn z_reskey_borrow(reskey: &z_owned_reskey_t) -> z_reskey_t {
    z_reskey_t {
        id: reskey.id,
        suffix: reskey.suffix._borrow,
    }
}

impl<'a> From<&'a z_owned_reskey_t> for ResKey<'a> {
    fn from(r: &'a z_owned_reskey_t) -> Self {
        unsafe {
            let len = if r.suffix._borrow.is_null() {
                0
            } else {
                libc::strlen(r.suffix._borrow)
            };
            match (r.id, len) {
                (id, 0) => ResKey::RId(id),
                (0, _) => ResKey::RName(
                    std::str::from_utf8(std::slice::from_raw_parts(
                        r.suffix._borrow as *const _,
                        len,
                    ))
                    .unwrap()
                    .into(),
                ),
                (id, _) => ResKey::RIdWithSuffix(
                    id,
                    std::str::from_utf8(std::slice::from_raw_parts(
                        r.suffix._borrow as *const _,
                        len,
                    ))
                    .unwrap()
                    .into(),
                ),
            }
        }
    }
}

impl<'a> From<z_reskey_t> for ResKey<'a> {
    fn from(r: z_reskey_t) -> Self {
        unsafe {
            let len = if r.suffix.is_null() {
                0
            } else {
                libc::strlen(r.suffix)
            };
            match (r.id, len) {
                (id, 0) => ResKey::RId(id),
                (0, _) => ResKey::RName(
                    std::str::from_utf8(std::slice::from_raw_parts(r.suffix as *const _, len))
                        .unwrap()
                        .into(),
                ),
                (id, _) => ResKey::RIdWithSuffix(
                    id,
                    std::str::from_utf8(std::slice::from_raw_parts(r.suffix as *const _, len))
                        .unwrap()
                        .into(),
                ),
            }
        }
    }
}

impl FromRaw<z_owned_reskey_t> for ResKey<'static> {
    #[inline]
    fn from_raw(r: z_owned_reskey_t) -> ResKey<'static> {
        if r.suffix._borrow.is_null() {
            ResKey::RId(r.id as ZInt)
        } else if r.id != 0 {
            ResKey::RIdWithSuffix(r.id as ZInt, String::from(r.suffix).into())
        } else {
            ResKey::RName(String::from(r.suffix).into())
        }
    }
    #[inline]
    fn into_raw(self) -> z_owned_reskey_t {
        match self {
            ResKey::RId(rid) => z_owned_reskey_t {
                id: rid as c_ulong,
                suffix: unsafe { z_string_new(std::ptr::null()) },
            },
            ResKey::RIdWithSuffix(rid, suffix) => z_owned_reskey_t {
                id: rid as c_ulong,
                suffix: z_owned_string_t::from(suffix.into_owned()),
            },
            ResKey::RName(suffix) => z_owned_reskey_t {
                id: 0,
                suffix: z_owned_string_t::from(suffix.into_owned()),
            },
        }
    }
}

/// A zenoh-net data sample.
///
/// A sample is the value associated to a given resource at a given point in time.
///
/// Members:
///   z_string_t key: The resource key of this data sample.
///   z_bytes_t value: The value of this data sample.
#[repr(C)]
pub struct z_owned_sample_t {
    pub key: z_owned_string_t,
    pub value: z_owned_bytes_t,
}
#[repr(C)]
pub struct z_sample_t {
    pub key: z_string_t,
    pub value: z_bytes_t,
}

impl From<Sample> for z_owned_sample_t {
    #[inline]
    fn from(s: Sample) -> Self {
        z_owned_sample_t {
            key: s.res_name.into(),
            value: s.value.payload.into(),
        }
    }
}

/// Free a :c:type:`zn_sample_t` contained key and value.
///
/// Parameters:
///     sample: The :c:type:`zn_sample_t` to free.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_sample_free(sample: &mut z_owned_sample_t) {
    z_string_free(&mut sample.key);
    z_bytes_free(&mut sample.value);
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_sample_check(sample: &z_owned_sample_t) -> bool {
    z_string_check(&sample.key) && z_bytes_check(&sample.value)
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_sample_borrow(sample: &z_owned_sample_t) -> z_sample_t {
    z_sample_t {
        key: z_string_borrow(&sample.key),
        value: z_bytes_borrow(&sample.value),
    }
}

/// A hello message returned by a zenoh entity to a scout message sent with :c:func:`zn_scout`.
///
/// Members:
///   unsigned int whatami: The kind of zenoh entity.
///   z_bytes_t pid: The peer id of the scouted entity (empty if absent).
///   z_str_array_t locators: The locators of the scouted entity.
///
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
                None => ZN_ROUTER,
            },
            pid: h.pid.into(),
            locators: h.locators.into(),
        }
    }
}

/// An array of :c:struct:`zn_hello_t` messages.
///
/// Members:
///   const zn_hello_t *val: A pointer to the array.
///   unsigned int len: The length of the array.
///
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
/// Free an array of :c:struct:`zn_hello_t` messages and it's contained :c:struct:`zn_hello_t` messages recursively.
///
/// Parameters:
///     strs: The array of :c:struct:`zn_hello_t` messages to free.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_hello_array_free(hellos: &mut z_owned_hello_array_t) {
    let hellos = Vec::from_raw_parts(
        hellos.val as *mut z_owned_hello_t,
        hellos.len as usize,
        hellos.len as usize,
    );
    for mut hello in hellos {
        z_bytes_free(&mut hello.pid);
        z_str_array_free(&mut hello.locators);
    }
}

/// The behavior to adopt in case of congestion while routing some data.
///
///     - **zn_congestion_control_t_BLOCK**
///     - **zn_congestion_control_t_DROP**
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum zn_congestion_control_t {
    BLOCK,
    DROP,
}

impl From<zn_congestion_control_t> for CongestionControl {
    fn from(val: zn_congestion_control_t) -> Self {
        match val {
            zn_congestion_control_t::BLOCK => CongestionControl::Block,
            zn_congestion_control_t::DROP => CongestionControl::Drop,
        }
    }
}

/// The subscription reliability.
///
///     - **zn_reliability_t_BEST_EFFORT**
///     - **zn_reliability_t_RELIABLE**
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
///     - **zn_submode_t_PUSH**
///     - **zn_submode_t_PULL**
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
///     unsigned int origin:
///     unsigned int period:
///     unsigned int duration:
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

/// Informations to be passed to :c:func:`zn_declare_subscriber` to configure the created :c:type:`zn_subscriber_t`.
///
/// Members:
///     zn_reliability_t reliability: The subscription reliability.
///     zn_submode_t mode: The subscription mode.
///     zn_period_t *period: The subscription period.
#[repr(C)]
pub struct z_subinfo_t {
    pub reliability: z_reliability_t,
    pub mode: z_submode_t,
    pub period: z_period_t,
}
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

/// An reply to a :c:func:`zn_query` (or :c:func:`zn_query_collect`).
///
/// Members:
///   zn_sample_t data: a :c:type:`zn_sample_t` containing the key and value of the reply.
///   unsigned int source_kind: The kind of the replier that sent this reply.
///   z_bytes_t replier_id: The id of the replier that sent this reply.
///
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_owned_reply_data_t {
    data: z_owned_sample_t,
    source_kind: c_uint,
    replier_id: z_owned_bytes_t,
}
impl z_owned_reply_data_t {
    #[inline]
    pub(crate) fn empty() -> Self {
        z_owned_reply_data_t {
            data: z_owned_sample_t {
                key: z_owned_string_t::default(),
                value: z_owned_bytes_t::default(),
            },
            source_kind: 0,
            replier_id: z_owned_bytes_t::default(),
        }
    }
}
impl From<Reply> for z_owned_reply_data_t {
    #[inline]
    fn from(r: Reply) -> Self {
        z_owned_reply_data_t {
            data: r.data.into(),
            source_kind: r.replier_kind as c_uint,
            replier_id: r.replier_id.into(),
        }
    }
}

/// Free a :c:type:`zn_reply_data_t` contained data and replier_id.
///
/// Parameters:
///     reply_data: The :c:type:`zn_reply_data_t` to free.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_reply_data_free(reply_data: &mut z_owned_reply_data_t) {
    z_sample_free(&mut reply_data.data);
    z_bytes_free(&mut reply_data.replier_id);
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_reply_data_check(reply_data: &z_owned_reply_data_t) -> bool {
    z_sample_check(&reply_data.data) && z_bytes_check(&reply_data.replier_id)
}

/// An array of :c:type:`zn_reply_data_t`.
/// Result of :c:func:`zn_query_collect`.
///
/// Members:
///   char *const *val: A pointer to the array.
///   unsigned int len: The length of the array.
///
#[repr(C)]
pub struct z_owned_reply_data_array_t {
    pub val: *const z_owned_reply_data_t,
    pub len: size_t,
}

/// Free a :c:type:`zn_reply_data_array_t` and it's contained replies.
///
/// Parameters:
///     replies: The :c:type:`zn_reply_data_array_t` to free.
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
    !replies.val.is_null()
}

/// The possible values of :c:member:`zn_reply_t.tag`
///
///     - **zn_reply_t_Tag_DATA**: The reply contains some data.
///     - **zn_reply_t_Tag_FINAL**: The reply does not contain any data and indicates that there will be no more replies for this query.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum z_reply_t_Tag {
    DATA,
    FINAL,
}

/// An reply to a :c:func:`zn_query`.
///
/// Members:
///   zn_reply_t_Tag tag: Indicates if the reply contains data or if it's a FINAL reply.
///   zn_reply_data_t data: The reply data if :c:member:`zn_reply_t.tag` equals :c:member:`zn_reply_t_Tag.zn_reply_t_Tag_DATA`.
///
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_owned_reply_t {
    pub tag: z_reply_t_Tag,
    pub data: z_owned_reply_data_t,
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_reply_free(reply: &mut z_owned_reply_t) {
    if reply.tag == z_reply_t_Tag::DATA {
        z_reply_data_free(&mut reply.data)
    }
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_reply_check(reply: &z_owned_reply_t) -> bool {
    z_reply_t_Tag::FINAL == reply.tag
        || (z_reply_t_Tag::DATA == reply.tag && z_reply_data_check(&reply.data))
}

/// The possible values of :c:member:`zn_target_t.tag`.
///
///     - **zn_target_t_BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
///     - **zn_target_t_COMPLETE**: A set of complete queryables.
///     - **zn_target_t_ALL**: All matching queryables.
///     - **zn_target_t_NONE**: No queryables.
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum z_target_t {
    BEST_MATCHING,
    ALL,
    NONE,
    ALL_COMPLETE,
    #[cfg(feature = "complete_n")]
    COMPLETE {
        n: c_uint,
    },
}

impl From<Target> for z_target_t {
    #[inline]
    fn from(t: Target) -> Self {
        match t {
            Target::BestMatching => z_target_t::BEST_MATCHING,
            Target::All => z_target_t::ALL,
            Target::None => z_target_t::NONE,
            Target::AllComplete => z_target_t::ALL_COMPLETE,
            #[cfg(feature = "complete_n")]
            Target::Complete(n) => z_target_t::COMPLETE { n: n as c_uint },
        }
    }
}

impl From<z_target_t> for Target {
    #[inline]
    fn from(val: z_target_t) -> Self {
        match val {
            z_target_t::BEST_MATCHING => Target::BestMatching,
            z_target_t::ALL => Target::All,
            z_target_t::NONE => Target::None,
            z_target_t::ALL_COMPLETE => Target::AllComplete,
            #[cfg(feature = "complete_n")]
            z_target_t::COMPLETE { n } => Target::Complete(n as ZInt),
        }
    }
}

/// Create a default :c:type:`zn_target_t`.
#[no_mangle]
pub extern "C" fn z_target_default() -> z_target_t {
    Target::default().into()
}

/// The zenoh-net queryables that should be target of a :c:func:`zn_query`.
///
/// Members:
///     unsigned int kind: A mask of queryable kinds.
///     zn_target_t target: The query target.
#[repr(C)]
pub struct z_query_target_t {
    pub kind: c_uint,
    pub target: z_target_t,
}
impl From<QueryTarget> for z_query_target_t {
    #[inline]
    fn from(qt: QueryTarget) -> Self {
        z_query_target_t {
            kind: qt.kind as c_uint,
            target: qt.target.into(),
        }
    }
}
impl From<z_query_target_t> for QueryTarget {
    #[inline]
    fn from(val: z_query_target_t) -> Self {
        QueryTarget {
            kind: val.kind.into(),
            target: val.target.into(),
        }
    }
}

/// Create a default :c:type:`zn_query_target_t`.
#[no_mangle]
pub extern "C" fn z_query_target_default() -> z_query_target_t {
    QueryTarget::default().into()
}

/// The kind of consolidation that should be applied on replies to a :c:func:`zn_query`.
///
///     - **zn_consolidation_mode_t_FULL**: Guaranties unicity of replies. Optimizes bandwidth.
///     - **zn_consolidation_mode_t_LAZY**: Does not garanty unicity. Optimizes latency.
///     - **zn_consolidation_mode_t_NONE**: No consolidation.
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

/// The kind of consolidation that should be applied on replies to a :c:func:`zn_query`
/// at the different stages of the reply process.
///
/// Members:
///   zn_consolidation_mode_t first_routers: The consolidation mode to apply on first routers of the replies routing path.
///   zn_consolidation_mode_t last_router: The consolidation mode to apply on last router of the replies routing path.
///   zn_consolidation_mode_t reception: The consolidation mode to apply at reception of the replies.
#[repr(C)]
pub struct z_query_consolidation_t {
    pub first_routers: z_consolidation_mode_t,
    pub last_router: z_consolidation_mode_t,
    pub reception: z_consolidation_mode_t,
}

impl From<QueryConsolidation> for z_query_consolidation_t {
    #[inline]
    fn from(qc: QueryConsolidation) -> Self {
        z_query_consolidation_t {
            first_routers: qc.first_routers.into(),
            last_router: qc.last_router.into(),
            reception: qc.reception.into(),
        }
    }
}

impl From<z_query_consolidation_t> for QueryConsolidation {
    #[inline]
    fn from(val: z_query_consolidation_t) -> Self {
        QueryConsolidation {
            first_routers: val.first_routers.into(),
            last_router: val.last_router.into(),
            reception: val.reception.into(),
        }
    }
}

/// Create a default :c:type:`zn_query_consolidation_t`.
#[no_mangle]
pub extern "C" fn z_query_consolidation_default() -> z_query_consolidation_t {
    QueryConsolidation::default().into()
}
