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

struct args_t {
    unsigned int size;  // positional_1
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    char* keyexpr = "test/thr";

    z_owned_config_t config = z_config_default();
    struct args_t args = parse_args(argc, argv, &config);
    uint8_t* value = (uint8_t*)z_malloc(args.size);
    memset(value, 1, args.size);

    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_publisher_options_t options = z_publisher_options_default();
    options.congestion_control = Z_CONGESTION_CONTROL_BLOCK;

    z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(keyexpr), &options);
    if (!z_check(pub)) {
        printf("Unable to declare publisher for key expression!\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    while (1) {
        z_publisher_put(z_loan(pub), (const uint8_t*)value, args.size, NULL);
    }

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));
}

void print_help() {
    printf(
        "\
    Usage: z_pub_thr [OPTIONS] <PAYLOAD_SIZE>\n\n\
    Arguments:\n\
        <PAYLOAD_SIZE> (required, int): Size of the payload to publish\n\n\
    Options:\n");
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
    parse_zenoh_common_args(argc, argv, config);
    char* arg = check_unknown_opts(argc, argv);
    if (arg) {
        printf("Unknown option %s\n", arg);
        exit(-1);
    }
    char** pos_args = parse_pos_args(argc, argv, 1);
    if (!pos_args) {
        printf("Unexpected additional positional arguments\n");
        exit(-1);
    }
    if (!pos_args[0]) {
        printf("<PAYLOAD_SIZE> argument is required\n");
        free(pos_args);
        exit(-1);
    }
    unsigned int size = atoi(pos_args[0]);
    free(pos_args);
    return (struct args_t){.size = size};
}
