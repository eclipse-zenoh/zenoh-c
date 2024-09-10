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

struct args_t {
    char* selector;  // -s
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

    z_view_keyexpr_t keyexpr;
    if (z_view_keyexpr_from_str(&keyexpr, args.selector) < 0) {
        printf("%s is not a valid key expression", args.selector);
        exit(-1);
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Sending Query '%s'...\n", args.selector);
    z_get_options_t opts;
    z_get_options_default(&opts);
    opts.target = Z_QUERY_TARGET_ALL;
    z_owned_fifo_handler_reply_t handler;
    z_owned_closure_reply_t closure;
    z_fifo_channel_reply_new(&closure, &handler, 16);
    z_get(z_loan(s), z_loan(keyexpr), "", z_move(closure),
          &opts);  // here, the closure is moved and will be dropped by zenoh when adequate
    z_owned_reply_t reply;
    for (z_result_t res = z_try_recv(z_loan(handler), &reply); res != Z_CHANNEL_DISCONNECTED;
         res = z_try_recv(z_loan(handler), &reply)) {
        if (res != Z_OK) {
            z_sleep_ms(50);
            continue;
        }
        if (z_reply_is_ok(z_loan(reply))) {
            const z_loaned_sample_t* sample = z_reply_ok(z_loan(reply));
            z_view_string_t key_str;
            z_owned_string_t payload_string;
            z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_str);
            z_bytes_deserialize_into_string(z_sample_payload(sample), &payload_string);
            printf(">> Received ('%.*s': '%.*s')\n", (int)z_string_len(z_loan(key_str)), z_string_data(z_loan(key_str)),
                   (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string)));
            z_drop(z_move(payload_string));
        } else {
            printf("Received an error\n");
        }
    }
    z_drop(z_move(reply));
    z_drop(z_move(handler));
    z_close(z_move(s), NULL);
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_non_blocking_get [OPTIONS]\n\n\
    Options:\n\
        -s <SELECTOR> (optional, string, default='%s'): The selection of resources to query\n",
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
    const char* selector = parse_opt(argc, argv, "s", true);
    if (!selector) {
        selector = DEFAULT_SELECTOR;
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
    return (struct args_t){.selector = (char*)selector};
}
