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

#define DEFAULT_SELECTOR "demo/example/**"
#define DEFAULT_VALUE NULL

struct args_t {
    char* selector;  // -s
    char* value;     // -v
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    z_owned_config_t config = z_config_default();
    struct args_t args = parse_args(argc, argv, &config);

    z_keyexpr_t keyexpr = z_keyexpr(args.selector);
    if (!z_check(keyexpr)) {
        printf("%s is not a valid key expression", args.selector);
        exit(-1);
    }

    printf("Opening session...\n");
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Sending Query '%s'...\n", args.selector);
    z_owned_reply_channel_t channel = zc_reply_fifo_new(16);
    z_get_options_t opts = z_get_options_default();
    if (args.value != NULL) {
        opts.value.payload = z_bytes_from_str(args.value);
    }
    z_get(z_loan(s), keyexpr, "", z_move(channel.send),
          &opts);  // here, the send is moved and will be dropped by zenoh when adequate
    z_owned_reply_t reply = z_reply_null();
    for (z_call(channel.recv, &reply); z_check(reply); z_call(channel.recv, &reply)) {
        if (z_reply_is_ok(&reply)) {
            z_sample_t sample = z_reply_ok(&reply);
            z_owned_str_t keystr = z_keyexpr_to_string(sample.keyexpr);
            printf(">> Received ('%s': '%.*s')\n", z_loan(keystr), (int)sample.payload.len, sample.payload.start);
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

void print_help() {
    printf(
        "\
    Usage: z_get [OPTIONS]\n\n\
    Options:\n\
        -s <SELECTOR> (optional, string, default='%s'): The selection of resources to query\n\
        -v <VALUE> (optional, string): An optional value to put in the query\n",
        DEFAULT_SELECTOR);
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
    char* selector = parse_opt(argc, argv, "s", true);
    if (!selector) {
        selector = DEFAULT_SELECTOR;
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
        free(pos_args);
        exit(-1);
    }
    free(pos_args);
    return (struct args_t){.selector = selector, .value = value};
}
