#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


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

typedef struct ALIGN(8) z_owned_config_t {
  uint8_t _0[8];
} z_owned_config_t;

typedef struct ALIGN(8) z_session_t {
  uint8_t _0[8];
} z_session_t;

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
 * Opens a zenoh session. Should the session opening fail, `z_check` ing the returned value will return `false`.
 */
ZENOHC_API
int8_t z_open(struct z_owned_session_t *this_,
              struct z_owned_config_t *config);

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
 * Initialises the zenoh runtime logger.
 *
 * Note that unless you built zenoh-c with the `logger-autoinit` feature disabled,
 * this will be performed automatically by `z_open` and `z_scout`.
 */
ZENOHC_API void zc_init_logger(void);

/**
 * Increments the session's reference count, returning a new owning handle.
 */
ZENOHC_API
int8_t zc_session_rcinc(struct z_owned_session_t *dst,
                        const struct z_owned_session_t *src);
