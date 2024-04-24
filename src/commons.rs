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

use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::ptr::null;
use std::str::FromStr;

use crate::transmute::unwrap_ref_unchecked;
use crate::transmute::Inplace;
use crate::transmute::TransmuteCopy;
use crate::transmute::TransmuteFromHandle;
use crate::transmute::TransmuteIntoHandle;
use crate::transmute::TransmuteRef;
use crate::transmute::TransmuteUninitPtr;
use crate::z_bytes_t;
use crate::z_id_t;
use crate::z_keyexpr_t;
use libc::{c_char, c_ulong};
use unwrap_infallible::UnwrapInfallible;
use zenoh::encoding::Encoding;
use zenoh::prelude::SampleKind;
use zenoh::publication::CongestionControl;
use zenoh::publication::Priority;
use zenoh::query::ConsolidationMode;
use zenoh::query::Mode;
use zenoh::query::QueryTarget;
use zenoh::query::ReplyKeyExpr;
use zenoh::sample::Locality;
use zenoh::sample::Sample;
use zenoh::time::Timestamp;
use zenoh::value::Value;

/// A zenoh unsigned integer
#[allow(non_camel_case_types)]
pub type z_zint_t = c_ulong;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum z_sample_kind_t {
    PUT = 0,
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

#[no_mangle]
pub extern "C" fn z_timestamp_npt64_time(timestamp: &z_timestamp_t) -> u64 {
    timestamp.transmute_copy().get_time().0
}

#[no_mangle]
pub extern "C" fn z_timestamp_get_id(timestamp: &z_timestamp_t) -> z_id_t {
    timestamp.transmute_copy().get_id().to_le_bytes().into()
}

/// A data sample.
///
/// A sample is the value associated to a given resource at a given point in time.
use crate::opaque_types::z_sample_t;
decl_transmute_handle!(Sample, z_sample_t);

/// The Key Expression of the sample.
///
/// `sample` is aliased by its return value.
#[no_mangle]
pub extern "C" fn z_sample_keyexpr(sample: &z_sample_t) -> &z_keyexpr_t {
    let sample = sample.transmute_ref();
    sample.key_expr().transmute_handle()
}
/// The encoding of the payload.
#[no_mangle]
pub extern "C" fn z_sample_encoding(sample: &z_sample_t) -> &z_encoding_t {
    let sample = sample.transmute_ref();
    sample.encoding().transmute_handle()
}
/// The sample's data, the return value aliases the sample.
///
#[no_mangle]
pub extern "C" fn z_sample_payload(sample: &z_sample_t) -> &z_bytes_t {
    let sample = sample.transmute_ref();
    sample.payload().transmute_handle()
}

/// The sample's kind (put or delete).
#[no_mangle]
pub extern "C" fn z_sample_kind(sample: &z_sample_t) -> z_sample_kind_t {
    let sample = sample.transmute_ref();
    sample.kind().into()
}
/// The samples timestamp
///
/// Returns true if Sample contains timestamp, false otherwise. In the latter case the timestamp_out value is not altered.
#[no_mangle]
pub extern "C" fn z_sample_timestamp(
    sample: &z_sample_t,
    timestamp_out: &mut z_timestamp_t,
) -> bool {
    let sample = sample.transmute_ref();
    if let Some(t) = sample.timestamp() {
        *timestamp_out = t.transmute_copy();
        true
    } else {
        false
    }
}
/// The qos with which the sample was received.
/// TODO: split to methods (priority, congestion_control, express)


/// Gets sample's attachment.
///
/// Returns NULL if sample does not contain an attachement.
#[no_mangle]
pub extern "C" fn z_sample_attachment(sample: &z_sample_t) -> *const z_bytes_t {
    let sample = sample.transmute_ref();
    match sample.attachment() {
        Some(attachment) => attachment.transmute_handle() as *const _,
        None => null(),
    }

}

pub use crate::opaque_types::zc_owned_sample_t;
decl_transmute_owned!(Option<Sample>, zc_owned_sample_t);

/// Clone a sample in the cheapest way available.
#[no_mangle]
pub extern "C" fn z_sample_clone(src: &z_sample_t, dst: *mut MaybeUninit<zc_owned_sample_t>) {
    let src = src.transmute_ref();
    let src = src.clone();
    let dst = dst.transmute_uninit_ptr();
    Inplace::init(dst, Some(src));
}


#[no_mangle]
pub extern "C" fn z_sample_priority(sample: &z_sample_t) -> z_priority_t {
    let sample = sample.transmute_ref();
    sample.priority().into()
}

#[no_mangle]
pub extern "C" fn z_sample_express(sample: &z_sample_t) -> bool {
    let sample = sample.transmute_ref();
    sample.express()
}

#[no_mangle]
pub extern "C" fn z_sample_congestion_control(sample: &z_sample_t) -> z_congestion_control_t {
    let sample = sample.transmute_ref();
    sample.congestion_control().into()
}

/// Returns `true` if `sample` is valid.
///
/// Note that there exist no fallinle constructors for `zc_owned_sample_t`, so validity is always guaranteed
/// unless the value has been dropped already.
#[no_mangle]
pub extern "C" fn z_sample_check(sample: &zc_owned_sample_t) -> bool {
    let sample = sample.transmute_ref();
    sample.is_some()
}

/// Borrow the sample, allowing calling its accessor methods.
///
/// Calling this function using a dropped sample is undefined behaviour.
#[no_mangle]
pub extern "C" fn zc_sample_loan(sample: & zc_owned_sample_t) -> &z_sample_t {
    unwrap_ref_unchecked(sample.transmute_ref()).transmute_handle()
}

/// Destroy the sample.
#[no_mangle]
pub extern "C" fn zc_sample_drop(sample: &mut zc_owned_sample_t) {
    Inplace::drop(sample.transmute_mut());
}

#[no_mangle]
pub extern "C" fn zc_sample_null(sample: *mut MaybeUninit<zc_owned_sample_t>) {
    Inplace::empty(sample.transmute_uninit_ptr());
}

pub use crate::opaque_types::z_encoding_t;
decl_transmute_handle!(Encoding, z_encoding_t);

/// An owned payload encoding.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
pub use crate::opaque_types::z_owned_encoding_t;
decl_transmute_owned!(Encoding, z_owned_encoding_t);

/// Constructs a null safe-to-drop value of 'z_owned_encoding_t' type
#[no_mangle]
pub extern "C" fn z_encoding_null(encoding: *mut MaybeUninit<z_owned_encoding_t>) {
    Inplace::empty(encoding.transmute_uninit_ptr());
}

/// Constructs a specific :c:type:`z_encoding_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_from_str(
    encoding: *mut MaybeUninit<z_owned_encoding_t>,
    s: *const c_char,
) -> i8 {
    let encoding = encoding.transmute_uninit_ptr();
    if s.is_null() {
        Inplace::empty(encoding);
        0
    } else {
        let s = CStr::from_ptr(s).to_string_lossy();
        let value = Encoding::from_str(s.as_ref()).unwrap_infallible();
        Inplace::init(encoding, value);
        0
    }
}

