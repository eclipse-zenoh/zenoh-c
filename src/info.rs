use crate::transmute::{TransmuteCopy, TransmuteFromHandle};
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
use crate::{errors, z_closure_zid_call, z_owned_closure_zid_t, z_session_t};
use std::mem::MaybeUninit;
use zenoh::config::ZenohId;
use zenoh::prelude::sync::SyncResolve;
use zenoh::session::SessionDeclarations;

pub use crate::opaque_types::z_id_t;
decl_transmute_copy!(ZenohId, z_id_t);

impl From<[u8; 16]> for z_id_t {
    fn from(value: [u8; 16]) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

/// Returns the local Zenoh ID.
///
/// Unless the `session` is invalid, that ID is guaranteed to be non-zero.
/// In other words, this function returning an array of 16 zeros means you failed
/// to pass it a valid session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_zid(session: z_session_t) -> z_id_t {
    let session = session.transmute_ref();
    session.info().zid().res_sync().transmute_copy()
}

/// Fetches the Zenoh IDs of all connected peers.
///
/// `callback` will be called once for each ID, is guaranteed to never be called concurrently,
/// and is guaranteed to be dropped before this function exits.
///
/// Retuns 0 on success, negative values on failure
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_peers_zid(
    session: z_session_t,
    callback: &mut z_owned_closure_zid_t,
) -> errors::z_error_t {
    let mut closure = z_owned_closure_zid_t::empty();
    std::mem::swap(&mut closure, callback);
    let session = session.transmute_ref();
    for id in session.info().peers_zid().res_sync() {
        z_closure_zid_call(&closure, &id.transmute_copy());
    }
    errors::Z_OK
}

/// Fetches the Zenoh IDs of all connected routers.
///
/// `callback` will be called once for each ID, is guaranteed to never be called concurrently,
/// and is guaranteed to be dropped before this function exits.
///
/// Retuns 0 on success, negative values on failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_routers_zid(
    session: z_session_t,
    callback: &mut z_owned_closure_zid_t,
) -> errors::z_error_t {
    let mut closure = z_owned_closure_zid_t::empty();
    std::mem::swap(&mut closure, callback);
    let session = session.transmute_ref();
    for id in session.info().routers_zid().res_sync() {
        z_closure_zid_call(&closure, &id.transmute_copy());
    }
    errors::Z_OK
}
