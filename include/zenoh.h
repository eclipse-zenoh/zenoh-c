#ifndef ZENOH_H
#define ZENOH_H

#include <assert.h>
#include <stdint.h>

#include "zenoh_configure.h"

#ifdef __cplusplus
extern "C" {
#endif

#define ZENOH_C "0.7.0"
#define ZENOH_C_MAJOR 0
#define ZENOH_C_MINOR 7
#define ZENOH_C_PATCH 0

#define ALIGN(n)
// __attribute__((aligned(n)))

#include "zenoh_concrete.h"
//
#include "zenoh_commons.h"
#ifdef __cplusplus
}
#endif
#include "zenoh_macros.h"
#endif
