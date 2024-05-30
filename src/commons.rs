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

use std::borrow::Cow;
use std::mem::MaybeUninit;
use std::ptr::null;
use std::slice::from_raw_parts;
use std::str::from_utf8;
use std::str::FromStr;

use crate::errors;
use crate::transmute::unwrap_ref_unchecked;
use crate::transmute::Inplace;
use crate::transmute::TransmuteCopy;
use crate::transmute::TransmuteFromHandle;
use crate::transmute::TransmuteIntoHandle;
use crate::transmute::TransmuteRef;
use crate::transmute::TransmuteUninitPtr;
use crate::z_id_t;
use crate::z_loaned_bytes_t;
use crate::z_loaned_keyexpr_t;
use crate::z_owned_str_t;
use crate::z_str_from_substring;
use libc::{c_char, c_ulong};
use unwrap_infallible::UnwrapInfallible;
use zenoh::encoding::Encoding;
use zenoh::publisher::CongestionControl;
use zenoh::publisher::Priority;
use zenoh::query::ConsolidationMode;
use zenoh::query::QueryTarget;
use zenoh::query::ReplyKeyExpr;
use zenoh::sample::Locality;
use zenoh::sample::Sample;
use zenoh::sample::SampleKind;
use zenoh::time::Timestamp;
use zenoh::value::Value;
use zenoh_protocol::zenoh::Consolidation;

/// A zenoh unsigned integer
#[allow(non_camel_case_types)]
pub type z_zint_t = c_ulong;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum z_sample_kind_t {
    /// The Sample was issued by a ``put`` operation.
    PUT = 0,
    /// The Sample was issued by a ``delete`` operation.
    DELETE = 1,
}

impl From<SampleKind> for z_sample_kind_t {
    fn from(k: SampleKind) -> Self {
        match k {
            SampleKind::Put => z_sample_kind_t::PUT,
            SampleKind::Delete => z_sample_kind_t::DELETE,
        }
    }
}

impl From<z_sample_kind_t> for SampleKind {
    fn from(k: z_sample_kind_t) -> Self {
        match k {
            z_sample_kind_t::PUT => SampleKind::Put,
            z_sample_kind_t::DELETE => SampleKind::Delete,
        }
    }
}
use crate::opaque_types::z_timestamp_t;
decl_transmute_copy!(Timestamp, z_timestamp_t);

/// Returns NPT64 time associated with this timestamp.
#[no_mangle]
pub extern "C" fn z_timestamp_npt64_time(this: &z_timestamp_t) -> u64 {
    this.transmute_copy().get_time().0
}

/// Returns id associated with this timestamp.
#[no_mangle]
pub extern "C" fn z_timestamp_id(this: &z_timestamp_t) -> z_id_t {
    this.transmute_copy().get_id().to_le_bytes().into()
}

use crate::opaque_types::z_loaned_sample_t;
decl_transmute_handle!(Sample, z_loaned_sample_t);

/// Returns the key expression of the sample.
#[no_mangle]
pub extern "C" fn z_sample_keyexpr(this: &z_loaned_sample_t) -> &z_loaned_keyexpr_t {
    this.transmute_ref().key_expr().transmute_handle()
}
/// Returns the encoding associated with the sample data.
#[no_mangle]
pub extern "C" fn z_sample_encoding(this: &z_loaned_sample_t) -> &z_loaned_encoding_t {
    this.transmute_ref().encoding().transmute_handle()
}
/// Returns the sample payload data.
#[no_mangle]
pub extern "C" fn z_sample_payload(this: &z_loaned_sample_t) -> &z_loaned_bytes_t {
    this.transmute_ref().payload().transmute_handle()
}

/// Returns the sample kind.
#[no_mangle]
pub extern "C" fn z_sample_kind(this: &z_loaned_sample_t) -> z_sample_kind_t {
    this.transmute_ref().kind().into()
}
/// Returns the sample timestamp.
///
/// Will return `NULL`, if sample is not associated with a timestamp.
#[no_mangle]
pub extern "C" fn z_sample_timestamp(this: &z_loaned_sample_t) -> Option<&z_timestamp_t> {
    if let Some(t) = this.transmute_ref().timestamp() {
        Some(t.transmute_ref())
    } else {
        None
    }
}

/// Returns sample attachment.
///
/// Returns `NULL`, if sample does not contain any attachement.
#[no_mangle]
pub extern "C" fn z_sample_attachment(this: &z_loaned_sample_t) -> *const z_loaned_bytes_t {
    match this.transmute_ref().attachment() {
        Some(attachment) => attachment.transmute_handle() as *const _,
        None => null(),
    }
}

pub use crate::opaque_types::z_owned_sample_t;
decl_transmute_owned!(Option<Sample>, z_owned_sample_t);

