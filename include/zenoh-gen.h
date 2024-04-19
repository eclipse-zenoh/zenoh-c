#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>




#define DEFAULT_SCOUTING_TIMEOUT 1000

/**
 * The kind of congestion control.
 *
 *     - **BLOCK**
 *     - **DROP**
 */
typedef enum z_congestion_control_t {
  Z_CONGESTION_CONTROL_T_BLOCK,
  Z_CONGESTION_CONTROL_T_DROP,
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
  Z_CONSOLIDATION_MODE_T_AUTO = -1,
  Z_CONSOLIDATION_MODE_T_NONE = 0,
  Z_CONSOLIDATION_MODE_T_MONOTONIC = 1,
  Z_CONSOLIDATION_MODE_T_LATEST = 2,
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
  Z_KEYEXPR_INTERSECTION_LEVEL_T_DISJOINT = 0,
  Z_KEYEXPR_INTERSECTION_LEVEL_T_INTERSECTS = 1,
  Z_KEYEXPR_INTERSECTION_LEVEL_T_INCLUDES = 2,
  Z_KEYEXPR_INTERSECTION_LEVEL_T_EQUALS = 3,
} z_keyexpr_intersection_level_t;

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
  Z_PRIORITY_T_REAL_TIME = 1,
  Z_PRIORITY_T_INTERACTIVE_HIGH = 2,
  Z_PRIORITY_T_INTERACTIVE_LOW = 3,
  Z_PRIORITY_T_DATA_HIGH = 4,
  Z_PRIORITY_T_DATA = 5,
  Z_PRIORITY_T_DATA_LOW = 6,
  Z_PRIORITY_T_BACKGROUND = 7,
} z_priority_t;

/**
 * The Queryables that should be target of a :c:func:`z_get`.
 *
 *     - **BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
 *     - **ALL_COMPLETE**: All complete queryables.
 *     - **ALL**: All matching queryables.
 */
typedef enum z_query_target_t {
  Z_QUERY_TARGET_T_BEST_MATCHING,
  Z_QUERY_TARGET_T_ALL,
  Z_QUERY_TARGET_T_ALL_COMPLETE,
} z_query_target_t;

typedef enum z_sample_kind_t {
  Z_SAMPLE_KIND_T_PUT = 0,
  Z_SAMPLE_KIND_T_DELETE = 1,
} z_sample_kind_t;

typedef enum zcu_locality_t {
  ZCU_LOCALITY_T_ANY = 0,
  ZCU_LOCALITY_T_SESSION_LOCAL = 1,
  ZCU_LOCALITY_T_REMOTE = 2,
} zcu_locality_t;

typedef enum zcu_reply_keyexpr_t {
  ZCU_REPLY_KEYEXPR_T_ANY = 0,
  ZCU_REPLY_KEYEXPR_T_MATCHING_QUERY = 1,
} zcu_reply_keyexpr_t;

/**
 * A split buffer that owns all of its data.
 *
 * To minimize copies and reallocations, Zenoh may provide you data in split buffers.
 */
typedef struct ALIGN(8) z_owned_bytes_t {
  uint8_t _0[40];
} z_owned_bytes_t;

typedef int8_t ZCError;

/**
 * A loaned payload.
 */
typedef struct ALIGN(8) z_bytes_t {
  uint8_t _0[8];
} z_bytes_t;

typedef struct z_owned_slice_t {
  uint8_t *start;
  size_t len;
} z_owned_slice_t;

/**
 * A map of maybe-owned vector of bytes to maybe-owned vector of bytes.
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
typedef struct z_owned_str_t {
  char *_cstr;
} z_owned_str_t;

/**
 * A contiguous view of bytes owned by some other entity.
 *
 * `start` being `null` is considered a gravestone value,
 * and empty slices are represented using a possibly dangling pointer for `start`.
 */
typedef struct z_slice_t {
  const uint8_t *start;
  size_t len;
} z_slice_t;

typedef struct ALIGN(8) z_slice_map_t {
  uint8_t _0[8];
} z_slice_map_t;

/**
 * A reader for payload data.
 */
typedef struct ALIGN(8) z_owned_bytes_t_reader_t {
  uint8_t _0[24];
} z_owned_bytes_t_reader_t;

typedef struct ALIGN(8) z_bytes_reader_t {
  uint8_t _0[8];
} z_bytes_reader_t;

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
typedef struct ALIGN(8) z_owned_session_t {
  uint8_t _0[8];
} z_owned_session_t;

