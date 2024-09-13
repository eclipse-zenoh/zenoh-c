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

#define N 10
#define DEFAULT_KEYEXPR "demo/example/zenoh-c-pub-shm"
#define DEFAULT_VALUE "Pub from C!"

struct args_t {
    char* keyexpr;               // -k
    char* value;                 // -v
    bool add_matching_listener;  // --add-matching-listener
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

#ifdef UNSTABLE
void matching_status_handler(const zc_matching_status_t* matching_status, void* arg) {
    if (matching_status->matching) {
        printf("Subscriber matched\n");
    } else {
        printf("No Subscribers matched\n");
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
    if (z_declare_publisher(&pub, z_loan(s), z_loan(ke), NULL) < 0) {
        printf("Unable to declare Publisher for key expression!\n");
        exit(-1);
    }
#ifdef UNSTABLE
    zc_owned_matching_listener_t listener;
    if (args.add_matching_listener) {
        zc_owned_closure_matching_status_t callback;
        z_closure(&callback, matching_status_handler, NULL, NULL);
        zc_publisher_matching_listener_declare(&listener, z_loan(pub), z_move(callback));
    }
#else
    if (add_matching_listener) {
        printf("To enable matching listener you must compile Zenoh-c with unstable feature support!\n");
        exit(-1);
    }
#endif

    printf("Creating POSIX SHM Provider...\n");
    const size_t total_size = 4096;
    const size_t buf_ok_size = total_size / 4;

    z_alloc_alignment_t alignment = {0};
    z_owned_memory_layout_t layout;
    z_memory_layout_new(&layout, total_size, alignment);

    z_owned_shm_provider_t provider;
    z_posix_shm_provider_new(&provider, z_loan(layout));

    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);

        z_buf_layout_alloc_result_t alloc;
        z_shm_provider_alloc_gc_defrag_blocking(&alloc, z_loan(provider), buf_ok_size, alignment);
        if (alloc.status == ZC_BUF_LAYOUT_ALLOC_STATUS_OK) {
            {
                uint8_t* buf = z_shm_mut_data_mut(z_loan_mut(alloc.buf));
                sprintf((char*)buf, "[%4d] %s", idx, args.value);
                printf("Putting Data ('%s': '%s')...\n", args.keyexpr, buf);
            }

            z_publisher_put_options_t options;
            z_publisher_put_options_default(&options);

            z_owned_bytes_t payload;
            z_bytes_serialize_from_shm_mut(&payload, z_move(alloc.buf));

            z_publisher_put(z_loan(pub), z_move(payload), &options);
        } else {
            printf("Unexpected failure during SHM buffer allocation...");
            break;
        }
    }

#ifdef UNSTABLE
    if (args.add_matching_listener) {
        zc_publisher_matching_listener_undeclare(z_move(listener));
    }
#endif

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s), NULL);

    z_drop(z_move(provider));
    z_drop(z_move(layout));

    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_pub_shm [OPTIONS]\n\n\
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
    return (struct args_t){
        .keyexpr = (char*)keyexpr, .value = (char*)value, .add_matching_listener = add_matching_listener};
}
