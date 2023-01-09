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

#ifdef _MSC_VER
#define ALIGN(n) __declspec(align(n))
#else
#define ALIGN(n) __attribute__((aligned(n)))
#endif

#include "zenoh_configure.h"

#if TARGET_ARCH == aarch64
#define TARGET_ARCH_AARCH64
#elif TARGET_ARCH == x86_64
#define TARGET_ARCH_X86_64
#elif TARGET_ARCH == arm
#define TARGET_ARCH_ARM
#elif TARGET_ARCH
#error TARGET_ARCH = #TARGET_ARCH not supported
#else
#error TARGET_ARCH not defined
#endif

// clang-format off
// include order is important
#include "zenoh_concrete.h"
#include "zenoh_commons.h"
// clang-format on

#ifdef __cplusplus
}
#endif
#include "zenoh_macros.h"
#endif