/**
 * Represents a Zenoh ID.
 *
 * In general, valid Zenoh IDs are LSB-first 128bit unsigned and non-zero integers.
 */
typedef struct ALIGN(1) z_id_t {
  uint8_t _0[16];
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
 *
 * Queries are atomically reference-counted, letting you extract them from the callback that handed them to you by cloning.
 * `z_query_t`'s are valid as long as at least one corresponding `z_owned_query_t` exists, including the one owned by Zenoh until the callback returns.
 */
typedef struct ALIGN(8) z_owned_query_t {
  uint8_t _0[16];
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

typedef struct ALIGN(8) z_query_t {
  uint8_t _0[8];
} z_query_t;

/**
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 *
 * Members:
 *   void *context: a pointer to an arbitrary state.
 *   void *call(z_query_t, const void *context): the typical callback function. `context` will be passed as its last argument.
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
  void (*call)(struct z_query_t, void *context);
  void (*drop)(void*);
} z_owned_closure_query_t;

typedef struct ALIGN(8) z_reply_t {
  uint8_t _0[8];
} z_reply_t;

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
  void (*call)(struct z_reply_t, void*);
  void (*drop)(void*);
} z_owned_closure_reply_t;

typedef struct ALIGN(8) z_sample_t {
  uint8_t _0[8];
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
typedef struct ALIGN(8) z_owned_config_t {
  uint8_t _0[8];
} z_owned_config_t;

/**
 * A loaned zenoh configuration.
 */
typedef struct ALIGN(8) z_config_t {
  uint8_t _0[8];
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
typedef struct ALIGN(8) z_owned_keyexpr_t {
  uint8_t _0[32];
} z_owned_keyexpr_t;

typedef struct ALIGN(8) z_session_t {
  uint8_t _0[8];
} z_session_t;

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
typedef struct ALIGN(8) z_keyexpr_t {
  uint8_t _0[8];
} z_keyexpr_t;

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
 * Options passed to the :c:func:`z_declare_queryable` function.
 *
 * Members:
 *     bool complete: The completeness of the Queryable.
 */
typedef struct z_queryable_options_t {
  bool complete;
} z_queryable_options_t;

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
typedef struct ALIGN(8) z_owned_queryable_t {
  uint8_t _0[32];
} z_owned_queryable_t;

typedef struct ALIGN(8) z_owned_encoding_t {
  uint8_t _0[48];
} z_owned_encoding_t;

/**
 * The encoding of a payload, in a MIME-like format.
 *
 * For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
 */
typedef struct ALIGN(8) z_encoding_t {
  uint8_t _0[8];
} z_encoding_t;

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
 *     z_query_target_t target: The Queryables that should be target of the query.
 *     z_query_consolidation_t consolidation: The replies consolidation strategy to apply on replies to the query.
 *     z_value_t value: An optional value to attach to the query.
 *    z_bytes_t attachment: The attachment to attach to the query.
 *     uint64_t timeout: The timeout for the query in milliseconds. 0 means default query timeout from zenoh configuration.
 */
typedef struct z_get_options_t {
  enum z_query_target_t target;
  struct z_query_consolidation_t consolidation;
  struct z_owned_bytes_t *payload;
  struct z_owned_encoding_t *encoding;
  struct z_owned_bytes_t *attachment;
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

typedef struct ALIGN(8) z_publisher_t {
  uint8_t _0[8];
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
 *     z_owned_encoding_t encoding: The encoding of the payload.
 *    z_owned_bytes_t attachment: The attachment to attach to the publication.
 */
typedef struct z_publisher_put_options_t {
  struct z_owned_encoding_t *encoding;
  struct z_owned_bytes_t *attachment;
} z_publisher_put_options_t;

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

typedef struct ALIGN(8) z_value_t {
  uint8_t _0[8];
} z_value_t;

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

typedef struct ALIGN(8) z_timestamp_t {
  uint8_t _0[24];
} z_timestamp_t;

typedef struct z_owned_scouting_config_t {
  struct z_owned_config_t _config;
  unsigned long zc_timeout_ms;
  uint8_t zc_what;
} z_owned_scouting_config_t;

/**
 * The body of a loop over a z_slice_map's key-value pairs.
 *
 * `key` and `value` are loaned to the body for the duration of a single call.
 * `context` is passed transparently through the iteration driver.
 *
 * Returning `true` is treated as `continue`.
 */
typedef bool (*z_slice_map_iter_body_t)(struct z_slice_t key, struct z_slice_t value, void *context);

/**
 * An owned sample.
 *
 * This is a read only type that can only be constructed by cloning a `z_sample_t`.
 * Like all owned types, its memory must be freed by passing a mutable reference to it to `zc_sample_drop`.
 */
typedef struct ALIGN(8) zc_owned_sample_t {
  uint8_t _0[240];
} zc_owned_sample_t;

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

#define Z_OK 0

#define Z_EINVAL -1

#define Z_EPARSE -2

#define Z_EIO -3

#define Z_ENETWORK -4

#define Z_EGENERIC INT8_MIN

extern const unsigned int Z_ROUTER;

extern const unsigned int Z_PEER;

extern const unsigned int Z_CLIENT;

extern const char *Z_CONFIG_MODE_KEY;

extern const char *Z_CONFIG_CONNECT_KEY;

extern const char *Z_CONFIG_LISTEN_KEY;

extern const char *Z_CONFIG_USER_KEY;

extern const char *Z_CONFIG_PASSWORD_KEY;

extern const char *Z_CONFIG_MULTICAST_SCOUTING_KEY;

extern const char *Z_CONFIG_MULTICAST_INTERFACE_KEY;

extern const char *Z_CONFIG_MULTICAST_IPV4_ADDRESS_KEY;

extern const char *Z_CONFIG_SCOUTING_TIMEOUT_KEY;

extern const char *Z_CONFIG_SCOUTING_DELAY_KEY;

extern const char *Z_CONFIG_ADD_TIMESTAMP_KEY;

/**
 * Returns `true` if the payload is in a valid state.
 */
ZENOHC_API bool z_bytes_check(const struct z_owned_bytes_t *payload);

/**
 * Increments the payload's reference count, returning an owned version of it.
 */
ZENOHC_API void z_bytes_clone(const struct z_owned_bytes_t *src, struct z_owned_bytes_t *dst);

/**
 * Decodes payload into owned bytes
 */
ZENOHC_API ZCError z_bytes_decode_into_bytes(struct z_bytes_t payload, struct z_owned_slice_t *dst);

/**
 * Decodes payload into bytes map.
 */
ZENOHC_API
ZCError z_bytes_decode_into_bytes_map(struct z_bytes_t payload,
                                      struct z_owned_slice_map_t *dst);

/**
 * Decodes payload into null-terminated string.
 */
ZENOHC_API ZCError z_bytes_decode_into_string(struct z_bytes_t payload, struct z_owned_str_t *dst);

/**
 * Decrements the payload's reference counter, destroying it if applicable.
 *
 * `this` will be reset to `z_buffer_null`, preventing UB on double-frees.
 */
ZENOHC_API void z_bytes_drop(struct z_owned_bytes_t *this_);

/**
 * Encodes byte sequence by aliasing.
 */
ZENOHC_API void z_bytes_encode_from_bytes(struct z_owned_bytes_t *this_, struct z_slice_t bytes);

/**
 * Encodes bytes map by copying.
 */
ZENOHC_API
void z_bytes_encode_from_bytes_map(struct z_owned_bytes_t *this_,
                                   struct z_slice_map_t bytes_map);

/**
 * Encodes a null-terminated string by aliasing.
 */
ZENOHC_API void z_bytes_encode_from_string(struct z_owned_bytes_t *this_, const char *cstr);

/**
 * Returns total number bytes in the payload.
 */
ZENOHC_API size_t z_bytes_len(struct z_bytes_t payload);

/**
 * Loans the payload, allowing you to call functions that only need a loan of it.
 */
ZENOHC_API struct z_bytes_t z_bytes_loan(const struct z_owned_bytes_t *payload);

/**
 * The gravestone value for `z_owned_bytes_t`.
 */
ZENOHC_API void z_bytes_null(struct z_owned_bytes_t *this_);

ZENOHC_API bool z_bytes_reader_check(const struct z_owned_bytes_t_reader_t *reader);

ZENOHC_API void z_bytes_reader_drop(struct z_owned_bytes_t_reader_t *this_);

ZENOHC_API
struct z_bytes_reader_t z_bytes_reader_loan(const struct z_owned_bytes_t_reader_t *reader);

/**
 * Creates a reader for the specified `payload`.
 *
 * Returns 0 in case of success, -1 if `payload` is not valid.
 */
ZENOHC_API
void z_bytes_reader_new(struct z_bytes_t payload,
                        struct z_owned_bytes_t_reader_t *this_);

ZENOHC_API void z_bytes_reader_null(struct z_owned_bytes_t_reader_t *this_);

/**
 * Reads data into specified destination.
 *
 * Will read at most `len` bytes.
 * Returns number of bytes read. If return value is smaller than `len`, it means that end of the payload was reached.
 */
ZENOHC_API
size_t z_bytes_reader_read(struct z_bytes_reader_t reader,
                           uint8_t *dest,
                           size_t len);

/**
 * Sets the `reader` position indicator for the payload to the value pointed to by offset.
 * The new position is exactly offset bytes measured from the beginning of the payload if origin is SEEK_SET,
 * from the current reader position if origin is SEEK_CUR, and from the end of the payload if origin is SEEK_END.
 * Return ​0​ upon success, negative error code otherwise.
 */
ZENOHC_API
ZCError z_bytes_reader_seek(struct z_bytes_reader_t reader,
                            int64_t offset,
                            int origin);

/**
 * Returns the read position indicator.
 * Returns read position indicator on success or -1L if failure occurs.
 */
ZENOHC_API int64_t z_bytes_reader_tell(struct z_bytes_reader_t reader);

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
                          struct z_query_t query);

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
                          struct z_reply_t reply);

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
ZENOHC_API
ZCError z_config_client(const char *const *peers,
                        size_t n_peers,
                        struct z_owned_config_t *this_);

/**
 * Clones the config.
 */
ZENOHC_API void z_config_clone(const struct z_config_t *src, struct z_owned_config_t *dst);

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
void z_config_new(struct z_owned_config_t *this_);

/**
 * Constructs a null safe-to-drop value of 'z_owned_config_t' type
 */
ZENOHC_API void z_config_null(struct z_owned_config_t *this_);

/**
 * Constructs a default, zenoh-allocated, peer mode configuration.
 */
ZENOHC_API void z_config_peer(struct z_owned_config_t *this_);

/**
 * Declare a key expression. The id is returned as a :c:type:`z_keyexpr_t` with a nullptr suffix.
 *
 * This numerical id will be used on the network to save bandwidth and
 * ease the retrieval of the concerned resource in the routing tables.
 */
ZENOHC_API
ZCError z_declare_keyexpr(struct z_owned_keyexpr_t *this_,
                          struct z_session_t session,
                          struct z_keyexpr_t key_expr);

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
ZCError z_declare_publisher(struct z_session_t session,
                            struct z_keyexpr_t key_expr,
                            const struct z_publisher_options_t *options,
                            struct z_owned_publisher_t *this_);

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
ZCError z_declare_queryable(struct z_session_t session,
                            struct z_keyexpr_t key_expr,
                            struct z_owned_closure_query_t *callback,
                            const struct z_queryable_options_t *options,
                            struct z_owned_queryable_t *this_);

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
 * Constructs a specific :c:type:`z_encoding_t`.
 */
ZENOHC_API int8_t z_encoding_from_str(struct z_owned_encoding_t *encoding, const char *s);

/**
 * Returns a :c:type:`z_encoding_t` loaned from `encoding`.
 */
ZENOHC_API struct z_encoding_t z_encoding_loan(const struct z_owned_encoding_t *encoding);

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
ZCError z_get(struct z_session_t session,
              struct z_keyexpr_t key_expr,
              const char *parameters,
              struct z_owned_closure_reply_t *callback,
              struct z_get_options_t options);

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
ZCError z_info_peers_zid(struct z_session_t session,
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
ZCError z_info_routers_zid(struct z_session_t session,
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
ZENOHC_API ZCError z_keyexpr(struct z_owned_keyexpr_t *this_, const char *name);

/**
 * Returns the key expression's internal string by aliasing it.
 *
 * Currently exclusive to zenoh-c
 */
ZENOHC_API struct z_slice_t z_keyexpr_as_bytes(struct z_keyexpr_t ke);

/**
 * Constructs a :c:type:`z_owned_keyexpr_t` by aliasing a string.
 * The string is canonized in-place before being passed to keyexpr.
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 */
ZENOHC_API
ZCError z_keyexpr_autocanonize(struct z_owned_keyexpr_t *this_,
                               char *name);

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
ZCError z_keyexpr_canonize(char *start,
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
ZCError z_keyexpr_canonize_null_terminated(char *start);

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
ZCError z_keyexpr_concat(struct z_keyexpr_t left,
                         const char *right_start,
                         size_t right_len,
                         struct z_owned_keyexpr_t *this_);

/**
 * Frees `keyexpr` and invalidates it for double-drop safety.
 */
ZENOHC_API void z_keyexpr_drop(struct z_owned_keyexpr_t *keyexpr);

/**
 * Returns ``0`` if both ``left`` and ``right`` are equal.
 */
ZENOHC_API bool z_keyexpr_equals(struct z_keyexpr_t left, struct z_keyexpr_t right);

/**
 * Returns ``0`` if ``left`` includes ``right``, i.e. the set defined by ``left`` contains every key belonging to the set
 * defined by ``right``.
 */
ZENOHC_API
bool z_keyexpr_includes(struct z_keyexpr_t left,
                        struct z_keyexpr_t right);

/**
 * Returns ``0`` if the keyexprs intersect, i.e. there exists at least one key which is contained in both of the
 * sets defined by ``left`` and ``right``.
 */
ZENOHC_API
bool z_keyexpr_intersects(struct z_keyexpr_t left,
                          struct z_keyexpr_t right);

/**
 * Returns ``0`` if the passed string is a valid (and canon) key expression.
 * Otherwise returns error value
 */
ZENOHC_API ZCError z_keyexpr_is_canon(const char *start, size_t len);

/**
 * Performs path-joining (automatically inserting) and returns the result as a `z_owned_keyexpr_t`.
 * In case of error, the return value will be set to its invalidated state.
 */
ZENOHC_API
ZCError z_keyexpr_join(struct z_keyexpr_t left,
                       struct z_keyexpr_t right,
                       struct z_owned_keyexpr_t *this_);

/**
 * Returns a :c:type:`z_keyexpr_t` loaned from :c:type:`z_owned_keyexpr_t`.
 */
ZENOHC_API struct z_keyexpr_t z_keyexpr_loan(const struct z_owned_keyexpr_t *key_expr);

/**
 * Constructs a :c:type:`z_owned_keyexpr_t` departing from a string, copying the passed string.
 */
ZENOHC_API ZCError z_keyexpr_new(const char *name, struct z_owned_keyexpr_t *this_);

/**
 * Constructs a :c:type:`z_owned_keyexpr_t` departing from a string, copying the passed string. The copied string is canonized.
 */
ZENOHC_API
ZCError z_keyexpr_new_autocanonize(const char *name,
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
enum z_keyexpr_intersection_level_t z_keyexpr_relation_to(struct z_keyexpr_t left,
                                                          struct z_keyexpr_t right);

/**
 * Constructs a null-terminated string departing from a :c:type:`z_keyexpr_t`.
 * The user is responsible of droping the returned string using `z_drop`
 */
ZENOHC_API struct z_owned_str_t z_keyexpr_to_string(struct z_keyexpr_t ke);

/**
 * Constructs a :c:type:`z_owned_keyexpr_t` by aliasing a string without checking any of `z_keyexpr_t`'s assertions:
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
void z_keyexpr_unchecked(struct z_owned_keyexpr_t *this_,
                         const char *name);

/**
 * Opens a zenoh session. Should the session opening fail, `z_check` ing the returned value will return `false`.
 * Config value is always consumed upon function return.
 */
ZENOHC_API
ZCError z_open(struct z_owned_session_t *this_,
               struct z_owned_config_t *config);

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
ZCError z_publisher_delete(struct z_publisher_t publisher,
                           struct z_publisher_delete_options_t _options);

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
ZENOHC_API struct z_keyexpr_t z_publisher_keyexpr(struct z_publisher_t publisher);

/**
 * Returns a :c:type:`z_publisher_t` loaned from `p`.
 */
ZENOHC_API struct z_publisher_t z_publisher_loan(const struct z_owned_publisher_t *p);

/**
 * Constructs a null safe-to-drop value of 'z_owned_publisher_t' type
 */
ZENOHC_API void z_publisher_null(struct z_owned_publisher_t *this_);

/**
 * Constructs the default value for :c:type:`z_publisher_options_t`.
 */
ZENOHC_API struct z_publisher_options_t z_publisher_options_default(void);

/**
 * Sends a `PUT` message onto the publisher's key expression, transfering the payload ownership.
 *
 * This is avoids copies when transfering data that was either:
 * - `zc_sample_payload_rcinc`'d from a sample, when forwarding samples from a subscriber/query to a publisher
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
ZCError z_publisher_put(struct z_publisher_t publisher,
                        struct z_owned_bytes_t *payload,
                        struct z_publisher_put_options_t options);

/**
 * Constructs the default value for :c:type:`z_publisher_put_options_t`.
 */
ZENOHC_API struct z_publisher_put_options_t z_publisher_put_options_default(void);

/**
 * Gets the attachment to the query by aliasing.
 *
 * Before calling this funciton, the user must ensure that `z_query_has_attachment` returns true.
 */
ZENOHC_API struct z_bytes_t z_query_attachment(struct z_query_t query);

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
bool z_query_check(const struct z_owned_query_t *query);

/**
 * Clones the query, allowing to keep it in an "open" state past the callback's return.
 *
 * This operation is infallible, but may return a gravestone value if `query` itself was a gravestone value (which cannot be the case in a callback).
 */
ZENOHC_API
void z_query_clone(struct z_owned_query_t *this_,
                   struct z_query_t query);

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
ZENOHC_API struct z_keyexpr_t z_query_keyexpr(struct z_query_t query);

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
ZENOHC_API void z_query_null(struct z_owned_query_t *this_);

/**
 * Get a query's `value selector <https://github.com/eclipse-zenoh/roadmap/tree/main/rfcs/ALL/Selectors>`_ by aliasing it.
 */
ZENOHC_API
struct z_slice_t z_query_parameters(struct z_query_t query);

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
ZCError z_query_reply(struct z_query_t query,
                      struct z_keyexpr_t key_expr,
                      struct z_owned_bytes_t *payload,
                      struct z_query_reply_options_t options);

/**
 * Constructs the default value for :c:type:`z_query_reply_options_t`.
 */
ZENOHC_API struct z_query_reply_options_t z_query_reply_options_default(void);

/**
 * Create a default :c:type:`z_query_target_t`.
 */
ZENOHC_API enum z_query_target_t z_query_target_default(void);

/**
 * Gets a query's `payload value <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md>`_ by aliasing it.
 *
 * **WARNING: This API has been marked as unstable: it works as advertised, but it may change in a future release.**
 * Before calling this funciton, the user must ensure that `z_query_has_value` returns true.
 */
ZENOHC_API
struct z_value_t z_query_value(struct z_query_t query);

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
ZENOHC_API struct z_queryable_options_t z_queryable_options_default(void);

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
ZENOHC_API struct z_owned_reply_channel_closure_t z_reply_channel_closure_null(void);

ZENOHC_API void z_reply_channel_drop(struct z_owned_reply_channel_t *channel);

/**
 * Constructs a null safe-to-drop value of 'z_owned_reply_channel_t' type
 */
ZENOHC_API struct z_owned_reply_channel_t z_reply_channel_null(void);

/**
 * Returns ``true`` if `reply` is valid.
 */
ZENOHC_API bool z_reply_check(const struct z_owned_reply_t *this_);

ZENOHC_API void z_reply_clone(struct z_owned_reply_t *this_, struct z_reply_t reply);

/**
 * Frees `reply`, invalidating it for double-drop safety.
 */
ZENOHC_API void z_reply_drop(struct z_owned_reply_t *this_);

/**
 * Yields the contents of the reply by asserting it indicates a failure.
 *
 * You should always make sure that :c:func:`z_reply_is_ok` returns ``false`` before calling this function.
 */
ZENOHC_API
struct z_value_t z_reply_err(struct z_reply_t reply);

/**
 * Returns ``true`` if the queryable answered with an OK, which allows this value to be treated as a sample.
 *
 * If this returns ``false``, you should use :c:func:`z_check` before trying to use :c:func:`z_reply_err` if you want to process the error that may be here.
 */
ZENOHC_API
bool z_reply_is_ok(struct z_reply_t reply);

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
 * You should always make sure that :c:func:`z_reply_is_ok` returns ``true`` before calling this function.
 */
ZENOHC_API
struct z_sample_t z_reply_ok(struct z_reply_t reply);

/**
 * Gets sample's attachment.
 *
 * Before calling this function, ensure that `zc_sample_has_attachment` returns true
 */
ZENOHC_API struct z_bytes_t z_sample_attachment(struct z_sample_t sample);

/**
 * The encoding of the payload.
 */
ZENOHC_API struct z_encoding_t z_sample_encoding(struct z_sample_t sample);

/**
 * The qos with which the sample was received.
 * TODO: split to methods (priority, congestion_control, express)
 * Checks if sample contains an attachment.
 */
ZENOHC_API bool z_sample_has_attachment(struct z_sample_t sample);

/**
 * The Key Expression of the sample.
 *
 * `sample` is aliased by its return value.
 */
ZENOHC_API struct z_keyexpr_t z_sample_keyexpr(const struct z_sample_t *sample);

/**
 * The sample's kind (put or delete).
 */
ZENOHC_API enum z_sample_kind_t z_sample_kind(const struct z_sample_t *sample);

/**
 * The sample's data, the return value aliases the sample.
 *
 */
ZENOHC_API struct z_bytes_t z_sample_payload(const struct z_sample_t *sample);

/**
 * The samples timestamp
 *
 * Returns true if Sample contains timestamp, false otherwise. In the latter case the timestamp_out value is not altered.
 */
ZENOHC_API
bool z_sample_timestamp(const struct z_sample_t *sample,
                        struct z_timestamp_t *timestamp_out);

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
ZCError z_scout(struct z_owned_scouting_config_t *config,
                struct z_owned_closure_hello_t *callback);

ZENOHC_API bool z_scouting_config_check(const struct z_owned_scouting_config_t *config);

ZENOHC_API void z_scouting_config_default(struct z_owned_scouting_config_t *this_);

ZENOHC_API void z_scouting_config_drop(struct z_owned_scouting_config_t *config);

ZENOHC_API
void z_scouting_config_from(struct z_config_t config,
                            struct z_owned_scouting_config_t *this_);

ZENOHC_API void z_scouting_config_null(struct z_owned_scouting_config_t *this_);

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
ZENOHC_API void z_session_null(struct z_owned_session_t *s);

/**
 * Returns ``true`` if `b` is initialized.
 */
ZENOHC_API bool z_slice_check(const struct z_owned_slice_t *b);

ZENOHC_API struct z_owned_slice_t z_slice_clone(const struct z_slice_t *b);

/**
 * Returns the gravestone value for `z_slice_t`
 */
ZENOHC_API struct z_slice_t z_slice_empty(void);

/**
 * Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
 *
 * `str == NULL` will cause this to return `z_slice_empty()`
 */
ZENOHC_API struct z_slice_t z_slice_from_str(const char *str);

/**
 * Returns ``true`` if `b` is initialized.
 */
ZENOHC_API bool z_slice_is_initialized(const struct z_slice_t *b);

ZENOHC_API struct z_slice_t z_slice_loan(const struct z_owned_slice_t *b);

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
 * Returns the value associated with `key`, returning a gravestone value if:
 * - `key` is in gravestone state.
 */
ZENOHC_API struct z_slice_t z_slice_map_get(struct z_slice_map_t this_, struct z_slice_t key);

/**
 * Associates `value` to `key` in the map, aliasing them.
 *
 * Note that once `key` is aliased, reinserting at the same key may alias the previous instance, or the new instance of `key`.
 *
 * Returns 0 in case of success, -1 if one of the arguments were in gravestone state.
 */
ZENOHC_API
ZCError z_slice_map_insert_by_alias(struct z_slice_map_t this_,
                                    struct z_slice_t key,
                                    struct z_slice_t value);

/**
 * Associates `value` to `key` in the map, copying them to obtain ownership: `key` and `value` are not aliased past the function's return.
 *
 * Returns 0 in case of success, -1 if one of the arguments were in gravestone state.
 */
ZENOHC_API
ZCError z_slice_map_insert_by_copy(struct z_slice_map_t this_,
                                   struct z_slice_t key,
                                   struct z_slice_t value);

/**
 * Returns true if the map is empty, false otherwise.
 */
ZENOHC_API bool z_slice_map_is_empty(struct z_slice_map_t this_);

ZENOHC_API
void z_slice_map_iterate(const struct z_slice_map_t *this_,
                         z_slice_map_iter_body_t body,
                         void *context);

/**
 * Returns number of key-value pairs in the map.
 */
ZENOHC_API size_t z_slice_map_len(struct z_slice_map_t this_);

/**
 * Constructs a new empty map.
 */
ZENOHC_API void z_slice_map_new(struct z_owned_slice_map_t *this_);

/**
 * Constructs the gravestone value for `z_owned_slice_map_t`
 */
ZENOHC_API void z_slice_map_null(struct z_owned_slice_map_t *this_);

/**
 * Deprecated in favor of `z_slice_from_str`: Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
 *
 * `str == NULL` will cause this to return `z_slice_empty()`
 */
ZENOHC_API
struct z_slice_t z_slice_new(const char *str);

/**
 * Returns the gravestone value for `z_owned_slice_t`
 */
ZENOHC_API struct z_owned_slice_t z_slice_null(void);

/**
 * Constructs a `len` bytes long view starting at `start`.
 */
ZENOHC_API struct z_slice_t z_slice_wrap(const uint8_t *start, size_t len);

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

ZENOHC_API struct z_id_t z_timestamp_get_id(const struct z_timestamp_t *timestamp);

ZENOHC_API uint64_t z_timestamp_npt64_time(const struct z_timestamp_t *timestamp);

/**
 * Undeclare the key expression generated by a call to :c:func:`z_declare_keyexpr`.
 * The keyxpr is consumed.
 */
ZENOHC_API int8_t z_undeclare_keyexpr(struct z_session_t session, struct z_owned_keyexpr_t *kexpr);

/**
 * Undeclares the given :c:type:`z_owned_publisher_t`, droping it and invalidating it for double-drop safety.
 */
ZENOHC_API
ZCError z_undeclare_publisher(struct z_owned_publisher_t *publisher);

/**
 * Undeclares a `z_owned_queryable_t`, droping it and invalidating it for doube-drop safety.
 *
 * Parameters:
 *     qable: The :c:type:`z_owned_queryable_t` to undeclare.
 */
ZENOHC_API ZCError z_undeclare_queryable(struct z_owned_queryable_t *qable);

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
ZCError zc_config_from_file(const char *path,
                            struct z_owned_config_t *this_);

/**
 * Reads a configuration from a JSON-serialized string, such as '{mode:"client",connect:{endpoints:["tcp/127.0.0.1:7447"]}}'.
 *
 * Passing a null-ptr will result in a gravestone value (`z_check(x) == false`).
 */
ZENOHC_API
ZCError zc_config_from_str(const char *s,
                           struct z_owned_config_t *this_);

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
 * Constructs a :c:type:`z_keyexpr_t` by aliasing a string.
 */
ZENOHC_API
ZCError zc_keyexpr_from_slice(struct z_owned_keyexpr_t *this_,
                              const char *name,
                              size_t len);

/**
 * Constructs a :c:type:`z_owned_keyexpr_t` by aliasing a string.
 * The string is canonized in-place before being passed to keyexpr.
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 */
ZENOHC_API
ZCError zc_keyexpr_from_slice_autocanonize(struct z_owned_keyexpr_t *this_,
                                           char *name,
                                           size_t *len);

/**
 * Constructs a :c:type:`z_owned_eyexpr_t` by aliasing a string without checking any of `z_keyexpr_t`'s assertions:
 * - `name` MUST be valid UTF8.
 * - `name` MUST follow the Key Expression specification, ie:
 *   - MUST NOT contain ``//``, MUST NOT start nor end with ``/``, MUST NOT contain any of the characters ``?#$``.
 *   - any instance of ``**`` may only be lead or followed by ``/``.
 *   - the key expression must have canon form.
 *
 * It is a loaned key expression that aliases `name`.
 */
ZENOHC_API
void zc_keyexpr_from_slice_unchecked(struct z_owned_keyexpr_t *this_,
                                     const char *start,
                                     size_t len);

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
 * Returns `true` if `sample` is valid.
 *
 * Note that there exist no fallinle constructors for `zc_owned_sample_t`, so validity is always guaranteed
 * unless the value has been dropped already.
 */
ZENOHC_API
bool zc_sample_check(const struct zc_owned_sample_t *sample);

/**
 * Clone a sample in the cheapest way available.
 */
ZENOHC_API void zc_sample_clone(const struct z_sample_t *src, struct zc_owned_sample_t *dst);

/**
 * Destroy the sample.
 */
ZENOHC_API void zc_sample_drop(struct zc_owned_sample_t *sample);

/**
 * Borrow the sample, allowing calling its accessor methods.
 *
 * Calling this function using a dropped sample is undefined behaviour.
 */
ZENOHC_API struct z_sample_t zc_sample_loan(const struct zc_owned_sample_t *sample);

ZENOHC_API void zc_sample_null(struct zc_owned_sample_t *sample);

/**
 * Increments the session's reference count, returning a new owning handle.
 */
ZENOHC_API
int8_t zc_session_rcinc(struct z_owned_session_t *dst,
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
ZENOHC_API struct zcu_owned_closure_matching_status_t zcu_closure_matching_status_null(void);

ZENOHC_API enum zcu_locality_t zcu_locality_default(void);

/**
 * Register callback for notifying subscribers matching.
 */
ZENOHC_API
ZCError zcu_publisher_matching_listener_callback(struct z_publisher_t publisher,
                                                 struct zcu_owned_closure_matching_status_t *callback,
                                                 struct zcu_owned_matching_listener_t *this_);

ZENOHC_API enum zcu_reply_keyexpr_t zcu_reply_keyexpr_default(void);
