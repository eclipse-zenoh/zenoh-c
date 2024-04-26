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
    errors, z_bytes_t, z_closure_query_call, z_keyexpr_t, z_owned_bytes_t, z_owned_closure_query_t,
    z_owned_encoding_t, z_session_t, z_value_t, z_view_slice_t, z_view_slice_wrap,
};
use std::mem::MaybeUninit;
use std::ptr::{null, null_mut};
use zenoh::prelude::SessionDeclarations;
use zenoh::prelude::SyncResolve;
use zenoh::prelude::{Query, Queryable};
use zenoh::sample::{SampleBuilderTrait, ValueBuilderTrait};

pub use crate::opaque_types::z_owned_queryable_t;
decl_transmute_owned!(Option<Queryable<'static, ()>>, z_owned_queryable_t);

/// Constructs a null safe-to-drop value of 'z_owned_queryable_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_queryable_null(this: *mut MaybeUninit<z_owned_queryable_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

pub use crate::opaque_types::z_query_t;
decl_transmute_handle!(Query, z_query_t);

/// Owned variant of a Query received by a Queryable.
///
/// You may construct it by `z_query_clone`-ing a loaned query.
/// When the last `z_owned_query_t` corresponding to a query is destroyed, or the callback that produced the query cloned to build them returns,
/// the query will receive its termination signal.
///
/// Holding onto an `z_owned_query_t` for too long (10s by default, can be set in `z_get`'s options) will trigger a timeout error
/// to be sent to the querier by the infrastructure, and new responses to the outdated query will be silently dropped.
pub use crate::opaque_types::z_owned_query_t;
decl_transmute_owned!(Option<Query>, z_owned_query_t);

/// The gravestone value of `z_owned_query_t`.
#[no_mangle]
pub extern "C" fn z_query_null(this: *mut MaybeUninit<z_owned_query_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}
/// Returns `false` if `this` is in a gravestone state, `true` otherwise.
///
/// This function may not be called with the null pointer, but can be called with the gravestone value.
#[no_mangle]
pub extern "C" fn z_query_check(query: &z_owned_query_t) -> bool {
    query.transmute_ref().is_some()
}
/// Aliases the query.
///
/// This function may not be called with the null pointer, but can be called with the gravestone value.
#[no_mangle]
pub extern "C" fn z_query_loan(this: &'static z_owned_query_t) -> &z_query_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}
/// Destroys the query, setting `this` to its gravestone value to prevent double-frees.
///
/// This function may not be called with the null pointer, but can be called with the gravestone value.
#[no_mangle]
pub extern "C" fn z_query_drop(this: &mut z_owned_query_t) {
    Inplace::drop(this.transmute_mut())
}
/// Clones the query, allowing to keep it in an "open" state past the callback's return.
///
/// This operation is infallible, but may return a gravestone value if `query` itself was a gravestone value (which cannot be the case in a callback).
#[no_mangle]
pub extern "C" fn z_query_clone(this: &z_query_t, dst: *mut MaybeUninit<z_owned_query_t>) {
    let this = this.transmute_ref();
    let this = this.clone();
    let dst = dst.transmute_uninit_ptr();
    Inplace::init(dst, Some(this));
}

/// Options passed to the :c:func:`z_declare_queryable` function.
///
/// Members:
///     bool complete: The completeness of the Queryable.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_queryable_options_t {
    pub complete: bool,
}
/// Constructs the default value for :c:type:`z_query_reply_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_queryable_options_default(this: &mut z_queryable_options_t) {
    *this = z_queryable_options_t { complete: false };
}

/// Represents the set of options that can be applied to a query reply,
/// sent via :c:func:`z_query_reply`.
///
/// Members:
///   z_owned_encoding_t encoding: The encoding of the payload.
///  z_owned_bytes_t attachment: The attachment to this reply.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_reply_options_t {
    pub encoding: *mut z_owned_encoding_t,
    pub attachment: *mut z_owned_bytes_t,
}

/// Constructs the default value for :c:type:`z_query_reply_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_query_reply_options_default(this: &mut z_query_reply_options_t) {
    *this = z_query_reply_options_t {
        encoding: null_mut(),
        attachment: null_mut(),
    };
}

