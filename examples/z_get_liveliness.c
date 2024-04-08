//
// Copyright (c) 2023 ZettaScale Technology
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
    char *expr = "group1/**";
    if (argc > 1) {
        expr = argv[1];
    }

    z_keyexpr_t keyexpr = z_keyexpr(expr);
    if (!z_check(keyexpr)) {
        printf("%s is not a valid key expression\n", expr);
        exit(-1);
    }

    z_owned_config_t config = z_config_default();
    if (argc > 2) {
        if (zc_config_insert_json(z_loan(config), Z_CONFIG_CONNECT_KEY, argv[2]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[2], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Sending liveliness query '%s'...\n", expr);
    z_owned_reply_channel_t channel = zc_reply_fifo_new(16);
    zc_liveliness_get(z_loan(s), keyexpr, z_move(channel.send), NULL);
    z_owned_reply_t reply = z_reply_null();
    for (z_call(channel.recv, &reply); z_check(reply); z_call(channel.recv, &reply)) {
        if (z_reply_is_ok(&reply)) {
            z_sample_t sample = z_reply_ok(&reply);
            z_owned_str_t keystr = z_keyexpr_to_string(z_sample_keyexpr(&sample));
            printf(">> Alive token ('%s')\n", z_loan(keystr));
            z_drop(z_move(keystr));
        } else {
            printf("Received an error\n");
        }
    }
    z_drop(z_move(reply));
    z_drop(z_move(channel));
    z_close(z_move(s));
    return 0;
}
