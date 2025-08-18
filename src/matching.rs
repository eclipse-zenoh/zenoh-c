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
use prebindgen_proc_macro::prebindgen;

use zenoh::{matching::MatchingListener, Wait};

pub use zenoh_ffi_opaque_types::opaque_types::{z_moved_matching_listener_t, z_owned_matching_listener_t};
use crate::{
    result,
    transmute::{RustTypeRef, RustTypeRefUninit, TakeRustType},
};
decl_c_type!(
    owned(z_owned_matching_listener_t, option MatchingListener<()>),
);

#[prebindgen]
/// @brief Constructs an empty matching listener.
pub fn z_internal_matching_listener_null(
    this_: &mut MaybeUninit<z_owned_matching_listener_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

#[prebindgen]
/// @brief Checks the matching listener is for the gravestone state
pub fn z_internal_matching_listener_check(this_: &z_owned_matching_listener_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @brief A struct that indicates if there exist Subscribers matching the Publisher's key expression or Queryables matching Querier's key expression and target.
#[prebindgen]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct z_matching_status_t {
    /// True if there exist matching Zenoh entities, false otherwise.
    pub matching: bool,
}

/// @brief Undeclares the given matching listener, droping and invalidating it.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn z_matching_listener_drop(this: &mut z_moved_matching_listener_t) {
    std::mem::drop(this.take_rust_type())
}

/// @brief Undeclares the given matching listener, droping and invalidating it.
/// @return 0 in case of success, negative error code otherwise.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn z_undeclare_matching_listener(
    this: &mut z_moved_matching_listener_t,
) -> result::z_result_t {
    if let Some(m) = this.take_rust_type() {
        if let Err(e) = m.undeclare().wait() {
            crate::report_error!("{}", e);
            return result::Z_ENETWORK;
        }
    }
    result::Z_OK
}
