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

use libc::c_char;
use std::{
    borrow::Cow,
    convert::TryFrom,
    ffi::CStr,
    ops::{Deref, DerefMut},
};
use zenoh_protocol_core::{ConsolidationMode, QueryTarget};

use zenoh::{
    prelude::{KeyExpr, SplitBuffer},
    query::{Mode, QueryConsolidation, Reply},
    value::Value,
};
use zenoh_util::core::{zresult::ErrNo, SyncResolve};

use crate::{
    _zc_stack_ke, z_bytes_t, z_closure_reply_call, z_encoding_default, z_encoding_t, z_keyexpr_t,
    z_owned_closure_reply_t, z_sample_t, z_session_t, LOG_INVALID_SESSION,
};

type ReplyInner = Option<Reply>;

#[allow(non_camel_case_types)]
pub type _z_u128 = u128;
/// An owned reply to a :c:func:`z_get`.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[repr(C)]
pub struct z_owned_reply_t {
    _1: _zc_res_s_v,
    _2: _z_u128,
}
#[repr(C)]
pub struct _zc_res_s_v {
    _0: u8,
    _1: [_z_u128; 2],
    _2: _zc_stack_ke,
    _3: [usize; 11],
    _4: u64,
}

impl From<ReplyInner> for z_owned_reply_t {
    fn from(mut val: ReplyInner) -> Self {
        if let Some(val) = &mut val {
            match &mut val.sample {
                Ok(inner) => inner.payload = inner.payload.contiguous().into_owned().into(),
                Err(inner) => inner.payload = inner.payload.contiguous().into_owned().into(),
            };
        }
        unsafe { std::mem::transmute(val) }
    }
}
impl From<Reply> for z_owned_reply_t {
    fn from(val: Reply) -> z_owned_reply_t {
        Some(val).into()
    }
}
impl Deref for z_owned_reply_t {
    type Target = ReplyInner;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
    }
}
impl DerefMut for z_owned_reply_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
    }
}
/// Returns ``true`` if the queryable answered with an OK, which allows this value to be treated as a sample.
///
/// If this returns ``false``, you should use :c:func:`z_check` before trying to use :c:func:`z_reply_err` if you want to process the error that may be here.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_is_ok(reply: &z_owned_reply_t) -> bool {
    reply.as_ref().map(|r| r.sample.is_ok()).unwrap_or(false)
}

/// Yields the contents of the reply by asserting it indicates a success.
///
/// You should always make sure that :c:func:`z_reply_is_ok` returns ``true`` before calling this function.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_ok(reply: &z_owned_reply_t) -> z_sample_t {
    if let Some(inner) = reply.as_ref().and_then(|s| s.sample.as_ref().ok()) {
        z_sample_t {
            keyexpr: inner.key_expr.borrowing_clone().into(),
            payload: match &inner.payload.contiguous() {
                Cow::Borrowed(payload) => crate::z_bytes_t { start: payload.as_ptr(), len: payload.len() },
                Cow::Owned(_) => unreachable!("z_reply_ok found a payload that wasn't contiguous by the time it was reached, which breaks some crate assertions. This is definitely a bug with zenoh, please contact us."),
            },
            encoding: (&inner.encoding).into(),
            kind: inner.kind.into(),
            timestamp: inner.timestamp.as_ref().into()
        }
    } else {
        panic!("Assertion failed: tried to treat `z_owned_reply_t` as Ok despite that not being the case")
    }
}

/// A zenoh value.
///
/// Members:
///   z_bytes_t payload: The payload of this zenoh value.
///   z_encoding_t encoding: The encoding of this zenoh value `payload`.
#[repr(C)]
pub struct z_value_t {
    pub payload: z_bytes_t,
    pub encoding: z_encoding_t,
}

/// Yields the contents of the reply by asserting it indicates a failure.
///
/// You should always make sure that :c:func:`z_reply_is_ok` returns ``false`` before calling this function.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_err(reply: &z_owned_reply_t) -> z_value_t {
    if let Some(inner) = reply.as_ref().and_then(|s| s.sample.as_ref().err()) {
        z_value_t {
            payload: match &inner.payload.contiguous() {
                Cow::Borrowed(payload) => crate::z_bytes_t { start: payload.as_ptr(), len: payload.len() },
                Cow::Owned(_) => unreachable!("z_reply_err found a payload that wasn't contiguous by the time it was reached, which breaks some crate assertions."),
            },
            encoding: (&inner.encoding).into(),
        }
    } else {
        panic!("Assertion failed: tried to treat `z_owned_reply_t` as Err despite that not being the case")
    }
}

/// Returns an invalidated :c:type:`z_owned_reply_t`.
///
/// This is useful when you wish to take ownership of a value from a callback to :c:func:`z_get`:
///
///     - copy the value of the callback's argument's pointee,
///     - overwrite the pointee with this function's return value,
///     - you are now responsible for dropping your copy of the reply.
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn z_reply_null() -> z_owned_reply_t {
    None.into()
}

/// WARNING: These options have been marked as unstable:
///     - with_value
/// They work as advertised, but we may change them in a future release.
#[repr(C)]
pub struct z_get_options_t {
    pub target: z_query_target_t,
    pub consolidation: z_query_consolidation_t,
    pub with_value: z_value_t,
}
#[no_mangle]
pub extern "C" fn z_get_options_default() -> z_get_options_t {
    z_get_options_t {
        target: QueryTarget::default().into(),
        consolidation: QueryConsolidation::default().into(),
        with_value: {
            z_value_t {
                payload: z_bytes_t::empty(),
                encoding: z_encoding_default(),
            }
        },
    }
}

