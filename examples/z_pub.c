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
//
#include <stdio.h>
#include <string.h>

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_KEYEXPR "demo/example/zenoh-c-pub"
#define DEFAULT_VALUE "Pub from C!"

struct args_t {
    char* keyexpr;               // -k
    char* value;                 // -v
    bool add_matching_listener;  // --add-matching-listener
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

void matching_status_handler(const zcu_matching_status_t* matching_status, void* arg) {
    if (matching_status->matching) {
        printf("Subscriber matched\n");
    } else {
        printf("No Subscribers matched\n");
    }
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

    printf("Declaring Publisher on '%s'...\n", args.keyexpr);
    z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(args.keyexpr), NULL);
    if (!z_check(pub)) {
        printf("Unable to declare Publisher for key expression!\n");
        exit(-1);
    }

    zcu_owned_matching_listener_t listener;
    if (args.add_matching_listener) {
        zcu_owned_closure_matching_status_t callback = z_closure(matching_status_handler);
        listener = zcu_publisher_matching_listener_callback(z_loan(pub), z_move(callback));
    }

    printf("Press CTRL-C to quit...\n");
    char buf[256];
    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);
        sprintf(buf, "[%4d] %s", idx, args.value);
        printf("Putting Data ('%s': '%s')...\n", args.keyexpr, buf);
        z_publisher_put_options_t options = z_publisher_put_options_default();
        options.encoding = z_encoding(Z_ENCODING_PREFIX_TEXT_PLAIN, NULL);
        z_publisher_put(z_loan(pub), (const uint8_t*)buf, strlen(buf), &options);
    }

    z_undeclare_publisher(z_move(pub));

    z_close(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_pub [OPTIONS]\n\n\
    Options:\n\
        -k <KEYEXPR> (optional, string, default='%s'): The key expression to write to\n\
        -v <VALUE> (optional, string, default='%s'): The value to write\n\
        --add-matching-listener (optional): Add matching listener\n",
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
    const char* arg = parse_opt(argc, argv, "add-matching-listener", false);
    bool add_matching_listener = false;
    if (arg) {
        add_matching_listener = true;
    }
    parse_zenoh_common_args(argc, argv, config);
    arg = check_unknown_opts(argc, argv);
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
    return (struct args_t){.keyexpr = (char*)keyexpr, .value = (char*)value, .add_matching_listener = add_matching_listener};
}
