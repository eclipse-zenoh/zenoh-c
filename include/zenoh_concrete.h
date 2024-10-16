//
// Copyright (c) 2024 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
// clang-format off
#ifdef DOCS
#define ALIGN(n)
#define ZENOHC_API
#endif
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#define DEFAULT_SCOUTING_TIMEOUT 1000
#define Z_CHANNEL_DISCONNECTED 1
#define Z_CHANNEL_NODATA 2
#define Z_OK 0
#define Z_EINVAL -1
#define Z_EPARSE -2
#define Z_EIO -3
#define Z_ENETWORK -4
#define Z_ENULL -5
#define Z_EUNAVAILABLE -6
#define Z_EDESERIALIZE -7
#define Z_ESESSION_CLOSED -8
#define Z_EUTF8 -9
#define Z_EBUSY_MUTEX -16
#define Z_EINVAL_MUTEX -22
#define Z_EAGAIN_MUTEX -11
#define Z_EPOISON_MUTEX -22
#define Z_EGENERIC INT8_MIN
