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
use crate::transmute::{IntoRustType, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit};
use crate::{
    errors, z_closure_query_call, z_closure_query_loan, z_congestion_control_t, z_loaned_bytes_t,
    z_loaned_encoding_t, z_loaned_keyexpr_t, z_loaned_session_t, z_moved_bytes_t,
    z_moved_closure_query_t, z_owned_bytes_t, z_owned_encoding_t, z_owned_source_info_t,
    z_priority_t, z_timestamp_t, z_view_string_from_substr, z_view_string_t,
};
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use zenoh::core::Priority;
use zenoh::core::Wait;
use zenoh::encoding::Encoding;
use zenoh::prelude::SessionDeclarations;
use zenoh::publisher::CongestionControl;
use zenoh::query::Query;
use zenoh::queryable::Queryable;
use zenoh::sample::{
    EncodingBuilderTrait, QoSBuilderTrait, SampleBuilderTrait, TimestampBuilderTrait,
};

pub use crate::opaque_types::z_loaned_queryable_t;
pub use crate::opaque_types::z_moved_queryable_t;
pub use crate::opaque_types::z_owned_queryable_t;
decl_c_type!(
    owned(z_owned_queryable_t, option Queryable<'static, ()>),
    loaned(z_loaned_queryable_t),
    moved(z_moved_queryable_t)
);

/// Constructs a queryable in its gravestone value.
#[no_mangle]
pub extern "C" fn z_queryable_null(this: &mut MaybeUninit<z_owned_queryable_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

// Borrows Queryable
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_queryable_loan(this: &z_owned_queryable_t) -> &z_loaned_queryable_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

pub use crate::opaque_types::z_loaned_query_t;
pub use crate::opaque_types::z_moved_query_t;
pub use crate::opaque_types::z_owned_query_t;
decl_c_type!(
    owned(z_owned_query_t, Option<Query>),
    loaned(z_loaned_query_t, Query),
    moved(z_moved_query_t)
);

/// Constructs query in its gravestone value.
#[no_mangle]
pub extern "C" fn z_query_null(this: &mut MaybeUninit<z_owned_query_t>) {
    this.as_rust_type_mut_uninit().write(None);
}
/// Returns `false` if `this` is in a gravestone state, `true` otherwise.
#[no_mangle]
pub extern "C" fn z_query_check(query: &z_owned_query_t) -> bool {
    query.as_rust_type_ref().is_some()
}
/// Borrows the query.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_query_loan(this: &'static z_owned_query_t) -> &z_loaned_query_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}
/// Destroys the query resetting it to its gravestone value.
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn z_query_drop(this: z_moved_query_t) {}
/// Constructs a shallow copy of the query, allowing to keep it in an "open" state past the callback's return.
///
/// This operation is infallible, but may return a gravestone value if `query` itself was a gravestone value (which cannot be the case in a callback).
#[no_mangle]
pub extern "C" fn z_query_clone(this: &z_loaned_query_t, dst: &mut MaybeUninit<z_owned_query_t>) {
    dst.as_rust_type_mut_uninit()
        .write(Some(this.as_rust_type_ref().clone()));
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
    this: &mut MaybeUninit<z_owned_queryable_t>,
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: z_moved_closure_query_t,
    options: Option<&mut z_queryable_options_t>,
) -> errors::z_error_t {
    let this = this.as_rust_type_mut_uninit();
    let session = session.as_rust_type_ref();
    let keyexpr = key_expr.as_rust_type_ref();
    let Some(callback) = callback.into_rust_type() else {
        return errors::Z_EINVAL;
    };
    let mut builder = session.declare_queryable(keyexpr);
    if let Some(options) = options {
        builder = builder.complete(options.complete);
    }
    let queryable = builder
        .callback(move |query| {
            z_closure_query_call(
                z_closure_query_loan(&callback),
                query.as_loaned_c_type_ref(),
            )
        })
        .wait();
    match queryable {
        Ok(q) => {
            this.write(Some(q));
            errors::Z_OK
        }
        Err(e) => {
            log::error!("{}", e);
            this.write(None);
            errors::Z_EGENERIC
        }
    }
}

/// Undeclares a `z_owned_queryable_t` and drops it.
///
/// Returns 0 in case of success, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_undeclare_queryable(this: z_moved_queryable_t) -> errors::z_error_t {
    if let Some(qable) = this.into_rust_type() {
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
pub extern "C" fn z_queryable_drop(this: z_moved_queryable_t) {
    z_undeclare_queryable(this);
}

/// Returns ``true`` if queryable is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_queryable_check(this: &z_owned_queryable_t) -> bool {
    this.as_rust_type_ref().is_some()
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
    payload: z_moved_bytes_t,
    options: Option<&mut z_query_reply_options_t>,
) -> errors::z_error_t {
    let query = this.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let Some(payload) = payload.into_rust_type() else {
        return errors::Z_EINVAL;
    };

    let mut reply = query.reply(key_expr, payload);
    if let Some(options) = options {
        if let Some(encoding) = unsafe { options.encoding.as_mut() } {
            let encoding = std::mem::take(encoding.as_rust_type_mut());
            reply = reply.encoding(encoding);
        };
        if let Some(source_info) = unsafe { options.source_info.as_mut() } {
            let source_info = std::mem::take(source_info.as_rust_type_mut());
            reply = reply.source_info(source_info);
        };
        if let Some(attachment) = unsafe { options.attachment.as_mut() } {
            let attachment = std::mem::take(attachment.as_rust_type_mut());
            reply = reply.attachment(attachment);
        }
        if !options.timestamp.is_null() {
            let timestamp = *unsafe { options.timestamp.as_mut() }
                .unwrap()
                .as_rust_type_ref();
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
    payload: z_moved_bytes_t,
    options: Option<&mut z_query_reply_err_options_t>,
) -> errors::z_error_t {
    let query = this.as_rust_type_ref();
    let Some(payload) = payload.into_rust_type() else {
        return errors::Z_EINVAL;
    };

    let reply = query.reply_err(payload).encoding(
        options
            .and_then(|o| {
                o.encoding
                    .as_mut()
                    .map(|e| std::mem::take(e.as_rust_type_mut()))
            })
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
    let query = this.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();

    let mut reply = query.reply_del(key_expr);
    if let Some(options) = options {
        if let Some(source_info) = unsafe { options.source_info.as_mut() } {
            let source_info = std::mem::take(source_info.as_rust_type_mut());
            reply = reply.source_info(source_info);
        };
        if let Some(attachment) = unsafe { options.attachment.as_mut() } {
            let attachment = std::mem::take(attachment.as_rust_type_mut());
            reply = reply.attachment(attachment);
        }
        if !options.timestamp.is_null() {
            let timestamp = *unsafe { options.timestamp.as_mut() }
                .unwrap()
                .as_rust_type_ref();
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
    this.as_rust_type_ref().key_expr().as_loaned_c_type_ref()
}

/// Gets query <a href="https://github.com/eclipse-zenoh/roadmap/tree/main/rfcs/ALL/Selectors">value selector</a>.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_parameters(
    this: &z_loaned_query_t,
    parameters: &mut MaybeUninit<z_view_string_t>,
) {
    let query = this.as_rust_type_ref();
    let params = query.parameters().as_str();
    unsafe { z_view_string_from_substr(parameters, params.as_ptr() as _, params.len()) };
}

/// Gets query <a href="https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md">payload</a>.
///
/// Returns NULL if query does not contain a payload.
#[no_mangle]
pub extern "C" fn z_query_payload(this: &z_loaned_query_t) -> Option<&z_loaned_bytes_t> {
    this.as_rust_type_ref()
        .payload()
        .map(|v| v.as_loaned_c_type_ref())
}

/// Gets query <a href="https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md">payload encoding</a>.
///
/// Returns NULL if query does not hame an encoding.
#[no_mangle]
pub extern "C" fn z_query_encoding(this: &z_loaned_query_t) -> Option<&z_loaned_encoding_t> {
    this.as_rust_type_ref()
        .encoding()
        .map(|v| v.as_loaned_c_type_ref())
}

/// Gets query attachment.
///
/// Returns NULL if query does not contain an attachment.
#[no_mangle]
pub extern "C" fn z_query_attachment(this: &z_loaned_query_t) -> Option<&z_loaned_bytes_t> {
    this.as_rust_type_ref()
        .attachment()
        .map(|a| a.as_loaned_c_type_ref())
}
