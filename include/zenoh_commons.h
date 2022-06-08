/**
 * The kind of consolidation that should be applied on replies to a :c:func:`z_get`.
 *
 *     - **z_consolidation_mode_t_FULL**: Guaranties unicity of replies. Optimizes bandwidth.
 *     - **z_consolidation_mode_t_LAZY**: Does not garanty unicity. Optimizes latency.
 *     - **z_consolidation_mode_t_NONE**: No consolidation.
 */
typedef enum {
  z_consolidation_mode_t_FULL,
  z_consolidation_mode_t_LAZY,
  z_consolidation_mode_t_NONE,
} z_consolidation_mode_t;
typedef enum {
  z_known_encoding_t_Empty = 0,
  z_known_encoding_t_AppOctetStream = 1,
  z_known_encoding_t_AppCustom = 2,
  z_known_encoding_t_TextPlain = 3,
  z_known_encoding_t_AppProperties = 4,
  z_known_encoding_t_AppJson = 5,
  z_known_encoding_t_AppSql = 6,
  z_known_encoding_t_AppInteger = 7,
  z_known_encoding_t_AppFloat = 8,
  z_known_encoding_t_AppXml = 9,
  z_known_encoding_t_AppXhtmlXml = 10,
  z_known_encoding_t_AppXWwwFormUrlencoded = 11,
  z_known_encoding_t_TextJson = 12,
  z_known_encoding_t_TextHtml = 13,
  z_known_encoding_t_TextXml = 14,
  z_known_encoding_t_TextCss = 15,
  z_known_encoding_t_TextCsv = 16,
  z_known_encoding_t_TextJavascript = 17,
  z_known_encoding_t_ImageJpeg = 18,
  z_known_encoding_t_ImagePng = 19,
  z_known_encoding_t_ImageGif = 20,
} z_known_encoding_t;
/**
 * The subscription reliability.
 *
 *     - **z_reliability_t_BEST_EFFORT**
 *     - **z_reliability_t_RELIABLE**
 */
typedef enum {
  z_reliability_t_BEST_EFFORT,
  z_reliability_t_RELIABLE,
} z_reliability_t;
/**
 * The possible values of :c:member:`z_owned_reply_t.tag`
 *
 *     - **z_reply_t_Tag_DATA**: The reply contains some data.
 *     - **z_reply_t_Tag_FINAL**: The reply does not contain any data and indicates that there will be no more replies for this query.
 */
typedef enum {
  z_reply_t_Tag_DATA,
  z_reply_t_Tag_FINAL,
} z_reply_t_Tag;
/**
 * The subscription mode.
 *
 *     - **z_submode_t_PUSH**
 *     - **z_submode_t_PULL**
 */
typedef enum {
  z_submode_t_PUSH,
  z_submode_t_PULL,
} z_submode_t;
typedef struct z_query_t z_query_t;
/**
 * A zenoh-allocated array of bytes.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct {
  const uint8_t *start;
  size_t len;
} z_owned_bytes_t;
/**
 * A loaned array of bytes.
 */
typedef struct {
  const uint8_t *start;
  size_t len;
} z_bytes_t;
/**
 * An owned, zenoh-allocated, null-terminated, string.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct {
  const char *_loan;
} z_owned_string_t;
/**
 * A loaned null-terminated string.
 */
typedef const char *z_string_t;
/**
 * A loaned key expression.
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
typedef struct {
  unsigned long id;
  z_bytes_t suffix;
} z_keyexpr_t;
typedef struct {
  z_known_encoding_t prefix;
  z_owned_bytes_t suffix;
  bool _freed;
} z_owned_encoding_t;
/**
 * The encoding of a payload, in a MIME-like format.
 *
 * For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
 *
 * `suffix` MUST be a valid UTF-8 string.
 */
