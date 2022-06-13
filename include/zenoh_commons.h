typedef enum z_congestion_control {
  z_congestion_control_BLOCK,
  z_congestion_control_DROP,
} z_congestion_control;
typedef enum z_known_encoding {
  z_known_encoding_Empty = 0,
  z_known_encoding_AppOctetStream = 1,
  z_known_encoding_AppCustom = 2,
  z_known_encoding_TextPlain = 3,
  z_known_encoding_AppProperties = 4,
  z_known_encoding_AppJson = 5,
  z_known_encoding_AppSql = 6,
  z_known_encoding_AppInteger = 7,
  z_known_encoding_AppFloat = 8,
  z_known_encoding_AppXml = 9,
  z_known_encoding_AppXhtmlXml = 10,
  z_known_encoding_AppXWwwFormUrlencoded = 11,
  z_known_encoding_TextJson = 12,
  z_known_encoding_TextHtml = 13,
  z_known_encoding_TextXml = 14,
  z_known_encoding_TextCss = 15,
  z_known_encoding_TextCsv = 16,
  z_known_encoding_TextJavascript = 17,
  z_known_encoding_ImageJpeg = 18,
  z_known_encoding_ImagePng = 19,
  z_known_encoding_ImageGif = 20,
} z_known_encoding;
typedef enum z_priority {
  z_priority_REAL_TIME = 1,
  z_priority_INTERACTIVE_HIGH = 2,
  z_priority_INTERACTIVE_LOW = 3,
  z_priority_DATA_HIGH = 4,
  z_priority_DATA = 5,
  z_priority_DATA_LOW = 6,
  z_priority_BACKGROUND = 7,
} z_priority;
/**
 * The subscription reliability.
 *
 *     - **z_reliability_BEST_EFFORT**
 *     - **z_reliability_RELIABLE**
 */
typedef enum z_reliability {
  z_reliability_BEST_EFFORT,
  z_reliability_RELIABLE,
} z_reliability;
typedef enum z_sample_kind {
  z_sample_kind_PUT = 0,
  z_sample_kind_DELETE = 1,
} z_sample_kind;
/**
 * An array of bytes.
 */
typedef struct z_bytes_t {
  const uint8_t *start;
  size_t len;
} z_bytes_t;
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
typedef struct z_owned_queryable_t {
  uintptr_t _0[4];
} z_owned_queryable_t;
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
typedef struct z_queryable_options_t {
  bool complete;
} z_queryable_options_t;
/**
 * The encoding of a payload, in a MIME-like format.
 *
 * For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
 *
 * `suffix` MUST be a valid UTF-8 string.
 */
typedef struct z_encoding_t {
  enum z_known_encoding prefix;
  struct z_bytes_t suffix;
} z_encoding_t;
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
  enum z_sample_kind kind;
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
 * Declare a subscriber for a given key expression.
 *
 * Members:
 *     `z_reliability reliability`: The subscription reliability.
 *     `void *cargs`: A pointer that will be passed to the **callback** at each call.
 *
 */
typedef struct z_subscriber_options_t {
  enum z_reliability reliability;
} z_subscriber_options_t;
typedef struct z_owned_encoding_t {
  enum z_known_encoding prefix;
  struct z_bytes_t suffix;
  bool _dropped;
} z_owned_encoding_t;
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
typedef struct z_owned_closure_response_t {
  void *context;
  void (*call)(struct z_query_t, const void*);
  void (*drop)(void*);
} z_owned_closure_response_t;
/**
 * Options passed to the :c:func:`z_put_ext` function.
 */
typedef struct z_put_options_t {
  struct z_encoding_t encoding;
  enum z_congestion_control congestion_control;
  enum z_priority priority;
} z_put_options_t;
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
extern const unsigned int Z_INFO_PID_KEY;
extern const unsigned int Z_INFO_PEER_PID_KEY;
extern const unsigned int Z_INFO_ROUTER_PID_KEY;
/**
 * Returns `true` if `b` is initialized.
 */
bool z_bytes_check(const struct z_bytes_t *b);
/**
 * Closes a zenoh session. This drops and invalidates `session` for double-drop safety.
 */
void z_close(struct z_owned_session_t *session);
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
 * Gets informations about an zenoh session.
 */
struct z_owned_info_t z_info(struct z_session_t session);
/**
 * Gets informations about an zenoh session as a properties-formatted string.
 */
char *z_info_as_str(struct z_session_t session);
/**
 * Returns `true` if `info` is valid.
 */
bool z_info_check(const struct z_owned_info_t *info);
/**
 * Frees `info`'s memory, while invalidating `info` for double-drop-safety.
 */
void z_info_drop(struct z_owned_info_t *info);
/**
 * Returns the information associated with `key` if it exists.
 * If it doesn't, the returned value is invalid, and doesn't need droping.
 */
char *z_info_get(struct z_info_t info, uint64_t key);
/**
 * Returns a :c:type:`z_info_t` loaned from `info`.
 */
struct z_info_t z_info_loan(const struct z_owned_info_t *info);
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
struct z_owned_keyexpr_t z_keyexpr_join(struct z_keyexpr_t left,
                                        const char *right_start,
                                        uintptr_t right_len);
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
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
void z_owned_closure_query_call(const struct z_owned_closure_query_t *closure,
                                struct z_query_t query);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
void z_owned_closure_query_drop(struct z_owned_closure_query_t *closure);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
void z_owned_closure_reply_call(const struct z_owned_closure_response_t *closure,
                                struct z_query_t sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
void z_owned_closure_reply_drop(struct z_owned_closure_response_t *closure);
/**
 * Calls the closure. Calling an uninitialized closure is a no-op.
 */
void z_owned_closure_sample_call(const struct z_owned_closure_sample_t *closure,
                                 const struct z_sample_t *sample);
/**
 * Drops the closure. Droping an uninitialized closure is a no-op.
 */
void z_owned_closure_sample_drop(struct z_owned_closure_sample_t *closure);
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
 * Undeclare the key expression generated by a call to :c:func:`z_declare_keyexpr`.
 */
void z_undeclare_keyexpr(struct z_session_t session, struct z_owned_keyexpr_t *keyexpr);
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
