//
// Copyright (c) 2024 ZettaScale Technology
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
// clang-format off
#ifdef DOCS
#define ALIGN(n)
#define ZENOHC_API
#endif
/**
 * The locality of samples to be received by subscribers or targeted by publishers.
 */
typedef enum z_locality_t {
  /**
   * Any
   */
  Z_LOCALITY_ANY = 0,
  /**
  * @warning This API is deprecated. Please use `Z_LOCALITY_ANY`.
  */
  ZC_LOCALITY_ANY = 0,
  /**
   * Only from local sessions.
   */
  Z_LOCALITY_SESSION_LOCAL = 1,
  /**
  * @warning This API is deprecated. Please use `Z_LOCALITY_SESSION_LOCAL`.
  */
  ZC_LOCALITY_SESSION_LOCAL = 1,
  /**
   * Only from remote sessions.
   */
  Z_LOCALITY_REMOTE = 2,
  /**
  * @warning This API is deprecated. Please use `Z_LOCALITY_REMOTE`.
  */
  ZC_LOCALITY_REMOTE = 2,
} z_locality_t;
/**
* @warning This API is deprecated. Please use `z_locality_t`.
*/
typedef z_locality_t zc_locality_t;
typedef enum z_congestion_control_t {
  /**
   * Messages are not dropped in case of congestion.
   */
  Z_CONGESTION_CONTROL_BLOCK = 0,
  /**
   * Messages are dropped in case of congestion.
   */
  Z_CONGESTION_CONTROL_DROP = 1,
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * Messages except the first one are dropped in case of congestion.
   */
  Z_CONGESTION_CONTROL_BLOCK_FIRST = 2,
#endif
} z_congestion_control_t;
/**
 * The priority of zenoh messages.
 */
typedef enum z_priority_t {
  /**
   * Priority for ``RealTime`` messages.
   */
  Z_PRIORITY_REAL_TIME = 1,
  /**
   * Highest priority for ``Interactive`` messages.
   */
  Z_PRIORITY_INTERACTIVE_HIGH = 2,
  /**
   * Lowest priority for ``Interactive`` messages.
   */
  Z_PRIORITY_INTERACTIVE_LOW = 3,
  /**
   * Highest priority for ``Data`` messages.
   */
  Z_PRIORITY_DATA_HIGH = 4,
  /**
   * Default priority for ``Data`` messages.
   */
  Z_PRIORITY_DATA = 5,
  /**
   * Lowest priority for ``Data`` messages.
   */
  Z_PRIORITY_DATA_LOW = 6,
  /**
   * Priority for ``Background traffic`` messages.
   */
  Z_PRIORITY_BACKGROUND = 7,
} z_priority_t;
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief The publisher reliability.
 * @note Currently `reliability` does not trigger any data retransmission on the wire.
 * It is rather used as a marker on the wire and it may be used to select the best link available (e.g. TCP for reliable data and UDP for best effort data).
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef enum z_reliability_t {
  /**
   * Defines reliability as ``BEST_EFFORT``
   */
  Z_RELIABILITY_BEST_EFFORT = 0,
  /**
   * Defines reliability as ``RELIABLE``
   */
  Z_RELIABILITY_RELIABLE = 1,
} z_reliability_t;
#endif
/**
 * The Queryables that should be target of a `z_get()`.
 */
typedef enum z_query_target_t {
  /**
   * The nearest complete queryable if any else all matching queryables.
   */
  Z_QUERY_TARGET_BEST_MATCHING = 0,
  /**
   * All matching queryables.
   */
  Z_QUERY_TARGET_ALL = 1,
  /**
   * All complete queryables.
   */
  Z_QUERY_TARGET_ALL_COMPLETE = 2,
} z_query_target_t;
/**
 * Consolidation mode values.
 */
typedef enum z_consolidation_mode_t {
  /**
   * Let Zenoh decide the best consolidation mode depending on the query selector.
   * If the selector contains time range properties, consolidation mode `NONE` is used.
   * Otherwise the `LATEST` consolidation mode is used.
   */
  Z_CONSOLIDATION_MODE_AUTO = -1,
  /**
   *  No consolidation is applied. Replies may come in any order and any number.
   */
  Z_CONSOLIDATION_MODE_NONE = 0,
  /**
   * It guarantees that any reply for a given key expression will be monotonic in time
   * w.r.t. the previous received replies for the same key expression. I.e., for the same key expression multiple
   * replies may be received. It is guaranteed that two replies received at t1 and t2 will have timestamp
   * ts2 > ts1. It optimizes latency.
   */
  Z_CONSOLIDATION_MODE_MONOTONIC = 1,
  /**
   * It guarantees unicity of replies for the same key expression.
   * It optimizes bandwidth.
   */
  Z_CONSOLIDATION_MODE_LATEST = 2,
} z_consolidation_mode_t;
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Key expressions types to which Queryable should reply to.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef enum zc_reply_keyexpr_t {
  /**
   * Replies to any key expression queries.
   */
  ZC_REPLY_KEYEXPR_ANY = 0,
  /**
   * Replies only to queries with intersecting key expressions.
   */
  ZC_REPLY_KEYEXPR_MATCHING_QUERY = 1,
} zc_reply_keyexpr_t;
#endif
typedef enum z_whatami_t {
  Z_WHATAMI_ROUTER = 1,
  Z_WHATAMI_PEER = 2,
  Z_WHATAMI_CLIENT = 4,
} z_whatami_t;
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Intersection level of 2 key expressions.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef enum z_keyexpr_intersection_level_t {
  /**
   * 2 key expressions do not intersect.
   */
  Z_KEYEXPR_INTERSECTION_LEVEL_DISJOINT = 0,
  /**
   * 2 key expressions intersect, i.e. there exists at least one key expression that is included by both.
   */
  Z_KEYEXPR_INTERSECTION_LEVEL_INTERSECTS = 1,
  /**
   * First key expression is the superset of second one.
   */
  Z_KEYEXPR_INTERSECTION_LEVEL_INCLUDES = 2,
  /**
   * 2 key expressions are equal.
   */
  Z_KEYEXPR_INTERSECTION_LEVEL_EQUALS = 3,
} z_keyexpr_intersection_level_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Session's provider state.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
typedef enum z_shm_provider_state {
  /**
   * Provider is disabled by configuration.
   */
  Z_SHM_PROVIDER_STATE_DISABLED,
  /**
   * Provider is concurrently-initializing.
   */
  Z_SHM_PROVIDER_STATE_INITIALIZING,
  /**
   * Provider is ready.
   */
  Z_SHM_PROVIDER_STATE_READY,
  /**
   * Error initializing provider.
   */
  Z_SHM_PROVIDER_STATE_ERROR,
} z_shm_provider_state;
#endif
typedef enum z_sample_kind_t {
  /**
   * The Sample was issued by a ``put`` operation.
   */
  Z_SAMPLE_KIND_PUT = 0,
  /**
   * The Sample was issued by a ``delete`` operation.
   */
  Z_SAMPLE_KIND_DELETE = 1,
} z_sample_kind_t;
typedef enum z_what_t {
  Z_WHAT_ROUTER = 1,
  Z_WHAT_PEER = 2,
  Z_WHAT_CLIENT = 4,
  Z_WHAT_ROUTER_PEER = 3,
  Z_WHAT_ROUTER_CLIENT = 5,
  Z_WHAT_PEER_CLIENT = 6,
  Z_WHAT_ROUTER_PEER_CLIENT = 7,
} z_what_t;
/**
 * Severity level of Zenoh log message.
 */
typedef enum zc_log_severity_t {
  /**
   * The `trace` level.
   *
   * Designates very low priority, often extremely verbose, information.
   */
  ZC_LOG_SEVERITY_TRACE = 0,
  /**
   * The "debug" level.
   *
   * Designates lower priority information.
   */
  ZC_LOG_SEVERITY_DEBUG = 1,
  /**
   * The "info" level.
   *
   * Designates useful information.
   */
  ZC_LOG_SEVERITY_INFO = 2,
  /**
   * The "warn" level.
   *
   * Designates hazardous situations.
   */
  ZC_LOG_SEVERITY_WARN = 3,
  /**
   * The "error" level.
   *
   * Designates very serious errors.
   */
  ZC_LOG_SEVERITY_ERROR = 4,
} zc_log_severity_t;
#if defined(Z_FEATURE_UNSTABLE_API)
typedef enum ze_advanced_publisher_heartbeat_mode_t {
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * Disable heartbeat-based last sample miss detection.
   */
  ZE_ADVANCED_PUBLISHER_HEARTBEAT_MODE_NONE = 0,
#endif
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * Allow last sample miss detection through periodic heartbeat.
   * Periodically send the last published Sample's sequence number to allow last sample recovery.
   */
  ZE_ADVANCED_PUBLISHER_HEARTBEAT_MODE_PERIODIC = 1,
#endif
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * Allow last sample miss detection through sporadic heartbeat.
   * Each period, the last published Sample's sequence number is sent with `z_congestion_control_t::BLOCK`
   * but only if it changed since last period.
   */
  ZE_ADVANCED_PUBLISHER_HEARTBEAT_MODE_SPORADIC = 2,
#endif
} ze_advanced_publisher_heartbeat_mode_t;
#endif
typedef struct z_moved_precomputed_layout_t {
  struct z_owned_precomputed_layout_t _this;
} z_moved_precomputed_layout_t;
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
typedef struct z_moved_precomputed_layout_t z_moved_alloc_layout_t;
#endif
typedef int8_t z_result_t;
typedef struct z_moved_bytes_t {
  struct z_owned_bytes_t _this;
} z_moved_bytes_t;
typedef struct z_moved_shm_t {
  struct z_owned_shm_t _this;
} z_moved_shm_t;
typedef struct z_moved_shm_mut_t {
  struct z_owned_shm_mut_t _this;
} z_moved_shm_mut_t;
typedef struct z_moved_slice_t {
  struct z_owned_slice_t _this;
} z_moved_slice_t;
typedef struct z_moved_string_t {
  struct z_owned_string_t _this;
} z_moved_string_t;
/**
 * An iterator over slices of serialized data.
 */
typedef struct ALIGN(8) z_bytes_slice_iterator_t {
  uint8_t _0[24];
} z_bytes_slice_iterator_t;
typedef struct z_moved_bytes_writer_t {
  struct z_owned_bytes_writer_t _this;
} z_moved_bytes_writer_t;
typedef struct z_moved_cancellation_token_t {
  struct z_owned_cancellation_token_t _this;
} z_moved_cancellation_token_t;
typedef struct z_moved_chunk_alloc_result_t {
  struct z_owned_chunk_alloc_result_t _this;
} z_moved_chunk_alloc_result_t;
typedef struct z_moved_ptr_in_segment_t {
  struct z_owned_ptr_in_segment_t _this;
} z_moved_ptr_in_segment_t;
/**
 * Monotonic clock
 */
typedef struct z_clock_t {
  uint64_t t;
  const void *t_base;
} z_clock_t;
/**
 * Options passed to the `z_close()` function.
 */
typedef struct z_close_options_t {
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * The timeout for close operation in milliseconds. 0 means default close timeout which is 10 seconds.
   */
  uint32_t internal_timeout_ms;
#endif
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * An optional uninitialized concurrent close handle. If set, the close operation will be executed
   * concurrently in separate task, and this handle will be initialized to be used for controlling
   * it's execution.
   */
  struct zc_owned_concurrent_close_handle_t *internal_out_concurrent;
#endif
#if !defined(Z_FEATURE_UNSTABLE_API)
  uint8_t _dummy;
#endif
} z_close_options_t;
/**
 * @brief A hello message-processing closure.
 *
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
 */
typedef struct z_owned_closure_hello_t {
  void *_context;
  void (*_call)(struct z_loaned_hello_t *hello, void *context);
  void (*_drop)(void *context);
} z_owned_closure_hello_t;
/**
 * Moved closure.
 */
typedef struct z_moved_closure_hello_t {
  struct z_owned_closure_hello_t _this;
} z_moved_closure_hello_t;
/**
 * @brief A struct that indicates if there exist Subscribers matching the Publisher's key expression or Queryables matching Querier's key expression and target.
 */
typedef struct z_matching_status_t {
  /**
   * True if there exist matching Zenoh entities, false otherwise.
   */
  bool matching;
} z_matching_status_t;
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief A matching status-processing closure.
 *
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
 */
typedef struct z_owned_closure_matching_status_t {
  void *_context;
  void (*_call)(const struct z_matching_status_t *matching_status, void *context);
  void (*_drop)(void *context);
} z_owned_closure_matching_status_t;
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Moved closure.
 */
typedef struct z_moved_closure_matching_status_t {
  struct z_owned_closure_matching_status_t _this;
} z_moved_closure_matching_status_t;
/**
 * @brief A query-processing closure.
 *
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
 */
typedef struct z_owned_closure_query_t {
  void *_context;
  void (*_call)(struct z_loaned_query_t *query, void *context);
  void (*_drop)(void *context);
} z_owned_closure_query_t;
/**
 * Moved closure.
 */
typedef struct z_moved_closure_query_t {
  struct z_owned_closure_query_t _this;
} z_moved_closure_query_t;
/**
 * @brief A reply-processing closure.
 *
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
 */
typedef struct z_owned_closure_reply_t {
  void *_context;
  void (*_call)(struct z_loaned_reply_t *reply, void *context);
  void (*_drop)(void *context);
} z_owned_closure_reply_t;
/**
 * Moved closure.
 */
typedef struct z_moved_closure_reply_t {
  struct z_owned_closure_reply_t _this;
} z_moved_closure_reply_t;
/**
 * @brief A sample-processing closure.
 *
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
 */
typedef struct z_owned_closure_sample_t {
  void *_context;
  void (*_call)(struct z_loaned_sample_t *sample, void *context);
  void (*_drop)(void *context);
} z_owned_closure_sample_t;
/**
 * Moved closure.
 */
typedef struct z_moved_closure_sample_t {
  struct z_owned_closure_sample_t _this;
} z_moved_closure_sample_t;
/**
 * @brief A zenoh id-processing closure.
 *
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 */
typedef struct z_owned_closure_zid_t {
  void *_context;
  void (*_call)(const struct z_id_t *z_id, void *context);
  void (*_drop)(void *context);
} z_owned_closure_zid_t;
/**
 * @brief Moved closure.
 */
typedef struct z_moved_closure_zid_t {
  struct z_owned_closure_zid_t _this;
} z_moved_closure_zid_t;
typedef struct z_moved_condvar_t {
  struct z_owned_condvar_t _this;
} z_moved_condvar_t;
typedef struct z_moved_config_t {
  struct z_owned_config_t _this;
} z_moved_config_t;
/**
 * Options passed to the `z_declare_queryable()` function.
 */
typedef struct z_queryable_options_t {
  /**
   * The completeness of the Queryable.
   */
  bool complete;
  /**
   * Restricts the matching requests that will be received by this Queryable to the ones
   * that have the compatible allowed_destination.
   */
  enum z_locality_t allowed_origin;
} z_queryable_options_t;
/**
 * Options passed to the `z_declare_subscriber()` function.
 */
typedef struct z_subscriber_options_t {
  /**
   * Restricts the matching publications that will be received by this Subscriber to the ones
   * that have the compatible allowed_destination.
   */
  enum z_locality_t allowed_origin;
} z_subscriber_options_t;
typedef struct z_moved_encoding_t {
  struct z_owned_encoding_t _this;
} z_moved_encoding_t;
/**
 * Options passed to the `z_declare_publisher()` function.
 */
typedef struct z_publisher_options_t {
  /**
   * Default encoding for messages put by this publisher.
   */
  struct z_moved_encoding_t *encoding;
  /**
   * The congestion control to apply when routing messages from this publisher.
   */
  enum z_congestion_control_t congestion_control;
  /**
   * The priority of messages from this publisher.
   */
  enum z_priority_t priority;
  /**
   * If set to ``true``, this message will not be batched. This usually has a positive impact on latency but negative impact on throughput.
   */
  bool is_express;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The publisher reliability.
   */
  enum z_reliability_t reliability;
#endif
  /**
   * The allowed destination for this publisher.
   */
  enum z_locality_t allowed_destination;
} z_publisher_options_t;
/**
 * The replies consolidation strategy to apply on replies to a `z_get()`.
 */
typedef struct z_query_consolidation_t {
  enum z_consolidation_mode_t mode;
} z_query_consolidation_t;
/**
 * @brief Options passed to the `z_declare_querier()` function.
 */
typedef struct z_querier_options_t {
  /**
   * The Queryables that should be target of the querier queries.
   */
  enum z_query_target_t target;
  /**
   * The replies consolidation strategy to apply on replies to the querier queries.
   */
  struct z_query_consolidation_t consolidation;
  /**
   * The congestion control to apply when routing the querier queries.
   */
  enum z_congestion_control_t congestion_control;
  /**
   * If set to ``true``, the querier queries will not be batched. This usually has a positive impact on latency but negative impact on throughput.
   */
  bool is_express;
  /**
   * The allowed destination for the querier queries.
   */
  enum z_locality_t allowed_destination;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The accepted replies for the querier queries.
   */
  enum zc_reply_keyexpr_t accept_replies;
#endif
  /**
   * The priority of the querier queries.
   */
  enum z_priority_t priority;
  /**
   * The timeout for the querier queries in milliseconds. 0 means default query timeout from zenoh configuration.
   */
  uint64_t timeout_ms;
} z_querier_options_t;
/**
 * Options passed to the `z_delete()` function.
 */
typedef struct z_delete_options_t {
  /**
   * The congestion control to apply when routing this delete message.
   */
  enum z_congestion_control_t congestion_control;
  /**
   * The priority of the delete message.
   */
  enum z_priority_t priority;
  /**
   * If set to ``true``, this message will not be batched. This usually has a positive impact on latency but negative impact on throughput.
   */
  bool is_express;
  /**
   * The timestamp of this message.
   */
  struct z_timestamp_t *timestamp;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The delete operation reliability.
   */
  enum z_reliability_t reliability;
#endif
  /**
   * The allowed destination of this message.
   */
  enum z_locality_t allowed_destination;
} z_delete_options_t;
typedef struct z_moved_fifo_handler_query_t {
  struct z_owned_fifo_handler_query_t _this;
} z_moved_fifo_handler_query_t;
typedef struct z_moved_fifo_handler_reply_t {
  struct z_owned_fifo_handler_reply_t _this;
} z_moved_fifo_handler_reply_t;
typedef struct z_moved_fifo_handler_sample_t {
  struct z_owned_fifo_handler_sample_t _this;
} z_moved_fifo_handler_sample_t;
/**
 * Options passed to the `z_get()` function.
 */
typedef struct z_get_options_t {
  /**
   * The Queryables that should be target of the query.
   */
  enum z_query_target_t target;
  /**
   * The replies consolidation strategy to apply on replies to the query.
   */
  struct z_query_consolidation_t consolidation;
  /**
   * An optional payload to attach to the query.
   */
  struct z_moved_bytes_t *payload;
  /**
   * An optional encoding of the query payload and or attachment.
   */
  struct z_moved_encoding_t *encoding;
  /**
   * The congestion control to apply when routing the query.
   */
  enum z_congestion_control_t congestion_control;
  /**
   * If set to ``true``, this message will not be batched. This usually has a positive impact on latency but negative impact on throughput.
   */
  bool is_express;
  /**
   * The allowed destination for the query.
   */
  enum z_locality_t allowed_destination;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The accepted replies for the query.
   */
  enum zc_reply_keyexpr_t accept_replies;
#endif
  /**
   * The priority of the query.
   */
  enum z_priority_t priority;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The source info for the query.
   */
  const struct z_source_info_t *source_info;
#endif
  /**
   * An optional attachment to attach to the query.
   */
  struct z_moved_bytes_t *attachment;
  /**
   * The timeout for the query in milliseconds. 0 means default query timeout from zenoh configuration.
   */
  uint64_t timeout_ms;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * Cancellation token to interrupt the query.
   */
  struct z_moved_cancellation_token_t *cancellation_token;
#endif
} z_get_options_t;
typedef struct z_moved_hello_t {
  struct z_owned_hello_t _this;
} z_moved_hello_t;
typedef struct z_moved_keyexpr_t {
  struct z_owned_keyexpr_t _this;
} z_moved_keyexpr_t;
/**
 * @brief The options for `z_liveliness_declare_subscriber()`
 */
typedef struct z_liveliness_subscriber_options_t {
  /**
   * If true, subscriber will receive the state change notifications for liveliness tokens that were declared before its declaration.
   */
  bool history;
} z_liveliness_subscriber_options_t;
/**
 * @brief The options for `z_liveliness_declare_token()`.
 */
typedef struct z_liveliness_token_options_t {
  uint8_t _dummy;
} z_liveliness_token_options_t;
/**
 * @brief The options for `z_liveliness_get()`
 */
typedef struct z_liveliness_get_options_t {
  /**
   * The timeout for the liveliness query in milliseconds. 0 means default query timeout from zenoh configuration.
   */
  uint64_t timeout_ms;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * Cancellation token to interrupt the query.
   */
  struct z_moved_cancellation_token_t *cancellation_token;
#endif
} z_liveliness_get_options_t;
typedef struct z_moved_liveliness_token_t {
  struct z_owned_liveliness_token_t _this;
} z_moved_liveliness_token_t;
typedef struct z_moved_matching_listener_t {
  struct z_owned_matching_listener_t _this;
} z_moved_matching_listener_t;
typedef struct z_moved_memory_layout_t {
  struct z_owned_memory_layout_t _this;
} z_moved_memory_layout_t;
typedef struct z_moved_mutex_t {
  struct z_owned_mutex_t _this;
} z_moved_mutex_t;
/**
 * Options passed to the `z_open()` function.
 */
typedef struct z_open_options_t {
  uint8_t _dummy;
} z_open_options_t;
/**
 * Represents the set of options that can be applied to the delete operation by a previously declared publisher,
 * whenever issued via `z_publisher_delete()`.
 */
typedef struct z_publisher_delete_options_t {
  /**
   * The timestamp of this message.
   */
  const struct z_timestamp_t *timestamp;
} z_publisher_delete_options_t;
typedef struct z_moved_publisher_t {
  struct z_owned_publisher_t _this;
} z_moved_publisher_t;
/**
 * Options passed to the `z_publisher_put()` function.
 */
typedef struct z_publisher_put_options_t {
  /**
   *  The encoding of the data to publish.
   */
  struct z_moved_encoding_t *encoding;
  /**
   * The timestamp of the publication.
   */
  const struct z_timestamp_t *timestamp;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The source info for the publication.
   */
  const struct z_source_info_t *source_info;
#endif
  /**
   * The attachment to attach to the publication.
   */
  struct z_moved_bytes_t *attachment;
} z_publisher_put_options_t;
/**
 * Options passed to the `z_put()` function.
 */
typedef struct z_put_options_t {
  /**
   * The encoding of the message.
   */
  struct z_moved_encoding_t *encoding;
  /**
   * The congestion control to apply when routing this message.
   */
  enum z_congestion_control_t congestion_control;
  /**
   * The priority of this message.
   */
  enum z_priority_t priority;
  /**
   * If set to ``true``, this message will not be batched. This usually has a positive impact on latency but negative impact on throughput.
   */
  bool is_express;
  /**
   * The timestamp of this message.
   */
  struct z_timestamp_t *timestamp;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The put operation reliability.
   */
  enum z_reliability_t reliability;
#endif
  /**
   * The allowed destination of this message.
   */
  enum z_locality_t allowed_destination;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The source info for the message.
   */
  const struct z_source_info_t *source_info;
#endif
  /**
   * The attachment to this message.
   */
  struct z_moved_bytes_t *attachment;
} z_put_options_t;
typedef struct z_moved_querier_t {
  struct z_owned_querier_t _this;
} z_moved_querier_t;
/**
 * @brief Options passed to the `z_querier_get()` function.
 */
typedef struct z_querier_get_options_t {
  /**
   * An optional payload to attach to the query.
   */
  struct z_moved_bytes_t *payload;
  /**
   * An optional encoding of the query payload and or attachment.
   */
  struct z_moved_encoding_t *encoding;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The source info for the query.
   */
  const struct z_source_info_t *source_info;
#endif
  /**
   * An optional attachment to attach to the query.
   */
  struct z_moved_bytes_t *attachment;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * Cancellation token to interrupt the query.
   */
  struct z_moved_cancellation_token_t *cancellation_token;
#endif
} z_querier_get_options_t;
typedef struct z_moved_query_t {
  struct z_owned_query_t _this;
} z_moved_query_t;
/**
 * Represents the set of options that can be applied to a query reply,
 * sent via `z_query_reply()`.
 */
typedef struct z_query_reply_options_t {
  /**
   * The encoding of the reply payload.
   */
  struct z_moved_encoding_t *encoding;
  /**
   * The congestion control to apply when routing the reply.
   */
  enum z_congestion_control_t congestion_control;
  /**
   * The priority of the reply.
   */
  enum z_priority_t priority;
  /**
   * If set to ``true``, this reply will not be batched. This usually has a positive impact on latency but negative impact on throughput.
   */
  bool is_express;
  /**
   * The timestamp of the reply.
   */
  struct z_timestamp_t *timestamp;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The source info for the reply.
   */
  const struct z_source_info_t *source_info;
#endif
  /**
   * The attachment to this reply.
   */
  struct z_moved_bytes_t *attachment;
} z_query_reply_options_t;
/**
 * Represents the set of options that can be applied to a query delete reply,
 * sent via `z_query_reply_del()`.
 */
typedef struct z_query_reply_del_options_t {
  /**
   * The congestion control to apply when routing the reply.
   */
  enum z_congestion_control_t congestion_control;
  /**
   * The priority of the reply.
   */
  enum z_priority_t priority;
  /**
   * If set to ``true``, this reply will not be batched. This usually has a positive impact on latency but negative impact on throughput.
   */
  bool is_express;
  /**
   * The timestamp of the reply.
   */
  struct z_timestamp_t *timestamp;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
   *
   * The source info for the reply.
   */
  const struct z_source_info_t *source_info;
#endif
  /**
   * The attachment to this reply.
   */
  struct z_moved_bytes_t *attachment;
} z_query_reply_del_options_t;
/**
 * Represents the set of options that can be applied to a query reply error,
 * sent via `z_query_reply_err()`.
 */
