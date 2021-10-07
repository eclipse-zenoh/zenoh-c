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


#ifndef ZENOH_GEN
#define ZENOH_GEN

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>


#define Z_SESSION_PADDING_U64 3

#define Z_CONFIG_PADDING_U64 66

#define Z_PUBLISHER_PADDING_U64 3

#define Z_SUBSCRIBER_PADDING_U64 1

#define Z_INFO_PADDING_U64 6

/**
 * The kind of consolidation that should be applied on replies to a :c:func:`zn_query`.
 *
 *     - **zn_consolidation_mode_t_FULL**: Guaranties unicity of replies. Optimizes bandwidth.
 *     - **zn_consolidation_mode_t_LAZY**: Does not garanty unicity. Optimizes latency.
 *     - **zn_consolidation_mode_t_NONE**: No consolidation.
 */
typedef enum z_consolidation_mode_t {
  z_consolidation_mode_t_FULL,
  z_consolidation_mode_t_LAZY,
  z_consolidation_mode_t_NONE,
} z_consolidation_mode_t;

/**
 * The subscription reliability.
 *
 *     - **zn_reliability_t_BEST_EFFORT**
 *     - **zn_reliability_t_RELIABLE**
 */
typedef enum z_reliability_t {
  z_reliability_t_BEST_EFFORT,
  z_reliability_t_RELIABLE,
} z_reliability_t;

/**
 * The possible values of :c:member:`zn_reply_t.tag`
 *
 *     - **zn_reply_t_Tag_DATA**: The reply contains some data.
 *     - **zn_reply_t_Tag_FINAL**: The reply does not contain any data and indicates that there will be no more replies for this query.
 */
typedef enum z_reply_t_Tag {
  z_reply_t_Tag_DATA,
  z_reply_t_Tag_FINAL,
} z_reply_t_Tag;

/**
 * The subscription mode.
 *
 *     - **zn_submode_t_PUSH**
 *     - **zn_submode_t_PULL**
 */
typedef enum z_submode_t {
  z_submode_t_PUSH,
  z_submode_t_PULL,
} z_submode_t;

typedef enum z_write_options_field_t {
  z_write_options_field_t_ENCODING,
  z_write_options_field_t_CONGESTION_CONTROL,
  z_write_options_field_t_KIND,
  z_write_options_field_t_PRIORITY,
} z_write_options_field_t;

typedef struct z_query_t z_query_t;

typedef struct z_queryable_t z_queryable_t;

typedef struct z_write_options_t z_write_options_t;

typedef struct z_bytes_t {
  const uint8_t *val;
  size_t len;
} z_bytes_t;

/**
 * An owned, zenoh-allocated, array of bytes.
 *
 * Members:
 *   const unsigned char *val: A pointer to the bytes array.
 *   unsigned int len: The length of the bytes array.
 *
 */
typedef struct z_owned_bytes_t {
  const uint8_t *val;
  size_t len;
} z_owned_bytes_t;

typedef struct z_owned_session_t {
  uint64_t _0[Z_SESSION_PADDING_U64];
} z_owned_session_t;

typedef struct z_owned_config_t {
  uint64_t _0[Z_CONFIG_PADDING_U64];
} z_owned_config_t;

typedef struct z_config_t {
  const struct z_owned_config_t *_0;
} z_config_t;

/**
 * An owned, zenoh-allocated, null-terminated, string.
 * Use `z_string_new` to construct and `z_string_free` to destroy.
 *
 * Members:
 *     `start`: the start of the held null-terminated string. `nullptr` is a legal value for `start`
 */
typedef struct z_owned_string_t {
  const char *_borrow;
} z_owned_string_t;

/**
 * An owned array of owned NULL terminated strings, allocated by zenoh.
 * Use `z_str_array_free` to destroy.
 *
 * Members:
 *   char *const *val: A pointer to the array.
 *   unsigned int len: The length of the array.
 */
typedef struct z_owned_str_array_t {
  const char *const *val;
  size_t len;
} z_owned_str_array_t;

/**
 * A hello message returned by a zenoh entity to a scout message sent with :c:func:`zn_scout`.
 *
 * Members:
 *   unsigned int whatami: The kind of zenoh entity.
 *   z_bytes_t pid: The peer id of the scouted entity (empty if absent).
 *   z_str_array_t locators: The locators of the scouted entity.
 *
 */
