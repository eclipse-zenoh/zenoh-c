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

#undef NDEBUG
#include <assert.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

int main(int argc, char **argv) {
    // Check that constants really have the expected values. It was found that
    // in case of incorrect DLL linking (missing __declspec(dllimport) clause)
    // the constants becomes wrong without any warning or error
    printf("Z_ROUTER: %d\n", Z_ROUTER);
    assert(Z_ROUTER == 1);
    printf("Z_PEER: %d\n", Z_PEER);
    assert(Z_PEER == 2);
    printf("Z_CLIENT: %d\n", Z_CLIENT);
    assert(Z_CLIENT == 4);
    printf("Z_CONFIG_MODE_KEY: %s\n", Z_CONFIG_MODE_KEY);
    assert(strcmp(Z_CONFIG_MODE_KEY, "mode") == 0);
    printf("Z_CONFIG_CONNECT_KEY: %s\n", Z_CONFIG_CONNECT_KEY);
    assert(strcmp(Z_CONFIG_CONNECT_KEY, "connect/endpoints") == 0);
    printf("Z_CONFIG_LISTEN_KEY: %s\n", Z_CONFIG_LISTEN_KEY);
    assert(strcmp(Z_CONFIG_LISTEN_KEY, "listen/endpoints") == 0);
    printf("Z_CONFIG_USER_KEY: %s\n", Z_CONFIG_USER_KEY);
    assert(strcmp(Z_CONFIG_USER_KEY, "transport/auth/usrpwd/user") == 0);
    printf("Z_CONFIG_PASSWORD_KEY: %s\n", Z_CONFIG_PASSWORD_KEY);
    assert(strcmp(Z_CONFIG_PASSWORD_KEY, "transport/auth/usrpwd/password") == 0);
    printf("Z_CONFIG_MULTICAST_SCOUTING_KEY: %s\n", Z_CONFIG_MULTICAST_SCOUTING_KEY);
    assert(strcmp(Z_CONFIG_MULTICAST_SCOUTING_KEY, "scouting/multicast/enabled") == 0);
    printf("Z_CONFIG_MULTICAST_INTERFACE_KEY: %s\n", Z_CONFIG_MULTICAST_INTERFACE_KEY);
    assert(strcmp(Z_CONFIG_MULTICAST_INTERFACE_KEY, "scouting/multicast/interface") == 0);
    printf("Z_CONFIG_MULTICAST_IPV4_ADDRESS_KEY: %s\n", Z_CONFIG_MULTICAST_IPV4_ADDRESS_KEY);
    assert(strcmp(Z_CONFIG_MULTICAST_IPV4_ADDRESS_KEY, "scouting/multicast/address") == 0);
    printf("Z_CONFIG_SCOUTING_TIMEOUT_KEY: %s\n", Z_CONFIG_SCOUTING_TIMEOUT_KEY);
    assert(strcmp(Z_CONFIG_SCOUTING_TIMEOUT_KEY, "scouting/timeout") == 0);
    printf("Z_CONFIG_SCOUTING_DELAY_KEY: %s\n", Z_CONFIG_SCOUTING_DELAY_KEY);
    assert(strcmp(Z_CONFIG_SCOUTING_DELAY_KEY, "scouting/delay") == 0);
    printf("Z_CONFIG_ADD_TIMESTAMP_KEY: %s\n", Z_CONFIG_ADD_TIMESTAMP_KEY);
    assert(strcmp(Z_CONFIG_ADD_TIMESTAMP_KEY, "add_timestamp") == 0);
}