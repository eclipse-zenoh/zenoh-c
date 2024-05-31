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
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::ptr::null;
use std::ptr::null_mut;
use zenoh::sample::SampleBuilderTrait;
use zenoh::sample::ValueBuilderTrait;
use zenoh::selector::Selector;

use zenoh::query::{ConsolidationMode, QueryConsolidation, QueryTarget, Reply};

use crate::errors;
use crate::transmute::unwrap_ref_unchecked;
use crate::transmute::Inplace;
use crate::transmute::TransmuteFromHandle;
use crate::transmute::TransmuteIntoHandle;
use crate::transmute::TransmuteRef;
use crate::transmute::TransmuteUninitPtr;
use crate::z_closure_reply_loan;
use crate::z_consolidation_mode_t;
use crate::z_loaned_sample_t;
use crate::z_loaned_value_t;
use crate::z_owned_bytes_t;
use crate::z_owned_encoding_t;
use crate::z_query_target_t;
use crate::{
    z_closure_reply_call, z_loaned_keyexpr_t, z_loaned_session_t, z_owned_closure_reply_t,
};
use ::zenoh::core::Wait;

pub use crate::opaque_types::z_owned_reply_t;
decl_transmute_owned!(Option<Reply>, z_owned_reply_t);
pub use crate::opaque_types::z_loaned_reply_t;
decl_transmute_handle!(Reply, z_loaned_reply_t);

/// Returns ``true`` if reply contains a valid response, ``false`` otherwise (in this case it contains a errror value).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_is_ok(this: &z_loaned_reply_t) -> bool {
    this.transmute_ref().result().is_ok()
}

/// Yields the contents of the reply by asserting it indicates a success.
///
/// Returns `NULL` if reply does not contain a sample (i. e. if `z_reply_is_ok` returns ``false``).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_ok(this: &z_loaned_reply_t) -> *const z_loaned_sample_t {
    match this.transmute_ref().result() {
        Ok(sample) => sample.transmute_handle(),
        Err(_) => null(),
    }
}

/// Yields the contents of the reply by asserting it indicates a failure.
///
/// Returns `NULL` if reply does not contain a error  (i. e. if `z_reply_is_ok` returns ``true``).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_err(this: &z_loaned_reply_t) -> *const z_loaned_value_t {
    match this.transmute_ref().result() {
        Ok(_) => null(),
        Err(v) => v.transmute_handle(),
    }
}

/// Constructs the reply in its gravestone state.
#[no_mangle]
pub extern "C" fn z_reply_null(this: *mut MaybeUninit<z_owned_reply_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}
/// Constructs an owned shallow copy of reply in provided uninitialized memory location.
#[no_mangle]
pub extern "C" fn z_reply_clone(this: &z_loaned_reply_t, dst: *mut MaybeUninit<z_owned_reply_t>) {
    Inplace::init(
        dst.transmute_uninit_ptr(),
        Some(this.transmute_ref().clone()),
    );
}

/// Options passed to the `z_get()` function.
#[repr(C)]
pub struct z_get_options_t {
    /// The Queryables that should be target of the query.
    pub target: z_query_target_t,
    /// The replies consolidation strategy to apply on replies to the query.
    pub consolidation: z_query_consolidation_t,
    /// An optional payload to attach to the query.
    pub payload: *mut z_owned_bytes_t,
    /// An optional encoding of the query payload and or attachment.
    pub encoding: *mut z_owned_encoding_t,
    /// An optional attachment to attach to the query.
    pub attachment: *mut z_owned_bytes_t,
    /// The timeout for the query in milliseconds. 0 means default query timeout from zenoh configuration.
    pub timeout_ms: u64,
}

/// Constructs default `z_get_options_t`
#[no_mangle]
pub extern "C" fn z_get_options_default(this: &mut z_get_options_t) {
    *this = z_get_options_t {
        target: QueryTarget::default().into(),
        consolidation: QueryConsolidation::default().into(),
        timeout_ms: 0,
        payload: null_mut(),
        encoding: null_mut(),
        attachment: null_mut(),
    };
}

