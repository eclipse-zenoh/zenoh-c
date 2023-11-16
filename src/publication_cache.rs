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

use std::ops::Deref;

use zenoh_ext::SessionExt;
use zenoh_util::core::zresult::ErrNo;
use zenoh_util::core::SyncResolve;

use crate::{
    impl_guarded_transmute, z_keyexpr_t, z_session_t, zcu_locality_default, zcu_locality_t,
    GuardedTransmute, UninitializedKeyExprError,
};

/// Options passed to the :c:func:`ze_declare_publication_cache` function.
///
/// Members:
///     z_keyexpr_t queryable_prefix: The prefix used for queryable
///     zcu_locality_t queryable_origin: The restriction for the matching queries that will be receive by this
///                       publication cache
///     size_t history: The the history size
///     size_t resources_limit: The limit number of cached resources
#[repr(C)]
pub struct ze_publication_cache_options_t {
    pub queryable_prefix: z_keyexpr_t,
    pub queryable_origin: zcu_locality_t,
    pub history: usize,
    pub resources_limit: usize,
}

/// Constructs the default value for :c:type:`ze_publication_cache_options_t`.
#[no_mangle]
pub extern "C" fn ze_publication_cache_options_default() -> ze_publication_cache_options_t {
    ze_publication_cache_options_t {
        queryable_prefix: z_keyexpr_t::null(),
        queryable_origin: zcu_locality_default(),
        history: 1,
        resources_limit: 0,
    }
}

type PublicationCache = Option<Box<zenoh_ext::PublicationCache<'static>>>;

/// An owned zenoh publication_cache.
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
pub struct ze_owned_publication_cache_t([usize; 1]);

impl_guarded_transmute!(PublicationCache, ze_owned_publication_cache_t);

impl From<PublicationCache> for ze_owned_publication_cache_t {
    fn from(val: PublicationCache) -> Self {
        val.transmute()
    }
}

impl AsRef<PublicationCache> for ze_owned_publication_cache_t {
    fn as_ref(&self) -> &PublicationCache {
        unsafe { std::mem::transmute(self) }
    }
}

impl AsMut<PublicationCache> for ze_owned_publication_cache_t {
    fn as_mut(&mut self) -> &mut PublicationCache {
        unsafe { std::mem::transmute(self) }
    }
}

impl ze_owned_publication_cache_t {
    pub fn new(pub_cache: zenoh_ext::PublicationCache<'static>) -> Self {
        Some(Box::new(pub_cache)).into()
    }
    pub fn null() -> Self {
        None.into()
    }
}

/// Declares a Publication Cache.
///
/// Parameters:
///     z_session_t session: The zenoh session.
///     z_keyexpr_t keyexpr: The key expression to publish.
///     ze_publication_cache_options_t options: Additional options for the publication_cache.
///
/// Returns:
///    :c:type:`ze_owned_publication_cache_t`.
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
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    options: Option<&ze_publication_cache_options_t>,
) -> ze_owned_publication_cache_t {
    match session.upgrade() {
        Some(s) => {
            let keyexpr = keyexpr.deref().as_ref().map(|s| s.clone().into_owned());
            if let Some(key_expr) = keyexpr {
                let mut p = s.declare_publication_cache(key_expr);
                if let Some(options) = options {
                    p = p.history(options.history.into());
                    p = p.queryable_allowed_origin(options.queryable_origin.into());
                    if options.resources_limit != 0 {
                        p = p.resources_limit(options.resources_limit)
                    }
                    if options.queryable_prefix.deref().is_some() {
                        let queryable_prefix = options
                            .queryable_prefix
                            .deref()
                            .as_ref()
                            .map(|s| s.clone().into_owned());
                        if let Some(queryable_prefix) = queryable_prefix {
                            p = p.queryable_prefix(queryable_prefix)
                        }
                    }
                }
                match p.res_sync() {
                    Ok(publication_cache) => ze_owned_publication_cache_t::new(publication_cache),
                    Err(e) => {
                        log::error!("{}", e);
                        ze_owned_publication_cache_t::null()
                    }
                }
            } else {
                log::error!("{}", UninitializedKeyExprError);
                ze_owned_publication_cache_t::null()
            }
        }
        None => ze_owned_publication_cache_t::null(),
    }
}

/// Constructs a null safe-to-drop value of 'ze_owned_publication_cache_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_publication_cache_null() -> ze_owned_publication_cache_t {
    ze_owned_publication_cache_t::null()
}

/// Returns ``true`` if `pub_cache` is valid.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_publication_cache_check(pub_cache: &ze_owned_publication_cache_t) -> bool {
    pub_cache.as_ref().is_some()
}

/// Closes the given :c:type:`ze_owned_publication_cache_t`, droping it and invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn ze_close_publication_cache(pub_cache: &mut ze_owned_publication_cache_t) -> i8 {
    if let Some(p) = pub_cache.as_mut().take() {
        if let Err(e) = p.close().res_sync() {
            log::error!("{}", e);
            return e.errno().get();
        }
    }
    0
}
