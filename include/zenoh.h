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
#elif _MSC_VER
typedef __declspec(align(16)) struct _z_u128_aligned {
    __int64 _0[2];
} _z_u128_aligned;
#define _z_u128 _z_u128_aligned
#else
// Let's hope that long double is 128 bit. If no, the assert below fires
#define _z_u128 long double
#endif

static_assert(sizeof(_z_u128) == 16, "Size of _z_u128 must be 128 bit");

typedef struct _z_u128_align_test {
    char c;
    _z_u128 u128;
} _z_u128_align_test;

static_assert(sizeof(_z_u128_align_test) == 32, "_z_u128 type must be aligned by 16-byte boundary");

#include "zenoh_concrete.h"
//
#include "zenoh_commons.h"
#ifdef __cplusplus
}
#endif
#include "zenoh_macros.h"
#endif
