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

use std::mem::MaybeUninit;
use std::ptr::null;
use zenoh::prelude::SyncResolve;

use zenoh_ext::SessionExt;

use crate::transmute::{Inplace, TransmuteFromHandle, TransmuteRef, TransmuteUninitPtr};
use crate::{errors, z_loaned_keyexpr_t, z_loaned_session_t, zcu_locality_default, zcu_locality_t};

/// Options passed to the `ze_declare_publication_cache` function.
///
/// Members:
///     z_loaned_keyexpr_t queryable_prefix: The prefix used for queryable
///     zcu_locality_t queryable_origin: The restriction for the matching queries that will be receive by this
///                       publication cache
///     bool queryable_complete: the `complete` option for the queryable
///     size_t history: The the history size
///     size_t resources_limit: The limit number of cached resources
#[repr(C)]
pub struct ze_publication_cache_options_t {
    pub queryable_prefix: *const z_loaned_keyexpr_t,
    pub queryable_origin: zcu_locality_t,
    pub queryable_complete: bool,
    pub history: usize,
    pub resources_limit: usize,
}

/// Constructs the default value for `ze_publication_cache_options_t`.
#[no_mangle]
pub extern "C" fn ze_publication_cache_options_default(this: &mut ze_publication_cache_options_t) {
    *this = ze_publication_cache_options_t {
        queryable_prefix: null(),
        queryable_origin: zcu_locality_default(),
        queryable_complete: false,
        history: 1,
        resources_limit: 0,
    };
}

pub use crate::opaque_types::ze_loaned_publication_cache_t;
pub use crate::opaque_types::ze_owned_publication_cache_t;
decl_transmute_owned!(
    Option<zenoh_ext::PublicationCache<'static>>,
    ze_owned_publication_cache_t
);
decl_transmute_handle!(
    zenoh_ext::PublicationCache<'static>,
    ze_loaned_publication_cache_t
);

/// Declares a Publication Cache.
///
/// Parameters:
///     z_loaned_session_t session: The zenoh session.
///     z_loaned_keyexpr_t key_expr: The key expression to publish.
///     ze_publication_cache_options_t options: Additional options for the publication_cache.
///
/// Returns:
///    `ze_owned_publication_cache_t`.
///
///
/// Example:
///    Declaring a publication cache `NULL` for the options:
///
///    .. code-block:: C
///
///       ze_owned_publication_cache_t pub_cache = ze_declare_publication_cache(z_loan(s), z_keyexpr(expr), NULL);
///
///    is equivalent to initializing and passing the default publication cache options:
///    
///    .. code-block:: C
///
///       ze_publication_cache_options_t opts = ze_publication_cache_options_default();
///       ze_owned_publication_cache_t pub_cache = ze_declare_publication_cache(z_loan(s), z_keyexpr(expr), &opts);
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_declare_publication_cache(
    this: *mut MaybeUninit<ze_owned_publication_cache_t>,
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    options: Option<&mut ze_publication_cache_options_t>,
) -> errors::z_error_t {
    let this = this.transmute_uninit_ptr();
    let session = session.transmute_ref();
    let key_expr = key_expr.transmute_ref();
    let mut p = session.declare_publication_cache(key_expr);
    if let Some(options) = options {
        p = p.history(options.history);
        p = p.queryable_allowed_origin(options.queryable_origin.into());
        p = p.queryable_complete(options.queryable_complete);
        if options.resources_limit != 0 {
            p = p.resources_limit(options.resources_limit)
        }
        if !options.queryable_prefix.is_null() {
            let queryable_prefix = unsafe { *options.queryable_prefix }.transmute_ref();
            p = p.queryable_prefix(queryable_prefix.clone());
        }
    }
    match p.res_sync() {
        Ok(publication_cache) => {
            Inplace::init(this, Some(publication_cache));
            errors::Z_OK
        }
        Err(e) => {
            log::error!("{}", e);
            Inplace::empty(this);
            errors::Z_EGENERIC
        }
    }
}

/// Constructs a null safe-to-drop value of 'ze_owned_publication_cache_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_publication_cache_null(this: *mut MaybeUninit<ze_owned_publication_cache_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Returns ``true`` if `pub_cache` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_publication_cache_check(this: &ze_owned_publication_cache_t) -> bool {
    this.transmute_ref().is_some()
}

/// Closes the given `ze_owned_publication_cache_t`, droping it and invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_undeclare_publication_cache(
    this: &mut ze_owned_publication_cache_t,
) -> errors::z_error_t {
    if let Some(p) = this.transmute_mut().extract().take() {
        if let Err(e) = p.close().res_sync() {
            log::error!("{}", e);
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}
