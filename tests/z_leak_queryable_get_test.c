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

const char *GET_KEY_EXPR = "test/valgrind/data";
const char *QUERYABLE_KEY_EXPR = "test/valgrind/**";

void query_handler(z_loaned_query_t *query, void *context) {
    (void)context;
    z_view_string_t key_string;
    z_keyexpr_as_view_string(z_query_keyexpr(query), &key_string);

    z_view_string_t params;
    z_query_parameters(query, &params);

    const z_loaned_bytes_t *payload = z_query_payload(query);

    z_owned_string_t payload_string;
    z_bytes_deserialize_into_string(payload, &payload_string);

    printf(">> [Queryable ] Received Query '%.*s' with value '%.*s'\n", (int)z_string_len(z_loan(key_string)),
           z_string_data(z_loan(key_string)), (int)z_string_len(z_loan(payload_string)),
           z_string_data(z_loan(payload_string)));
    z_drop(z_move(payload_string));
    z_query_reply_options_t options;
    z_query_reply_options_default(&options);

    z_owned_bytes_t reply_payload;
    z_bytes_clone(&reply_payload, payload);

    z_query_reply(query, z_query_keyexpr(query), z_move(reply_payload), &options);
}

int main(int argc, char **argv) {
    printf("Declaring Queryable on %s\n", QUERYABLE_KEY_EXPR);

    z_owned_keyexpr_t queryable_keyexpr;
    z_keyexpr_from_str(&queryable_keyexpr, QUERYABLE_KEY_EXPR);

    z_owned_config_t queryable_config;
    z_config_default(&queryable_config);

    z_owned_session_t queryable_session;
    z_open(&queryable_session, z_move(queryable_config), NULL);

    z_owned_closure_query_t callback;
    z_closure(&callback, query_handler, NULL, NULL);
    z_owned_queryable_t queryable;
    z_declare_queryable(&queryable, z_loan(queryable_session), z_loan(queryable_keyexpr), z_move(callback), NULL);

    z_view_keyexpr_t get_keyexpr;
    z_view_keyexpr_from_str(&get_keyexpr, GET_KEY_EXPR);

    z_owned_config_t get_config;
    z_config_default(&get_config);

    z_owned_session_t get_session;
    z_open(&get_session, z_move(get_config), NULL);

    z_sleep_s(1);

    size_t received_replies = 0;
    char buf[32] = {0};
    for (int i = 0; i < 5; ++i) {
        sprintf(buf, "data [%4d]", i);
        printf("Get with Data ('%s': '%s')...\n", GET_KEY_EXPR, buf);
        z_get_options_t options;
        z_get_options_default(&options);

        z_owned_bytes_t payload;
        z_bytes_serialize_from_str(&payload, buf);

        options.payload = z_move(payload);

        z_owned_fifo_handler_reply_t handler;
        z_owned_closure_reply_t closure;
        z_fifo_channel_reply_new(&closure, &handler, 16);

        z_get(z_loan(get_session), z_loan(get_keyexpr), "", z_move(closure), &options);

        z_owned_reply_t reply;
        while (z_recv(z_loan(handler), &reply) == Z_OK) {
            received_replies++;
            const z_loaned_sample_t *sample = z_reply_ok(z_loan(reply));
            assert(sample != NULL);

            z_view_string_t key_str;
            z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_str);

            z_owned_string_t reply_str;
            z_bytes_deserialize_into_string(z_sample_payload(sample), &reply_str);

            printf(">> Received ('%.*s': '%.*s')\n", (int)z_string_len(z_loan(key_str)), z_string_data(z_loan(key_str)),
                   (int)z_string_len(z_loan(reply_str)), z_string_data(z_loan(reply_str)));
            z_drop(z_move(reply_str));
            z_drop(z_move(reply));
        }

        z_drop(z_move(handler));
        z_sleep_s(1);
    }
    assert(received_replies == 5);

    z_undeclare_queryable(z_move(queryable));
    z_close(z_move(get_session), NULL);
    z_close(z_move(queryable_session), NULL);
    z_drop(z_move(queryable_keyexpr));

    zc_stop_z_runtime();

    return 0;
}
