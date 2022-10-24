#ifndef ZENOH_H
#define ZENOH_H
#ifdef __cplusplus
extern "C" {
#endif

#define ZENOH_C "0.6.0"
#define ZENOH_C_MAJOR 0
#define ZENOH_C_MINOR 6
#define ZENOH_C_PATCH 0

#define _z_u128 long double  // used for alignment.

#include "zenoh_concrete.h"
//
#include "zenoh_commons.h"
#ifdef __cplusplus
}
#include "zenoh_macros.h"
#endif
#endif