/// Query data from the matching queryables in the system.
/// Replies are provided through a callback function.
///
/// Returns a negative value upon failure.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression matching resources to query.
///     parameters: The query's parameters, similar to a url's query segment.
///     callback: The callback function that will be called on reception of replies for this query.
///               Note that the `reply` parameter of the callback is passed by mutable reference,
///               but **will** be dropped once your callback exits to help you avoid memory leaks.
///               If you'd rather take ownership, please refer to the documentation of :c:func:`z_reply_null`
///     options: additional options for the get.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_get(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    parameters: *const c_char,
    callback: &mut z_owned_closure_reply_t,
    options: Option<&z_get_options_t>,
) -> i8 {
    let mut closure = z_owned_closure_reply_t::empty();
    std::mem::swap(callback, &mut closure);
    let p = if parameters.is_null() {
        ""
    } else {
        CStr::from_ptr(parameters).to_str().unwrap()
    };
    let mut q = session
        .as_ref()
        .as_ref()
        .expect(LOG_INVALID_SESSION)
        .get(KeyExpr::try_from(keyexpr).unwrap().with_parameters(p));
    if let Some(options) = options {
        q = q
            .consolidation(options.consolidation)
            .target(options.target.into())
            .with_value(&options.with_value);
    }
    match q
        .callback(move |response| z_closure_reply_call(&closure, &mut response.into()))
        .res_sync()
    {
        Ok(()) => 0,
        Err(e) => {
            log::error!("{}", e);
            e.errno().get()
        }
    }
}

/// Frees `reply_data`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_drop(reply_data: &mut z_owned_reply_t) {
    std::mem::drop(reply_data.take());
}
/// Returns ``true`` if `reply_data` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_check(reply_data: &z_owned_reply_t) -> bool {
    reply_data.is_some()
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

impl From<&z_value_t> for Value {
    #[inline]
    fn from(val: &z_value_t) -> Value {
        unsafe {
            let value: Value =
                std::slice::from_raw_parts(val.payload.start, val.payload.len).into();
            value
        }
    }
}

impl From<&Value> for z_value_t {
    #[inline]
    fn from(val: &Value) -> z_value_t {
        let std::borrow::Cow::Borrowed(payload) = val.payload.contiguous() else{ panic!("Would have returned a reference to a temporary, make sure you the Value's payload is contiguous BEFORE calling this constructor.")};
        z_value_t {
            encoding: (&val.encoding).into(),
            payload: payload.into(),
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
/// Enumerators:
///      - **Z_CONSOLIDATION_MODE_AUTO**: Let Zenoh decide the best consolidation mode depending on the query selector
///          If the selector contains time range properties, consolidation mode `NONE` is used.
///          Otherwise the `LATEST` consolidation mode is used.
///      - **Z_CONSOLIDATION_MODE_NONE**: No consolidation is applied. Replies may come in any order and any number.
///      - **Z_CONSOLIDATION_MODE_MONOTONIC**: It guarantees that any reply for a given key expression will be monotonic in time
///          w.r.t. the previous received replies for the same key expression. I.e., for the same key expression multiple
///          replies may be received. It is guaranteed that two replies received at t1 and t2 will have timestamp
///          ts2 > ts1. It optimizes latency.
///      - **Z_CONSOLIDATION_MODE_LATEST**: It guarantees unicity of replies for the same key expression.
///          It optimizes bandwidth.
///
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_consolidation_mode_t {
    AUTO = -1,
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

impl Default for z_consolidation_mode_t {
    fn default() -> Self {
        z_consolidation_mode_t::NONE
    }
}

/// The replies consolidation strategy to apply on replies to a :c:func:`z_get`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_query_consolidation_t {
    pub mode: z_consolidation_mode_t,
}

impl From<QueryConsolidation> for z_query_consolidation_t {
    #[inline]
    fn from(qc: QueryConsolidation) -> Self {
        z_query_consolidation_t {
            mode: qc.mode().into(),
        }
    }
}

impl From<z_query_consolidation_t> for QueryConsolidation {
    #[inline]
    fn from(val: z_query_consolidation_t) -> Self {
        let cm: Mode<ConsolidationMode> = val.mode.into();
        match cm {
            Mode::Manual(cm) => QueryConsolidation::from(cm),
            Mode::Auto => QueryConsolidation::AUTO,
        }
    }
}

/// Creates a default :c:type:`z_query_consolidation_t` (consolidation mode AUTO).
#[no_mangle]
pub extern "C" fn z_query_consolidation_default() -> z_query_consolidation_t {
    QueryConsolidation::default().into()
}

/// Automatic query consolidation strategy selection.
///
/// A query consolidation strategy will automatically be selected depending the query selector.
/// If the selector contains time range properties, no consolidation is performed.
/// Otherwise the :c:func:`z_query_consolidation_latest` strategy is used.
///
/// Returns:
///   Returns the constructed :c:type:`z_query_consolidation_t`.
#[no_mangle]
pub extern "C" fn z_query_consolidation_auto() -> z_query_consolidation_t {
    QueryConsolidation::AUTO.into()
}

/// Latest value consolidation.
#[no_mangle]
pub extern "C" fn z_query_consolidation_latest() -> z_query_consolidation_t {
    QueryConsolidation::from(ConsolidationMode::Latest).into()
}

/// Monotonic consolidation.
#[no_mangle]
pub extern "C" fn z_query_consolidation_monotonic() -> z_query_consolidation_t {
    QueryConsolidation::from(ConsolidationMode::Monotonic).into()
}

/// Disable consolidation.
#[no_mangle]
pub extern "C" fn z_query_consolidation_none() -> z_query_consolidation_t {
    QueryConsolidation::from(ConsolidationMode::None).into()
}
