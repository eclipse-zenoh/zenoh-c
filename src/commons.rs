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

use std::{mem::MaybeUninit, ptr::null};

use libc::c_ulong;
#[cfg(feature = "unstable")]
use zenoh::{
    qos::Reliability,
    query::ReplyKeyExpr,
    sample::{Locality, SourceInfo},
    session::EntityGlobalId,
};
use zenoh::{
    qos::{CongestionControl, Priority},
    query::{ConsolidationMode, QueryTarget},
    sample::{Sample, SampleKind},
    time::Timestamp,
};

#[cfg(feature = "unstable")]
use crate::transmute::IntoCType;
#[cfg(feature = "unstable")]
use crate::z_moved_source_info_t;
use crate::{
    result,
    transmute::{
        CTypeRef, LoanedCTypeMut, LoanedCTypeRef, RustTypeMut, RustTypeMutUninit, RustTypeRef,
        TakeRustType,
    },
    z_id_t, z_loaned_bytes_t, z_loaned_encoding_t, z_loaned_keyexpr_t, z_loaned_session_t,
};

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
decl_c_type!(copy(z_timestamp_t, Timestamp));

/// Create uhlc timestamp from session id.
#[no_mangle]
pub extern "C" fn z_timestamp_new(
    this: &mut MaybeUninit<z_timestamp_t>,
    session: &z_loaned_session_t,
) -> result::z_result_t {
    let timestamp = session.as_rust_type_ref().new_timestamp();
    this.as_rust_type_mut_uninit().write(timestamp);
    result::Z_OK
}

/// Returns NPT64 time associated with this timestamp.
#[no_mangle]
pub extern "C" fn z_timestamp_ntp64_time(this_: &z_timestamp_t) -> u64 {
    this_.as_rust_type_ref().get_time().0
}

/// @brief Returns id associated with this timestamp.
#[no_mangle]
pub extern "C" fn z_timestamp_id(this_: &z_timestamp_t) -> z_id_t {
    this_.as_rust_type_ref().get_id().to_le_bytes().into()
}

use crate::opaque_types::z_loaned_sample_t;
pub use crate::opaque_types::{z_moved_sample_t, z_owned_sample_t};
decl_c_type!(
    owned(z_owned_sample_t, option Sample),
    loaned(z_loaned_sample_t),
);

/// Returns the key expression of the sample.
#[no_mangle]
pub extern "C" fn z_sample_keyexpr(this_: &z_loaned_sample_t) -> &z_loaned_keyexpr_t {
    this_.as_rust_type_ref().key_expr().as_loaned_c_type_ref()
}
/// Returns the encoding associated with the sample data.
#[no_mangle]
pub extern "C" fn z_sample_encoding(this_: &z_loaned_sample_t) -> &z_loaned_encoding_t {
    this_.as_rust_type_ref().encoding().as_loaned_c_type_ref()
}
/// Returns the sample payload data.
#[no_mangle]
pub extern "C" fn z_sample_payload(this_: &z_loaned_sample_t) -> &z_loaned_bytes_t {
    this_.as_rust_type_ref().payload().as_loaned_c_type_ref()
}

/// Returns the sample kind.
#[no_mangle]
pub extern "C" fn z_sample_kind(this_: &z_loaned_sample_t) -> z_sample_kind_t {
    this_.as_rust_type_ref().kind().into()
}
/// Returns the sample timestamp.
///
/// Will return `NULL`, if sample is not associated with a timestamp.
#[no_mangle]
pub extern "C" fn z_sample_timestamp(this_: &z_loaned_sample_t) -> Option<&z_timestamp_t> {
    if let Some(t) = this_.as_rust_type_ref().timestamp() {
        Some(t.as_ctype_ref())
    } else {
        None
    }
}

/// Returns sample attachment.
///
/// Returns `NULL`, if sample does not contain any attachment.
#[no_mangle]
pub extern "C" fn z_sample_attachment(this_: &z_loaned_sample_t) -> *const z_loaned_bytes_t {
    match this_.as_rust_type_ref().attachment() {
        Some(attachment) => attachment.as_loaned_c_type_ref() as *const _,
        None => null(),
    }
}
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the sample source_info.
#[no_mangle]
pub extern "C" fn z_sample_source_info(this_: &z_loaned_sample_t) -> &z_loaned_source_info_t {
    this_
        .as_rust_type_ref()
        .source_info()
        .as_loaned_c_type_ref()
}

/// Constructs an owned shallow copy of the sample (i.e. all modficiations applied to the copy, might be visible in the original) in provided uninitilized memory location.
#[no_mangle]
pub extern "C" fn z_sample_clone(
    dst: &mut MaybeUninit<z_owned_sample_t>,
    this: &z_loaned_sample_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(Some(this.as_rust_type_ref().clone()));
}

