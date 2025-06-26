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

#define DEFAULT_KEYEXPR "demo/example/zenoh-c-pub-shm"
#define DEFAULT_VALUE "Pub from C!"

struct args_t {
    char* keyexpr;               // -k, --key
    char* value;                 // -p, --payload
    bool add_matching_listener;  // --add-matching-listener
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

#if defined(Z_FEATURE_UNSTABLE_API)
void matching_status_handler(const z_matching_status_t* matching_status, void* arg) {
    if (matching_status->matching) {
        printf("Publisher has matching subscribers.\n");
    } else {
        printf("Publisher has NO MORE matching subscribers.\n");
    }
}
#endif

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

    printf("Declaring Publisher on '%s'...\n", args.keyexpr);
    z_owned_publisher_t pub;
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, args.keyexpr);
    if (z_declare_publisher(z_loan(s), &pub, z_loan(ke), NULL) < 0) {
        printf("Unable to declare Publisher for key expression!\n");
        exit(-1);
    }
#if defined(Z_FEATURE_UNSTABLE_API)
    if (args.add_matching_listener) {
        z_owned_closure_matching_status_t callback;
        z_closure(&callback, matching_status_handler, NULL, NULL);
        if (z_publisher_declare_background_matching_listener(z_loan(pub), z_move(callback)) < 0) {
            printf("Unable to declare background matching listener for key expression!\n");
            exit(-1);
        }
    }
#endif

    printf("Creating POSIX SHM Provider...\n");
    const size_t total_size = 4096;
    const size_t buf_ok_size = total_size / 4;

    z_owned_shm_provider_t provider;
    z_shm_provider_default_new(&provider, total_size);

    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);

        z_buf_layout_alloc_result_t alloc;
        z_shm_provider_alloc_gc_defrag_blocking(&alloc, z_loan(provider), buf_ok_size);
        if (alloc.status == ZC_BUF_LAYOUT_ALLOC_STATUS_OK) {
            {
                uint8_t* buf = z_shm_mut_data_mut(z_loan_mut(alloc.buf));
                sprintf((char*)buf, "[%4d] %s", idx, args.value);
                printf("Putting Data ('%s': '%s')...\n", args.keyexpr, buf);
            }

            z_publisher_put_options_t options;
            z_publisher_put_options_default(&options);

            z_owned_bytes_t payload;
            z_bytes_from_shm_mut(&payload, z_move(alloc.buf));

            z_publisher_put(z_loan(pub), z_move(payload), &options);
        } else {
            printf("Unexpected failure during SHM buffer allocation...");
            break;
        }
    }

    z_drop(z_move(pub));
    z_drop(z_move(s));
    z_drop(z_move(provider));

    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_pub_shm [OPTIONS]\n\n\
    Options:\n\
        -k, --key <KEYEXPR> (optional, string, default='%s'): The key expression to write to\n\
        -p, --payload <PAYLOAD> (optional, string, default='%s'): The value to write\n"
#if defined(Z_FEATURE_UNSTABLE_API)
        "       --add-matching-listener (optional): Add matching listener\n"
#endif
        ,
        DEFAULT_KEYEXPR, DEFAULT_VALUE);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", "key", (char*), (char*)DEFAULT_KEYEXPR);
    _Z_PARSE_ARG(args.value, "p", "payload", (char*), (char*)DEFAULT_VALUE);
#if defined(Z_FEATURE_UNSTABLE_API)
    args.add_matching_listener = _Z_CHECK_FLAG("add-matching-listener");
#endif
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
    return args;
}
