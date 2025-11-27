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
use std::mem::MaybeUninit;

use zenoh::{
    bytes::Encoding,
    handlers::Callback,
    qos::{CongestionControl, Priority},
    query::{Query, Queryable, QueryableBuilder},
    Wait,
};

pub use crate::opaque_types::{z_loaned_queryable_t, z_owned_queryable_t};
use crate::{
    result,
    transmute::{IntoRustType, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_query_call, z_closure_query_loan, z_congestion_control_t, z_loaned_bytes_t,
    z_loaned_encoding_t, z_loaned_keyexpr_t, z_loaned_session_t, z_locality_default, z_locality_t,
    z_moved_bytes_t, z_moved_closure_query_t, z_moved_encoding_t, z_moved_queryable_t,
    z_priority_t, z_timestamp_t, z_view_string_from_substr, z_view_string_t,
};
#[cfg(feature = "unstable")]
use crate::{transmute::IntoCType, z_entity_global_id_t, z_moved_source_info_t};
decl_c_type!(
    owned(z_owned_queryable_t, option Queryable<()>),
    loaned(z_loaned_queryable_t),
);

/// Constructs a queryable in its gravestone value.
#[no_mangle]
pub extern "C" fn z_internal_queryable_null(this_: &mut MaybeUninit<z_owned_queryable_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

// Borrows Queryable
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_queryable_loan(this_: &z_owned_queryable_t) -> &z_loaned_queryable_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

pub use crate::opaque_types::{z_loaned_query_t, z_moved_query_t, z_owned_query_t};
decl_c_type!(
    owned(z_owned_query_t, option Query),
    loaned(z_loaned_query_t),
);

/// Constructs query in its gravestone value.
#[no_mangle]
pub extern "C" fn z_internal_query_null(this_: &mut MaybeUninit<z_owned_query_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}
/// Returns `false` if `this` is in a gravestone state, `true` otherwise.
#[no_mangle]
pub extern "C" fn z_internal_query_check(query: &z_owned_query_t) -> bool {
    query.as_rust_type_ref().is_some()
}
/// Borrows the query.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_query_loan(
    this_: &'static z_owned_query_t,
) -> &'static z_loaned_query_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}
/// Mutably borrows the query.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_query_loan_mut(
    this_: &'static mut z_owned_query_t,
) -> &'static mut z_loaned_query_t {
    this_
        .as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// Takes ownership of the mutably borrowed query
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_query_take_from_loaned(
    dst: &mut MaybeUninit<z_owned_query_t>,
    src: &mut z_loaned_query_t,
) {
    let dst = dst.as_rust_type_mut_uninit();
    let src = src.as_rust_type_mut();
    let src = std::mem::replace(src, Query::empty());
    dst.write(Some(src));
}

/// Destroys the query resetting it to its gravestone value.
#[no_mangle]
pub extern "C" fn z_query_drop(this_: &mut z_moved_query_t) {
    let _ = this_.take_rust_type();
}
/// Constructs a shallow copy of the query, allowing to keep it in an "open" state past the callback's return.
///
/// This operation is infallible, but may return a gravestone value if `query` itself was a gravestone value (which cannot be the case in a callback).
#[no_mangle]
pub extern "C" fn z_query_clone(dst: &mut MaybeUninit<z_owned_query_t>, this_: &z_loaned_query_t) {
    dst.as_rust_type_mut_uninit()
        .write(Some(this_.as_rust_type_ref().clone()));
}

/// Options passed to the `z_declare_queryable()` function.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_queryable_options_t {
    /// The completeness of the Queryable.
    pub complete: bool,
    /// Restricts the matching requests that will be received by this Queryable to the ones
    /// that have the compatible allowed_destination.
    pub allowed_origin: z_locality_t,
}
/// Constructs the default value for `z_query_reply_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_queryable_options_default(this_: &mut MaybeUninit<z_queryable_options_t>) {
    this_.write(z_queryable_options_t {
        complete: false,
        allowed_origin: z_locality_default(),
    });
}

