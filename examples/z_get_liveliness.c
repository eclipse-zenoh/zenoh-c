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

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_KEYEXPR "group1/**"
#define DEFAULT_TIMEOUT_MS 10000

struct args_t {
    char* keyexpr;        // -k, --key
    uint64_t timeout_ms;  // -o, --timeout
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

    z_view_keyexpr_t keyexpr;
    if (z_view_keyexpr_from_str(&keyexpr, args.keyexpr) < 0) {
        printf("%s is not a valid key expression\n", args.keyexpr);
        exit(-1);
    }

    z_owned_session_t s;
    printf("Opening session...\n");
    if (z_open(&s, z_move(config), NULL) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Sending liveliness query '%s'...\n", args.keyexpr);
    z_owned_fifo_handler_reply_t handler;
    z_owned_closure_reply_t closure;
    z_fifo_channel_reply_new(&closure, &handler, 16);
    z_liveliness_get_options_t opts;
    z_liveliness_get_options_default(&opts);
    opts.timeout_ms = args.timeout_ms;
    z_liveliness_get(z_loan(s), z_loan(keyexpr), z_move(closure), &opts);
    z_owned_reply_t reply;
    while (z_recv(z_loan(handler), &reply) == Z_OK) {
        if (z_reply_is_ok(z_loan(reply))) {
            const z_loaned_sample_t* sample = z_reply_ok(z_loan(reply));
            z_view_string_t key_str;
            z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_str);
            printf(">> Alive token ('%.*s')\n", (int)z_string_len(z_loan(key_str)), z_string_data(z_loan(key_str)));
        } else {
            printf("Received an error\n");
        }
    }

    z_drop(z_move(reply));
    z_drop(z_move(handler));
    z_drop(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_get_liveliness [OPTIONS]\n\n\
    Options:\n\
        -k, --key <KEYEXPR> (optional, string, default='%s'): The key expression to query\n\
        -o, --timeout <TIMEOUT_MS> (optional, number, default = '%d'): Query timeout in milliseconds\n",
        DEFAULT_KEYEXPR, DEFAULT_TIMEOUT_MS);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", "key", (char*), (char*)DEFAULT_KEYEXPR);
    _Z_PARSE_ARG(args.timeout_ms, "o", "timeout", atoi, DEFAULT_TIMEOUT_MS);

    parse_zenoh_common_args(argc, argv, config);
    const char* arg = check_unknown_opts(argc, argv);
    if (arg) {
        printf("Unknown option %s\n", arg);
        exit(-1);
    }
    char** pos_args = parse_pos_args(argc, argv, 1);
    if (!pos_args || pos_args[0]) {
        printf("Unexpected positional arguments\n");
        free(pos_args);
        exit(-1);
    }
    free(pos_args);
    return args;
}
