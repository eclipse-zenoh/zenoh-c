#ifndef ZENOH_H
#define ZENOH_H

#include <assert.h>

#include "zenoh_configure.h"

#ifdef __cplusplus
extern "C" {
#endif

#define ZENOH_C "0.7.0"
#define ZENOH_C_MAJOR 0
#define ZENOH_C_MINOR 7
#define ZENOH_C_PATCH 0

#if RUST_U128_ALIGNMENT == 16

// 128-bit type used for alignment.
#ifdef __SIZEOF_INT128__
typedef __uint128_t _z_u128;
#elif _MSC_VER
typedef __declspec(align(16)) struct _z_u128 {
    __int64 _0[2];
} _z_u128;
#else
// Let's hope that long double is 128 bit. If no, the assert below fires
typedef long double _z_u128;
#endif

#elif RUST_U128_ALIGNMENT == 8

typedef struct _z_u128 {
    __int64 _0[2];
} _z_u128;

#else

#error "Unexpected or undefined RUST_U128_ALIGNMENT"

#endif

static_assert(sizeof(_z_u128) == 16, "Size of _z_u128 must be 128 bit");

static_assert(sizeof(struct {
                  char c;
                  _z_u128 u;
              }) == RUST_U128_ALIGNMENT + 16,
              "_z_u128 type must be aligned in the same way as in Rust");

#include "zenoh_concrete.h"
//
#include "zenoh_commons.h"
#ifdef __cplusplus
}
#endif
#include "zenoh_macros.h"
#endif
