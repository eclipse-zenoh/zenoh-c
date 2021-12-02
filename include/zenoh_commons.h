/**
 * The kind of consolidation that should be applied on replies to a :c:func:`z_get`.
 *
 *     - **z_consolidation_mode_t_FULL**: Guaranties unicity of replies. Optimizes bandwidth.
 *     - **z_consolidation_mode_t_LAZY**: Does not garanty unicity. Optimizes latency.
 *     - **z_consolidation_mode_t_NONE**: No consolidation.
 */
typedef enum z_consolidation_mode_t {
  z_consolidation_mode_t_FULL,
  z_consolidation_mode_t_LAZY,
  z_consolidation_mode_t_NONE,
} z_consolidation_mode_t;
/**
 * The different kind of options in a :c:type:`z_put_options_t`.
 *
 *     - **z_put_options_field_t_ENCODING**
 *     - **z_put_options_field_t_CONGESTION_CONTROL**
 *     - **z_put_options_field_t_KIND**
 *     - **z_put_options_field_t_PRIORITY**
 */
typedef enum z_put_options_field_t {
  z_put_options_field_t_ENCODING,
  z_put_options_field_t_CONGESTION_CONTROL,
  z_put_options_field_t_KIND,
  z_put_options_field_t_PRIORITY,
} z_put_options_field_t;
/**
 * The subscription reliability.
 *
 *     - **z_reliability_t_BEST_EFFORT**
 *     - **z_reliability_t_RELIABLE**
 */
typedef enum z_reliability_t {
  z_reliability_t_BEST_EFFORT,
  z_reliability_t_RELIABLE,
} z_reliability_t;
/**
 * The possible values of :c:member:`z_owned_reply_t.tag`
 *
 *     - **z_reply_t_Tag_DATA**: The reply contains some data.
 *     - **z_reply_t_Tag_FINAL**: The reply does not contain any data and indicates that there will be no more replies for this query.
 */
typedef enum z_reply_t_Tag {
  z_reply_t_Tag_DATA,
  z_reply_t_Tag_FINAL,
} z_reply_t_Tag;
/**
 * The subscription mode.
 *
 *     - **z_submode_t_PUSH**
 *     - **z_submode_t_PULL**
 */
typedef enum z_submode_t {
  z_submode_t_PUSH,
  z_submode_t_PULL,
} z_submode_t;
typedef struct z_query_t z_query_t;
/**
 * A borrowed array of bytes.
 */
typedef struct z_bytes_t {
  const uint8_t *start;
  size_t len;
} z_bytes_t;
/**
 * A zenoh-allocated array of bytes.
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
typedef struct z_owned_bytes_t {
  const uint8_t *start;
  size_t len;
} z_owned_bytes_t;
/**
 * An owned, zenoh-allocated, null-terminated, string.
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
typedef struct z_owned_string_t {
  const char *_borrow;
} z_owned_string_t;
/**
 * A borrowed null-terminated string.
 */
typedef const char *z_string_t;
/**
 * A borrowed key expression.
 *
 * Key expressions can identify a single key or a set of keys.
 * 
 * Examples : 
 *    - ``"/key/expression"``.
 *    - ``"/key/ex*"``.
 * 
 * Key expressions can be mapped to numerical ids through :c:func:`z_declare_expr`
 * for wire and computation efficiency.
 *
 * A key expression can be either:
 *   - A plain string expression.
 *   - A pure numerical id.
 *   - The combination of a numerical prefix and a string suffix.
 */
typedef struct z_keyexpr_t {
  unsigned long id;
  struct z_bytes_t suffix;
} z_keyexpr_t;
/**
 * A borrowed key expression.
 *
 * Key expressions can identify a single key or a set of keys.
 * 
 * Examples : 
 *    - ``"/key/expression"``.
 *    - ``"/key/ex*"``.
 * 
 * Key expressions can be mapped to numerical ids through :c:func:`z_declare_expr`
 * for wire and computation efficiency.
 *
 * A key expression can be either:
 *   - A plain string expression.
 *   - A pure numerical id.
 *   - The combination of a numerical prefix and a string suffix.
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
typedef struct z_owned_keyexpr_t {
  unsigned long id;
  struct z_owned_bytes_t suffix;
} z_owned_keyexpr_t;
/**
 * The possible values of :c:member:`z_target_t.tag`.
 *
 *     - **z_target_t_BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
 *     - **z_target_t_COMPLETE**: A set of complete queryables.
 *     - **z_target_t_ALL**: All matching queryables.
 *     - **z_target_t_NONE**: No queryables.
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
 * The zenoh queryables that should be target of a `z_get`.
 *
 * Members:
 *     `unsigned int kind`: A mask of queryable kinds.
 *     `z_target_t target`: The query target.
 */
