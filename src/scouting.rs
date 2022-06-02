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

// /// An owned array of owned NULL terminated strings, allocated by zenoh.
// ///
// /// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
// /// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
// /// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
// ///
// /// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
// #[repr(C)]
// pub struct z_owned_str_array_t {
//     pub val: *const *const c_char,
//     pub len: size_t,
// }

// impl<T> From<Vec<T>> for z_owned_str_array_t
// where
//     T: ToString,
// {
//     #[inline]
//     fn from(v: Vec<T>) -> Self {
//         let v = v
//             .into_iter()
//             .map(|t| {
//                 let s = CString::new(t.to_string()).unwrap();
//                 let res = s.as_ptr();
//                 std::mem::forget(s);
//                 res
//             })
//             .collect::<Vec<*const c_char>>();
//         let res = z_owned_str_array_t {
//             val: v.as_ptr(),
//             len: v.len() as size_t,
//         };
//         std::mem::forget(v);
//         res
//     }
// }

// impl<T> From<Option<Vec<T>>> for z_owned_str_array_t
// where
//     T: ToString,
// {
//     #[inline]
//     fn from(v: Option<Vec<T>>) -> Self {
//         match v {
//             Some(v) => v.into(),
//             None => z_owned_str_array_t {
//                 val: std::ptr::null(),
//                 len: 0,
//             },
//         }
//     }
// }

// /// Frees `strs` and invalidates it for double-free safety.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_str_array_free(strs: &mut z_owned_str_array_t) {
//     let locators = Vec::from_raw_parts(
//         strs.val as *mut *const c_char,
//         strs.len as usize,
//         strs.len as usize,
//     );
//     for locator in locators {
//         std::mem::drop(CString::from_raw(locator as *mut c_char));
//     }
//     strs.val = std::ptr::null();
//     strs.len = 0;
// }

// /// Returns `true` if `strs` is valid.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_str_array_check(strs: &z_owned_str_array_t) -> bool {
//     !strs.val.is_null() || strs.len == 0
// }

// /// The behavior to adopt in case of congestion while routing some data.
// ///
// ///     - **z_congestion_control_BLOCK**
// ///     - **z_congestion_control_DROP**
// #[allow(non_camel_case_types)]
// #[repr(C)]
// pub enum z_congestion_control {
//     BLOCK,
//     DROP,
// }

// impl From<z_congestion_control> for CongestionControl {
//     fn from(val: z_congestion_control) -> Self {
//         match val {
//             z_congestion_control::BLOCK => CongestionControl::Block,
//             z_congestion_control::DROP => CongestionControl::Drop,
//         }
//     }
// }

// /// A zenoh-allocated hello message returned by a zenoh entity to a scout message sent with `z_scout`.
// ///
// /// Members:
// ///   `unsigned int whatami`: The kind of zenoh entity.
// ///   `z_owned_bytes_t pid`: The peer id of the scouted entity (empty if absent).
// ///   `z_owned_str_array_t locators`: The locators of the scouted entity.
// ///
// /// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
// /// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
// /// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
// ///
// /// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
// #[repr(C)]
// pub struct z_owned_hello_t {
//     pub whatami: c_uint,
//     pub pid: z_owned_bytes_t,
//     pub locators: z_owned_str_array_t,
// }
// impl From<Hello> for z_owned_hello_t {
//     #[inline]
//     fn from(h: Hello) -> Self {
//         z_owned_hello_t {
//             whatami: match h.whatami {
//                 Some(whatami) => whatami as c_uint,
//                 None => Z_ROUTER,
//             },
//             pid: h.pid.into(),
//             locators: h.locators.into(),
//         }
//     }
// }

// /// Frees `hello`, invalidating it for double-free safety.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_hello_free(hello: &mut z_owned_hello_t) {
//     z_bytes_free(&mut hello.pid);
//     z_str_array_free(&mut hello.locators);
//     hello.whatami = 0;
// }
// /// Returns `true` if `hello` is valid.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_hello_check(hello: &z_owned_hello_t) -> bool {
//     hello.whatami != 0 && z_bytes_check(&hello.pid) && z_str_array_check(&hello.locators)
// }

// /// A zenoh-allocated array of `z_hello_t` messages.
// ///
// /// Members:
// ///   const z_hello_t *val: A pointer to the array.
// ///   unsigned int len: The length of the array.
// ///
// /// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
// /// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
// /// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
// ///
// /// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
// #[repr(C)]
// pub struct z_owned_hello_array_t {
//     pub val: *const z_owned_hello_t,
//     pub len: size_t,
// }
// impl From<Vec<Hello>> for z_owned_hello_array_t {
//     #[inline]
//     fn from(hvec: Vec<Hello>) -> Self {
//         let mut hvec = hvec
//             .into_iter()
//             .map(|h| h.into())
//             .collect::<Vec<z_owned_hello_t>>();
//         hvec.shrink_to_fit();
//         let res = z_owned_hello_array_t {
//             val: hvec.as_ptr(),
//             len: hvec.len() as size_t,
//         };
//         std::mem::forget(hvec);
//         res
//     }
// }
// /// Frees `hellos`, invalidating it for double-free safety.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_hello_array_free(hellos: &mut z_owned_hello_array_t) {
//     let hellos_vec = Vec::from_raw_parts(
//         hellos.val as *mut z_owned_hello_t,
//         hellos.len as usize,
//         hellos.len as usize,
//     );
//     for mut hello in hellos_vec {
//         z_hello_free(&mut hello);
//     }
//     hellos.val = std::ptr::null_mut();
//     hellos.len = 0;
// }
// /// Returns `true` if `hellos` is valid.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_hello_array_check(hellos: &z_owned_hello_array_t) -> bool {
//     !hellos.val.is_null() || hellos.len == 0
// }

// /// Scout for routers and/or peers.
// ///
// /// Parameters:
// ///     `what`: A whatami bitmask of zenoh entities kind to scout for.
// ///     `config`: A set of properties to configure the scouting.
// ///     `scout_period`: The time (in milliseconds) that should be spent scouting before returning the results.
// ///
// /// Returns:
// ///     An array of `z_hello_t` messages.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_scout(
//     what: c_uint,
//     config: &mut z_owned_config_t,
//     scout_period: c_ulong,
// ) -> z_owned_hello_array_t {
//     let what = WhatAmIMatcher::try_from(what as ZInt).unwrap_or(WhatAmI::Router | WhatAmI::Peer);
//     let config = config.as_mut().take().expect("invalid config");

//     let hellos = task::block_on(async move {
//         let mut hs = std::vec::Vec::<Hello>::new();
//         let mut stream = zenoh::scout(what, *config).res().unwrap();
//         let scout = async {
//             while let Some(hello) = stream.next().await {
//                 hs.push(hello)
//             }
//         };
//         let timeout = async_std::task::sleep(std::time::Duration::from_millis(scout_period as u64));
//         FutureExt::race(scout, timeout).await;
//         hs
//     });
//     hellos.into()
// }