/// Constructs an owned shallow copy of the sample (i.e. all modficiations applied to the copy, might be visible in the original) in provided uninitilized memory location.
#[no_mangle]
pub extern "C" fn z_sample_clone(
    this: &z_loaned_sample_t,
    dst: *mut MaybeUninit<z_owned_sample_t>,
) {
    let src = this.transmute_ref();
    let src = src.clone();
    let dst = dst.transmute_uninit_ptr();
    Inplace::init(dst, Some(src));
}

/// Returns sample qos priority value.
#[no_mangle]
pub extern "C" fn z_sample_priority(this: &z_loaned_sample_t) -> z_priority_t {
    this.transmute_ref().priority().into()
}

/// Returns whether sample qos express flag was set or not.
#[no_mangle]
pub extern "C" fn z_sample_express(this: &z_loaned_sample_t) -> bool {
    this.transmute_ref().express()
}

/// Returns sample qos congestion control value.
#[no_mangle]
pub extern "C" fn z_sample_congestion_control(this: &z_loaned_sample_t) -> z_congestion_control_t {
    this.transmute_ref().congestion_control().into()
}

/// Returns ``true`` if sample is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_sample_check(this: &z_owned_sample_t) -> bool {
    this.transmute_ref().is_some()
}

/// Borrows sample.
#[no_mangle]
pub extern "C" fn z_sample_loan(this: &z_owned_sample_t) -> &z_loaned_sample_t {
    unwrap_ref_unchecked(this.transmute_ref()).transmute_handle()
}

/// Frees the memory and invalidates the sample, resetting it to a gravestone state.
#[no_mangle]
pub extern "C" fn z_sample_drop(this: &mut z_owned_sample_t) {
    Inplace::drop(this.transmute_mut());
}

/// Constructs sample in its gravestone state.
#[no_mangle]
pub extern "C" fn z_sample_null(this: *mut MaybeUninit<z_owned_sample_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

validate_equivalence!(z_owned_sample_t, z_loaned_sample_t);

pub use crate::opaque_types::z_loaned_encoding_t;
decl_transmute_handle!(Encoding, z_loaned_encoding_t);
pub use crate::opaque_types::z_owned_encoding_t;
decl_transmute_owned!(Encoding, z_owned_encoding_t);

validate_equivalence!(z_owned_encoding_t, z_loaned_encoding_t);

/// Constructs a `z_owned_encoding_t` from a specified substring.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_from_substring(
    this: *mut MaybeUninit<z_owned_encoding_t>,
    s: *const c_char,
    len: usize,
) -> errors::z_error_t {
    let encoding = this.transmute_uninit_ptr();
    if s.is_null() {
        Inplace::empty(encoding);
        errors::Z_OK
    } else {
        let s = from_raw_parts(s as *const u8, len);
        match from_utf8(s) {
            Ok(s) => {
                Inplace::init(encoding, Encoding::from_str(s).unwrap_infallible());
                errors::Z_OK
            }
            Err(e) => {
                log::error!("Can not create encoding from non UTF-8 string: {}", e);
                Inplace::empty(encoding);
                errors::Z_EINVAL
            }
        }
    }
}

/// Constructs a `z_owned_encoding_t` from a specified string.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_from_str(
    this: *mut MaybeUninit<z_owned_encoding_t>,
    s: *const c_char,
) -> errors::z_error_t {
    z_encoding_from_substring(this, s, libc::strlen(s))
}

/// Constructs an owned non-null-terminated string from encoding
///
/// @param this_: Encoding.
/// @param out_str: Uninitialized memory location where a string to be constructed.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_to_string(
    this: &z_loaned_encoding_t,
    out_str: *mut MaybeUninit<z_owned_str_t>,
) {
    let s: Cow<'static, str> = this.transmute_ref().into();
    z_str_from_substring(out_str, s.as_bytes().as_ptr() as _, s.as_bytes().len());
}

/// Returns a loaned default `z_loaned_encoding_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_encoding_loan_default() -> &'static z_loaned_encoding_t {
    Encoding::ZENOH_BYTES.transmute_handle()
}

/// Constructs a default `z_owned_encoding_t`.
#[no_mangle]
pub extern "C" fn z_encoding_null(this: *mut MaybeUninit<z_owned_encoding_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Frees the memory and resets the encoding it to its default value.
#[no_mangle]
pub extern "C" fn z_encoding_drop(this: &mut z_owned_encoding_t) {
    Inplace::drop(this.transmute_mut());
}

/// Returns ``true`` if encoding is in non-default state, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_encoding_check(this: &'static z_owned_encoding_t) -> bool {
    *this.transmute_ref() != Encoding::default()
}

/// Borrows encoding.
#[no_mangle]
pub extern "C" fn z_encoding_loan(this: &z_owned_encoding_t) -> &z_loaned_encoding_t {
    this.transmute_ref().transmute_handle()
}