/// Represents the set of options that can be applied to a query reply,
/// sent via `z_query_reply()`.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_reply_options_t {
    /// The encoding of the reply payload.
    pub encoding: Option<&'static mut z_moved_encoding_t>,
    /// The congestion control to apply when routing the reply.
    pub congestion_control: z_congestion_control_t,
    /// The priority of the reply.
    pub priority: z_priority_t,
    /// If set to ``true``, this reply will not be batched. This usually has a positive impact on latency but negative impact on throughput.
    pub is_express: bool,
    /// The timestamp of the reply.
    pub timestamp: Option<&'static mut z_timestamp_t>,
    #[cfg(feature = "unstable")]
    /// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
    ///
    /// The source info for the reply.
    pub source_info: Option<&'static mut z_moved_source_info_t>,
    /// The attachment to this reply.
    pub attachment: Option<&'static mut z_moved_bytes_t>,
}

/// Constructs the default value for `z_query_reply_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_query_reply_options_default(this_: &mut MaybeUninit<z_query_reply_options_t>) {
    this_.write(z_query_reply_options_t {
        encoding: None,
        congestion_control: CongestionControl::DEFAULT_RESPONSE.into(),
        priority: Priority::default().into(),
        is_express: false,
        timestamp: None,
        #[cfg(feature = "unstable")]
        source_info: None,
        attachment: None,
    });
}

/// Represents the set of options that can be applied to a query reply error,
/// sent via `z_query_reply_err()`.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_reply_err_options_t {
    /// The encoding of the error payload.
    pub encoding: Option<&'static mut z_moved_encoding_t>,
}

/// Constructs the default value for `z_query_reply_err_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_query_reply_err_options_default(
    this: &mut MaybeUninit<z_query_reply_err_options_t>,
) {
    this.write(z_query_reply_err_options_t { encoding: None });
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
    /// If set to ``true``, this reply will not be batched. This usually has a positive impact on latency but negative impact on throughput.
    pub is_express: bool,
    /// The timestamp of the reply.
    pub timestamp: Option<&'static mut z_timestamp_t>,
    #[cfg(feature = "unstable")]
    /// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
    ///
    /// The source info for the reply.
    pub source_info: Option<&'static mut z_moved_source_info_t>,
    /// The attachment to this reply.
    pub attachment: Option<&'static mut z_moved_bytes_t>,
}

/// Constructs the default value for `z_query_reply_del_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_query_reply_del_options_default(
    this: &mut MaybeUninit<z_query_reply_del_options_t>,
) {
    this.write(z_query_reply_del_options_t {
        congestion_control: CongestionControl::DEFAULT_RESPONSE.into(),
        priority: Priority::default().into(),
        is_express: false,
        timestamp: None,
        #[cfg(feature = "unstable")]
        source_info: None,
        attachment: None,
    });
}

fn _declare_queryable_inner<'a, 'b>(
    session: &'a z_loaned_session_t,
    key_expr: &'b z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_query_t,
    options: Option<&mut z_queryable_options_t>,
) -> QueryableBuilder<'a, 'b, Callback<Query>> {
    let session = session.as_rust_type_ref();
    let keyexpr = key_expr.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let mut builder = session.declare_queryable(keyexpr);
    if let Some(options) = options {
        builder = builder
            .complete(options.complete)
            .allowed_origin(options.allowed_origin.into());
    }
    let queryable = builder.callback(move |query| {
        let mut owned_query = Some(query);
        z_closure_query_call(z_closure_query_loan(&callback), unsafe {
            owned_query
                .as_mut()
                .unwrap_unchecked()
                .as_loaned_c_type_mut()
        })
    });
    queryable
}

