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

const char *keyexpr = "demo/example/zenoh-c-queryable";
const char *value = "Queryable from C!";
z_view_keyexpr_t ke;

void query_handler(const z_loaned_query_t *query, void *context) {
    z_owned_str_t key_string;
    z_keyexpr_to_string(z_query_keyexpr(query), &key_string);

    z_view_slice_t params;
    z_query_parameters(query, &params);

    const z_loaned_bytes_t* payload =  z_value_payload(z_query_value(query));
    if (z_bytes_len(payload) > 0) {
        z_owned_str_t payload_string;
        z_bytes_decode_into_string(payload, &payload_string);

        printf(">> [Queryable ] Received Query '%s?%.*s' with value '%s'\n", 
            z_str_data(z_loan(key_string)), (int)z_slice_len(z_loan(params)), (const char*)z_slice_data(z_loan(params)),
            z_str_data(z_loan(payload_string)));
        z_drop(z_move(payload_string));
    } else {
        printf(">> [Queryable ] Received Query '%s?%.*s'\n", z_str_data(z_loan(key_string)),
        (int)z_slice_len(z_loan(params)), (const char*)z_slice_data(z_loan(params)));
    }
    z_query_reply_options_t options;
    z_query_reply_options_default(&options);
    
    z_view_str_t reply_string;
    z_view_str_wrap(&reply_string, value);
    z_owned_bytes_t reply_payload;
    z_bytes_encode_from_string(&reply_payload, z_loan(reply_string));

    z_view_keyexpr_t reply_keyexpr;
    z_view_keyexpr_new(&reply_keyexpr, (const char *)context);

    z_query_reply(query, z_loan(reply_keyexpr), z_move(reply_payload), &options);
    z_drop(z_move(key_string));
}

int main(int argc, char **argv) {
    if (argc > 1) {
        keyexpr = argv[1];
    }
    z_owned_config_t config;
    z_config_default(&config);
    if (argc > 2) {
        if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_CONNECT_KEY, argv[2]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[2], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config))) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    if (z_view_keyexpr_new(&ke, keyexpr)) {
        printf("%s is not a valid key expression", keyexpr);
        exit(-1);
    }

    printf("Declaring Queryable on '%s'...\n", keyexpr);
    z_owned_closure_query_t callback;
    z_closure(&callback, query_handler, NULL, (void*)keyexpr);
    z_owned_queryable_t qable;

    if (z_declare_queryable(&qable, z_loan(s), z_loan(ke), z_move(callback), NULL) < 0) {
        printf("Unable to create queryable.\n");
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

    z_undeclare_queryable(z_move(qable));
    z_close(z_move(s));
    return 0;
}