typedef struct z_owned_hello_t {
  unsigned int whatami;
  struct z_owned_bytes_t pid;
  struct z_owned_str_array_t locators;
} z_owned_hello_t;

/**
 * An array of :c:struct:`zn_hello_t` messages.
 *
 * Members:
 *   const zn_hello_t *val: A pointer to the array.
 *   unsigned int len: The length of the array.
 *
 */
typedef struct z_owned_hello_array_t {
  const struct z_owned_hello_t *val;
  size_t len;
} z_owned_hello_array_t;

typedef struct z_owned_info_t {
  uint64_t _0[Z_INFO_PADDING_U64];
} z_owned_info_t;

typedef struct z_session_t {
  const struct z_owned_session_t *_0;
} z_session_t;

typedef struct z_info_t {
  const struct z_owned_info_t *_0;
} z_info_t;

typedef struct z_owned_write_options_t {
  struct z_write_options_t *borrow;
} z_owned_write_options_t;

typedef struct z_owned_publisher_t {
  uint64_t _0[Z_PUBLISHER_PADDING_U64];
} z_owned_publisher_t;

typedef struct z_owned_subscriber_t {
  uint64_t _0[Z_SUBSCRIBER_PADDING_U64];
} z_owned_subscriber_t;

typedef struct z_reskey_t {
  unsigned long id;
  const char *suffix;
} z_reskey_t;

/**
 * The possible values of :c:member:`zn_target_t.tag`.
 *
 *     - **zn_target_t_BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
 *     - **zn_target_t_COMPLETE**: A set of complete queryables.
 *     - **zn_target_t_ALL**: All matching queryables.
 *     - **zn_target_t_NONE**: No queryables.
 */
typedef enum z_target_t_Tag {
  z_target_t_BEST_MATCHING,
  z_target_t_ALL,
  z_target_t_NONE,
  z_target_t_ALL_COMPLETE,
  z_target_t_COMPLETE,
} z_target_t_Tag;

typedef struct z_target_t_COMPLETE_Body {
  unsigned int n;
} z_target_t_COMPLETE_Body;

typedef struct z_target_t {
  z_target_t_Tag tag;
  union {
    z_target_t_COMPLETE_Body complete;
  };
} z_target_t;

/**
 * The zenoh-net queryables that should be target of a :c:func:`zn_query`.
 *
 * Members:
 *     unsigned int kind: A mask of queryable kinds.
 *     zn_target_t target: The query target.
 */
typedef struct z_query_target_t {
  unsigned int kind;
  struct z_target_t target;
} z_query_target_t;

/**
 * The kind of consolidation that should be applied on replies to a :c:func:`zn_query`
 * at the different stages of the reply process.
 *
 * Members:
 *   zn_consolidation_mode_t first_routers: The consolidation mode to apply on first routers of the replies routing path.
 *   zn_consolidation_mode_t last_router: The consolidation mode to apply on last router of the replies routing path.
 *   zn_consolidation_mode_t reception: The consolidation mode to apply at reception of the replies.
 */
typedef struct z_query_consolidation_t {
  enum z_consolidation_mode_t first_routers;
  enum z_consolidation_mode_t last_router;
  enum z_consolidation_mode_t reception;
} z_query_consolidation_t;

/**
 * A zenoh-net data sample.
 *
 * A sample is the value associated to a given resource at a given point in time.
 *
 * Members:
 *   z_string_t key: The resource key of this data sample.
 *   z_bytes_t value: The value of this data sample.
 */
typedef struct z_owned_sample_t {
  struct z_owned_string_t key;
  struct z_owned_bytes_t value;
} z_owned_sample_t;

/**
 * An reply to a :c:func:`zn_query` (or :c:func:`zn_query_collect`).
 *
 * Members:
 *   zn_sample_t data: a :c:type:`zn_sample_t` containing the key and value of the reply.
 *   unsigned int source_kind: The kind of the replier that sent this reply.
 *   z_bytes_t replier_id: The id of the replier that sent this reply.
 *
 */
typedef struct z_owned_reply_data_t {
  struct z_owned_sample_t data;
  unsigned int source_kind;
  struct z_owned_bytes_t replier_id;
} z_owned_reply_data_t;