typedef struct z_query_target_t {
  unsigned int kind;
  struct z_target_t target;
} z_query_target_t;
/**
 * The kind of consolidation that should be applied on replies to a :c:func:`z_get`
 * at the different stages of the reply process.
 *
 * Members:
 *   z_consolidation_mode_t first_routers: The consolidation mode to apply on first routers of the replies routing path.
 *   z_consolidation_mode_t last_router: The consolidation mode to apply on last router of the replies routing path.
 *   z_consolidation_mode_t reception: The consolidation mode to apply at reception of the replies.
 */
typedef struct z_query_consolidation_t {
  enum z_consolidation_mode_t first_routers;
  enum z_consolidation_mode_t last_router;
  enum z_consolidation_mode_t reception;
} z_query_consolidation_t;
/**
 * A zenoh-allocated data sample.
 *
 * A sample is the value associated to a given resource at a given point in time.
 *
 * Members:
 *   `z_owned_string_t key`: The resource key of this data sample.
 *   `z_owned_bytes_t value`: The value of this data sample.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by borrowing it using `z_X_borrow(&val)`.
 * The `z_borrow(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_borrow(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct z_owned_sample_t {
  struct z_owned_keyexpr_t key;
  struct z_owned_bytes_t value;
} z_owned_sample_t;
/**
 * An owned reply to a `z_get` (or `z_get_collect`).
 *
 * Members:
 *   `z_owned_sample_t data`: a :c:type:`z_sample_t` containing the key and value of the reply.
 *   `unsigned int source_kind`: The kind of the replier that sent this reply.
 *   `z_owned_bytes_t replier_id`: The id of the replier that sent this reply.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct z_owned_reply_data_t {
  struct z_owned_sample_t data;
  unsigned int source_kind;
  struct z_owned_bytes_t replier_id;
} z_owned_reply_data_t;
/**
 * An owned reply to a :c:func:`z_get`.
 *
 * Members:
 *   `z_reply_t_Tag tag`: Indicates if the reply contains data or if it's a FINAL reply.
 *   `z_owned_reply_data_t data`: The reply data if :c:member:`z_owned_reply_t.tag` equals :c:member:`z_reply_t_Tag.z_reply_t_Tag_DATA`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct z_owned_reply_t {
  enum z_reply_t_Tag tag;
  struct z_owned_reply_data_t data;
} z_owned_reply_t;
/**
 * A zenoh-allocated array of :c:type:`z_owned_reply_data_t`.
 *
 * Members:
 *   `char *const *val`: A pointer to the array.
 *   `unsigned int len`: The length of the array.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct z_owned_reply_data_array_t {
  const struct z_owned_reply_data_t *val;
  size_t len;
} z_owned_reply_data_array_t;
/**
 * An owned array of owned NULL terminated strings, allocated by zenoh.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct z_owned_str_array_t {
  const char *const *val;
  size_t len;
} z_owned_str_array_t;
/**
 * A zenoh-allocated hello message returned by a zenoh entity to a scout message sent with `z_scout`.
 *
 * Members:
 *   `unsigned int whatami`: The kind of zenoh entity.
 *   `z_owned_bytes_t pid`: The peer id of the scouted entity (empty if absent).
 *   `z_owned_str_array_t locators`: The locators of the scouted entity.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct z_owned_hello_t {
  unsigned int whatami;
  struct z_owned_bytes_t pid;
  struct z_owned_str_array_t locators;
} z_owned_hello_t;
/**
 * A zenoh-allocated array of `z_hello_t` messages.
 *
 * Members:
 *   const z_hello_t *val: A pointer to the array.
 *   unsigned int len: The length of the array.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct z_owned_hello_array_t {
  const struct z_owned_hello_t *val;
  size_t len;
} z_owned_hello_array_t;
/**
 * A borrowed data sample.
 *
 * A sample is the value associated to a given resource at a given point in time.
 *
 * Members:
 *   `z_string_t key`: The resource key of this data sample.
 *   `z_bytes_t value`: The value of this data sample.
 */
