#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


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
typedef struct ALIGN(8) z_owned_buffer_t {
  uint8_t _0[40];
} z_owned_buffer_t;

typedef struct ALIGN(8) z_buffer_t {
  uint8_t _0[8];
} z_buffer_t;

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

typedef struct z_owned_bytes_t {
  uint8_t *start;
  size_t len;
} z_owned_bytes_t;

typedef struct ALIGN(8) z_owned_session_t {
  uint8_t _0[8];
} z_owned_session_t;

typedef struct ALIGN(8) z_owned_config_t {
  uint8_t _0[8];
} z_owned_config_t;

typedef struct ALIGN(8) z_config_t {
  uint8_t _0[8];
} z_config_t;

typedef struct ALIGN(8) z_owned_encoding_t {
  uint8_t _0[48];
} z_owned_encoding_t;

typedef struct ALIGN(8) z_encoding_t {
  uint8_t _0[8];
} z_encoding_t;

typedef struct ALIGN(8) z_sample_t {
  uint8_t _0[8];
} z_sample_t;

typedef struct ALIGN(8) z_keyexpr_t {
  uint8_t _0[8];
} z_keyexpr_t;

/**
 * An owned payload, backed by a reference counted owner.
 *
 * The `payload` field may be modified, and Zenoh will take the new values into account.
 */
typedef struct z_owned_buffer_t zc_owned_payload_t;

typedef struct z_buffer_t zc_payload_t;

typedef struct ALIGN(1) z_id_t {
  uint8_t _0[16];
} z_id_t;

typedef struct z_timestamp_t {
  uint64_t time;
  struct z_id_t id;
} z_timestamp_t;

typedef struct ALIGN(8) z_session_t {
  uint8_t _0[8];
} z_session_t;

/**
 * The wrapper type for null-terminated string values allocated by zenoh. The instances of `z_owned_str_t`
 * should be released with `z_drop` macro or with `z_str_drop` function and checked to validity with
 * `z_check` and `z_str_check` correspondently
 */
typedef struct z_owned_str_t {
  char *_cstr;
} z_owned_str_t;

/**
 * A reader for payload data.
 */
typedef struct ALIGN(8) zc_owned_payload_reader {
  uint8_t _0[24];
} zc_owned_payload_reader;

typedef struct ALIGN(8) zc_payload_reader {
  uint8_t _0[8];
} zc_payload_reader;

/**
 * An owned sample.
 *
 * This is a read only type that can only be constructed by cloning a `z_sample_t`.
 * Like all owned types, its memory must be freed by passing a mutable reference to it to `zc_sample_drop`.
 */
typedef struct ALIGN(8) zc_owned_sample_t {
  uint8_t _0[240];
} zc_owned_sample_t;

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
 * Returns `true` if the buffer is in a valid state.
 */
ZENOHC_API bool z_buffer_check(const struct z_owned_buffer_t *buffer);

/**
 * Increments the buffer's reference count, returning an owned version of the buffer.
 */
ZENOHC_API void z_buffer_clone(struct z_owned_buffer_t *dst, const struct z_owned_buffer_t *buffer);

/**
 * Decrements the buffer's reference counter, destroying it if applicable.
 *
 * `buffer` will be reset to `z_buffer_null`, preventing UB on double-frees.
 */
ZENOHC_API void z_buffer_drop(struct z_owned_buffer_t *buffer);

/**
 * Returns total number bytes in the buffer.
 */
ZENOHC_API size_t z_buffer_len(struct z_buffer_t buffer);

/**
 * Loans the buffer, allowing you to call functions that only need a loan of it.
 */
ZENOHC_API struct z_buffer_t z_buffer_loan(const struct z_owned_buffer_t *buffer);

/**
 * The gravestone value for `z_owned_buffer_t`.
 */
ZENOHC_API void z_buffer_null(struct z_owned_buffer_t *this_);

/**
 * Returns the `index`th slice of the buffer, aliasing it.
 *
 * Out of bounds accesses will return `z_bytes_empty`.
 */
ZENOHC_API struct z_bytes_t z_buffer_slice_at(struct z_buffer_t buffer, size_t index);

/**
 * Returns the number of slices in the buffer.
 *
 * If the return value is 0 or 1, then the buffer's data is contiguous in memory.
 */
ZENOHC_API size_t z_buffer_slice_count(struct z_buffer_t buffer);

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
void z_config_new(struct z_owned_config_t *this_);

/**
 * Constructs a null safe-to-drop value of 'z_owned_config_t' type
 */
ZENOHC_API void z_config_null(struct z_owned_config_t *this_);