typedef struct z_query_reply_err_options_t {
  /**
   * The encoding of the error payload.
   */
  struct z_moved_encoding_t *encoding;
} z_query_reply_err_options_t;
typedef struct z_moved_queryable_t {
  struct z_owned_queryable_t _this;
} z_moved_queryable_t;
typedef struct z_moved_reply_t {
  struct z_owned_reply_t _this;
} z_moved_reply_t;
typedef struct z_moved_reply_err_t {
  struct z_owned_reply_err_t _this;
} z_moved_reply_err_t;
typedef struct z_moved_ring_handler_query_t {
  struct z_owned_ring_handler_query_t _this;
} z_moved_ring_handler_query_t;
typedef struct z_moved_ring_handler_reply_t {
  struct z_owned_ring_handler_reply_t _this;
} z_moved_ring_handler_reply_t;
typedef struct z_moved_ring_handler_sample_t {
  struct z_owned_ring_handler_sample_t _this;
} z_moved_ring_handler_sample_t;
typedef struct z_moved_sample_t {
  struct z_owned_sample_t _this;
} z_moved_sample_t;
/**
 * Options to pass to `z_scout()`.
 */
typedef struct z_scout_options_t {
  /**
   * The maximum duration in ms the scouting can take.
   */
  uint64_t timeout_ms;
  /**
   * Type of entities to scout for.
   */
  enum z_what_t what;
} z_scout_options_t;
typedef struct z_moved_session_t {
  struct z_owned_session_t _this;
} z_moved_session_t;
typedef struct z_moved_shared_shm_provider_t {
  struct z_owned_shared_shm_provider_t _this;
} z_moved_shared_shm_provider_t;
typedef struct z_moved_shm_client_t {
  struct z_owned_shm_client_t _this;
} z_moved_shm_client_t;
typedef struct z_moved_shm_client_storage_t {
  struct z_owned_shm_client_storage_t _this;
} z_moved_shm_client_storage_t;
typedef struct z_moved_shm_provider_t {
  struct z_owned_shm_provider_t _this;
} z_moved_shm_provider_t;
typedef struct z_moved_string_array_t {
  struct z_owned_string_array_t _this;
} z_moved_string_array_t;
typedef struct z_moved_subscriber_t {
  struct z_owned_subscriber_t _this;
} z_moved_subscriber_t;
typedef struct z_moved_task_t {
  struct z_owned_task_t _this;
} z_moved_task_t;
typedef struct z_task_attr_t {
  size_t _0;
} z_task_attr_t;
/**
 * Returns system clock time point corresponding to the current time instant.
 */
typedef struct z_time_t {
  uint64_t t;
} z_time_t;
/**
 * @brief A log-processing closure.
 *
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
 */
typedef struct zc_owned_closure_log_t {
  void *_context;
  void (*_call)(enum zc_log_severity_t severity, const struct z_loaned_string_t *msg, void *context);
  void (*_drop)(void *context);
} zc_owned_closure_log_t;
/**
 * Moved closure.
 */
typedef struct zc_moved_closure_log_t {
  struct zc_owned_closure_log_t _this;
} zc_moved_closure_log_t;
typedef struct zc_moved_concurrent_close_handle_t {
  struct zc_owned_concurrent_close_handle_t _this;
} zc_moved_concurrent_close_handle_t;
typedef struct zc_internal_encoding_data_t {
  uint16_t id;
  const uint8_t *schema_ptr;
  size_t schema_len;
} zc_internal_encoding_data_t;
typedef struct zc_moved_shm_client_list_t {
  struct zc_owned_shm_client_list_t _this;
} zc_moved_shm_client_list_t;
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Setting for advanced publisher's cache. The cache allows advanced subscribers to recover history and/or lost samples.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_advanced_publisher_cache_options_t {
  /**
   * Must be set to ``true``, to enable the cache.
   */
  bool is_enabled;
  /**
   * Number of samples to keep for each resource.
   */
  size_t max_samples;
  /**
   * The congestion control to apply to replies.
   */
  enum z_congestion_control_t congestion_control;
  /**
   * The priority of replies.
   */
  enum z_priority_t priority;
  /**
   * If set to ``true``, this cache replies will not be batched. This usually has a positive impact on latency but negative impact on throughput.
   */
  bool is_express;
} ze_advanced_publisher_cache_options_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Represents the set of options that can be applied to the delete operation by a previously declared advanced publisher,
 * whenever issued via `ze_advanced_publisher_delete()`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_advanced_publisher_delete_options_t {
  /**
   * Base delete options.
   */
  struct z_publisher_delete_options_t delete_options;
} ze_advanced_publisher_delete_options_t;
#endif
typedef struct ze_moved_advanced_publisher_t {
  struct ze_owned_advanced_publisher_t _this;
} ze_moved_advanced_publisher_t;
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Settings for sample miss detection on Advanced Publisher.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_advanced_publisher_sample_miss_detection_options_t {
  /**
   * Must be set to ``true``, to enable sample miss detection by adding sequence numbers.
   */
  bool is_enabled;
  /**
   * Allow last sample miss detection through sporadic or periodic heartbeat.
   */
  enum ze_advanced_publisher_heartbeat_mode_t heartbeat_mode;
  /**
   * If heartbeat_mode is not NONE, the publisher will send heartbeats with the specified period, which
   * can be used by Advanced Subscribers for last sample(s) miss detection (if last sample miss detection with zero query period is enabled).
   */
  uint64_t heartbeat_period_ms;
} ze_advanced_publisher_sample_miss_detection_options_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Options passed to the `ze_declare_advanced_publisher()` function.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_advanced_publisher_options_t {
  /**
   * Base publisher options.
   */
  struct z_publisher_options_t publisher_options;
  /**
   * Publisher cache settings.
   */
  struct ze_advanced_publisher_cache_options_t cache;
  /**
   * Settings to allow matching Subscribers to detect lost samples and optionally ask for retransimission.
   *
   * Retransmission can only be done if cache is enabled.
   */
  struct ze_advanced_publisher_sample_miss_detection_options_t sample_miss_detection;
  /**
   * Allow this publisher to be detected through liveliness.
   */
  bool publisher_detection;
  /**
   * An optional key expression to be added to the liveliness token key expression.
   * It can be used to convey meta data.
   */
  const struct z_loaned_keyexpr_t *publisher_detection_metadata;
} ze_advanced_publisher_options_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Options passed to the `ze_advanced_publisher_put()` function.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_advanced_publisher_put_options_t {
  /**
   * Base put options.
   */
  struct z_publisher_put_options_t put_options;
} ze_advanced_publisher_put_options_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief A struct that represents missed samples.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_miss_t {
  /**
   * The source of missed samples.
   */
  struct z_entity_global_id_t source;
  /**
   * The number of missed samples.
   */
  uint32_t nb;
} ze_miss_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief A sample miss-processing closure.
 *
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_owned_closure_miss_t {
  void *_context;
  void (*_call)(const struct ze_miss_t *matching_status, void *context);
  void (*_drop)(void *context);
} ze_owned_closure_miss_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Moved closure.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_moved_closure_miss_t {
  struct ze_owned_closure_miss_t _this;
} ze_moved_closure_miss_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief An owned Zenoh sample miss listener. Missed samples can only be detected from advanced publishers, enabling sample miss detection.
 *
 * A listener that sends notification when the advanced subscriber misses a sample .
 * Dropping the corresponding subscriber, also drops the listener.
 */
typedef struct ALIGN(8) ze_owned_sample_miss_listener_t {
  uint8_t _0[24];
} ze_owned_sample_miss_listener_t;
typedef struct ze_moved_advanced_subscriber_t {
  struct ze_owned_advanced_subscriber_t _this;
} ze_moved_advanced_subscriber_t;
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Settings for retrievieng historical data for Advanced Subscriber.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_advanced_subscriber_history_options_t {
  /**
   * Must be set to ``true``, to enable the history data recovery.
   */
  bool is_enabled;
  /**
   * Enable detection of late joiner publishers and query for their historical data.
   * Late joiner detection can only be achieved for Publishers that enable publisher_detection.
   * History can only be retransmitted by Publishers that enable caching.
   */
  bool detect_late_publishers;
  /**
   * Number of samples to query for each resource. ``0`` corresponds to no limit on number of samples.
   */
  size_t max_samples;
  /**
   * Maximum age of samples to query. ``0`` corresponds to no limit on samples' age.
   */
  uint64_t max_age_ms;
} ze_advanced_subscriber_history_options_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Settings for detection of the last sample(s) miss by Advanced Subscriber.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_advanced_subscriber_last_sample_miss_detection_options_t {
  /**
   * Must be set to ``true``, to enable the last sample(s) miss detection.
   */
  bool is_enabled;
  /**
   * Period for queries for not yet received Samples.
   *
   * These queries allow to retrieve the last Sample(s) if the last Sample(s) is/are lost.
   * So it is useful for sporadic publications but useless for periodic publications
   * with a period smaller or equal to this period. If set to 0, the last sample(s) miss detection will be performed
   * based on publisher's heartbeat if the latter is enabled.
   */
  uint64_t periodic_queries_period_ms;
} ze_advanced_subscriber_last_sample_miss_detection_options_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Settings for recovering lost messages for Advanced Subscriber.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_advanced_subscriber_recovery_options_t {
  /**
   * Must be set to ``true``, to enable the lost sample recovery.
   */
  bool is_enabled;
  /**
   * Setting for detecting last sample(s) miss.
   * Note that it does not affect intermediate sample miss detection/retrieval (which is performed automatically as long as recovery is enabled).
   * If this option is disabled, subscriber will be unable to detect/request retransmission of missed sample until it receives a more recent one from the same publisher.
   */
  struct ze_advanced_subscriber_last_sample_miss_detection_options_t last_sample_miss_detection;
} ze_advanced_subscriber_recovery_options_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Options passed to the `ze_declare_advanced_subscriber()` function.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_advanced_subscriber_options_t {
  /**
   * Base subscriber options.
   */
  struct z_subscriber_options_t subscriber_options;
  /**
   * Settings for querying historical data. History can only be retransmitted by Publishers that enable caching.
   */
  struct ze_advanced_subscriber_history_options_t history;
  /**
   * Settings for retransmission of detected lost Samples. Retransmission of lost samples can only be done by Publishers that enable
   * caching and sample_miss_detection.
   */
  struct ze_advanced_subscriber_recovery_options_t recovery;
  /**
   * Timeout to be used for history and recovery queries.
   * Default value will be used if set to ``0``.
   */
  uint64_t query_timeout_ms;
  /**
   * Allow this subscriber to be detected through liveliness.
   */
  bool subscriber_detection;
  /**
   * An optional key expression to be added to the liveliness token key expression.
   * It can be used to convey meta data.
   */
  const struct z_loaned_keyexpr_t *subscriber_detection_metadata;
} ze_advanced_subscriber_options_t;
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Loaned closure.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_loaned_closure_miss_t {
  size_t _0;
  size_t _1;
  size_t _2;
} ze_loaned_closure_miss_t;
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_publisher.
 * @brief Options passed to the `ze_declare_publication_cache()` function.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_publication_cache_options_t {
  /**
   * The suffix used for queryable.
   */
  const struct z_loaned_keyexpr_t *queryable_suffix;
  /**
   * The restriction for the matching queries that will be receive by this publication cache.
   */
  enum z_locality_t queryable_origin;
  /**
   * The `complete` option for the queryable.
   */
  bool queryable_complete;
  /**
   * The the history size (i.e. maximum number of messages to store).
   */
  size_t history;
  /**
   * The limit number of cached resources.
   */
  size_t resources_limit;
} ze_publication_cache_options_t;
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_subscriber.
 * @brief A set of options that can be applied to a querying subscriber,
 * upon its declaration via `ze_declare_querying_subscriber()`.
 *
 */
#if defined(Z_FEATURE_UNSTABLE_API)
typedef struct ze_querying_subscriber_options_t {
  /**
   * The restriction for the matching publications that will be receive by this subscriber.
   */
  enum z_locality_t allowed_origin;
  /**
   * The selector to be used for queries.
   */
  const struct z_loaned_keyexpr_t *query_selector;
  /**
   * The target to be used for queries.
   */
  enum z_query_target_t query_target;
  /**
   * The consolidation mode to be used for queries.
   */
  struct z_query_consolidation_t query_consolidation;
#if defined(Z_FEATURE_UNSTABLE_API)
  /**
   * The accepted replies for queries.
   */
  enum zc_reply_keyexpr_t query_accept_replies;
#endif
  /**
   * The timeout to be used for queries.
   */
  uint64_t query_timeout_ms;
} ze_querying_subscriber_options_t;
#endif
typedef struct ze_moved_publication_cache_t {
  struct ze_owned_publication_cache_t _this;
} ze_moved_publication_cache_t;
typedef struct ze_moved_querying_subscriber_t {
  struct ze_owned_querying_subscriber_t _this;
} ze_moved_querying_subscriber_t;
typedef struct ze_moved_sample_miss_listener_t {
  struct ze_owned_sample_miss_listener_t _this;
} ze_moved_sample_miss_listener_t;
typedef struct ze_moved_serializer_t {
  struct ze_owned_serializer_t _this;
} ze_moved_serializer_t;
ZENOHC_API extern const unsigned int Z_ROUTER;
ZENOHC_API extern const unsigned int Z_PEER;
ZENOHC_API extern const unsigned int Z_CLIENT;
/**
 * @warning This API has been marked as deprecated, use `z_precomputed_layout_alloc` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_alloc_layout_alloc(struct z_buf_alloc_result_t *out_result,
                          const z_loaned_alloc_layout_t *layout);
#endif
/**
 * @warning This API has been marked as deprecated, use `z_precomputed_layout_alloc_gc` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_alloc_layout_alloc_gc(struct z_buf_alloc_result_t *out_result,
                             const z_loaned_alloc_layout_t *layout);
#endif
/**
 * @warning This API has been marked as deprecated, use `z_precomputed_layout_alloc_gc_defrag` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_alloc_layout_alloc_gc_defrag(struct z_buf_alloc_result_t *out_result,
                                    const z_loaned_alloc_layout_t *layout);
#endif
/**
 * @warning This API has been marked as deprecated, use `z_precomputed_layout_alloc_gc_defrag_blocking` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_alloc_layout_alloc_gc_defrag_blocking(struct z_buf_alloc_result_t *out_result,
                                             const z_loaned_alloc_layout_t *layout);
#endif
/**
 * @warning This API has been marked as deprecated, use `z_precomputed_layout_alloc_gc_defrag_dealloc` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_alloc_layout_alloc_gc_defrag_dealloc(struct z_buf_alloc_result_t *out_result,
                                            const z_loaned_alloc_layout_t *layout);
#endif
/**
 * @warning This API has been marked as deprecated, use `z_precomputed_layout_drop` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API void z_alloc_layout_drop(z_moved_alloc_layout_t *this_);
#endif
/**
 * @warning This API has been marked as deprecated, use `z_precomputed_layout_loan` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API const z_loaned_alloc_layout_t *z_alloc_layout_loan(const z_owned_alloc_layout_t *this_);
#endif
/**
 * @warning This API has been marked as deprecated, use `z_shm_provider_alloc_layout` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_alloc_layout_new(z_owned_alloc_layout_t *this_,
                              const struct z_loaned_shm_provider_t *provider,
                              size_t size);
#endif
/**
 * @warning This API has been marked as deprecated, use `z_precomputed_layout_threadsafe_alloc_gc_defrag_async` instead.
 * @brief Make allocation performing garbage collection and/or defragmentation in async manner. Will return Z_EINVAL
 * if used with non-threadsafe SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_alloc_layout_threadsafe_alloc_gc_defrag_async(struct z_buf_alloc_result_t *out_result,
                                                           const z_loaned_alloc_layout_t *layout,
                                                           struct zc_threadsafe_context_t result_context,
                                                           void (*result_callback)(void*,
                                                                                   struct z_buf_alloc_result_t*));
#endif
/**
 * @warning This API has been marked as deprecated, use `z_shm_provider_alloc_layout_aligned` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_alloc_layout_with_alignment_new(z_owned_alloc_layout_t *this_,
                                             const struct z_loaned_shm_provider_t *provider,
                                             size_t size,
                                             struct z_alloc_alignment_t alignment);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Converts data into a loaned SHM buffer.
 *
 * @param this_: Data to convert.
 * @param dst: An uninitialized memory location where to construct an SHM buffer.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_bytes_as_loaned_shm(const struct z_loaned_bytes_t *this_,
                                 const struct z_loaned_shm_t **dst);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Converts data into a mutably loaned SHM buffer.
 *
 * @param this_: Data to convert.
 * @param dst: An uninitialized memory location where to construct an SHM buffer.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_bytes_as_mut_loaned_shm(struct z_loaned_bytes_t *this_,
                                     struct z_loaned_shm_t **dst);
#endif
/**
 * Constructs an owned shallow copy of data in provided uninitialized memory location.
 */
ZENOHC_API void z_bytes_clone(struct z_owned_bytes_t *dst, const struct z_loaned_bytes_t *this_);
/**
 * Converts a data from buffer into `z_owned_bytes_t` by copying.
 * @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
 * @param data: A pointer to the buffer containing data.
 * @param len: Length of the buffer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_bytes_copy_from_buf(struct z_owned_bytes_t *this_,
                                 const uint8_t *data,
                                 size_t len);
/**
 * Converts a slice into `z_owned_bytes_t` by copying.
 */
ZENOHC_API
void z_bytes_copy_from_slice(struct z_owned_bytes_t *this_,
                             const struct z_loaned_slice_t *slice);
/**
 * Converts a null-terminated string into `z_owned_bytes_t` by copying.
 * @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
 * @param str: a pointer to the null-terminated string.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t z_bytes_copy_from_str(struct z_owned_bytes_t *this_, const char *str);
/**
 * Converts a string into `z_owned_bytes_t` by copying.
 */
ZENOHC_API
void z_bytes_copy_from_string(struct z_owned_bytes_t *this_,
                              const struct z_loaned_string_t *str);
/**
 * Drops `this_`, resetting it to gravestone value. If there are any shallow copies
 * created by `z_bytes_clone()`, they would still stay valid.
 */
ZENOHC_API void z_bytes_drop(struct z_moved_bytes_t *this_);
/**
 * Constructs an empty instance of `z_owned_bytes_t`.
 */
ZENOHC_API void z_bytes_empty(struct z_owned_bytes_t *this_);
/**
 * Converts buffer into `z_owned_bytes_t`.
 * @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
 * @param data: A pointer to the buffer containing data. `this_` will take ownership of the buffer.
 * @param len: Length of the buffer.
 * @param deleter: A thread-safe function, that will be called on `data` when `this_` is dropped. Can be `NULL` if `data` is located in static memory and does not require a drop.
 * @param context: An optional context to be passed to `deleter`.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_bytes_from_buf(struct z_owned_bytes_t *this_,
                            uint8_t *data,
                            size_t len,
                            void (*deleter)(void *data, void *context),
                            void *context);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Converts from an immutable SHM buffer consuming it.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_bytes_from_shm(struct z_owned_bytes_t *this_,
                            struct z_moved_shm_t *shm);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Converts a mutable SHM buffer consuming it.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_bytes_from_shm_mut(struct z_owned_bytes_t *this_,
                                struct z_moved_shm_mut_t *shm);
#endif
/**
 * Converts a slice into `z_owned_bytes_t`.
 * The slice is consumed upon function return.
 */
ZENOHC_API void z_bytes_from_slice(struct z_owned_bytes_t *this_, struct z_moved_slice_t *slice);
/**
 * Converts a statically allocated constant buffer into `z_owned_bytes_t`.
 * @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
 * @param data: A pointer to the statically allocated constant data.
 * @param len: A length of the buffer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_bytes_from_static_buf(struct z_owned_bytes_t *this_,
                                   uint8_t *data,
                                   size_t len);
/**
 * Converts a statically allocated constant null-terminated string into `z_owned_bytes_t` by aliasing.
 * @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
 * @param str: a pointer to the statically allocated constant string.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_bytes_from_static_str(struct z_owned_bytes_t *this_,
                                   const char *str);
/**
 * Converts a null-terminated string into `z_owned_bytes_t`.
 * @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
 * @param str: a pointer to the string. `this_` will take ownership of the string.
 * @param deleter: A thread-safe function, that will be called on `str` when `this_` is dropped. Can be `NULL` if `str` is located in static memory and does not require a drop.
 * @param context: An optional context to be passed to `deleter`.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_bytes_from_str(struct z_owned_bytes_t *this_,
                            char *str,
                            void (*deleter)(void *data, void *context),
                            void *context);
/**
 * Converts a string into `z_owned_bytes_t`.
 * The string is consumed upon function return.
 */
ZENOHC_API void z_bytes_from_string(struct z_owned_bytes_t *this_, struct z_moved_string_t *s);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Attempts to get a contiguous view to the underlying bytes.
 * This is only possible if data is not fragmented, otherwise the function will fail.
 * In case of fragmented data, consider using `z_bytes_get_slice_iterator()`.
 *
 * @param this_: An instance of Zenoh data.
 * @param view: An uninitialized memory location where a contiguous view on data will be constructed.
 * @return  ​0​ upon success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t z_bytes_get_contiguous_view(const struct z_loaned_bytes_t *this_,
                                       struct z_view_slice_t *view);
#endif
/**
 * Returns a reader for the data.
 *
 * The `data` should outlive the reader.
 */
ZENOHC_API struct z_bytes_reader_t z_bytes_get_reader(const struct z_loaned_bytes_t *data);
/**
 * Returns an iterator on raw bytes slices contained in the `z_loaned_bytes_t`.
 *
 * Zenoh may store data in non-contiguous regions of memory, this iterator
 * then allows to access raw data directly without any attempt of deserializing it.
 * Please note that no guarantee is provided on the internal memory layout.
 * The only provided guarantee is on the bytes order that is preserved.
 */
ZENOHC_API
struct z_bytes_slice_iterator_t z_bytes_get_slice_iterator(const struct z_loaned_bytes_t *this_);
/**
 * Returns ``true`` if `this_` is empty, ``false`` otherwise.
 */
ZENOHC_API bool z_bytes_is_empty(const struct z_loaned_bytes_t *this_);
/**
 * Returns total number of bytes in the payload.
 */
ZENOHC_API size_t z_bytes_len(const struct z_loaned_bytes_t *this_);
/**
 * Borrows data.
 */
ZENOHC_API const struct z_loaned_bytes_t *z_bytes_loan(const struct z_owned_bytes_t *this_);
/**
 * Muatably borrows data.
 */
ZENOHC_API struct z_loaned_bytes_t *z_bytes_loan_mut(struct z_owned_bytes_t *this_);
/**
 * Reads data into specified destination.
 *
 * @param this_: Data reader to read from.
 * @param dst: Buffer where the read data is written.
 * @param len: Maximum number of bytes to read.
 * @return number of bytes read. If return value is smaller than `len`, it means that  theend of the data was reached.
 */
ZENOHC_API
size_t z_bytes_reader_read(struct z_bytes_reader_t *this_,
                           uint8_t *dst,
                           size_t len);
/**
 * Gets the number of bytes that can still be read.
 */
ZENOHC_API size_t z_bytes_reader_remaining(const struct z_bytes_reader_t *this_);
/**
 * Sets the `reader` position indicator for the payload to the value pointed to by offset.
 * The new position is exactly `offset` bytes measured from the beginning of the payload if origin is `SEEK_SET`,
 * from the current reader position if origin is `SEEK_CUR`, and from the end of the payload if origin is `SEEK_END`.
 * @return ​0​ upon success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_bytes_reader_seek(struct z_bytes_reader_t *this_,
                               int64_t offset,
                               int origin);
/**
 * Gets the read position indicator.
 * @return read position indicator on success or -1L if failure occurs.
 */
ZENOHC_API int64_t z_bytes_reader_tell(struct z_bytes_reader_t *this_);
/**
 * Gets next slice.
 * @param this_: Slice iterator.
 * @param slice: An unitialized memory location where the view for the next slice will be constructed.
 * @return `false` if there are no more slices (in this case slice will stay unchanged), `true` otherwise.
 */
ZENOHC_API
bool z_bytes_slice_iterator_next(struct z_bytes_slice_iterator_t *this_,
                                 struct z_view_slice_t *slice);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Converts data into an owned SHM buffer by copying it's shared reference.
 *
 * @param this_: Data to convert.
 * @param dst: An uninitialized memory location where to construct an SHM buffer.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_bytes_to_owned_shm(const struct z_loaned_bytes_t *this_,
                                struct z_owned_shm_t *dst);
#endif
/**
 * Converts data into an owned slice.
 *
 * @param this_: Data to convert.
 * @param dst: An uninitialized memory location where to construct a slice.
 */
ZENOHC_API
z_result_t z_bytes_to_slice(const struct z_loaned_bytes_t *this_,
                            struct z_owned_slice_t *dst);
/**
 * Converts data into an owned non-null-terminated string.
 *
 * @param this_: Data to convert.
 * @param dst: An uninitialized memory location where to construct a string.
 */
ZENOHC_API
z_result_t z_bytes_to_string(const struct z_loaned_bytes_t *this_,
                             struct z_owned_string_t *dst);
/**
 * Appends bytes.
 * This allows to compose a serialized data out of multiple `z_owned_bytes_t` that may point to different memory regions.
 * Said in other terms, it allows to create a linear view on different memory regions without copy.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_bytes_writer_append(struct z_loaned_bytes_writer_t *this_,
                                 struct z_moved_bytes_t *bytes);
/**
 * Drops `this_`, resetting it to gravestone value.
 */
ZENOHC_API void z_bytes_writer_drop(struct z_moved_bytes_writer_t *this_);
/**
 * @brief Constructs a data writer with empty payload.
 * @param this_: An uninitialized memory location where writer is to be constructed.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t z_bytes_writer_empty(struct z_owned_bytes_writer_t *this_);
/**
 * @brief Drop writer and extract underlying `bytes` object it was writing to.
 * @param this_: A writer instance.
 * @param bytes: An uninitialized memory location where `bytes` object` will be written to.
 */
ZENOHC_API
void z_bytes_writer_finish(struct z_moved_bytes_writer_t *this_,
                           struct z_owned_bytes_t *bytes);
/**
 * Borrows writer.
 */
ZENOHC_API
const struct z_loaned_bytes_writer_t *z_bytes_writer_loan(const struct z_owned_bytes_writer_t *this_);
/**
 * Muatably borrows writer.
 */
