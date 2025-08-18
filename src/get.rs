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

use std::{
    mem::MaybeUninit,
    ptr::{null, null_mut},
};

use libc::c_char;
use prebindgen_proc_macro::prebindgen;
use zenoh::{
    qos::{CongestionControl, Priority},
    query::{ConsolidationMode, QueryConsolidation, QueryTarget, Reply, ReplyError, Selector},
    session::SessionClosedError,
    Wait,
};

pub use zenoh_ffi_opaque_types::opaque_types::{z_loaned_reply_err_t, z_moved_reply_err_t, z_owned_reply_err_t};
use crate::{
    result::{self, Z_EINVAL},
    strlen_or_zero,
    transmute::{Gravestone, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_reply_call, z_closure_reply_loan, z_congestion_control_t, z_consolidation_mode_t,
    z_loaned_bytes_t, z_loaned_encoding_t, z_loaned_keyexpr_t, z_loaned_sample_t,
    z_loaned_session_t, z_moved_bytes_t, z_moved_closure_reply_t, z_moved_encoding_t, z_priority_t,
    z_query_target_t, zc_locality_default, zc_locality_t, CStringView,
};
#[cfg(feature = "unstable")]
use crate::{
    transmute::IntoCType, z_entity_global_id_t, z_moved_source_info_t, zc_reply_keyexpr_default,
    zc_reply_keyexpr_t,
};
decl_c_type!(
    owned(z_owned_reply_err_t, ReplyError),
    loaned(z_loaned_reply_err_t, ReplyError),
);

impl Gravestone for ReplyError {
    fn gravestone() -> Self {
        ReplyError::empty()
    }
    fn is_gravestone(&self) -> bool {
        self.payload().is_empty()
    }
}

/// Constructs an empty `z_owned_reply_err_t`.
#[prebindgen]
pub fn z_internal_reply_err_null(this_: &mut MaybeUninit<z_owned_reply_err_t>) {
    this_
        .as_rust_type_mut_uninit()
        .write(ReplyError::gravestone());
}

/// Returns ``true`` if reply error is in non-default state, ``false`` otherwise.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn z_internal_reply_err_check(this_: &'static z_owned_reply_err_t) -> bool {
    !this_.as_rust_type_ref().is_gravestone()
}

/// Returns reply error payload.
#[prebindgen]
pub fn z_reply_err_payload(this_: &z_loaned_reply_err_t) -> &z_loaned_bytes_t {
    this_.as_rust_type_ref().payload().as_loaned_c_type_ref()
}

/// Returns mutable reply error payload.
#[prebindgen]
pub fn z_reply_err_payload_mut(
    this_: &mut z_loaned_reply_err_t,
) -> &mut z_loaned_bytes_t {
    this_
        .as_rust_type_mut()
        .payload_mut()
        .as_loaned_c_type_mut()
}

/// Returns reply error encoding.
#[prebindgen]
pub fn z_reply_err_encoding(this_: &z_loaned_reply_err_t) -> &z_loaned_encoding_t {
    this_.as_rust_type_ref().encoding().as_loaned_c_type_ref()
}

