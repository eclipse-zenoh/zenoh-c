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

use prebindgen_proc_macro::prebindgen;
use zenoh::{session::ZenohId, Wait};

use crate::{
    result,
    transmute::{CTypeRef, IntoCType, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_zid_call, z_closure_zid_loan, z_loaned_session_t, z_moved_closure_zid_t,
    z_owned_string_t,
};
pub use zenoh_ffi_opaque_types::opaque_types::z_id_t;
decl_c_type!(copy(z_id_t, ZenohId));

/// @brief Formats the `z_id_t` into 16-digit hex string (LSB-first order)
#[prebindgen]
pub fn z_id_to_string(zid: &z_id_t, dst: &mut MaybeUninit<z_owned_string_t>) {
    let zid = zid.as_rust_type_ref();
    dst.as_rust_type_mut_uninit().write(zid.to_string().into());
}

/// @brief Returns the session's Zenoh ID.
///
/// Unless the `session` is invalid, that ID is guaranteed to be non-zero.
/// In other words, this function returning an array of 16 zeros means you failed
/// to pass it a valid session.
#[allow(clippy::missing_safety_doc)]
#[prebindgen]
pub unsafe fn z_info_zid(session: &z_loaned_session_t) -> z_id_t {
    let session = session.as_rust_type_ref();
    session.info().zid().wait().into_c_type()
}

/// @brief Fetches the Zenoh IDs of all connected peers.
///
/// `callback` will be called once for each ID, is guaranteed to never be called concurrently,
/// and is guaranteed to be dropped before this function exits.
///
/// Retuns 0 on success, negative values on failure
#[allow(clippy::missing_safety_doc)]
#[prebindgen]
pub unsafe fn z_info_peers_zid(
    session: &z_loaned_session_t,
    callback: &mut z_moved_closure_zid_t,
) -> result::z_result_t {
    let session = session.as_rust_type_ref();
    let callback = callback.take_rust_type();
    for mut id in session.info().peers_zid().wait() {
        z_closure_zid_call(z_closure_zid_loan(&callback), id.as_ctype_mut());
    }
    result::Z_OK
}

/// @brief Fetches the Zenoh IDs of all connected routers.
///
/// `callback` will be called once for each ID, is guaranteed to never be called concurrently,
/// and is guaranteed to be dropped before this function exits.
///
/// Retuns 0 on success, negative values on failure.
#[allow(clippy::missing_safety_doc)]
#[prebindgen]
pub unsafe fn z_info_routers_zid(
    session: &z_loaned_session_t,
    callback: &mut z_moved_closure_zid_t,
) -> result::z_result_t {
    let session = session.as_rust_type_ref();
    let callback = callback.take_rust_type();
    for mut id in session.info().routers_zid().wait() {
        z_closure_zid_call(z_closure_zid_loan(&callback), id.as_ctype_mut());
    }
    result::Z_OK
}
