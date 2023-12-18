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
 * A :c:type:`z_encoding_t` integer `prefix`.
 *
 *     - **Z_ENCODING_PREFIX_EMPTY**
 *     - **Z_ENCODING_PREFIX_APP_OCTET_STREAM**
 *     - **Z_ENCODING_PREFIX_APP_CUSTOM**
 *     - **Z_ENCODING_PREFIX_TEXT_PLAIN**
 *     - **Z_ENCODING_PREFIX_APP_PROPERTIES**
 *     - **Z_ENCODING_PREFIX_APP_JSON**
 *     - **Z_ENCODING_PREFIX_APP_SQL**
 *     - **Z_ENCODING_PREFIX_APP_INTEGER**
 *     - **Z_ENCODING_PREFIX_APP_FLOAT**
 *     - **Z_ENCODING_PREFIX_APP_XML**
 *     - **Z_ENCODING_PREFIX_APP_XHTML_XML**
 *     - **Z_ENCODING_PREFIX_APP_X_WWW_FORM_URLENCODED**
 *     - **Z_ENCODING_PREFIX_TEXT_JSON**
 *     - **Z_ENCODING_PREFIX_TEXT_HTML**
 *     - **Z_ENCODING_PREFIX_TEXT_XML**
 *     - **Z_ENCODING_PREFIX_TEXT_CSS**
 *     - **Z_ENCODING_PREFIX_TEXT_CSV**
 *     - **Z_ENCODING_PREFIX_TEXT_JAVASCRIPT**
 *     - **Z_ENCODING_PREFIX_IMAGE_JPEG**
 *     - **Z_ENCODING_PREFIX_IMAGE_PNG**
 *     - **Z_ENCODING_PREFIX_IMAGE_GIF**
 */
typedef enum z_encoding_prefix_t {
  Z_ENCODING_PREFIX_EMPTY = 0,
  Z_ENCODING_PREFIX_APP_OCTET_STREAM = 1,
  Z_ENCODING_PREFIX_APP_CUSTOM = 2,
  Z_ENCODING_PREFIX_TEXT_PLAIN = 3,
  Z_ENCODING_PREFIX_APP_PROPERTIES = 4,
  Z_ENCODING_PREFIX_APP_JSON = 5,
  Z_ENCODING_PREFIX_APP_SQL = 6,
  Z_ENCODING_PREFIX_APP_INTEGER = 7,
  Z_ENCODING_PREFIX_APP_FLOAT = 8,
  Z_ENCODING_PREFIX_APP_XML = 9,
  Z_ENCODING_PREFIX_APP_XHTML_XML = 10,
  Z_ENCODING_PREFIX_APP_X_WWW_FORM_URLENCODED = 11,
  Z_ENCODING_PREFIX_TEXT_JSON = 12,
  Z_ENCODING_PREFIX_TEXT_HTML = 13,
  Z_ENCODING_PREFIX_TEXT_XML = 14,
  Z_ENCODING_PREFIX_TEXT_CSS = 15,
  Z_ENCODING_PREFIX_TEXT_CSV = 16,
  Z_ENCODING_PREFIX_TEXT_JAVASCRIPT = 17,
  Z_ENCODING_PREFIX_IMAGE_JPEG = 18,
  Z_ENCODING_PREFIX_IMAGE_PNG = 19,
  Z_ENCODING_PREFIX_IMAGE_GIF = 20,
} z_encoding_prefix_t;
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
 * The Queryables that should be target of a :c:func:`z_get`.
 *
 *     - **BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
 *     - **ALL_COMPLETE**: All complete queryables.
 *     - **ALL**: All matching queryables.
 */
typedef enum z_query_target_t {
  Z_QUERY_TARGET_BEST_MATCHING,
  Z_QUERY_TARGET_ALL,
  Z_QUERY_TARGET_ALL_COMPLETE,
} z_query_target_t;
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
 * A contiguous view of bytes owned by some other entity.
 *
 * `start` being `null` is considered a gravestone value,
 * and empty slices are represented using a possibly dangling pointer for `start`.
 */
typedef struct z_bytes_t {
  size_t len;
  const uint8_t *start;
} z_bytes_t;
/**
 * The body of a loop over an attachment's key-value pairs.
 *
 * `key` and `value` are loaned to the body for the duration of a single call.
 * `context` is passed transparently through the iteration driver.
 *
 * Returning `0` is treated as `continue`.
 * Returning any other value is treated as `break`.
 */
typedef int8_t (*z_attachment_iter_body_t)(struct z_bytes_t key,
                                           struct z_bytes_t value,
                                           void *context);
/**
 * The driver of a loop over an attachment's key-value pairs.
 *
 * This function is expected to call `loop_body` once for each key-value pair
 * within `iterator`, passing `context`, and returning any non-zero value immediately (breaking iteration).
 */
typedef int8_t (*z_attachment_iter_driver_t)(const void *iterator,
                                             z_attachment_iter_body_t loop_body,
                                             void *context);
/**
 * A iteration based map of byte slice to byte slice.
 *
 * `iteration_driver == NULL` marks the gravestone value, as this type is often optional.
 * Users are encouraged to use `z_attachment_null` and `z_attachment_check` to interact.
 */
typedef struct z_attachment_t {
  const void *data;
  z_attachment_iter_driver_t iteration_driver;
} z_attachment_t;
/**
 * A map of maybe-owned vector of bytes to owned vector of bytes.
 *
 * In Zenoh C, this map is backed by Rust's standard HashMap, with a DoS-resistant hasher
 */
typedef struct z_owned_bytes_map_t {
  uint64_t _0[2];
  size_t _1[4];
} z_owned_bytes_map_t;
/**
 * Represents a Zenoh ID.
 *
 * In general, valid Zenoh IDs are LSB-first 128bit unsigned and non-zero integers.
 */
typedef struct z_id_t {
  uint8_t id[16];
} z_id_t;
/**
 * An owned array of owned, zenoh allocated, NULL terminated strings.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_str_array_t {
  char **val;
  size_t len;
} z_owned_str_array_t;
/**
 * A zenoh-allocated hello message returned by a zenoh entity to a scout message sent with `z_scout`.
 *
 * Members:
 *   unsigned int whatami: The kind of zenoh entity.
 *   z_owned_bytes_t pid: The peer id of the scouted entity (empty if absent).
 *   z_owned_str_array_t locators: The locators of the scouted entity.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct z_owned_hello_t {
  unsigned int _whatami;
  struct z_id_t _pid;
  struct z_owned_str_array_t _locators;
} z_owned_hello_t;
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
typedef struct z_owned_closure_hello_t {
  void *context;
  void (*call)(struct z_owned_hello_t*, void*);
  void (*drop)(void*);
} z_owned_closure_hello_t;
/**
 * Owned variant of a Query received by a Queryable.
 *
 * You may construct it by `z_query_clone`-ing a loaned query.
 * When the last `z_owned_query_t` corresponding to a query is destroyed, or the callback that produced the query cloned to build them returns,
 * the query will receive its termination signal.
 *
 * Holding onto an `z_owned_query_t` for too long (10s by default, can be set in `z_get`'s options) will trigger a timeout error
 * to be sent to the querier by the infrastructure, and new responses to the outdated query will be silently dropped.
 */
typedef struct z_owned_query_t {
  void *_0;
} z_owned_query_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 *
 * Members:
 *   void *context: a pointer to an arbitrary state.
 *   void *call(const struct z_query_t*, const void *context): the typical callback function. `context` will be passed as its last argument.
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
 *   void *call(const struct z_query_t*, const void *context): the typical callback function. `context` will be passed as its last argument.
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
  void (*call)(const struct z_query_t*, void *context);
  void (*drop)(void*);
} z_owned_closure_query_t;
/**
 * An owned reply to a :c:func:`z_get`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
#if defined(TARGET_ARCH_X86_64)
typedef struct ALIGN(8) z_owned_reply_t {
  uint64_t _0[28];
} z_owned_reply_t;
#endif
#if defined(TARGET_ARCH_AARCH64)
typedef struct ALIGN(16) z_owned_reply_t {
  uint64_t _0[30];
} z_owned_reply_t;
#endif
#if defined(TARGET_ARCH_ARM)
typedef struct ALIGN(8) z_owned_reply_t {
  uint64_t _0[19];
} z_owned_reply_t;
#endif
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
  void (*call)(struct z_owned_reply_t*, void*);
  void (*drop)(void*);
} z_owned_closure_reply_t;
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
#if !defined(TARGET_ARCH_ARM)
typedef struct ALIGN(8) z_keyexpr_t {
  uint64_t _0[4];
} z_keyexpr_t;
#endif
#if defined(TARGET_ARCH_ARM)
typedef struct ALIGN(4) z_keyexpr_t {
  uint32_t _0[5];
} z_keyexpr_t;
#endif
/**
 * The encoding of a payload, in a MIME-like format.
 *
 * For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
 *
 * Members:
 *   z_encoding_prefix_t prefix: The integer prefix of this encoding.
 *   z_bytes_t suffix: The suffix of this encoding. `suffix` MUST be a valid UTF-8 string.
 */
