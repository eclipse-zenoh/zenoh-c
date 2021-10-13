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
#define Z_SESSION_PADDING_U64 3
#define Z_CONFIG_PADDING_U64 66
#define Z_PUBLISHER_PADDING_U64 3
#define Z_SUBSCRIBER_PADDING_U64 1
#define Z_QUERYABLE_PADDING_U64 1
#define Z_WRITE_OPTIONS_PADDING_U64 6
#define Z_INFO_PADDING_U64 6
/**
 * An owned zenoh session.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.
 * The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_session_t {
  uint64_t _0[Z_SESSION_PADDING_U64];
} z_owned_session_t;
/**
 * An owned zenoh configuration.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.
 * The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_config_t {
  uint64_t _0[Z_CONFIG_PADDING_U64];
} z_owned_config_t;
/**
 * A borrowed zenoh config.
 */
typedef struct z_config_t {
  const struct z_owned_config_t *_0;
} z_config_t;
/**
 * A map of integers to strings providing informations on the zenoh session.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.
 * The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_info_t {
  uint64_t _0[Z_INFO_PADDING_U64];
} z_owned_info_t;
/**
 * A borrowed zenoh session.
 */
typedef struct z_session_t {
  const struct z_owned_session_t *_0;
} z_session_t;
typedef struct z_info_t {
  const struct z_owned_info_t *_0;
} z_info_t;
/**
 * An owned zenoh publisher.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.
 * The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_publisher_t {
  uint64_t _0[Z_PUBLISHER_PADDING_U64];
} z_owned_publisher_t;
/**
 * An owned zenoh subscriber. Destroying the subscriber cancels the subscription.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.
 * The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_subscriber_t {
  uint64_t _0[Z_SUBSCRIBER_PADDING_U64];
} z_owned_subscriber_t;
/**
 * An owned zenoh queryable.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.
 * The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_queryable_t {
  uint64_t _0[Z_QUERYABLE_PADDING_U64];
} z_owned_queryable_t;
/**
 * Options passed to the `z_write_ext` function.
 */
typedef struct z_write_options_t {
  uint64_t _0[Z_WRITE_OPTIONS_PADDING_U64];
} z_write_options_t;
