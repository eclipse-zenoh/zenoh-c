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

use crate::errors;
use crate::transmute::IntoRustType;
use crate::transmute::LoanedCTypeRef;
use crate::transmute::RustTypeRef;
use crate::transmute::RustTypeRefUninit;
use crate::z_id_t;
use crate::z_moved_closure_reply_t;
use crate::{
    z_closure_reply_call, z_closure_reply_loan, z_congestion_control_t, z_consolidation_mode_t,
    z_loaned_bytes_t, z_loaned_encoding_t, z_loaned_keyexpr_t, z_loaned_sample_t,
    z_loaned_session_t, z_owned_bytes_t, z_owned_encoding_t, z_owned_source_info_t, z_priority_t,
    z_query_target_t, zcu_locality_default, zcu_locality_t, zcu_reply_keyexpr_default,
    zcu_reply_keyexpr_t,
};
use ::zenoh::core::Wait;
use libc::c_char;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::null;
use std::ptr::null_mut;
use zenoh::bytes::ZBytes;
use zenoh::core::Priority;
use zenoh::encoding::Encoding;
use zenoh::query::ReplyError;
use zenoh::query::{ConsolidationMode, QueryConsolidation, QueryTarget, Reply};
use zenoh::sample::EncodingBuilderTrait;
use zenoh::sample::QoSBuilderTrait;
use zenoh::sample::SampleBuilderTrait;
use zenoh::selector::Selector;
use zenoh_protocol::core::CongestionControl;
use zenoh_protocol::core::ZenohIdProto;

// we need to add Default to ReplyError
#[repr(transparent)]
pub(crate) struct ReplyErrorNewtype(ReplyError);
impl Default for ReplyErrorNewtype {
    fn default() -> Self {
        Self(zenoh::internal::Value::new(ZBytes::empty(), Encoding::default()).into())
    }
}
impl Deref for ReplyErrorNewtype {
    type Target = ReplyError;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ReplyErrorNewtype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<&ReplyError> for &ReplyErrorNewtype {
    fn from(value: &ReplyError) -> Self {
        // SAFETY: ReplyErrorNewtype is repr(transparent) to ReplyError
        unsafe { core::mem::transmute::<&ReplyError, Self>(value) }
    }
}

pub use crate::opaque_types::z_loaned_reply_err_t;
pub use crate::opaque_types::z_moved_reply_err_t;
pub use crate::opaque_types::z_owned_reply_err_t;
decl_c_type!(
    owned(z_owned_reply_err_t, ReplyErrorNewtype),
    loaned(z_loaned_reply_err_t, ReplyErrorNewtype),
    moved(z_moved_reply_err_t)
);

/// Constructs an empty `z_owned_reply_err_t`.
#[no_mangle]
pub extern "C" fn z_reply_err_null(this: &mut MaybeUninit<z_owned_reply_err_t>) {
    this.as_rust_type_mut_uninit()
        .write(ReplyErrorNewtype::default());
}

/// Returns ``true`` if reply error is in non-default state, ``false`` otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_reply_err_check(this: &'static z_owned_reply_err_t) -> bool {
    !this.as_rust_type_ref().payload().is_empty()
}

/// Returns reply error payload.
#[no_mangle]
pub extern "C" fn z_reply_err_payload(this: &z_loaned_reply_err_t) -> &z_loaned_bytes_t {
    this.as_rust_type_ref().payload().as_loaned_c_type_ref()
}

/// Returns reply error encoding.
#[no_mangle]
pub extern "C" fn z_reply_err_encoding(this: &z_loaned_reply_err_t) -> &z_loaned_encoding_t {
    this.as_rust_type_ref().encoding().as_loaned_c_type_ref()
}

/// Borrows reply error.
#[no_mangle]
pub extern "C" fn z_reply_err_loan(this: &z_owned_reply_err_t) -> &z_loaned_reply_err_t {
    this.as_rust_type_ref().as_loaned_c_type_ref()
}

/// Frees the memory and resets the reply error it to its default value.
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_reply_err_drop(this: z_moved_reply_err_t) {}

pub use crate::opaque_types::z_loaned_reply_t;
pub use crate::opaque_types::z_moved_reply_t;
pub use crate::opaque_types::z_owned_reply_t;
decl_c_type!(
    owned(z_owned_reply_t, Option<Reply>),
    loaned(z_loaned_reply_t, Reply),
    moved(z_moved_reply_t)
);

/// Returns ``true`` if reply contains a valid response, ``false`` otherwise (in this case it contains a errror value).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_is_ok(this: &z_loaned_reply_t) -> bool {
    this.as_rust_type_ref().result().is_ok()
}