typedef struct z_sample_t {
  struct z_keyexpr_t key;
  struct z_bytes_t value;
} z_sample_t;
/**
 * The subscription period.
 * Equivalent of the rust `Option<zenoh::time::Period>` type, where `None` is represented by the `period` field being 0-valued.
 *
 * Members:
 *     `unsigned int origin`
 *     `unsigned int period`
 *     `unsigned int duration`
 */
typedef struct z_period_t {
  unsigned int origin;
  unsigned int period;
  unsigned int duration;
} z_period_t;
/**
 * Informations to be passed to :c:func:`z_subscribe` to configure the created :c:type:`z_owned_subscriber_t`.
 *
 * Members:
 *     `z_reliability_t reliability`: The subscription reliability.
 *     `z_submode_t mode`: The subscription mode.
 *     `z_period_t *period`: The subscription period.
 */
typedef struct z_subinfo_t {
  enum z_reliability_t reliability;
  enum z_submode_t mode;
  struct z_period_t period;
} z_subinfo_t;
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
/**
 * Returns `true` if `b` is valid.
 */
bool z_bytes_check(const struct z_owned_bytes_t *b);
/**
 * Frees `b` and invalidates it for double-free safety.
 */
void z_bytes_free(struct z_owned_bytes_t *b);
/**
 * Constructs a :c:type:`z_owned_bytes_t` of lengh `len` from the bytes
 * starting at address `start`.
 * The bytes from `start` are copied.
 */
struct z_owned_bytes_t z_bytes_new(const uint8_t *start, uintptr_t len);
/**
 * Closes a zenoh session. This frees and invalidates `session` for double-free safety.
 */
void z_close(struct z_owned_session_t *session);
/**
 * Returns a :c:type:`z_config_t` borrowed from `s`.
 */
struct z_config_t z_config_borrow(const struct z_owned_config_t *s);
/**
 * Returns `true` if `config` is valid.
 */
bool z_config_check(const struct z_owned_config_t *config);
/**
 * Constructs a default configuration client mode zenoh session.
 * If `peer` is not null, it is added to the configuration as remote peer.
 */
struct z_owned_config_t z_config_client(const char *const *peers, uintptr_t n_peers);
/**
 * Creates an default, zenoh-allocated, configuration.
 */
struct z_owned_config_t z_config_default(void);
/**
 * Creates an empty, zenoh-allocated, configuration.
 */
struct z_owned_config_t z_config_empty(void);
/**
 * Frees `config`, invalidating it for double-free safety.
 */
void z_config_free(struct z_owned_config_t *config);
/**
 * Constructs a configuration by parsing a file at `path`. Currently supported format is JSON5, a superset of JSON.
 */
struct z_owned_config_t z_config_from_file(const char *path);
/**
 * Reads a configuration from a properties-formated string, such as "mode=client;peer=tcp/127.0.0.1:7447".
 */
struct z_owned_config_t z_config_from_str(const char *s);
/**
 * Gets the property with the given integer key from the configuration.
 */
struct z_owned_string_t z_config_get(struct z_config_t config, unsigned int key);
/**
 * Gets the number of available keys for configuration.
 */
unsigned int z_config_len(struct z_config_t config);
/**
 * Return a new, zenoh-allocated, empty configuration.
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
struct z_owned_config_t z_config_new(void);
/**
 * Constructs a default configuration peer mode zenoh session.
 */
struct z_owned_config_t z_config_peer(void);
/**
 * Inserts a property with a given key to a properties map.
 * If a property with the same key already exists in the properties map, it is replaced.
 *
 * Parameters:
 *   config: A pointer to the properties map.
 *   key: The key of the property to add.
 *   value: The value of the property to add.
 */
void z_config_set(struct z_config_t config, unsigned long key, z_string_t value);
/**
 * Converts `config` into a properties-formated string, such as "mode=client;peer=tcp/127.0.0.1:7447".
 */
