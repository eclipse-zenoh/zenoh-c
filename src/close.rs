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
use prebindgen_proc_macro::prebindgen;

use std::mem::MaybeUninit;

use zenoh::{internal::builders::close::NolocalJoinHandle, Wait};

use crate::{
    opaque_types::zc_owned_concurrent_close_handle_t,
    result::{z_result_t, Z_EIO, Z_OK},
    transmute::{RustTypeRef, RustTypeRefUninit, TakeRustType},
    zc_moved_concurrent_close_handle_t,
};

decl_c_type!(
    owned(zc_owned_concurrent_close_handle_t, option NolocalJoinHandle<zenoh::Result<()>>),
);

/// @brief Blocking wait on close handle to complete. Returns `Z_EIO` if close finishes with error.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn zc_concurrent_close_handle_wait(
    handle: &mut zc_moved_concurrent_close_handle_t,
) -> z_result_t {
    match handle.take_rust_type().unwrap_unchecked().wait() {
        Ok(_) => Z_OK,
        Err(e) => {
            crate::report_error!("Close error: {}", e);
            Z_EIO
        }
    }
}

/// @brief Drops the close handle. The concurrent close task will not be interrupted.
#[prebindgen]
pub fn zc_concurrent_close_handle_drop(this_: &mut zc_moved_concurrent_close_handle_t) {
    let _ = this_.take_rust_type();
}

/// @brief Returns ``true`` if concurrent close handle is valid, ``false`` if it is in gravestone state.
#[prebindgen]
pub fn zc_internal_concurrent_close_handle_check(
    this_: &zc_owned_concurrent_close_handle_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @brief Constructs concurrent close handle in its gravestone state.
#[prebindgen]
pub fn zc_internal_concurrent_close_handle_null(
    this_: &mut MaybeUninit<zc_owned_concurrent_close_handle_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}
