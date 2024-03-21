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
#include <zenoh_macros.h>

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_KEYEXPR "demo/example/zenoh-c-queryable"
#define DEFAULT_VALUE "Queryable from C!"

struct args_t {
    char* keyexpr;  // -k
    char* value;    // -v
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

void query_handler(const z_query_t* query, void* context) {
    z_owned_closure_owned_query_t* channel = (z_owned_closure_owned_query_t*)context;
    z_owned_query_t oquery = z_query_clone(query);
    z_call(*channel, &oquery);
}

int main(int argc, char** argv) {
    z_owned_config_t config = z_config_default();
    struct args_t args = parse_args(argc, argv, &config);

    printf("Opening session...\n");
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        printf("Unable to open session!\n");
        exit(-1);
    }
    z_keyexpr_t keyexpr = z_keyexpr(args.keyexpr);
    if (!z_check(keyexpr)) {
        printf("%s is not a valid key expression", args.keyexpr);
        exit(-1);
    }

    printf("Declaring Queryable on '%s'...\n", args.keyexpr);
    z_owned_query_channel_t channel = zc_query_fifo_new(16);
    z_owned_closure_query_t callback = z_closure(query_handler, NULL, &channel.send);
    z_owned_queryable_t qable = z_declare_queryable(z_loan(s), keyexpr, z_move(callback), NULL);
    if (!z_check(qable)) {
        printf("Unable to create queryable.\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    z_owned_query_t oquery = z_query_null();
    for (z_call(channel.recv, &oquery); z_check(oquery); z_call(channel.recv, &oquery)) {
        z_query_t query = z_loan(oquery);
        z_owned_str_t keystr = z_keyexpr_to_string(z_query_keyexpr(&query));
        z_bytes_t pred = z_query_parameters(&query);
        z_value_t payload_value = z_query_value(&query);
        if (payload_value.payload.len > 0) {
            printf(">> [Queryable ] Received Query '%s?%.*s' with value '%.*s'\n", z_loan(keystr), (int)pred.len,
                   pred.start, (int)payload_value.payload.len, payload_value.payload.start);
        } else {
            printf(">> [Queryable ] Received Query '%s?%.*s'\n", z_loan(keystr), (int)pred.len, pred.start);
        }
        z_query_reply_options_t options = z_query_reply_options_default();
        options.encoding = z_encoding(Z_ENCODING_PREFIX_TEXT_PLAIN, NULL);
        z_query_reply(&query, keyexpr, (const unsigned char*)args.value, strlen(args.value), &options);
        z_drop(z_move(keystr));
        z_drop(z_move(oquery));
    }

    z_drop(z_move(qable));
    z_drop(z_move(channel));
    z_drop(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_queryable_with_channels [OPTIONS]\n\n\
    Options:\n\
        -k <KEYEXPR> (optional, string, default='%s'): The key expression matching queries to reply to\n\
        -v <VALUE> (optional, string, default='%s'): The value to reply to queries with\n",
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
    char* keyexpr = parse_opt(argc, argv, "k", true);
    if (!keyexpr) {
        keyexpr = DEFAULT_KEYEXPR;
    }
    char* value = parse_opt(argc, argv, "v", true);
    if (!value) {
        value = DEFAULT_VALUE;
    }
    parse_zenoh_common_args(argc, argv, config);
    char* arg = check_unknown_opts(argc, argv);
    if (arg) {
        printf("Unknown option %s\n", arg);
        exit(-1);
    }
    char** pos_args = parse_pos_args(argc, argv, 1);
    if (!pos_args || pos_args[0]) {
        printf("Unexpected positional arguments\n");
        exit(-1);
    }
    return (struct args_t){.keyexpr = keyexpr, .value = value};
}