struct z_owned_string_t z_config_to_str(struct z_config_t config);
/**
 * Associates a numerical id with the given key expression. The id is returned as a `z_keyexpr_t` with a nullptr suffix.
 *
 * This numerical id will be used on the network to save bandwidth and
 * ease the retrieval of the concerned resource in the routing tables.
 */
struct z_keyexpr_t z_declare_expr(struct z_session_t session,
                                  struct z_owned_keyexpr_t *keyexpr);
/**
 * Declares a publication for the given key expression, returning `true` on success.
 *
 * Written resources that match the given key will only be sent on the network
 * if matching subscribers exist in the system.
 */
bool z_declare_publication(struct z_session_t session, struct z_keyexpr_t keyexpr);
/**
 * Constructs a key expression from a string expression. `name`'s content is copied.
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
struct z_owned_keyexpr_t z_expr(const char *name);
/**
 * Query data from the matching queryables in the system.
 * Replies are provided through a callback function.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression matching resources to query.
 *     predicate: An indication to matching queryables about the queried data.
 *     target: The kind of queryables that should be target of this query.
 *     consolidation: The kind of consolidation that should be applied on replies.
 *     callback: The callback function that will be called on reception of replies for this query.
 *     arg: A pointer that will be passed to the **callback** on each call.
 */