/// Creates a Queryable for the given key expression.
///
/// Parameters:
///     session: The zenoh session.
///     key_expr: The key expression the Queryable will reply to.
///     callback: The callback function that will be called each time a matching query is received.
///     options: Options for the queryable.
///
/// Returns:
///    The created :c:type:`z_owned_queryable_t` or ``null`` if the creation failed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_declare_queryable(
    this: *mut MaybeUninit<z_owned_queryable_t>,
    session: &z_session_t,
    key_expr: &z_keyexpr_t,
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
        .callback(move |query| z_closure_query_call(&closure, query.transmute_handle()))
        .res_sync();
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

/// Undeclares a `z_owned_queryable_t`, droping it and invalidating it for doube-drop safety.
///
/// Parameters:
///     qable: The :c:type:`z_owned_queryable_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_undeclare_queryable(qable: &mut z_owned_queryable_t) -> errors::z_error_t {
    if let Some(qable) = qable.transmute_mut().extract().take() {
        if let Err(e) = qable.undeclare().res_sync() {
            log::error!("{}", e);
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}

/// Returns ``true`` if `qable` is valid.
#[no_mangle]
pub extern "C" fn z_queryable_check(qable: &z_owned_queryable_t) -> bool {
    qable.transmute_ref().is_some()
}

/// Send a reply to a query.
///
/// This function must be called inside of a Queryable callback passing the
/// query received as parameters of the callback function. This function can
/// be called multiple times to send multiple replies to a query. The reply
/// will be considered complete when the Queryable callback returns.
///
/// Parameters:
///     query: The query to reply to.
///     key_expr: The key of this reply.
///     payload: The value of this reply.
///     options: The options of this reply.
///
/// The payload and all owned options fields are consumed upon function return.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_reply(
    query: z_query_t,
    key_expr: z_keyexpr_t,
    payload: &mut z_owned_bytes_t,
    options: Option<&mut z_query_reply_options_t>,
) -> errors::z_error_t {
    let query = query.transmute_ref();
    let key_expr = key_expr.transmute_ref();

    let payload = match payload.transmute_mut().extract() {
        Some(p) => p,
        None => {
            log::debug!("Attempted to reply with a null payload");
            return errors::Z_EINVAL;
        }
    };

    let mut reply = query.reply(key_expr, payload);
    if let Some(options) = options {
        if !options.encoding.is_null() {
            let encoding = unsafe { *options.encoding }.transmute_mut().extract();
            reply = reply.encoding(encoding);
        };
        if !options.attachment.is_null() {
            let attachment = unsafe { *options.attachment }.transmute_mut().extract();
            reply = reply.attachment(attachment);
        }
    }

    if let Err(e) = reply.res_sync() {
        log::error!("{}", e);
        return errors::Z_EGENERIC;
    }
    errors::Z_OK
}

/// Get a query's key by aliasing it.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query_keyexpr(query: &z_query_t) -> &z_keyexpr_t {
    query.transmute_ref().key_expr().transmute_handle()
}

/// Get a query's `value selector <https://github.com/eclipse-zenoh/roadmap/tree/main/rfcs/ALL/Selectors>`_ by aliasing it.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_parameters(
    query: &z_query_t,
    parameters: *mut MaybeUninit<z_view_slice_t>,
) {
    let query = query.transmute_ref();
    let params = query.parameters().as_str();
    unsafe { z_view_slice_wrap(parameters, params.as_ptr(), params.len()) };
}

/// Checks if query contains a payload value.
#[no_mangle]
pub extern "C" fn z_query_has_value(query: &z_query_t) -> bool {
    query.transmute_ref().value().is_some()
}

/// Gets a query's `payload value <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md>`_ by aliasing it.
///
/// **WARNING: This API has been marked as unstable: it works as advertised, but it may change in a future release.**
/// Before calling this funciton, the user must ensure that `z_query_has_value` returns true.
#[no_mangle]
pub extern "C" fn z_query_value(query: &z_query_t) -> &z_value_t {
    query
        .transmute_ref()
        .value()
        .expect("Query does not contain a value")
        .transmute_handle()
}

/// Gets the attachment to the query by aliasing.
///
/// Returns NULL if query does not contain an attachment.
#[no_mangle]
pub extern "C" fn z_query_attachment(query: &z_query_t) -> *const z_bytes_t {
    match query.transmute_ref().attachment() {
        Some(attachment) => attachment.transmute_handle() as *const _,
        None => null(),
    }
}
