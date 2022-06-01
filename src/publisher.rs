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

// /// Declares a publication for the given key expression, returning `true` on success.
// ///
// /// Written resources that match the given key will only be sent on the network
// /// if matching subscribers exist in the system.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_declare_publication(session: z_session_t, keyexpr: z_keyexpr_t) -> bool {
//     session
//         .as_ref()
//         .as_ref()
//         .and_then(|s| s.declare_publication(keyexpr).res().ok())
//         .is_some()
// }

// /// Undeclares a publication for the given key expression.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_undeclare_publication(session: z_session_t, keyexpr: z_keyexpr_t) {
//     session
//         .as_ref()
//         .as_ref()
//         .map(|s| s.undeclare_publication(keyexpr).res().ok());
// }