ZENOHC_API
struct z_loaned_bytes_writer_t *z_bytes_writer_loan_mut(struct z_owned_bytes_writer_t *this_);
/**
 * Writes `len` bytes from `src` into underlying data.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_bytes_writer_write_all(struct z_loaned_bytes_writer_t *this_,
                                    const uint8_t *src,
                                    size_t len);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Interrupts all associated GET queries. If the query callback is being executed, the call blocks until execution of callback is finished.
 * In case of failure, some operations might not be cancelled.
 * Once cancelled, all newly added GET queries will cancel automatically.
 *
 * @return 0 in case of success, negative error code in case of failure.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t z_cancellation_token_cancel(struct z_loaned_cancellation_token_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Clones the cancellation token into provided uninitialized memory location.
 *
 * Cancelling token also cancels all of its clones.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void z_cancellation_token_clone(struct z_owned_cancellation_token_t *dst,
                                const struct z_loaned_cancellation_token_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Frees cancellation_token, and resets it to its gravestone state.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void z_cancellation_token_drop(struct z_moved_cancellation_token_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns ``true`` if cancellation token was cancelled (i .e. if `z_cancellation_token_cancel()` was called), ``false`` otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
bool z_cancellation_token_is_cancelled(const struct z_loaned_cancellation_token_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows cancellation token.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct z_loaned_cancellation_token_t *z_cancellation_token_loan(const struct z_owned_cancellation_token_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Mutably borrows cancellation token.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
struct z_loaned_cancellation_token_t *z_cancellation_token_loan_mut(struct z_owned_cancellation_token_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs a new cancellation token.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t z_cancellation_token_new(struct z_owned_cancellation_token_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Deletes Chunk Alloc Result.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_chunk_alloc_result_drop(struct z_moved_chunk_alloc_result_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new Chunk Alloc Result with Error value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_chunk_alloc_result_new_error(struct z_owned_chunk_alloc_result_t *this_,
                                    enum z_alloc_error_t alloc_error);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new Chunk Alloc Result with Ok value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_chunk_alloc_result_new_ok(struct z_owned_chunk_alloc_result_t *this_,
                                       struct z_allocated_chunk_t allocated_chunk);
#endif
/**
 * Get number of milliseconds passed since creation of `time`.
 */
ZENOHC_API uint64_t z_clock_elapsed_ms(const struct z_clock_t *time);
/**
 * Get number of seconds passed since creation of `time`.
 */
ZENOHC_API uint64_t z_clock_elapsed_s(const struct z_clock_t *time);
/**
 * Get number of microseconds passed since creation of `time`.
 */
ZENOHC_API uint64_t z_clock_elapsed_us(const struct z_clock_t *time);
/**
 * Returns monotonic clock time point corresponding to the current time instant.
 */
ZENOHC_API struct z_clock_t z_clock_now(void);
/**
 * Closes Zenoh session. This also drops all the closure callbacks remaining from not yet dropped or undeclared Zenoh entites (subscribers, queriers, etc).
 * After this operation, all calls for network operations for entites declared on this session will return a error.
 *
 * @return `0` in case of success, a negative value if an error occured while closing the session.
 */
ZENOHC_API
z_result_t z_close(struct z_loaned_session_t *session,
                   struct z_close_options_t *options);
/**
 * Constructs the default value for `z_close_options_t`.
 */
ZENOHC_API void z_close_options_default(struct z_close_options_t *this_);
/**
 * @brief Constructs closure.
 * A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 *
 * @param this_: uninitialized memory location where new closure will be constructed.
 * @param call: a closure body.
 * @param drop: an optional function to be called once on closure drop.
 * @param context: closure context.
 */
ZENOHC_API
void z_closure_hello(struct z_owned_closure_hello_t *this_,
                     void (*call)(struct z_loaned_hello_t *hello, void *context),
                     void (*drop)(void *context),
                     void *context);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_hello_call(const struct z_loaned_closure_hello_t *closure,
                          struct z_loaned_hello_t *hello);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_hello_drop(struct z_moved_closure_hello_t *this_);
/**
 * Borrows closure.
 */
ZENOHC_API
const struct z_loaned_closure_hello_t *z_closure_hello_loan(const struct z_owned_closure_hello_t *closure);
/**
 * Mutably norrows closure.
 */
ZENOHC_API
struct z_loaned_closure_hello_t *z_closure_hello_loan_mut(struct z_owned_closure_hello_t *closure);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 *
 * @brief Constructs closure.
 * @param this_: uninitialized memory location where new closure will be constructed.
 * @param call: a closure body.
 * @param drop: an optional function to be called once on closure drop.
 * @param context: closure context.
 */