/**
 * Constructs a default, zenoh-allocated, peer mode configuration.
 */
ZENOHC_API struct z_owned_config_t z_config_peer(void);

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
 * Opens a zenoh session. Should the session opening fail, `z_check` ing the returned value will return `false`.
 */
ZENOHC_API
int8_t z_open(struct z_owned_session_t *this_,
              struct z_owned_config_t *config);

/**
 * The qos with which the sample was received.
 * TODO: split to methods (priority, congestion_control, express)
 * The sample's attachment.
 *
 * `sample` is aliased by the return value.
 */
ZENOHC_API z_attachment_t z_sample_attachment(const struct z_sample_t *sample);

/**
 * The encoding of the payload.
 */
ZENOHC_API struct z_encoding_t z_sample_encoding(struct z_sample_t sample);

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
 * Returns the sample's payload after incrementing its internal reference count.
 *
 * Note that other samples may have received the same buffer, meaning that mutating this buffer may
 * affect the samples received by other subscribers.
 */
ZENOHC_API zc_owned_payload_t z_sample_owned_payload(const struct z_sample_t *sample);

/**
 * The sample's data, the return value aliases the sample.
 *
 * If you need ownership of the buffer, you may use `z_sample_owned_payload`.
 */
ZENOHC_API zc_payload_t z_sample_payload(const struct z_sample_t *sample);

/**
 * The samples timestamp
 */
ZENOHC_API struct z_timestamp_t z_sample_timestamp(const struct z_sample_t *sample);

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
 * Returns ``true`` if `ts` is a valid timestamp
 */
ZENOHC_API bool z_timestamp_check(struct z_timestamp_t ts);

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
 * Returns `false` if `payload` is the gravestone value.
 */
ZENOHC_API bool zc_payload_check(const zc_owned_payload_t *payload);

/**
 * Increments internal payload reference count, returning owned payload.
 */
ZENOHC_API void zc_payload_clone(zc_owned_payload_t *dst, const zc_owned_payload_t *payload);

/**
 * Decodes payload into null-terminated string
 */
ZENOHC_API int8_t zc_payload_decode_into_bytes(zc_payload_t payload, struct z_owned_bytes_t *b);

/**
 * Decodes payload into null-terminated string
 */
ZENOHC_API int8_t zc_payload_decode_into_string(zc_payload_t payload, struct z_owned_str_t *cstr);

/**
 * Decrements `payload`'s backing refcount, releasing the memory if appropriate.
 */
ZENOHC_API void zc_payload_drop(zc_owned_payload_t *payload);

/**
 * Encodes byte sequence by aliasing.
 */
ZENOHC_API void zc_payload_encode_from_bytes(zc_owned_payload_t *dst, struct z_bytes_t bytes);

/**
 * Encodes a null-terminated string by aliasing.
 */
ZENOHC_API void zc_payload_encode_from_string(zc_owned_payload_t *dst, const char *cstr);

/**
 * Returns total number bytes in the payload.
 */
ZENOHC_API size_t zc_payload_len(zc_payload_t payload);

/**
 * Returns a :c:type:`zc_payload_t` loaned from `payload`.
 */
ZENOHC_API zc_payload_t zc_payload_loan(const zc_owned_payload_t *payload);

/**
 * Constructs `zc_owned_payload_t`'s gravestone value.
 */
ZENOHC_API void zc_payload_null(zc_owned_payload_t *this_);

/**
 * Clones the `payload` by incrementing its reference counter.
 */
ZENOHC_API void zc_payload_rcinc(zc_owned_payload_t *dst, const zc_owned_payload_t *payload);

/**
 * Creates a reader for the specified `payload`.
 *
 * Returns 0 in case of success, -1 if `payload` is not valid.
 */
ZENOHC_API void zc_payload_reader_init(struct zc_owned_payload_reader *this_, zc_payload_t payload);

/**
 * Reads data into specified destination.
 *
 * Will read at most `len` bytes.
 * Returns number of bytes read. If return value is smaller than `len`, it means that end of the payload was reached.
 */
ZENOHC_API
size_t zc_payload_reader_read(struct zc_payload_reader reader,
                              uint8_t *dest,
                              size_t len);

/**
 * Returns number of the remaining bytes in the payload
 *
 */
ZENOHC_API size_t zc_payload_reader_remaining(struct zc_payload_reader reader);

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

ZENOHC_API enum zcu_locality_t zcu_locality_default(void);

ZENOHC_API enum zcu_reply_keyexpr_t zcu_reply_keyexpr_default(void);
