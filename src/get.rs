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
use std::ptr::null_mut;
use zenoh::sample::SampleBuilderTrait;
use zenoh::sample::ValueBuilderTrait;

use zenoh::prelude::{ConsolidationMode, Mode, QueryConsolidation, QueryTarget, Reply};

use crate::errors;
use crate::transmute::unwrap_ref_unchecked;
use crate::transmute::Inplace;
use crate::transmute::TransmuteFromHandle;
use crate::transmute::TransmuteIntoHandle;
use crate::transmute::TransmuteRef;
use crate::transmute::TransmuteUninitPtr;
use crate::z_consolidation_mode_t;
use crate::z_owned_bytes_t;
use crate::z_owned_encoding_t;
use crate::z_query_target_t;
use crate::z_sample_t;
use crate::z_value_t;
use crate::{z_closure_reply_call, z_keyexpr_t, z_owned_closure_reply_t, z_session_t};
use zenoh::prelude::SyncResolve;

pub use crate::opaque_types::z_owned_reply_t;
decl_transmute_owned!(Option<Reply>, z_owned_reply_t);
pub use crate::opaque_types::z_reply_t;
decl_transmute_handle!(Reply, z_reply_t);

/// Returns ``true`` if the queryable answered with an OK, which allows this value to be treated as a sample.
///
/// If this returns ``false``, you should use :c:func:`z_check` before trying to use :c:func:`z_reply_err` if you want to process the error that may be here.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_is_ok(reply: z_reply_t) -> bool {
    let reply = reply.transmute_ref();
    reply.result().is_ok()
}

/// Yields the contents of the reply by asserting it indicates a success.
///
/// You should always make sure that :c:func:`z_reply_is_ok` returns ``true`` before calling this function.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_ok(reply: z_reply_t) -> z_sample_t {
    let reply = reply.transmute_ref();
    reply
        .result()
        .expect("Reply does not contain a sample")
        .transmute_handle()
}

/// Yields the contents of the reply by asserting it indicates a failure.
///
/// You should always make sure that :c:func:`z_reply_is_ok` returns ``false`` before calling this function.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_err(reply: z_reply_t) -> z_value_t {
    let reply = reply.transmute_ref();
    reply
        .result()
        .expect_err("Reply does not contain error")
        .transmute_handle()
}

/// Returns an invalidated :c:type:`z_owned_reply_t`.
///
/// This is useful when you wish to take ownership of a value from a callback to :c:func:`z_get`:
///
///     - copy the value of the callback's argument's pointee,
///     - overwrite the pointee with this function's return value,
///     - you are now responsible for dropping your copy of the reply.
#[no_mangle]
pub extern "C" fn z_reply_null(this: *mut MaybeUninit<z_owned_reply_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

#[no_mangle]
pub extern "C" fn z_reply_clone(this: *mut MaybeUninit<z_owned_reply_t>, reply: z_reply_t) {
    Inplace::init(
        this.transmute_uninit_ptr(),
        Some(reply.transmute_ref().clone()),
    );
}

/// Options passed to the :c:func:`z_get` function.
///
/// Members:
///     z_query_target_t target: The Queryables that should be target of the query.
///     z_query_consolidation_t consolidation: The replies consolidation strategy to apply on replies to the query.
///     z_value_t value: An optional value to attach to the query.
///    z_bytes_t attachment: The attachment to attach to the query.
///     uint64_t timeout: The timeout for the query in milliseconds. 0 means default query timeout from zenoh configuration.
#[repr(C)]
pub struct z_get_options_t {
    pub target: z_query_target_t,
    pub consolidation: z_query_consolidation_t,
    pub payload: *mut z_owned_bytes_t,
    pub encoding: *mut z_owned_encoding_t,
    pub attachment: *mut z_owned_bytes_t,
    pub timeout_ms: u64,
}
#[no_mangle]
pub extern "C" fn z_get_options_default() -> z_get_options_t {
    z_get_options_t {
        target: QueryTarget::default().into(),
        consolidation: QueryConsolidation::default().into(),
        timeout_ms: 0,
        payload: null_mut(),
        encoding: null_mut(),
        attachment: null_mut(),
    }
}

/// Query data from the matching queryables in the system.
/// Replies are provided through a callback function.
///
/// Returns a negative value upon failure.
///
/// Parameters:
///     session: The zenoh session.
///     key_expr: The key expression matching resources to query.
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
    key_expr: z_keyexpr_t,
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

    let mut get = session.get(key_expr.clone().with_parameters(p));
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
            let attachment = unsafe { *options.payload }.transmute_mut().extract();
            get = get.attachment(attachment);
        }

        get = get
            .consolidation(options.consolidation)
            .timeout(std::time::Duration::from_millis(options.timeout_ms))
            .target(options.target.into());
    }
    match get
        .callback(move |response| z_closure_reply_call(&closure, response.transmute_handle()))
        .res_sync()
    {
        Ok(()) => errors::Z_OK,
        Err(e) => {
            log::error!("{}", e);
            errors::Z_EGENERIC
        }
    }
}

/// Frees `reply`, invalidating it for double-drop safety.
#[no_mangle]
pub extern "C" fn z_reply_drop(this: &mut z_owned_reply_t) {
    Inplace::drop(this.transmute_mut())
}
/// Returns ``true`` if `reply` is valid.
#[no_mangle]
pub extern "C" fn z_reply_check(this: &z_owned_reply_t) -> bool {
    this.transmute_ref().is_some()
}

#[no_mangle]
pub extern "C" fn z_reply_loan(this: &mut z_owned_reply_t) -> z_reply_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
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
