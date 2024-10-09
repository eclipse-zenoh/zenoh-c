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

    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);
    uint8_t* value = (uint8_t*)z_malloc(args.size);
    memset(value, 0, args.size);
    for (size_t i = 0; i < args.size; ++i) {
        value[i] = i % 10;
    }

    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_publisher_options_t options;
    z_publisher_options_default(&options);
    options.congestion_control = Z_CONGESTION_CONTROL_BLOCK;

    z_owned_publisher_t pub;
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);
    if (z_publisher_declare(&pub, z_loan(s), z_loan(ke), &options)) {
        printf("Unable to declare publisher for key expression!\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    z_owned_bytes_t payload;
    z_bytes_from_buf(&payload, value, args.size, NULL, NULL);
    while (1) {
        z_owned_bytes_t to_send;
        z_bytes_clone(&to_send, z_loan(payload));
        z_publisher_put(z_loan(pub), z_move(to_send), NULL);
    }

    z_drop(z_move(pub));
    z_drop(z_move(s));
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
    const char* arg = check_unknown_opts(argc, argv);
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
