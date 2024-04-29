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
/**
 * The kind of congestion control.
 *
 *     - **BLOCK**
 *     - **DROP**
 */
typedef enum z_congestion_control_t {
  Z_CONGESTION_CONTROL_BLOCK,
  Z_CONGESTION_CONTROL_DROP,
} z_congestion_control_t;
/**
 * Consolidation mode values.
 *
 *     - **Z_CONSOLIDATION_MODE_AUTO**: Let Zenoh decide the best consolidation mode depending on the query selector
 *       If the selector contains time range properties, consolidation mode `NONE` is used.
 *       Otherwise the `LATEST` consolidation mode is used.
 *     - **Z_CONSOLIDATION_MODE_NONE**: No consolidation is applied. Replies may come in any order and any number.
 *     - **Z_CONSOLIDATION_MODE_MONOTONIC**: It guarantees that any reply for a given key expression will be monotonic in time
 *       w.r.t. the previous received replies for the same key expression. I.e., for the same key expression multiple
 *       replies may be received. It is guaranteed that two replies received at t1 and t2 will have timestamp
 *       ts2 > ts1. It optimizes latency.
 *     - **Z_CONSOLIDATION_MODE_LATEST**: It guarantees unicity of replies for the same key expression.
 *       It optimizes bandwidth.
 */
typedef enum z_consolidation_mode_t {
  Z_CONSOLIDATION_MODE_AUTO = -1,
  Z_CONSOLIDATION_MODE_NONE = 0,
  Z_CONSOLIDATION_MODE_MONOTONIC = 1,
  Z_CONSOLIDATION_MODE_LATEST = 2,
} z_consolidation_mode_t;
/**
 * A :c:type:`z_keyexpr_intersection_level_t`.
 *
 *     - **Z_KEYEXPR_INTERSECTION_LEVEL_DISJOINT**
 *     - **Z_KEYEXPR_INTERSECTION_LEVEL_INTERSECTS**
 *     - **Z_KEYEXPR_INTERSECTION_LEVEL_INCLUDES**
 *     - **Z_KEYEXPR_INTERSECTION_LEVEL_EQUALS**
 */
typedef enum z_keyexpr_intersection_level_t {
  Z_KEYEXPR_INTERSECTION_LEVEL_DISJOINT = 0,
  Z_KEYEXPR_INTERSECTION_LEVEL_INTERSECTS = 1,
  Z_KEYEXPR_INTERSECTION_LEVEL_INCLUDES = 2,
  Z_KEYEXPR_INTERSECTION_LEVEL_EQUALS = 3,
} z_keyexpr_intersection_level_t;
/**
 * The Queryables that should be target of a :c:func:`z_get`.
 *
 *     - **BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
 *     - **ALL_COMPLETE**: All complete queryables.
 *     - **ALL**: All matching queryables.
 */
typedef enum z_loaned_query_target_t {
  Z_LOANED_QUERY_TARGET_BEST_MATCHING,
  Z_LOANED_QUERY_TARGET_ALL,
  Z_LOANED_QUERY_TARGET_ALL_COMPLETE,
} z_loaned_query_target_t;
/**
 * The priority of zenoh messages.
 *
 *     - **REAL_TIME**
 *     - **INTERACTIVE_HIGH**
 *     - **INTERACTIVE_LOW**
 *     - **DATA_HIGH**
 *     - **DATA**
 *     - **DATA_LOW**
 *     - **BACKGROUND**
 */
typedef enum z_priority_t {
  Z_PRIORITY_REAL_TIME = 1,
  Z_PRIORITY_INTERACTIVE_HIGH = 2,
  Z_PRIORITY_INTERACTIVE_LOW = 3,
  Z_PRIORITY_DATA_HIGH = 4,
  Z_PRIORITY_DATA = 5,
  Z_PRIORITY_DATA_LOW = 6,
  Z_PRIORITY_BACKGROUND = 7,
} z_priority_t;
/**
 * The subscription reliability.
 *
 *     - **Z_RELIABILITY_BEST_EFFORT**
 *     - **Z_RELIABILITY_RELIABLE**
 */
typedef enum z_reliability_t {
  Z_RELIABILITY_BEST_EFFORT,
  Z_RELIABILITY_RELIABLE,
} z_reliability_t;
typedef enum z_sample_kind_t {
  Z_SAMPLE_KIND_PUT = 0,
  Z_SAMPLE_KIND_DELETE = 1,
} z_sample_kind_t;
typedef enum zcu_locality_t {
  ZCU_LOCALITY_ANY = 0,
  ZCU_LOCALITY_SESSION_LOCAL = 1,
  ZCU_LOCALITY_REMOTE = 2,
} zcu_locality_t;
typedef enum zcu_reply_keyexpr_t {
  ZCU_REPLY_KEYEXPR_ANY = 0,
  ZCU_REPLY_KEYEXPR_MATCHING_QUERY = 1,
} zcu_reply_keyexpr_t;
/**
 * A split buffer that owns all of its data.
 *
 * To minimize copies and reallocations, Zenoh may provide you data in split buffers.
 */
typedef struct ALIGN(8) z_owned_bytes_t {
  uint8_t _0[40];
} z_owned_bytes_t;
/**
 * A loaned payload.
 */
typedef struct ALIGN(8) z_loaned_bytes_t {
  uint8_t _0[40];
} z_loaned_bytes_t;
typedef int8_t z_error_t;
/**
 * A contiguous view of bytes owned by some other entity.
 *
 * `start` being `null` is considered a gravestone value,
 * and empty slices are represented using a possibly dangling pointer for `start`.
 */
typedef struct ALIGN(8) z_owned_slice_t {
  uint8_t _0[16];
} z_owned_slice_t;
/**
 * A map of maybe-owned slices to maybe-owned slices.
 *
 * In Zenoh C, this map is backed by Rust's standard HashMap, with a DoS-resistant hasher
 */
typedef struct ALIGN(8) z_owned_slice_map_t {
  uint8_t _0[48];
} z_owned_slice_map_t;
/**
 * The wrapper type for null-terminated string values allocated by zenoh. The instances of `z_owned_str_t`
 * should be released with `z_drop` macro or with `z_str_drop` function and checked to validity with
 * `z_check` and `z_str_check` correspondently
 */
typedef struct ALIGN(8) z_owned_str_t {
  uint8_t _0[16];
} z_owned_str_t;
typedef struct ALIGN(8) z_loaned_slice_map_t {
  uint8_t _0[48];
} z_loaned_slice_map_t;
typedef struct ALIGN(8) z_loaned_slice_t {
  uint8_t _0[16];
} z_loaned_slice_t;
typedef struct ALIGN(8) z_loaned_str_t {
  uint8_t _0[16];
} z_loaned_str_t;
/**
 * A reader for payload data.
 */
typedef struct ALIGN(8) z_owned_bytes_reader_t {
  uint8_t _0[24];
} z_owned_bytes_reader_t;
typedef struct ALIGN(8) z_loaned_bytes_reader_t {
  uint8_t _0[24];
} z_loaned_bytes_reader_t;
/**
 * Clock
 * Uses monotonic clock
 */
typedef struct z_clock_t {
  uint64_t t;
  const void *t_base;
} z_clock_t;
typedef struct ALIGN(8) z_loaned_hello_t {
  uint8_t _0[48];
} z_loaned_hello_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 *
 * Members:
 *   void *context: a pointer to an arbitrary state.
 *   void *call(const struct z_hello_t* hello, const void *context): the typical callback function. `context` will be passed as its last argument.
 *   void *drop(void*): allows the callback's state to be freed.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 */
typedef struct z_owned_closure_hello_t {
  void *context;
  void (*call)(const struct z_loaned_hello_t*, void*);
  void (*drop)(void*);
} z_owned_closure_hello_t;
/**
 *
 * Queries are atomically reference-counted, letting you extract them from the callback that handed them to you by cloning.
 * `z_loaned_query_t`'s are valid as long as at least one corresponding `z_owned_query_t` exists, including the one owned by Zenoh until the callback returns.
 */
typedef struct ALIGN(8) z_owned_query_t {
  uint8_t _0[16];
} z_owned_query_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 *
 * Members:
 *   void *context: a pointer to an arbitrary state.
 *   void *call(const struct z_loaned_query_t*, const void *context): the typical callback function. `context` will be passed as its last argument.
 *   void *drop(void*): allows the callback's state to be freed.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 */
typedef struct z_owned_closure_owned_query_t {
  void *context;
  void (*call)(struct z_owned_query_t*, void *context);
  void (*drop)(void*);
} z_owned_closure_owned_query_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 *
 * Members:
 *   void *context: a pointer to an arbitrary state.
 *   void *call(z_loaned_query_t, const void *context): the typical callback function. `context` will be passed as its last argument.
 *   void *drop(void*): allows the callback's state to be freed.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 */
typedef struct z_owned_closure_query_t {
  void *context;
  void (*call)(const struct z_loaned_query_t*, void *context);
  void (*drop)(void*);
} z_owned_closure_query_t;
typedef struct ALIGN(8) z_loaned_reply_t {
  uint8_t _0[256];
} z_loaned_reply_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 *
 * Members:
 *   void *context: a pointer to an arbitrary state.
 *   void *call(const struct z_owned_reply_t*, const void *context): the typical callback function. `context` will be passed as its last argument.
 *   void *drop(void*): allows the callback's state to be freed.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 */
typedef struct z_owned_closure_reply_t {
  void *context;
  void (*call)(const struct z_loaned_reply_t*, void*);
  void (*drop)(void*);
} z_owned_closure_reply_t;
typedef struct ALIGN(8) z_loaned_sample_t {
  uint8_t _0[240];
} z_loaned_sample_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
 *
 * Members:
 *   void *context: a pointer to an arbitrary state.
 *   void *call(struct z_loaned_sample_t, const void *context): the typical callback function. `context` will be passed as its last argument.
 *   void *drop(void*): allows the callback's state to be freed.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 */
typedef struct z_owned_closure_sample_t {
  void *context;
  void (*call)(const struct z_loaned_sample_t*, void *context);
  void (*drop)(void*);
} z_owned_closure_sample_t;
/**
 * Represents a Zenoh ID.
 *
 * In general, valid Zenoh IDs are LSB-first 128bit unsigned and non-zero integers.
 */
typedef struct ALIGN(1) z_id_t {
  uint8_t id[16];
} z_id_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 *
 * Members:
 *   void *context: a pointer to an arbitrary state.
 *   void *call(const struct z_owned_reply_t*, const void *context): the typical callback function. `context` will be passed as its last argument.
 *   void *drop(void*): allows the callback's state to be freed.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 */
typedef struct z_owned_closure_zid_t {
  void *context;
  void (*call)(const struct z_id_t*, void*);
  void (*drop)(void*);
} z_owned_closure_zid_t;
typedef struct ALIGN(8) z_owned_condvar_t {
  uint8_t _0[24];
} z_owned_condvar_t;
typedef struct ALIGN(8) z_loaned_condvar_t {
  uint8_t _0[16];
} z_loaned_condvar_t;
typedef struct ALIGN(8) z_loaned_mutex_t {
  uint8_t _0[32];
} z_loaned_mutex_t;
/**
 * An owned zenoh configuration.
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
typedef struct ALIGN(8) z_owned_config_t {
  uint8_t _0[1544];
} z_owned_config_t;
/**
 * A loaned zenoh configuration.
 */
typedef struct ALIGN(8) z_loaned_config_t {
  uint8_t _0[1544];
} z_loaned_config_t;
/**
 * A zenoh-allocated key expression.
 *
 * Key expressions can identify a single key or a set of keys.
 *
 * Examples :
 *    - ``"key/expression"``.
 *    - ``"key/ex*"``.
 *
 * Key expressions can be mapped to numerical ids through :c:func:`z_declare_expr`
 * for wire and computation efficiency.
 *
 * A `key expression <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Key%20Expressions.md>`_ can be either:
 *   - A plain string expression.
 *   - A pure numerical id.
 *   - The combination of a numerical prefix and a string suffix.
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
typedef struct ALIGN(8) z_owned_keyexpr_t {
  uint8_t _0[32];
} z_owned_keyexpr_t;
/**
 * A loaned key expression.
 *
 * Key expressions can identify a single key or a set of keys.
 *
 * Examples :
 *    - ``"key/expression"``.
 *    - ``"key/ex*"``.
 *
 * Using :c:func:`z_declare_keyexpr` allows zenoh to optimize a key expression,
 * both for local processing and network-wise.
 */
