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
#include "zenoh.h"

const char *kind_to_str(z_sample_kind_t kind) {
    switch (kind) {
        case Z_SAMPLE_KIND_PUT:
            return "PUT";
        case Z_SAMPLE_KIND_DELETE:
            return "DELETE";
        default:
            return "UNKNOWN";
    }
}

void handle_sample(const z_loaned_sample_t *sample) {
    z_view_string_t keystr;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &keystr);
    z_owned_string_t payload_value;
    z_bytes_decode_into_string(z_sample_payload(sample), &payload_value);
    printf(">> [Subscriber] Received %s ('%.*s': '%.*s')\n", 
        kind_to_str(z_sample_kind(sample)), 
        (int)z_string_len(z_loan(keystr)), z_string_data(z_loan(keystr)),
        (int)z_string_len(z_loan(payload_value)), z_string_data(z_loan(payload_value))
    );
    z_drop(z_move(payload_value));
}

int main(int argc, char **argv) {
    char *expr = "demo/example/**";
    if (argc > 1) {
        expr = argv[1];
    }

    z_owned_config_t config;
    z_config_default(&config);
    if (argc > 2) {
        if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_LISTEN_KEY, argv[2]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[2], Z_CONFIG_LISTEN_KEY, Z_CONFIG_LISTEN_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s;

    if (z_open(&s, z_move(config)) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_owned_ring_handler_sample_t handler;
    z_owned_closure_sample_t closure;

    printf("Declaring Subscriber on '%s'...\n", expr);
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_string(&ke, expr);
    z_owned_subscriber_t sub;

    if (z_declare_subscriber(&sub, z_loan(s), z_loan(ke), z_move(closure), NULL) < 0) {
        printf("Unable to declare subscriber.\n");
        exit(-1);
    }

    printf("Press <enter> to pull data...\n");
    z_owned_sample_t sample;

    char c = 0;
    while (c != 'q') {
        c = getchar();
        if (c == -1) {
            z_sleep_s(1);
        } else {
            z_try_recv(z_loan(handler), &sample);
            if (z_check(sample)) {
                handle_sample(z_loan(sample));
                z_drop(z_move(sample));
            }
        }
    }
    z_undeclare_subscriber(z_move(sub));
    z_close(z_move(s));
    return 0;
}
