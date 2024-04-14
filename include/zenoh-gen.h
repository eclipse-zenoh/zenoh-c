#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


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

typedef struct z_owned_bytes_t {
  uint8_t *start;
  size_t len;
} z_owned_bytes_t;

/**
 * A contiguous view of bytes owned by some other entity.
 *
 * `start` being `null` is considered a gravestone value,
 * and empty slices are represented using a possibly dangling pointer for `start`.
 */
typedef struct z_bytes_t {
  const uint8_t *start;
  size_t len;
} z_bytes_t;

typedef struct ALIGN(8) z_owned_session_t {
  uint8_t _0[8];
} z_owned_session_t;

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

typedef struct ALIGN(8) z_owned_keyexpr_t {
  uint8_t _0[32];
} z_owned_keyexpr_t;

typedef struct ALIGN(8) z_session_t {
  uint8_t _0[8];
} z_session_t;

typedef struct ALIGN(8) z_keyexpr_t {
  uint8_t _0[8];
} z_keyexpr_t;

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
 * Returns ``true`` if `b` is initialized.
 */
ZENOHC_API bool z_bytes_check(const struct z_owned_bytes_t *b);

ZENOHC_API struct z_owned_bytes_t z_bytes_clone(const struct z_bytes_t *b);

/**
 * Returns the gravestone value for `z_bytes_t`
 */
ZENOHC_API struct z_bytes_t z_bytes_empty(void);

/**
 * Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
 *
 * `str == NULL` will cause this to return `z_bytes_empty()`
 */
ZENOHC_API struct z_bytes_t z_bytes_from_str(const char *str);

/**
 * Returns ``true`` if `b` is initialized.
 */
ZENOHC_API bool z_bytes_is_initialized(const struct z_bytes_t *b);

ZENOHC_API struct z_bytes_t z_bytes_loan(const struct z_owned_bytes_t *b);

/**
 * Deprecated in favor of `z_bytes_from_str`: Returns a view of `str` using `strlen` (this should therefore not be used with untrusted inputs).
 *
 * `str == NULL` will cause this to return `z_bytes_empty()`
 */
ZENOHC_API
struct z_bytes_t z_bytes_new(const char *str);

/**
 * Returns the gravestone value for `z_owned_bytes_t`
 */
ZENOHC_API struct z_owned_bytes_t z_bytes_null(void);

/**
 * Constructs a `len` bytes long view starting at `start`.
 */
ZENOHC_API struct z_bytes_t z_bytes_wrap(const uint8_t *start, size_t len);

/**
 * Closes a zenoh session. This drops and invalidates `session` for double-drop safety.
 *
 * Returns a negative value if an error occured while closing the session.
 * Returns the remaining reference count of the session otherwise, saturating at i8::MAX.
 */
ZENOHC_API int8_t z_close(struct z_owned_session_t *session);

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
int8_t z_declare_keyexpr(struct z_owned_keyexpr_t *this_,
                         struct z_session_t session,
                         struct z_keyexpr_t keyexpr);

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
 * Constructs a :c:type:`z_keyexpr_t` departing from a string.
 * It is a loaned key expression that aliases `name`.
 * The string is canonized in-place before being passed to keyexpr.
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 */
ZENOHC_API
struct z_keyexpr_t z_keyexpr_autocanonize(char *name);

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
ZENOHC_API int8_t z_keyexpr_new(struct z_owned_keyexpr_t *this_, const char *name);

/**
 * Constructs a :c:type:`z_keyexpr_t` departing from a string, copying the passed string. The copied string is canonized.
 */
ZENOHC_API
int8_t z_keyexpr_new_autocanonize(struct z_owned_keyexpr_t *this_,
                                  const char *name);

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
ZENOHC_API z_owned_str_t z_keyexpr_to_string(struct z_keyexpr_t keyexpr);

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
 * Undeclare the key expression generated by a call to :c:func:`z_declare_keyexpr`.
 */
ZENOHC_API int8_t z_undeclare_keyexpr(struct z_session_t session, struct z_owned_keyexpr_t *kexpr);

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
z_owned_str_t zc_config_get(struct z_config_t config,
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
z_owned_str_t zc_config_to_string(struct z_config_t config);

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
 * Constructs a :c:type:`z_keyexpr_t` departing from a string.
 * It is a loaned key expression that aliases `name`.
 * The string is canonized in-place before being passed to keyexpr.
 * May SEGFAULT if `start` is NULL or lies in read-only memory (as values initialized with string litterals do).
 */
ZENOHC_API
struct z_keyexpr_t zc_keyexpr_from_slice_autocanonize(char *name,
                                                      size_t *len);

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
 * Increments the session's reference count, returning a new owning handle.
 */
ZENOHC_API struct z_owned_session_t zc_session_rcinc(struct z_session_t session);