/// Returns sample qos priority value.
#[no_mangle]
pub extern "C" fn z_sample_priority(this_: &z_loaned_sample_t) -> z_priority_t {
    this_.as_rust_type_ref().priority().into()
}

/// Returns whether sample qos express flag was set or not.
#[no_mangle]
pub extern "C" fn z_sample_express(this_: &z_loaned_sample_t) -> bool {
    this_.as_rust_type_ref().express()
}

/// Returns sample qos congestion control value.
#[no_mangle]
pub extern "C" fn z_sample_congestion_control(this_: &z_loaned_sample_t) -> z_congestion_control_t {
    this_.as_rust_type_ref().congestion_control().into()
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the reliability setting the sample was delivered with.
#[no_mangle]
pub extern "C" fn z_sample_reliability(this_: &z_loaned_sample_t) -> z_reliability_t {
    this_.as_rust_type_ref().reliability().into()
}

/// Returns ``true`` if sample is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_sample_check(this_: &z_owned_sample_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Borrows sample.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_sample_loan(this_: &z_owned_sample_t) -> &z_loaned_sample_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Mutably borrows sample.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_sample_loan_mut(this_: &mut z_owned_sample_t) -> &mut z_loaned_sample_t {
    this_.as_rust_type_mut().as_loaned_c_type_mut()
}

/// Takes ownership of the mutably borrowed sample.
#[no_mangle]
pub extern "C" fn z_sample_take_loaned(
    dst: &mut MaybeUninit<z_owned_sample_t>,
    src: &mut z_loaned_sample_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(std::mem::take(src.as_rust_type_mut()));
}

/// Frees the memory and invalidates the sample, resetting it to a gravestone state.
#[no_mangle]
pub extern "C" fn z_sample_drop(this_: &mut z_moved_sample_t) {
    let _ = this_.take_rust_type();
}

/// Constructs sample in its gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_sample_null(this_: &mut MaybeUninit<z_owned_sample_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// The locality of samples to be received by subscribers or targeted by publishers.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum zc_locality_t {
    /// Any
    ANY = 0,
    /// Only from local sessions.
    SESSION_LOCAL = 1,
    /// Only from remote sessions.
    REMOTE = 2,
}

#[cfg(feature = "unstable")]
impl From<Locality> for zc_locality_t {
    fn from(k: Locality) -> Self {
        match k {
            Locality::Any => zc_locality_t::ANY,
            Locality::SessionLocal => zc_locality_t::SESSION_LOCAL,
            Locality::Remote => zc_locality_t::REMOTE,
        }
    }
}

