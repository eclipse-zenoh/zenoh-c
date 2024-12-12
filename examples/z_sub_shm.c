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
    char *keyexpr;  // -k, --key
};
struct args_t parse_args(int argc, char **argv, z_owned_config_t *config);

const char *kind_to_str(z_sample_kind_t kind);

void data_handler(z_loaned_sample_t *sample, void *arg) {
    z_view_string_t key_string;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_string);

// if Zenoh is built without SHM support, the only buffer type it can receive is RAW
#if !defined(Z_FEATURE_SHARED_MEMORY)
    char *payload_type = "RAW";
#endif

// if Zenoh is built with SHM support but without SHM API (that is unstable), it can
// receive buffers of any type, but there is no way to detect the buffer type
#if defined(Z_FEATURE_SHARED_MEMORY) && !defined(Z_FEATURE_UNSTABLE_API)
    char *payload_type = "UNKNOWN";
#endif

// if Zenoh is built with SHM support and with SHM API, we can detect the exact buffer type
#if defined(Z_FEATURE_SHARED_MEMORY) && defined(Z_FEATURE_UNSTABLE_API)
    char *payload_type = "RAW";
    {
        const z_loaned_shm_t *shm = NULL;
        if (z_bytes_as_loaned_shm(z_sample_payload(sample), &shm) == Z_OK) {
            payload_type = "SHM";
        }
    }
#endif

    z_owned_string_t payload_string;
    z_bytes_to_string(z_sample_payload(sample), &payload_string);

    printf(">> [Subscriber] Received %s ('%.*s': '%.*s') [%s]", kind_to_str(z_sample_kind(sample)),
           (int)z_string_len(z_loan(key_string)), z_string_data(z_loan(key_string)),
           (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string)), payload_type);

    const z_loaned_bytes_t *attachment = z_sample_attachment(sample);
    // checks if attachment exists
    if (attachment != NULL) {
        z_owned_string_t attachment_string;
        z_bytes_to_string(attachment, &attachment_string);
        printf(" (%.*s)", (int)z_string_len(z_loan(attachment_string)), z_string_data(z_loan(attachment_string)));
        z_drop(z_move(attachment_string));
    }
    printf("\n");

    z_drop(z_move(payload_string));
}

int main(int argc, char **argv) {
    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

    zc_init_log_from_env_or("error");

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, args.keyexpr);

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, NULL);
    printf("Declaring Subscriber on '%s'...\n", args.keyexpr);
    z_owned_subscriber_t sub;
    if (z_declare_subscriber(z_loan(s), &sub, z_loan(ke), z_move(callback), NULL) < 0) {
        printf("Unable to declare subscriber.\n");
        exit(-1);
    }

    printf("Enter 'q' to quit...\n");
    char c = 0;
    while (c != 'q') {
        c = getchar();
        if (c == -1) {
            z_sleep_s(1);
        }
    }

    z_drop(z_move(sub));
    z_drop(z_move(s));
    return 0;
}

const char *kind_to_str(z_sample_kind_t kind) {
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
    Usage: z_sub_shm [OPTIONS]\n\n\
    Options:\n\
        -k, --key <KEYEXPR> (optional, string, default='%s'): The key expression to subscribe to\n",
        DEFAULT_KEYEXPR);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char **argv, z_owned_config_t *config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", "key", (char *), (char *)DEFAULT_KEYEXPR);

    parse_zenoh_common_args(argc, argv, config);
    const char *arg = check_unknown_opts(argc, argv);
    if (arg) {
        printf("Unknown option %s\n", arg);
        exit(-1);
    }
    char **pos_args = parse_pos_args(argc, argv, 1);
    if (!pos_args || pos_args[0]) {
        printf("Unexpected positional arguments\n");
        free(pos_args);
        exit(-1);
    }
    free(pos_args);
    return args;
}
