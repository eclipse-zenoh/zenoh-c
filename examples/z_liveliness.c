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
    char *expr = "group1/zenoh-rs";
    if (argc > 1) {
        expr = argv[1];
    }

    z_loaned_keyexpr_t keyexpr = z_keyexpr(expr);
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

    printf("Declaring liveliness token '%s'...\n", expr);
    zc_owned_liveliness_token_t token = zc_liveliness_declare_token(z_loan(s), keyexpr, NULL);
    if (!z_check(token)) {
        printf("Unable to create liveliness token!\n");
        exit(-1);
    }

    printf("Enter 'd' to undeclare liveliness token, 'q' to quit...\n");
    char c = 0;
    while (c != 'q') {
        c = getchar();
        if (c == -1) {
            z_sleep_s(1);
        } else if (c == 'd') {
            printf("Undeclaring liveliness token...\n");
            z_drop(z_move(token));
        }
    }

    z_drop(z_move(token));
    z_close(z_move(s));
    return 0;
}