#[cfg(feature = "unstable")]
impl From<zc_locality_t> for Locality {
    fn from(k: zc_locality_t) -> Self {
        match k {
            zc_locality_t::ANY => Locality::Any,
            zc_locality_t::SESSION_LOCAL => Locality::SessionLocal,
            zc_locality_t::REMOTE => Locality::Remote,
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns default value of `zc_locality_t`
#[no_mangle]
pub extern "C" fn zc_locality_default() -> zc_locality_t {
    Locality::default().into()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief The publisher reliability.
/// @note Currently `reliability` does not trigger any data retransmission on the wire.
/// It is rather used as a marker on the wire and it may be used to select the best link available (e.g. TCP for reliable data and UDP for best effort data).
#[cfg(feature = "unstable")]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_reliability_t {
    /// Defines reliability as ``BEST_EFFORT``
    BEST_EFFORT,
    /// Defines reliability as ``RELIABLE``
    RELIABLE,
}

#[cfg(feature = "unstable")]
impl From<Reliability> for z_reliability_t {
    #[inline]
    fn from(r: Reliability) -> Self {
        match r {
            Reliability::BestEffort => z_reliability_t::BEST_EFFORT,
            Reliability::Reliable => z_reliability_t::RELIABLE,
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the default value for `reliability`.
#[no_mangle]
pub extern "C" fn z_reliability_default() -> z_reliability_t {
    Reliability::default().into()
}

#[cfg(feature = "unstable")]
impl From<z_reliability_t> for Reliability {
    #[inline]
    fn from(val: z_reliability_t) -> Self {
        match val {
            z_reliability_t::BEST_EFFORT => Reliability::BestEffort,
            z_reliability_t::RELIABLE => Reliability::Reliable,
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Key expressions types to which Queryable should reply to.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum zc_reply_keyexpr_t {
    /// Replies to any key expression queries.
    ANY = 0,
    /// Replies only to queries with intersecting key expressions.
    MATCHING_QUERY = 1,
}

#[cfg(feature = "unstable")]
impl From<ReplyKeyExpr> for zc_reply_keyexpr_t {
    fn from(k: ReplyKeyExpr) -> Self {
        match k {
            ReplyKeyExpr::Any => zc_reply_keyexpr_t::ANY,
            ReplyKeyExpr::MatchingQuery => zc_reply_keyexpr_t::MATCHING_QUERY,
        }
    }
}

#[cfg(feature = "unstable")]
impl From<zc_reply_keyexpr_t> for ReplyKeyExpr {
    fn from(k: zc_reply_keyexpr_t) -> Self {
        match k {
            zc_reply_keyexpr_t::ANY => ReplyKeyExpr::Any,
            zc_reply_keyexpr_t::MATCHING_QUERY => ReplyKeyExpr::MatchingQuery,
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the default value of #zc_reply_keyexpr_t.
#[no_mangle]
pub extern "C" fn zc_reply_keyexpr_default() -> zc_reply_keyexpr_t {
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
            z_consolidation_mode_t::AUTO => ConsolidationMode::Auto,
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

/// Returns the default value of #z_priority_t.
#[no_mangle]
pub extern "C" fn z_priority_default() -> z_priority_t {
    Priority::default().into()
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

#[cfg(feature = "unstable")]
use crate::z_entity_global_id_t;
#[cfg(feature = "unstable")]
decl_c_type!(copy(z_entity_global_id_t, EntityGlobalId));

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the zenoh id of entity global id.
#[no_mangle]
pub extern "C" fn z_entity_global_id_zid(this_: &z_entity_global_id_t) -> z_id_t {
    this_.as_rust_type_ref().zid().into_c_type()
}
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the entity id of the entity global id.
#[no_mangle]
pub extern "C" fn z_entity_global_id_eid(this_: &z_entity_global_id_t) -> u32 {
    this_.as_rust_type_ref().eid()
}
#[cfg(feature = "unstable")]
pub use crate::opaque_types::{z_loaned_source_info_t, z_owned_source_info_t};
#[cfg(feature = "unstable")]
decl_c_type!(
    owned(z_owned_source_info_t, SourceInfo),
    loaned(z_loaned_source_info_t, SourceInfo),
);

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Creates source info.
#[no_mangle]
pub extern "C" fn z_source_info_new(
    this: &mut MaybeUninit<z_owned_source_info_t>,
    source_id: &z_entity_global_id_t,
    source_sn: u32,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let source_info = SourceInfo::new(Some(*source_id.as_rust_type_ref()), Some(source_sn));
    this.write(source_info);
    result::Z_OK
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the source_id of the source info.
#[no_mangle]
pub extern "C" fn z_source_info_id(this_: &z_loaned_source_info_t) -> z_entity_global_id_t {
    match this_.as_rust_type_ref().source_id() {
        Some(source_id) => *source_id,
        None => EntityGlobalId::default(),
    }
    .into_c_type()
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the source_sn of the source info.
#[no_mangle]
pub extern "C" fn z_source_info_sn(this_: &z_loaned_source_info_t) -> u32 {
    this_.as_rust_type_ref().source_sn().unwrap_or_default()
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if source info is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_source_info_check(this_: &z_owned_source_info_t) -> bool {
    this_.as_rust_type_ref().source_id().is_some() || this_.as_rust_type_ref().source_sn().is_some()
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows source info.
#[no_mangle]
pub extern "C" fn z_source_info_loan(this_: &z_owned_source_info_t) -> &z_loaned_source_info_t {
    this_.as_rust_type_ref().as_loaned_c_type_ref()
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows source info.
#[no_mangle]
pub extern "C" fn z_source_info_loan_mut(this_: &z_owned_source_info_t) -> &z_loaned_source_info_t {
    this_.as_rust_type_mut().as_loaned_c_type_mut()
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Takes ownership of the mutably borrowed source info.
#[no_mangle]
pub extern "C" fn z_source_info_take_loaned(
    dst: &mut MaybeUninit<z_owned_source_info_t>,
    src: &mut z_loaned_source_info_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(std::mem::take(src.as_rust_type_mut()));
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Frees the memory and invalidates the source info, resetting it to a gravestone state.
#[no_mangle]
pub extern "C" fn z_source_info_drop(this_: &mut z_moved_source_info_t) {
    let _ = this_.take_rust_type();
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs source info in its gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_source_info_null(this_: &mut MaybeUninit<z_owned_source_info_t>) {
    this_.as_rust_type_mut_uninit().write(SourceInfo::default());
}
