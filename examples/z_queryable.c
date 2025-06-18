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

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_KEYEXPR "demo/example/zenoh-c-queryable"
#define DEFAULT_VALUE "Queryable from C!"
z_view_keyexpr_t ke;

struct args_t {
    char *keyexpr;  // -k, --key
    char *value;    // -p, --payload
    bool complete;  // --complete
};

char *value;

struct args_t parse_args(int argc, char **argv, z_owned_config_t *config);

void query_handler(z_loaned_query_t *query, void *context) {
    z_view_string_t key_string;
    z_keyexpr_as_view_string(z_query_keyexpr(query), &key_string);

    z_view_string_t params;
    z_query_parameters(query, &params);

    const z_loaned_bytes_t *payload = z_query_payload(query);
    if (payload != NULL && z_bytes_len(payload) > 0) {
        z_owned_string_t payload_string;
        z_bytes_to_string(payload, &payload_string);

        printf(">> [Queryable ] Received Query '%.*s?%.*s' with value '%.*s'\n", (int)z_string_len(z_loan(key_string)),
               z_string_data(z_loan(key_string)), (int)z_string_len(z_loan(params)), z_string_data(z_loan(params)),
               (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string)));
        z_drop(z_move(payload_string));
    } else {
        printf(">> [Queryable ] Received Query '%.*s?%.*s'\n", (int)z_string_len(z_loan(key_string)),
               z_string_data(z_loan(key_string)), (int)z_string_len(z_loan(params)), z_string_data(z_loan(params)));
    }
    z_query_reply_options_t options;
    z_query_reply_options_default(&options);

    z_owned_bytes_t reply_payload;
    z_bytes_from_static_str(&reply_payload, (char *)value);

    z_view_keyexpr_t reply_keyexpr;
    z_view_keyexpr_from_str(&reply_keyexpr, (const char *)context);
    printf(">> [Queryable ] Responding ('%s': '%s')\n", (const char *)context, value);

    z_query_reply(query, z_loan(reply_keyexpr), z_move(reply_payload), &options);
}

int main(int argc, char **argv) {
    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);
    value = args.value;

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    if (z_view_keyexpr_from_str(&ke, args.keyexpr) < 0) {
        printf("%s is not a valid key expression", args.keyexpr);
        exit(-1);
    }

    printf("Declaring Queryable on '%s'...\n", args.keyexpr);
    z_owned_closure_query_t callback;
    z_closure(&callback, query_handler, NULL, (void *)args.keyexpr);
    z_owned_queryable_t qable;

    z_queryable_options_t opts;
    z_queryable_options_default(&opts);
    opts.complete = args.complete;

    if (z_declare_queryable(z_loan(s), &qable, z_loan(ke), z_move(callback), &opts) < 0) {
        printf("Unable to create queryable.\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    while (1) {
        z_sleep_s(1);
    }

    z_drop(z_move(qable));
    z_drop(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_queryable [OPTIONS]\n\n\
    Options:\n\
        -k, --key <KEYEXPR> (optional, string, default='%s'): The key expression matching queries to reply to\n\
        -p, --payload <PAYLOAD> (optional, string, default='%s'): The value to reply to queries with\n\
        --complete (optional): Indicates whether queryable is complete or not",
        DEFAULT_KEYEXPR, DEFAULT_VALUE);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char **argv, z_owned_config_t *config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", "key", (char *), (char *)DEFAULT_KEYEXPR);
    _Z_PARSE_ARG(args.value, "p", "payload", (char *), (char *)DEFAULT_VALUE);
    args.complete = _Z_CHECK_FLAG("complete");

    parse_zenoh_common_args(argc, argv, config);
    const char *arg = check_unknown_opts(argc, argv);
    if (arg) {
        printf("Unknown option %s\n", arg);
        exit(-1);
    }
    char **pos_args = parse_pos_args(argc, argv, 1);
    if (!pos_args || pos_args[0]) {
        printf("Unexpected positional arguments\n");
        free(pos_args);
        exit(-1);
    }
    free(pos_args);
    return args;
}
