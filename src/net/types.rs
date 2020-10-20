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
use libc::{c_char, c_uchar, c_uint};
use std::ffi::CString;
use zenoh::net::*;

#[no_mangle]
pub static ZN_ROUTER: c_uint = whatami::ROUTER as c_uint;
#[no_mangle]
pub static ZN_PEER: c_uint = whatami::PEER as c_uint;
#[no_mangle]
pub static ZN_CLIENT: c_uint = whatami::CLIENT as c_uint;

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
pub static ZN_CONFIG_MULTICAST_ADDRESS_KEY: c_uint = config::ZN_MULTICAST_ADDRESS_KEY as c_uint;
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

/// A string.
///
/// Members:
///   const char *val: A pointer to the string.
///   unsigned int len: The length of the string.
///
#[repr(C)]
pub struct zn_string_t {
    pub val: *const c_char,
    pub len: c_uint,
}

/// An array of NULL terminated strings.
///
/// Members:
///   char *const *val: A pointer to the array.
///   unsigned int len: The length of the array.
///
#[repr(C)]
pub struct zn_str_array_t {
    pub val: *const *const c_char,
    pub len: c_uint,
}

impl<T> From<Vec<T>> for zn_str_array_t
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
        let res = zn_str_array_t {
            val: v.as_ptr(),
            len: v.len() as c_uint,
        };
        std::mem::forget(v);
        res
    }
}

impl<T> From<Option<Vec<T>>> for zn_str_array_t
where
    T: ToString,
{
    #[inline]
    fn from(v: Option<Vec<T>>) -> Self {
        match v {
            Some(v) => v.into(),
            None => zn_str_array_t {
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
pub unsafe extern "C" fn zn_str_array_free(strs: zn_str_array_t) {
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
pub struct zn_bytes_t {
    pub val: *const c_uchar,
    pub len: c_uint,
}

impl From<PeerId> for zn_bytes_t {
    #[inline]
    fn from(pid: PeerId) -> Self {
        let pid = pid.as_slice().to_vec().into_boxed_slice();
        let res = zn_bytes_t {
            val: pid.as_ptr(),
            len: pid.len() as c_uint,
        };
        std::mem::forget(pid);
        res
    }
}

impl From<Option<PeerId>> for zn_bytes_t {
    #[inline]
    fn from(pid: Option<PeerId>) -> Self {
        match pid {
            Some(pid) => pid.into(),
            None => zn_bytes_t {
                val: std::ptr::null(),
                len: 0,
            },
        }
    }
}

/// A zenoh-net data sample.
///
/// A sample is the value associated to a given resource at a given point in time.
///
/// Members:
///   zn_string_t key: The resource key of this data sample.
///   zn_bytes_t value: The value of this data sample.
#[repr(C)]
pub struct zn_sample_t {
    pub key: zn_string_t,
    pub value: zn_bytes_t,
}

/// Information on the source of a reply.
///
/// Members:
///   unsigned int kind: The kind of source.
///   zn_bytes_t id: The unique id of the source.
#[repr(C)]
pub struct zn_source_info_t {
    pub kind: c_uint,
    pub id: zn_bytes_t,
}

/// A hello message returned by a zenoh entity to a scout message sent with :c:func:`zn_scout`.
///
/// Members:
///   unsigned int whatami: The kind of zenoh entity.
///   zn_bytes_t pid: The peer id of the scouted entity (empty if absent).
///   zn_str_array_t locators: The locators of the scouted entity.
///
#[repr(C)]
pub struct zn_hello_t {
    pub whatami: c_uint,
    pub pid: zn_bytes_t,
    pub locators: zn_str_array_t,
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
    pub len: c_uint,
}

impl From<Vec<Hello>> for zn_hello_array_t {
    fn from(hvec: Vec<Hello>) -> Self {
        let mut hvec = hvec
            .into_iter()
            .map(|h| h.into())
            .collect::<Vec<zn_hello_t>>();
        hvec.shrink_to_fit();
        let res = zn_hello_array_t {
            val: hvec.as_ptr(),
            len: hvec.len() as c_uint,
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
    fn from(r: Reliability) -> Self {
        match r {
            Reliability::BestEffort => zn_reliability_t::BEST_EFFORT,
            Reliability::Reliable => zn_reliability_t::RELIABLE,
        }
    }
}

impl Into<Reliability> for zn_reliability_t {
    fn into(self) -> Reliability {
        match self {
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
    fn from(sm: SubMode) -> Self {
        match sm {
            SubMode::Push => zn_submode_t::PUSH,
            SubMode::Pull => zn_submode_t::PULL,
        }
    }
}

impl Into<SubMode> for zn_submode_t {
    fn into(self) -> SubMode {
        match self {
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
    fn from(p: Period) -> Self {
        zn_period_t {
            origin: p.origin as c_uint,
            period: p.period as c_uint,
            duration: p.duration as c_uint,
        }
    }
}

impl Into<Period> for zn_period_t {
    fn into(self) -> Period {
        Period {
            origin: self.origin as ZInt,
            period: self.period as ZInt,
            duration: self.duration as ZInt,
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

impl Into<SubInfo> for zn_subinfo_t {
    fn into(self) -> SubInfo {
        unsafe {
            SubInfo {
                reliability: self.reliability.into(),
                mode: self.mode.into(),
                period: if !self.period.is_null() {
                    Some((*Box::from_raw(self.period)).into())
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
    fn from(t: Target) -> Self {
        match t {
            Target::BestMatching => zn_target_t::BEST_MATCHING,
            Target::Complete { n } => zn_target_t::COMPLETE { n: n as c_uint },
            Target::All => zn_target_t::ALL,
            Target::None => zn_target_t::NONE,
        }
    }
}

impl Into<Target> for zn_target_t {
    fn into(self) -> Target {
        match self {
            zn_target_t::BEST_MATCHING => Target::BestMatching,
            zn_target_t::COMPLETE { n } => Target::Complete { n: n as ZInt },
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
    fn from(qt: QueryTarget) -> Self {
        zn_query_target_t {
            kind: qt.kind as c_uint,
            target: qt.target.into(),
        }
    }
}

impl Into<QueryTarget> for zn_query_target_t {
    fn into(self) -> QueryTarget {
        QueryTarget {
            kind: self.kind.into(),
            target: self.target.into(),
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
    fn from(cm: ConsolidationMode) -> Self {
        match cm {
            ConsolidationMode::Full => zn_consolidation_mode_t::FULL,
            ConsolidationMode::Lazy => zn_consolidation_mode_t::LAZY,
            ConsolidationMode::None => zn_consolidation_mode_t::NONE,
        }
    }
}

impl Into<ConsolidationMode> for zn_consolidation_mode_t {
    fn into(self) -> ConsolidationMode {
        match self {
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
    fn from(qc: QueryConsolidation) -> Self {
        zn_query_consolidation_t {
            first_routers: qc.first_routers.into(),
            last_router: qc.last_router.into(),
            reception: qc.reception.into(),
        }
    }
}

impl Into<QueryConsolidation> for zn_query_consolidation_t {
    fn into(self) -> QueryConsolidation {
        QueryConsolidation {
            first_routers: self.first_routers.into(),
            last_router: self.last_router.into(),
            reception: self.reception.into(),
        }
    }
}

/// Create a default :c:type:`zn_query_consolidation_t`.
#[no_mangle]
pub extern "C" fn zn_query_consolidation_default() -> zn_query_consolidation_t {
    QueryConsolidation::default().into()
}