/// Constructs a Queryable for the given key expression.
///
/// @param session: A Zenoh session.
/// @param queryable: An uninitialized memory location where queryable will be constructed.
/// @param key_expr: The key expression the Queryable will reply to.
/// @param callback: The callback function that will be called each time a matching query is received. Its ownership is passed to queryable.
/// @param options: Options for the queryable.
///
/// @return 0 in case of success, negative error code otherwise (in this case )
#[no_mangle]
pub extern "C" fn z_declare_queryable(
    session: &z_loaned_session_t,
    queryable: &mut MaybeUninit<z_owned_queryable_t>,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_query_t,
    options: Option<&mut z_queryable_options_t>,
) -> result::z_result_t {
    let this = queryable.as_rust_type_mut_uninit();
    let queryable = _declare_queryable_inner(session, key_expr, callback, options);
    match queryable.wait() {
        Ok(q) => {
            this.write(Some(q));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
    }
}

/// Declares a background queryable for a given keyexpr. The queryable callback will be be called
/// to proccess incoming queries until the corresponding session is closed or dropped.
///
/// @param session: The zenoh session.
/// @param key_expr: The key expression the Queryable will reply to.
/// @param callback: The callback function that will be called each time a matching query is received. Its ownership is passed to queryable.
/// @param options: Options for the queryable.
///
/// @return 0 in case of success, negative error code otherwise (in this case )
#[no_mangle]
pub extern "C" fn z_declare_background_queryable(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_query_t,
    options: Option<&mut z_queryable_options_t>,
) -> result::z_result_t {
    let queryable = _declare_queryable_inner(session, key_expr, callback, options);
    match queryable.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// Undeclares queryable callback and resets it to its gravestone state.
/// This is equivalent to calling `z_undeclare_queryable()` and discarding its return value.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_queryable_drop(this_: &mut z_moved_queryable_t) {
    std::mem::drop(this_.take_rust_type())
}

/// Returns ``true`` if queryable is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_internal_queryable_check(this_: &z_owned_queryable_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the ID of the queryable.
#[no_mangle]
pub extern "C" fn z_queryable_id(queryable: &z_loaned_queryable_t) -> z_entity_global_id_t {
    queryable.as_rust_type_ref().id().into_c_type()
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
    payload: &mut z_moved_bytes_t,
    options: Option<&mut z_query_reply_options_t>,
) -> result::z_result_t {
    let query = this.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let payload = payload.take_rust_type();
    let mut reply = query.reply(key_expr, payload);
    if let Some(options) = options {
        if let Some(encoding) = options.encoding.take() {
            reply = reply.encoding(encoding.take_rust_type());
        };
        #[cfg(feature = "unstable")]
        if let Some(source_info) = options.source_info.take() {
            reply = reply.source_info(source_info.take_rust_type());
        };
        if let Some(attachment) = options.attachment.take() {
            reply = reply.attachment(attachment.take_rust_type());
        }
        if let Some(timestamp) = options.timestamp.as_ref() {
            reply = reply.timestamp(Some(timestamp.into_rust_type()));
        }
        reply = reply.priority(options.priority.into());
        reply = reply.congestion_control(options.congestion_control.into());
        reply = reply.express(options.is_express);
    }

    if let Err(e) = reply.wait() {
        crate::report_error!("{}", e);
        return result::Z_EGENERIC;
    }
    result::Z_OK
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
    payload: &mut z_moved_bytes_t,
    options: Option<&mut z_query_reply_err_options_t>,
) -> result::z_result_t {
    let query = this.as_rust_type_ref();
    let payload = payload.take_rust_type();
    let reply = query.reply_err(payload).encoding(
        options
            .and_then(|o| o.encoding.take())
            .map(|e| e.take_rust_type())
            .unwrap_or(Encoding::default()),
    );

    if let Err(e) = reply.wait() {
        crate::report_error!("{}", e);
        return result::Z_EGENERIC;
    }
    result::Z_OK
}

/// Sends a delete reply to a query.
///
/// This function must be called inside of a Queryable callback passing the
/// query received as parameters of the callback function. This function can
/// be called multiple times to send multiple replies to a query. The reply
/// will be considered complete when the Queryable callback returns.
///
/// @param this_: The query to reply to.
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
) -> result::z_result_t {
    let query = this.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();

    let mut reply = query.reply_del(key_expr);
    if let Some(options) = options {
        #[cfg(feature = "unstable")]
        if let Some(source_info) = options.source_info.take() {
            reply = reply.source_info(source_info.take_rust_type());
        };
        if let Some(attachment) = options.attachment.take() {
            reply = reply.attachment(attachment.take_rust_type());
        }
        if let Some(timestamp) = options.timestamp.as_ref() {
            reply = reply.timestamp(Some(timestamp.into_rust_type()));
        }
        reply = reply.priority(options.priority.into());
        reply = reply.congestion_control(options.congestion_control.into());
        reply = reply.express(options.is_express);
    }

    if let Err(e) = reply.wait() {
        crate::report_error!("{}", e);
        return result::Z_EGENERIC;
    }
    result::Z_OK
}