typedef struct ALIGN(8) z_loaned_keyexpr_t {
  uint8_t _0[32];
} z_loaned_keyexpr_t;
/**
 * An owned zenoh publisher.
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
typedef struct ALIGN(8) z_owned_publisher_t {
  uint8_t _0[56];
} z_owned_publisher_t;
/**
 * Options passed to the :c:func:`z_declare_publisher` function.
 *
 * Members:
 *     z_congestion_control_t congestion_control: The congestion control to apply when routing messages from this publisher.
 *     z_priority_t priority: The priority of messages from this publisher.
 */
typedef struct z_publisher_options_t {
  enum z_congestion_control_t congestion_control;
  enum z_priority_t priority;
} z_publisher_options_t;
/**
 * Options passed to the :c:func:`z_declare_queryable` function.
 *
 * Members:
 *     bool complete: The completeness of the Queryable.
 */
typedef struct z_queryable_options_t {
  bool complete;
} z_queryable_options_t;
/**
 * Options passed to the :c:func:`z_declare_subscriber` or :c:func:`z_declare_pull_subscriber` function.
 *
 * Members:
 *     z_reliability_t reliability: The subscription reliability.
 */
typedef struct z_subscriber_options_t {
  enum z_reliability_t reliability;
} z_subscriber_options_t;
/**
 * Options passed to the :c:func:`z_delete` function.
 */
typedef struct z_delete_options_t {
  enum z_congestion_control_t congestion_control;
  enum z_priority_t priority;
} z_delete_options_t;
typedef struct ALIGN(8) z_owned_encoding_t {
  uint8_t _0[48];
} z_owned_encoding_t;
/**
 * The encoding of a payload, in a MIME-like format.
 *
 * For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
 */
typedef struct ALIGN(8) z_loaned_encoding_t {
  uint8_t _0[48];
} z_loaned_encoding_t;
/**
 * The replies consolidation strategy to apply on replies to a :c:func:`z_get`.
 */
typedef struct z_query_consolidation_t {
  enum z_consolidation_mode_t mode;
} z_query_consolidation_t;
/**
 * Options passed to the :c:func:`z_get` function.
 *
 * Members:
 *     z_loaned_query_target_t target: The Queryables that should be target of the query.
 *     z_query_consolidation_t consolidation: The replies consolidation strategy to apply on replies to the query.
 *     z_loaned_value_t value: An optional value to attach to the query.
 *    z_loaned_bytes_t attachment: The attachment to attach to the query.
 *     uint64_t timeout: The timeout for the query in milliseconds. 0 means default query timeout from zenoh configuration.
 */
typedef struct z_get_options_t {
  enum z_loaned_query_target_t target;
  struct z_query_consolidation_t consolidation;
  struct z_owned_bytes_t *payload;
  struct z_owned_encoding_t *encoding;
  struct z_owned_bytes_t *attachment;
  uint64_t timeout_ms;
} z_get_options_t;
/**
 * A zenoh-allocated hello message returned by a zenoh entity to a scout message sent with `z_scout`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct ALIGN(8) z_owned_hello_t {
  uint8_t _0[48];
} z_owned_hello_t;
/**
 * An array of maybe-owned slices
 *
 */
typedef struct ALIGN(8) z_owned_slice_array_t {
  uint8_t _0[24];
} z_owned_slice_array_t;
typedef struct ALIGN(8) z_view_slice_t {
  uint8_t _0[16];
} z_view_slice_t;
typedef struct ALIGN(8) z_timestamp_t {
  uint8_t _0[24];
} z_timestamp_t;
typedef struct ALIGN(8) z_owned_mutex_t {
  uint8_t _0[32];
} z_owned_mutex_t;
typedef struct ALIGN(8) z_loaned_publisher_t {
  uint8_t _0[56];
} z_loaned_publisher_t;
/**
 * Represents the set of options that can be applied to the delete operation by a previously declared publisher,
 * whenever issued via :c:func:`z_publisher_delete`.
 */
typedef struct z_publisher_delete_options_t {
  uint8_t __dummy;
} z_publisher_delete_options_t;
/**
 * Options passed to the :c:func:`z_publisher_put` function.
 *
 * Members:
 *     z_owned_encoding_t encoding: The encoding of the payload.
 *    z_owned_bytes_t attachment: The attachment to attach to the publication.
 */
typedef struct z_publisher_put_options_t {
  struct z_owned_encoding_t *encoding;
  struct z_owned_bytes_t *attachment;
} z_publisher_put_options_t;
/**
 * Options passed to the :c:func:`z_put` function.
 *
 * Members:
 *     z_loaned_encoding_t encoding: The encoding of the payload.
 *     z_congestion_control_t congestion_control: The congestion control to apply when routing this message.
 *     z_priority_t priority: The priority of this message.
 *    z_loaned_bytes_t attachment: The attachment to this message.
 */
typedef struct z_put_options_t {
  struct z_owned_encoding_t *encoding;
  enum z_congestion_control_t congestion_control;
  enum z_priority_t priority;
  struct z_owned_bytes_t *attachment;
} z_put_options_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 * - `this` is a pointer to an arbitrary state.
 * - `call` is the typical callback function. `this` will be passed as its last argument.
 * - `drop` allows the callback's state to be freed.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * We guarantee that:
 * - `call` will never be called once `drop` has started.
 * - `drop` will only be called ONCE, and AFTER EVERY `call` has ended.
 * - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 */
typedef struct z_owned_query_channel_closure_t {
  void *context;
  bool (*call)(struct z_owned_query_t*, void*);
  void (*drop)(void*);
} z_owned_query_channel_closure_t;
/**
 * A pair of closures
 */
typedef struct z_owned_query_channel_t {
  struct z_owned_closure_query_t send;
  struct z_owned_query_channel_closure_t recv;
} z_owned_query_channel_t;
/**
 * Represents the set of options that can be applied to a query reply,
 * sent via :c:func:`z_query_reply`.
 *
 * Members:
 *   z_owned_encoding_t encoding: The encoding of the payload.
 *  z_owned_bytes_t attachment: The attachment to this reply.
 */
typedef struct z_query_reply_options_t {
  struct z_owned_encoding_t *encoding;
  struct z_owned_bytes_t *attachment;
} z_query_reply_options_t;
typedef struct ALIGN(8) z_loaned_value_t {
  uint8_t _0[88];
} z_loaned_value_t;
/**
 * An owned reply to a :c:func:`z_get`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct ALIGN(8) z_owned_reply_t {
  uint8_t _0[256];
} z_owned_reply_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 * - `this` is a pointer to an arbitrary state.
 * - `call` is the typical callback function. `this` will be passed as its last argument.
 * - `drop` allows the callback's state to be freed.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * We guarantee that:
 * - `call` will never be called once `drop` has started.
 * - `drop` will only be called ONCE, and AFTER EVERY `call` has ended.
 * - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 */
typedef struct z_owned_reply_channel_closure_t {
  void *context;
  bool (*call)(struct z_owned_reply_t*, void*);
  void (*drop)(void*);
} z_owned_reply_channel_closure_t;
/**
 * A pair of closures, the `send` one accepting
 */
typedef struct z_owned_reply_channel_t {
  struct z_owned_closure_reply_t send;
  struct z_owned_reply_channel_closure_t recv;
} z_owned_reply_channel_t;
/**
 * An owned sample.
 *
 * This is a read only type that can only be constructed by cloning a `z_loaned_sample_t`.
 * Like all owned types, its memory must be freed by passing a mutable reference to it to `z_sample_drop`.
 */
typedef struct ALIGN(8) z_owned_sample_t {
  uint8_t _0[240];
} z_owned_sample_t;
typedef struct z_owned_scouting_config_t {
  struct z_owned_config_t _config;
  unsigned long zc_timeout_ms;
  uint8_t zc_what;
} z_owned_scouting_config_t;
typedef struct ALIGN(8) z_loaned_slice_array_t {
  uint8_t _0[24];
} z_loaned_slice_array_t;
/**
 * The body of a loop over a z_slice_map's key-value pairs.
 *
 * `key` and `value` are loaned to the body for the duration of a single call.
 * `context` is passed transparently through the iteration driver.
 *
 * Returning `true` is treated as `break`.
 */
typedef bool (*z_slice_map_iter_body_t)(const struct z_loaned_slice_t *key,
                                        const struct z_loaned_slice_t *value,
                                        void *context);
typedef struct ALIGN(8) z_loaned_subscriber_t {
  uint8_t _0[32];
} z_loaned_subscriber_t;
typedef struct ALIGN(8) z_owned_task_t {
  uint8_t _0[24];
} z_owned_task_t;
typedef struct z_task_attr_t {
  size_t _0;
} z_task_attr_t;
/**
 * Time
 * Uses system clock
 */
typedef struct z_time_t {
  uint64_t t;
} z_time_t;
typedef struct ALIGN(8) z_view_keyexpr_t {
  uint8_t _0[32];
} z_view_keyexpr_t;
typedef struct ALIGN(8) z_view_str_t {
  uint8_t _0[16];
} z_view_str_t;
/**
 * The options for `zc_liveliness_declare_token`
 */
typedef struct zc_liveliness_declaration_options_t {
  uint8_t _dummy;
} zc_liveliness_declaration_options_t;
/**
 * The options for :c:func:`zc_liveliness_declare_subscriber`
 */
typedef struct zc_liveliness_declare_subscriber_options_t {
  uint8_t _dummy;
} zc_liveliness_declare_subscriber_options_t;
/**
 * A liveliness token that can be used to provide the network with information about connectivity to its
 * declarer: when constructed, a PUT sample will be received by liveliness subscribers on intersecting key
 * expressions.
 *
 * A DELETE on the token's key expression will be received by subscribers if the token is destroyed, or if connectivity between the subscriber and the token's creator is lost.
 */
typedef struct ALIGN(8) zc_owned_liveliness_token_t {
  uint8_t _0[32];
} zc_owned_liveliness_token_t;
/**
 * The options for :c:func:`zc_liveliness_declare_subscriber`
 */
typedef struct zc_liveliness_get_options_t {
  uint32_t timeout_ms;
} zc_liveliness_get_options_t;
/**
 * A struct that indicates if there exist Subscribers matching the Publisher's key expression.
 *
 * Members:
 *   bool matching: true if there exist Subscribers matching the Publisher's key expression.
 */
typedef struct zcu_matching_status_t {
  bool matching;
} zcu_matching_status_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 *
 * Members:
 *   void *context: a pointer to an arbitrary state.
 *   void *call(const struct z_owned_reply_t*, const void *context): the typical callback function. `context` will be passed as its last argument.
 *   void *drop(void*): allows the callback's state to be freed.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 */
typedef struct zcu_owned_closure_matching_status_t {
  void *context;
  void (*call)(const struct zcu_matching_status_t*, void*);
  void (*drop)(void*);
} zcu_owned_closure_matching_status_t;
/**
 * An owned zenoh matching listener. Destroying the matching listener cancels the subscription.
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
typedef struct ALIGN(8) zcu_owned_matching_listener_t {
  uint8_t _0[40];
} zcu_owned_matching_listener_t;
/**
 * An owned zenoh publication_cache.
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
typedef struct ALIGN(8) ze_owned_publication_cache_t {
  uint8_t _0[96];
} ze_owned_publication_cache_t;
/**
 * Options passed to the :c:func:`ze_declare_publication_cache` function.
 *
 * Members:
 *     z_loaned_keyexpr_t queryable_prefix: The prefix used for queryable
 *     zcu_locality_t queryable_origin: The restriction for the matching queries that will be receive by this
 *                       publication cache
 *     bool queryable_complete: the `complete` option for the queryable
 *     size_t history: The the history size
 *     size_t resources_limit: The limit number of cached resources
 */