void z_get(struct z_session_t session,
           struct z_keyexpr_t keyexpr,
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
 *     session: The zenoh session.
 *     keyexpr: The key expression matching resources to query.
 *     predicate: An indication to matching queryables about the queried data.
 *     target: The kind of queryables that should be target of this query.
 *     consolidation: The kind of consolidation that should be applied on replies.
 *
 * Returns:
 *    An array containing all the replies for this query.
 */
struct z_owned_reply_data_array_t z_get_collect(struct z_session_t session,
                                                struct z_keyexpr_t keyexpr,
                                                const char *predicate,
                                                struct z_query_target_t target,
                                                struct z_query_consolidation_t consolidation);
/**
 * Returns `true` if `hellos` is valid.
 */
bool z_hello_array_check(const struct z_owned_hello_array_t *hellos);
/**
 * Frees `hellos`, invalidating it for double-free safety.
 */
void z_hello_array_free(struct z_owned_hello_array_t *hellos);
/**
 * Returns `true` if `hello` is valid.
 */
bool z_hello_check(const struct z_owned_hello_t *hello);
/**
 * Frees `hello`, invalidating it for double-free safety.
 */
void z_hello_free(struct z_owned_hello_t *hello);
/**
 * Constructs a key expression from an expression id.
 * Since id-only kes expressions don't need destruction, a `z_keyexpr_t` is returned instead of its owned variant.
 */
struct z_keyexpr_t z_id(unsigned long id);
/**
 * Constructs a key expression from an expression id and a suffix. `suffix`'s content is copied.
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
struct z_owned_keyexpr_t z_id_with_suffix(unsigned long id,
                                          const char *suffix);
/**
 * Gets informations about an zenoh session.
 */
struct z_owned_info_t z_info(struct z_session_t session);
/**
 * Gets informations about an zenoh session as a properties-formatted string.
 */
struct z_owned_string_t z_info_as_str(struct z_session_t session);
/**
 * Returns a :c:type:`z_info_t` borrowed from `info`.
 */
struct z_info_t z_info_borrow(const struct z_owned_info_t *info);
/**
 * Returns `true` if `info` is valid.
 */
bool z_info_check(const struct z_owned_info_t *info);
/**
 * Frees `info`'s memory, while invalidating `info` for double-free-safety.
 */
void z_info_free(struct z_owned_info_t *info);
/**
 * Returns the information associated with `key` if it exists.
 * If it doesn't, the returned value is invalid, and doesn't need freeing.
 */
struct z_owned_string_t z_info_get(struct z_info_t info, uint64_t key);
/**
 * Initialises the zenoh runtime logger
 */
void z_init_logger(void);
/**
 * Returns a :c:type:`z_keyexpr_t` borrowed from `keyexpr`.
 */
struct z_keyexpr_t z_keyexpr_borrow(const struct z_owned_keyexpr_t *keyexpr);
/**
 * Returns `true` if `keyexpr` is valid.
 */
bool z_keyexpr_check(const struct z_owned_keyexpr_t *keyexpr);
/**
 * Frees `keyexpr` and invalidates it for double-free safety.
 */
void z_keyexpr_free(struct z_owned_keyexpr_t *keyexpr);
/**
 * Constructs a zenoh-owned key expression. `suffix`'s contents will be copied.
 */
struct z_owned_keyexpr_t z_keyexpr_new(unsigned long id, const char *suffix);
/**
 * Constructs a borrowed key expression. The constructed value is valid as long as `suffix` is.
 */
struct z_keyexpr_t z_keyexpr_new_borrowed(unsigned long id, const char *suffix);
/**
 * Opens a zenoh session. Should the session opening fail, `z_check`ing the returned value will return `false`.
 */
struct z_owned_session_t z_open(struct z_owned_config_t *config);
/**
 * Pull data for a pull mode :c:type:`z_owned_subscriber_t`. The pulled data will be provided
 * by calling the **callback** function provided to the :c:func:`z_subscribe` function.
 *
 * Parameters:
 *     sub: The :c:type:`z_owned_subscriber_t` to pull from.
 */
void z_pull(const struct z_owned_subscriber_t *sub);
/**
 * Write data.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression to write.
 *     payload: The value to write.
 *     len: The length of the value to write.
 * Returns:
 *     ``0`` in case of success, ``1`` in case of failure.
 */
int z_put(struct z_session_t session,
          struct z_keyexpr_t keyexpr,
          const uint8_t *payload,
          unsigned int len);
/**
 * Write data with extended options.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression to write.
 *     payload: The value to write.
 *     len: The length of the value to write.
 *     options: The write options
 * Returns:
 *     ``0`` in case of success, ``1`` in case of failure.
 */
int z_put_ext(struct z_session_t session,
              struct z_keyexpr_t keyexpr,
              const uint8_t *payload,
              unsigned int len,
              const struct z_put_options_t *options);
/**
 * Constructs the default value for write options
 */
struct z_put_options_t z_put_options_default(void);
/**
 * Sets the value for the required field of a `z_put_options_t`.
 * Returns `false` if the value insertion failed.
 */
bool z_put_options_set(struct z_put_options_t *options,
                       enum z_put_options_field_t key,
                       unsigned int value);
/**
 * Creates a default :c:type:`z_query_consolidation_t`.
 */
struct z_query_consolidation_t z_query_consolidation_default(void);
/**
 * Gets the key expression of a received query as a non null-terminated string.
 */
struct z_keyexpr_t z_query_key_expr(const struct z_query_t *query);
/**
 * Gets the predicate of a received query as a non null-terminated string.
 */
struct z_bytes_t z_query_predicate(const struct z_query_t *query);
/**
 * Creates a default `z_query_target_t`.
 */
struct z_query_target_t z_query_target_default(void);
/**
 * Returns `true` if `qable` is valid.
 */
bool z_queryable_check(const struct z_owned_queryable_t *qable);
/**
 * Close a `z_owned_queryable_t`, freeing it and invalidating it for doube-free safety.
 *
 * Parameters:
 *     qable: The :c:type:`z_owned_queryable_t` to close.
 */
void z_queryable_close(struct z_owned_queryable_t *qable);
/**
 * Creates a Queryable for the given key expression.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression the Queryable will reply to.
 *     kind: The kind of Queryable.
 *     callback: The callback function that will be called each time a matching query is received.
 *     arg: A pointer that will be passed to the **callback** on each call.
 *
 * Returns:
 *    The created :c:type:`z_owned_queryable_t` or null if the creation failed.
 */
struct z_owned_queryable_t z_queryable_new(struct z_session_t session,
                                           struct z_keyexpr_t keyexpr,
                                           unsigned int kind,
                                           void (*callback)(const struct z_query_t*, const void*),
                                           void *arg);
/**
 * Returns `true` if `reply` is valid.
 */
bool z_reply_check(const struct z_owned_reply_t *reply);
bool z_reply_data_array_check(const struct z_owned_reply_data_array_t *replies);
/**
 * Free a :c:type:`z_owned_reply_data_array_t` and it's contained replies.
 *
 * Parameters:
 *     replies: The :c:type:`z_owned_reply_data_array_t` to free.
 *
 */
void z_reply_data_array_free(struct z_owned_reply_data_array_t *replies);
/**
 * Returns `true` if `reply_data` is valid.
 */
bool z_reply_data_check(const struct z_owned_reply_data_t *reply_data);
/**
 * Frees `reply_data`, invalidating it for double-free safety.
 */
void z_reply_data_free(struct z_owned_reply_data_t *reply_data);
/**
 * Frees `reply`, invalidating it for double-free safety.
 */
void z_reply_free(struct z_owned_reply_t *reply);
/**
 * Returns a :c:type:`z_sample_t` borrowed from `sample`.
 */
struct z_sample_t z_sample_borrow(const struct z_owned_sample_t *sample);
/**
 * Returns `true` if `sample` is valid.
 */
bool z_sample_check(const struct z_owned_sample_t *sample);
/**
 * Frees `sample`, invalidating it for double-free safety.
 */
void z_sample_free(struct z_owned_sample_t *sample);
/**
 * Scout for routers and/or peers.
 *
 * Parameters:
 *     `what`: A whatami bitmask of zenoh entities kind to scout for.
 *     `config`: A set of properties to configure the scouting.
 *     `scout_period`: The time that should be spent scouting before returning the results.
 *
 * Returns:
 *     An array of `z_hello_t` messages.
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
 *     key: The key of this reply.
 *     payload: The value of this reply.
 *     len: The length of the value of this reply.
 */
void z_send_reply(const struct z_query_t *query,
                  const char *key,
                  const uint8_t *payload,
                  unsigned int len);
/**
 * Returns a :c:type:`z_session_t` borrowed from `s`.
 */
struct z_session_t z_session_borrow(const struct z_owned_session_t *s);
/**
 * Returns `true`if `session`is valid.
 */
bool z_session_check(const struct z_owned_session_t *session);
/**
 * Returns `true` if `strs` is valid
 */
bool z_str_array_check(const struct z_owned_str_array_t *strs);
/**
 * Frees `strs` and invalidates it for double-free safety.
 */
void z_str_array_free(struct z_owned_str_array_t *strs);
/**
 * Returns a :c:type:`z_string_t` borrowed from `s`.
 */
z_string_t z_string_borrow(const struct z_owned_string_t *s);
/**
 * Returns `true` if `s` is valid
 */
bool z_string_check(const struct z_owned_string_t *s);
/**
 * Frees `s`'s memory, while invalidating `s` for double-free-safety.
 */
void z_string_free(struct z_owned_string_t *s);
/**
 * Construct a :c:type:`z_owned_string_t` from a NULL terminated string.
 * The contents of `s` are copied.
 */
struct z_owned_string_t z_string_new(const char *s);
/**
 * Create a default subscription info.
 */
struct z_subinfo_t z_subinfo_default(void);
/**
 * Returns the subscription period from `info`.
 */
const struct z_period_t *z_subinfo_period(const struct z_subinfo_t *info);
/**
 * Subscribes to the given key expression.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression to subscribe.
 *     sub_info: The :c:type:`z_subinfo_t` to configure the subscriber.
 *     callback: The callback function that will be called each time a data matching the subscribed expression is received.
 *     arg: A pointer that will be passed to the **callback** on each call.
 *
 * Returns:
 *    A :c:type:`z_owned_subscriber_t`.
 *
 *    To check if the subscription succeeded and if the subscriber is still valid,
 *    you may use `z_subscriber_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 *
 *    Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 *    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 *    After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 */
struct z_owned_subscriber_t z_subscribe(struct z_session_t session,
                                        struct z_keyexpr_t keyexpr,
                                        struct z_subinfo_t sub_info,
                                        void (*callback)(const struct z_sample_t*, const void*),
                                        void *arg);
/**
 * Returns `true` if `sub` is valid.
 */
bool z_subscriber_check(const struct z_owned_subscriber_t *sub);
/**
 * Unsubscribes from the passed `sub`, freeing it and invalidating it for double-free safety.
 */
void z_subscriber_close(struct z_owned_subscriber_t *sub);
/**
 * Create a default :c:type:`z_target_t`.
 */
struct z_target_t z_target_default(void);
/**
 * Unbinds the numerical id key generated by a call to :c:func:`z_declare_expr`.
 */
void z_undeclare_expr(struct z_session_t session, struct z_keyexpr_t keyexpr);
/**
 * Undeclares a publication for the given key expression.
 */
void z_undeclare_publication(struct z_session_t session, struct z_keyexpr_t keyexpr);