typedef struct {
  z_known_encoding_t prefix;
  z_bytes_t suffix;
} z_encoding_t;
/**
 * A zenoh-allocated key expression.
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
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
typedef struct {
  unsigned long id;
  z_owned_bytes_t suffix;
} z_owned_keyexpr_t;
/**
 * The possible values of :c:member:`z_target_t.tag`.
 *
 *     - **z_target_t_BEST_MATCHING**: The nearest complete queryable if any else all matching queryables.
 *     - **z_target_t_COMPLETE**: A set of complete queryables.
 *     - **z_target_t_ALL**: All matching queryables.
 *     - **z_target_t_NONE**: No queryables.
 */
typedef enum {
  z_target_t_BEST_MATCHING,
  z_target_t_ALL,
  z_target_t_NONE,
  z_target_t_ALL_COMPLETE,
#if defined(Z_DEFINE_COMPLETE_N)
  z_target_t_COMPLETE,
#endif
} z_target_t_Tag;
#if defined(Z_DEFINE_COMPLETE_N)
typedef struct {
  unsigned int n;
} z_target_t_COMPLETE_Body;
#endif
typedef struct {
  z_target_t_Tag tag;
  union {
#if defined(Z_DEFINE_COMPLETE_N)
    z_target_t_COMPLETE_Body complete;
#endif
  };
} z_target_t;
/**
 * The zenoh queryables that should be target of a `z_get`.
 *
 * Members:
 *     `unsigned int kind`: A mask of queryable kinds.
 *     `z_target_t target`: The query target.
 */
