//
// Copyright (c) 2022 ZettaScale Technology
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
#pragma once

#define Z_CONGESTION_CONTROL_DEFAULT Z_CONGESTION_CONTROL_DROP
#define Z_CONSOLIDATION_MODE_DEFAULT Z_CONSOLIDATION_MODE_AUTO
#define Z_PRIORITY_DEFAULT Z_PRIORITY_DATA
#define Z_QUERY_TARGET_DEFAULT Z_QUERY_TARGET_BEST_MATCHING
#define Z_RELIABILITY_DEFAULT Z_RELIABILITY_RELIABLE
#define Z_SAMPLE_KIND_DEFAULT Z_SAMPLE_KIND_PUT

// config keys
#define Z_CONFIG_MODE_KEY "mode"
#define Z_CONFIG_CONNECT_KEY "connect/endpoints"
#define Z_CONFIG_LISTEN_KEY "listen/endpoints"
#define Z_CONFIG_USER_KEY "transport/auth/usrpwd/user"
#define Z_CONFIG_PASSWORD_KEY "transport/auth/usrpwd/password"
#define Z_CONFIG_MULTICAST_SCOUTING_KEY "scouting/multicast/enabled"
#define Z_CONFIG_MULTICAST_INTERFACE_KEY "scouting/multicast/interface"
#define Z_CONFIG_MULTICAST_IPV4_ADDRESS_KEY "scouting/multicast/address"
#define Z_CONFIG_SCOUTING_DELAY_KEY "scouting/delay"
#define Z_CONFIG_SCOUTING_TIMEOUT_KEY "scouting/timeout"
#define Z_CONFIG_ADD_TIMESTAMP_KEY "timestamping/enabled"
#define Z_CONFIG_SHARED_MEMORY_KEY "transport/shared_memory/enabled"

// shm
#if (defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API))
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A 1-byte alignment.
#define ALIGN_1_BYTE (z_alloc_alignment_t{0})
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A 2-byte alignment.
#define ALIGN_2_BYTES (z_alloc_alignment_t{1})
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A 4-byte alignment.
#define ALIGN_4_BYTES (z_alloc_alignment_t{2})
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An 8-byte alignment.
#define ALIGN_8_BYTES (z_alloc_alignment_t{3})
#endif