/// Borrows reply error.
#[prebindgen]
pub fn z_reply_err_loan(this_: &z_owned_reply_err_t) -> &z_loaned_reply_err_t {
    this_.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Moves reply error.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_err_move(this_: &mut z_owned_reply_err_t) -> &mut z_moved_reply_err_t {
    std::mem::transmute(this_)
}

/// Mutably borrows reply error.
#[prebindgen]
pub fn z_reply_err_loan_mut(
    this_: &mut z_owned_reply_err_t,
) -> &mut z_loaned_reply_err_t {
    this_.as_rust_type_mut().as_loaned_c_type_mut()
}

/// Frees the memory and resets the reply error it to its default value.
#[prebindgen]
pub fn z_reply_err_drop(this_: &mut z_moved_reply_err_t) {
    let _ = this_.take_rust_type();
}

use zenoh_ffi_opaque_types::opaque_types::{z_loaned_reply_t, z_moved_reply_t, z_owned_reply_t};
decl_c_type!(
    owned(z_owned_reply_t, option Reply),
    loaned(z_loaned_reply_t),
);

/// Returns ``true`` if reply contains a valid response, ``false`` otherwise (in this case it contains a errror value).
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_is_ok(this_: &z_loaned_reply_t) -> bool {
    this_.as_rust_type_ref().result().is_ok()
}

/// Yields the contents of the reply by asserting it indicates a success.
///
/// Returns `NULL` if reply does not contain a sample (i. e. if `z_reply_is_ok` returns ``false``).
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_ok(this_: &z_loaned_reply_t) -> *const z_loaned_sample_t {
    match this_.as_rust_type_ref().result() {
        Ok(sample) => sample.as_loaned_c_type_ref() as _,
        Err(_) => null(),
    }
}

/// Yields the contents of the reply by asserting it indicates a success.
///
/// Returns `NULL` if reply does not contain a sample (i. e. if `z_reply_is_ok` returns ``false``).
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_ok_mut(this_: &mut z_loaned_reply_t) -> *mut z_loaned_sample_t {
    match this_.as_rust_type_mut().result_mut() {
        Ok(sample) => sample.as_loaned_c_type_mut() as _,
        Err(_) => null_mut(),
    }
}

/// Yields the contents of the reply by asserting it indicates a failure.
///
/// Returns `NULL` if reply does not contain a error  (i. e. if `z_reply_is_ok` returns ``true``).
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_err(this_: &z_loaned_reply_t) -> *const z_loaned_reply_err_t {
    match this_.as_rust_type_ref().result() {
        Ok(_) => null(),
        Err(v) => v.as_loaned_c_type_ref(),
    }
}

/// Yields the contents of the reply by asserting it indicates a failure.
///
/// Returns `NULL` if reply does not contain a error  (i. e. if `z_reply_is_ok` returns ``true``).
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_err_mut(
    this_: &mut z_loaned_reply_t,
) -> *mut z_loaned_reply_err_t {
    match this_.as_rust_type_mut().result_mut() {
        Ok(_) => null_mut(),
        Err(v) => v.as_loaned_c_type_mut(),
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Gets the global id of the zenoh entity that answered this Reply.
/// @return `true` if id is present.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_replier_id(
    this: &z_loaned_reply_t,
    out_id: &mut MaybeUninit<z_entity_global_id_t>,
) -> bool {
    match this.as_rust_type_ref().replier_id() {
        Some(val) => {
            out_id.write(val.into_c_type());
            true
        }
        None => false,
    }
}

/// Constructs the reply in its gravestone state.
#[prebindgen]
pub fn z_internal_reply_null(this_: &mut MaybeUninit<z_owned_reply_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}
/// Constructs an owned shallow copy of reply in provided uninitialized memory location.
#[prebindgen]
pub fn z_reply_clone(dst: &mut MaybeUninit<z_owned_reply_t>, this_: &z_loaned_reply_t) {
    dst.as_rust_type_mut_uninit()
        .write(Some(this_.as_rust_type_ref().clone()));
}

/// Options passed to the `z_get()` function.
#[prebindgen]
#[repr(C)]
pub struct z_get_options_t {
    /// The Queryables that should be target of the query.
    pub target: z_query_target_t,
    /// The replies consolidation strategy to apply on replies to the query.
    pub consolidation: z_query_consolidation_t,
    /// An optional payload to attach to the query.
    pub payload: Option<&'static mut z_moved_bytes_t>,
    /// An optional encoding of the query payload and or attachment.
    pub encoding: Option<&'static mut z_moved_encoding_t>,
    /// The congestion control to apply when routing the query.
    pub congestion_control: z_congestion_control_t,
    /// If set to ``true``, this message will not be batched. This usually has a positive impact on latency but negative impact on throughput.
    pub is_express: bool,
    /// The allowed destination for the query.
    pub allowed_destination: zc_locality_t,
    #[cfg(feature = "unstable")]
    /// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
    ///
    /// The accepted replies for the query.
    pub accept_replies: zc_reply_keyexpr_t,
    /// The priority of the query.
    pub priority: z_priority_t,
    #[cfg(feature = "unstable")]
    /// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
    ///
    /// The source info for the query.
    pub source_info: Option<&'static mut z_moved_source_info_t>,
    /// An optional attachment to attach to the query.
    pub attachment: Option<&'static mut z_moved_bytes_t>,
    /// The timeout for the query in milliseconds. 0 means default query timeout from zenoh configuration.
    pub timeout_ms: u64,
}

impl z_get_options_t {
    fn clear(&mut self) {
        if let Some(p) = self.payload.take() {
            p.take_rust_type();
        }
        if let Some(e) = self.encoding.take() {
            e.take_rust_type();
        }
        if let Some(a) = self.attachment.take() {
            a.take_rust_type();
        }
        #[cfg(feature = "unstable")]
        if let Some(si) = self.source_info.take() {
            si.take_rust_type();
        }
    }
}

/// Constructs default `z_get_options_t`
#[prebindgen]
pub fn z_get_options_default(this_: &mut MaybeUninit<z_get_options_t>) {
    this_.write(z_get_options_t {
        target: QueryTarget::default().into(),
        consolidation: QueryConsolidation::default().into(),
        congestion_control: CongestionControl::DEFAULT_REQUEST.into(),
        allowed_destination: zc_locality_default(),
        #[cfg(feature = "unstable")]
        accept_replies: zc_reply_keyexpr_default(),
        priority: Priority::default().into(),
        is_express: false,
        timeout_ms: 0,
        payload: None,
        encoding: None,
        #[cfg(feature = "unstable")]
        source_info: None,
        attachment: None,
    });
}

/// Query data from the matching queryables in the system.
/// Replies are provided through a callback function.
///
/// @param session: The zenoh session.
/// @param key_expr: The key expression matching resources to query.
/// @param parameters: The query's parameters null-terminated string, similar to a url's query segment.
/// @param callback: The callback function that will be called on reception of replies for this query. It will be automatically dropped once all replies are processed.
/// @param options: Additional options for the get. All owned fields will be consumed.
///
/// @return 0 in case of success, a negative error value upon failure.
#[allow(clippy::missing_safety_doc)]
#[prebindgen]
pub unsafe fn z_get(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    parameters: *const c_char,
    callback: &mut z_moved_closure_reply_t,
    options: Option<&mut z_get_options_t>,
) -> result::z_result_t {
    z_get_with_parameters_substr(
        session,
        key_expr,
        parameters,
        strlen_or_zero(parameters),
        callback,
        options,
    )
}

/// Query data from the matching queryables in the system.
/// Replies are provided through a callback function.
///
/// @param session: The zenoh session.
/// @param key_expr: The key expression matching resources to query.
/// @param parameters: The query's parameters string, similar to a url's query segment.
/// @param parameters_len: The parameters substring length.
/// @param callback: The callback function that will be called on reception of replies for this query. It will be automatically dropped once all replies are processed.
/// @param options: Additional options for the get. All owned fields will be consumed.
///
/// @return 0 in case of success, a negative error value upon failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_get_with_parameters_substr(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    parameters: *const c_char,
    parameters_len: usize,
    callback: &mut z_moved_closure_reply_t,
    options: Option<&mut z_get_options_t>,
) -> result::z_result_t {
    let callback = callback.take_rust_type();
    let pcs = match CStringView::new_borrowed(parameters as *const c_char, parameters_len) {
        Ok(cs) => cs,
        Err(r) => {
            if let Some(o) = options {
                o.clear();
            }
            return r;
        }
    };

    let p: &str = match (&pcs).try_into() {
        Ok(s) => s,
        Err(e) => {
            if let Some(o) = options {
                o.clear();
            }
            crate::report_error!("Parameters is not a valid utf-8 string: {e}");
            return Z_EINVAL;
        }
    };

    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let mut get = session.get(Selector::from((key_expr, p)));
    if let Some(options) = options {
        if let Some(payload) = options.payload.take() {
            get = get.payload(payload.take_rust_type());
        }
        if let Some(encoding) = options.encoding.take() {
            get = get.encoding(encoding.take_rust_type());
        }
        #[cfg(feature = "unstable")]
        if let Some(source_info) = options.source_info.take() {
            get = get.source_info(source_info.take_rust_type());
        }
        if let Some(attachment) = options.attachment.take() {
            get = get.attachment(attachment.take_rust_type());
        }

        get = get
            .consolidation(options.consolidation)
            .target(options.target.into())
            .congestion_control(options.congestion_control.into())
            .priority(options.priority.into())
            .express(options.is_express);
        #[cfg(feature = "unstable")]
        {
            get = get
                .allowed_destination(options.allowed_destination.into())
                .accept_replies(options.accept_replies.into());
        }

        if options.timeout_ms != 0 {
            get = get.timeout(std::time::Duration::from_millis(options.timeout_ms));
        }
    }
    match get
        .callback(move |response| {
            let mut owned_response = Some(response);
            z_closure_reply_call(
                z_closure_reply_loan(&callback),
                owned_response
                    .as_mut()
                    .unwrap_unchecked()
                    .as_loaned_c_type_mut(),
            )
        })
        .wait()
    {
        Ok(()) => result::Z_OK,
        Err(e) if e.downcast_ref::<SessionClosedError>().is_some() => result::Z_ESESSION_CLOSED,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// Frees reply, resetting it to its gravestone state.
#[prebindgen]
pub fn z_reply_drop(this_: &mut z_moved_reply_t) {
    let _ = this_.take_rust_type();
}

/// Returns ``true`` if `reply` is valid, ``false`` otherwise.
#[prebindgen]
pub fn z_internal_reply_check(this_: &z_owned_reply_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Borrows reply.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_loan(this_: &z_owned_reply_t) -> &z_loaned_reply_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Moves reply.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_move(this_: &mut z_owned_reply_t) -> &mut z_moved_reply_t {
    std::mem::transmute(this_)
}

/// Mutably borrows reply.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_loan_mut(this_: &mut z_owned_reply_t) -> &mut z_loaned_reply_t {
    this_
        .as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// Takes ownership of the mutably borrowed reply
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_reply_take_from_loaned(
    dst: &mut MaybeUninit<z_owned_reply_t>,
    src: &mut z_loaned_reply_t,
) {
    let dst = dst.as_rust_type_mut_uninit();
    let src = src.as_rust_type_mut();
    let src = std::mem::replace(src, Reply::empty());
    dst.write(Some(src));
}

/// The replies consolidation strategy to apply on replies to a `z_get()`.
#[prebindgen]
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
#[prebindgen]
pub fn z_query_consolidation_default() -> z_query_consolidation_t {
    QueryConsolidation::default().into()
}

/// Automatic query consolidation strategy selection.
///
/// A query consolidation strategy will automatically be selected depending the query selector.
/// If the selector contains time range properties, no consolidation is performed.
/// Otherwise the `z_query_consolidation_latest` strategy is used.
#[prebindgen]
pub fn z_query_consolidation_auto() -> z_query_consolidation_t {
    QueryConsolidation::AUTO.into()
}

/// Latest consolidation.
///
/// This strategy optimizes bandwidth on all links in the system but will provide a very poor latency.
#[prebindgen]
pub fn z_query_consolidation_latest() -> z_query_consolidation_t {
    QueryConsolidation::from(ConsolidationMode::Latest).into()
}

/// Monotonic consolidation.
///
/// This strategy offers the best latency. Replies are directly transmitted to the application when received
/// without needing to wait for all replies. This mode does not guarantee that there will be no duplicates.
#[prebindgen]
pub fn z_query_consolidation_monotonic() -> z_query_consolidation_t {
    QueryConsolidation::from(ConsolidationMode::Monotonic).into()
}

/// No consolidation.
///
/// This strategy is useful when querying timeseries data bases or when using quorums.
#[prebindgen]
pub fn z_query_consolidation_none() -> z_query_consolidation_t {
    QueryConsolidation::from(ConsolidationMode::None).into()
}

/// Constructs a copy of the reply error message.
#[prebindgen]
pub fn z_reply_err_clone(
    dst: &mut MaybeUninit<z_owned_reply_err_t>,
    this: &z_loaned_reply_err_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(this.as_rust_type_ref().clone());
}
