#ifndef ZENOH_H
#define ZENOH_H

#include <assert.h>

#ifdef __cplusplus
extern "C" {
#endif

#define ZENOH_C "0.7.0"
#define ZENOH_C_MAJOR 0
#define ZENOH_C_MINOR 7
#define ZENOH_C_PATCH 0

// 128-bit type used for alignment.
#ifdef __SIZEOF_INT128__
#define _z_u128 __uint128_t
#else
#define _z_u128 long double
#endif

static_assert(sizeof(_z_u128) == 16, "Size of _z_u128 must be 128 bit");

#include "zenoh_concrete.h"
//
#include "zenoh_commons.h"
#ifdef __cplusplus
}
#endif
#include "zenoh_macros.h"
#endif