/**
 * An reply to a :c:func:`zn_query`.
 *
 * Members:
 *   zn_reply_t_Tag tag: Indicates if the reply contains data or if it's a FINAL reply.
 *   zn_reply_data_t data: The reply data if :c:member:`zn_reply_t.tag` equals :c:member:`zn_reply_t_Tag.zn_reply_t_Tag_DATA`.
 *
 */
typedef struct z_owned_reply_t {
  enum z_reply_t_Tag tag;
  struct z_owned_reply_data_t data;
} z_owned_reply_t;

/**
 * An array of :c:type:`zn_reply_data_t`.
 * Result of :c:func:`zn_query_collect`.
 *
 * Members:
 *   char *const *val: A pointer to the array.
 *   unsigned int len: The length of the array.
 *
 */
typedef struct z_owned_reply_data_array_t {
  const struct z_owned_reply_data_t *val;
  size_t len;
} z_owned_reply_data_array_t;

typedef struct z_owned_queryable_t {
  struct z_queryable_t *borrow;
} z_owned_queryable_t;

/**
 * A resource key.
 *
 * Resources are identified by URI like string names.
 * Examples : ``"/some/resource/key"``.
 * Resource names can be mapped to numerical ids through :c:func:`zn_declare_resource`
 * for wire and computation efficiency.
 *
 * A resource key can be either:
 *   - A plain string resource name.
 *   - A pure numerical id.
 *   - The combination of a numerical prefix and a string suffix.
 *
 * Members:
 *   unsigned long id: The id or prefix of this resource key. ``0`` if empty.
 *   z_string_t suffix: The suffix of the ressource key. May be an empty string.
 */
typedef struct z_owned_reskey_t {
  unsigned long id;
  struct z_owned_string_t suffix;
} z_owned_reskey_t;

/**
 * The subscription period.
 * Equivalent of the rust `Option<zenoh::time::Period>` type, where `None` is represented by the `period` field being 0-valued.
 *
 * Members:
 *     unsigned int origin:
 *     unsigned int period:
 *     unsigned int duration:
 */
typedef struct z_period_t {
  unsigned int origin;
  unsigned int period;
  unsigned int duration;
} z_period_t;

/**
 * Informations to be passed to :c:func:`zn_declare_subscriber` to configure the created :c:type:`zn_subscriber_t`.
 *
 * Members:
 *     zn_reliability_t reliability: The subscription reliability.
 *     zn_submode_t mode: The subscription mode.
 *     zn_period_t *period: The subscription period.
 */
typedef struct z_subinfo_t {
  enum z_reliability_t reliability;
  enum z_submode_t mode;
  struct z_period_t period;
} z_subinfo_t;

typedef const char *z_string_t;

typedef struct z_sample_t {
  z_string_t key;
  struct z_bytes_t value;
} z_sample_t;

#define z_period_NONE (z_period_t){ .origin = 0, .period = 0, .duration = 0 }

extern const unsigned int ZN_ROUTER;

extern const unsigned int ZN_PEER;

extern const unsigned int ZN_CLIENT;

extern const unsigned int ZN_QUERYABLE_ALL_KINDS;

extern const unsigned int ZN_QUERYABLE_STORAGE;

extern const unsigned int ZN_QUERYABLE_EVAL;

extern const unsigned int ZN_CONFIG_MODE_KEY;

extern const unsigned int ZN_CONFIG_PEER_KEY;

extern const unsigned int ZN_CONFIG_LISTENER_KEY;

extern const unsigned int ZN_CONFIG_USER_KEY;

extern const unsigned int ZN_CONFIG_PASSWORD_KEY;

extern const unsigned int ZN_CONFIG_MULTICAST_SCOUTING_KEY;

extern const unsigned int ZN_CONFIG_MULTICAST_INTERFACE_KEY;

extern const unsigned int ZN_CONFIG_MULTICAST_IPV4_ADDRESS_KEY;

extern const unsigned int ZN_CONFIG_SCOUTING_TIMEOUT_KEY;

extern const unsigned int ZN_CONFIG_SCOUTING_DELAY_KEY;

extern const unsigned int ZN_CONFIG_ADD_TIMESTAMP_KEY;

extern const unsigned int ZN_CONFIG_LOCAL_ROUTING_KEY;

extern const unsigned int ZN_INFO_PID_KEY;

extern const unsigned int ZN_INFO_PEER_PID_KEY;

