typedef enum z_congestion_control_t {
  Z_CONGESTION_CONTROL_BLOCK,
  Z_CONGESTION_CONTROL_DROP,
} z_congestion_control_t;
/**
 * The kind of consolidation that should be applied on replies to a :c:func:`z_get`.
 *
 *     - **z_consolidation_mode_t_FULL**: Guaranties unicity of replies. Optimizes bandwidth.
 *     - **z_consolidation_mode_t_LAZY**: Does not garanty unicity. Optimizes latency.
 *     - **z_consolidation_mode_t_NONE**: No consolidation.
 */
typedef enum z_consolidation_mode_t {
  Z_CONSOLIDATION_MODE_FULL,
  Z_CONSOLIDATION_MODE_LAZY,
  Z_CONSOLIDATION_MODE_NONE,
} z_consolidation_mode_t;
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
 * The possible values of :c:member:`z_query_target_t.tag`.
 *
 *     - **z_query_target_t_BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
 *     - **z_query_target_t_COMPLETE**: A set of complete queryables.
 *     - **z_query_target_t_ALL**: All matching queryables.
 *     - **z_query_target_t_NONE**: No queryables.
 */
typedef enum z_query_target_t {
  Z_QUERY_TARGET_BEST_MATCHING,
  Z_QUERY_TARGET_ALL,
  Z_QUERY_TARGET_NONE,
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
/**
 * An array of bytes.
 */
typedef struct z_bytes_t {
  const uint8_t *start;
  size_t len;
} z_bytes_t;
typedef struct z_query_t {
  const void *_0;
} z_query_t;
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
typedef struct z_owned_closure_query_t {
  void *context;
  void (*call)(struct z_query_t, const void *context);
  void (*drop)(void*);
} z_owned_closure_query_t;
/**
 * An owned reply to a `z_get` (or `z_get_collect`).
 *
 * Members:
 *   `z_owned_sample_t sample`: a :c:type:`z_sample_t` containing the key and value of the reply.
 *   `z_owned_bytes_t replier_id`: The id of the replier that sent this reply.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct z_owned_reply_t {
  uint64_t _align[5];
  uintptr_t _padding[18];
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
typedef struct z_owned_closure_reply_t {
  void *context;
  void (*call)(struct z_owned_reply_t*, const void*);
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
typedef struct z_keyexpr_t {
  uint64_t _align[2];
  uintptr_t _padding[2];
} z_keyexpr_t;
/**
 * The encoding of a payload, in a MIME-like format.
 *
 * For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
 *
 * `suffix` MUST be a valid UTF-8 string.
 */
typedef struct z_encoding_t {
  enum z_encoding_prefix_t prefix;
  struct z_bytes_t suffix;
} z_encoding_t;
typedef struct z_timestamp_t {
  uint64_t time;
  struct z_bytes_t id;
} z_timestamp_t;
/**
 * A data sample.
 *
 * A sample is the value associated to a given resource at a given point in time.
 *
 * Members:
 *   `z_string_t key`: The resource key of this data sample.
 *   `z_bytes_t value`: The value of this data sample.
 */
typedef struct z_sample_t {
  struct z_keyexpr_t keyexpr;
  struct z_bytes_t payload;
  struct z_encoding_t encoding;
  enum z_sample_kind_t kind;
  struct z_timestamp_t timestamp;
} z_sample_t;
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
typedef struct z_owned_closure_sample_t {
  void *context;
  void (*call)(const struct z_sample_t*, const void *context);
  void (*drop)(void*);
} z_owned_closure_sample_t;
/**
 * Represents a Zenoh ID.
 *
 * In general, valid Zenoh IDs are LSB-first 128bit unsigned and non-zero integers.
 */
typedef struct z_id_t {
  uint8_t id[16];
} z_id_t;
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
typedef struct z_owned_closure_zid_t {
  void *context;
  void (*call)(const struct z_id_t*, const void*);
  void (*drop)(void*);
} z_owned_closure_zid_t;
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
 * A key expression can be either:
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
typedef struct z_owned_keyexpr_t {
  uint64_t _align[2];
  uintptr_t _padding[2];
} z_owned_keyexpr_t;
typedef struct z_owned_publisher_t {
  uint64_t _align[1];
  uintptr_t _padding[6];
} z_owned_publisher_t;
/**
 * The options for a publisher.
 *
 * Note that `local_routing` has 3 legal values: 0 which disables it, 1 which enables it, and -1 which leaves it up to the session.
 * Other values will behave like -1, but are considered UB.
 */
typedef struct z_publisher_options_t {
  int8_t local_routing;
  enum z_congestion_control_t congestion_control;
  enum z_priority_t priority;
} z_publisher_options_t;
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
typedef struct z_owned_pull_subscriber_t {
  uintptr_t _0[1];
} z_owned_pull_subscriber_t;
/**
 * Declare a subscriber for a given key expression.
 *
 * Members:
 *     `z_reliability_t reliability`: The subscription reliability.
 *     `void *cargs`: A pointer that will be passed to the **callback** at each call.
 *
 */
typedef struct z_subscriber_options_t {
  enum z_reliability_t reliability;
} z_subscriber_options_t;
typedef struct z_queryable_options_t {
  bool complete;
} z_queryable_options_t;
typedef struct z_owned_encoding_t {
  enum z_encoding_prefix_t prefix;
  struct z_bytes_t suffix;
  bool _dropped;
} z_owned_encoding_t;
/**
 * The kind of consolidation that should be applied on replies to a :c:func:`z_get`
 * at the different stages of the reply process.
 *
 * Members:
 *   z_consolidation_mode_t first_routers: The consolidation mode to apply on first routers of the replies routing path.
 *   z_consolidation_mode_t last_router: The consolidation mode to apply on last router of the replies routing path.
 *   z_consolidation_mode_t reception: The consolidation mode to apply at reception of the replies.
 */
typedef struct z_consolidation_strategy_t {
  enum z_consolidation_mode_t first_routers;
  enum z_consolidation_mode_t last_router;
  enum z_consolidation_mode_t reception;
} z_consolidation_strategy_t;
/**
 * The replies consolidation strategy to apply on replies to a :c:func:`z_get`.
 */
typedef enum z_query_consolidation_tag_t {
  Z_QUERY_CONSOLIDATION_AUTO,
  Z_QUERY_CONSOLIDATION_MANUAL,
} z_query_consolidation_tag_t;
typedef struct z_query_consolidation_t {
  z_query_consolidation_tag_t tag;
  union {
    struct {
      struct z_consolidation_strategy_t manual;
    };
  };
} z_query_consolidation_t;
typedef struct z_get_options_t {
  enum z_query_target_t target;
  struct z_query_consolidation_t consolidation;
} z_get_options_t;
typedef struct z_publisher_put_options_t {
  struct z_encoding_t encoding;
} z_publisher_put_options_t;
/**
 * Options passed to the :c:func:`z_put_ext` function.
 */
typedef struct z_put_options_t {
  struct z_encoding_t encoding;
  enum z_congestion_control_t congestion_control;
  enum z_priority_t priority;
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
typedef struct z_owned_reply_channel_closure_t {
  void *context;
  bool (*call)(struct z_owned_reply_t*, const void*);
  void (*drop)(void*);
} z_owned_reply_channel_closure_t;
/**
 * A pair of closures, the `send` one accepting
 */
typedef struct z_owned_reply_channel_t {
  struct z_owned_closure_reply_t send;
  struct z_owned_reply_channel_closure_t recv;
} z_owned_reply_channel_t;
typedef struct z_value_t {
  struct z_bytes_t payload;
  struct z_encoding_t encoding;
} z_value_t;
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
extern const char *Z_CONFIG_LOCAL_ROUTING_KEY;
/**
 * Returns `true` if `b` is initialized.
 */
bool z_bytes_check(const struct z_bytes_t *b);
/**
 * Closes a zenoh session. This drops and invalidates `session` for double-drop safety.
 */
void z_close(struct z_owned_session_t *session);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
void z_closure_query_call(const struct z_owned_closure_query_t *closure, struct z_query_t query);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
void z_closure_query_drop(struct z_owned_closure_query_t *closure);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
void z_closure_reply_call(const struct z_owned_closure_reply_t *closure,
                          struct z_owned_reply_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
void z_closure_reply_drop(struct z_owned_closure_reply_t *closure);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
void z_closure_sample_call(const struct z_owned_closure_sample_t *closure,
                           const struct z_sample_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
void z_closure_sample_drop(struct z_owned_closure_sample_t *closure);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
void z_closure_zid_call(const struct z_owned_closure_zid_t *closure, const struct z_id_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
void z_closure_zid_drop(struct z_owned_closure_zid_t *closure);
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
 * Creates a default, zenoh-allocated, configuration.
 */
struct z_owned_config_t z_config_default(void);
/**
 * Frees `config`, invalidating it for double-drop safety.
 */
void z_config_drop(struct z_owned_config_t *config);
/**
 * Creates an empty, zenoh-allocated, configuration.
 */
struct z_owned_config_t z_config_empty(void);
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
const char *z_config_get(struct z_config_t config, const char *key);
/**
 * Inserts a JSON-serialized `value` at the `key` position of the configuration.
 *
 * Returns `true` if insertion was succesful, `false` otherwise.
 */
bool z_config_insert_json(struct z_config_t config, const char *key, const char *value);
/**
 * Returns a :c:type:`z_config_t` loaned from `s`.
 */
struct z_config_t z_config_loan(const struct z_owned_config_t *s);
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
struct z_owned_config_t z_config_new(void);
/**
 * Constructs a default configuration peer mode zenoh session.
 */
struct z_owned_config_t z_config_peer(void);
/**
 * Converts `config` into a properties-formated string, such as "mode=client;peer=tcp/127.0.0.1:7447".
 */
char *z_config_to_string(struct z_config_t config);
/**
 * Declare a key expression. The id is returned as a :c:type:`z_keyexpr_t` with a nullptr suffix.
 *
 * This numerical id will be used on the network to save bandwidth and
 * ease the retrieval of the concerned resource in the routing tables.
 */
struct z_owned_keyexpr_t z_declare_keyexpr(struct z_session_t session, struct z_keyexpr_t keyexpr);
/**
 * Declares a publication for the given key expression, returning `true` on success.
 *
 * Written resources that match the given key will only be sent on the network
 * if matching subscribers exist in the system.
 */
struct z_owned_publisher_t z_declare_publisher(struct z_session_t session,
                                               struct z_keyexpr_t keyexpr,
                                               const struct z_publisher_options_t *options);
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
 *    ```
 *    z_owned_subscriber_t sub = z_declare_pull_subscriber(z_loan(s), z_keyexpr(expr), callback, NULL);
 *    ```
 *
 *    is equivalent to initializing and passing the default subscriber options:
 *
 *    ```
 *    z_subscriber_options_t opts = z_subscriber_options_default();
 *    z_owned_subscriber_t sub = z_declare_pull_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
 *    ```
 *
 *    Passing custom arguments to the **callback** can be done by defining a custom structure:
 *
 *    ```
 *    typedef struct {
 *      z_keyexpr_t forward;
 *      z_session_t session;
 *    } myargs_t;
 *
 *    void callback(const z_sample_t sample, const void *arg)
 *    {
 *      myargs_t *myargs = (myargs_t *)arg;
 *      z_put(myargs->session, myargs->forward, sample->value, NULL);
 *    }
 *
 *    int main() {
 *      myargs_t cargs = {
 *        forward = z_keyexpr("forward"),
 *        session = s,
 *      };
 *      z_subscriber_options_t opts = z_subscriber_options_default();
 *      opts.cargs = (void *)&cargs;
 *      z_owned_subscriber_t sub = z_declare_pull_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
 *    }
 *    ```
 */
struct z_owned_pull_subscriber_t z_declare_pull_subscriber(struct z_session_t session,
                                                           struct z_keyexpr_t keyexpr,
                                                           struct z_owned_closure_sample_t *callback,
                                                           const struct z_subscriber_options_t *opts);
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
 *    The created :c:type:`z_owned_queryable_t` or null if the creation failed.
 */
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
 *    ```
 *    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(expr), callback, NULL);
 *    ```
 *
 *    is equivalent to initializing and passing the default subscriber options:
 *
 *    ```
 *    z_subscriber_options_t opts = z_subscriber_options_default();
 *    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
 *    ```
 *
 *    Passing custom arguments to the **callback** can be done by defining a custom structure:
 *
 *    ```
 *    typedef struct {
 *      z_keyexpr_t forward;
 *      z_session_t session;
 *    } myargs_t;
 *
 *    void callback(const z_sample_t sample, const void *arg)
 *    {
 *      myargs_t *myargs = (myargs_t *)arg;
 *      z_put(myargs->session, myargs->forward, sample->value, NULL);
 *    }
 *
 *    int main() {
 *      myargs_t cargs = {
 *        forward = z_keyexpr("forward"),
 *        session = s,
 *      };
 *      z_subscriber_options_t opts = z_subscriber_options_default();
 *      opts.cargs = (void *)&cargs;
 *      z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(expr), callback, &opts);
 *    }
 *    ```
 */
struct z_owned_subscriber_t z_declare_subscriber(struct z_session_t session,
                                                 struct z_keyexpr_t keyexpr,
                                                 struct z_owned_closure_sample_t *callback,
                                                 const struct z_subscriber_options_t *opts);
/**
 * Returns `true` if `encoding` is valid.
 */
bool z_encoding_check(const struct z_owned_encoding_t *encoding);
/**
 * Frees `encoding`, invalidating it for double-drop safety.
 */
struct z_encoding_t z_encoding_default(void);
/**
 * Frees `encoding`, invalidating it for double-drop safety.
 */
void z_encoding_drop(struct z_owned_encoding_t *encoding);
/**
 * Returns a :c:type:`z_encoding_t` loaned from `encoding`.
 */
struct z_encoding_t z_encoding_loan(const struct z_owned_encoding_t *encoding);
/**
 * Query data from the matching queryables in the system.
 * Replies are provided through a callback function.
 *
 * Parameters:
 *     session: The zenoh session.
 *     keyexpr: The key expression matching resources to query.
 *     predicate: An indication to matching queryables about the queried data.
 *     callback: The callback function that will be called on reception of replies for this query.
 *               Note that the `reply` parameter of the callback is passed by mutable reference,
 *               but WILL be dropped once your callback exits to help you avoid memory leaks.
 *               If you'd rather take ownership, please refer to the documentation of `z_reply_null`
 *     options: additional options for the get.
 */
bool z_get(struct z_session_t session,
           struct z_keyexpr_t keyexpr,
           const char *predicate,
           struct z_owned_closure_reply_t *callback,
           const struct z_get_options_t *options);
struct z_get_options_t z_get_options_default(void);
/**
 * Fetches the Zenoh IDs of all connected peers.
 *
 * `callback` will be called once for each ID, is guaranteed to never be called concurrently,
 * and is guaranteed to be dropped before this function exits.
 */
void z_info_peers_zid(struct z_session_t session, struct z_owned_closure_zid_t *callback);
/**
 * Fetches the Zenoh IDs of all connected routers.
 *
 * `callback` will be called once for each ID, is guaranteed to never be called concurrently,
 * and is guaranteed to be dropped before this function exits.
 */
void z_info_routers_zid(struct z_session_t session, struct z_owned_closure_zid_t *callback);
/**
 * Returns the local Zenoh ID.
 *
 * Unless the `session` is invalid, that ID is guaranteed to be non-zero.
 * In other words, this function returning an array of 16 zeros means you failed
 * to pass it a valid session.
 */
struct z_id_t z_info_zid(struct z_session_t session);
/**
 * Initialises the zenoh runtime logger
 */
void z_init_logger(void);
/**
 * Constructs a :c:type:`z_keyexpr_t` departing from a string.
 * It is a loaned key expression that aliases `name`.
 */
struct z_keyexpr_t z_keyexpr(const char *name);
/**
 * Returns `true` if `keyexpr` is valid.
 */
bool z_keyexpr_check(const struct z_owned_keyexpr_t *keyexpr);
/**
 * Performs string concatenation and returns the result as a `z_owned_keyexpr_t`.
 * In case of error, the return value will be set to its invalidated state.
 *
 * You should probably prefer `z_keyexpr_join` as Zenoh may then take advantage of the hierachical separation it inserts.
 *
 * To avoid odd behaviors, concatenating a key expression starting with `*` to one ending with `*` is forbidden by this operation,
 * as this would extremely likely cause bugs.
 */
struct z_owned_keyexpr_t z_keyexpr_concat(struct z_keyexpr_t left,
                                          const char *right_start,
                                          uintptr_t right_len);
/**
 * Frees `keyexpr` and invalidates it for double-drop safety.
 */
void z_keyexpr_drop(struct z_owned_keyexpr_t *keyexpr);
/**
 * Returns `1` if `left` and `right` define equal sets.
 */
bool z_keyexpr_equals(struct z_keyexpr_t left, struct z_keyexpr_t right);
/**
 * Returns `1` if the set defined by `left` contains every key belonging to the set defined by `right`.
 */
bool z_keyexpr_includes(struct z_keyexpr_t left,
                        struct z_keyexpr_t right);
/**
 * Returns `1` if `left` and `right` define sets that have at least one key in common.
 */
bool z_keyexpr_intersects(struct z_keyexpr_t left, struct z_keyexpr_t right);
/**
 * Performs path-joining (automatically inserting) and returns the result as a `z_owned_keyexpr_t`.
 * In case of error, the return value will be set to its invalidated state.
 */
struct z_owned_keyexpr_t z_keyexpr_join(struct z_keyexpr_t left, struct z_keyexpr_t right);
/**
 * Returns a :c:type:`z_keyexpr_t` loaned from :c:type:`z_owned_keyexpr_t`.
 */
struct z_keyexpr_t z_keyexpr_loan(const struct z_owned_keyexpr_t *keyexpr);
/**
 * Constructs a :c:type:`z_keyexpr_t` departing from a string, copying the passed string.
 */
struct z_owned_keyexpr_t z_keyexpr_new(const char *name);
/**
 * Constructs a null-terminated string departing from a :c:type:`z_keyexpr_t`.
 * The user is responsible of droping the allocated string being returned.
 */
char *z_keyexpr_to_string(struct z_keyexpr_t keyexpr);
/**
 * Constructs a :c:type:`z_keyexpr_t` departing from a string without checking any of `z_keyexpr_t`'s assertions:
 * - `name` MUST be valid UTF8.
 * - `name` MUST follow the Key Expression specification, ie:
 *   - MUST NOT contain `//`, MUST NOT start nor end with `/`, MUST NOT contain any of the characters `?#$`.
 *   - any instance of `**` may only be lead or followed by `/`.
 *   - the key expression must have canon form.
 *
 * It is a loaned key expression that aliases `name`.
 */
struct z_keyexpr_t z_keyexpr_unchecked(const char *name);
/**
 * Returns `true` if `keyexpr` is valid.
 */
bool z_loaned_keyexpr_check(const struct z_keyexpr_t *keyexpr);
/**
 * Opens a zenoh session. Should the session opening fail, `z_check`ing the returned value will return `false`.
 */
struct z_owned_session_t z_open(struct z_owned_config_t *config);
/**
 * Sends a `DELETE` message onto the publisher's key expression.
 *
 * Returns 0 if successful.
 */
int8_t z_publisher_delete(const struct z_owned_publisher_t *publisher);
struct z_publisher_options_t z_publisher_options_default(void);
/**
 * Sends a `PUT` message onto the publisher's key expression.
 *
 * Returns 0 if successful.
 *
 * You may specify the payload's encoding through the options.
 */
int8_t z_publisher_put(const struct z_owned_publisher_t *publisher,
                       const uint8_t *payload,
                       uintptr_t len,
                       const struct z_publisher_put_options_t *options);
/**
 * Pull data for a pull mode :c:type:`z_owned_subscriber_t`. The pulled data will be provided
 * by calling the **callback** function provided to the :c:func:`z_subscribe` function.
 *
 * Parameters:
 *     sub: The :c:type:`z_owned_subscriber_t` to pull from.
 */
int8_t z_pull(const struct z_owned_pull_subscriber_t *sub);
/**
 * Returns `true` if `sub` is valid.
 */
bool z_pull_subscriber_check(const struct z_owned_pull_subscriber_t *sub);
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
int z_put(struct z_session_t session,
          struct z_keyexpr_t keyexpr,
          const uint8_t *payload,
          size_t len,
          const struct z_put_options_t *opts);
/**
 * Constructs the default value for write options
 */
struct z_put_options_t z_put_options_default(void);
/**
 * Automatic query consolidation strategy selection.
 *
 * A query consolidation strategy will automatically be selected depending
 * the query selector. If the selector contains time range properties,
 * no consolidation is performed. Otherwise the
 * :c:func:`z_query_consolidation_reception` strategy is used.
 */
struct z_query_consolidation_t z_query_consolidation_auto(void);
/**
 * Creates a default :c:type:`z_query_consolidation_t`.
 */
struct z_query_consolidation_t z_query_consolidation_default(void);
/**
 * Full consolidation performed everywhere.
 *
 * This mode optimizes bandwidth on all links in the system
 * but will provide a very poor latency.
 */
struct z_query_consolidation_t z_query_consolidation_full(void);
/**
 * Full consolidation performed on last router and at reception.
 *
 * This mode offers a good latency while optimizing bandwidth on
 * the last transport link between the router and the application.
 */
struct z_query_consolidation_t z_query_consolidation_last_router(void);
/**
 * Lazy consolidation performed at all stages.
 *
 * This strategy offers the best latency. Replies are directly
 * transmitted to the application when received without needing
 * to wait for all replies.
 *
 * This mode does not garantie that there will be no duplicates.
 */
struct z_query_consolidation_t z_query_consolidation_lazy(void);
/**
 * No consolidation performed.
 *
 * This is usefull when querying timeseries data bases or
 * when using quorums.
 */
struct z_query_consolidation_t z_query_consolidation_none(void);
/**
 * Full consolidation performed at reception.
 *
 * This is the default strategy. It offers the best latency while
 * garantying that there will be no duplicates.
 */
struct z_query_consolidation_t z_query_consolidation_reception(void);
/**
 * Get a query's key by aliasing it.
 */
struct z_keyexpr_t z_query_keyexpr(struct z_query_t query);
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
void z_query_reply(struct z_query_t query,
                   struct z_keyexpr_t key,
                   const uint8_t *payload,
                   uintptr_t len);
/**
 * Get a query's value selector by aliasing it.
 */
struct z_bytes_t z_query_value_selector(struct z_query_t query);
/**
 * Returns `true` if `qable` is valid.
 */
bool z_queryable_check(const struct z_owned_queryable_t *qable);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
bool z_reply_channel_closure_call(const struct z_owned_reply_channel_closure_t *closure,
                                  struct z_owned_reply_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
void z_reply_channel_closure_drop(struct z_owned_reply_channel_closure_t *closure);
void z_reply_channel_drop(struct z_owned_reply_channel_t *channel);
/**
 * Returns `true` if `reply_data` is valid.
 */
bool z_reply_check(const struct z_owned_reply_t *reply_data);
/**
 * Frees `reply_data`, invalidating it for double-drop safety.
 */
void z_reply_drop(struct z_owned_reply_t *reply_data);
/**
 * Yields the contents of the reply by asserting it indicates a failure.
 *
 * You should always make sure that `z_reply_is_ok()` returns `false` before calling this function.
 */
struct z_value_t z_reply_err(const struct z_owned_reply_t *reply);
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
struct z_owned_reply_channel_t z_reply_fifo_new(uintptr_t bound);
/**
 * Returns `true` if the queryable answered with an OK, which allows this value to be treated as a sample.
 *
 * If this returns `false`, you should use `z_check` before trying to use `z_reply_err` if you want to process the error that may be here.
 */
bool z_reply_is_ok(const struct z_owned_reply_t *reply);
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
struct z_owned_reply_channel_t z_reply_non_blocking_fifo_new(uintptr_t bound);
/**
 * Returns an invalidated `z_owned_reply_t`.
 *
 * This is useful when you wish to take ownership of a value from a callback to `z_get`:
 * - copy the value of the callback's argument's pointee,
 * - overwrite the pointee with this function's return value,
 * - you are now responsible for dropping your copy of the reply.
 */
struct z_owned_reply_t z_reply_null(void);
/**
 * Yields the contents of the reply by asserting it indicates a success.
 *
 * You should always make sure that `z_reply_is_ok()` returns `true` before calling this function.
 */
struct z_sample_t z_reply_ok(const struct z_owned_reply_t *reply);
/**
 * Returns `true` if `session` is valid.
 */
bool z_session_check(const struct z_owned_session_t *session);
/**
 * Returns a :c:type:`z_session_t` loaned from `s`.
 */
struct z_session_t z_session_loan(const struct z_owned_session_t *s);
/**
 * Returns `true` if `sub` is valid.
 */
bool z_subscriber_check(const struct z_owned_subscriber_t *sub);
/**
 * Create a default subscription info.
 */
struct z_subscriber_options_t z_subscriber_options_default(void);
/**
 * Returns `true` if `ts` is a valid timestamp
 */
bool z_timestamp_check(struct z_timestamp_t ts);
/**
 * Undeclare the key expression generated by a call to :c:func:`z_declare_keyexpr`.
 */
void z_undeclare_keyexpr(struct z_session_t session, struct z_owned_keyexpr_t *keyexpr);
/**
 * Undeclares a publication for the given key expression.
 */
void z_undeclare_publisher(struct z_owned_publisher_t *publisher);
void z_undeclare_pull_subscriber(struct z_owned_pull_subscriber_t *sub);
/**
 * Close a `z_owned_queryable_t`, droping it and invalidating it for doube-drop safety.
 *
 * Parameters:
 *     qable: The :c:type:`z_owned_queryable_t` to close.
 */
void z_undeclare_queryable(struct z_owned_queryable_t *qable);
void z_undeclare_subscriber(struct z_owned_subscriber_t *sub);
/**
 * Returns the key expression's internal string by aliasing it.
 *
 * Currently exclusive to zenoh-c
 */
struct z_bytes_t zc_keyexpr_as_bytes(struct z_keyexpr_t keyexpr);
