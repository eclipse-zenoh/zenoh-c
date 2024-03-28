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
#include <stdint.h>
#include <stdio.h>

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_KEYEXPR "demo/example/**"

struct args_t {
    char* keyexpr;  // -k
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

const char* kind_to_str(z_sample_kind_t kind);

int8_t attachment_reader(z_bytes_t key, z_bytes_t val, void* ctx) {
    printf("   attachment: %.*s: '%.*s'\n", (int)key.len, key.start, (int)val.len, val.start);
    return 0;
}

void data_handler(const z_sample_t* sample, void* arg) {
    z_owned_str_t keystr = z_keyexpr_to_string(sample->keyexpr);
    printf(">> [Subscriber] Received %s ('%s': '%.*s')\n", kind_to_str(sample->kind), z_loan(keystr),
           (int)sample->payload.len, sample->payload.start);

    // checks if attachment exists
    if (z_check(sample->attachment)) {
        // reads full attachment
        z_attachment_iterate(sample->attachment, attachment_reader, NULL);

        // reads particular attachment item
        z_bytes_t index = z_attachment_get(sample->attachment, z_bytes_from_str("index"));
        if (z_check(index)) {
            printf("   message number: %.*s\n", (int)index.len, index.start);
        }
    }
    z_drop(z_move(keystr));
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

    z_owned_closure_sample_t callback = z_closure(data_handler);
    printf("Declaring Subscriber on '%s'...\n", args.keyexpr);
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(args.keyexpr), z_move(callback), NULL);
    if (!z_check(sub)) {
        printf("Unable to declare subscriber.\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    while (1) {
        z_sleep_s(1);
    }

    z_undeclare_subscriber(z_move(sub));
    z_close(z_move(s));
    return 0;
}

const char* kind_to_str(z_sample_kind_t kind) {
    switch (kind) {
        case Z_SAMPLE_KIND_PUT:
            return "PUT";
        case Z_SAMPLE_KIND_DELETE:
            return "DELETE";
        default:
            return "UNKNOWN";
    }
}

void print_help() {
    printf(
        "\
    Usage: z_sub_attachement [OPTIONS]\n\n\
    Options:\n\
        -k <KEY> (optional, string, default='%s'): The key expression to subscribe to\n",
        DEFAULT_KEYEXPR);
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
    return (struct args_t){.keyexpr = keyexpr};
}