extern const unsigned int ZN_INFO_ROUTER_PID_KEY;

struct z_bytes_t z_bytes_borrow(const struct z_owned_bytes_t *b);

bool z_bytes_check(const struct z_owned_bytes_t *b);

/**
 * Free a :c:type:`z_bytes_t`.
 *
 * Parameters:
 *    b : The array to free.
 */
void z_bytes_free(struct z_owned_bytes_t *b);

/**
 * Close a zenoh-net session.
 *
 * Parameters:
 *     session: A zenoh-net session.
 */
void z_close(struct z_owned_session_t *session);

struct z_config_t z_config_borrow(const struct z_owned_config_t *s);

bool z_config_check(const struct z_owned_config_t *ps);

/**
 * Create a default set of properties for client mode zenoh-net session configuration.
 * If peer is not null, it is added to the configuration as remote peer.
 *
 * Parameters:
 *   peer: An optional peer locator.
 */
struct z_owned_config_t z_config_client(char *peer);

/**
 * Create a default set of properties for zenoh-net session configuration.
 */
struct z_owned_config_t z_config_default(void);

/**
 * Create an empty set of properties for zenoh-net session configuration.
 */
struct z_owned_config_t z_config_empty(void);

/**
 * Free a set of properties.
 *
 * Parameters:
 *   ps: A pointer to the properties.
 */
void z_config_free(struct z_owned_config_t *ps);

/**
 * Create a set of properties for zenoh-net session configuration, parsing a file listing the properties
 * (1 "key=value" per line, comments starting with '#' character are allowed).
 * Returns null if parsing fails.
 *
 * Parameters:
 *   path: The path to the file (must be in UTF-8).
 */
struct z_owned_config_t z_config_from_file(const char *path);

/**
 * Create a set of properties for zenoh-net session configuration, parsing a string listing the properties
 * in such format: "mode=client;peer=tcp/127.0.0.1:7447".
 */
struct z_owned_config_t z_config_from_str(const char *s);

/**
 * Get the property with the given key from a properties map.
 *
 * Parameters:
 *     ps: A pointer to properties map.
 *     key: The key of the property.
 *
 * Returns:
 *     The value of the property with key ``key`` in properties map ``ps``.
 */
struct z_owned_string_t z_config_get(const struct z_config_t *ps, unsigned int key);

/**
 * Get the length of the given properties map.
 *
 * Parameters:
 *     ps: A pointer to the properties map.
 *
 * Returns:
 *     The length of the given properties map.
 */
unsigned int z_config_len(const struct z_config_t *ps);

/**
 * Return a new empty configuration.
 */
struct z_owned_config_t z_config_new(void);

/**
 * Create a default set of properties for peer mode zenoh-net session configuration.
 */
struct z_owned_config_t z_config_peer(void);

/**
 * Insert a property with a given key to a properties map.
 * If a property with the same key already exists in the properties map, it is replaced.
 *
 * Parameters:
 *   ps: A pointer to the properties map.
 *   key: The key of the property to add.
 *   value: The value of the property to add.
 *
 * Returns:
 *     A pointer to the updated properties map.
 */
void z_config_set(struct z_config_t ps, unsigned long key, struct z_owned_string_t value);

/**
 * Convert a set of properties into a string.
 *
 * Parameters:
 *     config: The set of properties.
 *
 * Returns:
 *     A keys/values string containing with such format: "key1=value1;key2=value2;...".
 */
struct z_owned_string_t z_config_to_str(const struct z_config_t *config);

/**
 * Free an array of :c:struct:`zn_hello_t` messages and it's contained :c:struct:`zn_hello_t` messages recursively.
 *
 * Parameters:
 *     strs: The array of :c:struct:`zn_hello_t` messages to free.
 *
 */
void z_hello_array_free(struct z_owned_hello_array_t *hellos);

/**
 * Get informations about an zenoh-net session.
 *
 * Parameters:
 *     session: A zenoh-net session.
 *
 * Returns:
 *     A :c:type:`zn_properties_t` map containing informations on the given zenoh-net session.
 */
struct z_owned_info_t z_info(struct z_session_t session);

/**
 * Get informations about an zenoh-net session.
 *
 * Parameters:
 *     session: A zenoh-net session.
 *
 * Returns:
 *     A keys/values string containing informations on the given zenoh-net session.
 *     The format of the string is: "key1=value1;key2=value2;...".
 */
