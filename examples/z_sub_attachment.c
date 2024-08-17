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

void data_handler(const z_loaned_sample_t* sample, void* arg) {
    z_view_string_t key_string;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_string);

    z_owned_string_t payload_string;
    z_bytes_deserialize_into_string(z_sample_payload(sample), &payload_string);

    printf(">> [Subscriber] Received %s ('%.*s': '%.*s')\n", kind_to_str(z_sample_kind(sample)),
           (int)z_string_len(z_loan(key_string)), z_string_data(z_loan(key_string)),
           (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string)));

    const z_loaned_bytes_t* attachment = z_sample_attachment(sample);
    // checks if attachment exists
    if (attachment != NULL) {
        // reads full attachment
        z_bytes_iterator_t iter = z_bytes_get_iterator(attachment);
        z_owned_bytes_t kv;
        while (z_bytes_iterator_next(&iter, &kv)) {
            z_owned_bytes_t k, v;
            z_owned_string_t key, value;
            z_bytes_deserialize_into_pair(z_loan(kv), &k, &v);

            z_bytes_deserialize_into_string(z_loan(k), &key);
            z_bytes_deserialize_into_string(z_loan(v), &value);

            printf("   attachment: %.*s: '%.*s'\n", (int)z_string_len(z_loan(key)), z_string_data(z_loan(key)),
                   (int)z_string_len(z_loan(value)), z_string_data(z_loan(value)));
            z_drop(z_move(kv));
            z_drop(z_move(k));
            z_drop(z_move(v));
            z_drop(z_move(key));
            z_drop(z_move(value));
        }
    }
    z_drop(z_move(payload_string));
}

int main(int argc, char** argv) {
    z_owned_config_t config;
    z_config_default(&config);
    struct args_t args = parse_args(argc, argv, &config);
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, args.keyexpr);

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config))) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, NULL);
    printf("Declaring Subscriber on '%s'...\n", args.keyexpr);
    z_owned_subscriber_t sub;
    if (z_declare_subscriber(&sub, z_loan(s), z_loan(ke), z_move(callback), NULL)) {
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
    const char* keyexpr = parse_opt(argc, argv, "k", true);
    if (!keyexpr) {
        keyexpr = DEFAULT_KEYEXPR;
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
    return (struct args_t){.keyexpr = (char*)keyexpr};
}