typedef struct {
  unsigned int kind;
  z_target_t target;
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
typedef struct {
  z_consolidation_mode_t first_routers;
  z_consolidation_mode_t last_router;
  z_consolidation_mode_t reception;
} z_consolidation_strategy_t;
/**
 * The replies consolidation strategy to apply on replies to a :c:func:`z_get`.
 */
typedef enum {
  z_query_consolidation_t_AUTO,
  z_query_consolidation_t_MANUAL,
} z_query_consolidation_t_Tag;
typedef struct {
  z_query_consolidation_t_Tag tag;
  union {
    struct {
      z_consolidation_strategy_t manual;
    };
  };
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
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct {
  z_owned_keyexpr_t key;
  z_owned_bytes_t value;
  z_owned_encoding_t encoding;
} z_owned_sample_t;
/**
 * An owned reply to a `z_get` (or `z_get_collect`).
 *
 * Members:
 *   `z_owned_sample_t sample`: a :c:type:`z_sample_t` containing the key and value of the reply.
 *   `unsigned int source_kind`: The kind of the replier that sent this reply.
 *   `z_owned_bytes_t replier_id`: The id of the replier that sent this reply.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
 */
typedef struct {
  z_owned_sample_t sample;
  unsigned int source_kind;
  z_owned_bytes_t replier_id;
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
typedef struct {
  z_reply_t_Tag tag;
  z_owned_reply_data_t data;
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
typedef struct {
  const z_owned_reply_data_t *val;
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
typedef struct {
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
typedef struct {
  unsigned int whatami;
  z_owned_bytes_t pid;
  z_owned_str_array_t locators;
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
typedef struct {
  const z_owned_hello_t *val;
  size_t len;
} z_owned_hello_array_t;
/**
 * Options passed to the :c:func:`z_put_ext` function.
 */
typedef struct {
  z_encoding_t encoding;
  uint8_t kind;
  uint8_t congestion_control;
  uint8_t priority;
} z_put_options_t;
/**
 * A loaned data sample.
 *
 * A sample is the value associated to a given resource at a given point in time.
 *
 * Members:
 *   `z_string_t key`: The resource key of this data sample.
 *   `z_bytes_t value`: The value of this data sample.
 */
typedef struct {
  z_keyexpr_t key;
  z_bytes_t value;
  z_encoding_t encoding;
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
typedef struct {
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
typedef struct {
  z_reliability_t reliability;
  z_submode_t mode;
  z_period_t period;
} z_subinfo_t;
#define z_period_NONE (z_period_t){ .origin = 0, .period = 0, .duration = 0 }
extern const unsigned int Z_ROUTER;
extern const unsigned int Z_PEER;
extern const unsigned int Z_CLIENT;
extern const unsigned int Z_QUERYABLE_ALL_KINDS;
extern const unsigned int Z_QUERYABLE_STORAGE;
extern const unsigned int Z_QUERYABLE_EVAL;
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
 * Returns `true` if `b` is valid.
 */
bool z_bytes_check(const z_owned_bytes_t *b);
/**
 * Frees `b` and invalidates it for double-free safety.
 */
void z_bytes_free(z_owned_bytes_t *b);
z_bytes_t z_bytes_loan(const z_owned_bytes_t *b);
/**
 * Constructs a :c:type:`z_owned_bytes_t` of lengh `len` from the bytes
 * starting at address `start`.
 * The bytes from `start` are copied.
 */
z_owned_bytes_t z_bytes_new(const uint8_t *start, uintptr_t len);
/**
 * Closes a zenoh session. This frees and invalidates `session` for double-free safety.
 */
void z_close(z_owned_session_t *session);
/**
 * Returns `true` if `config` is valid.
 */
bool z_config_check(const z_owned_config_t *config);
/**
 * Constructs a default configuration client mode zenoh session.
 * If `peer` is not null, it is added to the configuration as remote peer.
 */
z_owned_config_t z_config_client(const char *const *peers, uintptr_t n_peers);
/**
 * Creates a default, zenoh-allocated, configuration.
 */
z_owned_config_t z_config_default(void);
/**
 * Creates an empty, zenoh-allocated, configuration.
 */
z_owned_config_t z_config_empty(void);
/**
 * Frees `config`, invalidating it for double-free safety.
 */
void z_config_free(z_owned_config_t *config);
/**
 * Constructs a configuration by parsing a file at `path`. Currently supported format is JSON5, a superset of JSON.
 */
z_owned_config_t z_config_from_file(const char *path);
/**
 * Reads a configuration from a properties-formated string, such as "mode=client;peer=tcp/127.0.0.1:7447".
 */
z_owned_config_t z_config_from_str(const char *s);
/**
 * Gets the property with the given integer key from the configuration.
 */
z_owned_string_t z_config_get(z_config_t config, z_string_t key);
/**
 * Inserts a JSON-serialized `value` at the `key` position of the configuration.
 *
 * Returns `true` if insertion was succesful, `false` otherwise.
 */
bool z_config_insert_json(z_config_t config, z_string_t key, z_string_t value);
/**
 * Returns a :c:type:`z_config_t` loaned from `s`.
 */
z_config_t z_config_loan(const z_owned_config_t *s);
/**
 * Return a new, zenoh-allocated, empty configuration.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
z_owned_config_t z_config_new(void);
/**
 * Constructs a default configuration peer mode zenoh session.
 */
z_owned_config_t z_config_peer(void);
/**
 * Converts `config` into a properties-formated string, such as "mode=client;peer=tcp/127.0.0.1:7447".
 */
z_owned_string_t z_config_to_str(z_config_t config);
/**
 * Associates a numerical id with the given key expression. The id is returned as a :c:type:`z_keyexpr_t` with a nullptr suffix.
 *
 * This numerical id will be used on the network to save bandwidth and
 * ease the retrieval of the concerned resource in the routing tables.
 */
z_keyexpr_t z_declare_expr(z_session_t session,
                           z_keyexpr_t keyexpr);
/**
 * Declares a publication for the given key expression, returning `true` on success.
 *
 * Written resources that match the given key will only be sent on the network
 * if matching subscribers exist in the system.
 */
bool z_declare_publication(z_session_t session, z_keyexpr_t keyexpr);
/**
 * Returns `true` if `encoding` is valid.
 */
bool z_encoding_check(const z_owned_encoding_t *encoding);
/**
 * Frees `encoding`, invalidating it for double-free safety.
 */
z_encoding_t z_encoding_default(void);
/**
 * Frees `encoding`, invalidating it for double-free safety.
 */
void z_encoding_free(z_owned_encoding_t *encoding);
/**
 * Returns a :c:type:`z_encoding_t` loaned from `encoding`.
 */
z_encoding_t z_encoding_loan(const z_owned_encoding_t *encoding);
/**
 * Constructs a loaned key expression from a string expression.
 */
z_keyexpr_t z_expr(const char *name);
/**
 * Constructs a key expression from a string expression. `name`'s content is copied.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
z_owned_keyexpr_t z_expr_new(const char *name);
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
void z_get(z_session_t session,
           z_keyexpr_t keyexpr,
           const char *predicate,
           z_query_target_t target,
           z_query_consolidation_t consolidation,
           void (*callback)(z_owned_reply_t, const void*),
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
z_owned_reply_data_array_t z_get_collect(z_session_t session,
                                         z_keyexpr_t keyexpr,
                                         const char *predicate,
                                         z_query_target_t target,
                                         z_query_consolidation_t consolidation);
/**
 * Returns `true` if `hellos` is valid.
 */
bool z_hello_array_check(const z_owned_hello_array_t *hellos);
/**
 * Frees `hellos`, invalidating it for double-free safety.
 */
void z_hello_array_free(z_owned_hello_array_t *hellos);
/**
 * Returns `true` if `hello` is valid.
 */
bool z_hello_check(const z_owned_hello_t *hello);
/**
 * Frees `hello`, invalidating it for double-free safety.
 */
void z_hello_free(z_owned_hello_t *hello);
/**
 * Constructs a key expression from an expression id.
 * Since id-only kes expressions don't need destruction, a `z_keyexpr_t` is returned instead of its owned variant.
 */
z_keyexpr_t z_id(unsigned long id);
/**
 * Constructs a loaned key expression from an expression id and a suffix.
 */
z_keyexpr_t z_id_with_suffix(unsigned long id, const char *suffix);
/**
 * Constructs a key expression from an expression id and a suffix. `suffix`'s content is copied.
 *
 * Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
 * The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
 *
 * Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
 * To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
 * After a move, `val` will still exist, but will no longer be valid. The destructors are double-free-safe, but other functions will still trust that your `val` is valid.
 *
 * To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
 */
z_owned_keyexpr_t z_id_with_suffix_new(unsigned long id,
                                       const char *suffix);
/**
 * Gets informations about an zenoh session.
 */
z_owned_info_t z_info(z_session_t session);
/**
 * Gets informations about an zenoh session as a properties-formatted string.
 */
z_owned_string_t z_info_as_str(z_session_t session);
/**
 * Returns `true` if `info` is valid.
 */
bool z_info_check(const z_owned_info_t *info);
/**
 * Frees `info`'s memory, while invalidating `info` for double-free-safety.
 */
void z_info_free(z_owned_info_t *info);
/**
 * Returns the information associated with `key` if it exists.
 * If it doesn't, the returned value is invalid, and doesn't need freeing.
 */
z_owned_string_t z_info_get(z_info_t info, uint64_t key);
/**
 * Returns a :c:type:`z_info_t` loaned from `info`.
 */
z_info_t z_info_loan(const z_owned_info_t *info);
/**
 * Initialises the zenoh runtime logger
 */
void z_init_logger(void);
/**
 * Returns `true` if `keyexpr` is valid.
 */
bool z_keyexpr_check(const z_owned_keyexpr_t *keyexpr);
/**
 * Frees `keyexpr` and invalidates it for double-free safety.
 */
void z_keyexpr_free(z_owned_keyexpr_t *keyexpr);
/**
 * Returns a :c:type:`z_keyexpr_t` loaned from `keyexpr`.
 */
z_keyexpr_t z_keyexpr_loan(const z_owned_keyexpr_t *keyexpr);
/**
 * Constructs a zenoh-owned key expression. `suffix`'s contents will be copied.
 */
z_owned_keyexpr_t z_keyexpr_new(unsigned long id, const char *suffix);
/**
 * Constructs a loaned key expression. The constructed value is valid as long as `suffix` is.
 */
z_keyexpr_t z_keyexpr_new_loaned(unsigned long id, const char *suffix);
/**
 * Opens a zenoh session. Should the session opening fail, `z_check`ing the returned value will return `false`.
 */
z_owned_session_t z_open(z_owned_config_t *config);
/**
 * Pull data for a pull mode :c:type:`z_owned_subscriber_t`. The pulled data will be provided
 * by calling the **callback** function provided to the :c:func:`z_subscribe` function.
 *
 * Parameters:
 *     sub: The :c:type:`z_owned_subscriber_t` to pull from.
 */
void z_pull(const z_owned_subscriber_t *sub);
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
int z_put(z_session_t session, z_keyexpr_t keyexpr, const uint8_t *payload, unsigned int len);
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
int z_put_ext(z_session_t session,
              z_keyexpr_t keyexpr,
              const uint8_t *payload,
              unsigned int len,
              const z_put_options_t *options);
/**
 * Constructs the default value for write options
 */
z_put_options_t z_put_options_default(void);
/**
 * Automatic query consolidation strategy selection.
 *
 * A query consolidation strategy will automatically be selected depending
 * the query selector. If the selector contains time range properties,
 * no consolidation is performed. Otherwise the
 * :c:func:`z_query_consolidation_reception` strategy is used.
 */
z_query_consolidation_t z_query_consolidation_auto(void);
/**
 * Creates a default :c:type:`z_query_consolidation_t`.
 */
z_query_consolidation_t z_query_consolidation_default(void);
/**
 * Full consolidation performed everywhere.
 *
 * This mode optimizes bandwidth on all links in the system
 * but will provide a very poor latency.
 */
z_query_consolidation_t z_query_consolidation_full(void);
/**
 * Full consolidation performed on last router and at reception.
 *
 * This mode offers a good latency while optimizing bandwidth on
 * the last transport link between the router and the application.
 */
z_query_consolidation_t z_query_consolidation_last_router(void);
/**
 * Lazy consolidation performed at all stages.
 *
 * This strategy offers the best latency. Replies are directly
 * transmitted to the application when received without needing
 * to wait for all replies.
 *
 * This mode does not garantie that there will be no duplicates.
 */
z_query_consolidation_t z_query_consolidation_lazy(void);
/**
 * No consolidation performed.
 *
 * This is usefull when querying timeseries data bases or
 * when using quorums.
 */
z_query_consolidation_t z_query_consolidation_none(void);
/**
 * Full consolidation performed at reception.
 *
 * This is the default strategy. It offers the best latency while
 * garantying that there will be no duplicates.
 */
z_query_consolidation_t z_query_consolidation_reception(void);
/**
 * Gets the key expression of a received query as a non null-terminated string.
 */
z_keyexpr_t z_query_key_expr(const z_query_t *query);
/**
 * Gets the predicate of a received query as a non null-terminated string.
 */
z_bytes_t z_query_predicate(const z_query_t *query);
/**
 * Creates a default `z_query_target_t`.
 */
z_query_target_t z_query_target_default(void);
/**
 * Returns `true` if `qable` is valid.
 */
bool z_queryable_check(const z_owned_queryable_t *qable);
/**
 * Close a `z_owned_queryable_t`, freeing it and invalidating it for doube-free safety.
 *
 * Parameters:
 *     qable: The :c:type:`z_owned_queryable_t` to close.
 */
void z_queryable_close(z_owned_queryable_t *qable);
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
z_owned_queryable_t z_queryable_new(z_session_t session,
                                    z_keyexpr_t keyexpr,
                                    unsigned int kind,
                                    void (*callback)(const z_query_t*, const void*),
                                    void *arg);
/**
 * Returns `true` if `reply` is valid.
 */
bool z_reply_check(const z_owned_reply_t *reply);
bool z_reply_data_array_check(const z_owned_reply_data_array_t *replies);
/**
 * Free a :c:type:`z_owned_reply_data_array_t` and it's contained replies.
 *
 * Parameters:
 *     replies: The :c:type:`z_owned_reply_data_array_t` to free.
 *
 */
void z_reply_data_array_free(z_owned_reply_data_array_t *replies);
/**
 * Returns `true` if `reply_data` is valid.
 */
bool z_reply_data_check(const z_owned_reply_data_t *reply_data);
/**
 * Frees `reply_data`, invalidating it for double-free safety.
 */
void z_reply_data_free(z_owned_reply_data_t *reply_data);
/**
 * Frees `reply`, invalidating it for double-free safety.
 */
void z_reply_free(z_owned_reply_t *reply);
/**
 * Returns `true` if `sample` is valid.
 */
bool z_sample_check(const z_owned_sample_t *sample);
/**
 * Frees `sample`, invalidating it for double-free safety.
 */
void z_sample_free(z_owned_sample_t *sample);
/**
 * Returns a :c:type:`z_sample_t` loaned from `sample`.
 */
z_sample_t z_sample_loan(const z_owned_sample_t *sample);
/**
 * Scout for routers and/or peers.
 *
 * Parameters:
 *     `what`: A whatami bitmask of zenoh entities kind to scout for.
 *     `config`: A set of properties to configure the scouting.
 *     `scout_period`: The time (in milliseconds) that should be spent scouting before returning the results.
 *
 * Returns:
 *     An array of `z_hello_t` messages.
 */
z_owned_hello_array_t z_scout(unsigned int what,
                              z_owned_config_t *config,
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
void z_send_reply(const z_query_t *query,
                  const char *key,
                  const uint8_t *payload,
                  unsigned int len);
/**
 * Returns `true` if `session` is valid.
 */
bool z_session_check(const z_owned_session_t *session);
/**
 * Returns a :c:type:`z_session_t` loaned from `s`.
 */
z_session_t z_session_loan(const z_owned_session_t *s);
/**
 * Returns `true` if `strs` is valid.
 */
bool z_str_array_check(const z_owned_str_array_t *strs);
/**
 * Frees `strs` and invalidates it for double-free safety.
 */
void z_str_array_free(z_owned_str_array_t *strs);
/**
 * Returns `true` if `s` is valid
 */
bool z_string_check(const z_owned_string_t *s);
/**
 * Frees `s`'s memory, while invalidating `s` for double-free-safety.
 */
void z_string_free(z_owned_string_t *s);
/**
 * Returns a :c:type:`z_string_t` loaned from `s`.
 */
z_string_t z_string_loan(const z_owned_string_t *s);
/**
 * Constructs a :c:type:`z_owned_string_t` from a NULL terminated string.
 * The contents of `s` are copied.
 */
z_owned_string_t z_string_new(const char *s);
/**
 * Create a default subscription info.
 */
z_subinfo_t z_subinfo_default(void);
/**
 * Returns the subscription period from `info`.
 */
const z_period_t *z_subinfo_period(const z_subinfo_t *info);
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
z_owned_subscriber_t z_subscribe(z_session_t session,
                                 z_keyexpr_t keyexpr,
                                 z_subinfo_t sub_info,
                                 void (*callback)(const z_sample_t*, const void*),
                                 void *arg);
/**
 * Returns `true` if `sub` is valid.
 */
bool z_subscriber_check(const z_owned_subscriber_t *sub);
/**
 * Unsubscribes from the passed `sub`, freeing it and invalidating it for double-free safety.
 */
void z_subscriber_close(z_owned_subscriber_t *sub);
/**
 * Create a default :c:type:`z_target_t`.
 */
z_target_t z_target_default(void);
/**
 * Unbinds the numerical id key generated by a call to :c:func:`z_declare_expr`.
 */
void z_undeclare_expr(z_session_t session, z_keyexpr_t keyexpr);
/**
 * Undeclares a publication for the given key expression.
 */
void z_undeclare_publication(z_session_t session, z_keyexpr_t keyexpr);
