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
use crate::transmute::{
    unwrap_ref_unchecked, Inplace, TransmuteFromHandle, TransmuteIntoHandle, TransmuteRef,
    TransmuteUninitPtr,
};
use crate::{
    errors, z_closure_query_call, z_closure_query_loan, z_congestion_control_t, z_loaned_bytes_t,
    z_loaned_encoding_t, z_loaned_keyexpr_t, z_loaned_session_t, z_owned_bytes_t,
    z_owned_closure_query_t, z_owned_encoding_t, z_owned_source_info_t, z_priority_t,
    z_timestamp_t, z_view_string_from_substring, z_view_string_t,
};
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use zenoh::core::Wait;
use zenoh::encoding::Encoding;
use zenoh::prelude::SessionDeclarations;
use zenoh::publisher::CongestionControl;
use zenoh::publisher::Priority;
use zenoh::queryable::{Query, Queryable};
use zenoh::sample::{
    QoSBuilderTrait, SampleBuilderTrait, TimestampBuilderTrait, ValueBuilderTrait,
};

pub use crate::opaque_types::z_owned_queryable_t;
decl_transmute_owned!(Option<Queryable<'static, ()>>, z_owned_queryable_t);
pub use crate::opaque_types::z_loaned_queryable_t;
decl_transmute_handle!(Queryable<'static, ()>, z_loaned_queryable_t);
validate_equivalence!(z_owned_queryable_t, z_loaned_queryable_t);

/// Constructs a queryable in its gravestone value.
#[no_mangle]
pub extern "C" fn z_queryable_null(this: *mut MaybeUninit<z_owned_queryable_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

// Borrows Queryable
#[no_mangle]
pub extern "C" fn z_queryable_loan(this: &z_owned_queryable_t) -> &z_loaned_queryable_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

pub use crate::opaque_types::z_loaned_query_t;
decl_transmute_handle!(Query, z_loaned_query_t);

pub use crate::opaque_types::z_owned_query_t;
decl_transmute_owned!(Option<Query>, z_owned_query_t);

validate_equivalence!(z_owned_query_t, z_loaned_query_t);

/// Constructs query in its gravestone value.
#[no_mangle]
pub extern "C" fn z_query_null(this: *mut MaybeUninit<z_owned_query_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}
/// Returns `false` if `this` is in a gravestone state, `true` otherwise.
#[no_mangle]
pub extern "C" fn z_query_check(query: &z_owned_query_t) -> bool {
    query.transmute_ref().is_some()
}
/// Borrows the query.
#[no_mangle]
pub extern "C" fn z_query_loan(this: &'static z_owned_query_t) -> &z_loaned_query_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}
/// Destroys the query resetting it to its gravestone value.
#[no_mangle]
pub extern "C" fn z_query_drop(this: &mut z_owned_query_t) {
    Inplace::drop(this.transmute_mut())
}
/// Constructs a shallow copy of the query, allowing to keep it in an "open" state past the callback's return.
///
/// This operation is infallible, but may return a gravestone value if `query` itself was a gravestone value (which cannot be the case in a callback).
#[no_mangle]
pub extern "C" fn z_query_clone(this: &z_loaned_query_t, dst: *mut MaybeUninit<z_owned_query_t>) {
    let this = this.transmute_ref();
    let this = this.clone();
    let dst = dst.transmute_uninit_ptr();
    Inplace::init(dst, Some(this));
}

/// Options passed to the `z_declare_queryable()` function.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_queryable_options_t {
    /// The completeness of the Queryable.
    pub complete: bool,
}
/// Constructs the default value for `z_query_reply_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_queryable_options_default(this: &mut z_queryable_options_t) {
    *this = z_queryable_options_t { complete: false };
}

/// Represents the set of options that can be applied to a query reply,
/// sent via `z_query_reply()`.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_reply_options_t {
    /// The encoding of the reply payload.
    pub encoding: *mut z_owned_encoding_t,
    /// The congestion control to apply when routing the reply.
    pub congestion_control: z_congestion_control_t,
    /// The priority of the reply.
    pub priority: z_priority_t,
    /// If true, Zenoh will not wait to batch this operation with others to reduce the bandwith.
    pub is_express: bool,
    /// The timestamp of the reply.
    pub timestamp: *mut z_timestamp_t,
    /// The source info for the reply.
    pub source_info: *mut z_owned_source_info_t,
    /// The attachment to this reply.
    pub attachment: *mut z_owned_bytes_t,
}

/// Constructs the default value for `z_query_reply_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_query_reply_options_default(this: &mut z_query_reply_options_t) {
    *this = z_query_reply_options_t {
        encoding: null_mut(),
        congestion_control: CongestionControl::Block.into(),
        priority: Priority::default().into(),
        is_express: false,
        timestamp: null_mut(),
        source_info: null_mut(),
        attachment: null_mut(),
    };
}