struct z_owned_string_t z_info_as_str(struct z_session_t session);

struct z_info_t z_info_borrow(const struct z_owned_info_t *info);

bool z_info_check(const struct z_owned_info_t *info);

void z_info_free(struct z_owned_info_t *info);

struct z_owned_string_t z_info_get(struct z_info_t info, uint64_t key);

/**
 * Initialise the zenoh runtime logger
 *
 */
void z_init_logger(void);

/**
 * Open a zenoh-net session
 *
 * Parameters:
 *     config: A set of properties.
 *
 * Returns:
 *     The created zenoh-net session or null if the creation did not succeed, wrapped in the z_owned_session_t type.
 *     Later functions do not check for null pointers, so you should do it to prevent segfaults.
 */
struct z_owned_session_t z_open(struct z_owned_config_t *config);

void z_owned_write_options_free(struct z_owned_write_options_t options);

struct z_owned_write_options_t z_owned_write_options_new(void);

bool z_publisher_check(const struct z_owned_publisher_t *publ);

/**
 * Pull data for a pull mode :c:type:`zn_subscriber_t`. The pulled data will be provided
 * by calling the **callback** function provided to the :c:func:`zn_declare_subscriber` function.
 *
 * Parameters:
 *     sub: The :c:type:`zn_subscriber_t` to pull from.
 */
void z_pull(const struct z_owned_subscriber_t *sub);

/**
 * Query data from the matching queryables in the system.
 * Replies are provided through a callback function.
 *
 * Parameters:
 *     session: The zenoh-net session.
 *     resource: The resource key to query.
 *     predicate: An indication to matching queryables about the queried data.
 *     target: The kind of queryables that should be target of this query.
 *     consolidation: The kind of consolidation that should be applied on replies.
 *     callback: The callback function that will be called on reception of replies for this query.
 *     arg: A pointer that will be passed to the **callback** on each call.
 */
void z_query(struct z_session_t session,
             struct z_reskey_t reskey,
             const char *predicate,
             struct z_query_target_t target,
             struct z_query_consolidation_t consolidation,
             void (*callback)(struct z_owned_reply_t, const void*),
             void *arg);

/**
 * Query data from the matching queryables in the system.
 * Replies are collected in an array.
 *
 * Parameters:
 *     session: The zenoh-net session.
 *     resource: The resource key to query.
 *     predicate: An indication to matching queryables about the queried data.
 *     target: The kind of queryables that should be target of this query.
 *     consolidation: The kind of consolidation that should be applied on replies.
 *
 * Returns:
 *    An array containing all the replies for this query.
 */
struct z_owned_reply_data_array_t z_query_collect(struct z_session_t session,
                                                  struct z_reskey_t reskey,
                                                  const char *predicate,
                                                  struct z_query_target_t target,
                                                  struct z_query_consolidation_t consolidation);

/**
 * Create a default :c:type:`zn_query_consolidation_t`.
 */
struct z_query_consolidation_t z_query_consolidation_default(void);

/**
 * Get the predicate of a received query.
 *
 * Parameters:
 *     query: The query.
 *
 * Returns:
 *     The predicate of the query.
 */
struct z_owned_string_t z_query_predicate(const struct z_query_t *query);

/**
 * Get the resource name of a received query.
 *
 * Parameters:
 *     query: The query.
 *
 * Returns:
 *     The resource name of the query.
 */
struct z_owned_string_t z_query_res_name(const struct z_query_t *query);

/**
 * Create a default :c:type:`zn_query_target_t`.
 */
struct z_query_target_t z_query_target_default(void);

/**
 * Declare a :c:type:`zn_publisher_t` for the given resource key.
 *
 * Written resources that match the given key will only be sent on the network
 * if matching subscribers exist in the system.
 *
 * Parameters:
 *     session: The zenoh-net session.
 *     resource: The resource key to publish.
 *
 * Returns:
 *    The created :c:type:`zn_publisher_t` or null if the declaration failed.
 */
struct z_owned_publisher_t z_register_publisher(struct z_session_t session,
                                                struct z_reskey_t reskey);

