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
    char *value = NULL;
    switch (argc) {
        default:
        case 3:
            value = argv[2];
        case 2:
            expr = argv[1];
            break;
        case 1:
            // Do nothing
            break;
    }
    z_owned_keyexpr_t keyexpr;
    if (z_keyexpr(&keyexpr, expr) < 0) {
        printf("%s is not a valid key expression", expr);
        exit(-1);
    }
    z_owned_config_t config;
    z_config_default(&config);
    if (argc > 3) {
        if (zc_config_insert_json(z_loan(config), Z_CONFIG_CONNECT_KEY, argv[3]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[3], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (!z_open(&s, z_move(config))) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Sending Query '%s'...\n", expr);
    z_owned_reply_channel_t channel = zc_reply_fifo_new(16);
    z_get_options_t opts = z_get_options_default();
    z_owned_bytes_t payload;
    if (value != NULL) {
        z_bytes_encode_from_string(&payload, value);
        opts.payload = &payload;
    }
    z_get(z_loan(s), z_loan(keyexpr), "", z_move(channel.send),
          z_move(opts));  // here, the send is moved and will be dropped by zenoh when adequate
    z_owned_reply_t reply;
    z_owned_str_t reply_str;
    for (z_call(channel.recv, &reply); z_check(reply); z_call(channel.recv, &reply)) {
        if (z_reply_is_ok(z_loan(reply))) {
            z_loaned_sample_t sample = z_reply_ok(z_loan(reply));
            z_owned_str_t key_str = z_loaned_keyexpr_to_string(z_sample_keyexpr(&sample));
            z_bytes_decode_into_string(z_sample_payload(&sample), &reply_str);
            printf(">> Received ('%s': '%s')\n", z_loan(key_str), z_loan(reply_str));
            z_drop(z_move(reply_str));
            z_drop(z_move(key_str));
        } else {
            printf("Received an error\n");
        }
    }
    z_drop(z_move(keyexpr));
    z_drop(z_move(reply));
    z_drop(z_move(channel));
    z_close(z_move(s));
    return 0;
}