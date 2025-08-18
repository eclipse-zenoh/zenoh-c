//
// Copyright (c) 2023 ZettaScale Technology.
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
#![allow(deprecated)]
use prebindgen_proc_macro::prebindgen;

use std::{mem::MaybeUninit, ptr::null};

use zenoh::Wait;
use zenoh_ext::{PublicationCacheBuilder, SessionExt};

use crate::{
    result,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_loaned_keyexpr_t, z_loaned_session_t, zc_locality_default, zc_locality_t,
};

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Options passed to the `ze_declare_publication_cache()` function.
#[prebindgen]
#[repr(C)]
pub struct ze_publication_cache_options_t {
    /// The suffix used for queryable.
    pub queryable_suffix: *const z_loaned_keyexpr_t,
    /// The restriction for the matching queries that will be receive by this publication cache.
    pub queryable_origin: zc_locality_t,
    /// The `complete` option for the queryable.
    pub queryable_complete: bool,
    /// The the history size (i.e. maximum number of messages to store).
    pub history: usize,
    /// The limit number of cached resources.
    pub resources_limit: usize,
}

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Constructs the default value for `ze_publication_cache_options_t`.
#[prebindgen]
pub fn ze_publication_cache_options_default(
    this: &mut MaybeUninit<ze_publication_cache_options_t>,
) {
    this.write(ze_publication_cache_options_t {
        queryable_suffix: null(),
        queryable_origin: zc_locality_default(),
        queryable_complete: false,
        history: 1,
        resources_limit: 0,
    });
}

pub use zenoh_ffi_opaque_types::opaque_types::{
    ze_loaned_publication_cache_t, ze_moved_publication_cache_t, ze_owned_publication_cache_t,
};
decl_c_type!(
    owned(
        ze_owned_publication_cache_t,
        option zenoh_ext::PublicationCache,
    ),
    loaned(ze_loaned_publication_cache_t),
);

fn _declare_publication_cache_inner<'a, 'b, 'c>(
    session: &'a z_loaned_session_t,
    key_expr: &'b z_loaned_keyexpr_t,
    options: Option<&'c mut ze_publication_cache_options_t>,
) -> PublicationCacheBuilder<'a, 'b, 'c> {
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let mut p = session.declare_publication_cache(key_expr);
    if let Some(options) = options {
        p = p
            .history(options.history)
            .queryable_allowed_origin(options.queryable_origin.into());
        p = p.queryable_complete(options.queryable_complete);
        if options.resources_limit != 0 {
            p = p.resources_limit(options.resources_limit)
        }
        if let Some(queryable_suffix) = unsafe { options.queryable_suffix.as_ref() } {
            let queryable_suffix = queryable_suffix.as_rust_type_ref();
            p = p.queryable_suffix(queryable_suffix.clone());
        }
    }
    p
}

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Constructs and declares a publication cache.
///
/// @param session: A Zenoh session.
/// @param pub_cache: An uninitialized location in memory where publication cache will be constructed.
/// @param key_expr: The key expression to publish to.
/// @param options: Additional options for the publication cache.
///
/// @returns 0 in case of success, negative error code otherwise.
#[prebindgen]
pub fn ze_declare_publication_cache(
    session: &z_loaned_session_t,
    pub_cache: &mut MaybeUninit<ze_owned_publication_cache_t>,
    key_expr: &z_loaned_keyexpr_t,
    options: Option<&mut ze_publication_cache_options_t>,
) -> result::z_result_t {
    let this = pub_cache.as_rust_type_mut_uninit();
    let p = _declare_publication_cache_inner(session, key_expr, options);
    match p.wait() {
        Ok(publication_cache) => {
            this.write(Some(publication_cache));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
    }
}

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Declares a background publication cache. It will function in background until the corresponding session is closed or dropped.
///
/// @param session: A Zenoh session.
/// @param key_expr: The key expression to publish to.
/// @param options: Additional options for the publication cache.
///
/// @returns 0 in case of success, negative error code otherwise.
#[prebindgen]
pub fn ze_declare_background_publication_cache(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    options: Option<&mut ze_publication_cache_options_t>,
) -> result::z_result_t {
    let p = _declare_publication_cache_inner(session, key_expr, options);
    match p.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Constructs a publication cache in a gravestone state.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_internal_publication_cache_null(this_: &mut MaybeUninit<ze_owned_publication_cache_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Returns ``true`` if publication cache is valid, ``false`` otherwise.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_internal_publication_cache_check(this_: &ze_owned_publication_cache_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Drops publication cache and resets it to its gravestone state.
/// This is equivalent to calling `ze_undeclare_publication_cache()` and discarding its return value.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn ze_publication_cache_drop(this: &mut ze_moved_publication_cache_t) {
    std::mem::drop(this.take_rust_type())
}

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Returns the key expression of the publication cache.
#[prebindgen]
pub fn ze_publication_cache_keyexpr(this_: &ze_loaned_publication_cache_t) -> &z_loaned_keyexpr_t {
    this_.as_rust_type_ref().key_expr().as_loaned_c_type_ref()
}

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Borrows publication cache.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn ze_publication_cache_loan(
    this_: &ze_owned_publication_cache_t,
) -> &ze_loaned_publication_cache_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Moves publication cache.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn ze_publication_cache_move(
    this_: &mut ze_owned_publication_cache_t,
) -> &mut ze_moved_publication_cache_t {
    std::mem::transmute(this_)
}

/// @warning This API is deprecated. Please use ze_advanced_publisher.
/// @brief Undeclares publication cache.
/// @return 0 in case of success, negative error code otherwise.
#[prebindgen]
pub fn ze_undeclare_publication_cache(
    this: &mut ze_moved_publication_cache_t,
) -> result::z_result_t {
    if let Some(p) = this.take_rust_type() {
        if let Err(e) = p.undeclare().wait() {
            crate::report_error!("{}", e);
            return result::Z_EGENERIC;
        }
    }
    result::Z_OK
}