/**
 * Declare a :c:type:`zn_queryable_t` for the given resource key.
 *
 * Parameters:
 *     session: The zenoh-net session.
 *     resource: The resource key the :c:type:`zn_queryable_t` will reply to.
 *     kind: The kind of :c:type:`zn_queryable_t`.
 *     callback: The callback function that will be called each time a matching query is received.
 *     arg: A pointer that will be passed to the **callback** on each call.
 *
 * Returns:
 *    The created :c:type:`zn_queryable_t` or null if the declaration failed.
 */
struct z_owned_queryable_t z_register_queryable(struct z_session_t session,
                                                struct z_reskey_t reskey,
                                                unsigned int kind,
                                                void (*callback)(struct z_query_t*, const void*),
                                                void *arg);

/**
 * Associate a numerical id with the given resource key.
 *
 * This numerical id will be used on the network to save bandwidth and
 * ease the retrieval of the concerned resource in the routing tables.
 *
 * Parameters:
 *     session: The zenoh-net session.
 *     resource: The resource key to map to a numerical id.
 *
 * Returns:
 *     A numerical id.
 */
struct z_owned_reskey_t z_register_resource(struct z_session_t session,
                                            struct z_owned_reskey_t reskey);

/**
 * Declare a :c:type:`zn_subscriber_t` for the given resource key.
 *
 * Parameters:
 *     session: The zenoh-net session.
 *     resource: The resource key to subscribe.
 *     sub_info: The :c:type:`zn_subinfo_t` to configure the :c:type:`zn_subscriber_t`.
 *     callback: The callback function that will be called each time a data matching the subscribed resource is received.
 *     arg: A pointer that will be passed to the **callback** on each call.
 *
 * Returns:
 *    The created :c:type:`zn_subscriber_t` or null if the declaration failed.
 */
struct z_owned_subscriber_t z_register_subscriber(struct z_session_t session,
                                                  struct z_reskey_t reskey,
                                                  struct z_subinfo_t sub_info,
                                                  void (*callback)(const struct z_sample_t*, const void*),
                                                  void *arg);

bool z_reply_check(const struct z_owned_reply_t *reply);

bool z_reply_data_array_check(const struct z_owned_reply_data_array_t *replies);

/**
 * Free a :c:type:`zn_reply_data_array_t` and it's contained replies.
 *
 * Parameters:
 *     replies: The :c:type:`zn_reply_data_array_t` to free.
 *
 */
void z_reply_data_array_free(struct z_owned_reply_data_array_t *replies);

bool z_reply_data_check(const struct z_owned_reply_data_t *reply_data);

/**
 * Free a :c:type:`zn_reply_data_t` contained data and replier_id.
 *
 * Parameters:
 *     reply_data: The :c:type:`zn_reply_data_t` to free.
 *
 */
void z_reply_data_free(struct z_owned_reply_data_t *reply_data);

void z_reply_free(struct z_owned_reply_t *reply);

struct z_reskey_t z_reskey_borrow(const struct z_owned_reskey_t *reskey);

bool z_reskey_check(const struct z_owned_reskey_t *reskey);

/**
 * Free a :c:type:`z_owned_reskey_t`.
 *
 * Parameters:
 *    b : The array to free.
 */
void z_reskey_free(struct z_owned_reskey_t *reskey);

/**
 * Free a :c:type:`z_owned_reskey_t`.
 *
 * Parameters:
 *    b : The array to free.
 */
struct z_owned_reskey_t z_reskey_new(unsigned long id, const char *suffix);

struct z_reskey_t z_reskey_new_borrowed(unsigned long id, const char *suffix);

/**
 * Create a resource key from a resource id.
 *
 * Parameters:
 *     id: The resource id.
 *
 * Returns:
 *     A new resource key.
 */
struct z_owned_reskey_t z_rid(unsigned long id);

/**
 * Create a resource key from a resource id and a suffix.
 *
 * Parameters:
 *     id: The resource id.
 *     suffix: The suffix, a NULL terminated string, copied on construction.
 *
 * Returns:
 *     A new resource key.
 */
struct z_owned_reskey_t z_rid_with_suffix(unsigned long id, const char *suffix);

/**
 * Create a resource key from a resource name.
 *
 * Parameters:
 *     id: The resource name, a NULL terminated string, copied on construction.
 *
 * Returns:
 *     A new resource key.
 */
struct z_owned_reskey_t z_rname(const char *name);

struct z_sample_t z_sample_borrow(const struct z_owned_sample_t *sample);

