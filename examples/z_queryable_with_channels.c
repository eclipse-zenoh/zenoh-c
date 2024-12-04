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

struct args_t {
    char* keyexpr;  // -k
    char* value;    // -v
    bool complete;  // --complete
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

z_view_keyexpr_t ke;

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

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
    z_owned_fifo_handler_query_t handler;
    z_owned_closure_query_t closure;
    z_fifo_channel_query_new(&closure, &handler, 16);
    z_owned_queryable_t qable;

    z_queryable_options_t opts;
    z_queryable_options_default(&opts);
    opts.complete = args.complete;

    if (z_declare_queryable(z_loan(s), &qable, z_loan(ke), z_move(closure), &opts) < 0) {
        printf("Unable to create queryable.\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    z_owned_query_t oquery;
    for (z_result_t res = z_recv(z_loan(handler), &oquery); res == Z_OK; res = z_recv(z_loan(handler), &oquery)) {
        const z_loaned_query_t* query = z_loan(oquery);
        z_view_string_t key_string;
        z_keyexpr_as_view_string(z_query_keyexpr(query), &key_string);

        z_view_string_t params;
        z_query_parameters(query, &params);

        const z_loaned_bytes_t* payload = z_query_payload(query);
        if (payload != NULL && z_bytes_len(payload) > 0) {
            z_owned_string_t payload_string;
            z_bytes_to_string(payload, &payload_string);

            printf(">> [Queryable ] Received Query '%.*s?%.*s' with value '%.*s'\n",
                   (int)z_string_len(z_loan(key_string)), z_string_data(z_loan(key_string)),
                   (int)z_string_len(z_loan(params)), z_string_data(z_loan(params)),
                   (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string)));
            z_drop(z_move(payload_string));
        } else {
            printf(">> [Queryable ] Received Query '%.*s?%.*s'\n", (int)z_string_len(z_loan(key_string)),
                   z_string_data(z_loan(key_string)), (int)z_string_len(z_loan(params)), z_string_data(z_loan(params)));
        }
        z_query_reply_options_t options;
        z_query_reply_options_default(&options);

        z_owned_bytes_t reply_payload;
        z_bytes_from_static_str(&reply_payload, args.value);
        z_query_reply(query, z_loan(ke), z_move(reply_payload), &options);
        z_drop(z_move(oquery));
    }

    z_drop(z_move(qable));
    z_drop(z_move(handler));
    z_drop(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_queryable_with_channels [OPTIONS]\n\n\
    Options:\n\
        -k <KEYEXPR> (optional, string, default='%s'): The key expression matching queries to reply to\n\
        -v <VALUE> (optional, string, default='%s'): The value to reply to queries with\n\
        --complete (optional, flag to indicate whether queryable is complete or not)",
        DEFAULT_KEYEXPR, DEFAULT_VALUE);
    printf(COMMON_HELP);
    printf(
        "\
        -h: print help\n");
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    if (parse_opt(argc, argv, "h", false)) {
        print_help();
        exit(1);
    }
    const char* keyexpr = parse_opt(argc, argv, "k", true);
    if (!keyexpr) {
        keyexpr = DEFAULT_KEYEXPR;
    }
    const char* value = parse_opt(argc, argv, "v", true);
    if (!value) {
        value = DEFAULT_VALUE;
    }
    const char* complete_arg = parse_opt(argc, argv, "complete", false);
    bool complete = false;
    if (complete_arg) {
        complete = true;
    }

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
    return (struct args_t){.keyexpr = (char*)keyexpr, .value = (char*)value, .complete = complete};
}
