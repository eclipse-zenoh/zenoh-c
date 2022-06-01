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

// /// Write data.
// ///
// /// Parameters:
// ///     session: The zenoh session.
// ///     keyexpr: The key expression to write.
// ///     payload: The value to write.
// ///     len: The length of the value to write.
// /// Returns:
// ///     ``0`` in case of success, ``1`` in case of failure.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_put(
//     session: z_session_t,
//     keyexpr: z_keyexpr_t,
//     payload: *const u8,
//     len: c_uint,
// ) -> c_int {
//     let r = session
//         .as_ref()
//         .as_ref()
//         .expect(LOG_INVALID_SESSION)
//         .put(
//             keyexpr,
//             slice::from_raw_parts(payload as *const u8, len as usize),
//         )
//         .res();

//     match r {
//         Ok(()) => 0,
//         _ => 1,
//     }
// }

// /// Options passed to the :c:func:`z_put_ext` function.
// #[repr(C)]
// #[allow(non_camel_case_types)]
// pub struct z_put_options_t {
//     encoding: z_encoding_t,
//     kind: u8,
//     congestion_control: u8,
//     priority: u8,
// }

// /// Constructs the default value for write options
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_put_options_default() -> z_put_options_t {
//     z_put_options_t {
//         encoding: z_encoding_default(),
//         kind: SampleKind::default() as u8,
//         congestion_control: CongestionControl::default() as u8,
//         priority: Priority::default() as u8,
//     }
// }

// /// Write data with extended options.
// ///
// /// Parameters:
// ///     session: The zenoh session.
// ///     keyexpr: The key expression to write.
// ///     payload: The value to write.
// ///     len: The length of the value to write.
// ///     options: The write options
// /// Returns:
// ///     ``0`` in case of success, ``1`` in case of failure.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_put_ext(
//     session: z_session_t,
//     keyexpr: z_keyexpr_t,
//     payload: *const u8,
//     len: c_uint,
//     options: &z_put_options_t,
// ) -> c_int {
//     let result = match session
//         .as_ref()
//         .as_ref()
//         .expect(LOG_INVALID_SESSION)
//         .put(
//             keyexpr,
//             slice::from_raw_parts(payload as *const u8, len as usize),
//         )
//         .encoding(options.encoding)
//         .kind(std::mem::transmute(options.kind))
//         .congestion_control(std::mem::transmute(options.congestion_control))
//         .priority(std::mem::transmute(options.priority))
//         .res()
//     {
//         Ok(()) => 0,
//         _ => 1,
//     };
//     result
// }
