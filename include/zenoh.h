#ifndef ZENOH_H
#define ZENOH_H
#ifdef __cplusplus
extern "C" {
#endif

#define ZENOH_C "0.6.0"
#define ZENOH_C_MAJOR 0
#define ZENOH_C_MINOR 6
#define ZENOH_C_PATCH 0

#define _Z_OWNED_REPLY_N_USIZE 18
#if defined(__x86_64__) || defined(_M_X64) || defined(i386) || defined(__i386__) || defined(__i386) || defined(_M_IX86)
#define _Z_OWNED_REPLY_N_U128 1
#define _Z_OWNED_REPLY_N_U64 3
#else
#define _Z_OWNED_REPLY_N_U128 4
#define _Z_OWNED_REPLY_N_U64 0
#endif

#define _z_u128 long double  // used for alignment.

#include "zenoh_concrete.h"
//
#include "zenoh_commons.h"
#include "zenoh_macros.h"
#ifdef __cplusplus
}
#endif
#endif
