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

// /// An owned reply to a `z_get` (or `z_get_collect`).
// ///
// /// Members:
// ///   `z_owned_sample_t sample`: a :c:type:`z_sample_t` containing the key and value of the reply.
// ///   `z_owned_bytes_t replier_id`: The id of the replier that sent this reply.
// ///
// /// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
// /// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
// /// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
// ///
// /// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
// #[repr(C)]
// pub enum z_owned_sample_result_t {
//     ok(z_owned_sample_t),
//     err(z_owned_value_t),
// }

// impl From<Result<Sample, Value>> for z_owned_sample_result_t {
//     fn from(val: Result<Sample, Value>) -> z_owned_sample_result_t {
//         match val {
//             Ok(s) => z_owned_sample_result_t::ok(s.into()),
//             Err(v) => z_owned_sample_result_t::err(v.into()),
//         }
//     }
// }

// /// Gets the key expression of a received query as a non null-terminated string.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub extern "C" fn z_query_key_expr(query: &z_query_t) -> z_keyexpr_t {
//     let (scope, s) = query.0.key_selector().as_id_and_suffix();
//     let suffix = z_bytes_t {
//         start: s.as_ptr(),
//         len: s.len(),
//     };
//     z_keyexpr_t {
//         id: scope as c_ulong,
//         suffix,
//     }
// }

// /// Gets the predicate of a received query as a non null-terminated string.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub extern "C" fn z_query_predicate(query: &z_query_t) -> z_bytes_t {
//     let s = query.0.selector().value_selector;
//     z_bytes_t {
//         start: s.as_ptr(),
//         len: s.len(),
//     }
// }

// /// Query data from the matching queryables in the system.
// /// Replies are provided through a callback function.
// ///
// /// Parameters:
// ///     session: The zenoh session.
// ///     keyexpr: The key expression matching resources to query.
// ///     predicate: An indication to matching queryables about the queried data.
// ///     target: The kind of queryables that should be target of this query.
// ///     consolidation: The kind of consolidation that should be applied on replies.
// ///     callback: The callback function that will be called on reception of replies for this query.
// ///     arg: A pointer that will be passed to the **callback** on each call.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_get(
//     session: z_session_t,
//     keyexpr: z_keyexpr_t,
//     predicate: *const c_char,
//     target: z_query_target_t,
//     consolidation: z_query_consolidation_t,
//     callback: extern "C" fn(z_owned_reply_t, *const c_void),
//     arg: *mut c_void,
// ) {
//     let p = CStr::from_ptr(predicate).to_str().unwrap();
//     let arg = Box::from_raw(arg);
//     let mut q = session
//         .as_ref()
//         .as_ref()
//         .expect(LOG_INVALID_SESSION)
//         .get(Selector {
//             key_selector: keyexpr.into(),
//             value_selector: Cow::Borrowed(p),
//         })
//         .target(target.into())
//         .consolidation(consolidation.into())
//         .res()
//         .unwrap();

//     task::spawn_blocking(move || {
//         task::block_on(async move {
//             let arg = Box::into_raw(arg);
//             while let Some(reply) = q.next().await {
//                 callback(
//                     z_owned_reply_t {
//                         tag: z_reply_t_Tag::DATA,
//                         data: reply.into(),
//                     },
//                     arg,
//                 )
//             }
//             callback(
//                 z_owned_reply_t {
//                     tag: z_reply_t_Tag::FINAL,
//                     data: z_owned_reply_data_t::empty(),
//                 },
//                 arg,
//             )
//             // while let Some(reply) = q.next().await {
//             //     callback(z_reply_t::DATA { data: reply.into() }, arg)
//             // }
//             // callback(z_reply_t::FINAL, arg)
//         })
//     });
// }

// /// Query data from the matching queryables in the system.
// /// Replies are collected in an array.
// ///
// /// Parameters:
// ///     session: The zenoh session.
// ///     keyexpr: The key expression matching resources to query.
// ///     predicate: An indication to matching queryables about the queried data.
// ///     target: The kind of queryables that should be target of this query.
// ///     consolidation: The kind of consolidation that should be applied on replies.
// ///
// /// Returns:
// ///    An array containing all the replies for this query.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_get_collect(
//     session: z_session_t,
//     keyexpr: z_keyexpr_t,
//     predicate: *const c_char,
//     target: z_query_target_t,
//     consolidation: z_query_consolidation_t,
// ) -> z_owned_reply_data_array_t {
//     let p = CStr::from_ptr(predicate).to_str().unwrap();
//     let mut replies = session
//         .as_ref()
//         .as_ref()
//         .expect(LOG_INVALID_SESSION)
//         .get(Selector {
//             key_selector: keyexpr.into(),
//             value_selector: Cow::Borrowed(p),
//         })
//         .target(target.into())
//         .consolidation(consolidation.into())
//         .res()
//         .unwrap()
//         .iter()
//         .map(|r| r.into())
//         .collect::<Vec<z_owned_reply_data_t>>();

