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

#define DEFAULT_SHARED_MEMORY_SIZE 32

struct args_t {
    unsigned int size;                         // positional_1
    unsigned long long shared_memory_size_mb;  // -s, --shared-memory
};

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    char* keyexpr = "test/thr";

    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

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
    if (z_declare_publisher(z_loan(s), &pub, z_loan(ke), &options)) {
        printf("Unable to declare publisher for key expression!\n");
        exit(-1);
    }

    printf("Creating POSIX SHM Provider...\n");
    z_alloc_alignment_t alignment = {0};
    z_owned_memory_layout_t layout;
    z_memory_layout_new(&layout, args.shared_memory_size_mb * 1024 * 1024, alignment);
    z_owned_shm_provider_t provider;
    z_posix_shm_provider_new(&provider, z_loan(layout));

    printf("Allocating single SHM buffer\n");
    z_buf_layout_alloc_result_t alloc;
    z_shm_provider_alloc(&alloc, z_loan(provider), args.size, alignment);
    if (alloc.status != ZC_BUF_LAYOUT_ALLOC_STATUS_OK) {
        printf("Unexpected failure during SHM buffer allocation...\n");
        return -1;
    }
    memset(z_shm_mut_data_mut(z_loan_mut(alloc.buf)), 1, args.size);
    z_owned_shm_t shm;
    z_shm_from_mut(&shm, z_move(alloc.buf));

    z_owned_bytes_t shmbs;
    if (z_bytes_from_shm(&shmbs, z_move(shm)) != Z_OK) {
        printf("Unexpected failure during SHM buffer serialization...\n");
        return -1;
    }

    while (1) {
        z_owned_bytes_t payload;
        z_bytes_clone(&payload, z_loan(shmbs));
        z_publisher_put(z_loan(pub), z_move(payload), NULL);
    }

    z_drop(z_move(pub));
    z_drop(z_move(s));

    z_drop(z_move(shm));
    z_drop(z_move(provider));
    z_drop(z_move(layout));
}

void print_help() {
    printf(
        "\
    Usage: z_pub_thr [OPTIONS] <PAYLOAD_SIZE>\n\n\
    Arguments:\n\
        <PAYLOAD_SIZE> (required, number): Size of the payload to publish\n\n\
    Options:\n\
        -s, --shared-memory <SHARED_MEMORY_SIZE> (optional, number, default='%d'): shared memory size in MBytes.\n",
        DEFAULT_SHARED_MEMORY_SIZE);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.shared_memory_size_mb, "s", "shared-memory", atoi, DEFAULT_SHARED_MEMORY_SIZE);

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
    args.size = atoi(pos_args[0]);
    free(pos_args);
    return args;
}
