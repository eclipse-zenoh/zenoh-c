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

#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

const char *PUB_KEY_EXPR = "test/valgrind/data";
const char *SUB_KEY_EXPR = "test/valgrind/**";

void data_handler(z_loaned_sample_t *sample, void *context) {
    (void)context;
    z_view_string_t key_string;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_string);

    z_owned_string_t payload_string;
    z_bytes_to_string(z_sample_payload(sample), &payload_string);

    printf(">> [Subscriber] Received ('%.*s': '%.*s')\n", (int)z_string_len(z_loan(key_string)),
           z_string_data(z_loan(key_string)), (int)z_string_len(z_loan(payload_string)),
           z_string_data(z_loan(payload_string)));
    z_drop(z_move(payload_string));
}

int main(int argc, char **argv) {
    printf("Declaring Publisher on %s\n", PUB_KEY_EXPR);

    z_owned_keyexpr_t pub_keyexpr;
    z_keyexpr_from_str(&pub_keyexpr, PUB_KEY_EXPR);

    z_owned_config_t pub_config;
    z_config_default(&pub_config);

    z_owned_session_t pub_session;
    z_open(&pub_session, z_move(pub_config), NULL);

    z_owned_publisher_t publisher;
    z_declare_publisher(&publisher, z_loan(pub_session), z_loan(pub_keyexpr), NULL);

    printf("Declaring Subscriber on %s\n", SUB_KEY_EXPR);

    z_view_keyexpr_t sub_keyexpr;
    z_view_keyexpr_from_str(&sub_keyexpr, SUB_KEY_EXPR);

    z_owned_config_t sub_config;
    z_config_default(&sub_config);

    z_owned_session_t sub_session;
    z_open(&sub_session, z_move(sub_config), NULL);

    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, NULL);

    z_owned_subscriber_t subscriber;
    z_declare_subscriber(&subscriber, z_loan(sub_session), z_loan(sub_keyexpr), z_move(callback), NULL);

    z_sleep_s(1);

    char buf[32] = {0};
    for (int i = 0; i < 5; ++i) {
        sprintf(buf, "data [%4d]", i);
        printf("Putting Data ('%s': '%s')...\n", PUB_KEY_EXPR, buf);
        z_publisher_put_options_t options;
        z_publisher_put_options_default(&options);

        z_owned_bytes_t payload;
        z_bytes_copy_from_str(&payload, buf);

        z_publisher_put(z_loan(publisher), z_move(payload), &options);
        z_sleep_s(1);
    }

    z_undeclare_publisher(z_move(publisher));
    z_undeclare_subscriber(z_move(subscriber));
    z_close(z_move(pub_session), NULL);
    z_close(z_move(sub_session), NULL);
    z_drop(z_move(pub_keyexpr));

    zc_stop_z_runtime();

    return 0;
}
