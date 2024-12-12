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

#define DEFAULT_KEYEXPR "group1/zenoh-rs"

struct args_t {
    char* keyexpr;  // -k, --key
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

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring liveliness token '%s'...\n", args.keyexpr);
    z_owned_liveliness_token_t token;
    if (z_liveliness_declare_token(z_loan(s), &token, z_loan(keyexpr), NULL) < 0) {
        printf("Unable to create liveliness token!\n");
        exit(-1);
    }

    printf("Press CTRL-C to undeclare liveliness token and quit...\n");
    while (1) {
        z_sleep_s(1);
    }

    // LivelinessTokens are automatically closed when dropped
    // Use the code below to manually undeclare it if needed
    printf("Undeclaring liveliness token...\n");
    z_drop(z_move(token));

    z_drop(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_liveliness [OPTIONS]\n\n\
    Options:\n\
        -k, --key <KEYEXPR> (optional, string, default='%s'): The key expression the liveliness token\n",
        DEFAULT_KEYEXPR);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", "key", (char*), (char*)DEFAULT_KEYEXPR);
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
