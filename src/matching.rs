//
// Copyright (c) 2017, 2024 ZettaScale Technology.
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

use zenoh::matching::MatchingListener;

pub use crate::opaque_types::{zc_moved_matching_listener_t, zc_owned_matching_listener_t};
use crate::transmute::{RustTypeRef, RustTypeRefUninit, TakeRustType};
decl_c_type!(
    owned(zc_owned_matching_listener_t, option MatchingListener<()>),
);

#[no_mangle]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs an empty matching listener.
pub extern "C" fn zc_internal_matching_listener_null(
    this_: &mut MaybeUninit<zc_owned_matching_listener_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

#[no_mangle]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Checks the matching listener is for the gravestone state
pub extern "C" fn zc_internal_matching_listener_check(
    this_: &zc_owned_matching_listener_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A struct that indicates if there exist Subscribers matching the Publisher's key expression or Queryables matching Queriers key expression and target.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct zc_matching_status_t {
    /// True if there exist matching Zenoh entities, false otherwise.
    pub matching: bool,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Undeclares the given matching listener, droping and invalidating it.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn zc_matching_listener_drop(this: &mut zc_moved_matching_listener_t) {
    std::mem::drop(this.take_rust_type())
}