typedef struct z_encoding_t {
  enum z_encoding_prefix_t prefix;
  struct z_bytes_t suffix;
} z_encoding_t;
typedef struct z_timestamp_t {
  uint64_t time;
  struct z_id_t id;
} z_timestamp_t;
/**
 * A data sample.
 *
 * A sample is the value associated to a given resource at a given point in time.
 *
 * Members:
 *   z_keyexpr_t keyexpr: The resource key of this data sample.
 *   z_bytes_t payload: The value of this data sample.
 *   z_encoding_t encoding: The encoding of the value of this data sample.
 *   z_sample_kind_t kind: The kind of this data sample (PUT or DELETE).
 *   z_timestamp_t timestamp: The timestamp of this data sample.
 *   z_attachment_t attachment: The attachment of this data sample.
 */
typedef struct z_sample_t {
  struct z_keyexpr_t keyexpr;
  struct z_bytes_t payload;
  struct z_encoding_t encoding;
  const void *_zc_buf;
  enum z_sample_kind_t kind;
  struct z_timestamp_t timestamp;
  struct z_attachment_t attachment;
} z_sample_t;
/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
 *
 * Members:
 *   void *context: a pointer to an arbitrary state.
 *   void *call(const struct z_sample_t*, const void *context): the typical callback function. `context` will be passed as its last argument.
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
  void (*call)(const struct z_sample_t*, void *context);
  void (*drop)(void*);
} z_owned_closure_sample_t;
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
typedef struct z_owned_config_t {
  void *_0;
} z_owned_config_t;
/**
 * A loaned zenoh configuration.
 */
typedef struct z_config_t {
  const struct z_owned_config_t *_0;
} z_config_t;
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
#if !defined(TARGET_ARCH_ARM)
typedef struct ALIGN(8) z_owned_keyexpr_t {
  uint64_t _0[4];
} z_owned_keyexpr_t;
#endif
#if defined(TARGET_ARCH_ARM)
typedef struct ALIGN(4) z_owned_keyexpr_t {
  uint32_t _0[5];
} z_owned_keyexpr_t;
#endif
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
#if !defined(TARGET_ARCH_ARM)
typedef struct ALIGN(8) z_owned_publisher_t {
  uint64_t _0[7];
} z_owned_publisher_t;
#endif
#if defined(TARGET_ARCH_ARM)
typedef struct ALIGN(4) z_owned_publisher_t {
  uint32_t _0[8];
} z_owned_publisher_t;
#endif
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
 * Represents the set of options that can be applied to a pull subscriber,
 * upon its declaration via :c:func:`z_declare_pull_subscriber`.
 *
 * Members:
 *   z_reliability_t reliability: The subscription reliability.
 */
typedef struct z_pull_subscriber_options_t {
  enum z_reliability_t reliability;
} z_pull_subscriber_options_t;
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
/**
 * An owned payload encoding.
 *
 * Members:
 *   z_encoding_prefix_t prefix: The integer prefix of this encoding.
 *   z_bytes_t suffix: The suffix of this encoding. `suffix` MUST be a valid UTF-8 string.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct z_owned_encoding_t {
  enum z_encoding_prefix_t prefix;
  struct z_bytes_t suffix;
  bool _dropped;
} z_owned_encoding_t;
/**
 * The replies consolidation strategy to apply on replies to a :c:func:`z_get`.
 */
typedef struct z_query_consolidation_t {
  enum z_consolidation_mode_t mode;
} z_query_consolidation_t;
/**
 * A zenoh value.
 *
 * Members:
 *   z_bytes_t payload: The payload of this zenoh value.
 *   z_encoding_t encoding: The encoding of this zenoh value `payload`.
 */
typedef struct z_value_t {
  struct z_bytes_t payload;
  struct z_encoding_t encoding;
} z_value_t;
/**
 * Options passed to the :c:func:`z_get` function.
 *
 * Members:
 *     z_query_target_t target: The Queryables that should be target of the query.
 *     z_query_consolidation_t consolidation: The replies consolidation strategy to apply on replies to the query.
 *     z_value_t value: An optional value to attach to the query.
 *     z_attachment_t attachment: The attachment to attach to the query.
 *     uint64_t timeout: The timeout for the query in milliseconds. 0 means default query timeout from zenoh configuration.
 */
typedef struct z_get_options_t {
  enum z_query_target_t target;
  struct z_query_consolidation_t consolidation;
  struct z_value_t value;
  struct z_attachment_t attachment;
  uint64_t timeout_ms;
} z_get_options_t;
/**
 * An borrowed array of borrowed, zenoh allocated, NULL terminated strings.
 */
typedef struct z_str_array_t {
  size_t len;
  const char *const *val;
} z_str_array_t;
/**
 * A reference-type hello message returned by a zenoh entity to a scout message sent with `z_scout`.
 *
 * Members:
 *   unsigned int whatami: The kind of zenoh entity.
 *   z_owned_bytes_t pid: The peer id of the scouted entity (empty if absent).
 *   z_owned_str_array_t locators: The locators of the scouted entity.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct z_hello_t {
  unsigned int whatami;
  struct z_id_t pid;
  struct z_str_array_t locators;
} z_hello_t;
/**
 * The wrapper type for null-terminated string values allocated by zenoh. The instances of `z_owned_str_t`
 * should be released with `z_drop` macro or with `z_str_drop` function and checked to validity with
 * `z_check` and `z_str_check` correspondently
 */
typedef struct z_owned_str_t {
  char *_cstr;
} z_owned_str_t;
/**
 * A loaned zenoh publisher.
 */
typedef struct z_publisher_t {
  const struct z_owned_publisher_t *_0;
} z_publisher_t;
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
 *     z_encoding_t encoding: The encoding of the payload.
 *     z_attachment_t attachment: The attachment to attach to the publication.
 */
typedef struct z_publisher_put_options_t {
  struct z_encoding_t encoding;
  struct z_attachment_t attachment;
} z_publisher_put_options_t;
typedef struct z_pull_subscriber_t {
  const struct z_owned_pull_subscriber_t *_0;
} z_pull_subscriber_t;
/**
 * Options passed to the :c:func:`z_put` function.
 *
 * Members:
 *     z_encoding_t encoding: The encoding of the payload.
 *     z_congestion_control_t congestion_control: The congestion control to apply when routing this message.
 *     z_priority_t priority: The priority of this message.
 *     z_attachment_t attachment: The attachment to this message.
 */
typedef struct z_put_options_t {
  struct z_encoding_t encoding;
  enum z_congestion_control_t congestion_control;
  enum z_priority_t priority;
  struct z_attachment_t attachment;
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
  struct z_owned_closure_owned_query_t send;
  struct z_owned_query_channel_closure_t recv;
} z_owned_query_channel_t;
/**
 * Represents the set of options that can be applied to a query reply,
 * sent via :c:func:`z_query_reply`.
 *
 * Members:
 *   z_encoding_t encoding: The encoding of the payload.
 *   z_attachment_t attachment: The attachment to this reply.
 */
typedef struct z_query_reply_options_t {
  struct z_encoding_t encoding;
  struct z_attachment_t attachment;
} z_query_reply_options_t;
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
typedef struct z_owned_scouting_config_t {
  struct z_owned_config_t _config;
  unsigned long zc_timeout_ms;
  uint8_t zc_what;
} z_owned_scouting_config_t;
/**
 * A loaned zenoh subscriber.
 */
typedef struct z_subscriber_t {
  const struct z_owned_subscriber_t *_0;
} z_subscriber_t;
/**
 * The options for `zc_liveliness_declare_token`
 */
typedef struct zc_owned_liveliness_declaration_options_t {
  uint8_t _inner;
} zc_owned_liveliness_declaration_options_t;
/**
 * The options for :c:func:`zc_liveliness_declare_subscriber`
 */
typedef struct zc_owned_liveliness_declare_subscriber_options_t {
  uint8_t _inner;
} zc_owned_liveliness_declare_subscriber_options_t;
/**
 * A liveliness token that can be used to provide the network with information about connectivity to its
 * declarer: when constructed, a PUT sample will be received by liveliness subscribers on intersecting key
 * expressions.
 *
 * A DELETE on the token's key expression will be received by subscribers if the token is destroyed, or if connectivity between the subscriber and the token's creator is lost.
 */
