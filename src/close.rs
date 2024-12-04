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

use std::mem::MaybeUninit;

use zenoh_runtime::ZRuntime;

#[cfg(feature = "unstable")]
use crate::opaque_types::zc_owned_concurrent_close_handle_t;
use crate::{
    result::{z_result_t, Z_EIO, Z_OK},
    transmute::{RustTypeRef, RustTypeRefUninit, TakeRustType},
    zc_moved_concurrent_close_handle_t,
};

#[cfg(feature = "unstable")]
decl_c_type!(
    owned(zc_owned_concurrent_close_handle_t, option tokio::task::JoinHandle<zenoh::Result<()>>),
);

/// @brief Blocking wait on close handle to complete. Returns `Z_EIO` if close finishes with error.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_concurrent_close_handle_wait(
    handle: &mut zc_moved_concurrent_close_handle_t,
) -> z_result_t {
    match ZRuntime::Application.block_on(handle.take_rust_type().unwrap_unchecked()) {
        Ok(_) => Z_OK,
        Err(e) => {
            tracing::error!("Close error: {}", e);
            Z_EIO
        }
    }
}

/// @brief Drops the close handle. The concurrent close task will not be interrupted.
#[no_mangle]
pub extern "C" fn zc_concurrent_close_handle_drop(this_: &mut zc_moved_concurrent_close_handle_t) {
    let _ = this_.take_rust_type();
}

/// @brief Returns ``true`` if concurrent close handle is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn zc_internal_concurrent_close_handle_check(
    this_: &zc_owned_concurrent_close_handle_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @brief Constructs concurrent close handle in its gravestone state.
#[no_mangle]
pub extern "C" fn zc_internal_concurrent_close_handle_null(
    this_: &mut MaybeUninit<zc_owned_concurrent_close_handle_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}
