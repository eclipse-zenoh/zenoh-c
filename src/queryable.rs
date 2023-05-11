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
use crate::{
    impl_guarded_transmute, z_bytes_t, z_closure_query_call, z_encoding_default, z_encoding_t,
    z_keyexpr_t, z_owned_closure_query_t, z_session_t, z_value_t, GuardedTransmute,
    LOG_INVALID_SESSION,
};
use libc::c_void;
use std::ops::Deref;
use zenoh::prelude::SessionDeclarations;
use zenoh::{
    prelude::{Sample, SplitBuffer},
    queryable::{Query, Queryable as CallbackQueryable},
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

/// Structs received by a Queryable.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_t(*mut c_void);
impl Deref for z_query_t {
    type Target = Query;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0 as *const _) }
    }
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
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_reply_options_t {
    pub encoding: z_encoding_t,
}

/// Constructs the default value for :c:type:`z_query_reply_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_query_reply_options_default() -> z_query_reply_options_t {
    z_query_reply_options_t {
        encoding: z_encoding_default(),
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
        .callback(move |query| {
            z_closure_query_call(&closure, &z_query_t(&query as *const _ as *mut c_void))
        })
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
    if let Some(key) = &*key {
        let mut s = Sample::new(
            key.clone().into_owned(),
            std::slice::from_raw_parts(payload as *const u8, len),
        );
        if let Some(o) = options {
            s.encoding = o.encoding.into();
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
    query.key_expr().borrowing_clone().into()
}

/// Get a query's `value selector <https://github.com/eclipse-zenoh/roadmap/tree/main/rfcs/ALL/Selectors>`_ by aliasing it.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_query_parameters(query: &z_query_t) -> z_bytes_t {
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
    match query.value() {
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
