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
 * Initialises the zenoh runtime logger.
 *
 * Note that unless you built zenoh-c with the `logger-autoinit` feature disabled,
 * this will be performed automatically by `z_open` and `z_scout`.
 */
ZENOHC_API void zc_init_logger(void);
