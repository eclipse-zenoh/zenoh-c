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

// type Queryable = Option<Arc<Sender<bool>>>;
// /// An owned zenoh queryable.
// ///
// /// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
// /// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
// ///
// /// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
// /// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
// /// After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
// ///
// /// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
// #[repr(C)]
// #[allow(non_camel_case_types)]
// pub struct z_owned_queryable_t([usize; 1]);
// impl AsRef<Queryable> for z_owned_queryable_t {
//     fn as_ref(&self) -> &Queryable {
//         unsafe { std::mem::transmute(self) }
//     }
// }
// impl AsMut<Queryable> for z_owned_queryable_t {
//     fn as_mut(&mut self) -> &mut Queryable {
//         unsafe { std::mem::transmute(self) }
//     }
// }
// #[allow(non_camel_case_types)]
// pub struct z_query_t(Query);

// /// Creates a Queryable for the given key expression.
// ///
// /// Parameters:
// ///     session: The zenoh session.
// ///     keyexpr: The key expression the Queryable will reply to.
// ///     callback: The callback function that will be called each time a matching query is received.
// ///     arg: A pointer that will be passed to the **callback** on each call.
// ///
// /// Returns:
// ///    The created :c:type:`z_owned_queryable_t` or null if the creation failed.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_queryable_new(
//     session: z_session_t,
//     keyexpr: z_keyexpr_t,
//     callback: extern "C" fn(&z_query_t, *const c_void),
//     arg: *mut c_void,
// ) -> z_owned_queryable_t {
//     let arg = Box::from_raw(arg);
//     let (tx, rx) = bounded::<bool>(1);
//     let r = z_owned_queryable_t(std::mem::transmute(Some(Arc::new(tx))));
//     let queryable = session
//         .as_ref()
//         .as_ref()
//         .expect(LOG_INVALID_SESSION)
//         .queryable(keyexpr)
//         .res()
//         .unwrap();
//     let mut queryable: zenoh::queryable::Queryable<'static> = std::mem::transmute(queryable);

//     // Note: This is done to ensure that even if the call-back into C
//     // does any blocking call we do not incour the risk of blocking
//     // any of the task resolving futures.
//     task::spawn_blocking(move || {
//         task::block_on(async move {
//             let arg = Box::into_raw(arg);
//             loop {
//                 select!(
//                 query = queryable.receiver().next().fuse() => {
//                   // This is a bit brutal but avoids an allocation and
//                   // a copy that would be otherwise required to add the
//                   // C string terminator. See the test_sub.c to find out how to deal
//                   // with non null terminated strings.
//                   let query = z_query_t(query.unwrap());
//                   callback(&query, arg);
//                 },
//                 _ = rx.recv().fuse() => {
//                     let _ = queryable.close().await;
//                     return
//                 })
//             }
//         })
//     });
//     r
// }

// /// Close a `z_owned_queryable_t`, freeing it and invalidating it for doube-free safety.
// ///
// /// Parameters:
// ///     qable: The :c:type:`z_owned_queryable_t` to close.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_queryable_close(qable: &mut z_owned_queryable_t) {
//     let qable = qable.as_mut();
//     match qable {
//         Some(tx) => {
//             let _ = async_std::task::block_on(tx.send(true));
//             *qable = None;
//         }
//         None => (),
//     }
// }

// /// Returns `true` if `qable` is valid.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_queryable_check(qable: &z_owned_queryable_t) -> bool {
//     qable.as_ref().is_some()
// }

// /// Send a reply to a query.
// ///
// /// This function must be called inside of a Queryable callback passing the
// /// query received as parameters of the callback function. This function can
// /// be called multiple times to send multiple replies to a query. The reply
// /// will be considered complete when the Queryable callback returns.
// ///
// /// Parameters:
// ///     query: The query to reply to.
// ///     key: The key of this reply.
// ///     payload: The value of this reply.
// ///     len: The length of the value of this reply.
// #[allow(clippy::missing_safety_doc)]
// #[no_mangle]
// pub unsafe extern "C" fn z_send_reply(
//     query: &z_query_t,
//     key: *const c_char,
//     payload: *const u8,
//     len: c_uint,
// ) {
//     let name = CStr::from_ptr(key).to_str().unwrap();
//     let s = Sample::new(
//         name.to_string(),
//         slice::from_raw_parts(payload as *const u8, len as usize),
//     );
//     query.0.replies_sender.send(s);
// }

// // @TODO: replace when stable https://github.com/rust-lang/rust/issues/65816
// #[inline]
// pub(crate) fn vec_into_raw_parts<T>(v: Vec<T>) -> (*mut T, usize, usize) {
//     let mut me = ManuallyDrop::new(v);
//     (me.as_mut_ptr(), me.len(), me.capacity())
// }
