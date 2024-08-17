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

#define DEFAULT_KEYEXPR "demo/example/zenoh-c-put"
#define DEFAULT_VALUE "Put from C!"

struct args_t {
    char* keyexpr;  // -k
    char* value;    // -v
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config)) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Putting Data ('%s': '%s')...\n", args.keyexpr, args.value);

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, args.keyexpr);

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, args.value);

    z_owned_bytes_t attachment, key, val;
    z_bytes_from_static_str(&key, (char*)"hello");
    z_bytes_from_static_str(&val, (char*)"there");
    z_bytes_from_pair(&attachment, z_move(key), z_move(val));

    z_put_options_t options;
    z_put_options_default(&options);
    options.attachment =
        z_move(attachment);  // attachement is going to be consumed by z_put, so no need to drop it manually

    int res = z_put(z_loan(s), z_loan(ke), z_move(payload), &options);
    if (res < 0) {
        printf("Put failed...\n");
    }

    z_close(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_put [OPTIONS]\n\n\
    Options:\n\
        -k <KEYEXPR> (optional, string, default='%s'): The key expression to write to\n\
        -v <VALUE> (optional, string, default='%s'): The value to write\n",
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
    return (struct args_t){.keyexpr = (char*)keyexpr, .value = (char*)value};
}
