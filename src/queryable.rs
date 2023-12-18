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
use crate::attachment::{
    attachment_iteration_driver, insert_in_attachment_builder, z_attachment_check,
    z_attachment_iterate, z_attachment_null, z_attachment_t,
};
use crate::{
    impl_guarded_transmute, z_bytes_t, z_closure_query_call, z_encoding_default, z_encoding_t,
    z_keyexpr_t, z_owned_closure_query_t, z_session_t, z_value_t, GuardedTransmute,
    LOG_INVALID_SESSION,
};
use libc::c_void;
use std::ops::{Deref, DerefMut};
use zenoh::prelude::SessionDeclarations;
use zenoh::{
    prelude::{Sample, SplitBuffer},
    queryable::{Query, Queryable as CallbackQueryable},
    sample::AttachmentBuilder,
    value::Value,
};
use zenoh_util::core::{zresult::ErrNo, SyncResolve};

type Queryable = Option<CallbackQueryable<'static, ()>>;
/// An owned zenoh queryable.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[cfg(not(target_arch = "arm"))]
#[repr(C, align(8))]
pub struct z_owned_queryable_t([u64; 4]);

#[cfg(target_arch = "arm")]
#[repr(C, align(4))]
pub struct z_owned_queryable_t([u32; 4]);

impl_guarded_transmute!(Queryable, z_owned_queryable_t);

impl From<Queryable> for z_owned_queryable_t {
    fn from(val: Queryable) -> Self {
        val.transmute()
    }
}
impl AsRef<Queryable> for z_owned_queryable_t {
    fn as_ref(&self) -> &Queryable {
        unsafe { std::mem::transmute(self) }
    }
}
impl AsMut<Queryable> for z_owned_queryable_t {
    fn as_mut(&mut self) -> &mut Queryable {
        unsafe { std::mem::transmute(self) }
    }
}

impl z_owned_queryable_t {
    pub fn null() -> Self {
        None.into()
    }
}

/// Constructs a null safe-to-drop value of 'z_owned_queryable_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_queryable_null() -> z_owned_queryable_t {
    z_owned_queryable_t::null()
}

/// Loaned variant of a Query received by a Queryable.
///
/// Queries are atomically reference-counted, letting you extract them from the callback that handed them to you by cloning.
/// `z_query_t`'s are valid as long as at least one corresponding `z_owned_query_t` exists, including the one owned by Zenoh until the callback returns.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_t(*mut c_void);
impl From<&Query> for z_query_t {
    fn from(value: &Query) -> Self {
        z_query_t(value as *const _ as *mut _)
    }
}
impl From<Option<&Query>> for z_query_t {
    fn from(value: Option<&Query>) -> Self {
        value.map_or(Self(core::ptr::null_mut()), Into::into)
    }
}
impl Deref for z_query_t {
    type Target = Option<&'static Query>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

/// Owned variant of a Query received by a Queryable.
///
/// You may construct it by `z_query_clone`-ing a loaned query.
/// When the last `z_owned_query_t` corresponding to a query is destroyed, or the callback that produced the query cloned to build them returns,
/// the query will receive its termination signal.
///
/// Holding onto an `z_owned_query_t` for too long (10s by default, can be set in `z_get`'s options) will trigger a timeout error
/// to be sent to the querier by the infrastructure, and new responses to the outdated query will be silently dropped.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_owned_query_t(*mut c_void);

impl From<Option<Query>> for z_owned_query_t {
    fn from(value: Option<Query>) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<Query> for z_owned_query_t {
    fn from(value: Query) -> Self {
        Some(value).into()
    }
}
impl Deref for z_owned_query_t {
    type Target = Option<Query>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl DerefMut for z_owned_query_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl Drop for z_owned_query_t {
    fn drop(&mut self) {
        let _: Option<Query> = self.take();
    }
}
/// The gravestone value of `z_owned_query_t`.
#[no_mangle]
pub extern "C" fn z_query_null() -> z_owned_query_t {
    unsafe { core::mem::transmute(None::<Query>) }
}
/// Returns `false` if `this` is in a gravestone state, `true` otherwise.
///
/// This function may not be called with the null pointer, but can be called with the gravestone value.
#[no_mangle]
pub extern "C" fn z_query_check(this: &z_owned_query_t) -> bool {
    this.is_some()
}
/// Aliases the query.
///
/// This function may not be called with the null pointer, but can be called with the gravestone value.
#[no_mangle]
pub extern "C" fn z_query_loan(this: &z_owned_query_t) -> z_query_t {
    this.as_ref().into()
}
/// Destroys the query, setting `this` to its gravestone value to prevent double-frees.
///
/// This function may not be called with the null pointer, but can be called with the gravestone value.
#[no_mangle]
pub extern "C" fn z_query_drop(this: &mut z_owned_query_t) {
    let _: Option<Query> = this.take();
}
/// Clones the query, allowing to keep it in an "open" state past the callback's return.
///
/// This operation is infallible, but may return a gravestone value if `query` itself was a gravestone value (which cannot be the case in a callback).
#[no_mangle]
pub extern "C" fn z_query_clone(query: Option<&z_query_t>) -> z_owned_query_t {
    query.and_then(|q| q.cloned()).into()
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
pub extern "C" fn z_queryable_options_default() -> z_queryable_options_t {
    z_queryable_options_t { complete: false }
}

/// Represents the set of options that can be applied to a query reply,
/// sent via :c:func:`z_query_reply`.
///
/// Members:
///   z_encoding_t encoding: The encoding of the payload.
///   z_attachment_t attachment: The attachment to this reply.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_reply_options_t {
    pub encoding: z_encoding_t,
    pub attachment: z_attachment_t,
}

/// Constructs the default value for :c:type:`z_query_reply_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_query_reply_options_default() -> z_query_reply_options_t {
    z_query_reply_options_t {
        encoding: z_encoding_default(),
        attachment: z_attachment_null(),
    }
}

/// Creates a Queryable for the given key expression.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression the Queryable will reply to.
///     callback: The callback function that will be called each time a matching query is received.
///     options: Options for the queryable.
///
/// Returns:
///    The created :c:type:`z_owned_queryable_t` or ``null`` if the creation failed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_declare_queryable(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    callback: &mut z_owned_closure_query_t,
    options: Option<&z_queryable_options_t>,
) -> z_owned_queryable_t {
    let mut closure = z_owned_closure_query_t::empty();
    std::mem::swap(&mut closure, callback);

    let session = match session.upgrade() {
        Some(s) => s,
        None => {
            log::error!("{}", LOG_INVALID_SESSION);
            return None.into();
        }
    };
    let mut builder = session.declare_queryable(keyexpr);
    if let Some(options) = options {
        builder = builder.complete(options.complete);
    }
    builder
        .callback(move |query| z_closure_query_call(&closure, &z_query_t::from(&query)))
        .res_sync()
        .map_err(|e| log::error!("{}", e))
        .ok()
        .into()
}

/// Undeclares a `z_owned_queryable_t`, droping it and invalidating it for doube-drop safety.
///
/// Parameters:
///     qable: The :c:type:`z_owned_queryable_t` to undeclare.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_undeclare_queryable(qable: &mut z_owned_queryable_t) -> i8 {
    if let Some(qable) = qable.as_mut().take() {
        if let Err(e) = qable.undeclare().res_sync() {
            log::error!("{}", e);
            return e.errno().get();
        }
    }
    0
}