typedef struct ze_publication_cache_options_t {
  const struct z_loaned_keyexpr_t *queryable_prefix;
  enum zcu_locality_t queryable_origin;
  bool queryable_complete;
  size_t history;
  size_t resources_limit;
} ze_publication_cache_options_t;
/**
 * An owned zenoh querying subscriber. Destroying the subscriber cancels the subscription.
 *
 * Like most `ze_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `ze_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct ALIGN(8) ze_owned_querying_subscriber_t {
  uint8_t _0[64];
} ze_owned_querying_subscriber_t;
/**
 * Represents the set of options that can be applied to a querying subscriber,
 * upon its declaration via :c:func:`ze_declare_querying_subscriber`.
 *
 * Members:
 *   z_reliability_t reliability: The subscription reliability.
 *   zcu_locality_t allowed_origin: The restriction for the matching publications that will be
 *                                  receive by this subscriber.
 *   z_loaned_keyexpr_t query_selector: The selector to be used for queries.
 *   z_loaned_query_target_t query_target: The target to be used for queries.
 *   z_query_consolidation_t query_consolidation: The consolidation mode to be used for queries.
 *   zcu_reply_keyexpr_t query_accept_replies: The accepted replies for queries.
 *   uint64_t query_timeout_ms: The timeout to be used for queries.
 */
typedef struct ze_querying_subscriber_options_t {
  enum z_reliability_t reliability;
  enum zcu_locality_t allowed_origin;
  const struct z_loaned_keyexpr_t *query_selector;
  enum z_loaned_query_target_t query_target;
  struct z_query_consolidation_t query_consolidation;
  enum zcu_reply_keyexpr_t query_accept_replies;
  uint64_t query_timeout_ms;
} ze_querying_subscriber_options_t;
typedef struct ALIGN(8) ze_loaned_querying_subscriber_t {
  uint8_t _0[64];
} ze_loaned_querying_subscriber_t;
ZENOHC_API extern const unsigned int Z_ROUTER;
ZENOHC_API extern const unsigned int Z_PEER;
ZENOHC_API extern const unsigned int Z_CLIENT;
ZENOHC_API extern const char *Z_CONFIG_MODE_KEY;
ZENOHC_API extern const char *Z_CONFIG_CONNECT_KEY;
ZENOHC_API extern const char *Z_CONFIG_LISTEN_KEY;
ZENOHC_API extern const char *Z_CONFIG_USER_KEY;
ZENOHC_API extern const char *Z_CONFIG_PASSWORD_KEY;
ZENOHC_API extern const char *Z_CONFIG_MULTICAST_SCOUTING_KEY;
ZENOHC_API extern const char *Z_CONFIG_MULTICAST_INTERFACE_KEY;
ZENOHC_API extern const char *Z_CONFIG_MULTICAST_IPV4_ADDRESS_KEY;
ZENOHC_API extern const char *Z_CONFIG_SCOUTING_TIMEOUT_KEY;
ZENOHC_API extern const char *Z_CONFIG_SCOUTING_DELAY_KEY;
ZENOHC_API extern const char *Z_CONFIG_ADD_TIMESTAMP_KEY;
/**
 * Returns `true` if the payload is in a valid state.
 */
ZENOHC_API bool z_bytes_check(const struct z_owned_bytes_t *payload);
/**
 * Increments the payload's reference count, returning an owned version of it.
 */
ZENOHC_API void z_bytes_clone(const struct z_loaned_bytes_t *src, struct z_owned_bytes_t *dst);
/**
 * Decodes payload into owned bytes
 */
ZENOHC_API
z_error_t z_bytes_decode_into_bytes(const struct z_loaned_bytes_t *payload,
                                    struct z_owned_slice_t *dst);
/**
 * Decodes payload into bytes map.
 */
ZENOHC_API
z_error_t z_bytes_decode_into_bytes_map(const struct z_loaned_bytes_t *payload,
                                        struct z_owned_slice_map_t *dst);
/**
 * Decodes payload into null-terminated string.
 */
ZENOHC_API
z_error_t z_bytes_decode_into_string(const struct z_loaned_bytes_t *payload,
                                     struct z_owned_str_t *dst);
/**
 * Decrements the payload's reference counter, destroying it if applicable.
 *
 * `this` will be reset to `z_buffer_null`, preventing UB on double-frees.
 */
ZENOHC_API void z_bytes_drop(struct z_owned_bytes_t *this_);
/**
 * Encodes bytes map by copying.
 */
ZENOHC_API
void z_bytes_encode_from_bytes_map(struct z_owned_bytes_t *this_,
                                   const struct z_loaned_slice_map_t *bytes_map);
/**
 * Encodes byte sequence by aliasing.
 */
ZENOHC_API
void z_bytes_encode_from_slice(struct z_owned_bytes_t *this_,
                               const struct z_loaned_slice_t *bytes);
/**
 * Encodes a loaned string by aliasing.
 */
ZENOHC_API
void z_bytes_encode_from_string(struct z_owned_bytes_t *this_,
                                const struct z_loaned_str_t *s);
/**
 * Returns total number bytes in the payload.
 */
ZENOHC_API size_t z_bytes_len(const struct z_loaned_bytes_t *payload);
/**
 * Loans the payload, allowing you to call functions that only need a loan of it.
 */
ZENOHC_API const struct z_loaned_bytes_t *z_bytes_loan(const struct z_owned_bytes_t *payload);
/**
 * The gravestone value for `z_owned_bytes_t`.
 */
ZENOHC_API void z_bytes_null(struct z_owned_bytes_t *this_);
ZENOHC_API bool z_bytes_reader_check(const struct z_owned_bytes_reader_t *this_);
ZENOHC_API void z_bytes_reader_drop(struct z_owned_bytes_reader_t *this_);
ZENOHC_API
const struct z_loaned_bytes_reader_t *z_bytes_reader_loan(const struct z_owned_bytes_reader_t *reader);
ZENOHC_API
struct z_loaned_bytes_reader_t *z_bytes_reader_loan_mut(struct z_owned_bytes_reader_t *reader);
/**
 * Creates a reader for the specified `payload`.
 *
 * Returns 0 in case of success, -1 if `payload` is not valid.
 */
ZENOHC_API
void z_bytes_reader_new(struct z_loaned_bytes_t payload,
                        struct z_owned_bytes_reader_t *this_);
ZENOHC_API void z_bytes_reader_null(struct z_owned_bytes_reader_t *this_);
/**
 * Reads data into specified destination.
 *
 * Will read at most `len` bytes.
 * Returns number of bytes read. If return value is smaller than `len`, it means that end of the payload was reached.
 */
ZENOHC_API
size_t z_bytes_reader_read(struct z_loaned_bytes_reader_t *this_,
                           uint8_t *dest,
                           size_t len);
/**
 * Sets the `reader` position indicator for the payload to the value pointed to by offset.
 * The new position is exactly offset bytes measured from the beginning of the payload if origin is SEEK_SET,
 * from the current reader position if origin is SEEK_CUR, and from the end of the payload if origin is SEEK_END.
 * Return ​0​ upon success, negative error code otherwise.
 */
ZENOHC_API
z_error_t z_bytes_reader_seek(struct z_loaned_bytes_reader_t *this_,
                              int64_t offset,
                              int origin);
ZENOHC_API uint64_t z_clock_elapsed_ms(const struct z_clock_t *time);
ZENOHC_API uint64_t z_clock_elapsed_s(const struct z_clock_t *time);
ZENOHC_API uint64_t z_clock_elapsed_us(const struct z_clock_t *time);
ZENOHC_API struct z_clock_t z_clock_now(void);
/**
 * Closes a zenoh session. This drops and invalidates `session` for double-drop safety.
 *
 * Returns a negative value if an error occured while closing the session.
 * Returns the remaining reference count of the session otherwise, saturating at i8::MAX.
 */
