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
use crate::net::{string_into_raw_parts, vec_into_raw_parts};
use libc::{c_char, c_uint, c_ulong, size_t};
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
};
use zenoh::{
    config,
    config::WhatAmI,
    info,
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

/// An owned, rust-allocated, string.
///
/// Members:
///   const char *val: A pointer to the string.
///   unsigned int len: The length of the string.
///
#[repr(C)]
pub struct z_string_t {
    pub val: *const c_char,
    pub len: size_t,
}

impl FromRaw<z_string_t> for String {
    #[inline]
    fn from_raw(r: z_string_t) -> Self {
        unsafe { String::from_raw_parts(r.val as *mut u8, r.len, r.len) }
    }

    #[inline]
    fn into_raw(self) -> z_string_t {
        let (ptr, len, _) = string_into_raw_parts(self);
        z_string_t {
            val: ptr as *const c_char,
            len: len as size_t,
        }
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_string_free(zs: z_string_t) {
    let _ = String::from_raw(zs);
}

/// Construct a :c:type:`z_string_t` from a NULL terminated string.
/// The content of the given string is copied.
///
/// Parameters:
///     s: The NULL terminated string.
///
/// Returns:
///     A new :c:type:`z_string_t`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_string_make(s: *const c_char) -> z_string_t {
    String::into_raw(CStr::from_ptr(s).to_string_lossy().into_owned())
}

/// An array of NULL terminated strings.
///
/// Members:
///   char *const *val: A pointer to the array.
///   unsigned int len: The length of the array.
///
#[repr(C)]
pub struct z_str_array_t {
    pub val: *const *const c_char,
    pub len: size_t,
}

impl<T> From<Vec<T>> for z_str_array_t
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
        let res = z_str_array_t {
            val: v.as_ptr(),
            len: v.len() as size_t,
        };
        std::mem::forget(v);
        res
    }
}