/// Returns ``true`` if `qable` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_queryable_check(qable: &z_owned_queryable_t) -> bool {
    qable.as_ref().is_some()
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
///     key: The key of this reply.
///     payload: The value of this reply.
///     len: The length of the value of this reply.
///     options: The options of this reply.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_reply(
    query: &z_query_t,
    key: z_keyexpr_t,
    payload: *const u8,
    len: usize,
    options: Option<&z_query_reply_options_t>,
) -> i8 {
    let Some(query) = query.as_ref() else {
        log::error!("Called `z_query_reply` with invalidated `query`");
        return i8::MIN;
    };
    if let Some(key) = &*key {
        let mut s = Sample::new(
            key.clone().into_owned(),
            std::slice::from_raw_parts(payload, len),
        );
        if let Some(o) = options {
            s.encoding = o.encoding.into();
            if z_attachment_check(&o.attachment) {
                let mut attachment_builder = AttachmentBuilder::new();
                z_attachment_iterate(
                    o.attachment,
                    insert_in_attachment_builder,
                    &mut attachment_builder as *mut AttachmentBuilder as *mut c_void,
                );
                s = s.with_attachment(attachment_builder.build());
            };
        }
        if let Err(e) = query.reply(Ok(s)).res_sync() {
            log::error!("{}", e);
            return e.errno().get();
        }
        0
    } else {
        i8::MIN
    }
}

/// Get a query's key by aliasing it.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query_keyexpr(query: &z_query_t) -> z_keyexpr_t {
    let Some(query) = query.as_ref() else {
        return z_keyexpr_t::null();
    };
    query.key_expr().borrowing_clone().into()
}

/// Get a query's `value selector <https://github.com/eclipse-zenoh/roadmap/tree/main/rfcs/ALL/Selectors>`_ by aliasing it.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query_parameters(query: &z_query_t) -> z_bytes_t {
    let Some(query) = query.as_ref() else {
        return z_bytes_t::empty();
    };
    let complement = query.parameters();
    z_bytes_t {
        start: complement.as_ptr(),
        len: complement.len(),
    }
}

/// Get a query's `payload value <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md>`_ by aliasing it.
///
/// **WARNING: This API has been marked as unstable: it works as advertised, but it may change in a future release.**
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_value(query: &z_query_t) -> z_value_t {
    match query.as_ref().and_then(|q| q.value()) {
        Some(value) => {
            #[allow(mutable_transmutes)]
            if let std::borrow::Cow::Owned(payload) = value.payload.contiguous() {
                unsafe { std::mem::transmute::<_, &mut Value>(value).payload = payload.into() }
            }
            value.into()
        }
        None => (&Value::empty()).into(),
    }
}

/// Returns the attachment to the query by aliasing.
///
/// `z_check(return_value) == false` if there was no attachment to the query.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_attachment(query: &z_query_t) -> z_attachment_t {
    match query.as_ref().and_then(|q| q.attachment()) {
        Some(attachment) => z_attachment_t {
            data: attachment as *const _ as *mut c_void,
            iteration_driver: Some(attachment_iteration_driver),
        },
        None => z_attachment_null(),
    }
}