/// Represents the set of options that can be applied to a query reply error,
/// sent via `z_query_reply_err()`.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_reply_err_options_t {
    /// The encoding of the error payload.
    pub encoding: *mut z_owned_encoding_t,
}

/// Constructs the default value for `z_query_reply_err_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_query_reply_err_options_default(this: &mut z_query_reply_err_options_t) {
    *this = z_query_reply_err_options_t {
        encoding: null_mut(),
    };
}

/// Represents the set of options that can be applied to a query delete reply,
/// sent via `z_query_reply_del()`.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_reply_del_options_t {
    /// The congestion control to apply when routing the reply.
    pub congestion_control: z_congestion_control_t,
    /// The priority of the reply.
    pub priority: z_priority_t,
    /// If true, Zenoh will not wait to batch this operation with others to reduce the bandwith.
    pub is_express: bool,
    /// The timestamp of the reply.
    pub timestamp: *mut z_timestamp_t,
    /// The source info for the reply.
    pub source_info: *mut z_owned_source_info_t,
    /// The attachment to this reply.
    pub attachment: *mut z_owned_bytes_t,
}

/// Constructs the default value for `z_query_reply_del_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_query_reply_del_options_default(this: &mut z_query_reply_del_options_t) {
    *this = z_query_reply_del_options_t {
        congestion_control: CongestionControl::Block.into(),
        priority: Priority::default().into(),
        is_express: false,
        timestamp: null_mut(),
        source_info: null_mut(),
        attachment: null_mut(),
    };
}

/// Constructs a Queryable for the given key expression.
///
/// @param this_: An uninitialized memory location where queryable will be constructed.
/// @param session: The zenoh session.
/// @param key_expr: The key expression the Queryable will reply to.
/// @param callback: The callback function that will be called each time a matching query is received. Its ownership is passed to queryable.
/// @param options: Options for the queryable.
///
/// @return 0 in case of success, negative error code otherwise (in this case )
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_declare_queryable(
    this: *mut MaybeUninit<z_owned_queryable_t>,
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_owned_closure_query_t,
    options: Option<&mut z_queryable_options_t>,
) -> errors::z_error_t {
    let this = this.transmute_uninit_ptr();
    let mut closure = z_owned_closure_query_t::empty();
    std::mem::swap(&mut closure, callback);
    let session = session.transmute_ref();
    let keyexpr = key_expr.transmute_ref();
    let mut builder = session.declare_queryable(keyexpr);
    if let Some(options) = options {
        builder = builder.complete(options.complete);
    }
    let queryable = builder
        .callback(move |query| {
            z_closure_query_call(z_closure_query_loan(&closure), query.transmute_handle())
        })
        .wait();
    match queryable {
        Ok(q) => {
            Inplace::init(this, Some(q));
            errors::Z_OK
        }
        Err(e) => {
            log::error!("{}", e);
            errors::Z_EGENERIC
        }
    }
}

/// Undeclares a `z_owned_queryable_t` and drops it.
///
/// Returns 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_undeclare_queryable(this: &mut z_owned_queryable_t) -> errors::z_error_t {
    if let Some(qable) = this.transmute_mut().extract().take() {
        if let Err(e) = qable.undeclare().wait() {
            log::error!("{}", e);
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}

/// Frees memory and resets it to its gravesztone state. Will also attempt to undeclare queryable.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_queryable_drop(this: &mut z_owned_queryable_t) {
    z_undeclare_queryable(this);
}

/// Returns ``true`` if queryable is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_queryable_check(this: &z_owned_queryable_t) -> bool {
    this.transmute_ref().is_some()
}

/// Sends a reply to a query.
///
/// This function must be called inside of a Queryable callback passing the
/// query received as parameters of the callback function. This function can
/// be called multiple times to send multiple replies to a query. The reply
/// will be considered complete when the Queryable callback returns.
///
/// @param this_: The query to reply to.
/// @param key_expr: The key of this reply.
/// @param payload: The payload of this reply. Will be consumed.
/// @param options: The options of this reply. All owned fields will be consumed.
///
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query_reply(
    this: &z_loaned_query_t,
    key_expr: &z_loaned_keyexpr_t,
    payload: &mut z_owned_bytes_t,
    options: Option<&mut z_query_reply_options_t>,
) -> errors::z_error_t {
    let query = this.transmute_ref();
    let key_expr = key_expr.transmute_ref();
    let payload = payload.transmute_mut().extract();

    let mut reply = query.reply(key_expr, payload);
    if let Some(options) = options {
        if let Some(encoding) = unsafe { options.encoding.as_mut() } {
            let encoding = encoding.transmute_mut().extract();
            reply = reply.encoding(encoding);
        };
        if let Some(source_info) = unsafe { options.source_info.as_mut() } {
            let source_info = source_info.transmute_mut().extract();
            reply = reply.source_info(source_info);
        };
        if let Some(attachment) = unsafe { options.attachment.as_mut() } {
            let attachment = attachment.transmute_mut().extract();
            reply = reply.attachment(attachment);
        }
        if !options.timestamp.is_null() {
            let timestamp = *unsafe { options.timestamp.as_mut() }
                .unwrap()
                .transmute_ref();
            reply = reply.timestamp(Some(timestamp));
        }
        reply = reply.priority(options.priority.into());
        reply = reply.congestion_control(options.congestion_control.into());
        reply = reply.express(options.is_express);
    }

    if let Err(e) = reply.wait() {
        log::error!("{}", e);
        return errors::Z_EGENERIC;
    }
    errors::Z_OK
}