impl<T> From<Option<Vec<T>>> for z_str_array_t
where
    T: ToString,
{
    #[inline]
    fn from(v: Option<Vec<T>>) -> Self {
        match v {
            Some(v) => v.into(),
            None => z_str_array_t {
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
pub unsafe extern "C" fn zn_str_array_free(strs: z_str_array_t) {
    let locators = Vec::from_raw_parts(
        strs.val as *mut *const c_char,
        strs.len as usize,
        strs.len as usize,
    );
    for locator in locators {
        let _ = CString::from_raw(locator as *mut c_char);
    }
}

/// An array of bytes.
///
/// Members:
///   const unsigned char *val: A pointer to the bytes array.
///   unsigned int len: The length of the bytes array.
///
#[repr(C)]
pub struct z_bytes_t {
    pub val: *const u8,
    pub len: size_t,
}

impl From<PeerId> for z_bytes_t {
    #[inline]
    fn from(pid: PeerId) -> Self {
        let pid = pid.as_slice().to_vec().into_boxed_slice();
        let res = z_bytes_t {
            val: pid.as_ptr(),
            len: pid.len() as size_t,
        };
        std::mem::forget(pid);
        res
    }
}

impl From<Option<PeerId>> for z_bytes_t {
    #[inline]
    fn from(pid: Option<PeerId>) -> Self {
        match pid {
            Some(pid) => pid.into(),
            None => z_bytes_t {
                val: std::ptr::null(),
                len: 0,
            },
        }
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
///
///   - A plain string resource name.
///   - A pure numerical id.
///   - The combination of a numerical prefix and a string suffix.
///
/// Members:
///   unsigned long id: The id or prefix of this resource key. ``0`` if empty.
///   const char *suffix: The suffix of this resource key. ``NULL`` if pure numerical id.
#[repr(C)]
pub struct zn_reskey_t {
    pub id: c_ulong,
    pub suffix: *const c_char,
}

impl FromRaw<zn_reskey_t> for ResKey<'static> {
    #[inline]
    fn from_raw(r: zn_reskey_t) -> ResKey<'static> {
        unsafe {
            if r.suffix.is_null() || libc::strlen(r.suffix) == 0 {
                ResKey::RId(r.id as ZInt)
            } else if r.id != 0 {
                ResKey::RIdWithSuffix(
                    r.id as ZInt,
                    Cow::Owned(String::from_raw_parts(
                        r.suffix as *mut u8,
                        libc::strlen(r.suffix),
                        libc::strlen(r.suffix) + 1,
                    )),
                )
            } else {
                ResKey::RName(
                    String::from_raw_parts(
                        r.suffix as *mut u8,
                        libc::strlen(r.suffix),
                        libc::strlen(r.suffix) + 1,
                    )
                    .into(),
                )
            }
        }
    }

    #[inline]
    fn into_raw(self) -> zn_reskey_t {
        match self {
            ResKey::RId(rid) => zn_reskey_t {
                id: rid as c_ulong,
                suffix: std::ptr::null(),
            },
            ResKey::RIdWithSuffix(rid, suffix) => zn_reskey_t {
                id: rid as c_ulong,
                suffix: string_into_raw_parts(suffix.into_owned()).0,
            },
            ResKey::RName(suffix) => zn_reskey_t {
                id: 0,
                suffix: string_into_raw_parts(suffix.into_owned()).0,
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
pub struct zn_sample_t {
    pub key: z_string_t,
    pub value: z_bytes_t,
}

impl From<Sample> for zn_sample_t {
    #[inline]
    fn from(s: Sample) -> Self {
        //TODO replace when stable https://github.com/rust-lang/rust/issues/65816
        let (val, len, _cap) = vec_into_raw_parts(s.value.payload.to_vec());
        zn_sample_t {
            key: s.res_name.into_raw(),
            value: z_bytes_t { val, len },
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
pub unsafe extern "C" fn zn_sample_free(sample: zn_sample_t) {
    String::from_raw(sample.key);
    Vec::from_raw_parts(
        sample.value.val as *mut zn_reply_data_t,
        sample.value.len,
        sample.value.len,
    );
}

/// A hello message returned by a zenoh entity to a scout message sent with :c:func:`zn_scout`.
///
/// Members:
///   unsigned int whatami: The kind of zenoh entity.
///   z_bytes_t pid: The peer id of the scouted entity (empty if absent).
///   z_str_array_t locators: The locators of the scouted entity.
///
#[repr(C)]
pub struct zn_hello_t {
    pub whatami: c_uint,
    pub pid: z_bytes_t,
    pub locators: z_str_array_t,
}

impl From<Hello> for zn_hello_t {
    #[inline]
    fn from(h: Hello) -> Self {
        zn_hello_t {
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
pub struct zn_hello_array_t {
    pub val: *const zn_hello_t,
    pub len: size_t,
}

impl From<Vec<Hello>> for zn_hello_array_t {
    #[inline]
    fn from(hvec: Vec<Hello>) -> Self {
        let mut hvec = hvec
            .into_iter()
            .map(|h| h.into())
            .collect::<Vec<zn_hello_t>>();
        hvec.shrink_to_fit();
        let res = zn_hello_array_t {
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
pub unsafe extern "C" fn zn_hello_array_free(hellos: zn_hello_array_t) {
    let hellos = Vec::from_raw_parts(
        hellos.val as *mut zn_hello_t,
        hellos.len as usize,
        hellos.len as usize,
    );
    for hello in hellos {
        zn_str_array_free(hello.locators);
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
pub enum zn_reliability_t {
    BEST_EFFORT,
    RELIABLE,
}

impl From<Reliability> for zn_reliability_t {
    #[inline]
    fn from(r: Reliability) -> Self {
        match r {
            Reliability::BestEffort => zn_reliability_t::BEST_EFFORT,
            Reliability::Reliable => zn_reliability_t::RELIABLE,
        }
    }
}

impl From<zn_reliability_t> for Reliability {
    #[inline]
    fn from(val: zn_reliability_t) -> Self {
        match val {
            zn_reliability_t::BEST_EFFORT => Reliability::BestEffort,
            zn_reliability_t::RELIABLE => Reliability::Reliable,
        }
    }
}

/// The subscription mode.
///
///     - **zn_submode_t_PUSH**
///     - **zn_submode_t_PULL**
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum zn_submode_t {
    PUSH,
    PULL,
}

impl From<SubMode> for zn_submode_t {
    #[inline]
    fn from(sm: SubMode) -> Self {
        match sm {
            SubMode::Push => zn_submode_t::PUSH,
            SubMode::Pull => zn_submode_t::PULL,
        }
    }
}

impl From<zn_submode_t> for SubMode {
    #[inline]
    fn from(val: zn_submode_t) -> Self {
        match val {
            zn_submode_t::PUSH => SubMode::Push,
            zn_submode_t::PULL => SubMode::Pull,
        }
    }
}

/// The subscription period.
///
/// Members:
///     unsigned int origin:
///     unsigned int period:
///     unsigned int duration:
#[repr(C)]
pub struct zn_period_t {
    pub origin: c_uint,
    pub period: c_uint,
    pub duration: c_uint,
}

impl From<Period> for zn_period_t {
    #[inline]
    fn from(p: Period) -> Self {
        zn_period_t {
            origin: p.origin as c_uint,
            period: p.period as c_uint,
            duration: p.duration as c_uint,
        }
    }
}

impl From<zn_period_t> for Period {
    #[inline]
    fn from(val: zn_period_t) -> Self {
        Period {
            origin: val.origin as ZInt,
            period: val.period as ZInt,
            duration: val.duration as ZInt,
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
pub struct zn_subinfo_t {
    pub reliability: zn_reliability_t,
    pub mode: zn_submode_t,
    pub period: *mut zn_period_t,
}

impl From<SubInfo> for zn_subinfo_t {
    #[inline]
    fn from(si: SubInfo) -> Self {
        zn_subinfo_t {
            reliability: si.reliability.into(),
            mode: si.mode.into(),
            period: match si.period {
                Some(period) => Box::into_raw(Box::new(period.into())),
                None => std::ptr::null_mut(),
            },
        }
    }
}

impl From<zn_subinfo_t> for SubInfo {
    #[inline]
    fn from(val: zn_subinfo_t) -> Self {
        unsafe {
            SubInfo {
                reliability: val.reliability.into(),
                mode: val.mode.into(),
                period: if !val.period.is_null() {
                    Some((*Box::from_raw(val.period)).into())
                } else {
                    None
                },
            }
        }
    }
}

/// Create a default subscription info.
#[no_mangle]
pub extern "C" fn zn_subinfo_default() -> zn_subinfo_t {
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
pub struct zn_reply_data_t {
    data: zn_sample_t,
    source_kind: c_uint,
    replier_id: z_bytes_t,
}

impl zn_reply_data_t {
    #[inline]
    pub(crate) fn empty() -> Self {
        zn_reply_data_t {
            data: zn_sample_t {
                key: z_string_t {
                    val: std::ptr::null(),
                    len: 0,
                },
                value: z_bytes_t {
                    val: std::ptr::null(),
                    len: 0,
                },
            },
            source_kind: 0,
            replier_id: z_bytes_t {
                val: std::ptr::null(),
                len: 0,
            },
        }
    }
}

impl From<Reply> for zn_reply_data_t {
    #[inline]
    fn from(r: Reply) -> Self {
        zn_reply_data_t {
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
pub unsafe extern "C" fn zn_reply_data_free(reply_data: zn_reply_data_t) {
    zn_sample_free(reply_data.data);
    Vec::from_raw_parts(
        reply_data.replier_id.val as *mut u8,
        reply_data.replier_id.len,
        reply_data.replier_id.len,
    );
}

/// An array of :c:type:`zn_reply_data_t`.
/// Result of :c:func:`zn_query_collect`.
///
/// Members:
///   char *const *val: A pointer to the array.
///   unsigned int len: The length of the array.
///
#[repr(C)]
pub struct zn_reply_data_array_t {
    pub val: *const zn_reply_data_t,
    pub len: size_t,
}

/// Free a :c:type:`zn_reply_data_array_t` and it's contained replies.
///
/// Parameters:
///     replies: The :c:type:`zn_reply_data_array_t` to free.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn zn_reply_data_array_free(replies: zn_reply_data_array_t) {
    let vec = Vec::from_raw_parts(
        replies.val as *mut zn_reply_data_t,
        replies.len,
        replies.len,
    );
    for rd in vec {
        zn_reply_data_free(rd);
    }
}

/// The possible values of :c:member:`zn_reply_t.tag`
///
///     - **zn_reply_t_Tag_DATA**: The reply contains some data.
///     - **zn_reply_t_Tag_FINAL**: The reply does not contain any data and indicates that there will be no more replies for this query.
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum zn_reply_t_Tag {
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
pub struct zn_reply_t {
    pub tag: zn_reply_t_Tag,
    pub data: zn_reply_data_t,
}

/// The possible values of :c:member:`zn_target_t.tag`.
///
///     - **zn_target_t_BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
///     - **zn_target_t_COMPLETE**: A set of complete queryables.
///     - **zn_target_t_ALL**: All matching queryables.
///     - **zn_target_t_NONE**: No queryables.
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum zn_target_t {
    BEST_MATCHING,
    COMPLETE { n: c_uint },
    ALL,
    NONE,
}

impl From<Target> for zn_target_t {
    #[inline]
    fn from(t: Target) -> Self {
        match t {
            Target::BestMatching => zn_target_t::BEST_MATCHING,
            // Target::Complete { n } => zn_target_t::COMPLETE { n: n as c_uint },
            Target::All => zn_target_t::ALL,
            Target::None => zn_target_t::NONE,
            Target::AllComplete => todo!(),
        }
    }
}

impl From<zn_target_t> for Target {
    #[inline]
    fn from(val: zn_target_t) -> Self {
        match val {
            zn_target_t::BEST_MATCHING => Target::BestMatching,
            zn_target_t::COMPLETE { .. } => {
                todo!("zenoh feature \"complete\" not supported by zenoh-c")
            } // Target::Complete { n: n as ZInt },
            zn_target_t::ALL => Target::All,
            zn_target_t::NONE => Target::None,
        }
    }
}

/// Create a default :c:type:`zn_target_t`.
#[no_mangle]
pub extern "C" fn zn_target_default() -> zn_target_t {
    Target::default().into()
}

/// The zenoh-net queryables that should be target of a :c:func:`zn_query`.
///
/// Members:
///     unsigned int kind: A mask of queryable kinds.
///     zn_target_t target: The query target.
#[repr(C)]
pub struct zn_query_target_t {
    pub kind: c_uint,
    pub target: zn_target_t,
}

impl From<QueryTarget> for zn_query_target_t {
    #[inline]
    fn from(qt: QueryTarget) -> Self {
        zn_query_target_t {
            kind: qt.kind as c_uint,
            target: qt.target.into(),
        }
    }
}

impl From<zn_query_target_t> for QueryTarget {
    #[inline]
    fn from(val: zn_query_target_t) -> Self {
        QueryTarget {
            kind: val.kind.into(),
            target: val.target.into(),
        }
    }
}

/// Create a default :c:type:`zn_query_target_t`.
#[no_mangle]
pub extern "C" fn zn_query_target_default() -> zn_query_target_t {
    QueryTarget::default().into()
}

/// The kind of consolidation that should be applied on replies to a :c:func:`zn_query`.
///
///     - **zn_consolidation_mode_t_FULL**: Guaranties unicity of replies. Optimizes bandwidth.
///     - **zn_consolidation_mode_t_LAZY**: Does not garanty unicity. Optimizes latency.
///     - **zn_consolidation_mode_t_NONE**: No consolidation.
#[repr(C)]
pub enum zn_consolidation_mode_t {
    FULL,
    LAZY,
    NONE,
}

impl From<ConsolidationMode> for zn_consolidation_mode_t {
    #[inline]
    fn from(cm: ConsolidationMode) -> Self {
        match cm {
            ConsolidationMode::Full => zn_consolidation_mode_t::FULL,
            ConsolidationMode::Lazy => zn_consolidation_mode_t::LAZY,
            ConsolidationMode::None => zn_consolidation_mode_t::NONE,
        }
    }
}

impl From<zn_consolidation_mode_t> for ConsolidationMode {
    #[inline]
    fn from(val: zn_consolidation_mode_t) -> Self {
        match val {
            zn_consolidation_mode_t::NONE => ConsolidationMode::None,
            zn_consolidation_mode_t::LAZY => ConsolidationMode::Lazy,
            zn_consolidation_mode_t::FULL => ConsolidationMode::Full,
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
pub struct zn_query_consolidation_t {
    pub first_routers: zn_consolidation_mode_t,
    pub last_router: zn_consolidation_mode_t,
    pub reception: zn_consolidation_mode_t,
}

impl From<QueryConsolidation> for zn_query_consolidation_t {
    #[inline]
    fn from(qc: QueryConsolidation) -> Self {
        zn_query_consolidation_t {
            first_routers: qc.first_routers.into(),
            last_router: qc.last_router.into(),
            reception: qc.reception.into(),
        }
    }
}

impl From<zn_query_consolidation_t> for QueryConsolidation {
    #[inline]
    fn from(val: zn_query_consolidation_t) -> Self {
        QueryConsolidation {
            first_routers: val.first_routers.into(),
            last_router: val.last_router.into(),
            reception: val.reception.into(),
        }
    }
}

/// Create a default :c:type:`zn_query_consolidation_t`.
#[no_mangle]
pub extern "C" fn zn_query_consolidation_default() -> zn_query_consolidation_t {
    QueryConsolidation::default().into()
}