pub use crate::opaque_types::z_owned_value_t;
decl_transmute_owned!(Value, z_owned_value_t);
pub use crate::opaque_types::z_loaned_value_t;
decl_transmute_handle!(Value, z_loaned_value_t);

/// Constructs an empty `z_owned_value_t`.
#[no_mangle]
pub extern "C" fn z_value_null(this: *mut MaybeUninit<z_owned_value_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns ``true`` if value is in non-default state, ``false`` otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_value_check(this: &'static z_owned_value_t) -> bool {
    !this.transmute_ref().is_empty()
}

/// Returns value payload.
#[no_mangle]
pub extern "C" fn z_value_payload(this: &z_loaned_value_t) -> &z_loaned_bytes_t {
    this.transmute_ref().payload().transmute_handle()
}

/// Returns value encoding.
#[no_mangle]
pub extern "C" fn z_value_encoding(this: &z_loaned_value_t) -> &z_loaned_encoding_t {
    this.transmute_ref().encoding().transmute_handle()
}

/// Borrows value.
#[no_mangle]
pub extern "C" fn z_value_loan(this: &z_owned_value_t) -> &z_loaned_value_t {
    this.transmute_ref().transmute_handle()
}

/// Frees the memory and resets the value it to its default value.
#[no_mangle]
pub extern "C" fn z_value_drop(this: &mut z_owned_value_t) {
    Inplace::drop(this.transmute_mut());
}

/// The locality of samples to be received by subscribers or targeted by publishers.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum zcu_locality_t {
    /// Any
    ANY = 0,
    /// Only from local sessions.
    SESSION_LOCAL = 1,
    /// Only from remote sessions.
    REMOTE = 2,
}

impl From<Locality> for zcu_locality_t {
    fn from(k: Locality) -> Self {
        match k {
            Locality::Any => zcu_locality_t::ANY,
            Locality::SessionLocal => zcu_locality_t::SESSION_LOCAL,
            Locality::Remote => zcu_locality_t::REMOTE,
        }
    }
}

impl From<zcu_locality_t> for Locality {
    fn from(k: zcu_locality_t) -> Self {
        match k {
            zcu_locality_t::ANY => Locality::Any,
            zcu_locality_t::SESSION_LOCAL => Locality::SessionLocal,
            zcu_locality_t::REMOTE => Locality::Remote,
        }
    }
}

/// Returns default value of `zcu_locality_t`
#[no_mangle]
pub extern "C" fn zcu_locality_default() -> zcu_locality_t {
    Locality::default().into()
}

/// Key expressions types to which Queryable should reply to.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum zcu_reply_keyexpr_t {
    /// Replies to any key expression queries.
    ANY = 0,
    /// Replies only to queries with intersecting key expressions.
    MATCHING_QUERY = 1,
}

impl From<ReplyKeyExpr> for zcu_reply_keyexpr_t {
    fn from(k: ReplyKeyExpr) -> Self {
        match k {
            ReplyKeyExpr::Any => zcu_reply_keyexpr_t::ANY,
            ReplyKeyExpr::MatchingQuery => zcu_reply_keyexpr_t::MATCHING_QUERY,
        }
    }
}

impl From<zcu_reply_keyexpr_t> for ReplyKeyExpr {
    fn from(k: zcu_reply_keyexpr_t) -> Self {
        match k {
            zcu_reply_keyexpr_t::ANY => ReplyKeyExpr::Any,
            zcu_reply_keyexpr_t::MATCHING_QUERY => ReplyKeyExpr::MatchingQuery,
        }
    }
}

/// Returns the default value of #zcu_reply_keyexpr_t.
#[no_mangle]
pub extern "C" fn zcu_reply_keyexpr_default() -> zcu_reply_keyexpr_t {
    ReplyKeyExpr::default().into()
}

/// The Queryables that should be target of a `z_get()`.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_query_target_t {
    /// The nearest complete queryable if any else all matching queryables.
    BEST_MATCHING,
    /// All matching queryables.
    ALL,
    /// All complete queryables.
    ALL_COMPLETE,
}

impl From<QueryTarget> for z_query_target_t {
    #[inline]
    fn from(t: QueryTarget) -> Self {
        match t {
            QueryTarget::BestMatching => z_query_target_t::BEST_MATCHING,
            QueryTarget::All => z_query_target_t::ALL,
            QueryTarget::AllComplete => z_query_target_t::ALL_COMPLETE,
        }
    }
}

impl From<z_query_target_t> for QueryTarget {
    #[inline]
    fn from(val: z_query_target_t) -> Self {
        match val {
            z_query_target_t::BEST_MATCHING => QueryTarget::BestMatching,
            z_query_target_t::ALL => QueryTarget::All,
            z_query_target_t::ALL_COMPLETE => QueryTarget::AllComplete,
        }
    }
}

