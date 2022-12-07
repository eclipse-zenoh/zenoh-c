//

// Copyright (c) 2022 ZettaScale Technology

//

// This program and the accompanying materials are made available under the

// terms of the Eclipse Public License 2.0 which is available at

// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0

// which is available at https://www.apache.org/licenses/LICENSE-2.0.

//

// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0

//

// Contributors:

//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>

//

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#define DEFAULT_SCOUTING_TIMEOUT 1000
/**
 * An owned zenoh session.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_session_t {
  uintptr_t _0[3];
} z_owned_session_t;
/**
 * Structs received by a Queryable.
 */
typedef struct z_query_t {
  void *_0;
} z_query_t;
typedef struct _zc_stack_ke {
  uint64_t _0;
  uintptr_t _1[2];
  uint32_t _2;
  uint16_t _3;
  uint8_t _4;
} _zc_stack_ke;
typedef struct _zc_res_s_v {
  uint8_t _3;
  _z_u128 _0;
  struct _zc_stack_ke _1;
  uintptr_t _2[15];
} _zc_res_s_v;
/**
 * A loaned zenoh session.
 */
typedef struct z_session_t {
  const struct z_owned_session_t *_0;
} z_session_t;
/**
 * An owned zenoh pull subscriber. Destroying the subscriber cancels the subscription.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_pull_subscriber_t {
  uintptr_t _0[1];
} z_owned_pull_subscriber_t;
/**
 * An owned zenoh queryable.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_queryable_t {
  uintptr_t _0[4];
} z_owned_queryable_t;
/**
 * An owned zenoh subscriber. Destroying the subscriber cancels the subscription.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_subscriber_t {
  uintptr_t _0[1];
} z_owned_subscriber_t;