bool z_sample_check(const struct z_owned_sample_t *sample);

/**
 * Free a :c:type:`zn_sample_t` contained key and value.
 *
 * Parameters:
 *     sample: The :c:type:`zn_sample_t` to free.
 *
 */
void z_sample_free(struct z_owned_sample_t *sample);

/**
 * Scout for routers and/or peers.
 *
 * Parameters:
 *     what: A whatami bitmask of zenoh entities kind to scout for.
 *     config: A set of properties to configure the scouting.
 *     scout_period: The time that should be spent scouting before returnng the results.
 *
 * Returns:
 *     An array of :c:struct:`zn_hello_t` messages.
 */
struct z_owned_hello_array_t z_scout(unsigned int what,
                                     struct z_owned_config_t *config,
                                     unsigned long scout_period);

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
 *     key: The resource key of this reply.
 *     payload: The value of this reply.
 *     len: The length of the value of this reply.
 */
void z_send_reply(struct z_query_t *query,
                  const char *key,
                  const uint8_t *payload,
                  unsigned int len);

struct z_session_t z_session_borrow(const struct z_owned_session_t *s);

bool z_session_check(const struct z_owned_session_t *config);

/**
 * Free an array of NULL terminated strings and it's contained NULL terminated strings recursively.
 *
 * Parameters:
 *     strs: The array of NULL terminated strings to free.
 *
 */
void z_str_array_free(struct z_owned_str_array_t *strs);

const char *z_string_borrow(const struct z_owned_string_t *s);

bool z_string_check(const struct z_owned_string_t *s);

/**
 * Frees the passed z_string_t.
 */
void z_string_free(struct z_owned_string_t *s);

/**
 * Constructs a :c:type:`z_string_t` from a NULL terminated string.
 * The contents of `s` is copied.
 *
 * Parameters:
 *     s: The NULL terminated string.
 *
 * Returns:
 *     A new :c:type:`z_string_t`.
 */
struct z_owned_string_t z_string_new(const char *s);

/**
 * Create a default subscription info.
 */
struct z_subinfo_t z_subinfo_default(void);

const struct z_period_t *z_subinfo_period(const struct z_subinfo_t *info);

bool z_subscriber_check(const struct z_owned_subscriber_t *sub);

/**
 * Create a default :c:type:`zn_target_t`.
 */
struct z_target_t z_target_default(void);

/**
 * Undeclare a :c:type:`zn_publisher_t`.
 *
 * Parameters:
 *     sub: The :c:type:`zn_publisher_t` to undeclare.
 */
void z_unregister_publisher(struct z_owned_publisher_t *publ);

/**
 * Undeclare a :c:type:`zn_queryable_t`.
 *
 * Parameters:
 *     qable: The :c:type:`zn_queryable_t` to undeclare.
 */
void z_unregister_queryable(struct z_owned_queryable_t qable);

/**
 * Undeclare a :c:type:`zn_subscriber_t`.
 *
 * Parameters:
 *     sub: The :c:type:`zn_subscriber_t` to undeclare.
 */
void z_unregister_subscriber(struct z_owned_subscriber_t *sub);

/**
 * Write data.
 *
 * Parameters:
 *     session: The zenoh-net session.
 *     resource: The resource key to write.
 *     payload: The value to write.
 *     len: The length of the value to write.
 * Returns:
 *     ``0`` in case of success, ``1`` in case of failure.
 */
int z_write(struct z_session_t session,
            struct z_reskey_t reskey,
            const uint8_t *payload,
            unsigned int len);

/**
 * Write data with extended options.
 *
 * Parameters:
 *     session: The zenoh-net session.
 *     resource: The resource key to write.
 *     payload: The value to write.
 *     len: The length of the value to write.
 *     encoding: The encoding of the value.
 *     kind: The kind of value.
 *     congestion_control: The behavior to adopt in case of congestion while routing some data.
 * Returns:
 *     ``0`` in case of success, ``1`` in case of failure.
 */
int z_write_ext(struct z_session_t session,
                struct z_reskey_t reskey,
                const uint8_t *payload,
                unsigned int len,
                const struct z_write_options_t *options);

void z_write_options_set(struct z_write_options_t *options,
                         enum z_write_options_field_t key,
                         unsigned int value);

#endif /* ZENOH_GEN */