typedef struct zc_owned_liveliness_token_t {
  size_t _inner[4];
} zc_owned_liveliness_token_t;
/**
 * The options for :c:func:`zc_liveliness_declare_subscriber`
 */
typedef struct zc_owned_liveliness_get_options_t {
  uint32_t timeout_ms;
} zc_owned_liveliness_get_options_t;
/**
 * An owned payload, backed by a reference counted owner.
 *
 * The `payload` field may be modified, and Zenoh will take the new values into account,
 * however, assuming `ostart` and `olen` are the respective values of `payload.start` and
 * `payload.len` when constructing the `zc_owned_payload_t payload` value was created,
 * then `payload.start` MUST remain within the `[ostart, ostart + olen[` interval, and
 * `payload.len` must remain within `[0, olen -(payload.start - ostart)]`.
 *
 * Should this invariant be broken when the payload is passed to one of zenoh's `put_owned`
 * functions, then the operation will fail (but the passed value will still be consumed).
 */
typedef struct zc_owned_payload_t {
  struct z_bytes_t payload;
  size_t _owner[5];
} zc_owned_payload_t;
typedef struct zc_owned_shmbuf_t {
  size_t _0[9];
} zc_owned_shmbuf_t;
typedef struct zc_owned_shm_manager_t {
  size_t _0;
} zc_owned_shm_manager_t;
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
typedef struct ze_owned_publication_cache_t {
  size_t _0[1];
} ze_owned_publication_cache_t;
/**
 * Options passed to the :c:func:`ze_declare_publication_cache` function.
 *
 * Members:
 *     z_keyexpr_t queryable_prefix: The prefix used for queryable
 *     zcu_locality_t queryable_origin: The restriction for the matching queries that will be receive by this
 *                       publication cache
 *     size_t history: The the history size
 *     size_t resources_limit: The limit number of cached resources
 */
