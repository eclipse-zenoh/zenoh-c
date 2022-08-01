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

use std::ops::Deref;

use libc::c_void;
use zenoh::{
    prelude::Sample,
    queryable::{CallbackQueryable, Query},
    Session,
};
use zenoh_util::core::SyncResolve;

use crate::{
    z_bytes_t, z_closure_query_call, z_keyexpr_t, z_owned_closure_query_t, z_session_t,
    LOG_INVALID_SESSION,
};

type Queryable = Option<CallbackQueryable<'static>>;
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
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_owned_queryable_t([usize; 4]);
impl From<Queryable> for z_owned_queryable_t {
    fn from(val: Queryable) -> Self {
        unsafe { std::mem::transmute(val) }
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
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_query_t(*const c_void);
impl Deref for z_query_t {
    type Target = Query;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0 as *const _) }
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_queryable_options_t {
    pub complete: bool,
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
///    The created :c:type:`z_owned_queryable_t` or null if the creation failed.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_declare_queryable(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    callback: &mut z_owned_closure_query_t,
    options: Option<&z_queryable_options_t>,
) -> z_owned_queryable_t {
    let mut closure = z_owned_closure_query_t::empty();
    std::mem::swap(&mut closure, callback);
    let session = match session.as_ref().as_ref() {
        Some(s) => std::mem::transmute::<&Session, &'static Session>(s),
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
            z_closure_query_call(&closure, z_query_t(&query as *const _ as *const c_void))
        })
        .res_sync()
        .map_err(|e| log::error!("{}", e))
        .ok()
        .into()
}

/// Close a `z_owned_queryable_t`, droping it and invalidating it for doube-drop safety.
///
/// Parameters:
///     qable: The :c:type:`z_owned_queryable_t` to close.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_undeclare_queryable(qable: &mut z_owned_queryable_t) {
    if let Some(qable) = qable.as_mut().take() {
        match qable.undeclare().res_sync() {
            Ok(()) => {}
            Err(e) => log::error!("{}", e),
        }
    }
}

/// Returns ``true`` if `qable` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_queryable_check(qable: &z_owned_queryable_t) -> bool {
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
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_reply(
    query: z_query_t,
    key: z_keyexpr_t,
    payload: *const u8,
    len: usize,
) {
    if let Some(key) = &*key {
        let s = Sample::new(
            key.clone().into_owned(),
            std::slice::from_raw_parts(payload as *const u8, len as usize),
        );
        if let Err(e) = query.reply(Ok(s)).res_sync() {
            log::error!("{}", e)
        }
    }
}

/// Get a query's key by aliasing it.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_keyexpr(query: z_query_t) -> z_keyexpr_t {
    query.key_expr().borrowing_clone().into()
}

/// Get a query's value selector by aliasing it.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_query_value_selector(query: z_query_t) -> z_bytes_t {
    let complement = query.value_selector();
    z_bytes_t {
        start: complement.as_ptr(),
        len: complement.len(),
    }
}
