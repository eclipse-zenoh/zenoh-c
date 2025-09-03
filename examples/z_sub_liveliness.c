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
//
#include <stdio.h>

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_KEYEXPR "group1/**"

struct args_t {
    char* keyexpr;  // -k, --key
    bool history;   // --history
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

void data_handler(z_loaned_sample_t* sample, void* arg) {
    z_view_string_t key_string;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_string);
    switch (z_sample_kind(sample)) {
        case Z_SAMPLE_KIND_PUT:
            printf(">> [LivelinessSubscriber] New alive token ('%.*s')\n", (int)z_string_len(z_loan(key_string)),
                   z_string_data(z_loan(key_string)));
            break;
        case Z_SAMPLE_KIND_DELETE:
            printf(">> [LivelinessSubscriber] Dropped token ('%.*s')\n", (int)z_string_len(z_loan(key_string)),
                   z_string_data(z_loan(key_string)));
            break;
    }
}

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);
    z_view_keyexpr_t ke;
    if (z_view_keyexpr_from_str(&ke, args.keyexpr) < 0) {
        printf("%s is not a valid key expression\n", args.keyexpr);
        exit(-1);
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring liveliness subscriber on '%s'...\n", args.keyexpr);
    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, NULL);
    z_liveliness_subscriber_options_t opts;
    z_liveliness_subscriber_options_default(&opts);
    opts.history = args.history;
    z_owned_subscriber_t sub;
    if (z_liveliness_declare_subscriber(z_loan(s), &sub, z_loan(ke), z_move(callback), &opts) < 0) {
        printf("Unable to declare liveliness subscriber.\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    while (1) {
        z_sleep_s(1);
    }

    z_drop(z_move(sub));
    z_drop(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_sub_liveliness [OPTIONS]\n\n\
    Options:\n\
        -k, --key <KEYEXPR> (optional, string, default='%s'): The key expression matching liveliness tokens to subscribe to\n\
        --history (optional): Get historical liveliness tokens.\n",
        DEFAULT_KEYEXPR);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", "key", (char*), (char*)DEFAULT_KEYEXPR);
    args.history = _Z_CHECK_FLAG("history");

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