ZENOHC_API
void z_closure_matching_status(struct z_owned_closure_matching_status_t *this_,
                               void (*call)(const struct z_matching_status_t *matching_status,
                                            void *context),
                               void (*drop)(void *context),
                               void *context);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_matching_status_call(const struct z_loaned_closure_matching_status_t *closure,
                                    const struct z_matching_status_t *mathing_status);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Drops the closure, resetting it to its gravestone state. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_matching_status_drop(struct z_moved_closure_matching_status_t *closure_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows closure.
 */
ZENOHC_API
const struct z_loaned_closure_matching_status_t *z_closure_matching_status_loan(const struct z_owned_closure_matching_status_t *closure);
/**
 * @brief Constructs closure.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 *
 * @param this_: uninitialized memory location where new closure will be constructed.
 * @param call: a closure body.
 * @param drop: an optional function to be called once on closure drop.
 * @param context: closure context.
 */
ZENOHC_API
void z_closure_query(struct z_owned_closure_query_t *this_,
                     void (*call)(struct z_loaned_query_t *query, void *context),
                     void (*drop)(void *context),
                     void *context);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_query_call(const struct z_loaned_closure_query_t *closure,
                          struct z_loaned_query_t *query);
/**
 * Drops the closure, resetting it to its gravestone state.
 */
ZENOHC_API void z_closure_query_drop(struct z_moved_closure_query_t *closure_);
/**
 * Borrows closure.
 */
ZENOHC_API
const struct z_loaned_closure_query_t *z_closure_query_loan(const struct z_owned_closure_query_t *closure);
/**
 * Mutably borrows closure.
 */
ZENOHC_API
struct z_loaned_closure_query_t *z_closure_query_loan_mut(struct z_owned_closure_query_t *closure);
/**
 * @brief Constructs closure.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 *
 * @param this_: uninitialized memory location where new closure will be constructed.
 * @param call: a closure body.
 * @param drop: an optional function to be called once on closure drop.
 * @param context: closure context.
 */
ZENOHC_API
void z_closure_reply(struct z_owned_closure_reply_t *this_,
                     void (*call)(struct z_loaned_reply_t *reply, void *context),
                     void (*drop)(void *context),
                     void *context);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_reply_call(const struct z_loaned_closure_reply_t *closure,
                          struct z_loaned_reply_t *reply);
/**
 * Drops the closure, resetting it to its gravestone state. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_reply_drop(struct z_moved_closure_reply_t *closure_);
/**
 * Borrows closure.
 */
ZENOHC_API
const struct z_loaned_closure_reply_t *z_closure_reply_loan(const struct z_owned_closure_reply_t *closure);
/**
 * Mutably borrows closure.
 */
ZENOHC_API
struct z_loaned_closure_reply_t *z_closure_reply_loan_mut(struct z_owned_closure_reply_t *closure);
/**
 * @brief Constructs closure.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 *
 * @param this_: uninitialized memory location where new closure will be constructed.
 * @param call: a closure body.
 * @param drop: an optional function to be called once on closure drop.
 * @param context: closure context.
 */
ZENOHC_API
void z_closure_sample(struct z_owned_closure_sample_t *this_,
                      void (*call)(struct z_loaned_sample_t *sample, void *context),
                      void (*drop)(void *context),
                      void *context);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_sample_call(const struct z_loaned_closure_sample_t *closure,
                           struct z_loaned_sample_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void z_closure_sample_drop(struct z_moved_closure_sample_t *closure_);
/**
 * Borrows closure.
 */
ZENOHC_API
const struct z_loaned_closure_sample_t *z_closure_sample_loan(const struct z_owned_closure_sample_t *closure);
/**
 * Mutably borrows closure.
 */
ZENOHC_API
struct z_loaned_closure_sample_t *z_closure_sample_loan_mut(struct z_owned_closure_sample_t *closure);
/**
 * @brief Constructs closure.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 *
 * @param this_: uninitialized memory location where new closure will be constructed.
 * @param call: a closure body.
 * @param drop: an optional function to be called once on closure drop.
 * @param context: closure context.
 */
ZENOHC_API
void z_closure_zid(struct z_owned_closure_zid_t *this_,
                   void (*call)(const struct z_id_t *z_id, void *context),
                   void (*drop)(void *context),
                   void *context);
/**
 * @brief Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void z_closure_zid_call(const struct z_loaned_closure_zid_t *closure,
                        const struct z_id_t *z_id);
/**
 * @brief Drops the closure, resetting it to its gravestone state. Droping an uninitialized (null) closure is a no-op.
 */
ZENOHC_API
void z_closure_zid_drop(struct z_moved_closure_zid_t *closure_);
/**
 * @brief Borrows closure.
 */
ZENOHC_API
const struct z_loaned_closure_zid_t *z_closure_zid_loan(const struct z_owned_closure_zid_t *closure);
/**
 * @brief Mutably borrows closure.
 */
ZENOHC_API
const struct z_loaned_closure_zid_t *z_closure_zid_loan_mut(const struct z_owned_closure_zid_t *closure);
/**
 * Drops conditional variable.
 */
ZENOHC_API void z_condvar_drop(struct z_moved_condvar_t *this_);
/**
 * Constructs conditional variable.
 */
ZENOHC_API void z_condvar_init(struct z_owned_condvar_t *this_);
/**
 * Borrows conditional variable.
 */
ZENOHC_API const struct z_loaned_condvar_t *z_condvar_loan(const struct z_owned_condvar_t *this_);
/**
 * Mutably borrows conditional variable.
 */
ZENOHC_API struct z_loaned_condvar_t *z_condvar_loan_mut(struct z_owned_condvar_t *this_);
/**
 * Wakes up one blocked thread waiting on this condiitonal variable.
 * @return 0 in case of success, negative error code in case of failure.
 */
ZENOHC_API z_result_t z_condvar_signal(const struct z_loaned_condvar_t *this_);
/**
 * Blocks the current thread until the conditional variable receives a notification.
 *
 * The function atomically unlocks the guard mutex `m` and blocks the current thread.
 * When the function returns the lock will have been re-aquired again.
 * Note: The function may be subject to spurious wakeups.
 */
ZENOHC_API
z_result_t z_condvar_wait(const struct z_loaned_condvar_t *this_,
                          struct z_loaned_mutex_t *m);
/**
 * Clones the config into provided uninitialized memory location.
 */
ZENOHC_API void z_config_clone(struct z_owned_config_t *dst, const struct z_loaned_config_t *this_);
/**
 * Constructs a new empty configuration.
 */
ZENOHC_API z_result_t z_config_default(struct z_owned_config_t *this_);
/**
 * Frees `config`, and resets it to its gravestone state.
 */
ZENOHC_API void z_config_drop(struct z_moved_config_t *this_);
/**
 * Borrows config.
 */
ZENOHC_API const struct z_loaned_config_t *z_config_loan(const struct z_owned_config_t *this_);
/**
 * Mutably borrows config.
 */
ZENOHC_API struct z_loaned_config_t *z_config_loan_mut(struct z_owned_config_t *this_);
/**
 * Declares a background queryable for a given keyexpr. The queryable callback will be be called
 * to proccess incoming queries until the corresponding session is closed or dropped.
 *
 * @param session: The zenoh session.
 * @param key_expr: The key expression the Queryable will reply to.
 * @param callback: The callback function that will be called each time a matching query is received. Its ownership is passed to queryable.
 * @param options: Options for the queryable.
 *
 * @return 0 in case of success, negative error code otherwise (in this case )
 */
ZENOHC_API
z_result_t z_declare_background_queryable(const struct z_loaned_session_t *session,
                                          const struct z_loaned_keyexpr_t *key_expr,
                                          struct z_moved_closure_query_t *callback,
                                          struct z_queryable_options_t *options);
/**
 * Constructs and declares a background subscriber. Subscriber callback will be called to process the messages,
 * until the corresponding session is closed or dropped.
 *
 * @param session: The zenoh session.
 * @param key_expr: The key expression to subscribe.
 * @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
 * @param options: The options to be passed to the subscriber declaration.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_declare_background_subscriber(const struct z_loaned_session_t *session,
                                           const struct z_loaned_keyexpr_t *key_expr,
                                           struct z_moved_closure_sample_t *callback,
                                           struct z_subscriber_options_t *options);
/**
 * Constructs and declares a key expression on the network. This reduces key key expression to a numerical id,
 * which allows to save the bandwitdth, when passing key expression between Zenoh entities.
 *
 * @param session: Session on which to declare key expression.
 * @param declared_key_expr: An uninitialized location in memory where key expression will be constructed.
 * @param key_expr: Key expression to declare on network.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_declare_keyexpr(const struct z_loaned_session_t *session,
                             struct z_owned_keyexpr_t *declared_key_expr,
                             const struct z_loaned_keyexpr_t *key_expr);
/**
 * Constructs and declares a publisher for the given key expression.
 *
 * Data can be put and deleted with this publisher with the help of the
 * `z_publisher_put()` and `z_publisher_delete()` functions.
 *
 * @param session: The Zenoh session.
 * @param publisher: An uninitialized location in memory where publisher will be constructed.
 * @param key_expr: The key expression to publish.
 * @param options: Additional options for the publisher.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_declare_publisher(const struct z_loaned_session_t *session,
                               struct z_owned_publisher_t *publisher,
                               const struct z_loaned_keyexpr_t *key_expr,
                               struct z_publisher_options_t *options);
/**
 * @brief Constructs and declares a querier on the given key expression.
 *
 * The queries can be send with the help of the `z_querier_get()` function.
 *
 * @param session: The Zenoh session.
 * @param querier: An uninitialized location in memory where querier will be constructed.
 * @param key_expr: The key expression to send queries on.
 * @param options: Additional options for the querier.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_declare_querier(const struct z_loaned_session_t *session,
                             struct z_owned_querier_t *querier,
                             const struct z_loaned_keyexpr_t *key_expr,
                             struct z_querier_options_t *options);
/**
 * Constructs a Queryable for the given key expression.
 *
 * @param session: A Zenoh session.
 * @param queryable: An uninitialized memory location where queryable will be constructed.
 * @param key_expr: The key expression the Queryable will reply to.
 * @param callback: The callback function that will be called each time a matching query is received. Its ownership is passed to queryable.
 * @param options: Options for the queryable.
 *
 * @return 0 in case of success, negative error code otherwise (in this case )
 */
ZENOHC_API
z_result_t z_declare_queryable(const struct z_loaned_session_t *session,
                               struct z_owned_queryable_t *queryable,
                               const struct z_loaned_keyexpr_t *key_expr,
                               struct z_moved_closure_query_t *callback,
                               struct z_queryable_options_t *options);
/**
 * Constructs and declares a subscriber for a given key expression. Dropping subscriber undeclares its callback.
 *
 * @param session: The zenoh session.
 * @param subscriber: An uninitialized location in memory, where subscriber will be constructed.
 * @param key_expr: The key expression to subscribe.
 * @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
 * @param options: The options to be passed to the subscriber declaration.
 *
 * @return 0 in case of success, negative error code otherwise (in this case subscriber will be in its gravestone state).
 */
ZENOHC_API
z_result_t z_declare_subscriber(const struct z_loaned_session_t *session,
                                struct z_owned_subscriber_t *subscriber,
                                const struct z_loaned_keyexpr_t *key_expr,
                                struct z_moved_closure_sample_t *callback,
                                struct z_subscriber_options_t *options);
/**
 * Sends request to delete data on specified key expression (used when working with <a href="https://zenoh.io/docs/manual/abstractions/#storage"> Zenoh storages </a>).
 *
 * @param session: The zenoh session.
 * @param key_expr: The key expression to delete.
 * @param options: The delete options.
 *
 * @return 0 in case of success, negative values in case of failure.
 */
ZENOHC_API
z_result_t z_delete(const struct z_loaned_session_t *session,
                    const struct z_loaned_keyexpr_t *key_expr,
                    struct z_delete_options_t *options);
/**
 * Constructs the default value for `z_delete_options_t`.
 */
ZENOHC_API void z_delete_options_default(struct z_delete_options_t *this_);
/**
 * A Concise Binary Object Representation (CBOR)-encoded data.
 *
 * Constant alias for string: `"application/cbor"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_cbor(void);
/**
 * A Common Data Representation (CDR)-encoded data.
 *
 * Constant alias for string: `"application/cdr"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_cdr(void);
/**
 * Constrained Application Protocol (CoAP) data intended for CoAP-to-HTTP and HTTP-to-CoAP proxies.
 *
 * Constant alias for string: `"application/coap-payload"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_coap_payload(void);
/**
 * A Java serialized object.
 *
 * Constant alias for string: `"application/java-serialized-object"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_java_serialized_object(void);
/**
 * JSON data intended to be consumed by an application.
 *
 * Constant alias for string: `"application/json"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_json(void);
/**
 * Defines a JSON document structure for expressing a sequence of operations to apply to a JSON document.
 *
 * Constant alias for string: `"application/json-patch+json"`.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_encoding_application_json_patch_json(void);
/**
 * A JSON text sequence consists of any number of JSON texts, all encoded in UTF-8.
 *
 * Constant alias for string: `"application/json-seq"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_json_seq(void);
/**
 * A JSONPath defines a string syntax for selecting and extracting JSON values from within a given JSON value.
 *
 * Constant alias for string: `"application/jsonpath"`.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_encoding_application_jsonpath(void);
/**
 * A JSON Web Token (JWT).
 *
 * Constant alias for string: `"application/jwt"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_jwt(void);
/**
 * An application-specific MPEG-4 encoded data, either audio or video.
 *
 * Constant alias for string: `"application/mp4"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_mp4(void);
/**
 * An application-specific stream of bytes.
 *
 * Constant alias for string: `"application/octet-stream"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_octet_stream(void);
/**
 * An [openmetrics](https://github.com/OpenObservability/OpenMetrics) data, common used by [Prometheus](https://prometheus.io/).
 *
 * Constant alias for string: `"application/openmetrics-text"`.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_encoding_application_openmetrics_text(void);
/**
 * An application-specific protobuf-encoded data.
 *
 * Constant alias for string: `"application/protobuf"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_protobuf(void);
/**
 * A Python object serialized using [pickle](https://docs.python.org/3/library/pickle.html).
 *
 * Constant alias for string: `"application/python-serialized-object"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_python_serialized_object(void);
/**
 * A SOAP 1.2 message serialized as XML 1.0.
 *
 * Constant alias for string: `"application/soap+xml"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_soap_xml(void);
/**
 * An application-specific SQL query.
 *
 * Constant alias for string: `"application/sql"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_sql(void);
/**
 * An encoded a list of tuples, each consisting of a name and a value.
 *
 * Constant alias for string: `"application/x-www-form-urlencoded"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_x_www_form_urlencoded(void);
/**
 * An XML file intended to be consumed by an application..
 *
 * Constant alias for string: `"application/xml"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_xml(void);
/**
 * YAML data intended to be consumed by an application.
 *
 * Constant alias for string: `"application/yaml"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_yaml(void);
/**
 * A YANG-encoded data commonly used by the Network Configuration Protocol (NETCONF).
 *
 * Constant alias for string: `"application/yang"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_application_yang(void);
/**
 * A MPEG-4 Advanced Audio Coding (AAC) media.
 *
 * Constant alias for string: `"audio/aac"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_audio_aac(void);
/**
 * A Free Lossless Audio Codec (FLAC) media.
 *
 * Constant alias for string: `"audio/flac"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_audio_flac(void);
/**
 * An audio codec defined in MPEG-1, MPEG-2, MPEG-4, or registered at the MP4 registration authority.
 *
 * Constant alias for string: `"audio/mp4"`.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_encoding_audio_mp4(void);
/**
 * An Ogg-encapsulated audio stream.
 *
 * Constant alias for string: `"audio/ogg"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_audio_ogg(void);
/**
 * A Vorbis-encoded audio stream.
 *
 * Constant alias for string: `"audio/vorbis"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_audio_vorbis(void);
/**
 * Constructs an owned copy of the encoding in provided uninitilized memory location.
 */
ZENOHC_API
void z_encoding_clone(struct z_owned_encoding_t *dst,
                      const struct z_loaned_encoding_t *this_);
/**
 * Frees the memory and resets the encoding it to its default value.
 */
ZENOHC_API void z_encoding_drop(struct z_moved_encoding_t *this_);
/**
 * Returns ``true`` if `this_` equals to `other`, ``false`` otherwise.
 */
ZENOHC_API
bool z_encoding_equals(const struct z_loaned_encoding_t *this_,
                       const struct z_loaned_encoding_t *other);
/**
 * Constructs a `z_owned_encoding_t` from a specified string.
 */
ZENOHC_API z_result_t z_encoding_from_str(struct z_owned_encoding_t *this_, const char *s);
/**
 * Constructs a `z_owned_encoding_t` from a specified substring.
 */
ZENOHC_API
z_result_t z_encoding_from_substr(struct z_owned_encoding_t *this_,
                                  const char *s,
                                  size_t len);
/**
 * A BitMap (BMP) image.
 *
 * Constant alias for string: `"image/bmp"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_image_bmp(void);
/**
 * A Graphics Interchange Format (GIF) image.
 *
 * Constant alias for string: `"image/gif"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_image_gif(void);
/**
 * A Joint Photographic Experts Group (JPEG) image.
 *
 * Constant alias for string: `"image/jpeg"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_image_jpeg(void);
/**
 * A Portable Network Graphics (PNG) image.
 *
 * Constant alias for string: `"image/png"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_image_png(void);
/**
 * A Web Portable (WebP) image.
 *
 *  Constant alias for string: `"image/webp"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_image_webp(void);
/**
 * Borrows encoding.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_encoding_loan(const struct z_owned_encoding_t *this_);
/**
 * Returns a loaned default `z_loaned_encoding_t`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_loan_default(void);
/**
 * Mutably borrows encoding.
 */
ZENOHC_API struct z_loaned_encoding_t *z_encoding_loan_mut(struct z_owned_encoding_t *this_);
/**
 * Set a schema to this encoding from a c string. Zenoh does not define what a schema is and its semantichs is left to the implementer.
 * E.g. a common schema for `text/plain` encoding is `utf-8`.
 */
ZENOHC_API
z_result_t z_encoding_set_schema_from_str(struct z_loaned_encoding_t *this_,
                                          const char *s);
/**
 * Set a schema to this encoding from a c substring. Zenoh does not define what a schema is and its semantichs is left to the implementer.
 * E.g. a common schema for `text/plain` encoding is `utf-8`.
 */
ZENOHC_API
z_result_t z_encoding_set_schema_from_substr(struct z_loaned_encoding_t *this_,
                                             const char *s,
                                             size_t len);
/**
 * A CSS file.
 *
 * Constant alias for string: `"text/css"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_text_css(void);
/**
 * A CSV file.
 *
 * Constant alias for string: `"text/csv"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_text_csv(void);
/**
 * An HTML file.
 *
 * Constant alias for string: `"text/html"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_text_html(void);
/**
 * A JavaScript file.
 *
 * Constant alias for string: `"text/javascript"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_text_javascript(void);
/**
 * JSON data intended to be human readable.
 *
 * Constant alias for string: `"text/json"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_text_json(void);
/**
 * JSON5 encoded data that are human readable.
 *
 * Constant alias for string: `"text/json5"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_text_json5(void);
/**
 * A MarkDown file.
 *
 * Constant alias for string: `"text/markdown"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_text_markdown(void);
/**
 * A textual file.
 *
 * Constant alias for string: `"text/plain"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_text_plain(void);
/**
 * An XML file that is human readable.
 *
 * Constant alias for string: `"text/xml"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_text_xml(void);
/**
 * YAML data intended to be human readable.
 *
 * Constant alias for string: `"text/yaml"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_text_yaml(void);
/**
 * Constructs an owned non-null-terminated string from encoding
 *
 * @param this_: Encoding.
 * @param out_str: Uninitialized memory location where a string to be constructed.
 */
ZENOHC_API
void z_encoding_to_string(const struct z_loaned_encoding_t *this_,
                          struct z_owned_string_t *out_str);
/**
 * A h261-encoded video stream.
 *
 * Constant alias for string: `"video/h261"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_video_h261(void);
/**
 * A h263-encoded video stream.
 *
 * Constant alias for string: `"video/h263"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_video_h263(void);
/**
 * A h264-encoded video stream.
 *
 * Constant alias for string: `"video/h264"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_video_h264(void);
/**
 * A h265-encoded video stream.
 *
 * Constant alias for string: `"video/h265"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_video_h265(void);
/**
 * A h266-encoded video stream.
 *
 * Constant alias for string: `"video/h266"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_video_h266(void);
/**
 * A video codec defined in MPEG-1, MPEG-2, MPEG-4, or registered at the MP4 registration authority.
 *
 * Constant alias for string: `"video/mp4"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_video_mp4(void);
/**
 * An Ogg-encapsulated video stream.
 *
 * Constant alias for string: `"video/ogg"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_video_ogg(void);
/**
 * An uncompressed, studio-quality video stream.
 *
 * Constant alias for string: `"video/raw"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_video_raw(void);
/**
 * A VP8-encoded video stream.
 *
 * Constant alias for string: `"video/vp8"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_video_vp8(void);
/**
 * A VP9-encoded video stream.
 *
 * Constant alias for string: `"video/vp9"`.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_video_vp9(void);
/**
 * Just some bytes.
 *
 * Constant alias for string: `"zenoh/bytes"`.
 *
 * This encoding supposes that the payload was created with `z_bytes_from_buf()`, `z_bytes_from_slice()` or
 * similar functions and its data can be accessed via `z_bytes_to_slice()`.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_encoding_zenoh_bytes(void);
/**
 * Zenoh serialized data.
 *
 * Constant alias for string: `"zenoh/serialized"`.
 *
 * This encoding supposes that the payload was created with serialization functions.
 * The `schema` field may contain the details of serialziation format.
 */
ZENOHC_API const struct z_loaned_encoding_t *z_encoding_zenoh_serialized(void);
/**
 * A UTF-8 string.
 *
 * Constant alias for string: `"zenoh/string"`.
 *
 * This encoding supposes that the payload was created with `z_bytes_from_str()`, `z_bytes_from_string()` or
 * similar functions and its data can be accessed via `z_bytes_to_string()`.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_encoding_zenoh_string(void);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the entity id of the entity global id.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
uint32_t z_entity_global_id_eid(const struct z_entity_global_id_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the zenoh id of entity global id.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
struct z_id_t z_entity_global_id_zid(const struct z_entity_global_id_t *this_);
#endif
/**
 * Constructs send and recieve ends of the fifo channel
 */
ZENOHC_API
void z_fifo_channel_query_new(struct z_owned_closure_query_t *callback,
                              struct z_owned_fifo_handler_query_t *handler,
                              size_t capacity);
/**
 * Constructs send and recieve ends of the fifo channel
 */
ZENOHC_API
void z_fifo_channel_reply_new(struct z_owned_closure_reply_t *callback,
                              struct z_owned_fifo_handler_reply_t *handler,
                              size_t capacity);
/**
 * Constructs send and recieve ends of the fifo channel
 */
ZENOHC_API
void z_fifo_channel_sample_new(struct z_owned_closure_sample_t *callback,
                               struct z_owned_fifo_handler_sample_t *handler,
                               size_t capacity);
/**
 * Drops the handler and resets it to a gravestone state.
 */
ZENOHC_API void z_fifo_handler_query_drop(struct z_moved_fifo_handler_query_t *this_);
/**
 * Borrows handler.
 */
ZENOHC_API
const struct z_loaned_fifo_handler_query_t *z_fifo_handler_query_loan(const struct z_owned_fifo_handler_query_t *this_);
/**
 * Returns query from the fifo buffer. If there are no more pending queries will block until next query is received, or until
 * the channel is dropped (normally when Queryable is dropped).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the query will be in the gravestone state),
 * `Z_CHANNEL_NODATA` if the channel is still alive, but its buffer is empty (the query will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_fifo_handler_query_recv(const struct z_loaned_fifo_handler_query_t *this_,
                                     struct z_owned_query_t *query);
/**
 * Returns query from the fifo buffer. If there are no more pending queries will return immediately (with query set to its gravestone state).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the query will be in the gravestone state),
 * `Z_CHANNEL_NODATA` if the channel is still alive, but its buffer is empty (the query will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_fifo_handler_query_try_recv(const struct z_loaned_fifo_handler_query_t *this_,
                                         struct z_owned_query_t *query);
/**
 * Drops the handler and resets it to a gravestone state.
 */
ZENOHC_API void z_fifo_handler_reply_drop(struct z_moved_fifo_handler_reply_t *this_);
/**
 * Borrows handler.
 */
ZENOHC_API
const struct z_loaned_fifo_handler_reply_t *z_fifo_handler_reply_loan(const struct z_owned_fifo_handler_reply_t *this_);
/**
 * Returns reply from the fifo buffer. If there are no more pending replies will block until next reply is received, or until
 * the channel is dropped (normally when all replies are received).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the reply will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_fifo_handler_reply_recv(const struct z_loaned_fifo_handler_reply_t *this_,
                                     struct z_owned_reply_t *reply);
/**
 * Returns reply from the fifo buffer. If there are no more pending replies will return immediately (with reply set to its gravestone state).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the reply will be in the gravestone state),
 * `Z_CHANNEL_NODATA` if the channel is still alive, but its buffer is empty (the reply will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_fifo_handler_reply_try_recv(const struct z_loaned_fifo_handler_reply_t *this_,
                                         struct z_owned_reply_t *reply);
/**
 * Drops the handler and resets it to a gravestone state.
 */
ZENOHC_API void z_fifo_handler_sample_drop(struct z_moved_fifo_handler_sample_t *this_);
/**
 * Borrows handler.
 */
ZENOHC_API
const struct z_loaned_fifo_handler_sample_t *z_fifo_handler_sample_loan(const struct z_owned_fifo_handler_sample_t *this_);
/**
 * Returns sample from the fifo buffer. If there are no more pending replies will block until next sample is received, or until
 * the channel is dropped (normally when there are no more samples to receive).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the sample will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_fifo_handler_sample_recv(const struct z_loaned_fifo_handler_sample_t *this_,
                                      struct z_owned_sample_t *sample);
/**
 * Returns sample from the fifo buffer.
 * If there are no more pending replies will return immediately (with sample set to its gravestone state).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the sample will be in the gravestone state),
 * `Z_CHANNEL_NODATA` if the channel is still alive, but its buffer is empty (the sample will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_fifo_handler_sample_try_recv(const struct z_loaned_fifo_handler_sample_t *this_,
                                          struct z_owned_sample_t *sample);
/**
 * Query data from the matching queryables in the system.
 * Replies are provided through a callback function.
 *
 * @param session: The zenoh session.
 * @param key_expr: The key expression matching resources to query.
 * @param parameters: The query's parameters null-terminated string, similar to a url's query segment.
 * @param callback: The callback function that will be called on reception of replies for this query. It will be automatically dropped once all replies are processed.
 * @param options: Additional options for the get. All owned fields will be consumed.
 *
 * @return 0 in case of success, a negative error value upon failure.
 */
ZENOHC_API
z_result_t z_get(const struct z_loaned_session_t *session,
                 const struct z_loaned_keyexpr_t *key_expr,
                 const char *parameters,
                 struct z_moved_closure_reply_t *callback,
                 struct z_get_options_t *options);
/**
 * Constructs default `z_get_options_t`
 */
ZENOHC_API void z_get_options_default(struct z_get_options_t *this_);
/**
 * Query data from the matching queryables in the system.
 * Replies are provided through a callback function.
 *
 * @param session: The zenoh session.
 * @param key_expr: The key expression matching resources to query.
 * @param parameters: The query's parameters string, similar to a url's query segment.
 * @param parameters_len: The parameters substring length.
 * @param callback: The callback function that will be called on reception of replies for this query. It will be automatically dropped once all replies are processed.
 * @param options: Additional options for the get. All owned fields will be consumed.
 *
 * @return 0 in case of success, a negative error value upon failure.
 */
ZENOHC_API
z_result_t z_get_with_parameters_substr(const struct z_loaned_session_t *session,
                                        const struct z_loaned_keyexpr_t *key_expr,
                                        const char *parameters,
                                        size_t parameters_len,
                                        struct z_moved_closure_reply_t *callback,
                                        struct z_get_options_t *options);
/**
 * Constructs an owned copy of hello message.
 */
ZENOHC_API void z_hello_clone(struct z_owned_hello_t *dst, const struct z_loaned_hello_t *this_);
/**
 * Frees memory and resets hello message to its gravestone state.
 */
ZENOHC_API void z_hello_drop(struct z_moved_hello_t *this_);
/**
 * Borrows hello message.
 */
ZENOHC_API const struct z_loaned_hello_t *z_hello_loan(const struct z_owned_hello_t *this_);
/**
 * Mutably borrows hello message.
 */
ZENOHC_API struct z_loaned_hello_t *z_hello_loan_mut(struct z_owned_hello_t *this_);
/**
 * Constructs an array of non-owned locators (in the form non-null-terminated strings) of Zenoh entity that sent hello message.
 *
 * The lifetime of locator strings is bound to `this_`.
 */
ZENOHC_API
void z_hello_locators(const struct z_loaned_hello_t *this_,
                      struct z_owned_string_array_t *locators_out);
/**
 * Takes ownership of the mutably borrowed hello
 */
ZENOHC_API void z_hello_take_from_loaned(struct z_owned_hello_t *dst, struct z_loaned_hello_t *src);
/**
 * Returns type of Zenoh entity that transmitted hello message.
 */
ZENOHC_API enum z_whatami_t z_hello_whatami(const struct z_loaned_hello_t *this_);
/**
 * @brief Returns id of Zenoh entity that transmitted hello message.
 */
ZENOHC_API struct z_id_t z_hello_zid(const struct z_loaned_hello_t *this_);
/**
 * @brief Formats the `z_id_t` into 16-digit hex string (LSB-first order)
 */
ZENOHC_API void z_id_to_string(const struct z_id_t *zid, struct z_owned_string_t *dst);
/**
 * @brief Fetches the Zenoh IDs of all connected peers.
 *
 * `callback` will be called once for each ID, is guaranteed to never be called concurrently,
 * and is guaranteed to be dropped before this function exits.
 *
 * Retuns 0 on success, negative values on failure
 */
ZENOHC_API
z_result_t z_info_peers_zid(const struct z_loaned_session_t *session,
                            struct z_moved_closure_zid_t *callback);
/**
 * @brief Fetches the Zenoh IDs of all connected routers.
 *
 * `callback` will be called once for each ID, is guaranteed to never be called concurrently,
 * and is guaranteed to be dropped before this function exits.
 *
 * Retuns 0 on success, negative values on failure.
 */
ZENOHC_API
z_result_t z_info_routers_zid(const struct z_loaned_session_t *session,
                              struct z_moved_closure_zid_t *callback);
/**
 * @brief Returns the session's Zenoh ID.
 *
 * Unless the `session` is invalid, that ID is guaranteed to be non-zero.
 * In other words, this function returning an array of 16 zeros means you failed
 * to pass it a valid session.
 */
ZENOHC_API struct z_id_t z_info_zid(const struct z_loaned_session_t *session);
/**
 * @warning This API has been marked as deprecated, use `z_internal_precomputed_layout_check` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_alloc_layout_check(const z_owned_alloc_layout_t *this_);
#endif
/**
 * @warning This API has been marked as deprecated, use `z_internal_precomputed_layout_null` instead.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_alloc_layout_null(z_owned_alloc_layout_t *this_);
#endif
/**
 * Returns ``true`` if `this_` is in a valid state, ``false`` if it is in a gravestone state.
 */
ZENOHC_API bool z_internal_bytes_check(const struct z_owned_bytes_t *this_);
/**
 * The gravestone value for `z_owned_bytes_t`.
 */
ZENOHC_API void z_internal_bytes_null(struct z_owned_bytes_t *this_);
/**
 * Returns ``true`` if `this_` is in a valid state, ``false`` if it is in a gravestone state.
 */
ZENOHC_API bool z_internal_bytes_writer_check(const struct z_owned_bytes_writer_t *this_);
/**
 * Constructs a writer in a gravestone state.
 */
ZENOHC_API void z_internal_bytes_writer_null(struct z_owned_bytes_writer_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns ``true`` if cancellation_token is valid, ``false`` if it is in a gravestone state.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
bool z_internal_cancellation_token_check(const struct z_owned_cancellation_token_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs cancellation token in its gravestone state.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void z_internal_cancellation_token_null(struct z_owned_cancellation_token_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @return ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_chunk_alloc_result_check(const struct z_owned_chunk_alloc_result_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs Chunk Alloc Result in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_chunk_alloc_result_null(struct z_owned_chunk_alloc_result_t *this_);
#endif
/**
 * Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API bool z_internal_closure_hello_check(const struct z_owned_closure_hello_t *this_);
/**
 * Constructs a closure in a gravestone state.
 */
ZENOHC_API void z_internal_closure_hello_null(struct z_owned_closure_hello_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API
bool z_internal_closure_matching_status_check(const struct z_owned_closure_matching_status_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs a null value of 'z_owned_closure_matching_status_t' type
 */
ZENOHC_API
void z_internal_closure_matching_status_null(struct z_owned_closure_matching_status_t *this_);
/**
 * Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API bool z_internal_closure_query_check(const struct z_owned_closure_query_t *this_);
/**
 * Constructs a closure in its gravestone state.
 */
ZENOHC_API void z_internal_closure_query_null(struct z_owned_closure_query_t *this_);
/**
 * Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API bool z_internal_closure_reply_check(const struct z_owned_closure_reply_t *this_);
/**
 * Constructs a closure int its gravestone state.
 */
ZENOHC_API void z_internal_closure_reply_null(struct z_owned_closure_reply_t *this_);
/**
 * Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API bool z_internal_closure_sample_check(const struct z_owned_closure_sample_t *this_);
/**
 * Constructs a closure in its gravestone state.
 */
ZENOHC_API void z_internal_closure_sample_null(struct z_owned_closure_sample_t *this_);
/**
 * @brief Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API bool z_internal_closure_zid_check(const struct z_owned_closure_zid_t *this_);
/**
 * @brief Constructs a null closure.
 */
ZENOHC_API void z_internal_closure_zid_null(struct z_owned_closure_zid_t *this_);
/**
 * Returns ``true`` if conditional variable is valid, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_condvar_check(const struct z_owned_condvar_t *this_);
/**
 * Constructs conditional variable in a gravestone state.
 */
ZENOHC_API void z_internal_condvar_null(struct z_owned_condvar_t *this_);
/**
 * Returns ``true`` if config is valid, ``false`` if it is in a gravestone state.
 */
ZENOHC_API bool z_internal_config_check(const struct z_owned_config_t *this_);
/**
 * Constructs config in its gravestone state.
 */
ZENOHC_API void z_internal_config_null(struct z_owned_config_t *this_);
/**
 * Returns the default congestion control value of zenoh push network messages, typically used for put operations.
 */
ZENOHC_API
enum z_congestion_control_t z_internal_congestion_control_default_push(void);
/**
 * Returns the default congestion control value of zenoh request network messages, typically used for get operations.
 */
ZENOHC_API
enum z_congestion_control_t z_internal_congestion_control_default_request(void);
/**
 * Returns the default congestion control value of zenoh response network messages, typically used for reply operations.
 */
ZENOHC_API
enum z_congestion_control_t z_internal_congestion_control_default_response(void);
/**
 * Returns ``true`` if encoding is in non-default state, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_encoding_check(const struct z_owned_encoding_t *this_);
/**
 * Constructs a default `z_owned_encoding_t`.
 */
ZENOHC_API void z_internal_encoding_null(struct z_owned_encoding_t *this_);
/**
 * Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API
bool z_internal_fifo_handler_query_check(const struct z_owned_fifo_handler_query_t *this_);
/**
 * Constructs a handler in gravestone state.
 */
ZENOHC_API void z_internal_fifo_handler_query_null(struct z_owned_fifo_handler_query_t *this_);
/**
 * Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API
bool z_internal_fifo_handler_reply_check(const struct z_owned_fifo_handler_reply_t *this_);
/**
 * Constructs a handler in gravestone state.
 */
ZENOHC_API void z_internal_fifo_handler_reply_null(struct z_owned_fifo_handler_reply_t *this_);
/**
 * Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API
bool z_internal_fifo_handler_sample_check(const struct z_owned_fifo_handler_sample_t *this_);
/**
 * Constructs a handler in gravestone state.
 */
ZENOHC_API void z_internal_fifo_handler_sample_null(struct z_owned_fifo_handler_sample_t *this_);
/**
 * Returns ``true`` if `hello message` is valid, ``false`` if it is in a gravestone state.
 */
ZENOHC_API bool z_internal_hello_check(const struct z_owned_hello_t *this_);
/**
 * Constructs hello message in a gravestone state.
 */
ZENOHC_API void z_internal_hello_null(struct z_owned_hello_t *this_);
/**
 * Returns ``true`` if `keyexpr` is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API bool z_internal_keyexpr_check(const struct z_owned_keyexpr_t *this_);
/**
 * Constructs an owned key expression in a gravestone state.
 */
ZENOHC_API void z_internal_keyexpr_null(struct z_owned_keyexpr_t *this_);
/**
 * @brief Returns ``true`` if liveliness token is valid, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_liveliness_token_check(const struct z_owned_liveliness_token_t *this_);
/**
 * @brief Constructs liveliness token in its gravestone state.
 */
ZENOHC_API void z_internal_liveliness_token_null(struct z_owned_liveliness_token_t *this_);
/**
 * @brief Checks the matching listener is for the gravestone state
 */
ZENOHC_API bool z_internal_matching_listener_check(const struct z_owned_matching_listener_t *this_);
/**
 * @brief Constructs an empty matching listener.
 */
ZENOHC_API void z_internal_matching_listener_null(struct z_owned_matching_listener_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_memory_layout_check(const struct z_owned_memory_layout_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs Memory Layout in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_memory_layout_null(struct z_owned_memory_layout_t *this_);
#endif
/**
 * Returns ``true`` if mutex is valid, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_mutex_check(const struct z_owned_mutex_t *this_);
/**
 * Constructs mutex in a gravestone state.
 */
ZENOHC_API void z_internal_mutex_null(struct z_owned_mutex_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_precomputed_layout_check(const struct z_owned_precomputed_layout_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs Alloc Layout in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_precomputed_layout_null(struct z_owned_precomputed_layout_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_ptr_in_segment_check(const struct z_owned_ptr_in_segment_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs data pointer in SHM Segment in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_ptr_in_segment_null(struct z_owned_ptr_in_segment_t *this_);
#endif
/**
 * Returns ``true`` if publisher is valid, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_publisher_check(const struct z_owned_publisher_t *this_);
/**
 * Constructs a publisher in a gravestone state.
 */
ZENOHC_API void z_internal_publisher_null(struct z_owned_publisher_t *this_);
/**
 * @brief Returns ``true`` if querier is valid, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_querier_check(const struct z_owned_querier_t *this_);
/**
 * @brief Constructs a querier in a gravestone state.
 */
ZENOHC_API void z_internal_querier_null(struct z_owned_querier_t *this_);
/**
 * Returns `false` if `this` is in a gravestone state, `true` otherwise.
 */
ZENOHC_API bool z_internal_query_check(const struct z_owned_query_t *query);
/**
 * Constructs query in its gravestone value.
 */
ZENOHC_API void z_internal_query_null(struct z_owned_query_t *this_);
/**
 * Returns ``true`` if queryable is valid, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_queryable_check(const struct z_owned_queryable_t *this_);
/**
 * Constructs a queryable in its gravestone value.
 */
ZENOHC_API void z_internal_queryable_null(struct z_owned_queryable_t *this_);
/**
 * Returns ``true`` if `reply` is valid, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_reply_check(const struct z_owned_reply_t *this_);
/**
 * Returns ``true`` if reply error is in non-default state, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_reply_err_check(const struct z_owned_reply_err_t *this_);
/**
 * Constructs an empty `z_owned_reply_err_t`.
 */
ZENOHC_API void z_internal_reply_err_null(struct z_owned_reply_err_t *this_);
/**
 * Constructs the reply in its gravestone state.
 */
ZENOHC_API void z_internal_reply_null(struct z_owned_reply_t *this_);
/**
 * Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API
bool z_internal_ring_handler_query_check(const struct z_owned_ring_handler_query_t *this_);
/**
 * Constructs a handler in gravestone state.
 */
ZENOHC_API void z_internal_ring_handler_query_null(struct z_owned_ring_handler_query_t *this_);
/**
 * Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API
bool z_internal_ring_handler_reply_check(const struct z_owned_ring_handler_reply_t *this_);
/**
 * Constructs a handler in gravestone state.
 */
ZENOHC_API void z_internal_ring_handler_reply_null(struct z_owned_ring_handler_reply_t *this_);
/**
 * Returns ``true`` if handler is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API
bool z_internal_ring_handler_sample_check(const struct z_owned_ring_handler_sample_t *this_);
/**
 * Constructs a handler in gravestone state.
 */
ZENOHC_API void z_internal_ring_handler_sample_null(struct z_owned_ring_handler_sample_t *this_);
/**
 * Returns ``true`` if sample is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API bool z_internal_sample_check(const struct z_owned_sample_t *this_);
/**
 * Constructs sample in its gravestone state.
 */
ZENOHC_API void z_internal_sample_null(struct z_owned_sample_t *this_);
/**
 * Returns ``true`` if `session` is valid, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_session_check(const struct z_owned_session_t *this_);
/**
 * Constructs a Zenoh session in its gravestone state.
 */
ZENOHC_API void z_internal_session_null(struct z_owned_session_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_shared_shm_provider_check(const struct z_owned_shared_shm_provider_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs Shared SHM Provider in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_shared_shm_provider_null(struct z_owned_shared_shm_provider_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @return ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_shm_check(const struct z_owned_shm_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @return Returns ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_shm_client_check(const struct z_owned_shm_client_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs SHM client in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_shm_client_null(struct z_owned_shm_client_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @return ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_shm_client_storage_check(const struct z_owned_shm_client_storage_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Constructs SHM Client Storage in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_shm_client_storage_null(struct z_owned_shm_client_storage_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @return ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_shm_mut_check(const struct z_owned_shm_mut_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs ZShmMut slice in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_shm_mut_null(struct z_owned_shm_mut_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs ZShm slice in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_shm_null(struct z_owned_shm_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool z_internal_shm_provider_check(const struct z_owned_shm_provider_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs SHM Provider in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_internal_shm_provider_null(struct z_owned_shm_provider_t *this_);
#endif
/**
 * @return ``true`` if slice is not empty, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_slice_check(const struct z_owned_slice_t *this_);
/**
 * Constructs an empty `z_owned_slice_t`.
 */
ZENOHC_API void z_internal_slice_null(struct z_owned_slice_t *this_);
/**
 * @return ``true`` if the string array is valid, ``false`` if it is in a gravestone state.
 */
ZENOHC_API bool z_internal_string_array_check(const struct z_owned_string_array_t *this_);
/**
 * Constructs string array in its gravestone state.
 */
ZENOHC_API void z_internal_string_array_null(struct z_owned_string_array_t *this_);
/**
 * @return ``true`` if `this_` is a valid string, ``false`` if it is in gravestone state.
 */
ZENOHC_API bool z_internal_string_check(const struct z_owned_string_t *this_);
/**
 * Constructs owned string in a gravestone state.
 */
ZENOHC_API void z_internal_string_null(struct z_owned_string_t *this_);
/**
 * Returns ``true`` if subscriber is valid, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_subscriber_check(const struct z_owned_subscriber_t *this_);
/**
 * Constructs a subscriber in a gravestone state.
 */
ZENOHC_API void z_internal_subscriber_null(struct z_owned_subscriber_t *this_);
/**
 * Returns ``true`` if task is valid, ``false`` otherwise.
 */
ZENOHC_API bool z_internal_task_check(const struct z_owned_task_t *this_);
/**
 * Constructs task in a gravestone state.
 */
ZENOHC_API void z_internal_task_null(struct z_owned_task_t *this_);
/**
 * Constructs a non-owned non-null-terminated string from key expression.
 */
ZENOHC_API
void z_keyexpr_as_view_string(const struct z_loaned_keyexpr_t *this_,
                              struct z_view_string_t *out_string);
/**
 * Canonizes the passed string in place, possibly shortening it by modifying `len`.
 *
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 *
 * @return 0 upon success, negative error values upon failure (if the passed string was an invalid
 * key expression for reasons other than a non-canon form).
 */
ZENOHC_API
z_result_t z_keyexpr_canonize(char *start,
                              size_t *len);
/**
 * Canonizes the passed string in place, possibly shortening it by placing a new null-terminator.
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 *
 * @return 0 upon success, negative error values upon failure (if the passed string was an invalid
 * key expression for reasons other than a non-canon form).
 */
ZENOHC_API
z_result_t z_keyexpr_canonize_null_terminated(char *start);
/**
 * Constructs a copy of the key expression.
 */
ZENOHC_API
void z_keyexpr_clone(struct z_owned_keyexpr_t *dst,
                     const struct z_loaned_keyexpr_t *this_);
/**
 * Constructs key expression by concatenation of key expression in `left` with a string in `right`.
 * Returns 0 in case of success, negative error code otherwise.
 *
 * You should probably prefer `z_keyexpr_join` as Zenoh may then take advantage of the hierachical separation it inserts.
 * To avoid odd behaviors, concatenating a key expression starting with `*` to one ending with `*` is forbidden by this operation,
 * as this would extremely likely cause bugs.
 */
ZENOHC_API
z_result_t z_keyexpr_concat(struct z_owned_keyexpr_t *this_,
                            const struct z_loaned_keyexpr_t *left,
                            const char *right_start,
                            size_t right_len);
/**
 * Frees key expression and resets it to its gravestone state.
 */
ZENOHC_API void z_keyexpr_drop(struct z_moved_keyexpr_t *this_);
/**
 * Returns ``true`` if both ``left`` and ``right`` are equal, ``false`` otherwise.
 */
ZENOHC_API
bool z_keyexpr_equals(const struct z_loaned_keyexpr_t *left,
                      const struct z_loaned_keyexpr_t *right);
/**
 * Constructs a `z_owned_keyexpr_t` from a string, copying the passed string.
 * @return 0 in case of success, negative error code in case of failure (for example if `expr` is not a valid key expression or if it is
 * not in canon form.
 */
ZENOHC_API
z_result_t z_keyexpr_from_str(struct z_owned_keyexpr_t *this_,
                              const char *expr);
/**
 * Constructs `z_owned_keyexpr_t` from a string, copying the passed string. The copied string is canonized.
 * @return 0 in case of success, negative error code in case of failure (for example if expr is not a valid key expression
 * even despite canonization).
 */
ZENOHC_API
z_result_t z_keyexpr_from_str_autocanonize(struct z_owned_keyexpr_t *this_,
                                           const char *expr);
/**
 * Constructs a `z_owned_keyexpr_t` by copying a substring.
 *
 * @param this_: An uninitialized location in memory where key expression will be constructed.
 * @param expr: A buffer with length >= `len`.
 * @param len: Number of characters from `expr` to consider.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_keyexpr_from_substr(struct z_owned_keyexpr_t *this_,
                                 const char *expr,
                                 size_t len);
/**
 * Constructs a `z_keyexpr_t` by copying a substring.
 *
 * @param this_: An uninitialized location in memory where key expression will be constructed.
 * @param start: A buffer of with length >= `len`.
 * @param len: Number of characters from `expr` to consider. Will be modified to be equal to canonized key expression length.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_keyexpr_from_substr_autocanonize(struct z_owned_keyexpr_t *this_,
                                              const char *start,
                                              size_t *len);
/**
 * Returns ``true`` if ``left`` includes ``right``, i.e. the set defined by ``left`` contains every key belonging to the set
 * defined by ``right``, ``false`` otherwise.
 */
ZENOHC_API
bool z_keyexpr_includes(const struct z_loaned_keyexpr_t *left,
                        const struct z_loaned_keyexpr_t *right);
/**
 * Returns ``true`` if the keyexprs intersect, i.e. there exists at least one key which is contained in both of the
 * sets defined by ``left`` and ``right``, ``false`` otherwise.
 */
ZENOHC_API
bool z_keyexpr_intersects(const struct z_loaned_keyexpr_t *left,
                          const struct z_loaned_keyexpr_t *right);
/**
 * Returns 0 if the passed string is a valid (and canon) key expression.
 * Otherwise returns negative error value.
 */
ZENOHC_API z_result_t z_keyexpr_is_canon(const char *start, size_t len);
/**
 * Constructs key expression by performing path-joining (automatically inserting '/' in-between) of `left` with `right`.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_keyexpr_join(struct z_owned_keyexpr_t *this_,
                          const struct z_loaned_keyexpr_t *left,
                          const struct z_loaned_keyexpr_t *right);
/**
 * Borrows `z_owned_keyexpr_t`.
 */
ZENOHC_API const struct z_loaned_keyexpr_t *z_keyexpr_loan(const struct z_owned_keyexpr_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the relation between `left` and `right` from `left`'s point of view.
 *
 * @note This is slower than `z_keyexpr_intersects` and `keyexpr_includes`, so you should favor these methods for most applications.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
enum z_keyexpr_intersection_level_t z_keyexpr_relation_to(const struct z_loaned_keyexpr_t *left,
                                                          const struct z_loaned_keyexpr_t *right);
#endif
/**
 * @brief Declares a background subscriber on liveliness tokens that intersect `key_expr`. Subscriber callback will be called to process the messages,
 * until the corresponding session is closed or dropped.
 * @param session: The Zenoh session.
 * @param key_expr: The key expression to subscribe to.
 * @param callback: The callback function that will be called each time a liveliness token status is changed.
 * @param options: The options to be passed to the liveliness subscriber declaration.
 *
 * @return 0 in case of success, negative error values otherwise.
 */
ZENOHC_API
z_result_t z_liveliness_declare_background_subscriber(const struct z_loaned_session_t *session,
                                                      const struct z_loaned_keyexpr_t *key_expr,
                                                      struct z_moved_closure_sample_t *callback,
                                                      struct z_liveliness_subscriber_options_t *options);
/**
 * @brief Declares a subscriber on liveliness tokens that intersect `key_expr`.
 *
 * @param session: A Zenoh session.
 * @param subscriber: An uninitialized memory location where subscriber will be constructed.
 * @param key_expr: The key expression to subscribe to.
 * @param callback: The callback function that will be called each time a liveliness token status is changed.
 * @param options: The options to be passed to the liveliness subscriber declaration.
 *
 * @return 0 in case of success, negative error values otherwise.
 */
ZENOHC_API
z_result_t z_liveliness_declare_subscriber(const struct z_loaned_session_t *session,
                                           struct z_owned_subscriber_t *subscriber,
                                           const struct z_loaned_keyexpr_t *key_expr,
                                           struct z_moved_closure_sample_t *callback,
                                           struct z_liveliness_subscriber_options_t *options);
/**
 * @brief Constructs and declares a liveliness token on the network.
 *
 * Liveliness token subscribers on an intersecting key expression will receive a PUT sample when connectivity
 * is achieved, and a DELETE sample if it's lost.
 *
 * @param session: A Zenos session to declare the liveliness token.
 * @param token: An uninitialized memory location where liveliness token will be constructed.
 * @param key_expr: A keyexpr to declare a liveliess token for.
 * @param _options: Liveliness token declaration properties.
 */
ZENOHC_API
z_result_t z_liveliness_declare_token(const struct z_loaned_session_t *session,
                                      struct z_owned_liveliness_token_t *token,
                                      const struct z_loaned_keyexpr_t *key_expr,
                                      const struct z_liveliness_token_options_t *_options);
/**
 * @brief Queries liveliness tokens currently on the network with a key expression intersecting with `key_expr`.
 *
 * @param session: The Zenoh session.
 * @param key_expr: The key expression to query liveliness tokens for.
 * @param callback: The callback function that will be called for each received reply.
 * @param options: Additional options for the liveliness get operation.
 */
ZENOHC_API
z_result_t z_liveliness_get(const struct z_loaned_session_t *session,
                            const struct z_loaned_keyexpr_t *key_expr,
                            struct z_moved_closure_reply_t *callback,
                            struct z_liveliness_get_options_t *options);
/**
 * @brief Constructs default value `z_liveliness_get_options_t`.
 */
ZENOHC_API void z_liveliness_get_options_default(struct z_liveliness_get_options_t *this_);
/**
 * @brief Constucts default value for `z_liveliness_declare_subscriber_options_t`.
 */
ZENOHC_API
void z_liveliness_subscriber_options_default(struct z_liveliness_subscriber_options_t *this_);
/**
 * @brief Undeclares liveliness token, frees memory and resets it to a gravestone state.
 */
ZENOHC_API void z_liveliness_token_drop(struct z_moved_liveliness_token_t *this_);
/**
 * @brief Borrows token.
 */
ZENOHC_API
const struct z_loaned_liveliness_token_t *z_liveliness_token_loan(const struct z_owned_liveliness_token_t *this_);
/**
 * @brief Constructs default value for `z_liveliness_token_options_t`.
 */
ZENOHC_API void z_liveliness_token_options_default(struct z_liveliness_token_options_t *this_);
/**
 * @brief Destroys a liveliness token, notifying subscribers of its destruction.
 */
ZENOHC_API z_result_t z_liveliness_undeclare_token(struct z_moved_liveliness_token_t *this_);
/**
 * @brief Returns default value of `z_locality_t`
 */
ZENOHC_API enum z_locality_t z_locality_default(void);
/**
 * @brief Undeclares the given matching listener, droping and invalidating it.
 */
ZENOHC_API void z_matching_listener_drop(struct z_moved_matching_listener_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Deletes Memory Layout.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_memory_layout_drop(struct z_moved_memory_layout_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Extract data from Memory Layout.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_memory_layout_get_data(const struct z_loaned_memory_layout_t *this_,
                              size_t *out_size,
                              struct z_alloc_alignment_t *out_alignment);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows Memory Layout.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const struct z_loaned_memory_layout_t *z_memory_layout_loan(const struct z_owned_memory_layout_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new Memory Layout.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_memory_layout_new(struct z_owned_memory_layout_t *this_,
                               size_t size,
                               struct z_alloc_alignment_t alignment);
#endif
/**
 * Drops mutex and resets it to its gravestone state.
 */
ZENOHC_API void z_mutex_drop(struct z_moved_mutex_t *this_);
/**
 * Constructs a mutex.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t z_mutex_init(struct z_owned_mutex_t *this_);
/**
 * Mutably borrows mutex.
 */
ZENOHC_API struct z_loaned_mutex_t *z_mutex_loan_mut(struct z_owned_mutex_t *this_);
/**
 * Locks mutex. If mutex is already locked, blocks the thread until it aquires the lock.
 * @return 0 in case of success, negative error code in case of failure.
 */
ZENOHC_API z_result_t z_mutex_lock(struct z_loaned_mutex_t *this_);
/**
 * Tries to lock mutex. If mutex is already locked, return immediately.
 * @return 0 in case of success, negative value if failed to aquire the lock.
 */
ZENOHC_API z_result_t z_mutex_try_lock(struct z_loaned_mutex_t *this_);
/**
 * Unlocks previously locked mutex. If mutex was not locked by the current thread, the behaviour is undefined.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_mutex_unlock(struct z_loaned_mutex_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Each session's runtime may create its own provider to manage internal optimizations.
 * This method exposes that provider so it can also be accessed at the application level.
 *
 * Note that the provider may not be immediately available or may be disabled via configuration.
 * Provider initialization is concurrent and triggered by access events (both transport-internal and through this API).
 *
 * To use this provider, both *shared_memory* and *transport_optimization* config sections must be enabled.
 *
 * @param out_provider: A [`z_owned_shared_shm_provider_t`](z_owned_shared_shm_provider_t) object that will be
 * initialized from Session's provider if it exists. Initialized only if the returned value is `Z_OK`.
 * @param out_state: A [`z_shm_provider_state`](z_shm_provider_state) that indicates the status of the provider.
 * Always initialized by this function.
 * @return 0 in case if provider is avalable, negative error code otherwise.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_obtain_shm_provider(const struct z_loaned_session_t *this_,
                                 struct z_owned_shared_shm_provider_t *out_provider,
                                 enum z_shm_provider_state *out_state);
#endif
/**
 * Constructs and opens a new Zenoh session.
 *
 * @return 0 in case of success, negative error code otherwise (in this case the session will be in its gravestone state).
 */
ZENOHC_API
z_result_t z_open(struct z_owned_session_t *this_,
                  struct z_moved_config_t *config,
                  const struct z_open_options_t *_options);
/**
 * Constructs the default value for `z_open_options_t`.
 */
ZENOHC_API void z_open_options_default(struct z_open_options_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs and opens a new Zenoh session with specified client storage.
 *
 * @return 0 in case of success, negative error code otherwise (in this case the session will be in its gravestone state).
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_open_with_custom_shm_clients(struct z_owned_session_t *this_,
                                          struct z_moved_config_t *config,
                                          const struct z_loaned_shm_client_storage_t *shm_clients);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new POSIX SHM Client.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_posix_shm_client_new(struct z_owned_shm_client_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new POSIX SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_posix_shm_provider_new(struct z_owned_shm_provider_t *this_,
                                    size_t size);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new POSIX SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_posix_shm_provider_with_layout_new(struct z_owned_shm_provider_t *this_,
                                                const struct z_loaned_memory_layout_t *layout);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation without any additional actions.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_precomputed_layout_alloc(struct z_buf_alloc_result_t *out_result,
                                const struct z_loaned_precomputed_layout_t *layout);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation performing garbage collection if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_precomputed_layout_alloc_gc(struct z_buf_alloc_result_t *out_result,
                                   const struct z_loaned_precomputed_layout_t *layout);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation performing garbage collection and/or defragmentation if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_precomputed_layout_alloc_gc_defrag(struct z_buf_alloc_result_t *out_result,
                                          const struct z_loaned_precomputed_layout_t *layout);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation performing garbage collection and/or defragmentation and/or blocking if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_precomputed_layout_alloc_gc_defrag_blocking(struct z_buf_alloc_result_t *out_result,
                                                   const struct z_loaned_precomputed_layout_t *layout);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation performing garbage collection and/or defragmentation and/or forced deallocation if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_precomputed_layout_alloc_gc_defrag_dealloc(struct z_buf_alloc_result_t *out_result,
                                                  const struct z_loaned_precomputed_layout_t *layout);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Deletes Alloc Layout.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_precomputed_layout_drop(struct z_moved_precomputed_layout_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows Alloc Layout.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const struct z_loaned_precomputed_layout_t *z_precomputed_layout_loan(const struct z_owned_precomputed_layout_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation performing garbage collection and/or defragmentation in async manner. Will return Z_EINVAL
 * if used with non-threadsafe SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_precomputed_layout_threadsafe_alloc_gc_defrag_async(struct z_buf_alloc_result_t *out_result,
                                                                 const struct z_loaned_precomputed_layout_t *layout,
                                                                 struct zc_threadsafe_context_t result_context,
                                                                 void (*result_callback)(void*,
                                                                                         struct z_buf_alloc_result_t*));
#endif
/**
 * Returns the default value of #z_priority_t.
 */
ZENOHC_API enum z_priority_t z_priority_default(void);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Makes a shallow data pointer in SHM Segment copy.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_ptr_in_segment_clone(struct z_owned_ptr_in_segment_t *out,
                            const struct z_loaned_ptr_in_segment_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Deletes data pointer in SHM Segment.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_ptr_in_segment_drop(struct z_moved_ptr_in_segment_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows data pointer in SHM Segment.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const struct z_loaned_ptr_in_segment_t *z_ptr_in_segment_loan(const struct z_owned_ptr_in_segment_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new data pointer in SHM Segment.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_ptr_in_segment_new(struct z_owned_ptr_in_segment_t *this_,
                          uint8_t *ptr,
                          struct zc_threadsafe_context_t segment);
#endif
/**
 * @brief Declares a matching listener, registering a callback for notifying subscribers matching with a given publisher.
 * The callback will be run in the background until the corresponding publisher is dropped.
 *
 * @param publisher: A publisher to associate with matching listener.
 * @param callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber disconnects or when the first subscriber connects).
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_publisher_declare_background_matching_listener(const struct z_loaned_publisher_t *publisher,
                                                            struct z_moved_closure_matching_status_t *callback);
/**
 * @brief Constructs matching listener, registering a callback for notifying subscribers matching with a given publisher.
 *
 * @param publisher: A publisher to associate with matching listener.
 * @param matching_listener: An uninitialized memory location where matching listener will be constructed. The matching listener's callback will be automatically dropped when the publisher is dropped.
 * @param callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber disconnects or when the first subscriber connects).
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_publisher_declare_matching_listener(const struct z_loaned_publisher_t *publisher,
                                                 struct z_owned_matching_listener_t *matching_listener,
                                                 struct z_moved_closure_matching_status_t *callback);
/**
 * Sends a `DELETE` message onto the publisher's key expression.
 *
 * @return 0 in case of success, negative error code in case of failure.
 */
ZENOHC_API
z_result_t z_publisher_delete(const struct z_loaned_publisher_t *publisher,
                              struct z_publisher_delete_options_t *options);
/**
 * Constructs the default values for the delete operation via a publisher entity.
 */
ZENOHC_API void z_publisher_delete_options_default(struct z_publisher_delete_options_t *this_);
/**
 * Frees memory and resets publisher to its gravestone state.
 * This is equivalent to calling `z_undeclare_publisher()` and discarding its return value.
 */
ZENOHC_API void z_publisher_drop(struct z_moved_publisher_t *this_);
/**
 * @brief Gets publisher matching status - i.e. if there are any subscribers matching its key expression.
 *
 * @return 0 in case of success, negative error code otherwise (in this case matching_status is not updated).
 */
ZENOHC_API
z_result_t z_publisher_get_matching_status(const struct z_loaned_publisher_t *this_,
                                           struct z_matching_status_t *matching_status);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the ID of the publisher.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
struct z_entity_global_id_t z_publisher_id(const struct z_loaned_publisher_t *publisher);
#endif
/**
 * Returns the key expression of the publisher.
 */
ZENOHC_API
const struct z_loaned_keyexpr_t *z_publisher_keyexpr(const struct z_loaned_publisher_t *publisher);
/**
 * Borrows publisher.
 */
ZENOHC_API
const struct z_loaned_publisher_t *z_publisher_loan(const struct z_owned_publisher_t *this_);
/**
 * Mutably borrows publisher.
 */
ZENOHC_API struct z_loaned_publisher_t *z_publisher_loan_mut(struct z_owned_publisher_t *this_);
/**
 * Constructs the default value for `z_publisher_options_t`.
 */
ZENOHC_API void z_publisher_options_default(struct z_publisher_options_t *this_);
/**
 * Sends a `PUT` message onto the publisher's key expression, transfering the payload ownership.
 *
 *
 * The payload and all owned options fields are consumed upon function return.
 *
 * @param this_: The publisher.
 * @param payload: The data to publish. Will be consumed.
 * @param options: The publisher put options. All owned fields will be consumed.
 *
 * @return 0 in case of success, negative error values in case of failure.
 */
ZENOHC_API
z_result_t z_publisher_put(const struct z_loaned_publisher_t *this_,
                           struct z_moved_bytes_t *payload,
                           struct z_publisher_put_options_t *options);
/**
 * Constructs the default value for `z_publisher_put_options_t`.
 */
ZENOHC_API void z_publisher_put_options_default(struct z_publisher_put_options_t *this_);
/**
 * Publishes data on specified key expression.
 *
 * @param session: The Zenoh session.
 * @param key_expr: The key expression to publish to.
 * @param payload: The value to put (consumed upon function return).
 * @param options: The put options (all owned values will be consumed upon function return).
 *
 * @return 0 in case of success, negative error values in case of failure.
 */
ZENOHC_API
z_result_t z_put(const struct z_loaned_session_t *session,
                 const struct z_loaned_keyexpr_t *key_expr,
                 struct z_moved_bytes_t *payload,
                 struct z_put_options_t *options);
/**
 * Constructs the default value for `z_put_options_t`.
 */
ZENOHC_API void z_put_options_default(struct z_put_options_t *this_);
/**
 * @brief Declares a matching listener, registering a callback for notifying queryables matching the given querier key expression and target.
 * The callback will be run in the background until the corresponding querier is dropped.
 *
 * @param querier: A querier to associate with matching listener.
 * @param callback: A closure that will be called every time the matching status of the querier changes (If last queryable disconnects or when the first queryable connects).
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_querier_declare_background_matching_listener(const struct z_loaned_querier_t *querier,
                                                          struct z_moved_closure_matching_status_t *callback);
/**
 * @brief Constructs matching listener, registering a callback for notifying queryables matching with a given querier's key expression and target.
 *
 * @param querier: A querier to associate with matching listener.
 * @param matching_listener: An uninitialized memory location where matching listener will be constructed. The matching listener's callback will be automatically dropped when the querier is dropped.
 * @param callback: A closure that will be called every time the matching status of the querier changes (If last queryable disconnects or when the first queryable connects).
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_querier_declare_matching_listener(const struct z_loaned_querier_t *querier,
                                               struct z_owned_matching_listener_t *matching_listener,
                                               struct z_moved_closure_matching_status_t *callback);
/**
 * @brief Frees memory and resets querier to its gravestone state.
 * This is equivalent to calling `z_undeclare_querier()` and discarding its return value.
 */
ZENOHC_API void z_querier_drop(struct z_moved_querier_t *this_);
/**
 * @brief Query data from the matching queryables in the system.
 * Replies are provided through a callback function.
 *
 * @param querier: The querier to make query from.
 * @param parameters: The query's parameters null-terminated string, similar to a url's query segment.
 * @param callback: The callback function that will be called on reception of replies for this query. It will be automatically dropped once all replies are processed.
 * @param options: Additional options for the get. All owned fields will be consumed.
 *
 * @return 0 in case of success, a negative error value upon failure.
 */
ZENOHC_API
z_result_t z_querier_get(const struct z_loaned_querier_t *querier,
                         const char *parameters,
                         struct z_moved_closure_reply_t *callback,
                         struct z_querier_get_options_t *options);
/**
 * @brief Gets querier matching status - i.e. if there are any queryables matching its key expression and target.
 *
 * @return 0 in case of success, negative error code otherwise (in this case matching_status is not updated).
 */
ZENOHC_API
z_result_t z_querier_get_matching_status(const struct z_loaned_querier_t *this_,
                                         struct z_matching_status_t *matching_status);
/**
 * @brief Constructs the default value for `z_querier_get_options_t`.
 */
ZENOHC_API void z_querier_get_options_default(struct z_querier_get_options_t *this_);
/**
 * @brief Query data from the matching queryables in the system.
 * Replies are provided through a callback function.
 *
 * @param querier: The querier to make query from.
 * @param parameters: The query's parameters, similar to a url's query segment.
 * @param parameters_len: The length of the query's parameters substring.
 * @param callback: The callback function that will be called on reception of replies for this query. It will be automatically dropped once all replies are processed.
 * @param options: Additional options for the get. All owned fields will be consumed.
 *
 * @return 0 in case of success, a negative error value upon failure.
 */
ZENOHC_API
z_result_t z_querier_get_with_parameters_substr(const struct z_loaned_querier_t *querier,
                                                const char *parameters,
                                                size_t parameters_len,
                                                struct z_moved_closure_reply_t *callback,
                                                struct z_querier_get_options_t *options);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the ID of the querier.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
struct z_entity_global_id_t z_querier_id(const struct z_loaned_querier_t *querier);
#endif
/**
 * @brief Returns the key expression of the querier.
 */
ZENOHC_API
const struct z_loaned_keyexpr_t *z_querier_keyexpr(const struct z_loaned_querier_t *querier);
/**
 * @brief Borrows querier.
 */
ZENOHC_API const struct z_loaned_querier_t *z_querier_loan(const struct z_owned_querier_t *this_);
/**
 * @brief Mutably borrows querier.
 */
ZENOHC_API struct z_loaned_querier_t *z_querier_loan_mut(struct z_owned_querier_t *this_);
/**
 * @brief Constructs the default value for `z_querier_options_t`.
 */
ZENOHC_API void z_querier_options_default(struct z_querier_options_t *this_);
/**
 * Gets query attachment.
 *
 * Returns NULL if query does not contain an attachment.
 */
ZENOHC_API const struct z_loaned_bytes_t *z_query_attachment(const struct z_loaned_query_t *this_);
/**
 * Gets mutable query attachment.
 *
 * Returns NULL if query does not contain an attachment.
 */
ZENOHC_API struct z_loaned_bytes_t *z_query_attachment_mut(struct z_loaned_query_t *this_);
/**
 * Constructs a shallow copy of the query, allowing to keep it in an "open" state past the callback's return.
 *
 * This operation is infallible, but may return a gravestone value if `query` itself was a gravestone value (which cannot be the case in a callback).
 */
ZENOHC_API
void z_query_clone(struct z_owned_query_t *dst,
                   const struct z_loaned_query_t *this_);
/**
 * Automatic query consolidation strategy selection.
 *
 * A query consolidation strategy will automatically be selected depending the query selector.
 * If the selector contains time range properties, no consolidation is performed.
 * Otherwise the `z_query_consolidation_latest` strategy is used.
 */
ZENOHC_API struct z_query_consolidation_t z_query_consolidation_auto(void);
/**
 * Creates a default `z_query_consolidation_t` (consolidation mode AUTO).
 */
ZENOHC_API struct z_query_consolidation_t z_query_consolidation_default(void);
/**
 * Latest consolidation.
 *
 * This strategy optimizes bandwidth on all links in the system but will provide a very poor latency.
 */
ZENOHC_API
struct z_query_consolidation_t z_query_consolidation_latest(void);
/**
 * Monotonic consolidation.
 *
 * This strategy offers the best latency. Replies are directly transmitted to the application when received
 * without needing to wait for all replies. This mode does not guarantee that there will be no duplicates.
 */
ZENOHC_API
struct z_query_consolidation_t z_query_consolidation_monotonic(void);
/**
 * No consolidation.
 *
 * This strategy is useful when querying timeseries data bases or when using quorums.
 */
ZENOHC_API struct z_query_consolidation_t z_query_consolidation_none(void);
/**
 * Destroys the query resetting it to its gravestone value.
 */
ZENOHC_API void z_query_drop(struct z_moved_query_t *this_);
/**
 * Gets query <a href="https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md">payload encoding</a>.
 *
 * Returns NULL if query does not contain an encoding.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_query_encoding(const struct z_loaned_query_t *this_);
/**
 * Gets query key expression.
 */
ZENOHC_API const struct z_loaned_keyexpr_t *z_query_keyexpr(const struct z_loaned_query_t *this_);
/**
 * Borrows the query.
 */
ZENOHC_API const struct z_loaned_query_t *z_query_loan(const struct z_owned_query_t *this_);
/**
 * Mutably borrows the query.
 */
ZENOHC_API struct z_loaned_query_t *z_query_loan_mut(struct z_owned_query_t *this_);
/**
 * Gets query <a href="https://github.com/eclipse-zenoh/roadmap/tree/main/rfcs/ALL/Selectors">value selector</a>.
 */
ZENOHC_API
void z_query_parameters(const struct z_loaned_query_t *this_,
                        struct z_view_string_t *parameters);
/**
 * Gets query <a href="https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md">payload</a>.
 *
 * Returns NULL if query does not contain a payload.
 */
ZENOHC_API
const struct z_loaned_bytes_t *z_query_payload(const struct z_loaned_query_t *this_);
/**
 * Gets mutable query <a href="https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Query%20Payload.md">payload</a>.
 *
 * Returns NULL if query does not contain a payload.
 */
ZENOHC_API
struct z_loaned_bytes_t *z_query_payload_mut(struct z_loaned_query_t *this_);
/**
 * Sends a reply to a query.
 *
 * This function must be called inside of a Queryable callback passing the
 * query received as parameters of the callback function. This function can
 * be called multiple times to send multiple replies to a query. The reply
 * will be considered complete when the Queryable callback returns.
 *
 * @param this_: The query to reply to.
 * @param key_expr: The key of this reply.
 * @param payload: The payload of this reply. Will be consumed.
 * @param options: The options of this reply. All owned fields will be consumed.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_query_reply(const struct z_loaned_query_t *this_,
                         const struct z_loaned_keyexpr_t *key_expr,
                         struct z_moved_bytes_t *payload,
                         struct z_query_reply_options_t *options);
/**
 * Sends a delete reply to a query.
 *
 * This function must be called inside of a Queryable callback passing the
 * query received as parameters of the callback function. This function can
 * be called multiple times to send multiple replies to a query. The reply
 * will be considered complete when the Queryable callback returns.
 *
 * @param this_: The query to reply to.
 * @param key_expr: The key of this delete reply.
 * @param options: The options of this delete reply. All owned fields will be consumed.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_query_reply_del(const struct z_loaned_query_t *this_,
                             const struct z_loaned_keyexpr_t *key_expr,
                             struct z_query_reply_del_options_t *options);
/**
 * Constructs the default value for `z_query_reply_del_options_t`.
 */
ZENOHC_API void z_query_reply_del_options_default(struct z_query_reply_del_options_t *this_);
/**
 * Sends a error reply to a query.
 *
 * This function must be called inside of a Queryable callback passing the
 * query received as parameters of the callback function. This function can
 * be called multiple times to send multiple replies to a query. The reply
 * will be considered complete when the Queryable callback returns.
 *
 * @param this_: The query to reply to.
 * @param payload: The payload carrying error message. Will be consumed.
 * @param options: The options of this reply. All owned fields will be consumed.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_query_reply_err(const struct z_loaned_query_t *this_,
                             struct z_moved_bytes_t *payload,
                             struct z_query_reply_err_options_t *options);
/**
 * Constructs the default value for `z_query_reply_err_options_t`.
 */
ZENOHC_API void z_query_reply_err_options_default(struct z_query_reply_err_options_t *this_);
/**
 * Constructs the default value for `z_query_reply_options_t`.
 */
ZENOHC_API void z_query_reply_options_default(struct z_query_reply_options_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the query source_info. Will return NULL, if source info is not set.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct z_source_info_t *z_query_source_info(const struct z_loaned_query_t *this_);
#endif
/**
 * Takes ownership of the mutably borrowed query
 */
ZENOHC_API void z_query_take_from_loaned(struct z_owned_query_t *dst, struct z_loaned_query_t *src);
/**
 * Create a default `z_query_target_t`.
 */
ZENOHC_API enum z_query_target_t z_query_target_default(void);
/**
 * Undeclares queryable callback and resets it to its gravestone state.
 * This is equivalent to calling `z_undeclare_queryable()` and discarding its return value.
 */
ZENOHC_API void z_queryable_drop(struct z_moved_queryable_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the ID of the queryable.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
struct z_entity_global_id_t z_queryable_id(const struct z_loaned_queryable_t *queryable);
#endif
/**
 * @brief Returns the key expression of the queryable.
 */
ZENOHC_API
const struct z_loaned_keyexpr_t *z_queryable_keyexpr(const struct z_loaned_queryable_t *queryable);
ZENOHC_API
const struct z_loaned_queryable_t *z_queryable_loan(const struct z_owned_queryable_t *this_);
/**
 * Constructs the default value for `z_query_reply_options_t`.
 */
ZENOHC_API void z_queryable_options_default(struct z_queryable_options_t *this_);
/**
 * Fills buffer with random data.
 */
ZENOHC_API void z_random_fill(void *buf, size_t len);
/**
 * Generates random `uint16_t`.
 */
ZENOHC_API uint16_t z_random_u16(void);
/**
 * Generates random `uint32_t`.
 */
ZENOHC_API uint32_t z_random_u32(void);
/**
 * Generates random `uint64_t`.
 */
ZENOHC_API uint64_t z_random_u64(void);
/**
 * Generates random `uint8_t`.
 */
ZENOHC_API uint8_t z_random_u8(void);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Reference the global client storage.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_ref_shm_client_storage_global(struct z_owned_shm_client_storage_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the default value for `reliability`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
enum z_reliability_t z_reliability_default(void);
#endif
/**
 * Constructs an owned shallow copy of reply in provided uninitialized memory location.
 */
ZENOHC_API void z_reply_clone(struct z_owned_reply_t *dst, const struct z_loaned_reply_t *this_);
/**
 * Frees reply, resetting it to its gravestone state.
 */
ZENOHC_API void z_reply_drop(struct z_moved_reply_t *this_);
/**
 * Yields the contents of the reply by asserting it indicates a failure.
 *
 * Returns `NULL` if reply does not contain a error  (i. e. if `z_reply_is_ok` returns ``true``).
 */
ZENOHC_API const struct z_loaned_reply_err_t *z_reply_err(const struct z_loaned_reply_t *this_);
/**
 * Constructs a copy of the reply error message.
 */
ZENOHC_API
void z_reply_err_clone(struct z_owned_reply_err_t *dst,
                       const struct z_loaned_reply_err_t *this_);
/**
 * Frees the memory and resets the reply error it to its default value.
 */
ZENOHC_API void z_reply_err_drop(struct z_moved_reply_err_t *this_);
/**
 * Returns reply error encoding.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_reply_err_encoding(const struct z_loaned_reply_err_t *this_);
/**
 * Borrows reply error.
 */
ZENOHC_API
const struct z_loaned_reply_err_t *z_reply_err_loan(const struct z_owned_reply_err_t *this_);
/**
 * Mutably borrows reply error.
 */
ZENOHC_API struct z_loaned_reply_err_t *z_reply_err_loan_mut(struct z_owned_reply_err_t *this_);
/**
 * Yields the contents of the reply by asserting it indicates a failure.
 *
 * Returns `NULL` if reply does not contain a error  (i. e. if `z_reply_is_ok` returns ``true``).
 */
ZENOHC_API struct z_loaned_reply_err_t *z_reply_err_mut(struct z_loaned_reply_t *this_);
/**
 * Returns reply error payload.
 */
ZENOHC_API
const struct z_loaned_bytes_t *z_reply_err_payload(const struct z_loaned_reply_err_t *this_);
/**
 * Returns mutable reply error payload.
 */
ZENOHC_API struct z_loaned_bytes_t *z_reply_err_payload_mut(struct z_loaned_reply_err_t *this_);
/**
 * Returns ``true`` if reply contains a valid response, ``false`` otherwise (in this case it contains a errror value).
 */
ZENOHC_API
bool z_reply_is_ok(const struct z_loaned_reply_t *this_);
/**
 * Borrows reply.
 */
ZENOHC_API const struct z_loaned_reply_t *z_reply_loan(const struct z_owned_reply_t *this_);
/**
 * Mutably borrows reply.
 */
ZENOHC_API struct z_loaned_reply_t *z_reply_loan_mut(struct z_owned_reply_t *this_);
/**
 * Yields the contents of the reply by asserting it indicates a success.
 *
 * Returns `NULL` if reply does not contain a sample (i. e. if `z_reply_is_ok` returns ``false``).
 */
ZENOHC_API const struct z_loaned_sample_t *z_reply_ok(const struct z_loaned_reply_t *this_);
/**
 * Yields the contents of the reply by asserting it indicates a success.
 *
 * Returns `NULL` if reply does not contain a sample (i. e. if `z_reply_is_ok` returns ``false``).
 */
ZENOHC_API struct z_loaned_sample_t *z_reply_ok_mut(struct z_loaned_reply_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Gets the global id of the zenoh entity that answered this Reply.
 * @return `true` if id is present.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
bool z_reply_replier_id(const struct z_loaned_reply_t *this_,
                        struct z_entity_global_id_t *out_id);
#endif
/**
 * Takes ownership of the mutably borrowed reply
 */
ZENOHC_API void z_reply_take_from_loaned(struct z_owned_reply_t *dst, struct z_loaned_reply_t *src);
/**
 * Constructs send and recieve ends of the ring channel
 */
ZENOHC_API
void z_ring_channel_query_new(struct z_owned_closure_query_t *callback,
                              struct z_owned_ring_handler_query_t *handler,
                              size_t capacity);
/**
 * Constructs send and recieve ends of the ring channel
 */
ZENOHC_API
void z_ring_channel_reply_new(struct z_owned_closure_reply_t *callback,
                              struct z_owned_ring_handler_reply_t *handler,
                              size_t capacity);
/**
 * Constructs send and recieve ends of the ring channel
 */
ZENOHC_API
void z_ring_channel_sample_new(struct z_owned_closure_sample_t *callback,
                               struct z_owned_ring_handler_sample_t *handler,
                               size_t capacity);
/**
 * Drops the handler and resets it to a gravestone state.
 */
ZENOHC_API void z_ring_handler_query_drop(struct z_moved_ring_handler_query_t *this_);
/**
 * Borrows handler.
 */
ZENOHC_API
const struct z_loaned_ring_handler_query_t *z_ring_handler_query_loan(const struct z_owned_ring_handler_query_t *this_);
/**
 * Returns query from the ring buffer. If there are no more pending queries will block until next query is received, or until
 * the channel is dropped (normally when Queryable is dropped).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the query will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_ring_handler_query_recv(const struct z_loaned_ring_handler_query_t *this_,
                                     struct z_owned_query_t *query);
/**
 * Returns query from the ring buffer. If there are no more pending queries will return immediately (with query set to its gravestone state).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the query will be in the gravestone state),
 * Z_CHANNEL_NODATA if the channel is still alive, but its buffer is empty (the query will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_ring_handler_query_try_recv(const struct z_loaned_ring_handler_query_t *this_,
                                         struct z_owned_query_t *query);
/**
 * Drops the handler and resets it to a gravestone state.
 */
ZENOHC_API void z_ring_handler_reply_drop(struct z_moved_ring_handler_reply_t *this_);
/**
 * Borrows handler.
 */
ZENOHC_API
const struct z_loaned_ring_handler_reply_t *z_ring_handler_reply_loan(const struct z_owned_ring_handler_reply_t *this_);
/**
 * Returns reply from the ring buffer. If there are no more pending replies will block until next reply is received, or until
 * the channel is dropped (normally when all replies are received).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the reply will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_ring_handler_reply_recv(const struct z_loaned_ring_handler_reply_t *this_,
                                     struct z_owned_reply_t *reply);
/**
 * Returns reply from the ring buffer. If there are no more pending replies will return immediately (with reply set to its gravestone state).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the reply will be in the gravestone state),
 * `Z_CHANNEL_NODATA` if the channel is still alive, but its buffer is empty (the reply will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_ring_handler_reply_try_recv(const struct z_loaned_ring_handler_reply_t *this_,
                                         struct z_owned_reply_t *reply);
/**
 * Drops the handler and resets it to a gravestone state.
 */
ZENOHC_API void z_ring_handler_sample_drop(struct z_moved_ring_handler_sample_t *this_);
/**
 * Borrows handler.
 */
ZENOHC_API
const struct z_loaned_ring_handler_sample_t *z_ring_handler_sample_loan(const struct z_owned_ring_handler_sample_t *this_);
/**
 * Returns sample from the ring buffer. If there are no more pending replies will block until next sample is received, or until
 * the channel is dropped (normally when there are no more replies to receive).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the sample will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_ring_handler_sample_recv(const struct z_loaned_ring_handler_sample_t *this_,
                                      struct z_owned_sample_t *sample);
/**
 * Returns sample from the ring buffer. If there are no more pending replies will return immediately (with sample set to its gravestone state).
 * @return 0 in case of success, `Z_CHANNEL_DISCONNECTED` if channel was dropped (the sample will be in the gravestone state),
 * `Z_CHANNEL_NODATA` if the channel is still alive, but its buffer is empty (the sample will be in the gravestone state).
 */
ZENOHC_API
z_result_t z_ring_handler_sample_try_recv(const struct z_loaned_ring_handler_sample_t *this_,
                                          struct z_owned_sample_t *sample);
/**
 * Returns sample attachment.
 *
 * Returns `NULL`, if sample does not contain any attachment.
 */
ZENOHC_API
const struct z_loaned_bytes_t *z_sample_attachment(const struct z_loaned_sample_t *this_);
/**
 * Constructs an owned shallow copy of the sample (i.e. all modficiations applied to the copy, might be visible in the original) in provided uninitilized memory location.
 */
ZENOHC_API
void z_sample_clone(struct z_owned_sample_t *dst,
                    const struct z_loaned_sample_t *this_);
/**
 * Returns sample qos congestion control value.
 */
ZENOHC_API
enum z_congestion_control_t z_sample_congestion_control(const struct z_loaned_sample_t *this_);
/**
 * Frees the memory and invalidates the sample, resetting it to a gravestone state.
 */
ZENOHC_API void z_sample_drop(struct z_moved_sample_t *this_);
/**
 * Returns the encoding associated with the sample data.
 */
ZENOHC_API
const struct z_loaned_encoding_t *z_sample_encoding(const struct z_loaned_sample_t *this_);
/**
 * Returns whether sample qos express flag was set or not.
 */
ZENOHC_API bool z_sample_express(const struct z_loaned_sample_t *this_);
/**
 * Returns the key expression of the sample.
 */
ZENOHC_API const struct z_loaned_keyexpr_t *z_sample_keyexpr(const struct z_loaned_sample_t *this_);
/**
 * Returns the sample kind.
 */
ZENOHC_API enum z_sample_kind_t z_sample_kind(const struct z_loaned_sample_t *this_);
/**
 * Borrows sample.
 */
ZENOHC_API const struct z_loaned_sample_t *z_sample_loan(const struct z_owned_sample_t *this_);
/**
 * Mutably borrows sample.
 */
ZENOHC_API struct z_loaned_sample_t *z_sample_loan_mut(struct z_owned_sample_t *this_);
/**
 * Returns the sample payload data.
 */
ZENOHC_API const struct z_loaned_bytes_t *z_sample_payload(const struct z_loaned_sample_t *this_);
/**
 * Returns the mutable sample payload data.
 */
ZENOHC_API struct z_loaned_bytes_t *z_sample_payload_mut(struct z_loaned_sample_t *this_);
/**
 * Returns sample qos priority value.
 */
ZENOHC_API enum z_priority_t z_sample_priority(const struct z_loaned_sample_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the reliability setting the sample was delivered with.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
enum z_reliability_t z_sample_reliability(const struct z_loaned_sample_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the sample source_info. Will return NULL, if source info is not set.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct z_source_info_t *z_sample_source_info(const struct z_loaned_sample_t *this_);
#endif
/**
 * Takes ownership of the mutably borrowed sample.
 */
ZENOHC_API
void z_sample_take_from_loaned(struct z_owned_sample_t *dst,
                               struct z_loaned_sample_t *src);
/**
 * Returns the sample timestamp.
 *
 * Will return `NULL`, if sample is not associated with a timestamp.
 */
ZENOHC_API const struct z_timestamp_t *z_sample_timestamp(const struct z_loaned_sample_t *this_);
/**
 * Scout for routers and/or peers.
 *
 * @param config: A set of properties to configure scouting session.
 * @param callback: A closure that will be called on each hello message received from discoverd Zenoh entities.
 * @param options: A set of scouting options
 *
 * @return 0 if successful, negative error values upon failure.
 */
ZENOHC_API
z_result_t z_scout(struct z_moved_config_t *config,
                   struct z_moved_closure_hello_t *callback,
                   const struct z_scout_options_t *options);
/**
 * Constructs the default values for the scouting operation.
 */
ZENOHC_API void z_scout_options_default(struct z_scout_options_t *this_);
/**
 * Closes and invalidates the session.
 */
ZENOHC_API void z_session_drop(struct z_moved_session_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the ID of the session.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
struct z_entity_global_id_t z_session_id(const struct z_loaned_session_t *session);
#endif
/**
 * Checks if zenoh session is closed.
 *
 * @return `true` if session is closed, `false` otherwise.
 */
ZENOHC_API bool z_session_is_closed(const struct z_loaned_session_t *session);
/**
 * Borrows session.
 */
ZENOHC_API const struct z_loaned_session_t *z_session_loan(const struct z_owned_session_t *this_);
ZENOHC_API struct z_loaned_session_t *z_session_loan_mut(struct z_owned_session_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Constructs a shallow copy of shared SHM provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shared_shm_provider_clone(struct z_owned_shared_shm_provider_t *dst,
                                 const struct z_loaned_shared_shm_provider_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Deletes Shared SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shared_shm_provider_drop(struct z_moved_shared_shm_provider_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows Shared SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const struct z_loaned_shared_shm_provider_t *z_shared_shm_provider_loan(const struct z_owned_shared_shm_provider_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Loan as SHM Provider. Provides access to the underlying SHM Provider to be used where SHM Provider is expected.
 */
#if ((defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API)) && (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API)))
ZENOHC_API
const struct z_loaned_shm_provider_t *z_shared_shm_provider_loan_as(const struct z_loaned_shared_shm_provider_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Deletes SHM Client.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_client_drop(struct z_moved_shm_client_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new SHM Client.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_client_new(struct z_owned_shm_client_t *this_,
                      struct zc_threadsafe_context_t context,
                      struct zc_shm_client_callbacks_t callbacks);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Performs a shallow copy of SHM Client Storage.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_client_storage_clone(struct z_owned_shm_client_storage_t *this_,
                                const struct z_loaned_shm_client_storage_t *from);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Derefs SHM Client Storage.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_client_storage_drop(struct z_moved_shm_client_storage_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows SHM Client Storage.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const struct z_loaned_shm_client_storage_t *z_shm_client_storage_loan(const struct z_owned_shm_client_storage_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Create a new client storage object.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_shm_client_storage_new(struct z_owned_shm_client_storage_t *this_,
                                    const struct zc_loaned_shm_client_list_t *clients,
                                    bool add_default_client_set);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Construct client storage with default client set.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_client_storage_new_default(struct z_owned_shm_client_storage_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Converts borrowed ZShm slice to owned ZShm slice by performing a shallow SHM reference copy.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_clone(struct z_owned_shm_t *out,
                 const struct z_loaned_shm_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @return the pointer of the ZShm slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const unsigned char *z_shm_data(const struct z_loaned_shm_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Deletes ZShm slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_drop(struct z_moved_shm_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs ZShm slice from ZShmMut slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_from_mut(struct z_owned_shm_t *this_,
                    struct z_moved_shm_mut_t *that);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @return the length of the ZShm slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
size_t z_shm_len(const struct z_loaned_shm_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows ZShm slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const struct z_loaned_shm_t *z_shm_loan(const struct z_owned_shm_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Mutably borrows ZShm slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
struct z_loaned_shm_t *z_shm_loan_mut(struct z_owned_shm_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @return the immutable pointer to the underlying data.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const unsigned char *z_shm_mut_data(const struct z_loaned_shm_mut_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @return the mutable pointer to the underlying data.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
unsigned char *z_shm_mut_data_mut(struct z_loaned_shm_mut_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Deletes ZShmMut slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_mut_drop(struct z_moved_shm_mut_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @return the length of the ZShmMut slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
size_t z_shm_mut_len(const struct z_loaned_shm_mut_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows ZShmMut slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const struct z_loaned_shm_mut_t *z_shm_mut_loan(const struct z_owned_shm_mut_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Mutably borrows ZShmMut slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
struct z_loaned_shm_mut_t *z_shm_mut_loan_mut(struct z_owned_shm_mut_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Tries to obtain mutable SHM buffer instead of immutable one.
 * @param this_: mutable SHM buffer to be initialized upon success
 * @param that: immutable SHM buffer
 * @param immut: immutable SHM buffer returned back to caller's side
 * ONLY in case of Z_EUNAVAILABLE failure
 * @return Z_OK in case of success, Z_EUNAVAILABLE in case of unsuccessful write access,
 * Z_EINVAL if moved value is incorrect.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_shm_mut_try_from_immut(struct z_owned_shm_mut_t *this_,
                                    struct z_moved_shm_t *that,
                                    struct z_owned_shm_t *immut);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation without any additional actions.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_alloc(struct z_buf_layout_alloc_result_t *out_result,
                          const struct z_loaned_shm_provider_t *provider,
                          size_t size);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make aligned allocation without any additional actions.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_alloc_aligned(struct z_buf_layout_alloc_result_t *out_result,
                                  const struct z_loaned_shm_provider_t *provider,
                                  size_t size,
                                  struct z_alloc_alignment_t alignment);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation performing garbage collection if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_alloc_gc(struct z_buf_layout_alloc_result_t *out_result,
                             const struct z_loaned_shm_provider_t *provider,
                             size_t size);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make aligned allocation performing garbage collection if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_alloc_gc_aligned(struct z_buf_layout_alloc_result_t *out_result,
                                     const struct z_loaned_shm_provider_t *provider,
                                     size_t size,
                                     struct z_alloc_alignment_t alignment);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation performing garbage collection and/or defragmentation if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_alloc_gc_defrag(struct z_buf_layout_alloc_result_t *out_result,
                                    const struct z_loaned_shm_provider_t *provider,
                                    size_t size);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make aligned allocation performing garbage collection and/or defragmentation if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_alloc_gc_defrag_aligned(struct z_buf_layout_alloc_result_t *out_result,
                                            const struct z_loaned_shm_provider_t *provider,
                                            size_t size,
                                            struct z_alloc_alignment_t alignment);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make aligned allocation performing garbage collection and/or defragmentation in async manner. Will return Z_EINVAL
 * if used with non-threadsafe SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_shm_provider_alloc_gc_defrag_aligned_async(struct z_buf_layout_alloc_result_t *out_result,
                                                        const struct z_loaned_shm_provider_t *provider,
                                                        size_t size,
                                                        struct z_alloc_alignment_t alignment,
                                                        struct zc_threadsafe_context_t result_context,
                                                        void (*result_callback)(void*,
                                                                                struct z_buf_layout_alloc_result_t*));
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation performing garbage collection and/or defragmentation in async manner. Will return Z_EINVAL
 * if used with non-threadsafe SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_shm_provider_alloc_gc_defrag_async(struct z_buf_layout_alloc_result_t *out_result,
                                                const struct z_loaned_shm_provider_t *provider,
                                                size_t size,
                                                struct zc_threadsafe_context_t result_context,
                                                void (*result_callback)(void*,
                                                                        struct z_buf_layout_alloc_result_t*));
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation performing garbage collection and/or defragmentation and/or blocking if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_alloc_gc_defrag_blocking(struct z_buf_layout_alloc_result_t *out_result,
                                             const struct z_loaned_shm_provider_t *provider,
                                             size_t size);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make aligned allocation performing garbage collection and/or defragmentation and/or blocking if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_alloc_gc_defrag_blocking_aligned(struct z_buf_layout_alloc_result_t *out_result,
                                                     const struct z_loaned_shm_provider_t *provider,
                                                     size_t size,
                                                     struct z_alloc_alignment_t alignment);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make allocation performing garbage collection and/or defragmentation and/or forced deallocation if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_alloc_gc_defrag_dealloc(struct z_buf_layout_alloc_result_t *out_result,
                                            const struct z_loaned_shm_provider_t *provider,
                                            size_t size);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Make aligned allocation performing garbage collection and/or defragmentation and/or forced deallocation if needed.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_alloc_gc_defrag_dealloc_aligned(struct z_buf_layout_alloc_result_t *out_result,
                                                    const struct z_loaned_shm_provider_t *provider,
                                                    size_t size,
                                                    struct z_alloc_alignment_t alignment);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new Alloc Layout for SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_shm_provider_alloc_layout(struct z_owned_precomputed_layout_t *this_,
                                       const struct z_loaned_shm_provider_t *provider,
                                       size_t size);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new Alloc Layout for SHM Provider specifying the exact alignment.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_shm_provider_alloc_layout_aligned(struct z_owned_precomputed_layout_t *this_,
                                               const struct z_loaned_shm_provider_t *provider,
                                               size_t size,
                                               struct z_alloc_alignment_t alignment);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Return the memory size available in the provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
size_t z_shm_provider_available(const struct z_loaned_shm_provider_t *provider);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new SHM Provider ith default backend.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_shm_provider_default_new(struct z_owned_shm_provider_t *this_,
                                      size_t size);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Perform memory defragmentation. The real operations taken depend on the provider's backend allocator
 * implementation.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
size_t z_shm_provider_defragment(const struct z_loaned_shm_provider_t *provider);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Deletes SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_drop(struct z_moved_shm_provider_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Perform memory garbage collection and reclaim all dereferenced SHM buffers.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
size_t z_shm_provider_garbage_collect(const struct z_loaned_shm_provider_t *provider);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const struct z_loaned_shm_provider_t *z_shm_provider_loan(const struct z_owned_shm_provider_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Map the preallocated data chunk into SHM buffer.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t z_shm_provider_map(struct z_owned_shm_mut_t *out_result,
                              const struct z_loaned_shm_provider_t *provider,
                              struct z_allocated_chunk_t allocated_chunk,
                              size_t len);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_new(struct z_owned_shm_provider_t *this_,
                        struct zc_context_t context,
                        struct zc_shm_provider_backend_callbacks_t callbacks);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new threadsafe SHM Provider.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void z_shm_provider_threadsafe_new(struct z_owned_shm_provider_t *this_,
                                   struct zc_threadsafe_context_t context,
                                   struct zc_shm_provider_backend_callbacks_t callbacks);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Mutably borrows ZShm slice as borrowed ZShmMut slice.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
struct z_loaned_shm_mut_t *z_shm_try_mut(struct z_owned_shm_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Tries to reborrow mutably-borrowed ZShm slice as borrowed ZShmMut slice.
 * @return borrowed ZShmMut slice in case of success, NULL otherwise.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
struct z_loaned_shm_mut_t *z_shm_try_reloan_mut(struct z_loaned_shm_t *this_);
#endif
/**
 * Puts current thread to sleep for specified amount of milliseconds.
 */
ZENOHC_API z_result_t z_sleep_ms(size_t time);
/**
 * Puts current thread to sleep for specified amount of seconds.
 */
ZENOHC_API z_result_t z_sleep_s(size_t time);
/**
 * Puts current thread to sleep for specified amount of microseconds.
 */
ZENOHC_API z_result_t z_sleep_us(size_t time);
/**
 * Constructs an owned copy of a slice.
 */
ZENOHC_API void z_slice_clone(struct z_owned_slice_t *dst, const struct z_loaned_slice_t *this_);
/**
 * Constructs a slice by copying a `len` bytes long sequence starting at `start`.
 *
 * @return -1 if `start == NULL` and `len > 0` (creating an empty slice), 0 otherwise.
 */
ZENOHC_API
z_result_t z_slice_copy_from_buf(struct z_owned_slice_t *this_,
                                 const uint8_t *start,
                                 size_t len);
/**
 * @return the pointer to the slice data.
 */
ZENOHC_API const uint8_t *z_slice_data(const struct z_loaned_slice_t *this_);
/**
 * Frees the memory and invalidates the slice.
 */
ZENOHC_API void z_slice_drop(struct z_moved_slice_t *this_);
/**
 * Constructs an empty `z_owned_slice_t`.
 */
ZENOHC_API void z_slice_empty(struct z_owned_slice_t *this_);
/**
 * Constructs a slice by transferring ownership of `data` to it.
 * @param this_: Pointer to an uninitialized memoery location where slice will be constructed.
 * @param data: Pointer to the data to be owned by `this_`.
 * @param len: Number of bytes in `data`.
 * @param drop: A thread-safe delete function to free the `data`. Will be called once when `this_` is dropped. Can be NULL, in case if `data` is allocated in static memory.
 * @param context: An optional context to be passed to the `deleter`.
 *
 * @return -1 if `start == NULL` and `len > 0` (creating an empty slice), 0 otherwise.
 */
ZENOHC_API
z_result_t z_slice_from_buf(struct z_owned_slice_t *this_,
                            uint8_t *data,
                            size_t len,
                            void (*drop)(void *data, void *context),
                            void *context);
/**
 * @return ``true`` if slice is empty, ``false`` otherwise.
 */
ZENOHC_API bool z_slice_is_empty(const struct z_loaned_slice_t *this_);
/**
 * @return the length of the slice.
 */
ZENOHC_API size_t z_slice_len(const struct z_loaned_slice_t *this_);
/**
 * Borrows slice.
 */
ZENOHC_API const struct z_loaned_slice_t *z_slice_loan(const struct z_owned_slice_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the source id of the source info.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
struct z_entity_global_id_t z_source_info_id(const struct z_source_info_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates source info.
 *
 * @param source_id: Non-null pointer to source entity global id.
 * @param source_sn: Source sequence number.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
struct z_source_info_t z_source_info_new(const struct z_entity_global_id_t *source_id,
                                         uint32_t source_sn);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the source_sn of the source info.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
uint32_t z_source_info_sn(const struct z_source_info_t *this_);
#endif
/**
 * Constructs an owned copy of a string array.
 */
ZENOHC_API
void z_string_array_clone(struct z_owned_string_array_t *dst,
                          const struct z_loaned_string_array_t *this_);
/**
 * Destroys the string array, resetting it to its gravestone value.
 */
ZENOHC_API void z_string_array_drop(struct z_moved_string_array_t *this_);
/**
 * @return the value at the position of index in the string array.
 *
 * Will return `NULL` if the index is out of bounds.
 */
ZENOHC_API
const struct z_loaned_string_t *z_string_array_get(const struct z_loaned_string_array_t *this_,
                                                   size_t index);
/**
 * @return ``true`` if the array is empty, ``false`` otherwise.
 */
ZENOHC_API bool z_string_array_is_empty(const struct z_loaned_string_array_t *this_);
/**
 * @return number of elements in the array.
 */
ZENOHC_API size_t z_string_array_len(const struct z_loaned_string_array_t *this_);
/**
 * Borrows string array.
 */
ZENOHC_API
const struct z_loaned_string_array_t *z_string_array_loan(const struct z_owned_string_array_t *this_);
/**
 * Mutably borrows string array.
 */
ZENOHC_API
struct z_loaned_string_array_t *z_string_array_loan_mut(struct z_owned_string_array_t *this_);
/**
 * Constructs a new empty string array.
 */
ZENOHC_API void z_string_array_new(struct z_owned_string_array_t *this_);
/**
 * Appends specified value to the end of the string array by alias.
 *
 * @return the new length of the array.
 */
ZENOHC_API
size_t z_string_array_push_by_alias(struct z_loaned_string_array_t *this_,
                                    const struct z_loaned_string_t *value);
/**
 * Appends specified value to the end of the string array by copying.
 *
 * @return the new length of the array.
 */
ZENOHC_API
size_t z_string_array_push_by_copy(struct z_loaned_string_array_t *this_,
                                   const struct z_loaned_string_t *value);
ZENOHC_API const struct z_loaned_slice_t *z_string_as_slice(const struct z_loaned_string_t *this_);
/**
 * Constructs an owned copy of a string.
 */
ZENOHC_API void z_string_clone(struct z_owned_string_t *dst, const struct z_loaned_string_t *this_);
/**
 * Constructs an owned string by copying `str` into it (including terminating 0), using `strlen` (this should therefore not be used with untrusted inputs).
 *
 * @return -1 if `str == NULL` (and creates a string in a gravestone state), 0 otherwise.
 */
ZENOHC_API
z_result_t z_string_copy_from_str(struct z_owned_string_t *this_,
                                  const char *str);
/**
 * Constructs an owned string by copying a `str` substring of length `len`.
 *
 * @return -1 if `str == NULL` and `len > 0` (and creates a string in a gravestone state), 0 otherwise.
 */
ZENOHC_API
z_result_t z_string_copy_from_substr(struct z_owned_string_t *this_,
                                     const char *str,
                                     size_t len);
/**
 * @return the pointer of the string data.
 */
ZENOHC_API const char *z_string_data(const struct z_loaned_string_t *this_);
/**
 * Frees memory and invalidates `z_owned_string_t`, putting it in gravestone state.
 */
ZENOHC_API void z_string_drop(struct z_moved_string_t *this_);
/**
 * Constructs an empty owned string.
 */
ZENOHC_API void z_string_empty(struct z_owned_string_t *this_);
/**
 * Constructs an owned string by transferring ownership of a null-terminated string `str` to it.
 * @param this_: Pointer to an uninitialized memory location where an owned string will be constructed.
 * @param str: Pointer to a null terminated string to be owned by `this_`.
 * @param drop: A thread-safe delete function to free the `str`. Will be called once when `str` is dropped. Can be NULL, in case if `str` is allocated in static memory.
 * @param context: An optional context to be passed to the `deleter`.
 * @return -1 if `str == NULL` and `len > 0` (and creates a string in a gravestone state), 0 otherwise.
 */
ZENOHC_API
z_result_t z_string_from_str(struct z_owned_string_t *this_,
                             char *str,
                             void (*drop)(void *value, void *context),
                             void *context);
/**
 * @return ``true`` if string is empty, ``false`` otherwise.
 */
ZENOHC_API bool z_string_is_empty(const struct z_loaned_string_t *this_);
/**
 * @return the length of the string (without terminating 0 character).
 */
ZENOHC_API size_t z_string_len(const struct z_loaned_string_t *this_);
/**
 * Borrows string.
 */
ZENOHC_API const struct z_loaned_string_t *z_string_loan(const struct z_owned_string_t *this_);
/**
 * Undeclares subscriber callback and resets it to its gravestone state.
 * This is equivalent to calling `z_undeclare_subscriber()` and discarding its return value.
 */
ZENOHC_API void z_subscriber_drop(struct z_moved_subscriber_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the ID of the subscriber.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
struct z_entity_global_id_t z_subscriber_id(const struct z_loaned_subscriber_t *subscriber);
#endif
/**
 * Returns the key expression of the subscriber.
 */
ZENOHC_API
const struct z_loaned_keyexpr_t *z_subscriber_keyexpr(const struct z_loaned_subscriber_t *subscriber);
/**
 * Borrows subscriber.
 */
ZENOHC_API
const struct z_loaned_subscriber_t *z_subscriber_loan(const struct z_owned_subscriber_t *this_);
/**
 * Constructs the default value for `z_subscriber_options_t`.
 */
ZENOHC_API void z_subscriber_options_default(struct z_subscriber_options_t *this_);
/**
 * Detaches the task and releases all allocated resources.
 */
ZENOHC_API void z_task_detach(struct z_moved_task_t *this_);
/**
 * Drop the task. Same as `z_task_detach`. Use `z_task_join` to wait for the task completion.
 */
ZENOHC_API void z_task_drop(struct z_moved_task_t *this_);
/**
 * Constructs a new task.
 *
 * @param this_: An uninitialized memory location where task will be constructed.
 * @param _attr: Attributes of the task (currently unused).
 * @param fun: Function to be executed by the task.
 * @param arg: Argument that will be passed to the function `fun`.
 */
ZENOHC_API
z_result_t z_task_init(struct z_owned_task_t *this_,
                       const struct z_task_attr_t *_attr,
                       void *(*fun)(void *arg),
                       void *arg);
/**
 * Joins the task and releases all allocated resources
 */
ZENOHC_API z_result_t z_task_join(struct z_moved_task_t *this_);
/**
 * Get number of milliseconds passed since creation of `time`.
 */
ZENOHC_API uint64_t z_time_elapsed_ms(const struct z_time_t *time);
/**
 * Get number of seconds passed since creation of `time`.
 */
ZENOHC_API uint64_t z_time_elapsed_s(const struct z_time_t *time);
/**
 * Get number of microseconds passed since creation of `time`.
 */
ZENOHC_API uint64_t z_time_elapsed_us(const struct z_time_t *time);
/**
 * Initialize clock with current time instant.
 */
ZENOHC_API struct z_time_t z_time_now(void);
/**
 * Converts current system time into null-terminated human readable string and writes it to the `buf`.
 *
 * @param buf: A buffer where the string will be writtent
 * @param len: Maximum number of characters to write (including terminating 0). The string will be truncated
 * if it is longer than `len`.
 */
ZENOHC_API
const char *z_time_now_as_str(const char *buf,
                              size_t len);
/**
 * @brief Returns id associated with this timestamp.
 */
ZENOHC_API struct z_id_t z_timestamp_id(const struct z_timestamp_t *this_);
/**
 * Create uhlc timestamp from session id.
 */
ZENOHC_API
z_result_t z_timestamp_new(struct z_timestamp_t *this_,
                           const struct z_loaned_session_t *session);
/**
 * Returns NPT64 time associated with this timestamp.
 */
ZENOHC_API uint64_t z_timestamp_ntp64_time(const struct z_timestamp_t *this_);
/**
 * Undeclares the key expression generated by a call to `z_declare_keyexpr()`.
 * The key expression is consumed.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_undeclare_keyexpr(const struct z_loaned_session_t *session,
                               struct z_moved_keyexpr_t *key_expr);
/**
 * @brief Undeclares the given matching listener, droping and invalidating it.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t z_undeclare_matching_listener(struct z_moved_matching_listener_t *this_);
/**
 * @brief Undeclares the given publisher.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t z_undeclare_publisher(struct z_moved_publisher_t *this_);
/**
 * @brief Undeclares the given querier.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t z_undeclare_querier(struct z_moved_querier_t *this_);
/**
 * Undeclares a `z_owned_queryable_t`.
 * Returns 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t z_undeclare_queryable(struct z_moved_queryable_t *this_);
/**
 * Undeclares the subscriber.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t z_undeclare_subscriber(struct z_moved_subscriber_t *this_);
/**
 * Constructs a view key expression in empty state
 */
ZENOHC_API void z_view_keyexpr_empty(struct z_view_keyexpr_t *this_);
/**
 * Constructs a `z_view_keyexpr_t` by aliasing a string.
 * @return 0 in case of success, negative error code in case of failure (for example if expr is not a valid key expression or if it is
 * not in canon form.
 * `expr` must outlive the constucted key expression.
 */
ZENOHC_API
z_result_t z_view_keyexpr_from_str(struct z_view_keyexpr_t *this_,
                                   const char *expr);
/**
 * Constructs a `z_view_keyexpr_t` by aliasing a string.
 * The string is canonized in-place before being passed to keyexpr, possibly shortening it by modifying `len`.
 * May SEGFAULT if `expr` is NULL or lies in read-only memory (as values initialized with string litterals do).
 * `expr` must outlive the constucted key expression.
 */
ZENOHC_API
z_result_t z_view_keyexpr_from_str_autocanonize(struct z_view_keyexpr_t *this_,
                                                char *expr);
/**
 * Constructs a `z_view_keyexpr_t` by aliasing a string without checking any of `z_view_keyexpr_t`'s assertions:
 *
 *  - `s` MUST be valid UTF8.
 *  - `s` MUST follow the Key Expression specification, i.e.:
 *   - MUST NOT contain `//`, MUST NOT start nor end with `/`, MUST NOT contain any of the characters `?#$`.
 *   - any instance of `**` may only be lead or followed by `/`.
 *   - the key expression must have canon form.
 *
 * `s` must outlive constructed key expression.
 */
ZENOHC_API
void z_view_keyexpr_from_str_unchecked(struct z_view_keyexpr_t *this_,
                                       const char *s);
/**
 * Constructs a `z_view_keyexpr_t` by aliasing a substring.
 * `expr` must outlive the constucted key expression.
 *
 * @param this_: An uninitialized location in memory where key expression will be constructed.
 * @param expr: A buffer with length >= `len`.
 * @param len: Number of characters from `expr` to consider.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_view_keyexpr_from_substr(struct z_view_keyexpr_t *this_,
                                      const char *expr,
                                      size_t len);
/**
 * Constructs a `z_view_keyexpr_t` by aliasing a substring.
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 * `expr` must outlive the constucted key expression.
 *
 * @param this_: An uninitialized location in memory where key expression will be constructed
 * @param start: A buffer of with length >= `len`.
 * @param len: Number of characters from `expr` to consider. Will be modified to be equal to canonized key expression length.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t z_view_keyexpr_from_substr_autocanonize(struct z_view_keyexpr_t *this_,
                                                   char *start,
                                                   size_t *len);
/**
 * Constructs a `z_view_keyexpr_t` by aliasing a substring without checking any of `z_view_keyexpr_t`'s assertions:
 *
 * - `start` MUST be valid UTF8.
 * - `start` MUST follow the Key Expression specification, i.e.:
 *  - MUST NOT contain ``//``, MUST NOT start nor end with ``/``, MUST NOT contain any of the characters ``?#$``.
 *  - any instance of ``**`` may only be lead or followed by ``/``.
 *  - the key expression must have canon form.
 *
 * `start` must outlive constructed key expression.
 */
ZENOHC_API
void z_view_keyexpr_from_substr_unchecked(struct z_view_keyexpr_t *this_,
                                          const char *start,
                                          size_t len);
/**
 * Returns ``true`` if `keyexpr` is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API bool z_view_keyexpr_is_empty(const struct z_view_keyexpr_t *this_);
/**
 * Borrows `z_view_keyexpr_t`.
 */
ZENOHC_API
const struct z_loaned_keyexpr_t *z_view_keyexpr_loan(const struct z_view_keyexpr_t *this_);
/**
 * Constructs an empty view slice.
 */
ZENOHC_API void z_view_slice_empty(struct z_view_slice_t *this_);
/**
 * Constructs a `len` bytes long view starting at `start`.
 *
 * @return -1 if `start == NULL` and `len > 0` (and creates an empty view slice), 0 otherwise.
 */
ZENOHC_API
z_result_t z_view_slice_from_buf(struct z_view_slice_t *this_,
                                 const uint8_t *start,
                                 size_t len);
/**
 * @return ``true`` if the slice is not empty, ``false`` otherwise.
 */
ZENOHC_API bool z_view_slice_is_empty(const struct z_view_slice_t *this_);
/**
 * Borrows view slice.
 */
ZENOHC_API const struct z_loaned_slice_t *z_view_slice_loan(const struct z_view_slice_t *this_);
/**
 * Constructs an empty view string.
 */
ZENOHC_API void z_view_string_empty(struct z_view_string_t *this_);
/**
 * Constructs a view string of `str`, using `strlen` (this should therefore not be used with untrusted inputs).
 *
 * @return -1 if `str == NULL` (and creates a string in a gravestone state), 0 otherwise.
 */
ZENOHC_API
z_result_t z_view_string_from_str(struct z_view_string_t *this_,
                                  const char *str);
/**
 * Constructs a view string to a specified substring of length `len`.
 *
 * @return -1 if `str == NULL` and `len > 0` (and creates a string in a gravestone state), 0 otherwise.
 */
ZENOHC_API
z_result_t z_view_string_from_substr(struct z_view_string_t *this_,
                                     const char *str,
                                     size_t len);
/**
 * @return ``true`` if view string is valid, ``false`` if it is in a gravestone state.
 */
ZENOHC_API bool z_view_string_is_empty(const struct z_view_string_t *this_);
/**
 * Borrows view string.
 */
ZENOHC_API const struct z_loaned_string_t *z_view_string_loan(const struct z_view_string_t *this_);
/**
 * Constructs a non-owned non-null-terminated string from the kind of zenoh entity.
 *
 * The string has static storage (i.e. valid until the end of the program).
 * @param whatami: A whatami bitmask of zenoh entity kind.
 * @param str_out: An uninitialized memory location where strring will be constructed.
 *
 * @return 0 if successful, negative error values if whatami contains an invalid bitmask.
 */
ZENOHC_API
z_result_t z_whatami_to_view_string(enum z_whatami_t whatami,
                                    struct z_view_string_t *str_out);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Linux: Trigger cleanup for orphaned SHM segments
 * If process that created named SHM segment crashes or exits by a signal, the segment persists in the system
 * disregarding if it is used by other Zenoh processes or not. This is the detail of POSIX specification for
 * shared memory that is hard to bypass. To deal with this we developed a cleanup routine that enumerates all
 * segments and tries to find processes that are using it. If no such process found, segment will be removed.
 * There is no ideal signal to trigger this cleanup, so by default, zenoh triggers it in the following moments:
 * - first POSIX SHM segment creation
 * - process exit via exit() call or return from maint function
 *
 * It is OK to additionally trigger this function at any time, but be aware that this can be costly.
 *
 * For non-linux platforms this function currently does nothing
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void zc_cleanup_orphaned_shm_segments(void);
#endif
/**
 * @brief Constructs closure.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 *
 * @param this_: uninitialized memory location where new closure will be constructed.
 * @param call: a closure body.
 * @param drop: an optional function to be called once on closure drop.
 * @param context: closure context.
 */
ZENOHC_API
void zc_closure_log(struct zc_owned_closure_log_t *this_,
                    void (*call)(enum zc_log_severity_t severity,
                                 const struct z_loaned_string_t *msg,
                                 void *context),
                    void (*drop)(void *context),
                    void *context);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
ZENOHC_API
void zc_closure_log_call(const struct zc_loaned_closure_log_t *closure,
                         enum zc_log_severity_t severity,
                         const struct z_loaned_string_t *msg);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
ZENOHC_API void zc_closure_log_drop(struct zc_moved_closure_log_t *closure_);
/**
 * Borrows closure.
 */
ZENOHC_API
const struct zc_loaned_closure_log_t *zc_closure_log_loan(const struct zc_owned_closure_log_t *closure);
/**
 * @brief Drops the close handle. The concurrent close task will not be interrupted.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API void zc_concurrent_close_handle_drop(struct zc_moved_concurrent_close_handle_t *this_);
#endif
/**
 * @brief Blocking wait on close handle to complete. Returns `Z_EIO` if close finishes with error.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t zc_concurrent_close_handle_wait(struct zc_moved_concurrent_close_handle_t *handle);
#endif
/**
 * Constructs a configuration by parsing a file path stored in ZENOH_CONFIG environmental variable.
 *
 * Returns 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t zc_config_from_env(struct z_owned_config_t *this_);
/**
 * Constructs a configuration by parsing a file at `path` null-terminated string. Currently supported format is JSON5, a superset of JSON.
 *
 * Returns 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t zc_config_from_file(struct z_owned_config_t *this_,
                               const char *path);
/**
 * Constructs a configuration by parsing a file at `path` susbstring of specified length. Currently supported format is JSON5, a superset of JSON.
 *
 * Returns 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t zc_config_from_file_substr(struct z_owned_config_t *this_,
                                      const char *path,
                                      size_t len);
/**
 * Reads a configuration from a JSON-serialized string, such as '{mode:"client",connect:{endpoints:["tcp/127.0.0.1:7447"]}}'.
 *
 * Returns 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t zc_config_from_str(struct z_owned_config_t *this_,
                              const char *s);
/**
 * Reads a configuration from a JSON-serialized substring of specified lenght, such as '{mode:"client",connect:{endpoints:["tcp/127.0.0.1:7447"]}}'.
 *
 * Returns 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t zc_config_from_substr(struct z_owned_config_t *this_,
                                 const char *s,
                                 size_t len);
/**
 * Gets the property with the given path key from the configuration, and constructs and owned string from it.
 */
ZENOHC_API
z_result_t zc_config_get_from_str(const struct z_loaned_config_t *this_,
                                  const char *key,
                                  struct z_owned_string_t *out_value_string);
/**
 * Gets the property with the given path key from the configuration, and constructs and owned string from it.
 */
ZENOHC_API
z_result_t zc_config_get_from_substr(const struct z_loaned_config_t *this_,
                                     const char *key,
                                     size_t key_len,
                                     struct z_owned_string_t *out_value_string);
/**
 * Inserts a JSON-serialized `value` at the `key` position of the configuration.
 *
 * Returns 0 if successful, a negative error code otherwise.
 */
ZENOHC_API
z_result_t zc_config_insert_json5(struct z_loaned_config_t *this_,
                                  const char *key,
                                  const char *value);
/**
 * Inserts a JSON-serialized `value` at the `key` position of the configuration.
 *
 * Returns 0 if successful, a negative error code otherwise.
 */
ZENOHC_API
z_result_t zc_config_insert_json5_from_substr(struct z_loaned_config_t *this_,
                                              const char *key,
                                              size_t key_len,
                                              const char *value,
                                              size_t value_len);
/**
 * Constructs a json string representation of the `config`, such as '{"mode":"client","connect":{"endpoints":["tcp/127.0.0.1:7447"]}}'.
 *
 * Returns 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t zc_config_to_string(const struct z_loaned_config_t *config,
                               struct z_owned_string_t *out_config_string);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs a view string on last error message.
 * The view string only remains valid until next faillable zenoh API call from the same thread.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void zc_get_last_error(struct z_view_string_t *out);
#endif
/**
 * Initializes the zenoh runtime logger, using rust environment settings or the provided fallback level.
 * E.g.: `RUST_LOG=info` will enable logging at info level. Similarly, you can set the variable to `error` or `debug`.
 *
 * Note that if the environment variable is not set, then fallback filter will be used instead.
 * See https://docs.rs/env_logger/latest/env_logger/index.html for accepted filter format.
 *
 * @param fallback_filter: The fallback filter if the `RUST_LOG` environment variable is not set.
 */
ZENOHC_API
z_result_t zc_init_log_from_env_or(const char *fallback_filter);
/**
 * Initializes the zenoh runtime logger with custom callback.
 *
 * @param min_severity: Minimum severity level of log message to be be passed to the `callback`.
 * Messages with lower severity levels will be ignored.
 * @param callback: A closure that will be called with each log message severity level and content.
 */
ZENOHC_API
void zc_init_log_with_callback(enum zc_log_severity_t min_severity,
                               struct zc_moved_closure_log_t *callback);
/**
 * Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
 */
ZENOHC_API bool zc_internal_closure_log_check(const struct zc_owned_closure_log_t *this_);
/**
 * Constructs a closure in a gravestone state.
 */
ZENOHC_API void zc_internal_closure_log_null(struct zc_owned_closure_log_t *this_);
/**
 * @brief Returns ``true`` if concurrent close handle is valid, ``false`` if it is in gravestone state.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
bool zc_internal_concurrent_close_handle_check(const struct zc_owned_concurrent_close_handle_t *this_);
#endif
/**
 * @brief Constructs concurrent close handle in its gravestone state.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void zc_internal_concurrent_close_handle_null(struct zc_owned_concurrent_close_handle_t *this_);
#endif
ZENOHC_API
void zc_internal_encoding_from_data(struct z_owned_encoding_t *this_,
                                    struct zc_internal_encoding_data_t data);
ZENOHC_API
struct zc_internal_encoding_data_t zc_internal_encoding_get_data(const struct z_loaned_encoding_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns ``true`` if `this` is valid.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
bool zc_internal_shm_client_list_check(const struct zc_owned_shm_client_list_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs SHM client list in its gravestone value.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void zc_internal_shm_client_list_null(struct zc_owned_shm_client_list_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use `z_locality_default().
 * @brief Returns default value of `z_locality_t`
 */
ZENOHC_API enum z_locality_t zc_locality_default(void);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the default value of #zc_reply_keyexpr_t.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
enum zc_reply_keyexpr_t zc_reply_keyexpr_default(void);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Add client to the list.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t zc_shm_client_list_add_client(struct zc_loaned_shm_client_list_t *this_,
                                         struct z_moved_shm_client_t *client);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Deletes list of SHM Clients.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void zc_shm_client_list_drop(struct zc_moved_shm_client_list_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows list of SHM Clients.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
const struct zc_loaned_shm_client_list_t *zc_shm_client_list_loan(const struct zc_owned_shm_client_list_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Mutably borrows list of SHM Clients.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
struct zc_loaned_shm_client_list_t *zc_shm_client_list_loan_mut(struct zc_owned_shm_client_list_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Creates a new empty list of SHM Clients.
 */
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
void zc_shm_client_list_new(struct zc_owned_shm_client_list_t *this_);
#endif
/**
 * Stops all Zenoh tasks and drops all related static variables.
 * All Zenoh-related structures should be properly dropped/undeclared PRIOR to this call.
 * None of Zenoh functionality can be used after this call.
 * Useful to suppress memory leaks messages due to Zenoh static variables (since they are never destroyed due to Rust language design).
 */
ZENOHC_API
void zc_stop_z_runtime(void);
/**
 * Initializes the zenoh runtime logger, using rust environment settings.
 * E.g.: `RUST_LOG=info` will enable logging at info level. Similarly, you can set the variable to `error` or `debug`.
 *
 * Note that if the environment variable is not set, then logging will not be enabled.
 * See https://docs.rs/env_logger/latest/env_logger/index.html for accepted filter format.
 */
ZENOHC_API
void zc_try_init_log_from_env(void);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs the default value for `ze_advanced_publisher_cache_options_t`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_publisher_cache_options_default(struct ze_advanced_publisher_cache_options_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Declares a matching listener, registering a callback for notifying subscribers matching with a given advanced publisher.
 * The callback will be run in the background until the corresponding publisher is dropped.
 *
 * @param publisher: An advanced publisher to associate with matching listener.
 * @param callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber disconnects or when the first subscriber connects).
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if (defined(Z_FEATURE_UNSTABLE_API) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t ze_advanced_publisher_declare_background_matching_listener(const struct ze_loaned_advanced_publisher_t *publisher,
                                                                      struct z_moved_closure_matching_status_t *callback);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs matching listener, registering a callback for notifying subscribers matching with a given advanced publisher.
 *
 * @param publisher: An advanced publisher to associate with matching listener.
 * @param matching_listener: An uninitialized memory location where matching listener will be constructed. The matching listener's callback will be automatically dropped when the publisher is dropped.
 * @param callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber disconnects or when the first subscriber connects).
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if (defined(Z_FEATURE_UNSTABLE_API) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t ze_advanced_publisher_declare_matching_listener(const struct ze_loaned_advanced_publisher_t *publisher,
                                                           struct z_owned_matching_listener_t *matching_listener,
                                                           struct z_moved_closure_matching_status_t *callback);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Sends a `DELETE` message onto the advanced publisher's key expression.
 *
 * @return 0 in case of success, negative error code in case of failure.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_advanced_publisher_delete(const struct ze_loaned_advanced_publisher_t *publisher,
                                        struct ze_advanced_publisher_delete_options_t *options);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Constructs the default values for the delete operation via an advanced publisher entity.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_publisher_delete_options_default(struct ze_advanced_publisher_delete_options_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Frees memory and resets advanced_publisher to its gravestone state.
 * This is equivalent to calling `z_undeclare_publisher()` and discarding its return value.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_publisher_drop(struct ze_moved_advanced_publisher_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Gets advanced publisher matching status - i.e. if there are any subscribers matching its key expression.
 *
 * @return 0 in case of success, negative error code otherwise (in this case matching_status is not updated).
 */
#if (defined(Z_FEATURE_UNSTABLE_API) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
z_result_t ze_advanced_publisher_get_matching_status(const struct ze_loaned_advanced_publisher_t *this_,
                                                     struct z_matching_status_t *matching_status);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the ID of the advanced publisher.
 */
#if (defined(Z_FEATURE_UNSTABLE_API) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
struct z_entity_global_id_t ze_advanced_publisher_id(const struct ze_loaned_advanced_publisher_t *publisher);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Returns the key expression of the publisher.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct z_loaned_keyexpr_t *ze_advanced_publisher_keyexpr(const struct ze_loaned_advanced_publisher_t *publisher);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Borrows advanced publisher.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct ze_loaned_advanced_publisher_t *ze_advanced_publisher_loan(const struct ze_owned_advanced_publisher_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Mutably borrows advanced publisher.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
struct ze_loaned_advanced_publisher_t *ze_advanced_publisher_loan_mut(struct ze_owned_advanced_publisher_t *this_);
#endif
/**
 * Constructs the default value for `z_publisher_options_t`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_publisher_options_default(struct ze_advanced_publisher_options_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Sends a `PUT` message onto the advanced publisher's key expression, transfering the payload ownership.
 *
 * The payload and all owned options fields are consumed upon function return.
 *
 * @param this_: The advanced publisher.
 * @param payload: The data to publish. Will be consumed.
 * @param options: The advanced publisher put options. All owned fields will be consumed.
 *
 * @return 0 in case of success, negative error values in case of failure.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_advanced_publisher_put(const struct ze_loaned_advanced_publisher_t *this_,
                                     struct z_moved_bytes_t *payload,
                                     struct ze_advanced_publisher_put_options_t *options);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Constructs the default value for `ze_advanced_publisher_put_options_t`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_publisher_put_options_default(struct ze_advanced_publisher_put_options_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs the default value for `ze_advanced_publisher_sample_miss_detection_options_t`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_publisher_sample_miss_detection_options_default(struct ze_advanced_publisher_sample_miss_detection_options_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Declares a sample miss listener, registering a callback for notifying subscriber about missed samples.
 * The callback will be run in the background until the corresponding subscriber is dropped.
 *
 * @param subscriber: A subscriber to associate with sample miss listener.
 * @param callback: A closure that will be called every time the sample miss is detected.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_advanced_subscriber_declare_background_sample_miss_listener(const struct ze_loaned_advanced_subscriber_t *subscriber,
                                                                          struct ze_moved_closure_miss_t *callback);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs sample miss listener, registering a callback for notifying subscriber about missed samples.
 *
 * @param subscriber: A subscriber to associate with sample miss listener.
 * @param sample_miss_listener: An uninitialized memory location where sample miss listener will be constructed. The sample miss listener's callback will be automatically dropped when the subscriber is dropped.
 * @param callback: A closure that will be called every time the sample miss is detected.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_advanced_subscriber_declare_sample_miss_listener(const struct ze_loaned_advanced_subscriber_t *subscriber,
                                                               struct ze_owned_sample_miss_listener_t *sample_miss_listener,
                                                               struct ze_moved_closure_miss_t *callback);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Declares a liveliness token listener for matching publishers detection. Only advanced publishers, enabling publisher detection can be detected.
 *
 * @param subscriber: The advanced subscriber instance.
 * @param liveliness_subscriber: An uninitialized memory location where liveliness subscriber will be constructed.
 * @param callback: The callback function that will be called each time a liveliness token status is changed.
 * @param options: The options to be passed to the liveliness subscriber declaration.
 *
 * @return 0 in case of success, negative error values otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_advanced_subscriber_detect_publishers(const struct ze_loaned_advanced_subscriber_t *subscriber,
                                                    struct z_owned_subscriber_t *liveliness_subscriber,
                                                    struct z_moved_closure_sample_t *callback,
                                                    struct z_liveliness_subscriber_options_t *options);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Declares a background subscriber on liveliness tokens of matching publishers. Subscriber callback will be called to process the messages,
 * until the corresponding session is closed or dropped. Only advanced publishers. enabling publisher detection can be detected.
 * @param subscriber: The advanced subscriber instance.
 * @param callback: The callback function that will be called each time a liveliness token status is changed.
 * @param options: The options to be passed to the liveliness subscriber declaration.
 *
 * @return 0 in case of success, negative error values otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_advanced_subscriber_detect_publishers_background(const struct ze_loaned_advanced_subscriber_t *subscriber,
                                                               struct z_moved_closure_sample_t *callback,
                                                               struct z_liveliness_subscriber_options_t *options);
#endif
/**
 * Undeclares advanced subscriber callback and resets it to its gravestone state.
 * This is equivalent to calling `ze_undeclare_advanced_subscriber()` and discarding its return value.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_subscriber_drop(struct ze_moved_advanced_subscriber_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs the default value for `ze_advanced_subscriber_history_options_t`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_subscriber_history_options_default(struct ze_advanced_subscriber_history_options_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns the ID of the advanced subscriber.
 */
#if (defined(Z_FEATURE_UNSTABLE_API) && defined(Z_FEATURE_UNSTABLE_API))
ZENOHC_API
struct z_entity_global_id_t ze_advanced_subscriber_id(const struct ze_loaned_advanced_subscriber_t *subscriber);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Returns the key expression of the advanced subscriber.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct z_loaned_keyexpr_t *ze_advanced_subscriber_keyexpr(const struct ze_loaned_advanced_subscriber_t *subscriber);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs the default value for `ze_advanced_subscriber_last_sample_miss_detection_options_t`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_subscriber_last_sample_miss_detection_options_default(struct ze_advanced_subscriber_last_sample_miss_detection_options_t *this_);
#endif
/**
 * Borrows subscriber.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct ze_loaned_advanced_subscriber_t *ze_advanced_subscriber_loan(const struct ze_owned_advanced_subscriber_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs the default value for `ze_advanced_subscriber_options_t`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_subscriber_options_default(struct ze_advanced_subscriber_options_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs the default value for `ze_advanced_subscriber_recovery_options_t`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_advanced_subscriber_recovery_options_default(struct ze_advanced_subscriber_recovery_options_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 *
 * Closures are not guaranteed not to be called concurrently.
 *
 * It is guaranteed that:
 *   - `call` will never be called once `drop` has started.
 *   - `drop` will only be called **once**, and **after every** `call` has ended.
 *   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
 *
 * @brief Constructs closure.
 * @param this_: uninitialized memory location where new closure will be constructed.
 * @param call: a closure body.
 * @param drop: an optional function to be called once on closure drop.
 * @param context: closure context.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_closure_miss(struct ze_owned_closure_miss_t *this_,
                     void (*call)(const struct ze_miss_t *matching_status, void *context),
                     void (*drop)(void *context),
                     void *context);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Calls the closure. Calling an uninitialized closure is a no-op.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_closure_miss_call(const struct ze_loaned_closure_miss_t *closure,
                          const struct ze_miss_t *mathing_status);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Drops the closure, resetting it to its gravestone state. Droping an uninitialized closure is a no-op.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_closure_miss_drop(struct ze_moved_closure_miss_t *closure_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Borrows closure.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct ze_loaned_closure_miss_t *ze_closure_miss_loan(const struct ze_owned_closure_miss_t *closure);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Constructs and declares an advanced publisher for the given key expression.
 *
 * Data can be put and deleted with this publisher with the help of the
 * `ze_advanced_publisher_put()` and `ze_advanced_publisher_delete()` functions.
 *
 * @param session: The Zenoh session.
 * @param publisher: An uninitialized location in memory where advanced publisher will be constructed.
 * @param key_expr: The key expression to publish to.
 * @param options: Additional options for the advanced publisher.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_declare_advanced_publisher(const struct z_loaned_session_t *session,
                                         struct ze_owned_advanced_publisher_t *publisher,
                                         const struct z_loaned_keyexpr_t *key_expr,
                                         struct ze_advanced_publisher_options_t *options);
#endif
/**
 * Constructs and declares an advanced subscriber for a given key expression. Dropping subscriber undeclares its callback.
 *
 * @param session: The zenoh session.
 * @param subscriber: An uninitialized location in memory, where advanced subscriber will be constructed.
 * @param key_expr: The key expression to subscribe.
 * @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
 * @param options: The options to be passed to the subscriber declaration.
 *
 * @return 0 in case of success, negative error code otherwise (in this case subscriber will be in its gravestone state).
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_declare_advanced_subscriber(const struct z_loaned_session_t *session,
                                          struct ze_owned_advanced_subscriber_t *subscriber,
                                          const struct z_loaned_keyexpr_t *key_expr,
                                          struct z_moved_closure_sample_t *callback,
                                          struct ze_advanced_subscriber_options_t *options);
#endif
/**
 * Constructs and declares a background advanced subscriber. Subscriber callback will be called to process the messages,
 * until the corresponding session is closed or dropped.
 *
 * @param session: The zenoh session.
 * @param key_expr: The key expression to subscribe.
 * @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
 * @param options: The options to be passed to the subscriber declaration.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_declare_background_advanced_subscriber(const struct z_loaned_session_t *session,
                                                     const struct z_loaned_keyexpr_t *key_expr,
                                                     struct z_moved_closure_sample_t *callback,
                                                     struct ze_advanced_subscriber_options_t *options);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_publisher.
 * @brief Declares a background publication cache. It will function in background until the corresponding session is closed or dropped.
 *
 * @param session: A Zenoh session.
 * @param key_expr: The key expression to publish to.
 * @param options: Additional options for the publication cache.
 *
 * @returns 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_declare_background_publication_cache(const struct z_loaned_session_t *session,
                                                   const struct z_loaned_keyexpr_t *key_expr,
                                                   struct ze_publication_cache_options_t *options);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_subscriber.
 * @brief Declares a background querying subscriber for a given key expression. Subscriber callback will be called to process the messages,
 * until the corresponding session is closed or dropped.
 *
 * @param session: A Zenoh session.
 * @param key_expr: A key expression to subscribe to.
 * @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
 * @param options: Additional options for the querying subscriber.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_declare_background_querying_subscriber(const struct z_loaned_session_t *session,
                                                     const struct z_loaned_keyexpr_t *key_expr,
                                                     struct z_moved_closure_sample_t *callback,
                                                     struct ze_querying_subscriber_options_t *options);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_publisher.
 * @brief Constructs and declares a publication cache.
 *
 * @param session: A Zenoh session.
 * @param pub_cache: An uninitialized location in memory where publication cache will be constructed.
 * @param key_expr: The key expression to publish to.
 * @param options: Additional options for the publication cache.
 *
 * @returns 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_declare_publication_cache(const struct z_loaned_session_t *session,
                                        struct ze_owned_publication_cache_t *pub_cache,
                                        const struct z_loaned_keyexpr_t *key_expr,
                                        struct ze_publication_cache_options_t *options);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_subscriber.
 * @brief Constructs and declares a querying subscriber for a given key expression.
 *
 * @param session: A Zenoh session.
 * @param querying_subscriber: An uninitialized memory location where querying subscriber will be constructed.
 * @param key_expr: A key expression to subscribe to.
 * @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
 * @param options: Additional options for the querying subscriber.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_declare_querying_subscriber(const struct z_loaned_session_t *session,
                                          struct ze_owned_querying_subscriber_t *querying_subscriber,
                                          const struct z_loaned_keyexpr_t *key_expr,
                                          struct z_moved_closure_sample_t *callback,
                                          struct ze_querying_subscriber_options_t *options);
#endif
/**
 * @brief Deserializes into a bool.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_bool(const struct z_loaned_bytes_t *this_, bool *dst);
/**
 * @brief Deserializes into a signed integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_double(const struct z_loaned_bytes_t *this_, double *dst);
/**
 * @brief Deserializes into a float.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_float(const struct z_loaned_bytes_t *this_, float *dst);
/**
 * @brief Deserializes into a signed integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_int16(const struct z_loaned_bytes_t *this_, int16_t *dst);
/**
 * @brief Deserializes into a signed integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_int32(const struct z_loaned_bytes_t *this_, int32_t *dst);
/**
 * @brief Deserializes into a signed integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_int64(const struct z_loaned_bytes_t *this_, int64_t *dst);
/**
 * @brief Deserializes into a signed integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_int8(const struct z_loaned_bytes_t *this_, int8_t *dst);
/**
 * @brief Deserializes into a slice.
 */
ZENOHC_API
z_result_t ze_deserialize_slice(const struct z_loaned_bytes_t *this_,
                                struct z_owned_slice_t *slice);
/**
 * @brief Deserializes into a UTF-8 string.
 */
ZENOHC_API
z_result_t ze_deserialize_string(const struct z_loaned_bytes_t *this_,
                                 struct z_owned_string_t *str);
/**
 * @brief Deserializes into an unsigned integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_uint16(const struct z_loaned_bytes_t *this_, uint16_t *dst);
/**
 * @brief Deserializes into an unsigned integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_uint32(const struct z_loaned_bytes_t *this_, uint32_t *dst);
/**
 * @brief Deserializes into an unsigned integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_uint64(const struct z_loaned_bytes_t *this_, uint64_t *dst);
/**
 * @brief Deserializes into an unsigned integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserialize_uint8(const struct z_loaned_bytes_t *this_, uint8_t *dst);
/**
 * @brief Deserializes into a bool.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_deserializer_deserialize_bool(struct ze_deserializer_t *this_, bool *dst);
/**
 * @brief Deserializes into a signed integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_double(struct ze_deserializer_t *this_,
                                              double *dst);
/**
 * @brief Deserializes into a float.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_float(struct ze_deserializer_t *this_,
                                             float *dst);
/**
 * @brief Deserializes into a signed integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_int16(struct ze_deserializer_t *this_,
                                             int16_t *dst);
/**
 * @brief Deserializes into a signed integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_int32(struct ze_deserializer_t *this_,
                                             int32_t *dst);
/**
 * @brief Deserializes into a signed integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_int64(struct ze_deserializer_t *this_,
                                             int64_t *dst);
/**
 * @brief Deserializes into a signed integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_int8(struct ze_deserializer_t *this_,
                                            int8_t *dst);
/**
 * @brief Initiates deserialization of a sequence of multiple elements.
 * @param this_: A serializer instance.
 * @param len:  pointer where the length of the sequence (previously passed via `z_bytes_writer_serialize_sequence_begin`) will be written.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_sequence_length(struct ze_deserializer_t *this_,
                                                       size_t *len);
/**
 * @brief Deserializes into a slice.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_slice(struct ze_deserializer_t *this_,
                                             struct z_owned_slice_t *slice);
/**
 * @brief Deserializes into a string.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_string(struct ze_deserializer_t *this_,
                                              struct z_owned_string_t *str);
/**
 * @brief Deserializes into an unsigned integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_uint16(struct ze_deserializer_t *this_,
                                              uint16_t *dst);
/**
 * @brief Deserializes into an unsigned integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_uint32(struct ze_deserializer_t *this_,
                                              uint32_t *dst);
/**
 * @brief Deserializes into an unsigned integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_uint64(struct ze_deserializer_t *this_,
                                              uint64_t *dst);
/**
 * @brief Deserializes into an unsigned integer.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_deserializer_deserialize_uint8(struct ze_deserializer_t *this_,
                                             uint8_t *dst);
/**
 * @brief Gets deserializer for`this_`.
 */
ZENOHC_API
struct ze_deserializer_t ze_deserializer_from_bytes(const struct z_loaned_bytes_t *this_);
/**
 * @brief Checks if deserializer parsed all of its data.
 * @return `true` if there is no more data to parse, `false` otherwise.
 */
ZENOHC_API bool ze_deserializer_is_done(const struct ze_deserializer_t *this_);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Returns ``true`` if advanced publisher is valid, ``false`` otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
bool ze_internal_advanced_publisher_check(const struct ze_owned_advanced_publisher_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Constructs an advanced publisher in a gravestone state.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_internal_advanced_publisher_null(struct ze_owned_advanced_publisher_t *this_);
#endif
/**
 * Returns ``true`` if advanced subscriber is valid, ``false`` otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
bool ze_internal_advanced_subscriber_check(const struct ze_owned_advanced_subscriber_t *this_);
#endif
/**
 * Constructs a subscriber in a gravestone state.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API void ze_internal_advanced_subscriber_null(struct ze_owned_advanced_subscriber_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
bool ze_internal_closure_miss_check(const struct ze_owned_closure_miss_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs a null value of 'ze_owned_closure_miss_t' type
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_internal_closure_miss_null(struct ze_owned_closure_miss_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_publisher.
 * @brief Returns ``true`` if publication cache is valid, ``false`` otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
bool ze_internal_publication_cache_check(const struct ze_owned_publication_cache_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_publisher.
 * @brief Constructs a publication cache in a gravestone state.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API void ze_internal_publication_cache_null(struct ze_owned_publication_cache_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_subscriber.
 * @brief Returns ``true`` if querying subscriber is valid, ``false`` otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
bool ze_internal_querying_subscriber_check(const struct ze_owned_querying_subscriber_t *this_);
#endif
/**
 * Constructs a querying subscriber in a gravestone state.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API void ze_internal_querying_subscriber_null(struct ze_owned_querying_subscriber_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Checks the sample_miss listener is for the gravestone state
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
bool ze_internal_sample_miss_listener_check(const struct ze_owned_sample_miss_listener_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Constructs an empty sample miss listener.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_internal_sample_miss_listener_null(struct ze_owned_sample_miss_listener_t *this_);
#endif
/**
 * @brief Returns ``true`` if `this_` is in a valid state, ``false`` if it is in a gravestone state.
 */
ZENOHC_API bool ze_internal_serializer_check(const struct ze_owned_serializer_t *this_);
/**
 * @brief Constructs a serializer in a gravestone state.
 */
ZENOHC_API void ze_internal_serializer_null(struct ze_owned_serializer_t *this_);
/**
 * @warning This API is deprecated. Please use ze_advanced_publisher.
 * @brief Drops publication cache and resets it to its gravestone state.
 * This is equivalent to calling `ze_undeclare_publication_cache()` and discarding its return value.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API void ze_publication_cache_drop(struct ze_moved_publication_cache_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_publisher.
 * @brief Returns the key expression of the publication cache.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct z_loaned_keyexpr_t *ze_publication_cache_keyexpr(const struct ze_loaned_publication_cache_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_publisher.
 * @brief Borrows publication cache.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct ze_loaned_publication_cache_t *ze_publication_cache_loan(const struct ze_owned_publication_cache_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_publisher.
 * @brief Constructs the default value for `ze_publication_cache_options_t`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API void ze_publication_cache_options_default(struct ze_publication_cache_options_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_subscriber.
 * @brief Undeclares querying subscriber callback and resets it to its gravestone state.
 * This is equivalent to calling `ze_undeclare_querying_subscriber()` and discarding its return value.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_querying_subscriber_drop(struct ze_moved_querying_subscriber_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_subscriber.
 * @brief Make querying subscriber perform an additional query on a specified selector.
 * The queried samples will be merged with the received publications and made available in the subscriber callback.
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_querying_subscriber_get(const struct ze_loaned_querying_subscriber_t *this_,
                                      const struct z_loaned_keyexpr_t *selector,
                                      struct z_get_options_t *options);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_subscriber.
 * @brief Borrows querying subscriber.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
const struct ze_loaned_querying_subscriber_t *ze_querying_subscriber_loan(const struct ze_owned_querying_subscriber_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_subscriber.
 * @brief Constructs the default value for `ze_querying_subscriber_options_t`.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_querying_subscriber_options_default(struct ze_querying_subscriber_options_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Undeclares the given sample miss listener, droping and invalidating it.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
void ze_sample_miss_listener_drop(struct ze_moved_sample_miss_listener_t *this_);
#endif
/**
 * @brief Serializes a bool.
 */
ZENOHC_API z_result_t ze_serialize_bool(struct z_owned_bytes_t *this_, bool val);
/**
 * @brief Serializes a data from buffer by.
 * @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
 * @param data: A pointer to the buffer containing data.
 * @param len: Length of the buffer.
 */
ZENOHC_API
z_result_t ze_serialize_buf(struct z_owned_bytes_t *this_,
                            const uint8_t *data,
                            size_t len);
/**
 * @brief Serializes a double.
 */
ZENOHC_API z_result_t ze_serialize_double(struct z_owned_bytes_t *this_, double val);
/**
 * @brief Serializes a float.
 */
ZENOHC_API z_result_t ze_serialize_float(struct z_owned_bytes_t *this_, float val);
/**
 * @brief Serializes a signed integer.
 */
ZENOHC_API z_result_t ze_serialize_int16(struct z_owned_bytes_t *this_, int16_t val);
/**
 * @brief Serializes a signed integer.
 */
ZENOHC_API z_result_t ze_serialize_int32(struct z_owned_bytes_t *this_, int32_t val);
/**
 * @brief Serializes a signed integer.
 */
ZENOHC_API z_result_t ze_serialize_int64(struct z_owned_bytes_t *this_, int64_t val);
/**
 * @brief Serializes a signed integer.
 */
ZENOHC_API z_result_t ze_serialize_int8(struct z_owned_bytes_t *this_, int8_t val);
/**
 * @brief Serializes a slice.
 */
ZENOHC_API
z_result_t ze_serialize_slice(struct z_owned_bytes_t *this_,
                              const struct z_loaned_slice_t *slice);
/**
 * @brief Serializes a null-terminated string.
 * The string should be a valid UTF-8.
 * @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
 * @param str: a pointer to the null-terminated string.
 */
ZENOHC_API z_result_t ze_serialize_str(struct z_owned_bytes_t *this_, const char *str);
/**
 * @brief Serializes a string.
 * The string should be a valid UTF-8.
 * @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
 * @param str: a string to serialize.
 */
ZENOHC_API
z_result_t ze_serialize_string(struct z_owned_bytes_t *this_,
                               const struct z_loaned_string_t *str);
/**
 * @brief Serializes a substring.
 * The substring should be a valid UTF-8.
 * @param this_: An uninitialized location in memory where `z_owned_bytes_t` is to be constructed.
 * @param start: a pointer to the the start of the substring.
 * @param len: the length of the substring.
 */
ZENOHC_API
z_result_t ze_serialize_substr(struct z_owned_bytes_t *this_,
                               const char *start,
                               size_t len);
/**
 * @brief Serializes an unsigned integer.
 */
ZENOHC_API z_result_t ze_serialize_uint16(struct z_owned_bytes_t *this_, uint16_t val);
/**
 * @brief Serializes an unsigned integer.
 */
ZENOHC_API z_result_t ze_serialize_uint32(struct z_owned_bytes_t *this_, uint32_t val);
/**
 * @brief Serializes an unsigned integer.
 */
ZENOHC_API z_result_t ze_serialize_uint64(struct z_owned_bytes_t *this_, uint64_t val);
/**
 * @brief Serializes an unsigned integer.
 */
ZENOHC_API z_result_t ze_serialize_uint8(struct z_owned_bytes_t *this_, uint8_t val);
/**
 * @brief Drops `this_`, resetting it to gravestone value.
 */
ZENOHC_API void ze_serializer_drop(struct ze_moved_serializer_t *this_);
/**
 * @brief Constructs a serializer with empty payload.
 * @param this_: An uninitialized memory location where serializer is to be constructed.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API z_result_t ze_serializer_empty(struct ze_owned_serializer_t *this_);
/**
 * @brief Drop serializer and extract underlying `bytes` object it was writing to.
 * @param this_: A serializer instance.
 * @param bytes: An uninitialized memory location where `bytes` object` will be written to.
 */
ZENOHC_API
void ze_serializer_finish(struct ze_moved_serializer_t *this_,
                          struct z_owned_bytes_t *bytes);
/**
 * @brief Borrows serializer.
 */
ZENOHC_API
const struct ze_loaned_serializer_t *ze_serializer_loan(const struct ze_owned_serializer_t *this_);
/**
 * @brief Muatably borrows serializer.
 */
ZENOHC_API
struct ze_loaned_serializer_t *ze_serializer_loan_mut(struct ze_owned_serializer_t *this_);
/**
 * @brief Serializes a bool.
 */
ZENOHC_API z_result_t ze_serializer_serialize_bool(struct ze_loaned_serializer_t *this_, bool val);
/**
 * @brief Serializes a data from buffer.
 * @param this_: A serializer instance.
 * @param data: A pointer to the buffer containing data.
 * @param len: Length of the buffer.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_buf(struct ze_loaned_serializer_t *this_,
                                       const uint8_t *data,
                                       size_t len);
/**
 * @brief Serializes a double.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_double(struct ze_loaned_serializer_t *this_,
                                          double val);
/**
 * @brief Serializes a float.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_float(struct ze_loaned_serializer_t *this_,
                                         float val);
/**
 * @brief Serializes a signed integer.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_int16(struct ze_loaned_serializer_t *this_,
                                         int16_t val);
/**
 * @brief Serializes a signed integer.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_int32(struct ze_loaned_serializer_t *this_,
                                         int32_t val);
/**
 * @brief Serializes a signed integer.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_int64(struct ze_loaned_serializer_t *this_,
                                         int64_t val);
/**
 * @brief Serializes a signed integer.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_int8(struct ze_loaned_serializer_t *this_,
                                        int8_t val);
/**
 * @brief Initiates serialization of a sequence of multiple elements.
 * @param this_: A serializer instance.
 * @param len: Length of the sequence. Could be read during deserialization using `ze_deserializer_deserialize_sequence_length`.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_sequence_length(struct ze_loaned_serializer_t *this_,
                                                   size_t len);
/**
 * @brief Serializes a slice.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_slice(struct ze_loaned_serializer_t *this_,
                                         const struct z_loaned_slice_t *slice);
/**
 * @brief Serializes a null-terminated string.
 * The string should be a valid UTF-8.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_str(struct ze_loaned_serializer_t *this_,
                                       const char *str);
/**
 * @brief Serializes a string.
 * The string should be a valid UTF-8.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_string(struct ze_loaned_serializer_t *this_,
                                          const struct z_loaned_string_t *str);
/**
 * @brief Serializes a substring of specified length.
 * The subsstring should be a valid UTF-8.
 * @return 0 in case of success, negative error code otherwise.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_substr(struct ze_loaned_serializer_t *this_,
                                          const char *start,
                                          size_t len);
/**
 * @brief Serializes an unsigned integer.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_uint16(struct ze_loaned_serializer_t *this_,
                                          uint16_t val);
/**
 * @brief Serializes an unsigned integer.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_uint32(struct ze_loaned_serializer_t *this_,
                                          uint32_t val);
/**
 * @brief Serializes an unsigned integer.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_uint64(struct ze_loaned_serializer_t *this_,
                                          uint64_t val);
/**
 * @brief Serializes an unsigned integer.
 */
ZENOHC_API
z_result_t ze_serializer_serialize_uint8(struct ze_loaned_serializer_t *this_,
                                         uint8_t val);
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Undeclares the given advanced publisher.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_undeclare_advanced_publisher(struct ze_moved_advanced_publisher_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * Undeclares the advanced subscriber.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_undeclare_advanced_subscriber(struct ze_moved_advanced_subscriber_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_publisher.
 * @brief Undeclares publication cache.
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API z_result_t ze_undeclare_publication_cache(struct ze_moved_publication_cache_t *this_);
#endif
/**
 * @warning This API is deprecated. Please use ze_advanced_subscriber.
 * @brief Undeclares the given querying subscriber.
 *
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_undeclare_querying_subscriber(struct ze_moved_querying_subscriber_t *this_);
#endif
/**
 * @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
 * @brief Undeclares the given sample miss listener, droping and invalidating it.
 * @return 0 in case of success, negative error code otherwise.
 */
#if defined(Z_FEATURE_UNSTABLE_API)
ZENOHC_API
z_result_t ze_undeclare_sample_miss_listener(struct ze_moved_sample_miss_listener_t *this_);
#endif
