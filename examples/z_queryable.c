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
#if defined(WIN32) || defined(_WIN32) || defined(__WIN32) && !defined(__CYGWIN__)
#include <windows.h>
#define sleep(x) Sleep(x * 1000)
#else
#include <unistd.h>
#endif
#include "zenoh.h"

char *expr = "demo/example/zenoh-c-queryable";
char *value = "Queryable from C!";
z_keyexpr_t keyexpr;

void query_handler(z_query_t query, const void *context) {
    char *keystr = z_keyexpr_to_string(z_query_keyexpr(query));
    z_bytes_t pred = z_query_value_selector(query);
    printf(">> [Queryable ] Received Query '%s%.*s'\n", keystr, (int)pred.len, pred.start);
    z_query_reply_options_t options = z_query_reply_options_default();
    options.encoding = z_encoding(Z_ENCODING_PREFIX_TEXT_PLAIN, NULL);
    z_query_reply(query, z_keyexpr(keystr), (const unsigned char *)value, strlen(value), &options);
    free(keystr);
}

int main(int argc, char **argv) {
    z_init_logger();

    if (argc > 1) {
        expr = argv[1];
    }
    z_owned_config_t config = z_config_default();
    if (argc > 2) {
        if (!zc_config_insert_json(z_loan(config), Z_CONFIG_CONNECT_KEY, argv[2])) {
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
    keyexpr = z_keyexpr(expr);
    if (!z_check(keyexpr)) {
        printf("%s is not a valid key expression", expr);
        exit(-1);
    }

    printf("Creating Queryable on '%s'...\n", expr);
    z_owned_closure_query_t callback = z_closure(query_handler);
    z_owned_queryable_t qable = z_declare_queryable(z_loan(s), keyexpr, z_move(callback), NULL);
    if (!z_check(qable)) {
        printf("Unable to create queryable.\n");
        exit(-1);
    }

    printf("Enter 'q' to quit...\n");
    char c = 0;
    while (c != 'q') {
        c = getchar();
        if (c == -1) {
            sleep(1);
        }
    }

    z_undeclare_queryable(z_move(qable));
    z_close(z_move(s));
    return 0;
}
