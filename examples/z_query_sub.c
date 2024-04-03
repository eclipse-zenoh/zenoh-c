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

const char *kind_to_str(z_sample_kind_t kind);

void data_handler(const z_sample_t *sample, void *arg) {
    z_owned_str_t keystr = z_keyexpr_to_string(z_sample_keyexpr(sample));
    z_owned_str_t payload = zc_payload_decode_into_string(z_sample_payload(sample));
    printf(">> [Subscriber] Received %s ('%s': '%s')\n", kind_to_str(z_sample_kind(sample)), z_loan(keystr),
        z_loan(payload));
    z_drop(z_move(payload));
    z_drop(z_move(keystr));
}

int main(int argc, char **argv) {
    char *expr = "demo/example/**";
    if (argc > 1) {
        expr = argv[1];
    }

    z_owned_config_t config = z_config_default();
    if (argc > 2) {
        if (zc_config_insert_json(z_loan(config), Z_CONFIG_LISTEN_KEY, argv[2]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[2], Z_CONFIG_LISTEN_KEY, Z_CONFIG_LISTEN_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    ze_querying_subscriber_options_t sub_opts = ze_querying_subscriber_options_default();
    z_owned_closure_sample_t callback = z_closure(data_handler);
    printf("Declaring querying subscriber on '%s'...\n", expr);
    ze_owned_querying_subscriber_t sub =
        ze_declare_querying_subscriber(z_loan(s), z_keyexpr(expr), z_move(callback), &sub_opts);
    if (!z_check(sub)) {
        printf("Unable to declare querying subscriber.\n");
        exit(-1);
    }

    printf("Enter 'q' to quit...\n");
    char c = 0;
    while (c != 'q') {
        c = getchar();
        if (c == -1) {
            z_sleep_s(1);
        }
    }

    z_drop(z_move(sub));
    z_close(z_move(s));

    return 0;
}

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
