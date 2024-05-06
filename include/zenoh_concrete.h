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
#ifdef DOCS
#define ALIGN(n)
#define ZENOHC_API
#endif
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#define DEFAULT_SCOUTING_TIMEOUT 1000
/**
 * An owned Zenoh session.
 */
typedef struct ALIGN(8) z_owned_session_t {
  uint8_t _0[8];
} z_owned_session_t;
/**
 * A loaned Zenoh query.
 *
 * It is valid as long as at least the corresponding `z_owned_query_t` exists, including the one owned by Zenoh until the callback returns.
 */
typedef struct ALIGN(8) z_loaned_query_t {
  uint8_t _0[16];
} z_loaned_query_t;
/**
 * A loaned Zenoh session.
 */
typedef struct ALIGN(8) z_loaned_session_t {
  uint8_t _0[40];
} z_loaned_session_t;
/**
 * An owned Zenoh queryable.
 *
 * Responds to queries sent via :c:func:`z_get` with intersecting key expression.
 */
typedef struct ALIGN(8) z_owned_queryable_t {
  uint8_t _0[32];
} z_owned_queryable_t;
/**
 * An owned Zenoh subscriber.
 *
 * Receives data from publication on intersecting key expressions.
 * Destroying the subscriber cancels the subscription.
 */
typedef struct ALIGN(8) z_owned_subscriber_t {
  uint8_t _0[32];
} z_owned_subscriber_t;
typedef struct ALIGN(8) z_loaned_subscriber_t {
  uint8_t _0[32];
} z_loaned_subscriber_t;
/**
 * An owned Zenoh publication cache.
 *
 * Used to store publications on intersecting key expressions. Can be queried later via :c:func:z_get to retrieve this data
 * (for example by Querying Subscriber).
 */
typedef struct ALIGN(8) ze_owned_publication_cache_t {
  uint8_t _0[96];
} ze_owned_publication_cache_t;
/**
 * An owned Zenoh querying subscriber.
 *
 * In addition to receiving the data it is subscribed to,
 * it also will fetch data from a Quryable at startup and peridodically (using :c:func: `ze_querying_subscriber_get`).
 */
typedef struct ALIGN(8) ze_owned_querying_subscriber_t {
  uint8_t _0[64];
} ze_owned_querying_subscriber_t;
/**
 * A loaned Zenoh querying subscriber
 */
typedef struct ALIGN(8) ze_loaned_querying_subscriber_t {
  uint8_t _0[64];
} ze_loaned_querying_subscriber_t;
#define Z_OK 0
#define Z_EINVAL -1
#define Z_EPARSE -2
#define Z_EIO -3
#define Z_ENETWORK -4
#define Z_ENULL -5
#define Z_EUNAVAILABLE -6
#define Z_EBUSY_MUTEX -16
#define Z_EINVAL_MUTEX -22
#define Z_EAGAIN_MUTEX -11
#define Z_EPOISON_MUTEX -22
#define Z_EGENERIC INT8_MIN