/// Query data from the matching queryables in the system.
/// Replies are provided through a callback function.
///
/// @param session: The zenoh session.
/// @param key_expr: The key expression matching resources to query.
/// @param parameters: The query's parameters, similar to a url's query segment.
/// @param callback: The callback function that will be called on reception of replies for this query. It will be automatically dropped once all replies are processed.
/// @param options: Additional options for the get. All owned fields will be consumed.
///
/// @return 0 in case of success, a negative error value upon failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_get(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    parameters: *const c_char,
    callback: &mut z_owned_closure_reply_t,
    options: Option<&mut z_get_options_t>,
) -> errors::z_error_t {
    let mut closure = z_owned_closure_reply_t::empty();
    std::mem::swap(callback, &mut closure);
    let p = if parameters.is_null() {
        ""
    } else {
        CStr::from_ptr(parameters).to_str().unwrap()
    };
    let session = session.transmute_ref();
    let key_expr = key_expr.transmute_ref();
    let mut get = session.get(Selector::new(key_expr, p));
    if let Some(options) = options {
        if !options.payload.is_null() {
            if let Some(payload) = unsafe { *options.payload }.transmute_mut().extract() {
                get = get.payload(payload);
            }
        }
        if !options.encoding.is_null() {
            let encoding = unsafe { *options.encoding }.transmute_mut().extract();
            get = get.encoding(encoding);
        }
        if !options.attachment.is_null() {
            let attachment = unsafe { *options.attachment }.transmute_mut().extract();
            get = get.attachment(attachment);
        }

        get = get
            .consolidation(options.consolidation)
            .target(options.target.into());

        if options.timeout_ms != 0 {
            get = get.timeout(std::time::Duration::from_millis(options.timeout_ms));
        }
    }
    match get
        .callback(move |response| {
            z_closure_reply_call(z_closure_reply_loan(&closure), response.transmute_handle())
        })
        .wait()
    {
        Ok(()) => errors::Z_OK,
        Err(e) => {
            log::error!("{}", e);
            errors::Z_EGENERIC
        }
    }
}

/// Frees reply, resetting it to its gravestone state.
#[no_mangle]
pub extern "C" fn z_reply_drop(this: &mut z_owned_reply_t) {
    Inplace::drop(this.transmute_mut())
}

/// Returns ``true`` if `reply` is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_reply_check(this: &z_owned_reply_t) -> bool {
    this.transmute_ref().is_some()
}

/// Borrows reply.
#[no_mangle]
pub extern "C" fn z_reply_loan(this: &z_owned_reply_t) -> &z_loaned_reply_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

/// The replies consolidation strategy to apply on replies to a `z_get()`.
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
        let cm: ConsolidationMode = val.mode.into();
        cm.into()
    }
}

/// Creates a default `z_query_consolidation_t` (consolidation mode AUTO).
#[no_mangle]
pub extern "C" fn z_query_consolidation_default() -> z_query_consolidation_t {
    QueryConsolidation::default().into()
}

/// Automatic query consolidation strategy selection.
///
/// A query consolidation strategy will automatically be selected depending the query selector.
/// If the selector contains time range properties, no consolidation is performed.
/// Otherwise the `z_query_consolidation_latest` strategy is used.
#[no_mangle]
pub extern "C" fn z_query_consolidation_auto() -> z_query_consolidation_t {
    QueryConsolidation::AUTO.into()
}

/// Latest consolidation.
///
/// This strategy optimizes bandwidth on all links in the system but will provide a very poor latency.
#[no_mangle]
pub extern "C" fn z_query_consolidation_latest() -> z_query_consolidation_t {
    QueryConsolidation::from(ConsolidationMode::Latest).into()
}

/// Monotonic consolidation.
///
/// This strategy offers the best latency. Replies are directly transmitted to the application when received
/// without needing to wait for all replies. This mode does not guarantee that there will be no duplicates.
#[no_mangle]
pub extern "C" fn z_query_consolidation_monotonic() -> z_query_consolidation_t {
    QueryConsolidation::from(ConsolidationMode::Monotonic).into()
}

/// No consolidation.
///
/// This strategy is useful when querying timeseries data bases or when using quorums.
#[no_mangle]
pub extern "C" fn z_query_consolidation_none() -> z_query_consolidation_t {
    QueryConsolidation::from(ConsolidationMode::None).into()
}
