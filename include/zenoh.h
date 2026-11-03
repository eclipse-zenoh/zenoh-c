#ifndef ZENOH_H
#define ZENOH_H

/**
 * @defgroup borrowed_cursor_types Borrowed cursors
 * @brief Copy and lifetime contract for borrowed cursor values.
 *
 * Copying a cursor preserves its position and creates an independent cursor. Advancing
 * one copy does not advance another or invalidate previously returned borrowed views.
 * The cursor, all its copies, and borrowed views share the original owner's storage. That owner (bytes,
 * Hello message, or link) must remain alive and unchanged while any copy or view is used.
 * Copying does not retain the owner or extend its lifetime. Neither these cursor values
 * nor their borrowed views require drop.
 */

#include <assert.h>
#include <stdint.h>

#include "zenoh_configure.h"

#ifdef __cplusplus
extern "C" {
#endif

#ifdef _MSC_VER
#define ALIGN(n) __declspec(align(n))
#else
#define ALIGN(n) __attribute__((aligned(n)))
#endif

#if defined(ZENOHC_DYN_LIB) && defined(_MSC_VER)
#define ZENOHC_API __declspec(dllimport)
#else
#define ZENOHC_API
#endif

// clang-format off
// include order is important
#include "zenoh_concrete.h"
#include "zenoh_opaque.h"
#include "zenoh_commons.h"
#include "zenoh_constants.h"
// clang-format on

#ifdef __cplusplus
}
#endif
#include "zenoh_macros.h"
#include "zenoh_memory.h"
#endif