ZENOHC_API z_error_t z_close(struct z_owned_session_t *session);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_hello_call(const struct z_owned_closure_hello_t *closure,
                          const struct z_loaned_hello_t *hello);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_hello_drop(struct z_owned_closure_hello_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_closure_hello_t' type
 */
ZENOHC_API void z_closure_hello_null(struct z_owned_closure_hello_t *this_);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_owned_query_call(const struct z_owned_closure_owned_query_t *closure,
                                struct z_owned_query_t *query);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_owned_query_drop(struct z_owned_closure_owned_query_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_closure_query_t' type
 */
ZENOHC_API struct z_owned_closure_owned_query_t z_closure_owned_query_null(void);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_query_call(const struct z_owned_closure_query_t *closure,
                          const struct z_loaned_query_t *query);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_query_drop(struct z_owned_closure_query_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_closure_query_t' type
 */
ZENOHC_API void z_closure_query_null(struct z_owned_closure_query_t *this_);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_reply_call(const struct z_owned_closure_reply_t *closure,
                          const struct z_loaned_reply_t *reply);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_reply_drop(struct z_owned_closure_reply_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_closure_reply_t' type
 */
ZENOHC_API void z_closure_reply_null(struct z_owned_closure_reply_t *this_);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_sample_call(const struct z_owned_closure_sample_t *closure,
                           const struct z_loaned_sample_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_sample_drop(struct z_owned_closure_sample_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_closure_sample_t' type
 */
ZENOHC_API void z_closure_sample_null(struct z_owned_closure_sample_t *this_);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_zid_call(const struct z_owned_closure_zid_t *closure,
                        const struct z_id_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_zid_drop(struct z_owned_closure_zid_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_closure_zid_t' type
 */
ZENOHC_API void z_closure_zid_null(struct z_owned_closure_zid_t *this_);
ZENOHC_API bool z_condvar_check(const struct z_owned_condvar_t *this_);
ZENOHC_API void z_condvar_drop(struct z_owned_condvar_t *this_);
ZENOHC_API void z_condvar_init(struct z_owned_condvar_t *this_);
ZENOHC_API const struct z_loaned_condvar_t *z_condvar_loan(const struct z_owned_condvar_t *this_);
ZENOHC_API struct z_loaned_condvar_t *z_condvar_loan_mut(struct z_owned_condvar_t *this_);
ZENOHC_API void z_condvar_null(struct z_owned_condvar_t *this_);
ZENOHC_API z_error_t z_condvar_signal(const struct z_loaned_condvar_t *this_);
ZENOHC_API
z_error_t z_condvar_wait(const struct z_loaned_condvar_t *this_,
                         struct z_loaned_mutex_t *m);
/**
 * Returns ``true`` if `config` is valid.
 */
ZENOHC_API bool z_config_check(const struct z_owned_config_t *config);
/**
 * Constructs a default, zenoh-allocated, client mode configuration.
 * If `peer` is not null, it is added to the configuration as remote peer.
 */
ZENOHC_API
z_error_t z_config_client(struct z_owned_config_t *this_,
                          const char *const *peers,
                          size_t n_peers);
/**
 * Clones the config.
 */
ZENOHC_API void z_config_clone(const struct z_loaned_config_t *src, struct z_owned_config_t *dst);
/**
 * Return a new, zenoh-allocated, empty configuration.
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
ZENOHC_API
void z_config_default(struct z_owned_config_t *this_);
/**
 * Frees `config`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_config_drop(struct z_owned_config_t *config);
/**
 * Returns a :c:type:`z_loaned_config_t` loaned from `s`.
 */
ZENOHC_API const struct z_loaned_config_t *z_config_loan(const struct z_owned_config_t *this_);
/**
 * Returns a :c:type:`z_loaned_config_t` loaned from `s`.
 */
ZENOHC_API struct z_loaned_config_t *z_config_loan_mut(struct z_owned_config_t *this_);
/**
 * Constructs a null safe-to-drop value of 'z_owned_config_t' type
 */
ZENOHC_API void z_config_null(struct z_owned_config_t *this_);
/**
 * Constructs a default, zenoh-allocated, peer mode configuration.
 */
ZENOHC_API void z_config_peer(struct z_owned_config_t *this_);
/**
 * Declare a key expression. The id is returned as a :c:type:`z_loaned_keyexpr_t` with a nullptr suffix.
 *
 * This numerical id will be used on the network to save bandwidth and
 * ease the retrieval of the concerned resource in the routing tables.
 */
ZENOHC_API
z_error_t z_declare_keyexpr(struct z_owned_keyexpr_t *this_,
                            const struct z_loaned_session_t *session,
                            const struct z_loaned_keyexpr_t *key_expr);
/**
 * Declares a publisher for the given key expression.
 *
 * Data can be put and deleted with this publisher with the help of the
 * :c:func:`z_publisher_put` and :c:func:`z_publisher_delete` functions.
 *
 * Parameters:
 *     session: The zenoh session.
 *     key_expr: The key expression to publish.
 *     options: additional options for the publisher.
 *
 * Returns:
 *    A :c:type:`z_owned_publisherr_t`.
 *
 *    To check if the publisher decalration succeeded and if the publisher is still valid,
 *    you may use `z_publisher_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 *
 *    Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 *    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 *    After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * Example:
 *    Declaring a publisher passing `NULL` for the options:
 *
 *    .. code-block:: C
 *
 *       z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(expr), NULL);
 *
 *    is equivalent to initializing and passing the default publisher options:
 *
 *    .. code-block:: C
 *
 *       z_publisher_options_t opts = z_publisher_options_default();
 *       z_owned_publisher_t sub = z_declare_publisher(z_loan(s), z_keyexpr(expr), &opts);
 */
ZENOHC_API
z_error_t z_declare_publisher(struct z_owned_publisher_t *this_,
                              const struct z_loaned_session_t *session,
                              const struct z_loaned_keyexpr_t *key_expr,
                              const struct z_publisher_options_t *options);
/**
 * Creates a Queryable for the given key expression.
 *
 * Parameters:
 *     session: The zenoh session.
 *     key_expr: The key expression the Queryable will reply to.
 *     callback: The callback function that will be called each time a matching query is received.
 *     options: Options for the queryable.
 *
 * Returns:
 *    The created :c:type:`z_owned_queryable_t` or ``null`` if the creation failed.
 */
ZENOHC_API
z_error_t z_declare_queryable(struct z_owned_queryable_t *this_,
                              const struct z_loaned_session_t *session,
                              const struct z_loaned_keyexpr_t *key_expr,
                              struct z_owned_closure_query_t *callback,
                              struct z_queryable_options_t *options);
/**
 * Declare a subscriber for a given key expression.
 *
 * Parameters:
 *     session: The zenoh session.
 *     key_expr: The key expression to subscribe.
 *     callback: The callback function that will be called each time a data matching the subscribed expression is received.
 *     opts: The options to be passed to describe the options to be passed to the subscriber declaration.
 *
 * Returns:
 *    A :c:type:`z_owned_subscriber_t`.
 *
 *    To check if the subscription succeeded and if the subscriber is still valid,
 *    you may use `z_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 *
 *    Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 *    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 *    After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * Example:
 *    Declaring a subscriber passing `NULL` for the options:
 *
 *    .. code-block:: C
 *
 *       z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(expr), callback, NULL);
 *
 *    is equivalent to initializing and passing the default subscriber options:
 *
 *    .. code-block:: C
 *
 *       z_subscriber_options_t opts = z_subscriber_options_default();
 *       z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
 */
ZENOHC_API
z_error_t z_declare_subscriber(struct z_owned_subscriber_t *this_,
                               const struct z_loaned_session_t *session,
                               const struct z_loaned_keyexpr_t *key_expr,
                               struct z_owned_closure_sample_t *callback,
                               struct z_subscriber_options_t *options);
/**
 * Delete data.
 *
 * Parameters:
 *     session: The zenoh session.
 *     key_expr: The key expression to delete.
 *     options: The delete options.
 * Returns:
 *     ``0`` in case of success, negative values in case of failure.
 */
ZENOHC_API
z_error_t z_delete(const struct z_loaned_session_t *session,
                   const struct z_loaned_keyexpr_t *key_expr,
                   struct z_delete_options_t *options);
/**
 * Constructs the default value for :c:type:`z_put_options_t`.
 */
ZENOHC_API void z_delete_options_default(struct z_delete_options_t *this_);
/**
 * Returns ``true`` if `encoding` is valid.
 */
ZENOHC_API bool z_encoding_check(const struct z_owned_encoding_t *encoding);
/**
 * Constructs a default :c:type:`z_loaned_encoding_t`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_default(void);
/**
 * Frees `encoding`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_encoding_drop(struct z_owned_encoding_t *encoding);
/**
 * Constructs a specific :c:type:`z_loaned_encoding_t`.
 */
ZENOHC_API int8_t z_encoding_from_str(struct z_owned_encoding_t *encoding, const char *s);
/**
 * Returns a :c:type:`z_loaned_encoding_t` loaned from `encoding`.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_encoding_loan(const struct z_owned_encoding_t *encoding);
/**
 * Constructs a null safe-to-drop value of 'z_owned_encoding_t' type
 */
ZENOHC_API void z_encoding_null(struct z_owned_encoding_t *encoding);
/**
 * Query data from the matching queryables in the system.
 * Replies are provided through a callback function.
 *
 * Returns a negative value upon failure.
 *
 * Parameters:
 *     session: The zenoh session.
 *     key_expr: The key expression matching resources to query.
 *     parameters: The query's parameters, similar to a url's query segment.
 *     callback: The callback function that will be called on reception of replies for this query.
 *               Note that the `reply` parameter of the callback is passed by mutable reference,
 *               but **will** be dropped once your callback exits to help you avoid memory leaks.
 *               If you'd rather take ownership, please refer to the documentation of :c:func:`z_reply_null`
 *     options: additional options for the get.
 */
ZENOHC_API
z_error_t z_get(const struct z_loaned_session_t *session,
                const struct z_loaned_keyexpr_t *key_expr,
                const char *parameters,
                struct z_owned_closure_reply_t *callback,
                struct z_get_options_t *options);
ZENOHC_API void z_get_options_default(struct z_get_options_t *this_);
ZENOHC_API bool z_hello_check(const struct z_owned_hello_t *this_);
/**
 * Frees `hello`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_hello_drop(struct z_owned_hello_t *this_);
/**
 * Returns a :c:type:`z_hello_t` loaned from :c:type:`z_owned_hello_t`.
 */
ZENOHC_API const struct z_loaned_hello_t *z_hello_loan(const struct z_owned_hello_t *this_);
/**
 * Returns an array of non-owned locators as an array of non-null terminated strings.
 *
 * The lifetime of locator strings is bound to `this`.
 */
ZENOHC_API
void z_hello_locators(const struct z_loaned_hello_t *this_,
                      struct z_owned_slice_array_t *locators_out);
ZENOHC_API void z_hello_null(struct z_owned_hello_t *this_);
ZENOHC_API uint8_t z_hello_whatami(const struct z_loaned_hello_t *this_);
ZENOHC_API struct z_id_t z_hello_zid(const struct z_loaned_hello_t *this_);
/**
 * Fetches the Zenoh IDs of all connected peers.
 *
 * `callback` will be called once for each ID, is guaranteed to never be called concurrently,
 * and is guaranteed to be dropped before this function exits.
 *
 * Retuns 0 on success, negative values on failure
 */
ZENOHC_API
z_error_t z_info_peers_zid(const struct z_loaned_session_t *session,
                           struct z_owned_closure_zid_t *callback);
/**
 * Fetches the Zenoh IDs of all connected routers.
 *
 * `callback` will be called once for each ID, is guaranteed to never be called concurrently,
 * and is guaranteed to be dropped before this function exits.
 *
 * Retuns 0 on success, negative values on failure.
 */
ZENOHC_API
z_error_t z_info_routers_zid(const struct z_loaned_session_t *session,
                             struct z_owned_closure_zid_t *callback);
/**
 * Returns the local Zenoh ID.
 *
 * Unless the `session` is invalid, that ID is guaranteed to be non-zero.
 * In other words, this function returning an array of 16 zeros means you failed
 * to pass it a valid session.
 */
ZENOHC_API struct z_id_t z_info_zid(const struct z_loaned_session_t *session);
/**
 * Returns the key expression's internal string by aliasing it.
 *
 * Currently exclusive to zenoh-c
 */
ZENOHC_API void z_keyexpr_as_bytes(const struct z_loaned_keyexpr_t *ke, struct z_view_slice_t *b);
/**
 * Canonizes the passed string in place, possibly shortening it by modifying `len`.
 *
 * Returns ``0`` upon success, negative values upon failure.
 * Returns a negative value if canonization failed, which indicates that the passed string was an invalid
 * key expression for reasons other than a non-canon form.
 *
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 */
ZENOHC_API
z_error_t z_keyexpr_canonize(char *start,
                             size_t *len);
/**
 * Canonizes the passed string in place, possibly shortening it by placing a new null-terminator.
 *
 * Returns ``0`` upon success, negative values upon failure.
 * Returns a negative value if canonization failed, which indicates that the passed string was an invalid
 * key expression for reasons other than a non-canon form.
 *
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 */
ZENOHC_API
z_error_t z_keyexpr_canonize_null_terminated(char *start);
/**
 * Returns ``true`` if `keyexpr` is valid.
 */
ZENOHC_API bool z_keyexpr_check(const struct z_owned_keyexpr_t *keyexpr);
/**
 * Performs string concatenation and returns the result as a `z_owned_keyexpr_t`.
 * In case of error, the return value will be set to its invalidated state.
 *
 * You should probably prefer `z_keyexpr_join` as Zenoh may then take advantage of the hierachical separation it inserts.
 *
 * To avoid odd behaviors, concatenating a key expression starting with `*` to one ending with `*` is forbidden by this operation,
 * as this would extremely likely cause bugs.
 */
ZENOHC_API
z_error_t z_keyexpr_concat(struct z_owned_keyexpr_t *this_,
                           const struct z_loaned_keyexpr_t *left,
                           const char *right_start,
                           size_t right_len);
/**
 * Frees `keyexpr` and invalidates it for double-drop safety.
 */
ZENOHC_API void z_keyexpr_drop(struct z_owned_keyexpr_t *keyexpr);
/**
 * Returns ``0`` if both ``left`` and ``right`` are equal.
 */
ZENOHC_API
bool z_keyexpr_equals(const struct z_loaned_keyexpr_t *left,
                      const struct z_loaned_keyexpr_t *right);
/**
 * Returns ``0`` if ``left`` includes ``right``, i.e. the set defined by ``left`` contains every key belonging to the set
 * defined by ``right``.
 */
ZENOHC_API
bool z_keyexpr_includes(const struct z_loaned_keyexpr_t *left,
                        const struct z_loaned_keyexpr_t *right);
/**
 * Returns ``0`` if the keyexprs intersect, i.e. there exists at least one key which is contained in both of the
 * sets defined by ``left`` and ``right``.
 */
ZENOHC_API
bool z_keyexpr_intersects(const struct z_loaned_keyexpr_t *left,
                          const struct z_loaned_keyexpr_t *right);
/**
 * Returns ``0`` if the passed string is a valid (and canon) key expression.
 * Otherwise returns error value
 */
ZENOHC_API z_error_t z_keyexpr_is_canon(const char *start, size_t len);
/**
 * Performs path-joining (automatically inserting) and returns the result as a `z_owned_keyexpr_t`.
 * In case of error, the return value will be set to its invalidated state.
 */
ZENOHC_API
z_error_t z_keyexpr_join(struct z_owned_keyexpr_t *this_,
                         const struct z_loaned_keyexpr_t *left,
                         const struct z_loaned_keyexpr_t *right);
/**
 * Returns a :c:type:`z_loaned_keyexpr_t` loaned from :c:type:`z_owned_keyexpr_t`.
 */
ZENOHC_API
const struct z_loaned_keyexpr_t *z_keyexpr_loan(const struct z_owned_keyexpr_t *key_expr);
/**
 * Constructs a :c:type:`z_owned_keyexpr_t` departing from a string, copying the passed string.
 */
ZENOHC_API z_error_t z_keyexpr_new(const char *name, struct z_owned_keyexpr_t *this_);
/**
 * Constructs a :c:type:`z_owned_keyexpr_t` departing from a string, copying the passed string. The copied string is canonized.
 */
ZENOHC_API
z_error_t z_keyexpr_new_autocanonize(const char *name,
                                     struct z_owned_keyexpr_t *this_);
/**
 * Constructs a null safe-to-drop value of 'z_owned_keyexpr_t' type
 */
ZENOHC_API void z_keyexpr_null(struct z_owned_keyexpr_t *this_);
/**
 * Returns the relation between `left` and `right` from `left`'s point of view.
 *
 * Note that this is slower than `z_keyexpr_intersects` and `keyexpr_includes`, so you should favor these methods for most applications.
 */
ZENOHC_API
enum z_keyexpr_intersection_level_t z_keyexpr_relation_to(const struct z_loaned_keyexpr_t *left,
                                                          const struct z_loaned_keyexpr_t *right);
/**
 * Constructs a null-terminated string departing from a :c:type:`z_loaned_keyexpr_t`.
 * The user is responsible of droping the returned string using `z_drop`
 */
ZENOHC_API void z_keyexpr_to_string(const struct z_loaned_keyexpr_t *ke, struct z_owned_str_t *s);
/**
 * Returns the read position indicator.
 * Returns read position indicator on success or -1L if failure occurs.
 */
ZENOHC_API int64_t z_loaned_bytes_reader_tell(struct z_loaned_bytes_reader_t *this_);
ZENOHC_API z_error_t z_loaned_mutex_try_lock(struct z_loaned_mutex_t *this_);
/**
 * Create a default :c:type:`z_loaned_query_target_t`.
 */
ZENOHC_API enum z_loaned_query_target_t z_loaned_query_target_default(void);
/**
 * The samples timestamp
 *
 * Returns true if Sample contains timestamp, false otherwise. In the latter case the timestamp_out value is not altered.
 */
ZENOHC_API
bool z_loaned_sample_timestamp(const struct z_loaned_sample_t *sample,
                               struct z_timestamp_t *timestamp_out);
ZENOHC_API bool z_mutex_check(const struct z_owned_mutex_t *this_);
ZENOHC_API void z_mutex_drop(struct z_owned_mutex_t *this_);
ZENOHC_API z_error_t z_mutex_init(struct z_owned_mutex_t *this_);
ZENOHC_API struct z_loaned_mutex_t *z_mutex_loan_mut(struct z_owned_mutex_t *this_);
ZENOHC_API z_error_t z_mutex_lock(struct z_loaned_mutex_t *this_);
ZENOHC_API void z_mutex_null(struct z_owned_mutex_t *this_);
ZENOHC_API z_error_t z_mutex_unlock(struct z_loaned_mutex_t *this_);
/**
 * Opens a zenoh session. Should the session opening fail, `z_check` ing the returned value will return `false`.
 * Config value is always consumed upon function return.
 */
ZENOHC_API
z_error_t z_open(struct z_owned_session_t *this_,
                 struct z_owned_config_t *config);
/**
 * Returns ``true`` if `this` is initialized.
 */
ZENOHC_API bool z_owned_slice_check(const struct z_owned_slice_t *this_);
/**
 * Returns ``true`` if `pub` is valid.
 */
ZENOHC_API bool z_publisher_check(const struct z_owned_publisher_t *this_);
/**
 * Sends a `DELETE` message onto the publisher's key expression.
 *
 * Returns:
 *     ``0`` in case of success, ``1`` in case of failure.
 */
ZENOHC_API
z_error_t z_publisher_delete(const struct z_loaned_publisher_t *publisher,
                             struct z_publisher_delete_options_t _options);
/**
 * Constructs the default values for the delete operation via a publisher entity.
 *
 * Returns:
 *   Returns the constructed :c:type:`z_publisher_delete_options_t`.
 */
ZENOHC_API void z_publisher_delete_options_default(struct z_publisher_delete_options_t *this_);
/**
 * Returns the key expression of the publisher
 */
ZENOHC_API
const struct z_loaned_keyexpr_t *z_publisher_keyexpr(const struct z_loaned_publisher_t *publisher);
/**
 * Returns a :c:type:`z_loaned_publisher_t` loaned from `p`.
 */
ZENOHC_API
const struct z_loaned_publisher_t *z_publisher_loan(const struct z_owned_publisher_t *this_);
/**
 * Constructs a null safe-to-drop value of 'z_owned_publisher_t' type
 */
ZENOHC_API void z_publisher_null(struct z_owned_publisher_t *this_);
/**
 * Constructs the default value for :c:type:`z_publisher_options_t`.
 */
ZENOHC_API void z_publisher_options_default(struct z_publisher_options_t *this_);
/**
 * Sends a `PUT` message onto the publisher's key expression, transfering the payload ownership.
 *
 * This is avoids copies when transfering data that was either:
 * - `z_sample_payload_rcinc`'d from a sample, when forwarding samples from a subscriber/query to a publisher
 * - constructed from a `zc_owned_shmbuf_t`
 *
 * The payload and all owned options fields are consumed upon function return.
 *
 * Parameters:
 *     session: The zenoh session.
 *     payload: The value to put.
 *     options: The publisher put options.
 * Returns:
 *     ``0`` in case of success, negative values in case of failure.
 */
ZENOHC_API
z_error_t z_publisher_put(const struct z_loaned_publisher_t *publisher,
                          struct z_owned_bytes_t *payload,
                          struct z_publisher_put_options_t *options);
/**
 * Constructs the default value for :c:type:`z_publisher_put_options_t`.
 */
ZENOHC_API void z_publisher_put_options_default(struct z_publisher_put_options_t *this_);
/**
 * Put data, transfering its ownership.
 *
 *
 * The payload's encoding and attachment can be sepcified through the options. These values are consumed upon function
 * return.
 *
 * Parameters:
 *     session: The zenoh session.
 *     key_expr: The key expression to put.
 *     payload: The value to put (consumed upon function return).
 *     options: The put options.
 * Returns:
 *     ``0`` in case of success, negative error values in case of failure.
 */
ZENOHC_API
z_error_t z_put(const struct z_loaned_session_t *session,
                const struct z_loaned_keyexpr_t *key_expr,
                struct z_owned_bytes_t *payload,
                struct z_put_options_t *options);
/**
 * Constructs the default value for :c:type:`z_put_options_t`.
 */
ZENOHC_API void z_put_options_default(struct z_put_options_t *this_);
/**
 * Gets the attachment to the query by aliasing.
 *
 * Returns NULL if query does not contain an attachment.
 */
ZENOHC_API const struct z_loaned_bytes_t *z_query_attachment(const struct z_loaned_query_t *query);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
bool z_query_channel_closure_call(const struct z_owned_query_channel_closure_t *closure,
                                  struct z_owned_query_t *query);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_query_channel_closure_drop(struct z_owned_query_channel_closure_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_query_channel_closure_t' type
 */
ZENOHC_API void z_query_channel_closure_null(struct z_owned_query_channel_closure_t *this_);
ZENOHC_API void z_query_channel_drop(struct z_owned_query_channel_t *channel);
/**
 * Constructs a null safe-to-drop value of 'z_owned_query_channel_t' type
 */
ZENOHC_API struct z_owned_query_channel_t z_query_channel_null(void);
/**
 * Returns `false` if `this` is in a gravestone state, `true` otherwise.
 *
 * This function may not be called with the null pointer, but can be called with the gravestone value.
 */
ZENOHC_API
bool z_query_check(const struct z_owned_query_t *query);
/**
 * Clones the query, allowing to keep it in an "open" state past the callback's return.
 *
 * This operation is infallible, but may return a gravestone value if `query` itself was a gravestone value (which cannot be the case in a callback).
 */
ZENOHC_API
void z_query_clone(const struct z_loaned_query_t *this_,
                   struct z_owned_query_t *dst);
/**
 * Automatic query consolidation strategy selection.
 *
 * A query consolidation strategy will automatically be selected depending the query selector.
 * If the selector contains time range properties, no consolidation is performed.
 * Otherwise the :c:func:`z_query_consolidation_latest` strategy is used.
 *
 * Returns:
 *   Returns the constructed :c:type:`z_query_consolidation_t`.
 */
ZENOHC_API struct z_query_consolidation_t z_query_consolidation_auto(void);
/**
 * Creates a default :c:type:`z_query_consolidation_t` (consolidation mode AUTO).
 */
ZENOHC_API struct z_query_consolidation_t z_query_consolidation_default(void);
/**
 * Latest value consolidation.
 */
ZENOHC_API struct z_query_consolidation_t z_query_consolidation_latest(void);
/**
 * Monotonic consolidation.
 */
ZENOHC_API struct z_query_consolidation_t z_query_consolidation_monotonic(void);
/**
 * Disable consolidation.
 */
ZENOHC_API struct z_query_consolidation_t z_query_consolidation_none(void);
/**
 * Destroys the query, setting `this` to its gravestone value to prevent double-frees.
 *
 * This function may not be called with the null pointer, but can be called with the gravestone value.
 */
ZENOHC_API
void z_query_drop(struct z_owned_query_t *this_);
/**
 * Checks if query contains a payload value.
 */
ZENOHC_API bool z_query_has_value(const struct z_loaned_query_t *query);
/**
 * Get a query's key by aliasing it.
 */
ZENOHC_API const struct z_loaned_keyexpr_t *z_query_keyexpr(const struct z_loaned_query_t *query);
/**
 * Aliases the query.
 *
 * This function may not be called with the null pointer, but can be called with the gravestone value.
 */
ZENOHC_API
const struct z_loaned_query_t *z_query_loan(const struct z_owned_query_t *this_);
/**
 * The gravestone value of `z_owned_query_t`.
 */
ZENOHC_API void z_query_null(struct z_owned_query_t *this_);
/**
 * Get a query's `value selector <https://github.com/eclipse-zenoh/roadmap/tree/main/rfcs/ALL/Selectors>`_ by aliasing it.
 */
ZENOHC_API
void z_query_parameters(const struct z_loaned_query_t *query,
                        struct z_view_slice_t *parameters);
/**
 * Send a reply to a query.
 *
 * This function must be called inside of a Queryable callback passing the
 * query received as parameters of the callback function. This function can
 * be called multiple times to send multiple replies to a query. The reply
 * will be considered complete when the Queryable callback returns.
 *
 * Parameters:
 *     query: The query to reply to.
 *     key_expr: The key of this reply.
 *     payload: The value of this reply.
 *     options: The options of this reply.
 *
 * The payload and all owned options fields are consumed upon function return.
 */
ZENOHC_API
z_error_t z_query_reply(const struct z_loaned_query_t *query,
                        const struct z_loaned_keyexpr_t *key_expr,
                        struct z_owned_bytes_t *payload,
                        struct z_query_reply_options_t *options);
/**
 * Constructs the default value for :c:type:`z_query_reply_options_t`.
 */
ZENOHC_API void z_query_reply_options_default(struct z_query_reply_options_t *this_);
/**
 * Gets a query's `payload value <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md>`_ by aliasing it.
 *
 * **WARNING: This API has been marked as unstable: it works as advertised, but it may change in a future release.**
 * Before calling this funciton, the user must ensure that `z_query_has_value` returns true.
 */
ZENOHC_API
const struct z_loaned_value_t *z_query_value(const struct z_loaned_query_t *query);
/**
 * Returns ``true`` if `qable` is valid.
 */
ZENOHC_API bool z_queryable_check(const struct z_owned_queryable_t *qable);
/**
 * Constructs a null safe-to-drop value of 'z_owned_queryable_t' type
 */
ZENOHC_API void z_queryable_null(struct z_owned_queryable_t *this_);
/**
 * Constructs the default value for :c:type:`z_query_reply_options_t`.
 */
ZENOHC_API void z_queryable_options_default(struct z_queryable_options_t *this_);
ZENOHC_API void z_random_fill(void *buf, size_t len);
ZENOHC_API uint16_t z_random_u16(void);
ZENOHC_API uint32_t z_random_u32(void);
ZENOHC_API uint64_t z_random_u64(void);
ZENOHC_API uint8_t z_random_u8(void);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
bool z_reply_channel_closure_call(const struct z_owned_reply_channel_closure_t *closure,
                                  struct z_owned_reply_t *reply);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_reply_channel_closure_drop(struct z_owned_reply_channel_closure_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_reply_channel_closure_t' type
 */
ZENOHC_API void z_reply_channel_closure_null(struct z_owned_reply_channel_closure_t *this_);
ZENOHC_API void z_reply_channel_drop(struct z_owned_reply_channel_t *channel);
/**
 * Constructs a null safe-to-drop value of 'z_owned_reply_channel_t' type
 */
ZENOHC_API void z_reply_channel_null(struct z_owned_reply_channel_t *this_);
/**
 * Returns ``true`` if `reply` is valid.
 */
ZENOHC_API bool z_reply_check(const struct z_owned_reply_t *this_);
ZENOHC_API void z_reply_clone(struct z_owned_reply_t *this_, const struct z_loaned_reply_t *reply);
/**
 * Frees `reply`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_reply_drop(struct z_owned_reply_t *this_);
/**
 * Yields the contents of the reply by asserting it indicates a failure.
 *
 * Returns null if reply does not contain a error  (i. e. if :c:func:`z_reply_is_ok` returns ``true``).
 */
ZENOHC_API
const struct z_loaned_value_t *z_reply_err(const struct z_loaned_reply_t *reply);
/**
 * Returns ``true`` if the queryable answered with an OK, which allows this value to be treated as a sample.
 *
 * If this returns ``false``, you should use :c:func:`z_check` before trying to use :c:func:`z_reply_err` if you want to process the error that may be here.
 */
ZENOHC_API
bool z_reply_is_ok(const struct z_loaned_reply_t *reply);
ZENOHC_API const struct z_loaned_reply_t *z_reply_loan(const struct z_owned_reply_t *this_);
/**
 * Returns an invalidated :c:type:`z_owned_reply_t`.
 *
 * This is useful when you wish to take ownership of a value from a callback to :c:func:`z_get`:
 *
 *     - copy the value of the callback's argument's pointee,
 *     - overwrite the pointee with this function's return value,
 *     - you are now responsible for dropping your copy of the reply.
 */
ZENOHC_API void z_reply_null(struct z_owned_reply_t *this_);
/**
 * Yields the contents of the reply by asserting it indicates a success.
 *
 * Returns null if reply does not contains a sample (i. e. if :c:func:`z_reply_is_ok` returns ``false``).
 */
ZENOHC_API
const struct z_loaned_sample_t *z_reply_ok(const struct z_loaned_reply_t *reply);
/**
 * The qos with which the sample was received.
 * TODO: split to methods (priority, congestion_control, express)
 * Gets sample's attachment.
 *
 * Returns NULL if sample does not contain an attachement.
 */
ZENOHC_API
const struct z_loaned_bytes_t *z_sample_attachment(const struct z_loaned_sample_t *sample);
/**
 * Returns `true` if `sample` is valid.
 *
 * Note that there exist no fallinle constructors for `z_owned_sample_t`, so validity is always guaranteed
 * unless the value has been dropped already.
 */
ZENOHC_API
bool z_sample_check(const struct z_owned_sample_t *sample);
/**
 * Clone a sample in the cheapest way available.
 */
ZENOHC_API void z_sample_clone(const struct z_loaned_sample_t *src, struct z_owned_sample_t *dst);
ZENOHC_API
enum z_congestion_control_t z_sample_congestion_control(const struct z_loaned_sample_t *sample);
/**
 * Destroy the sample.
 */
ZENOHC_API void z_sample_drop(struct z_owned_sample_t *sample);
/**
 * The encoding of the payload.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_sample_encoding(const struct z_loaned_sample_t *sample);
ZENOHC_API bool z_sample_express(const struct z_loaned_sample_t *sample);
/**
 * The Key Expression of the sample.
 *
 * `sample` is aliased by its return value.
 */
ZENOHC_API
const struct z_loaned_keyexpr_t *z_sample_keyexpr(const struct z_loaned_sample_t *sample);
/**
 * The sample's kind (put or delete).
 */
ZENOHC_API enum z_sample_kind_t z_sample_kind(const struct z_loaned_sample_t *sample);
/**
 * Borrow the sample, allowing calling its accessor methods.
 *
 * Calling this function using a dropped sample is undefined behaviour.
 */
ZENOHC_API const struct z_loaned_sample_t *z_sample_loan(const struct z_owned_sample_t *sample);
ZENOHC_API void z_sample_null(struct z_owned_sample_t *sample);
/**
 * The sample's data, the return value aliases the sample.
 *
 */
ZENOHC_API const struct z_loaned_bytes_t *z_sample_payload(const struct z_loaned_sample_t *sample);
ZENOHC_API enum z_priority_t z_sample_priority(const struct z_loaned_sample_t *sample);
/**
 * Scout for routers and/or peers.
 *
 * Parameters:
 *     what: A whatami bitmask of zenoh entities kind to scout for.
 *     config: A set of properties to configure the scouting.
 *     timeout: The time (in milliseconds) that should be spent scouting.
 *
 * Returns 0 if successful, negative values upon failure.
 */
ZENOHC_API
z_error_t z_scout(struct z_owned_scouting_config_t *config,
                  struct z_owned_closure_hello_t *callback);
ZENOHC_API bool z_scouting_config_check(const struct z_owned_scouting_config_t *config);
ZENOHC_API void z_scouting_config_default(struct z_owned_scouting_config_t *this_);
ZENOHC_API void z_scouting_config_drop(struct z_owned_scouting_config_t *config);
ZENOHC_API
void z_scouting_config_from(struct z_owned_scouting_config_t *this_,
                            const struct z_loaned_config_t *config);
ZENOHC_API void z_scouting_config_null(struct z_owned_scouting_config_t *this_);
/**
 * Returns ``true`` if `session` is valid.
 */
ZENOHC_API bool z_session_check(const struct z_owned_session_t *this_);
/**
 * Returns a :c:type:`z_loaned_session_t` loaned from `s`.
 *
 * This handle doesn't increase the refcount of the session, but does allow to do so with `zc_session_rcinc`.
 *
 * # Safety
 * The returned `z_loaned_session_t` aliases `z_owned_session_t`'s internal allocation,
 * attempting to use it after all owned handles to the session (including publishers, queryables and subscribers)
 * have been destroyed is UB (likely SEGFAULT)
 */
ZENOHC_API
const struct z_loaned_session_t *z_session_loan(const struct z_owned_session_t *this_);
/**
 * Constructs a null safe-to-drop value of 'z_owned_session_t' type
 */
ZENOHC_API void z_session_null(struct z_owned_session_t *this_);
ZENOHC_API int8_t z_sleep_ms(size_t time);
ZENOHC_API int8_t z_sleep_s(size_t time);
ZENOHC_API int8_t z_sleep_us(size_t time);
/**
 * Returns `true` if the array is not in its gravestone state
 */
ZENOHC_API bool z_slice_array_check(const struct z_owned_slice_array_t *this_);
/**
 * Destroys the array, resetting `this` to its gravestone value.
 *
 * This function is double-free safe, passing a pointer to the gravestone value will have no effect.
 */
ZENOHC_API void z_slice_array_drop(struct z_owned_slice_array_t *this_);
/**
 * Returns the value at the position of index in the array.
 *
 * Will return NULL if the index is out of bounds.
 */
ZENOHC_API
const struct z_loaned_slice_t *z_slice_array_get(const struct z_loaned_slice_array_t *this_,
                                                 size_t index);
/**
 * Returns true if the array is empty, false otherwise.
 */
ZENOHC_API bool z_slice_array_is_empty(const struct z_loaned_slice_array_t *this_);
/**
 * Returns number of key-value pairs in the map.
 */
ZENOHC_API size_t z_slice_array_len(const struct z_loaned_slice_array_t *this_);
ZENOHC_API
const struct z_loaned_slice_array_t *z_slice_array_loan(const struct z_owned_slice_array_t *this_);
ZENOHC_API
struct z_loaned_slice_array_t *z_slice_array_loan_mut(struct z_owned_slice_array_t *this_);
/**
 * Constructs a new empty array.
 */
ZENOHC_API void z_slice_array_new(struct z_owned_slice_array_t *this_);
/**
 * Constructs the gravestone value for `z_owned_slice_array_t`
 */
ZENOHC_API void z_slice_array_null(struct z_owned_slice_array_t *this_);
/**
 * Appends specified value to the end of the array by aliasing.
 *
 * Returns the new length of the array.
 */
ZENOHC_API
size_t z_slice_array_push_by_alias(struct z_loaned_slice_array_t *this_,
                                   const struct z_loaned_slice_t *value);
/**
 * Appends specified value to the end of the array by copying.
 *
 * Returns the new length of the array.
 */
ZENOHC_API
size_t z_slice_array_push_by_copy(struct z_loaned_slice_array_t *this_,
                                  const struct z_loaned_slice_t *value);
ZENOHC_API void z_slice_clone(const struct z_loaned_slice_t *this_, struct z_owned_slice_t *dst);
ZENOHC_API const uint8_t *z_slice_data(const struct z_loaned_slice_t *this_);
/**
 * Frees `this` and invalidates it for double-drop safety.
 */
ZENOHC_API void z_slice_drop(struct z_owned_slice_t *this_);
/**
 * Returns an empty `z_owned_slice_t`
 */
ZENOHC_API void z_slice_empty(struct z_owned_slice_t *this_);
/**
 * Copies a string into `z_owned_slice_t` using `strlen` (this should therefore not be used with untrusted inputs).
 *
 * Calling this with `str == NULL` is equivalent to `z_slice_null`.
 */
ZENOHC_API
void z_slice_from_str(struct z_owned_slice_t *this_,
                      const char *str);
ZENOHC_API size_t z_slice_len(const struct z_loaned_slice_t *this_);
ZENOHC_API const struct z_loaned_slice_t *z_slice_loan(const struct z_owned_slice_t *this_);
/**
 * Returns `true` if the map is not in its gravestone state
 */
ZENOHC_API bool z_slice_map_check(const struct z_owned_slice_map_t *map);
/**
 * Destroys the map, resetting `this` to its gravestone value.
 *
 * This function is double-free safe, passing a pointer to the gravestone value will have no effect.
 */
ZENOHC_API void z_slice_map_drop(struct z_owned_slice_map_t *this_);
/**
 * Returns the value associated with `key`.
 *
 * Will return NULL if the key is not present in the map.
 */
ZENOHC_API
const struct z_loaned_slice_t *z_slice_map_get(const struct z_loaned_slice_map_t *this_,
                                               const struct z_loaned_slice_t *key);
/**
 * Associates `value` to `key` in the map, aliasing them.
 *
 * Note that once `key` is aliased, reinserting at the same key may alias the previous instance, or the new instance of `key`.
 *
 * Returns 1 if there was already an entry associated with the key, 0 otherwise.
 */
ZENOHC_API
z_error_t z_slice_map_insert_by_alias(struct z_loaned_slice_map_t *this_,
                                      const struct z_loaned_slice_t *key,
                                      const struct z_loaned_slice_t *value);
/**
 * Associates `value` to `key` in the map, copying them to obtain ownership: `key` and `value` are not aliased past the function's return.
 *
 * Returns 1 if there was already an entry associated with the key, 0 otherwise.
 */
ZENOHC_API
uint8_t z_slice_map_insert_by_copy(struct z_loaned_slice_map_t *this_,
                                   const struct z_loaned_slice_t *key,
                                   const struct z_loaned_slice_t *value);
/**
 * Returns true if the map is empty, false otherwise.
 */
ZENOHC_API bool z_slice_map_is_empty(const struct z_loaned_slice_map_t *this_);
ZENOHC_API
void z_slice_map_iterate(const struct z_loaned_slice_map_t *this_,
                         z_slice_map_iter_body_t body,
                         void *context);
/**
 * Returns number of key-value pairs in the map.
 */
ZENOHC_API size_t z_slice_map_len(const struct z_loaned_slice_map_t *this_);
ZENOHC_API
const struct z_loaned_slice_map_t *z_slice_map_loan(const struct z_owned_slice_map_t *this_);
ZENOHC_API struct z_loaned_slice_map_t *z_slice_map_loan_mut(struct z_owned_slice_map_t *this_);
/**
 * Constructs a new empty map.
 */
ZENOHC_API void z_slice_map_new(struct z_owned_slice_map_t *this_);
/**
 * Constructs the gravestone value for `z_owned_slice_map_t`
 */
ZENOHC_API void z_slice_map_null(struct z_owned_slice_map_t *this_);
ZENOHC_API void z_slice_null(struct z_owned_slice_t *this_);
/**
 * Constructs a `len` bytes long view starting at `start`.
 */
ZENOHC_API void z_slice_wrap(struct z_owned_slice_t *this_, const uint8_t *start, size_t len);
ZENOHC_API const struct z_loaned_slice_t *z_str_as_slice(const struct z_loaned_str_t *this_);
/**
 * Returns ``true`` if `s` is a valid string
 */
ZENOHC_API bool z_str_check(const struct z_owned_str_t *this_);
ZENOHC_API void z_str_clone(const struct z_loaned_str_t *this_, struct z_owned_str_t *dst);
ZENOHC_API const char *z_str_data(const struct z_loaned_str_t *this_);
/**
 * Frees `z_owned_str_t`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_str_drop(struct z_owned_str_t *this_);
ZENOHC_API void z_str_empty(struct z_owned_str_t *this_);
/**
 * Copies a a substring of length `len`into `z_owned_str_t`.
 *
 * Calling this with `str == NULL` is equivalent to `z_str_null`.
 */
ZENOHC_API void z_str_from_substring(struct z_owned_str_t *this_, const char *str, size_t len);
/**
 * Returns :c:type:`z_loaned_str_t` structure loaned from :c:type:`z_owned_str_t`.
 */
ZENOHC_API const struct z_loaned_str_t *z_str_loan(const struct z_owned_str_t *this_);
/**
 * Returns undefined `z_owned_str_t`
 */
ZENOHC_API void z_str_null(struct z_owned_str_t *this_);
/**
 * Copies a string into `z_owned_str_t` using `strlen` (this should therefore not be used with untrusted inputs).
 *
 * Calling this with `str == NULL` is equivalent to `z_str_null`.
 */
ZENOHC_API
void z_str_wrap(struct z_owned_str_t *this_,
                const char *str);
/**
 * Returns ``true`` if `sub` is valid.
 */
ZENOHC_API bool z_subscriber_check(const struct z_owned_subscriber_t *subscriber);
/**
 * Returns the key expression of the subscriber.
 */
ZENOHC_API
const struct z_loaned_keyexpr_t *z_subscriber_keyexpr(const struct z_loaned_subscriber_t *subscriber);
/**
 * Returns a :c:type:`z_loaned_subscriber_t` loaned from `this`.
 */
ZENOHC_API
const struct z_loaned_subscriber_t *z_subscriber_loan(const struct z_owned_subscriber_t *this_);
/**
 * Constructs a null safe-to-drop value of 'z_owned_subscriber_t' type
 */
ZENOHC_API void z_subscriber_null(struct z_owned_subscriber_t *this_);
/**
 * Constructs the default value for :c:type:`z_subscriber_options_t`.
 */
ZENOHC_API void z_subscriber_options_default(struct z_subscriber_options_t *this_);
ZENOHC_API bool z_task_check(const struct z_owned_task_t *this_);
/**
 * Detaches the task and releases all allocated resources.
 */
ZENOHC_API void z_task_detach(struct z_owned_task_t *this_);
ZENOHC_API
z_error_t z_task_init(struct z_owned_task_t *this_,
                      const struct z_task_attr_t *_attr,
                      void (*fun)(void *arg),
                      void *arg);
/**
 * Joins the task and releases all allocated resources
 */
ZENOHC_API z_error_t z_task_join(struct z_owned_task_t *this_);
ZENOHC_API void z_task_null(struct z_owned_task_t *this_);
ZENOHC_API uint64_t z_time_elapsed_ms(const struct z_time_t *time);
ZENOHC_API uint64_t z_time_elapsed_s(const struct z_time_t *time);
ZENOHC_API uint64_t z_time_elapsed_us(const struct z_time_t *time);
ZENOHC_API struct z_time_t z_time_now(void);
ZENOHC_API const char *z_time_now_as_str(const char *buf, size_t len);
ZENOHC_API struct z_id_t z_timestamp_get_id(const struct z_timestamp_t *timestamp);
ZENOHC_API uint64_t z_timestamp_npt64_time(const struct z_timestamp_t *timestamp);
/**
 * Undeclare the key expression generated by a call to :c:func:`z_declare_keyexpr`.
 * The keyxpr is consumed.
 */
ZENOHC_API
z_error_t z_undeclare_keyexpr(const struct z_loaned_session_t *session,
                              struct z_owned_keyexpr_t *kexpr);
/**
 * Undeclares the given :c:type:`z_owned_publisher_t`, droping it and invalidating it for double-drop safety.
 */
ZENOHC_API
z_error_t z_undeclare_publisher(struct z_owned_publisher_t *this_);
/**
 * Undeclares a `z_owned_queryable_t`, droping it and invalidating it for doube-drop safety.
 *
 * Parameters:
 *     qable: The :c:type:`z_owned_queryable_t` to undeclare.
 */
ZENOHC_API z_error_t z_undeclare_queryable(struct z_owned_queryable_t *qable);
/**
 * Undeclares the given :c:type:`z_owned_subscriber_t`, droping it and invalidating it for double-drop safety.
 */
ZENOHC_API
z_error_t z_undeclare_subscriber(struct z_owned_subscriber_t *subscriber);
ZENOHC_API const struct z_loaned_encoding_t *z_value_encoding(const struct z_loaned_value_t *this_);
ZENOHC_API const struct z_loaned_bytes_t *z_value_payload(const struct z_loaned_value_t *this_);
/**
 * Constructs a :c:type:`z_view_keyexpr_t` departing from a string.
 * It is a loaned key expression that aliases `name`.
 */
ZENOHC_API z_error_t z_view_keyexpr(struct z_view_keyexpr_t *this_, const char *name);
/**
 * Constructs a :c:type:`z_view_keyexpr_t` by aliasing a string.
 * The string is canonized in-place before being passed to keyexpr.
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 */
ZENOHC_API
z_error_t z_view_keyexpr_autocanonize(struct z_view_keyexpr_t *this_,
                                      char *name);
/**
 * Returns ``true`` if `keyexpr` is valid.
 */
ZENOHC_API bool z_view_keyexpr_check(const struct z_view_keyexpr_t *keyexpr);
/**
 * Constructs a :c:type:`z_view_keyexpr_t` by aliasing a string.
 */
ZENOHC_API
z_error_t z_view_keyexpr_from_slice(struct z_view_keyexpr_t *this_,
                                    const char *name,
                                    size_t len);
/**
 * Constructs a :c:type:`z_view_keyexpr_t` by aliasing a string.
 * The string is canonized in-place before being passed to keyexpr.
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 */
ZENOHC_API
z_error_t z_view_keyexpr_from_slice_autocanonize(struct z_view_keyexpr_t *this_,
                                                 char *name,
                                                 size_t *len);
/**
 * Constructs a :c:type:`z_view_keyexpr_t` by aliasing a string without checking any of `z_view_keyexpr_t`'s assertions:
 * - `name` MUST be valid UTF8.
 * - `name` MUST follow the Key Expression specification, ie:
 *   - MUST NOT contain ``//``, MUST NOT start nor end with ``/``, MUST NOT contain any of the characters ``?#$``.
 *   - any instance of ``**`` may only be lead or followed by ``/``.
 *   - the key expression must have canon form.
 *
 * It is a loaned key expression that aliases `name`.
 */
ZENOHC_API
void z_view_keyexpr_from_slice_unchecked(struct z_view_keyexpr_t *this_,
                                         const char *start,
                                         size_t len);
/**
 * Returns a :c:type:`z_loaned_keyexpr_t` loaned from :c:type:`z_view_keyexpr_t`.
 */
ZENOHC_API
const struct z_loaned_keyexpr_t *z_view_keyexpr_loan(const struct z_view_keyexpr_t *key_expr);
ZENOHC_API void z_view_keyexpr_null(struct z_view_keyexpr_t *this_);
/**
 * Constructs a :c:type:`z_view_keyexpr_t` by aliasing a string without checking any of `z_view_keyexpr_t`'s assertions:
 *
 *  - `name` MUST be valid UTF8.
 *  - `name` MUST follow the Key Expression specification, ie:
 *
 *   - MUST NOT contain `//`, MUST NOT start nor end with `/`, MUST NOT contain any of the characters `?#$`.
 *   - any instance of `**` may only be lead or followed by `/`.
 *   - the key expression must have canon form.
 *
 * It is a view key expression that aliases `name`.
 */
ZENOHC_API
void z_view_keyexpr_unchecked(struct z_view_keyexpr_t *this_,
                              const char *s);
/**
 * Returns ``true`` if `this` is initialized.
 */
ZENOHC_API bool z_view_slice_check(const struct z_view_slice_t *this_);
/**
 * Returns an empty `z_view_slice_t`
 */
ZENOHC_API void z_view_slice_empty(struct z_view_slice_t *this_);
/**
 * Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
 *
 * Calling this with `str == NULL` is equivalent to `z_view_slice_null`.
 */
ZENOHC_API void z_view_slice_from_str(struct z_view_slice_t *this_, const char *str);
ZENOHC_API const struct z_loaned_slice_t *z_view_slice_loan(const struct z_view_slice_t *this_);
ZENOHC_API void z_view_slice_null(struct z_view_slice_t *this_);
/**
 * Constructs a `len` bytes long view starting at `start`.
 */
ZENOHC_API void z_view_slice_wrap(struct z_view_slice_t *this_, const uint8_t *start, size_t len);
ZENOHC_API void z_view_str_empty(struct z_view_str_t *this_);
ZENOHC_API size_t z_view_str_len(const struct z_loaned_str_t *this_);
/**
 * Returns :c:type:`z_loaned_str_t` structure loaned from :c:type:`z_view_str_t`.
 */
ZENOHC_API const struct z_loaned_str_t *z_view_str_loan(const struct z_view_str_t *this_);
/**
 * Returns undefined `z_owned_str_t`
 */
ZENOHC_API void z_view_str_null(struct z_view_str_t *this_);
/**
 * Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
 *
 * Calling this with `str == NULL` is equivalent to `z_view_str_null`.
 */
ZENOHC_API void z_view_str_wrap(struct z_view_str_t *this_, const char *str);
/**
 * Converts the kind of zenoh entity into a string.
 *
 * Parameters:
 *     whatami: A whatami bitmask of zenoh entity kind.
 *     buf: Buffer to write a null-terminated string to.
 *     len: Maximum number of bytes that can be written to the `buf`.
 *
 * Returns 0 if successful, negative values if whatami contains an invalid bitmask or `buf` is null,
 * or number of remaining bytes, if the null-terminated string size exceeds `len`.
 */
ZENOHC_API int8_t z_whatami_to_str(uint8_t whatami, char *buf, size_t len);
/**
 * Constructs a configuration by parsing a file at `path`. Currently supported format is JSON5, a superset of JSON.
 */
ZENOHC_API
z_error_t zc_config_from_file(struct z_owned_config_t *this_,
                              const char *path);
/**
 * Reads a configuration from a JSON-serialized string, such as '{mode:"client",connect:{endpoints:["tcp/127.0.0.1:7447"]}}'.
 *
 * Passing a null-ptr will result in a gravestone value (`z_check(x) == false`).
 */
ZENOHC_API
z_error_t zc_config_from_str(struct z_owned_config_t *this_,
                             const char *s);
/**
 * Gets the property with the given path key from the configuration, returning an owned, null-terminated, JSON serialized string.
 * Use `z_drop` to safely deallocate this string
 */
ZENOHC_API
z_error_t zc_config_get(const struct z_loaned_config_t *config,
                        const char *key,
                        struct z_owned_str_t *value_string);
/**
 * Inserts a JSON-serialized `value` at the `key` position of the configuration.
 *
 * Returns 0 if successful, a negative value otherwise.
 */
ZENOHC_API
z_error_t zc_config_insert_json(struct z_loaned_config_t *config,
                                const char *key,
                                const char *value);
/**
 * Converts `config` into a JSON-serialized string, such as '{"mode":"client","connect":{"endpoints":["tcp/127.0.0.1:7447"]}}'.
 */
ZENOHC_API
z_error_t zc_config_to_string(const struct z_loaned_config_t *config,
                              struct z_owned_str_t *config_string);
/**
 * Initialises the zenoh runtime logger.
 *
 * Note that unless you built zenoh-c with the `logger-autoinit` feature disabled,
 * this will be performed automatically by `z_open` and `z_scout`.
 */
ZENOHC_API void zc_init_logger(void);
ZENOHC_API
void zc_liveliness_declaration_options_default(struct zc_liveliness_declaration_options_t *this_);
/**
 * Declares a subscriber on liveliness tokens that intersect `key`.
 *
 * Parameters:
 *     z_loaned_session_t session: The zenoh session.
 *     z_loaned_keyexpr_t key_expr: The key expression to subscribe.
 *     z_owned_closure_sample_t callback: The callback function that will be called each time a
 *                                        liveliness token status changed.
 *     zc_owned_liveliness_declare_subscriber_options_t _options: The options to be passed to describe the options to be passed to the liveliness subscriber declaration.
 *
 * Returns:
 *    A :c:type:`z_owned_subscriber_t`.
 *
 *    To check if the subscription succeeded and if the subscriber is still valid,
 *    you may use `z_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
ZENOHC_API
z_error_t zc_liveliness_declare_subscriber(struct z_owned_subscriber_t *this_,
                                           const struct z_loaned_session_t *session,
                                           const struct z_loaned_keyexpr_t *key_expr,
                                           struct z_owned_closure_sample_t *callback,
                                           struct zc_liveliness_declare_subscriber_options_t *_options);
/**
 * Constructs and declares a liveliness token on the network.
 *
 * Liveliness token subscribers on an intersecting key expression will receive a PUT sample when connectivity
 * is achieved, and a DELETE sample if it's lost.
 *
 * Passing `NULL` as options is valid and equivalent to a pointer to the default options.
 */
ZENOHC_API
z_error_t zc_liveliness_declare_token(struct zc_owned_liveliness_token_t *this_,
                                      const struct z_loaned_session_t *session,
                                      const struct z_loaned_keyexpr_t *key_expr,
                                      struct zc_liveliness_declaration_options_t *_options);
/**
 * Queries liveliness tokens currently on the network with a key expression intersecting with `key`.
 *
 * Note that the same "value stealing" tricks apply as with a normal :c:func:`z_get`
 *
 * Passing `NULL` as options is valid and equivalent to passing a pointer to the default options.
 */
ZENOHC_API
z_error_t zc_liveliness_get(const struct z_loaned_session_t *session,
                            const struct z_loaned_keyexpr_t *key_expr,
                            struct z_owned_closure_reply_t *callback,
                            struct zc_liveliness_get_options_t *options);
/**
 * The gravestone value for `zc_liveliness_get_options_t`
 */
ZENOHC_API void zc_liveliness_get_options_default(struct zc_liveliness_get_options_t *this_);
ZENOHC_API
void zc_liveliness_subscriber_options_default(struct zc_liveliness_declare_subscriber_options_t *this_);
/**
 * Returns `true` unless the token is at its gravestone value.
 */
ZENOHC_API bool zc_liveliness_token_check(const struct zc_owned_liveliness_token_t *this_);
ZENOHC_API void zc_liveliness_token_drop(struct zc_owned_liveliness_token_t *this_);
/**
 * The gravestone value for liveliness tokens.
 */
ZENOHC_API void zc_liveliness_token_null(struct zc_owned_liveliness_token_t *this_);
/**
 * Destroys a liveliness token, notifying subscribers of its destruction.
 */
ZENOHC_API z_error_t zc_liveliness_undeclare_token(struct zc_owned_liveliness_token_t *this_);
/**
 * Creates a new blocking fifo channel, returned as a pair of closures.
 *
 * If `bound` is different from 0, that channel will be bound and apply back-pressure when full.
 *
 * The `send` end should be passed as callback to a `z_get` call.
 *
 * The `recv` end is a synchronous closure that will block until either a `z_owned_query_t` is available,
 * which it will then return; or until the `send` closure is dropped and all queries have been consumed,
 * at which point it will return an invalidated `z_owned_query_t`, and so will further calls.
 */
ZENOHC_API
void zc_query_fifo_new(struct z_owned_query_channel_t *this_,
                       size_t bound);
/**
 * Creates a new non-blocking fifo channel, returned as a pair of closures.
 *
 * If `bound` is different from 0, that channel will be bound and apply back-pressure when full.
 *
 * The `send` end should be passed as callback to a `z_get` call.
 *
 * The `recv` end is a synchronous closure that will block until either a `z_owned_query_t` is available,
 * which it will then return; or until the `send` closure is dropped and all queries have been consumed,
 * at which point it will return an invalidated `z_owned_query_t`, and so will further calls.
 */
ZENOHC_API
void zc_query_non_blocking_fifo_new(struct z_owned_query_channel_t *this_,
                                    size_t bound);
/**
 * Creates a new blocking fifo channel, returned as a pair of closures.
 *
 * If `bound` is different from 0, that channel will be bound and apply back-pressure when full.
 *
 * The `send` end should be passed as callback to a `z_get` call.
 *
 * The `recv` end is a synchronous closure that will block until either a `z_owned_reply_t` is available,
 * which it will then return; or until the `send` closure is dropped and all replies have been consumed,
 * at which point it will return an invalidated `z_owned_reply_t`, and so will further calls.
 */
ZENOHC_API
void zc_reply_fifo_new(struct z_owned_reply_channel_t *this_,
                       size_t bound);
/**
 * Creates a new non-blocking fifo channel, returned as a pair of closures.
 *
 * If `bound` is different from 0, that channel will be bound and apply back-pressure when full.
 *
 * The `send` end should be passed as callback to a `z_get` call.
 *
 * The `recv` end is a synchronous closure that will block until either a `z_owned_reply_t` is available,
 * which it will then return; or until the `send` closure is dropped and all replies have been consumed,
 * at which point it will return an invalidated `z_owned_reply_t`, and so will further calls.
 */
ZENOHC_API
void zc_reply_non_blocking_fifo_new(struct z_owned_reply_channel_t *this_,
                                    size_t bound);
/**
 * Increments the session's reference count, returning a new owning handle.
 */
ZENOHC_API
z_error_t zc_session_clone(struct z_owned_session_t *dst,
                           const struct z_owned_session_t *src);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void zcu_closure_matching_status_call(const struct zcu_owned_closure_matching_status_t *closure,
                                      const struct zcu_matching_status_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API
void zcu_closure_matching_status_drop(struct zcu_owned_closure_matching_status_t *closure);
/**
 * Constructs a null safe-to-drop value of 'zcu_owned_closure_matching_status_t' type
 */
ZENOHC_API void zcu_closure_matching_status_null(struct zcu_owned_closure_matching_status_t *this_);
ZENOHC_API enum zcu_locality_t zcu_locality_default(void);
/**
 * Register callback for notifying subscribers matching.
 */
ZENOHC_API
z_error_t zcu_publisher_matching_listener_callback(struct zcu_owned_matching_listener_t *this_,
                                                   const struct z_loaned_publisher_t *publisher,
                                                   struct zcu_owned_closure_matching_status_t *callback);
ZENOHC_API enum zcu_reply_keyexpr_t zcu_reply_keyexpr_default(void);
/**
 * Declares a Publication Cache.
 *
 * Parameters:
 *     z_loaned_session_t session: The zenoh session.
 *     z_loaned_keyexpr_t key_expr: The key expression to publish.
 *     ze_publication_cache_options_t options: Additional options for the publication_cache.
 *
 * Returns:
 *    :c:type:`ze_owned_publication_cache_t`.
 *
 *
 * Example:
 *    Declaring a publication cache `NULL` for the options:
 *
 *    .. code-block:: C
 *
 *       ze_owned_publication_cache_t pub_cache = ze_declare_publication_cache(z_loan(s), z_keyexpr(expr), NULL);
 *
 *    is equivalent to initializing and passing the default publication cache options:
 *
 *    .. code-block:: C
 *
 *       ze_publication_cache_options_t opts = ze_publication_cache_options_default();
 *       ze_owned_publication_cache_t pub_cache = ze_declare_publication_cache(z_loan(s), z_keyexpr(expr), &opts);
 */
ZENOHC_API
z_error_t ze_declare_publication_cache(struct ze_owned_publication_cache_t *this_,
                                       const struct z_loaned_session_t *session,
                                       const struct z_loaned_keyexpr_t *key_expr,
                                       struct ze_publication_cache_options_t *options);
/**
 * Declares a Querying Subscriber for a given key expression.
 *
 * Parameters:
 *     z_loaned_session_t session: The zenoh session.
 *     z_loaned_keyexpr_t keyexpr: The key expression to subscribe.
 *     z_owned_closure_sample_t callback: The callback function that will be called each time a data matching the subscribed expression is received.
 *     ze_querying_subscriber_options_t options: Additional options for the querying subscriber.
 *
 * Returns:
 *    :c:type:`ze_owned_subscriber_t`.
 *
 *    To check if the subscription succeeded and if the querying subscriber is still valid,
 *    you may use `ze_querying_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 *
 *    Like all `ze_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 *    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 *    After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * Example:
 *    Declaring a subscriber passing ``NULL`` for the options:
 *
 *    .. code-block:: C
 *
 *       ze_owned_subscriber_t sub = ze_declare_querying_subscriber(z_loan(s), z_keyexpr(expr), callback, NULL);
 *
 *    is equivalent to initializing and passing the default subscriber options:
 *
 *    .. code-block:: C
 *
 *       z_subscriber_options_t opts = z_subscriber_options_default();
 *       ze_owned_subscriber_t sub = ze_declare_querying_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
 */
ZENOHC_API
z_error_t ze_declare_querying_subscriber(struct ze_owned_querying_subscriber_t *this_,
                                         const struct z_loaned_session_t *session,
                                         const struct z_loaned_keyexpr_t *key_expr,
                                         struct z_owned_closure_sample_t *callback,
                                         struct ze_querying_subscriber_options_t *options);
/**
 * Returns ``true`` if `pub_cache` is valid.
 */
ZENOHC_API bool ze_publication_cache_check(const struct ze_owned_publication_cache_t *this_);
/**
 * Constructs a null safe-to-drop value of 'ze_owned_publication_cache_t' type
 */
ZENOHC_API void ze_publication_cache_null(struct ze_owned_publication_cache_t *this_);
/**
 * Constructs the default value for :c:type:`ze_publication_cache_options_t`.
 */
ZENOHC_API void ze_publication_cache_options_default(struct ze_publication_cache_options_t *this_);
/**
 * Returns ``true`` if `this` is valid.
 */
ZENOHC_API bool ze_querying_subscriber_check(const struct ze_owned_querying_subscriber_t *this_);
/**
 * Make a :c:type:`ze_owned_querying_subscriber_t` to perform an additional query on a specified selector.
 * The queried samples will be merged with the received publications and made available in the subscriber callback.
 */
ZENOHC_API
z_error_t ze_querying_subscriber_get(const struct ze_loaned_querying_subscriber_t *sub,
                                     const struct z_loaned_keyexpr_t *selector,
                                     const struct z_get_options_t *options);
/**
 * Returns a :c:type:`ze_querying_subscriber_loan` loaned from `this`.
 */
ZENOHC_API
const struct ze_loaned_querying_subscriber_t *ze_querying_subscriber_loan(const struct ze_owned_querying_subscriber_t *this_);
/**
 * Constructs a null safe-to-drop value of 'ze_owned_querying_subscriber_t' type
 */
ZENOHC_API void ze_querying_subscriber_null(struct ze_owned_querying_subscriber_t *this_);
/**
 * Constructs the default value for :c:type:`ze_querying_subscriber_options_t`.
 */
ZENOHC_API
void ze_querying_subscriber_options_default(struct ze_querying_subscriber_options_t *this_);
/**
 * Closes the given :c:type:`ze_owned_publication_cache_t`, droping it and invalidating it for double-drop safety.
 */
ZENOHC_API
z_error_t ze_undeclare_publication_cache(struct ze_owned_publication_cache_t *this_);
/**
 * Undeclares the given :c:type:`ze_owned_querying_subscriber_t`, droping it and invalidating it for double-drop safety.
 */
ZENOHC_API
z_error_t ze_undeclare_querying_subscriber(struct ze_owned_querying_subscriber_t *this_);