/// Constructs a default :c:type:`z_encoding_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_encoding_default() -> &'static z_encoding_t {
    Encoding::ZENOH_BYTES.transmute_handle()
}

/// Frees `encoding`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_encoding_drop(encoding: &mut z_owned_encoding_t) {
    Inplace::drop(encoding.transmute_mut());
}

/// Returns ``true`` if `encoding` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_encoding_check(encoding: &'static z_owned_encoding_t) -> bool {
    *encoding.transmute_ref() != Encoding::default()
}

/// Returns a :c:type:`z_encoding_t` loaned from `encoding`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_encoding_loan(encoding: &z_owned_encoding_t) -> &z_encoding_t {
    encoding.transmute_ref().transmute_handle()
}

pub use crate::opaque_types::z_owned_value_t;
decl_transmute_owned!(Value, z_owned_value_t);
pub use crate::opaque_types::z_value_t;
decl_transmute_handle!(Value, z_value_t);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum zcu_locality_t {
    ANY = 0,
    SESSION_LOCAL = 1,
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

#[no_mangle]
pub extern "C" fn zcu_locality_default() -> zcu_locality_t {
    Locality::default().into()
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum zcu_reply_keyexpr_t {
    ANY = 0,
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

#[no_mangle]
pub extern "C" fn zcu_reply_keyexpr_default() -> zcu_reply_keyexpr_t {
    ReplyKeyExpr::default().into()
}

/// The Queryables that should be target of a :c:func:`z_get`.
///
///     - **BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
///     - **ALL_COMPLETE**: All complete queryables.
///     - **ALL**: All matching queryables.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_query_target_t {
    BEST_MATCHING,
    ALL,
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

/// Create a default :c:type:`z_query_target_t`.
#[no_mangle]
pub extern "C" fn z_query_target_default() -> z_query_target_t {
    QueryTarget::default().into()
}

/// Consolidation mode values.
///
///     - **Z_CONSOLIDATION_MODE_AUTO**: Let Zenoh decide the best consolidation mode depending on the query selector
///       If the selector contains time range properties, consolidation mode `NONE` is used.
///       Otherwise the `LATEST` consolidation mode is used.
///     - **Z_CONSOLIDATION_MODE_NONE**: No consolidation is applied. Replies may come in any order and any number.
///     - **Z_CONSOLIDATION_MODE_MONOTONIC**: It guarantees that any reply for a given key expression will be monotonic in time
///       w.r.t. the previous received replies for the same key expression. I.e., for the same key expression multiple
///       replies may be received. It is guaranteed that two replies received at t1 and t2 will have timestamp
///       ts2 > ts1. It optimizes latency.
///     - **Z_CONSOLIDATION_MODE_LATEST**: It guarantees unicity of replies for the same key expression.
///       It optimizes bandwidth.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub enum z_consolidation_mode_t {
    AUTO = -1,
    #[default]
    NONE = 0,
    MONOTONIC = 1,
    LATEST = 2,
}

impl From<Mode<ConsolidationMode>> for z_consolidation_mode_t {
    #[inline]
    fn from(cm: Mode<ConsolidationMode>) -> Self {
        match cm {
            Mode::Manual(cm) => Self::from(cm),
            Mode::Auto => z_consolidation_mode_t::AUTO,
        }
    }
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

impl From<z_consolidation_mode_t> for Mode<ConsolidationMode> {
    #[inline]
    fn from(val: z_consolidation_mode_t) -> Self {
        match val {
            z_consolidation_mode_t::AUTO => Mode::Auto,
            z_consolidation_mode_t::NONE => Mode::Manual(ConsolidationMode::None),
            z_consolidation_mode_t::MONOTONIC => Mode::Manual(ConsolidationMode::Monotonic),
            z_consolidation_mode_t::LATEST => Mode::Manual(ConsolidationMode::Latest),
        }
    }
}

/// The priority of zenoh messages.
///
///     - **REAL_TIME**
///     - **INTERACTIVE_HIGH**
///     - **INTERACTIVE_LOW**
///     - **DATA_HIGH**
///     - **DATA**
///     - **DATA_LOW**
///     - **BACKGROUND**
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_priority_t {
    REAL_TIME = 1,
    INTERACTIVE_HIGH = 2,
    INTERACTIVE_LOW = 3,
    DATA_HIGH = 4,
    DATA = 5,
    DATA_LOW = 6,
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

/// The kind of congestion control.
///
///     - **BLOCK**
///     - **DROP**
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_congestion_control_t {
    BLOCK,
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