/// Gets query key expression.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query_keyexpr(this_: &z_loaned_query_t) -> &z_loaned_keyexpr_t {
    this_.as_rust_type_ref().key_expr().as_loaned_c_type_ref()
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
pub extern "C" fn z_query_payload(this_: &z_loaned_query_t) -> Option<&z_loaned_bytes_t> {
    this_
        .as_rust_type_ref()
        .payload()
        .map(|v| v.as_loaned_c_type_ref())
}

/// Gets mutable query <a href="https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md">payload</a>.
///
/// Returns NULL if query does not contain a payload.
#[no_mangle]
pub extern "C" fn z_query_payload_mut(
    this_: &mut z_loaned_query_t,
) -> Option<&mut z_loaned_bytes_t> {
    this_
        .as_rust_type_mut()
        .payload_mut()
        .map(|v| v.as_loaned_c_type_mut())
}

/// Gets query <a href="https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md">payload encoding</a>.
///
/// Returns NULL if query does not contain an encoding.
#[no_mangle]
pub extern "C" fn z_query_encoding(this_: &z_loaned_query_t) -> Option<&z_loaned_encoding_t> {
    this_
        .as_rust_type_ref()
        .encoding()
        .map(|v| v.as_loaned_c_type_ref())
}

/// Gets query attachment.
///
/// Returns NULL if query does not contain an attachment.
#[no_mangle]
pub extern "C" fn z_query_attachment(this_: &z_loaned_query_t) -> Option<&z_loaned_bytes_t> {
    this_
        .as_rust_type_ref()
        .attachment()
        .map(|a| a.as_loaned_c_type_ref())
}

/// Gets mutable query attachment.
///
/// Returns NULL if query does not contain an attachment.
#[no_mangle]
pub extern "C" fn z_query_attachment_mut(
    this_: &mut z_loaned_query_t,
) -> Option<&mut z_loaned_bytes_t> {
    this_
        .as_rust_type_mut()
        .attachment_mut()
        .map(|a| a.as_loaned_c_type_mut())
}

/// Undeclares a `z_owned_queryable_t`.
/// Returns 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_undeclare_queryable(this_: &mut z_moved_queryable_t) -> result::z_result_t {
    if let Some(qable) = this_.take_rust_type() {
        if let Err(e) = qable.undeclare().wait() {
            crate::report_error!("{}", e);
            return result::Z_EGENERIC;
        }
    }
    result::Z_OK
}

/// @brief Returns the key expression of the queryable.
#[no_mangle]
pub extern "C" fn z_queryable_keyexpr(queryable: &z_loaned_queryable_t) -> &z_loaned_keyexpr_t {
    queryable
        .as_rust_type_ref()
        .key_expr()
        .as_loaned_c_type_ref()
}
