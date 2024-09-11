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

void insert_get() {
    z_owned_config_t config;
    z_config_default(&config);
    zc_config_insert_json(z_loan_mut(config), "mode", "\"client\"");
    zc_config_insert_json(z_loan_mut(config), "connect/endpoints",
                          "[\"tcp/127.0.0.1\", \"tcp/192.168.0.1\", \"tcp/10.0.0.1\"]");
    z_owned_string_t endpoints;
    zc_config_get_from_str(z_loan(config), "connect/endpoints", &endpoints);
    assert(strncmp(z_string_data(z_loan(endpoints)), "[\"tcp/127.0.0.1\",\"tcp/192.168.0.1\",\"tcp/10.0.0.1\"]",
                   z_string_len(z_loan(endpoints))) == 0);
    z_drop(z_move(endpoints));
    z_owned_string_t mode;
    zc_config_get_from_str(z_loan(config), "mode", &mode);
    assert(strncmp(z_string_data(z_loan(mode)), "\"client\"", z_string_len(z_loan(mode))) == 0);
    z_drop(z_move(mode));
    z_drop(z_move(config));
}

int main(int argc, char **argv) {
    zc_init_logging();
    insert_get();
}
