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

int main(int argc, char **argv) {
    char *expr = "demo/example/**";
    if (argc > 1) {
        expr = argv[1];
    }
    z_view_keyexpr_t keyexpr;
    if (z_view_keyexpr_from_string(&keyexpr, expr) < 0) {
        printf("%s is not a valid key expression", expr);
        exit(-1);
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

    printf("Sending Query '%s'...\n", expr);
    z_get_options_t opts;
    z_get_options_default(&opts);
    opts.target = Z_QUERY_TARGET_ALL;
    z_owned_reply_channel_t channel;
    zc_reply_non_blocking_fifo_new(&channel, 16);
    z_get(z_loan(s), z_loan(keyexpr), "", z_move(channel.send),
          z_move(opts));  // here, the send is moved and will be dropped by zenoh when adequate
    z_owned_reply_t reply;
    for (bool call_success = z_call(z_loan(channel.recv), &reply); !call_success || z_check(reply);
        call_success = z_call(z_loan(channel.recv), &reply)) {
        if (!call_success) {
            continue;
        }
        if (z_reply_is_ok(z_loan(reply))) {
            const z_loaned_sample_t* sample = z_reply_ok(z_loan(reply));
            z_view_string_t key_str;
            z_owned_string_t payload_string;
            z_view_string_from_keyexpr(&key_str, z_sample_keyexpr(sample));
            z_bytes_decode_into_string(z_sample_payload(sample), &payload_string);
            printf(">> Received ('%.*s': '%.*s')\n", 
               (int)z_string_len(z_loan(key_str)), z_string_data(z_loan(key_str)),
               (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string))
            );
            z_drop(z_move(payload_string));
        } else {
            printf("Received an error\n");
        }
    }
    z_drop(z_move(reply));
    z_drop(z_move(channel));
    z_close(z_move(s));
    return 0;
}
