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