/// Sends a error reply to a query.
///
/// This function must be called inside of a Queryable callback passing the
/// query received as parameters of the callback function. This function can
/// be called multiple times to send multiple replies to a query. The reply
/// will be considered complete when the Queryable callback returns.
///
/// @param this_: The query to reply to.
/// @param payload: The payload carrying error message. Will be consumed.
/// @param options: The options of this reply. All owned fields will be consumed.
///
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_reply_err(
    this: &z_loaned_query_t,
    payload: &mut z_owned_bytes_t,
    options: Option<&mut z_query_reply_err_options_t>,
) -> errors::z_error_t {
    let query = this.transmute_ref();
    let payload = payload.transmute_mut().extract();

    let reply = query.reply_err(payload).encoding(
        options
            .and_then(|o| o.encoding.as_mut().map(|e| e.transmute_mut().extract()))
            .unwrap_or(Encoding::default()),
    );

    if let Err(e) = reply.wait() {
        log::error!("{}", e);
        return errors::Z_EGENERIC;
    }
    errors::Z_OK
}

/// Sends a delete reply to a query.
///
/// This function must be called inside of a Queryable callback passing the
/// query received as parameters of the callback function. This function can
/// be called multiple times to send multiple replies to a query. The reply
/// will be considered complete when the Queryable callback returns.
///
/// @param this: The query to reply to.
/// @param key_expr: The key of this delete reply.
/// @param options: The options of this delete reply. All owned fields will be consumed.
///
/// @return 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_reply_del(
    this: &z_loaned_query_t,
    key_expr: &z_loaned_keyexpr_t,
    options: Option<&mut z_query_reply_del_options_t>,
) -> errors::z_error_t {
    let query = this.transmute_ref();
    let key_expr = key_expr.transmute_ref();

    let mut reply = query.reply_del(key_expr);
    if let Some(options) = options {
        if let Some(source_info) = unsafe { options.source_info.as_mut() } {
            let source_info = source_info.transmute_mut().extract();
            reply = reply.source_info(source_info);
        };
        if let Some(attachment) = unsafe { options.attachment.as_mut() } {
            let attachment = attachment.transmute_mut().extract();
            reply = reply.attachment(attachment);
        }
        if !options.timestamp.is_null() {
            let timestamp = *unsafe { options.timestamp.as_mut() }
                .unwrap()
                .transmute_ref();
            reply = reply.timestamp(Some(timestamp));
        }
        reply = reply.priority(options.priority.into());
        reply = reply.congestion_control(options.congestion_control.into());
        reply = reply.express(options.is_express);
    }

    if let Err(e) = reply.wait() {
        log::error!("{}", e);
        return errors::Z_EGENERIC;
    }
    errors::Z_OK
}

/// Gets query key expression.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query_keyexpr(this: &z_loaned_query_t) -> &z_loaned_keyexpr_t {
    this.transmute_ref().key_expr().transmute_handle()
}

/// Gets query <a href="https://github.com/eclipse-zenoh/roadmap/tree/main/rfcs/ALL/Selectors">value selector</a>.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_parameters(
    this: &z_loaned_query_t,
    parameters: *mut MaybeUninit<z_view_string_t>,
) {
    let query = this.transmute_ref();
    let params = query.parameters().as_str();
    unsafe { z_view_string_from_substring(parameters, params.as_ptr() as _, params.len()) };
}

/// Gets query <a href="https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md">payload</a>.
///
/// Returns NULL if query does not contain a payload.
#[no_mangle]
pub extern "C" fn z_query_payload(this: &z_loaned_query_t) -> Option<&z_loaned_bytes_t> {
    this.transmute_ref().payload().map(|v| v.transmute_handle())
}

/// Gets query <a href="https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md">payload encoding</a>.
///
/// Returns NULL if query does not hame an encoding.
#[no_mangle]
pub extern "C" fn z_query_encoding(this: &z_loaned_query_t) -> Option<&z_loaned_encoding_t> {
    this.transmute_ref()
        .encoding()
        .map(|v| v.transmute_handle())
}

/// Gets query attachment.
///
/// Returns NULL if query does not contain an attachment.
#[no_mangle]
pub extern "C" fn z_query_attachment(this: &z_loaned_query_t) -> Option<&z_loaned_bytes_t> {
    this.transmute_ref()
        .attachment()
        .map(|a| a.transmute_handle())
}