//     replies.shrink_to_fit();
//     //TODO replace when stable https://github.com/rust-lang/rust/issues/65816
//     let (val, len, _cap) = vec_into_raw_parts(replies);
//     z_owned_reply_data_array_t { val, len }
// }

// #[allow(non_camel_case_types)]
// #[repr(C)]
// pub struct z_owned_reply_data_t {
//     sample: z_owned_sample_t,
//     replier_id: z_owned_bytes_t,
// }
// impl z_owned_reply_data_t {
//     #[inline]
//     pub(crate) fn empty() -> Self {
//         z_owned_reply_data_t {
//             sample: z_owned_sample_t {
//                 key: z_owned_keyexpr_t::null(),
//                 value: z_owned_bytes_t::empty(),
//                 encoding: z_owned_encoding_t {
//                     prefix: z_known_encoding::Empty,
//                     suffix: z_owned_bytes_t {
//                         start: std::ptr::null(),
//                         len: 0,
//                     },
//                     _freed: false,
//                 },
//             },
//             replier_id: z_owned_bytes_t::default(),
//         }
//     }
// }

// impl From<Reply> for z_owned_reply_data_t {
//     #[inline]
//     fn from(r: Reply) -> Self {
//         z_owned_reply_data_t {
//             sample: r.sample.unwrap().into(),
//             replier_id: r.replier_id.into(),
//         }
//     }
// }

// /// Frees `reply_data`, invalidating it for double-free safety.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_reply_data_free(reply_data: &mut z_owned_reply_data_t) {
//     z_sample_free(&mut reply_data.sample);
//     z_bytes_free(&mut reply_data.replier_id);
// }
// /// Returns `true` if `reply_data` is valid.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_reply_data_check(reply_data: &z_owned_reply_data_t) -> bool {
//     z_sample_check(&reply_data.sample) && z_bytes_check(&reply_data.replier_id)
// }

// /// A zenoh-allocated array of :c:type:`z_owned_reply_data_t`.
// ///
// /// Members:
// ///   `char *const *val`: A pointer to the array.
// ///   `unsigned int len`: The length of the array.
// ///
// /// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
// /// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
// /// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
// ///
// /// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
// #[repr(C)]
// pub struct z_owned_reply_data_array_t {
//     pub val: *const z_owned_reply_data_t,
//     pub len: size_t,
// }

