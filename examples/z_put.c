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
//
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

int main(int argc, char **argv) {
    char *keyexpr = "demo/example/zenoh-c-put";
    char *value = "Put from C!";

    if (argc > 1) keyexpr = argv[1];
    if (argc > 2) value = argv[2];

    z_owned_config_t config;
    z_config_default(&config);

    if (argc > 3) {
        if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_CONNECT_KEY, argv[3]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[3], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config)) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Putting Data ('%s': '%s')...\n", keyexpr, value);

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, value);

    z_owned_bytes_t attachment, key, val;
    z_bytes_from_static_str(&key, (char *)"hello");
    z_bytes_from_static_str(&val, (char *)"there");
    z_bytes_from_pair(&attachment, z_move(key), z_move(val));

    z_put_options_t options;
    z_put_options_default(&options);
    options.attachment =
        z_move(attachment);  // attachement is going to be consumed by z_put, so no need to drop it manually

    int res = z_put(z_loan(s), z_loan(ke), z_move(payload), &options);
    if (res < 0) {
        printf("Put failed...\n");
    }

    z_close(z_move(s));
    return 0;
}
