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

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_KEYEXPR "demo/example/**"

struct args_t {
    char* keyexpr;  // -k
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

const char* kind_to_str(z_sample_kind_t kind);

void handle_sample(const z_loaned_sample_t* sample) {
    z_view_string_t keystr;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &keystr);
    z_owned_string_t payload_value;
    z_bytes_deserialize_into_string(z_sample_payload(sample), &payload_value);
    printf(">> [Subscriber] Received %s ('%.*s': '%.*s')\n", kind_to_str(z_sample_kind(sample)),
           (int)z_string_len(z_loan(keystr)), z_string_data(z_loan(keystr)), (int)z_string_len(z_loan(payload_value)),
           z_string_data(z_loan(payload_value)));
    z_drop(z_move(payload_value));
}

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

    z_owned_closure_sample_t closure;
    z_owned_ring_handler_sample_t handler;
    z_ring_channel_sample_new(&closure, &handler, 3);

    printf("Declaring Subscriber on '%s'...\n", args.keyexpr);
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, args.keyexpr);
    z_owned_subscriber_t sub;

    if (z_declare_subscriber(&sub, z_loan(s), z_loan(ke), z_move(closure), NULL) < 0) {
        printf("Unable to declare subscriber.\n");
        exit(-1);
    }

    printf("Press <enter> to pull data...\n");
    z_owned_sample_t sample;

    char c = 0;
    while (c != 'q') {
        c = getchar();
        if (c == -1) {
            z_sleep_s(1);
        } else {
            z_result_t res = z_try_recv(z_loan(handler), &sample);
            if (res == Z_OK) {
                handle_sample(z_loan(sample));
                z_drop(z_move(sample));
            }
        }
    }
    z_undeclare_subscriber(z_move(sub));
    z_close(z_move(s), NULL);
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
    Usage: z_pull [OPTIONS]\n\n\
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