// /// Free a :c:type:`z_owned_reply_data_array_t` and it's contained replies.
// ///
// /// Parameters:
// ///     replies: The :c:type:`z_owned_reply_data_array_t` to free.
// ///
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_reply_data_array_free(replies: &mut z_owned_reply_data_array_t) {
//     let vec = Vec::from_raw_parts(
//         replies.val as *mut z_owned_reply_data_t,
//         replies.len,
//         replies.len,
//     );
//     for mut rd in vec {
//         z_reply_data_free(&mut rd);
//     }
//     replies.val = std::ptr::null();
//     replies.len = 0;
// }

// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_reply_data_array_check(replies: &z_owned_reply_data_array_t) -> bool {
//     !replies.val.is_null() || replies.len == 0
// }

// /// The possible values of :c:member:`z_owned_reply_t.tag`
// ///
// ///     - **z_reply_t_Tag_DATA**: The reply contains some data.
// ///     - **z_reply_t_Tag_FINAL**: The reply does not contain any data and indicates that there will be no more replies for this query.
// #[allow(non_camel_case_types)]
// #[repr(C)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum z_reply_t_Tag {
//     DATA,
//     FINAL,
// }

// /// An owned reply to a :c:func:`z_get`.
// ///
// /// Members:
// ///   `z_reply_t_Tag tag`: Indicates if the reply contains data or if it's a FINAL reply.
// ///   `z_owned_reply_data_t data`: The reply data if :c:member:`z_owned_reply_t.tag` equals :c:member:`z_reply_t_Tag.z_reply_t_Tag_DATA`.
// ///
// /// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
// /// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
// /// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
// ///
// /// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
// #[allow(non_camel_case_types)]
// #[repr(C)]
// pub struct z_owned_reply_t {
//     pub tag: z_reply_t_Tag,
//     pub data: z_owned_reply_data_t,
// }
// /// Frees `reply`, invalidating it for double-free safety.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_reply_free(reply: &mut z_owned_reply_t) {
//     if reply.tag == z_reply_t_Tag::DATA {
//         z_reply_data_free(&mut reply.data)
//     }
// }
// /// Returns `true` if `reply` is valid.
// #[no_mangle]
// #[allow(clippy::missing_safety_doc)]
// pub unsafe extern "C" fn z_reply_check(reply: &z_owned_reply_t) -> bool {
//     z_reply_t_Tag::FINAL == reply.tag
//         || (z_reply_t_Tag::DATA == reply.tag && z_reply_data_check(&reply.data))
// }

// /// The possible values of :c:member:`z_query_target_t.tag`.
// ///
// ///     - **z_query_target_t_BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
// ///     - **z_query_target_t_COMPLETE**: A set of complete queryables.
// ///     - **z_query_target_t_ALL**: All matching queryables.
// ///     - **z_query_target_t_NONE**: No queryables.
// #[allow(non_camel_case_types)]
// #[repr(C)]
// pub enum z_query_target_t {
//     BEST_MATCHING,
//     ALL,
//     NONE,
//     ALL_COMPLETE,
//     // #[cfg(feature = "complete_n")]
//     // COMPLETE {
//     //     n: c_uint,
//     // },
// }

// impl From<QueryTarget> for z_query_target_t {
//     #[inline]
//     fn from(t: QueryTarget) -> Self {
//         match t {
//             QueryTarget::BestMatching => z_query_target_t::BEST_MATCHING,
//             QueryTarget::All => z_query_target_t::ALL,
//             QueryTarget::None => z_query_target_t::NONE,
//             QueryTarget::AllComplete => z_query_target_t::ALL_COMPLETE,
//             // #[cfg(feature = "complete_n")]
//             // QueryTarget::Complete(n) => z_query_target_t::COMPLETE { n: n as c_uint },
//         }
//     }
// }

// impl From<z_query_target_t> for QueryTarget {
//     #[inline]
//     fn from(val: z_query_target_t) -> Self {
//         match val {
//             z_query_target_t::BEST_MATCHING => QueryTarget::BestMatching,
//             z_query_target_t::ALL => QueryTarget::All,
//             z_query_target_t::NONE => QueryTarget::None,
//             z_query_target_t::ALL_COMPLETE => QueryTarget::AllComplete,
//             // #[cfg(feature = "complete_n")]
//             // z_query_target_t::COMPLETE { n } => QueryTarget::Complete(n as ZInt),
//         }
//     }
// }

// /// Create a default :c:type:`z_query_target_t`.
// #[no_mangle]
// pub extern "C" fn z_query_target_default() -> z_query_target_t {
//     QueryTarget::default().into()
// }

// /// The kind of consolidation that should be applied on replies to a :c:func:`z_get`.
// ///
// ///     - **z_consolidation_mode_t_FULL**: Guaranties unicity of replies. Optimizes bandwidth.
// ///     - **z_consolidation_mode_t_LAZY**: Does not garanty unicity. Optimizes latency.
// ///     - **z_consolidation_mode_t_NONE**: No consolidation.
// #[repr(C)]
// pub enum z_consolidation_mode_t {
//     FULL,
//     LAZY,
//     NONE,
// }

// impl From<ConsolidationMode> for z_consolidation_mode_t {
//     #[inline]
//     fn from(cm: ConsolidationMode) -> Self {
//         match cm {
//             ConsolidationMode::Full => z_consolidation_mode_t::FULL,
//             ConsolidationMode::Lazy => z_consolidation_mode_t::LAZY,
//             ConsolidationMode::None => z_consolidation_mode_t::NONE,
//         }
//     }
// }

// impl From<z_consolidation_mode_t> for ConsolidationMode {
//     #[inline]
//     fn from(val: z_consolidation_mode_t) -> Self {
//         match val {
//             z_consolidation_mode_t::NONE => ConsolidationMode::None,
//             z_consolidation_mode_t::LAZY => ConsolidationMode::Lazy,
//             z_consolidation_mode_t::FULL => ConsolidationMode::Full,
//         }
//     }
// }

// /// The kind of consolidation that should be applied on replies to a :c:func:`z_get`
// /// at the different stages of the reply process.
// ///
// /// Members:
// ///   z_consolidation_mode_t first_routers: The consolidation mode to apply on first routers of the replies routing path.
// ///   z_consolidation_mode_t last_router: The consolidation mode to apply on last router of the replies routing path.
// ///   z_consolidation_mode_t reception: The consolidation mode to apply at reception of the replies.
// #[repr(C)]
// pub struct z_consolidation_strategy_t {
//     pub first_routers: z_consolidation_mode_t,
//     pub last_router: z_consolidation_mode_t,
//     pub reception: z_consolidation_mode_t,
// }

// impl From<ConsolidationStrategy> for z_consolidation_strategy_t {
//     #[inline]
//     fn from(cs: ConsolidationStrategy) -> Self {
//         z_consolidation_strategy_t {
//             first_routers: cs.first_routers.into(),
//             last_router: cs.last_router.into(),
//             reception: cs.reception.into(),
//         }
//     }
// }

// impl From<z_consolidation_strategy_t> for ConsolidationStrategy {
//     #[inline]
//     fn from(val: z_consolidation_strategy_t) -> Self {
//         ConsolidationStrategy {
//             first_routers: val.first_routers.into(),
//             last_router: val.last_router.into(),
//             reception: val.reception.into(),
//         }
//     }
// }

// /// The replies consolidation strategy to apply on replies to a :c:func:`z_get`.
// #[repr(C)]
// pub enum z_query_consolidation_t {
//     AUTO,
//     MANUAL(z_consolidation_strategy_t),
// }

// impl From<QueryConsolidation> for z_query_consolidation_t {
//     #[inline]
//     fn from(qc: QueryConsolidation) -> Self {
//         match qc {
//             QueryConsolidation::Auto => z_query_consolidation_t::AUTO,
//             QueryConsolidation::Manual(strategy) => {
//                 z_query_consolidation_t::MANUAL(strategy.into())
//             }
//         }
//     }
// }

// impl From<z_query_consolidation_t> for QueryConsolidation {
//     #[inline]
//     fn from(val: z_query_consolidation_t) -> Self {
//         match val {
//             z_query_consolidation_t::AUTO => QueryConsolidation::Auto,
//             z_query_consolidation_t::MANUAL(strategy) => {
//                 QueryConsolidation::Manual(strategy.into())
//             }
//         }
//     }
// }

// /// Automatic query consolidation strategy selection.
// ///
// /// A query consolidation strategy will automatically be selected depending
// /// the query selector. If the selector contains time range properties,
// /// no consolidation is performed. Otherwise the
// /// :c:func:`z_query_consolidation_reception` strategy is used.
// #[no_mangle]
// pub extern "C" fn z_query_consolidation_auto() -> z_query_consolidation_t {
//     QueryConsolidation::auto().into()
// }

// /// No consolidation performed.
// ///
// /// This is usefull when querying timeseries data bases or
// /// when using quorums.
// #[no_mangle]
// pub extern "C" fn z_query_consolidation_none() -> z_query_consolidation_t {
//     QueryConsolidation::none().into()
// }

// /// Lazy consolidation performed at all stages.
// ///
// /// This strategy offers the best latency. Replies are directly
// /// transmitted to the application when received without needing
// /// to wait for all replies.
// ///
// /// This mode does not garantie that there will be no duplicates.
// #[no_mangle]
// pub extern "C" fn z_query_consolidation_lazy() -> z_query_consolidation_t {
//     QueryConsolidation::lazy().into()
// }

// /// Full consolidation performed at reception.
// ///
// /// This is the default strategy. It offers the best latency while
// /// garantying that there will be no duplicates.
// #[no_mangle]
// pub extern "C" fn z_query_consolidation_reception() -> z_query_consolidation_t {
//     QueryConsolidation::reception().into()
// }

// /// Full consolidation performed on last router and at reception.
// ///
// /// This mode offers a good latency while optimizing bandwidth on
// /// the last transport link between the router and the application.
// #[no_mangle]
// pub extern "C" fn z_query_consolidation_last_router() -> z_query_consolidation_t {
//     QueryConsolidation::last_router().into()
// }

// /// Full consolidation performed everywhere.
// ///
// /// This mode optimizes bandwidth on all links in the system
// /// but will provide a very poor latency.
// #[no_mangle]
// pub extern "C" fn z_query_consolidation_full() -> z_query_consolidation_t {
//     QueryConsolidation::full().into()
// }

// /// Creates a default :c:type:`z_query_consolidation_t`.
// #[no_mangle]
// pub extern "C" fn z_query_consolidation_default() -> z_query_consolidation_t {
//     QueryConsolidation::default().into()
// }
