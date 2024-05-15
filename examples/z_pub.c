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

void matching_status_handler(const zcu_matching_status_t *matching_status, void *arg) {
    if (matching_status->matching) {
        printf("Subscriber matched\n");
    } else {
        printf("No Subscribers matched\n");
    }
}

int main(int argc, char **argv) {
    char *keyexpr = "demo/example/zenoh-c-pub";
    char *value = "Pub from C!";
    bool add_matching_listener = false;

    if (argc > 1) keyexpr = argv[1];
    if (argc > 2) value = argv[2];
    if (argc > 3) add_matching_listener = atoi(argv[3]);

    z_owned_config_t config;
    z_config_default(&config);
    if (argc > 4) {
        if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_CONNECT_KEY, argv[4]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[4], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config)) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring Publisher on '%s'...\n", keyexpr);
    z_owned_publisher_t pub;
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_string(&ke, keyexpr);
    if (z_declare_publisher(&pub, z_loan(s), z_loan(ke), NULL) < 0) {
        printf("Unable to declare Publisher for key expression!\n");
        exit(-1);
    }

    zcu_owned_matching_listener_t listener;
    if (add_matching_listener) {
        zcu_owned_closure_matching_status_t callback;
        z_closure(&callback, matching_status_handler, NULL, NULL);
        zcu_publisher_matching_listener_callback(&listener, z_loan(pub), z_move(callback));
    }

    char buf[256] = {};
    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);
        sprintf(buf, "[%4d] %s", idx, value);
        printf("Putting Data ('%s': '%s')...\n", keyexpr, buf);
        z_publisher_put_options_t options;
        z_publisher_put_options_default(&options);

        z_view_str_t payload_str;
        z_view_str_wrap(&payload_str, buf);

        z_owned_bytes_t payload;
        z_bytes_encode_from_string(&payload, z_loan(payload_str));
        
        z_publisher_put(z_loan(pub), z_move(payload), &options);
    }

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));
    return 0;
}
