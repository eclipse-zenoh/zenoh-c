/*
 * Copyright (c) 2017, 2020 ADLINK Technology Inc.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
 * which is available at https://www.apache.org/licenses/LICENSE-2.0.
 *
 * SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
 *
 * Contributors:
 *   ADLINK zenoh team, <zenoh@adlink-labs.tech>
 */
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
/**
 * An owned zenoh session.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_session_t {
  uintptr_t _0[3];
} z_owned_session_t;
/**
 * An owned zenoh configuration.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_config_t {
  void *_0;
} z_owned_config_t;
/**
 * A loaned zenoh config.
 */
typedef struct z_config_t {
  const struct z_owned_config_t *_0;
} z_config_t;
/**
 * A loaned zenoh session.
 */
typedef struct z_session_t {
  const struct z_owned_session_t *_0;
} z_session_t;
/**
 * A map of integers to strings providing informations on the zenoh session.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_info_t {
  uintptr_t _0[6];
} z_owned_info_t;
typedef struct z_info_t {
  const struct z_owned_info_t *_0;
} z_info_t;
/**
 * An owned zenoh subscriber. Destroying the subscriber cancels the subscription.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_subscriber_t {
  uintptr_t _0[1];
} z_owned_subscriber_t;
/**
 * Options passed to the :c:func:`z_put_ext` function.
 */
typedef struct z_put_options_t {
  uint64_t align;
  uintptr_t pad[5];
} z_put_options_t;
/**
 * An owned zenoh queryable.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_queryable_t {
  uintptr_t _0[1];
} z_owned_queryable_t;