typedef struct ze_publication_cache_options_t {
  struct z_keyexpr_t queryable_prefix;
  enum zcu_locality_t queryable_origin;
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
typedef struct ze_owned_querying_subscriber_t {
  size_t _0[1];
} ze_owned_querying_subscriber_t;
/**
 * Represents the set of options that can be applied to a querying subscriber,
 * upon its declaration via :c:func:`ze_declare_querying_subscriber`.
 *
 * Members:
 *   z_reliability_t reliability: The subscription reliability.
 *   zcu_locality_t allowed_origin: The restriction for the matching publications that will be
 *                                  receive by this subscriber.
 *   z_keyexpr_t query_selector: The selector to be used for queries.
 *   z_query_target_t query_target: The target to be used for queries.
 *   z_query_consolidation_t query_consolidation: The consolidation mode to be used for queries.
 *   zcu_reply_keyexpr_t query_accept_replies: The accepted replies for queries.
 *   uint64_t query_timeout_ms: The timeout to be used for queries.
 */
typedef struct ze_querying_subscriber_options_t {
  enum z_reliability_t reliability;
  enum zcu_locality_t allowed_origin;
  struct z_keyexpr_t query_selector;
  enum z_query_target_t query_target;
  struct z_query_consolidation_t query_consolidation;
  enum zcu_reply_keyexpr_t query_accept_replies;
  uint64_t query_timeout_ms;
} ze_querying_subscriber_options_t;
typedef struct ze_querying_subscriber_t {
  const struct ze_owned_querying_subscriber_t *_0;
} ze_querying_subscriber_t;
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
 * Returns the gravestone value for `z_attachment_t`.
 */
ZENOHC_API bool z_attachment_check(const struct z_attachment_t *this_);
/**
 * Returns the value associated with the key.
 */
ZENOHC_API struct z_bytes_t z_attachment_get(struct z_attachment_t this_, struct z_bytes_t key);
/**
 * Iterate over `this`'s key-value pairs, breaking if `body` returns a non-zero
 * value for a key-value pair, and returning the latest return value.
 *
 * `context` is passed to `body` to allow stateful closures.
 *
 * This function takes no ownership whatsoever.
 */
ZENOHC_API
int8_t z_attachment_iterate(struct z_attachment_t this_,
                            z_attachment_iter_body_t body,
                            void *context);
/**
 * Returns the gravestone value for `z_attachment_t`.
 */
ZENOHC_API struct z_attachment_t z_attachment_null(void);
/**
 * Returns ``true`` if `b` is initialized.
 */
ZENOHC_API bool z_bytes_check(const struct z_bytes_t *b);
/**
 * Aliases `this` into a generic `z_attachment_t`, allowing it to be passed to corresponding APIs.
 */
ZENOHC_API struct z_attachment_t z_bytes_map_as_attachment(const struct z_owned_bytes_map_t *this_);
/**
 * Returns `true` if the map is not in its gravestone state
 */
ZENOHC_API bool z_bytes_map_check(const struct z_owned_bytes_map_t *this_);
/**
 * Destroys the map, resetting `this` to its gravestone value.
 *
 * This function is double-free safe, passing a pointer to the gravestone value will have no effect.
 */
ZENOHC_API void z_bytes_map_drop(struct z_owned_bytes_map_t *this_);
/**
 * Constructs a map from the provided attachment, copying keys and values.
 *
 * If `this` is at gravestone value, the returned value will also be at gravestone value.
 */
ZENOHC_API struct z_owned_bytes_map_t z_bytes_map_from_attachment(struct z_attachment_t this_);
/**
 * Constructs a map from the provided attachment, aliasing the attachment's keys and values.
 *
 * If `this` is at gravestone value, the returned value will also be at gravestone value.
 */
ZENOHC_API
struct z_owned_bytes_map_t z_bytes_map_from_attachment_aliasing(struct z_attachment_t this_);
/**
 * Returns the value associated with `key`, returning a gravestone value if:
 * - `this` or `key` is in gravestone state.
 * - `this` has no value associated to `key`
 */
ZENOHC_API
struct z_bytes_t z_bytes_map_get(const struct z_owned_bytes_map_t *this_,
                                 struct z_bytes_t key);
/**
 * Associates `value` to `key` in the map, aliasing them.
 *
 * Note that once `key` is aliased, reinserting at the same key may alias the previous instance, or the new instance of `key`.
 *
 * Calling this with `NULL` or the gravestone value is undefined behaviour.
 */
ZENOHC_API
void z_bytes_map_insert_by_alias(const struct z_owned_bytes_map_t *this_,
                                 struct z_bytes_t key,
                                 struct z_bytes_t value);
/**
 * Associates `value` to `key` in the map, copying them to obtain ownership: `key` and `value` are not aliased past the function's return.
 *
 * Calling this with `NULL` or the gravestone value is undefined behaviour.
 */
ZENOHC_API
void z_bytes_map_insert_by_copy(const struct z_owned_bytes_map_t *this_,
                                struct z_bytes_t key,
                                struct z_bytes_t value);
/**
 * Iterates over the key-value pairs in the map.
 *
 * `body` will be called once per pair, with `ctx` as its last argument.
 * If `body` returns a non-zero value, the iteration will stop immediately and the value will be returned.
 * Otherwise, this will return 0 once all pairs have been visited.
 * `body` is not given ownership of the key nor value, which alias the pairs in the map.
 * It is safe to keep these aliases until existing keys are modified/removed, or the map is destroyed.
 * Note that this map is unordered.
 *
 * Calling this with `NULL` or the gravestone value is undefined behaviour.
 */
ZENOHC_API
int8_t z_bytes_map_iter(const struct z_owned_bytes_map_t *this_,
                        z_attachment_iter_body_t body,
                        void *ctx);
/**
 * Constructs a new map.
 */
ZENOHC_API struct z_owned_bytes_map_t z_bytes_map_new(void);
/**
 * Constructs the gravestone value for `z_owned_bytes_map_t`
 */
ZENOHC_API struct z_owned_bytes_map_t z_bytes_map_null(void);
/**
 * Returns a view of `str` using `strlen`.
 *
 * `str == NULL` will cause this to return `z_bytes_null()`
 */
ZENOHC_API struct z_bytes_t z_bytes_new(const char *str);
/**
 * Returns the gravestone value for `z_bytes_t`
 */
ZENOHC_API struct z_bytes_t z_bytes_null(void);
/**
 * Closes a zenoh session. This drops and invalidates `session` for double-drop safety.
 *
 * Returns a negative value if an error occured while closing the session.
 * Returns the remaining reference count of the session otherwise, saturating at i8::MAX.
 */
ZENOHC_API int8_t z_close(struct z_owned_session_t *session);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_hello_call(const struct z_owned_closure_hello_t *closure,
                          struct z_owned_hello_t *hello);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_hello_drop(struct z_owned_closure_hello_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_closure_hello_t' type
 */
ZENOHC_API struct z_owned_closure_hello_t z_closure_hello_null(void);
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
                          const struct z_query_t *query);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_query_drop(struct z_owned_closure_query_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_closure_query_t' type
 */
ZENOHC_API struct z_owned_closure_query_t z_closure_query_null(void);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_reply_call(const struct z_owned_closure_reply_t *closure,
                          struct z_owned_reply_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_reply_drop(struct z_owned_closure_reply_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_closure_reply_t' type
 */
ZENOHC_API struct z_owned_closure_reply_t z_closure_reply_null(void);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_sample_call(const struct z_owned_closure_sample_t *closure,
                           const struct z_sample_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_sample_drop(struct z_owned_closure_sample_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_closure_sample_t' type
 */
ZENOHC_API struct z_owned_closure_sample_t z_closure_sample_null(void);
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
ZENOHC_API struct z_owned_closure_zid_t z_closure_zid_null(void);
/**
 * Returns ``true`` if `config` is valid.
 */
ZENOHC_API bool z_config_check(const struct z_owned_config_t *config);
/**
 * Constructs a default, zenoh-allocated, client mode configuration.
 * If `peer` is not null, it is added to the configuration as remote peer.
 */
ZENOHC_API struct z_owned_config_t z_config_client(const char *const *peers, size_t n_peers);
/**
 * Creates a default, zenoh-allocated, configuration.
 */
ZENOHC_API struct z_owned_config_t z_config_default(void);
/**
 * Frees `config`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_config_drop(struct z_owned_config_t *config);
/**
 * Returns a :c:type:`z_config_t` loaned from `s`.
 */
ZENOHC_API struct z_config_t z_config_loan(const struct z_owned_config_t *s);
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
struct z_owned_config_t z_config_new(void);
/**
 * Constructs a null safe-to-drop value of 'z_owned_config_t' type
 */
ZENOHC_API struct z_owned_config_t z_config_null(void);
/**
 * Constructs a default, zenoh-allocated, peer mode configuration.
 */
ZENOHC_API struct z_owned_config_t z_config_peer(void);
/**
 * Declare a key expression. The id is returned as a :c:type:`z_keyexpr_t` with a nullptr suffix.
 *
 * This numerical id will be used on the network to save bandwidth and
 * ease the retrieval of the concerned resource in the routing tables.
 */
ZENOHC_API
struct z_owned_keyexpr_t z_declare_keyexpr(struct z_session_t session,
                                           struct z_keyexpr_t keyexpr);
/**
 * Declares a publisher for the given key expression.
 *
 * Data can be put and deleted with this publisher with the help of the
 * :c:func:`z_publisher_put` and :c:func:`z_publisher_delete` functions.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression to publish.
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
struct z_owned_publisher_t z_declare_publisher(struct z_session_t session,
                                               struct z_keyexpr_t keyexpr,
                                               const struct z_publisher_options_t *options);
/**
 * Declares a pull subscriber for a given key expression.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression to subscribe.
 *     callback: The callback function that will be called each time a data matching the subscribed expression is received.
 *     opts: additional options for the pull subscriber.
 *
 * Returns:
 *    A :c:type:`z_owned_subscriber_t`.
 *
 *    To check if the subscription succeeded and if the pull subscriber is still valid,
 *    you may use `z_pull_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 *
 *    Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 *    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 *    After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * Example:
 *    Declaring a subscriber passing ``NULL`` for the options:
 *
 *    .. code-block:: C
 *
 *       z_owned_subscriber_t sub = z_declare_pull_subscriber(z_loan(s), z_keyexpr(expr), callback, NULL);
 *
 *    is equivalent to initializing and passing the default subscriber options:
 *
 *    .. code-block:: C
 *
 *       z_subscriber_options_t opts = z_subscriber_options_default();
 *       z_owned_subscriber_t sub = z_declare_pull_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
 */
ZENOHC_API
struct z_owned_pull_subscriber_t z_declare_pull_subscriber(struct z_session_t session,
                                                           struct z_keyexpr_t keyexpr,
                                                           struct z_owned_closure_sample_t *callback,
                                                           const struct z_pull_subscriber_options_t *opts);
/**
 * Creates a Queryable for the given key expression.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression the Queryable will reply to.
 *     callback: The callback function that will be called each time a matching query is received.
 *     options: Options for the queryable.
 *
 * Returns:
 *    The created :c:type:`z_owned_queryable_t` or ``null`` if the creation failed.
 */
ZENOHC_API
struct z_owned_queryable_t z_declare_queryable(struct z_session_t session,
                                               struct z_keyexpr_t keyexpr,
                                               struct z_owned_closure_query_t *callback,
                                               const struct z_queryable_options_t *options);
/**
 * Declare a subscriber for a given key expression.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression to subscribe.
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
struct z_owned_subscriber_t z_declare_subscriber(struct z_session_t session,
                                                 struct z_keyexpr_t keyexpr,
                                                 struct z_owned_closure_sample_t *callback,
                                                 const struct z_subscriber_options_t *opts);
/**
 * Delete data.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression to delete.
 *     options: The put options.
 * Returns:
 *     ``0`` in case of success, negative values in case of failure.
 */
ZENOHC_API
int8_t z_delete(struct z_session_t session,
                struct z_keyexpr_t keyexpr,
                const struct z_delete_options_t *opts);
/**
 * Constructs the default value for :c:type:`z_put_options_t`.
 */
ZENOHC_API struct z_delete_options_t z_delete_options_default(void);
/**
 * Constructs a specific :c:type:`z_encoding_t`.
 */
ZENOHC_API struct z_encoding_t z_encoding(enum z_encoding_prefix_t prefix, const char *suffix);
/**
 * Returns ``true`` if `encoding` is valid.
 */
ZENOHC_API bool z_encoding_check(const struct z_owned_encoding_t *encoding);
/**
 * Constructs a default :c:type:`z_encoding_t`.
 */
ZENOHC_API struct z_encoding_t z_encoding_default(void);
/**
 * Frees `encoding`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_encoding_drop(struct z_owned_encoding_t *encoding);
/**
 * Returns a :c:type:`z_encoding_t` loaned from `encoding`.
 */
ZENOHC_API struct z_encoding_t z_encoding_loan(const struct z_owned_encoding_t *encoding);
/**
 * Constructs a null safe-to-drop value of 'z_owned_encoding_t' type
 */
ZENOHC_API struct z_owned_encoding_t z_encoding_null(void);
/**
 * Query data from the matching queryables in the system.
 * Replies are provided through a callback function.
 *
 * Returns a negative value upon failure.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression matching resources to query.
 *     parameters: The query's parameters, similar to a url's query segment.
 *     callback: The callback function that will be called on reception of replies for this query.
 *               Note that the `reply` parameter of the callback is passed by mutable reference,
 *               but **will** be dropped once your callback exits to help you avoid memory leaks.
 *               If you'd rather take ownership, please refer to the documentation of :c:func:`z_reply_null`
 *     options: additional options for the get.
 */
ZENOHC_API
int8_t z_get(struct z_session_t session,
             struct z_keyexpr_t keyexpr,
             const char *parameters,
             struct z_owned_closure_reply_t *callback,
             const struct z_get_options_t *options);
ZENOHC_API struct z_get_options_t z_get_options_default(void);
/**
 * Returns ``true`` if `hello` is valid.
 */
ZENOHC_API bool z_hello_check(const struct z_owned_hello_t *hello);
/**
 * Frees `hello`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_hello_drop(struct z_owned_hello_t *hello);
/**
 * Returns a :c:type:`z_hello_t` loaned from :c:type:`z_owned_hello_t`.
 */
ZENOHC_API struct z_hello_t z_hello_loan(const struct z_owned_hello_t *hello);
/**
 * Constructs a gravestone value for hello, useful to steal one from a callback
 */
ZENOHC_API struct z_owned_hello_t z_hello_null(void);
/**
 * Fetches the Zenoh IDs of all connected peers.
 *
 * `callback` will be called once for each ID, is guaranteed to never be called concurrently,
 * and is guaranteed to be dropped before this function exits.
 *
 * Retuns 0 on success, negative values on failure
 */
ZENOHC_API
int8_t z_info_peers_zid(struct z_session_t session,
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
int8_t z_info_routers_zid(struct z_session_t session,
                          struct z_owned_closure_zid_t *callback);
/**
 * Returns the local Zenoh ID.
 *
 * Unless the `session` is invalid, that ID is guaranteed to be non-zero.
 * In other words, this function returning an array of 16 zeros means you failed
 * to pass it a valid session.
 */
ZENOHC_API struct z_id_t z_info_zid(struct z_session_t session);
/**
 * Constructs a :c:type:`z_keyexpr_t` departing from a string.
 * It is a loaned key expression that aliases `name`.
 */
ZENOHC_API struct z_keyexpr_t z_keyexpr(const char *name);
/**
 * Returns the key expression's internal string by aliasing it.
 *
 * Currently exclusive to zenoh-c
 */
ZENOHC_API struct z_bytes_t z_keyexpr_as_bytes(struct z_keyexpr_t keyexpr);
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
int8_t z_keyexpr_canonize(char *start,
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
int8_t z_keyexpr_canonize_null_terminated(char *start);
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
struct z_owned_keyexpr_t z_keyexpr_concat(struct z_keyexpr_t left,
                                          const char *right_start,
                                          size_t right_len);
/**
 * Frees `keyexpr` and invalidates it for double-drop safety.
 */
ZENOHC_API void z_keyexpr_drop(struct z_owned_keyexpr_t *keyexpr);
/**
 * Returns ``0`` if both ``left`` and ``right`` are equal. Otherwise, it returns a ``-1``, or other ``negative value`` for errors.
 */
ZENOHC_API
int8_t z_keyexpr_equals(struct z_keyexpr_t left,
                        struct z_keyexpr_t right);
/**
 * Returns ``0`` if ``left`` includes ``right``, i.e. the set defined by ``left`` contains every key belonging to the set
 * defined by ``right``. Otherwise, it returns a ``-1``, or other ``negative value`` for errors.
 */
ZENOHC_API
int8_t z_keyexpr_includes(struct z_keyexpr_t left,
                          struct z_keyexpr_t right);
/**
 * Returns ``0`` if the keyexprs intersect, i.e. there exists at least one key which is contained in both of the
 * sets defined by ``left`` and ``right``. Otherwise, it returns a ``-1``, or other ``negative value`` for errors.
 */
ZENOHC_API
int8_t z_keyexpr_intersects(struct z_keyexpr_t left,
                            struct z_keyexpr_t right);
/**
 * Returns ``0`` if the passed string is a valid (and canon) key expression.
 * Otherwise returns error value
 */
ZENOHC_API int8_t z_keyexpr_is_canon(const char *start, size_t len);
/**
 * Returns ``true`` if `keyexpr` is initialized.
 */
ZENOHC_API bool z_keyexpr_is_initialized(const struct z_keyexpr_t *keyexpr);
/**
 * Performs path-joining (automatically inserting) and returns the result as a `z_owned_keyexpr_t`.
 * In case of error, the return value will be set to its invalidated state.
 */
ZENOHC_API
struct z_owned_keyexpr_t z_keyexpr_join(struct z_keyexpr_t left,
                                        struct z_keyexpr_t right);
/**
 * Returns a :c:type:`z_keyexpr_t` loaned from :c:type:`z_owned_keyexpr_t`.
 */
ZENOHC_API struct z_keyexpr_t z_keyexpr_loan(const struct z_owned_keyexpr_t *keyexpr);
/**
 * Constructs a :c:type:`z_keyexpr_t` departing from a string, copying the passed string.
 */
ZENOHC_API struct z_owned_keyexpr_t z_keyexpr_new(const char *name);
/**
 * Constructs a null safe-to-drop value of 'z_owned_keyexpr_t' type
 */
ZENOHC_API struct z_owned_keyexpr_t z_keyexpr_null(void);
/**
 * Constructs a null-terminated string departing from a :c:type:`z_keyexpr_t`.
 * The user is responsible of droping the returned string using `z_drop`
 */
ZENOHC_API struct z_owned_str_t z_keyexpr_to_string(struct z_keyexpr_t keyexpr);
/**
 * Constructs a :c:type:`z_keyexpr_t` departing from a string without checking any of `z_keyexpr_t`'s assertions:
 *
 *  - `name` MUST be valid UTF8.
 *  - `name` MUST follow the Key Expression specification, ie:
 *
 *   - MUST NOT contain `//`, MUST NOT start nor end with `/`, MUST NOT contain any of the characters `?#$`.
 *   - any instance of `**` may only be lead or followed by `/`.
 *   - the key expression must have canon form.
 *
 * It is a loaned key expression that aliases `name`.
 */
ZENOHC_API
struct z_keyexpr_t z_keyexpr_unchecked(const char *name);
/**
 * Opens a zenoh session. Should the session opening fail, `z_check` ing the returned value will return `false`.
 */
ZENOHC_API
struct z_owned_session_t z_open(struct z_owned_config_t *config);
/**
 * Returns ``true`` if `pub` is valid.
 */
ZENOHC_API bool z_publisher_check(const struct z_owned_publisher_t *pbl);
/**
 * Sends a `DELETE` message onto the publisher's key expression.
 *
 * Returns:
 *     ``0`` in case of success, ``1`` in case of failure.
 */
ZENOHC_API
int8_t z_publisher_delete(struct z_publisher_t publisher,
                          const struct z_publisher_delete_options_t *_options);
/**
 * Constructs the default values for the delete operation via a publisher entity.
 *
 * Returns:
 *   Returns the constructed :c:type:`z_publisher_delete_options_t`.
 */
ZENOHC_API struct z_publisher_delete_options_t z_publisher_delete_options_default(void);
/**
 * Returns the key expression of the publisher
 */
ZENOHC_API struct z_owned_keyexpr_t z_publisher_keyexpr(struct z_publisher_t publisher);
/**
 * Returns a :c:type:`z_publisher_t` loaned from `p`.
 */
ZENOHC_API struct z_publisher_t z_publisher_loan(const struct z_owned_publisher_t *p);
/**
 * Constructs a null safe-to-drop value of 'z_owned_publisher_t' type
 */
ZENOHC_API struct z_owned_publisher_t z_publisher_null(void);
/**
 * Constructs the default value for :c:type:`z_publisher_options_t`.
 */
ZENOHC_API struct z_publisher_options_t z_publisher_options_default(void);
/**
 * Sends a `PUT` message onto the publisher's key expression.
 *
 * The payload's encoding can be sepcified through the options.
 *
 * Parameters:
 *     session: The zenoh session.
 *     payload: The value to put.
 *     len: The length of the value to put.
 *     options: The publisher put options.
 * Returns:
 *     ``0`` in case of success, negative values in case of failure.
 */
ZENOHC_API
int8_t z_publisher_put(struct z_publisher_t publisher,
                       const uint8_t *payload,
                       size_t len,
                       const struct z_publisher_put_options_t *options);
/**
 * Constructs the default value for :c:type:`z_publisher_put_options_t`.
 */
ZENOHC_API struct z_publisher_put_options_t z_publisher_put_options_default(void);
/**
 * Returns ``true`` if `sub` is valid.
 */
ZENOHC_API bool z_pull_subscriber_check(const struct z_owned_pull_subscriber_t *sub);
/**
 * Returns ``true`` if `sub` is valid.
 */
ZENOHC_API
struct z_pull_subscriber_t z_pull_subscriber_loan(const struct z_owned_pull_subscriber_t *sub);
/**
 * Constructs a null safe-to-drop value of 'z_owned_pull_subscriber_t' type
 */
ZENOHC_API struct z_owned_pull_subscriber_t z_pull_subscriber_null(void);
/**
 * Constructs the default value for :c:type:`z_pull_subscriber_options_t`.
 */
ZENOHC_API struct z_pull_subscriber_options_t z_pull_subscriber_options_default(void);
/**
 * Put data.
 *
 * The payload's encoding can be sepcified through the options.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression to put.
 *     payload: The value to put.
 *     len: The length of the value to put.
 *     options: The put options.
 * Returns:
 *     ``0`` in case of success, negative values in case of failure.
 */
ZENOHC_API
int8_t z_put(struct z_session_t session,
             struct z_keyexpr_t keyexpr,
             const uint8_t *payload,
             size_t len,
             const struct z_put_options_t *opts);
/**
 * Constructs the default value for :c:type:`z_put_options_t`.
 */
ZENOHC_API struct z_put_options_t z_put_options_default(void);
/**
 * Returns the attachment to the query by aliasing.
 *
 * `z_check(return_value) == false` if there was no attachment to the query.
 */
ZENOHC_API struct z_attachment_t z_query_attachment(const struct z_query_t *query);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
bool z_query_channel_closure_call(const struct z_owned_query_channel_closure_t *closure,
                                  struct z_owned_query_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_query_channel_closure_drop(struct z_owned_query_channel_closure_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_query_channel_closure_t' type
 */
ZENOHC_API struct z_owned_query_channel_closure_t z_query_channel_closure_null(void);
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
bool z_query_check(const struct z_owned_query_t *this_);
/**
 * Clones the query, allowing to keep it in an "open" state past the callback's return.
 *
 * This operation is infallible, but may return a gravestone value if `query` itself was a gravestone value (which cannot be the case in a callback).
 */
ZENOHC_API
struct z_owned_query_t z_query_clone(const struct z_query_t *query);
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
 * Get a query's key by aliasing it.
 */
ZENOHC_API struct z_keyexpr_t z_query_keyexpr(const struct z_query_t *query);
/**
 * Aliases the query.
 *
 * This function may not be called with the null pointer, but can be called with the gravestone value.
 */
ZENOHC_API
struct z_query_t z_query_loan(const struct z_owned_query_t *this_);
/**
 * The gravestone value of `z_owned_query_t`.
 */
ZENOHC_API struct z_owned_query_t z_query_null(void);
/**
 * Get a query's `value selector <https://github.com/eclipse-zenoh/roadmap/tree/main/rfcs/ALL/Selectors>`_ by aliasing it.
 */
ZENOHC_API
struct z_bytes_t z_query_parameters(const struct z_query_t *query);
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
 *     key: The key of this reply.
 *     payload: The value of this reply.
 *     len: The length of the value of this reply.
 *     options: The options of this reply.
 */
ZENOHC_API
int8_t z_query_reply(const struct z_query_t *query,
                     struct z_keyexpr_t key,
                     const uint8_t *payload,
                     size_t len,
                     const struct z_query_reply_options_t *options);
/**
 * Constructs the default value for :c:type:`z_query_reply_options_t`.
 */
ZENOHC_API struct z_query_reply_options_t z_query_reply_options_default(void);
/**
 * Create a default :c:type:`z_query_target_t`.
 */
ZENOHC_API enum z_query_target_t z_query_target_default(void);
/**
 * Get a query's `payload value <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md>`_ by aliasing it.
 *
 * **WARNING: This API has been marked as unstable: it works as advertised, but it may change in a future release.**
 */
ZENOHC_API
struct z_value_t z_query_value(const struct z_query_t *query);
/**
 * Returns ``true`` if `qable` is valid.
 */
ZENOHC_API bool z_queryable_check(const struct z_owned_queryable_t *qable);
/**
 * Constructs a null safe-to-drop value of 'z_owned_queryable_t' type
 */
ZENOHC_API struct z_owned_queryable_t z_queryable_null(void);
/**
 * Constructs the default value for :c:type:`z_query_reply_options_t`.
 */
ZENOHC_API struct z_queryable_options_t z_queryable_options_default(void);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
bool z_reply_channel_closure_call(const struct z_owned_reply_channel_closure_t *closure,
                                  struct z_owned_reply_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_reply_channel_closure_drop(struct z_owned_reply_channel_closure_t *closure);
/**
 * Constructs a null safe-to-drop value of 'z_owned_reply_channel_closure_t' type
 */
ZENOHC_API struct z_owned_reply_channel_closure_t z_reply_channel_closure_null(void);
ZENOHC_API void z_reply_channel_drop(struct z_owned_reply_channel_t *channel);
/**
 * Constructs a null safe-to-drop value of 'z_owned_reply_channel_t' type
 */
ZENOHC_API struct z_owned_reply_channel_t z_reply_channel_null(void);
/**
 * Returns ``true`` if `reply_data` is valid.
 */
ZENOHC_API bool z_reply_check(const struct z_owned_reply_t *reply_data);
/**
 * Frees `reply_data`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_reply_drop(struct z_owned_reply_t *reply_data);
/**
 * Yields the contents of the reply by asserting it indicates a failure.
 *
 * You should always make sure that :c:func:`z_reply_is_ok` returns ``false`` before calling this function.
 */
ZENOHC_API
struct z_value_t z_reply_err(const struct z_owned_reply_t *reply);
/**
 * Returns ``true`` if the queryable answered with an OK, which allows this value to be treated as a sample.
 *
 * If this returns ``false``, you should use :c:func:`z_check` before trying to use :c:func:`z_reply_err` if you want to process the error that may be here.
 */
ZENOHC_API
bool z_reply_is_ok(const struct z_owned_reply_t *reply);
/**
 * Returns an invalidated :c:type:`z_owned_reply_t`.
 *
 * This is useful when you wish to take ownership of a value from a callback to :c:func:`z_get`:
 *
 *     - copy the value of the callback's argument's pointee,
 *     - overwrite the pointee with this function's return value,
 *     - you are now responsible for dropping your copy of the reply.
 */
ZENOHC_API struct z_owned_reply_t z_reply_null(void);
/**
 * Yields the contents of the reply by asserting it indicates a success.
 *
 * You should always make sure that :c:func:`z_reply_is_ok` returns ``true`` before calling this function.
 */
ZENOHC_API
struct z_sample_t z_reply_ok(const struct z_owned_reply_t *reply);
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
int8_t z_scout(struct z_owned_scouting_config_t *config,
               struct z_owned_closure_hello_t *callback);
ZENOHC_API bool z_scouting_config_check(const struct z_owned_scouting_config_t *config);
ZENOHC_API struct z_owned_scouting_config_t z_scouting_config_default(void);
ZENOHC_API void z_scouting_config_drop(struct z_owned_scouting_config_t *config);
ZENOHC_API struct z_owned_scouting_config_t z_scouting_config_from(struct z_config_t config);
ZENOHC_API struct z_owned_scouting_config_t z_scouting_config_null(void);
/**
 * Returns ``true`` if `session` is valid.
 */
ZENOHC_API bool z_session_check(const struct z_owned_session_t *session);
/**
 * Returns a :c:type:`z_session_t` loaned from `s`.
 *
 * This handle doesn't increase the refcount of the session, but does allow to do so with `zc_session_rcinc`.
 *
 * # Safety
 * The returned `z_session_t` aliases `z_owned_session_t`'s internal allocation,
 * attempting to use it after all owned handles to the session (including publishers, queryables and subscribers)
 * have been destroyed is UB (likely SEGFAULT)
 */
ZENOHC_API
struct z_session_t z_session_loan(const struct z_owned_session_t *s);
/**
 * Constructs a null safe-to-drop value of 'z_owned_session_t' type
 */
ZENOHC_API struct z_owned_session_t z_session_null(void);
/**
 * Returns ``true`` if `strs` is valid.
 */
ZENOHC_API bool z_str_array_check(const struct z_owned_str_array_t *strs);
/**
 * Frees `strs` and invalidates it for double-drop safety.
 */
ZENOHC_API void z_str_array_drop(struct z_owned_str_array_t *strs);
/**
 * Returns a :c:type:`z_str_array_t` loaned from :c:type:`z_owned_str_array_t`.
 */
ZENOHC_API struct z_str_array_t z_str_array_loan(const struct z_owned_str_array_t *strs);
/**
 * Returns ``true`` if `s` is a valid string
 */
ZENOHC_API bool z_str_check(const struct z_owned_str_t *s);
/**
 * Frees `z_owned_str_t`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_str_drop(struct z_owned_str_t *s);
/**
 * Returns :c:type:`z_str_t` structure loaned from :c:type:`z_owned_str_t`.
 */
ZENOHC_API const char *z_str_loan(const struct z_owned_str_t *s);
/**
 * Returns undefined `z_owned_str_t`
 */
ZENOHC_API struct z_owned_str_t z_str_null(void);
/**
 * Returns ``true`` if `sub` is valid.
 */
ZENOHC_API bool z_subscriber_check(const struct z_owned_subscriber_t *sub);
/**
 * Returns the key expression of the subscriber.
 */
ZENOHC_API struct z_owned_keyexpr_t z_subscriber_keyexpr(struct z_subscriber_t subscriber);
/**
 * Returns a :c:type:`z_subscriber_t` loaned from `p`.
 */
ZENOHC_API struct z_subscriber_t z_subscriber_loan(const struct z_owned_subscriber_t *p);
/**
 * Constructs a null safe-to-drop value of 'z_owned_subscriber_t' type
 */
ZENOHC_API struct z_owned_subscriber_t z_subscriber_null(void);
/**
 * Constructs the default value for :c:type:`z_subscriber_options_t`.
 */
ZENOHC_API struct z_subscriber_options_t z_subscriber_options_default(void);
/**
 * Pull data for :c:type:`z_owned_pull_subscriber_t`. The pulled data will be provided
 * by calling the **callback** function provided to the :c:func:`z_declare_subscriber` function.
 *
 * Parameters:
 *     sub: The :c:type:`z_owned_pull_subscriber_t` to pull from.
 */
ZENOHC_API int8_t z_subscriber_pull(struct z_pull_subscriber_t sub);
/**
 * Returns ``true`` if `ts` is a valid timestamp
 */
ZENOHC_API bool z_timestamp_check(struct z_timestamp_t ts);
/**
 * Undeclare the key expression generated by a call to :c:func:`z_declare_keyexpr`.
 */
ZENOHC_API int8_t z_undeclare_keyexpr(struct z_session_t session, struct z_owned_keyexpr_t *kexpr);
/**
 * Undeclares the given :c:type:`z_owned_publisher_t`, droping it and invalidating it for double-drop safety.
 */
ZENOHC_API
int8_t z_undeclare_publisher(struct z_owned_publisher_t *publisher);
/**
 * Undeclares the given :c:type:`z_owned_pull_subscriber_t`, droping it and invalidating it for double-drop safety.
 */
ZENOHC_API
int8_t z_undeclare_pull_subscriber(struct z_owned_pull_subscriber_t *sub);
/**
 * Undeclares a `z_owned_queryable_t`, droping it and invalidating it for doube-drop safety.
 *
 * Parameters:
 *     qable: The :c:type:`z_owned_queryable_t` to undeclare.
 */
ZENOHC_API int8_t z_undeclare_queryable(struct z_owned_queryable_t *qable);
/**
 * Undeclares the given :c:type:`z_owned_subscriber_t`, droping it and invalidating it for double-drop safety.
 */
ZENOHC_API
int8_t z_undeclare_subscriber(struct z_owned_subscriber_t *sub);
/**
 * Constructs a configuration by parsing a file at `path`. Currently supported format is JSON5, a superset of JSON.
 */
ZENOHC_API
struct z_owned_config_t zc_config_from_file(const char *path);
/**
 * Reads a configuration from a JSON-serialized string, such as '{mode:"client",connect:{endpoints:["tcp/127.0.0.1:7447"]}}'.
 *
 * Passing a null-ptr will result in a gravestone value (`z_check(x) == false`).
 */
ZENOHC_API
struct z_owned_config_t zc_config_from_str(const char *s);
/**
 * Gets the property with the given path key from the configuration, returning an owned, null-terminated, JSON serialized string.
 * Use `z_drop` to safely deallocate this string
 */
ZENOHC_API
struct z_owned_str_t zc_config_get(struct z_config_t config,
                                   const char *key);
/**
 * Inserts a JSON-serialized `value` at the `key` position of the configuration.
 *
 * Returns 0 if successful, a negative value otherwise.
 */
ZENOHC_API
int8_t zc_config_insert_json(struct z_config_t config,
                             const char *key,
                             const char *value);
/**
 * Converts `config` into a JSON-serialized string, such as '{"mode":"client","connect":{"endpoints":["tcp/127.0.0.1:7447"]}}'.
 */
ZENOHC_API
struct z_owned_str_t zc_config_to_string(struct z_config_t config);
/**
 * Initialises the zenoh runtime logger.
 *
 * Note that unless you built zenoh-c with the `logger-autoinit` feature disabled,
 * this will be performed automatically by `z_open` and `z_scout`.
 */
ZENOHC_API void zc_init_logger(void);
/**
 * Constructs a :c:type:`z_keyexpr_t` departing from a string.
 * It is a loaned key expression that aliases `name`.
 */
ZENOHC_API struct z_keyexpr_t zc_keyexpr_from_slice(const char *name, size_t len);
/**
 * Constructs a :c:type:`z_keyexpr_t` departing from a string without checking any of `z_keyexpr_t`'s assertions:
 * - `name` MUST be valid UTF8.
 * - `name` MUST follow the Key Expression specification, ie:
 *   - MUST NOT contain ``//``, MUST NOT start nor end with ``/``, MUST NOT contain any of the characters ``?#$``.
 *   - any instance of ``**`` may only be lead or followed by ``/``.
 *   - the key expression must have canon form.
 *
 * It is a loaned key expression that aliases `name`.
 */
ZENOHC_API
struct z_keyexpr_t zc_keyexpr_from_slice_unchecked(const char *start,
                                                   size_t len);
/**
 * Returns `true` if the options are valid.
 */
ZENOHC_API
bool zc_liveliness_declaration_options_check(const struct zc_owned_liveliness_declaration_options_t *_opts);
/**
 * Destroys the options.
 */
ZENOHC_API
void zc_liveliness_declaration_options_drop(struct zc_owned_liveliness_declaration_options_t *opts);
/**
 * The gravestone value for `zc_owned_liveliness_declaration_options_t`
 */
ZENOHC_API
struct zc_owned_liveliness_declaration_options_t zc_liveliness_declaration_options_null(void);
/**
 * Declares a subscriber on liveliness tokens that intersect `key`.
 *
 * Parameters:
 *     z_session_t session: The zenoh session.
 *     z_keyexpr_t keyexpr: The key expression to subscribe.
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
struct z_owned_subscriber_t zc_liveliness_declare_subscriber(struct z_session_t session,
                                                             struct z_keyexpr_t key,
                                                             struct z_owned_closure_sample_t *callback,
                                                             const struct zc_owned_liveliness_declare_subscriber_options_t *_options);
/**
 * Constructs and declares a liveliness token on the network.
 *
 * Liveliness token subscribers on an intersecting key expression will receive a PUT sample when connectivity
 * is achieved, and a DELETE sample if it's lost.
 *
 * Passing `NULL` as options is valid and equivalent to a pointer to the default options.
 */
ZENOHC_API
struct zc_owned_liveliness_token_t zc_liveliness_declare_token(struct z_session_t session,
                                                               struct z_keyexpr_t key,
                                                               const struct zc_owned_liveliness_declaration_options_t *_options);
/**
 * Queries liveliness tokens currently on the network with a key expression intersecting with `key`.
 *
 * Note that the same "value stealing" tricks apply as with a normal :c:func:`z_get`
 *
 * Passing `NULL` as options is valid and equivalent to passing a pointer to the default options.
 */
ZENOHC_API
int8_t zc_liveliness_get(struct z_session_t session,
                         struct z_keyexpr_t key,
                         struct z_owned_closure_reply_t *callback,
                         const struct zc_owned_liveliness_get_options_t *options);
/**
 * Returns `true` if the options are valid.
 */
ZENOHC_API
bool zc_liveliness_get_options_check(const struct zc_owned_liveliness_get_options_t *_opts);
/**
 * The gravestone value for `zc_owned_liveliness_get_options_t`
 */
ZENOHC_API struct zc_owned_liveliness_get_options_t zc_liveliness_get_options_default(void);
/**
 * Destroys the options.
 */
ZENOHC_API void zc_liveliness_get_options_drop(struct zc_owned_liveliness_get_options_t *opts);
/**
 * The gravestone value for `zc_owned_liveliness_get_options_t`
 */
ZENOHC_API struct zc_owned_liveliness_get_options_t zc_liveliness_get_options_null(void);
/**
 * Returns `true` if the options are valid.
 */
ZENOHC_API
bool zc_liveliness_subscriber_options_check(const struct zc_owned_liveliness_declare_subscriber_options_t *_opts);
/**
 * Destroys the options.
 */
ZENOHC_API
void zc_liveliness_subscriber_options_drop(struct zc_owned_liveliness_declare_subscriber_options_t *opts);
/**
 * The gravestone value for `zc_owned_liveliness_declare_subscriber_options_t`
 */
ZENOHC_API
struct zc_owned_liveliness_declare_subscriber_options_t zc_liveliness_subscriber_options_null(void);
/**
 * Returns `true` unless the token is at its gravestone value.
 */
ZENOHC_API bool zc_liveliness_token_check(const struct zc_owned_liveliness_token_t *token);
/**
 * The gravestone value for liveliness tokens.
 */
ZENOHC_API struct zc_owned_liveliness_token_t zc_liveliness_token_null(void);
/**
 * Destroys a liveliness token, notifying subscribers of its destruction.
 */
ZENOHC_API void zc_liveliness_undeclare_token(struct zc_owned_liveliness_token_t *token);
/**
 * Returns `false` if `payload` is the gravestone value.
 */
ZENOHC_API bool zc_payload_check(const struct zc_owned_payload_t *payload);
/**
 * Decrements `payload`'s backing refcount, releasing the memory if appropriate.
 */
ZENOHC_API void zc_payload_drop(struct zc_owned_payload_t *payload);
/**
 * Constructs `zc_owned_payload_t`'s gravestone value.
 */
ZENOHC_API struct zc_owned_payload_t zc_payload_null(void);
/**
 * Clones the `payload` by incrementing its reference counter.
 */
ZENOHC_API struct zc_owned_payload_t zc_payload_rcinc(const struct zc_owned_payload_t *payload);
/**
 * Sends a `PUT` message onto the publisher's key expression, transfering the buffer ownership.
 *
 * This is avoids copies when transfering data that was either:
 * - `zc_sample_payload_rcinc`'d from a sample, when forwarding samples from a subscriber/query to a publisher
 * - constructed from a `zc_owned_shmbuf_t`
 *
 * The payload's encoding can be sepcified through the options.
 *
 * Parameters:
 *     session: The zenoh session.
 *     payload: The value to put.
 *     len: The length of the value to put.
 *     options: The publisher put options.
 * Returns:
 *     ``0`` in case of success, negative values in case of failure.
 */
ZENOHC_API
int8_t zc_publisher_put_owned(struct z_publisher_t publisher,
                              struct zc_owned_payload_t *payload,
                              const struct z_publisher_put_options_t *options);
/**
 * Put data, transfering the buffer ownership.
 *
 * This is avoids copies when transfering data that was either:
 * - `zc_sample_payload_rcinc`'d from a sample, when forwarding samples from a subscriber/query to a publisher
 * - constructed from a `zc_owned_shmbuf_t`
 *
 * The payload's encoding can be sepcified through the options.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression to put.
 *     payload: The value to put.
 *     options: The put options.
 * Returns:
 *     ``0`` in case of success, negative values in case of failure.
 */
ZENOHC_API
int8_t zc_put_owned(struct z_session_t session,
                    struct z_keyexpr_t keyexpr,
                    struct zc_owned_payload_t *payload,
                    const struct z_put_options_t *opts);
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
struct z_owned_query_channel_t zc_query_fifo_new(size_t bound);
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
struct z_owned_query_channel_t zc_query_non_blocking_fifo_new(size_t bound);
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
struct z_owned_reply_channel_t zc_reply_fifo_new(size_t bound);
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
struct z_owned_reply_channel_t zc_reply_non_blocking_fifo_new(size_t bound);
/**
 * Clones the sample's payload by incrementing its backing refcount (this doesn't imply any copies).
 */
ZENOHC_API struct zc_owned_payload_t zc_sample_payload_rcinc(const struct z_sample_t *sample);
/**
 * Increments the session's reference count, returning a new owning handle.
 */
ZENOHC_API struct z_owned_session_t zc_session_rcinc(struct z_session_t session);
/**
 * Allocates a buffer of size `capacity` in the manager's memory.
 *
 * # Safety
 * Calling this function concurrently with other shm functions on the same manager is UB.
 */
ZENOHC_API
struct zc_owned_shmbuf_t zc_shm_alloc(const struct zc_owned_shm_manager_t *manager,
                                      size_t capacity);
/**
 * Runs a defragmentation pass on the SHM manager.
 *
 * Note that this doesn't trigger a garbage collection pass, nor does it move currently allocated data.
 *
 * # Safety
 * Calling this function concurrently with other shm functions on the same manager is UB.
 */
ZENOHC_API
size_t zc_shm_defrag(const struct zc_owned_shm_manager_t *manager);
/**
 * Runs a garbage collection pass on the SHM manager.
 *
 * Returns the number of bytes that have been freed by the pass.
 *
 * # Safety
 * Calling this function concurrently with other shm functions on the same manager is UB.
 */
ZENOHC_API size_t zc_shm_gc(const struct zc_owned_shm_manager_t *manager);
ZENOHC_API bool zc_shm_manager_check(const struct zc_owned_shm_manager_t *manager);
ZENOHC_API void zc_shm_manager_drop(struct zc_owned_shm_manager_t *manager);
ZENOHC_API
struct zc_owned_shm_manager_t zc_shm_manager_new(struct z_session_t session,
                                                 const char *id,
                                                 size_t size);
ZENOHC_API struct zc_owned_shm_manager_t zc_shm_manager_null(void);
/**
 * Returns the capacity of the SHM buffer.
 */
ZENOHC_API size_t zc_shmbuf_capacity(const struct zc_owned_shmbuf_t *buf);
/**
 * Returns `false` if `buf` is in its gravestone state.
 */
ZENOHC_API bool zc_shmbuf_check(const struct zc_owned_shmbuf_t *buf);
/**
 * Drops the SHM buffer, decrementing its backing reference counter.
 */
ZENOHC_API void zc_shmbuf_drop(struct zc_owned_shmbuf_t *buf);
/**
 * Constructs an owned payload from an owned SHM buffer.
 */
ZENOHC_API struct zc_owned_payload_t zc_shmbuf_into_payload(struct zc_owned_shmbuf_t *buf);
/**
 * Returns the length of the SHM buffer.
 *
 * Note that when constructing an SHM buffer, length is defaulted to its capacity.
 */
ZENOHC_API size_t zc_shmbuf_length(const struct zc_owned_shmbuf_t *buf);
/**
 * Constructs a null safe-to-drop value of type `zc_owned_shmbuf_t`
 */
ZENOHC_API struct zc_owned_shmbuf_t zc_shmbuf_null(void);
/**
 * Returns the start of the SHM buffer.
 */
ZENOHC_API uint8_t *zc_shmbuf_ptr(const struct zc_owned_shmbuf_t *buf);
/**
 * Sets the length of the SHM buffer.
 *
 * This lets Zenoh know how much of the data to write over the network when sending the value to non-SHM-compatible neighboors.
 */
ZENOHC_API
void zc_shmbuf_set_length(const struct zc_owned_shmbuf_t *buf,
                          size_t len);
ZENOHC_API enum zcu_locality_t zcu_locality_default(void);
ZENOHC_API enum zcu_reply_keyexpr_t zcu_reply_keyexpr_default(void);
/**
 * Declares a Publication Cache.
 *
 * Parameters:
 *     z_session_t session: The zenoh session.
 *     z_keyexpr_t keyexpr: The key expression to publish.
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
struct ze_owned_publication_cache_t ze_declare_publication_cache(struct z_session_t session,
                                                                 struct z_keyexpr_t keyexpr,
                                                                 const struct ze_publication_cache_options_t *options);
/**
 * Declares a Querying Subscriber for a given key expression.
 *
 * Parameters:
 *     z_session_t session: The zenoh session.
 *     z_keyexpr_t keyexpr: The key expression to subscribe.
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
struct ze_owned_querying_subscriber_t ze_declare_querying_subscriber(struct z_session_t session,
                                                                     struct z_keyexpr_t keyexpr,
                                                                     struct z_owned_closure_sample_t *callback,
                                                                     const struct ze_querying_subscriber_options_t *options);
/**
 * Returns ``true`` if `pub_cache` is valid.
 */
ZENOHC_API bool ze_publication_cache_check(const struct ze_owned_publication_cache_t *pub_cache);
/**
 * Constructs a null safe-to-drop value of 'ze_owned_publication_cache_t' type
 */
ZENOHC_API struct ze_owned_publication_cache_t ze_publication_cache_null(void);
/**
 * Constructs the default value for :c:type:`ze_publication_cache_options_t`.
 */
ZENOHC_API struct ze_publication_cache_options_t ze_publication_cache_options_default(void);
/**
 * Returns ``true`` if `sub` is valid.
 */
ZENOHC_API bool ze_querying_subscriber_check(const struct ze_owned_querying_subscriber_t *sub);
/**
 * Make a :c:type:`ze_owned_querying_subscriber_t` to perform an additional query on a specified selector.
 * The queried samples will be merged with the received publications and made available in the subscriber callback.
 */
ZENOHC_API
int8_t ze_querying_subscriber_get(struct ze_querying_subscriber_t sub,
                                  struct z_keyexpr_t selector,
                                  const struct z_get_options_t *options);
/**
 * Returns a :c:type:`ze_querying_subscriber_loan` loaned from `p`.
 */
ZENOHC_API
struct ze_querying_subscriber_t ze_querying_subscriber_loan(const struct ze_owned_querying_subscriber_t *p);
/**
 * Constructs a null safe-to-drop value of 'ze_owned_querying_subscriber_t' type
 */
ZENOHC_API struct ze_owned_querying_subscriber_t ze_querying_subscriber_null(void);
/**
 * Constructs the default value for :c:type:`ze_querying_subscriber_options_t`.
 */
ZENOHC_API struct ze_querying_subscriber_options_t ze_querying_subscriber_options_default(void);
/**
 * Closes the given :c:type:`ze_owned_publication_cache_t`, droping it and invalidating it for double-drop safety.
 */
ZENOHC_API
int8_t ze_undeclare_publication_cache(struct ze_owned_publication_cache_t *pub_cache);
/**
 * Undeclares the given :c:type:`ze_owned_querying_subscriber_t`, droping it and invalidating it for double-drop safety.
 */
ZENOHC_API
int8_t ze_undeclare_querying_subscriber(struct ze_owned_querying_subscriber_t *sub);
