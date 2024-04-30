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
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

void config_client() {
    const char *peers[] = {"tcp/127.0.0.1", "tcp/192.168.0.1", "tcp/10.0.0.1"};
    z_owned_config_t config;
    z_config_client(&config, peers, 3);
    z_owned_str_t endpoints;
    zc_config_get(z_loan(config), "connect/endpoints", &endpoints);
    assert(strcmp(z_str_data(z_loan(endpoints)), "[\"tcp/127.0.0.1\",\"tcp/192.168.0.1\",\"tcp/10.0.0.1\"]") == 0);
    z_drop(z_move(endpoints));
    z_drop(z_move(config));
}

void config_peer() {
    z_owned_config_t config;
    z_config_peer(&config);
    z_owned_str_t mode;
    zc_config_get(z_loan(config), "mode", &mode);
    assert(strcmp(z_str_data(z_loan(mode)), "\"peer\"") == 0);
    z_drop(z_move(mode));
}

int main(int argc, char **argv) {
    zc_init_logger();
    config_client();
    config_peer();
}