/// Create a default `z_query_target_t`.
#[no_mangle]
pub extern "C" fn z_query_target_default() -> z_query_target_t {
    QueryTarget::default().into()
}

/// Consolidation mode values.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub enum z_consolidation_mode_t {
    /// Let Zenoh decide the best consolidation mode depending on the query selector.
    /// If the selector contains time range properties, consolidation mode `NONE` is used.
    /// Otherwise the `LATEST` consolidation mode is used.
    AUTO = -1,
    #[default]
    ///  No consolidation is applied. Replies may come in any order and any number.
    NONE = 0,
    /// It guarantees that any reply for a given key expression will be monotonic in time
    /// w.r.t. the previous received replies for the same key expression. I.e., for the same key expression multiple
    /// replies may be received. It is guaranteed that two replies received at t1 and t2 will have timestamp
    /// ts2 > ts1. It optimizes latency.
    MONOTONIC = 1,
    /// It guarantees unicity of replies for the same key expression.
    /// It optimizes bandwidth.
    LATEST = 2,
}

impl From<ConsolidationMode> for z_consolidation_mode_t {
    #[inline]
    fn from(cm: ConsolidationMode) -> Self {
        match cm {
            ConsolidationMode::Auto => z_consolidation_mode_t::AUTO,
            ConsolidationMode::None => z_consolidation_mode_t::NONE,
            ConsolidationMode::Monotonic => z_consolidation_mode_t::MONOTONIC,
            ConsolidationMode::Latest => z_consolidation_mode_t::LATEST,
        }
    }
}

impl From<z_consolidation_mode_t> for ConsolidationMode {
    #[inline]
    fn from(val: z_consolidation_mode_t) -> Self {
        match val {
            z_consolidation_mode_t::AUTO => Consolidation::Auto,
            z_consolidation_mode_t::NONE => ConsolidationMode::None,
            z_consolidation_mode_t::MONOTONIC => ConsolidationMode::Monotonic,
            z_consolidation_mode_t::LATEST => ConsolidationMode::Latest,
        }
    }
}

/// The priority of zenoh messages.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_priority_t {
    /// Priority for ``RealTime`` messages.
    REAL_TIME = 1,
    /// Highest priority for ``Interactive`` messages.
    INTERACTIVE_HIGH = 2,
    /// Lowest priority for ``Interactive`` messages.
    INTERACTIVE_LOW = 3,
    /// Highest priority for ``Data`` messages.
    DATA_HIGH = 4,
    /// Default priority for ``Data`` messages.
    DATA = 5,
    /// Lowest priority for ``Data`` messages.
    DATA_LOW = 6,
    /// Priority for ``Background traffic`` messages.
    BACKGROUND = 7,
}

impl From<Priority> for z_priority_t {
    fn from(p: Priority) -> Self {
        match p {
            Priority::RealTime => z_priority_t::REAL_TIME,
            Priority::InteractiveHigh => z_priority_t::INTERACTIVE_HIGH,
            Priority::InteractiveLow => z_priority_t::INTERACTIVE_LOW,
            Priority::DataHigh => z_priority_t::DATA_HIGH,
            Priority::Data => z_priority_t::DATA,
            Priority::DataLow => z_priority_t::DATA_LOW,
            Priority::Background => z_priority_t::BACKGROUND,
        }
    }
}

impl From<z_priority_t> for Priority {
    fn from(p: z_priority_t) -> Self {
        match p {
            z_priority_t::REAL_TIME => Priority::RealTime,
            z_priority_t::INTERACTIVE_HIGH => Priority::InteractiveHigh,
            z_priority_t::INTERACTIVE_LOW => Priority::InteractiveLow,
            z_priority_t::DATA_HIGH => Priority::DataHigh,
            z_priority_t::DATA => Priority::Data,
            z_priority_t::DATA_LOW => Priority::DataLow,
            z_priority_t::BACKGROUND => Priority::Background,
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_congestion_control_t {
    /// Messages are not dropped in case of congestion.
    BLOCK,
    /// Messages are dropped in case of congestion.
    DROP,
}

impl From<CongestionControl> for z_congestion_control_t {
    fn from(cc: CongestionControl) -> Self {
        match cc {
            CongestionControl::Block => z_congestion_control_t::BLOCK,
            CongestionControl::Drop => z_congestion_control_t::DROP,
        }
    }
}

impl From<z_congestion_control_t> for CongestionControl {
    fn from(cc: z_congestion_control_t) -> Self {
        match cc {
            z_congestion_control_t::BLOCK => CongestionControl::Block,
            z_congestion_control_t::DROP => CongestionControl::Drop,
        }
    }
}
