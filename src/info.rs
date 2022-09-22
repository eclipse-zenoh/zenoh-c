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
use crate::{session::*, z_closure_zid_call, z_owned_closure_zid_t};
use zenoh::prelude::sync::SyncResolve;
use zenoh_protocol_core::ZenohId;

/// Represents a Zenoh ID.
///
/// In general, valid Zenoh IDs are LSB-first 128bit unsigned and non-zero integers.
#[repr(C)]
pub struct z_id_t {
    pub id: [u8; 16],
}

/// Returns the local Zenoh ID.
///
/// Unless the `session` is invalid, that ID is guaranteed to be non-zero.
/// In other words, this function returning an array of 16 zeros means you failed
/// to pass it a valid session.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_zid(session: z_session_t) -> z_id_t {
    match session.as_ref() {
        Some(s) => std::mem::transmute::<ZenohId, z_id_t>(s.info().zid().res_sync()),
        None => z_id_t { id: [0; 16] },
    }
}

/// Fetches the Zenoh IDs of all connected peers.
///
/// `callback` will be called once for each ID, is guaranteed to never be called concurrently,
/// and is guaranteed to be dropped before this function exits.
///
/// Retuns 0 on success
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_peers_zid(
    session: z_session_t,
    callback: &mut z_owned_closure_zid_t,
) -> i8 {
    let mut closure = z_owned_closure_zid_t::empty();
    std::mem::swap(&mut closure, callback);
    match session.as_ref() {
        Some(s) => {
            for id in s.info().peers_zid().res_sync() {
                z_closure_zid_call(&closure, &std::mem::transmute(id));
            }
            0
        }
        None => i8::MIN,
    }
}

/// Fetches the Zenoh IDs of all connected routers.
///
/// `callback` will be called once for each ID, is guaranteed to never be called concurrently,
/// and is guaranteed to be dropped before this function exits.
///
/// Retuns 0 on success
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_info_routers_zid(
    session: z_session_t,
    callback: &mut z_owned_closure_zid_t,
) -> i8 {
    let mut closure = z_owned_closure_zid_t::empty();
    std::mem::swap(&mut closure, callback);
    match session.as_ref() {
        Some(s) => {
            for id in s.info().routers_zid().res_sync() {
                z_closure_zid_call(&closure, &std::mem::transmute(id));
            }
            0
        }
        None => i8::MIN,
    }
}