/// Yields the contents of the reply by asserting it indicates a success.
///
/// Returns `NULL` if reply does not contain a sample (i. e. if `z_reply_is_ok` returns ``false``).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_ok(this: &z_loaned_reply_t) -> *const z_loaned_sample_t {
    match this.as_rust_type_ref().result() {
        Ok(sample) => sample.as_loaned_c_type_ref() as _,
        Err(_) => null(),
    }
}

/// Yields the contents of the reply by asserting it indicates a failure.
///
/// Returns `NULL` if reply does not contain a error  (i. e. if `z_reply_is_ok` returns ``true``).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_err(this: &z_loaned_reply_t) -> *const z_loaned_reply_err_t {
    match this.as_rust_type_ref().result() {
        Ok(_) => null(),
        Err(v) => std::convert::Into::<&ReplyErrorNewtype>::into(v).as_loaned_c_type_ref(),
    }
}

/// Gets the id of the zenoh instance that answered this Reply.
/// Returns `true` if id is present
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_replier_id(
    this: &z_loaned_reply_t,
    out_id: &mut MaybeUninit<z_id_t>,
) -> bool {
    match this.as_rust_type_ref().replier_id() {
        Some(val) => {
            out_id.write(
                std::convert::Into::<ZenohIdProto>::into(val)
                    .to_le_bytes()
                    .into(),
            );
            true
        }
        None => false,
    }
}

/// Constructs the reply in its gravestone state.
#[no_mangle]
pub extern "C" fn z_reply_null(this: &mut MaybeUninit<z_owned_reply_t>) {
    this.as_rust_type_mut_uninit().write(None);
}
/// Constructs an owned shallow copy of reply in provided uninitialized memory location.
#[no_mangle]
pub extern "C" fn z_reply_clone(this: &z_loaned_reply_t, dst: &mut MaybeUninit<z_owned_reply_t>) {
    dst.as_rust_type_mut_uninit()
        .write(Some(this.as_rust_type_ref().clone()));
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
    /// The congestion control to apply when routing the query.
    pub congestion_control: z_congestion_control_t,
    /// The allowed destination for the query.
    pub allowed_destination: zcu_locality_t,
    /// The accepted replies for the query.
    pub accept_replies: zcu_reply_keyexpr_t,
    /// The priority of the query.
    pub priority: z_priority_t,
    /// The source info for the query.
    pub source_info: *mut z_owned_source_info_t,
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
        congestion_control: CongestionControl::default().into(),
        allowed_destination: zcu_locality_default(),
        accept_replies: zcu_reply_keyexpr_default(),
        priority: Priority::default().into(),
        timeout_ms: 0,
        payload: null_mut(),
        encoding: null_mut(),
        source_info: null_mut(),
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
    callback: z_moved_closure_reply_t,
    options: Option<&mut z_get_options_t>,
) -> errors::z_error_t {
    let Some(callback) = callback.into_rust_type() else {
        return errors::Z_EINVAL;
    };
    let p = if parameters.is_null() {
        ""
    } else {
        CStr::from_ptr(parameters).to_str().unwrap()
    };
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let mut get = session.get(Selector::from((key_expr, p)));
    if let Some(options) = options {
        if let Some(payload) = unsafe { options.payload.as_mut() } {
            let payload = std::mem::take(payload.as_rust_type_mut());
            get = get.payload(payload);
        }
        if let Some(encoding) = unsafe { options.encoding.as_mut() } {
            let encoding = std::mem::take(encoding.as_rust_type_mut());
            get = get.encoding(encoding);
        }
        if let Some(source_info) = unsafe { options.source_info.as_mut() } {
            let source_info = std::mem::take(source_info.as_rust_type_mut());
            get = get.source_info(source_info);
        }
        if let Some(attachment) = unsafe { options.attachment.as_mut() } {
            let attachment = std::mem::take(attachment.as_rust_type_mut());
            get = get.attachment(attachment);
        }

        get = get
            .consolidation(options.consolidation)
            .target(options.target.into())
            .congestion_control(options.congestion_control.into())
            .allowed_destination(options.allowed_destination.into())
            .accept_replies(options.accept_replies.into())
            .priority(options.priority.into());

        if options.timeout_ms != 0 {
            get = get.timeout(std::time::Duration::from_millis(options.timeout_ms));
        }
    }
    match get
        .callback(move |response| {
            z_closure_reply_call(
                z_closure_reply_loan(&callback),
                response.as_loaned_c_type_ref(),
            )
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
#[allow(unused_variables)]
pub extern "C" fn z_reply_drop(this: z_moved_reply_t) {}

/// Returns ``true`` if `reply` is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_reply_check(this: &z_owned_reply_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Borrows reply.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_reply_loan(this: &z_owned_reply_t) -> &z_loaned_reply_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
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
