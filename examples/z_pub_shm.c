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
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "parse_args.h"
#include "zenoh.h"

#define N 10
#define DEFAULT_KEYEXPR "demo/example/zenoh-c-pub-shm"
#define DEFAULT_VALUE "Pub from C!"

struct args_t {
    char* keyexpr;  // -k
    char* value;    // -v
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    z_owned_config_t config = z_config_default();
    struct args_t args = parse_args(argc, argv, &config);

    // Enable shared memory
    if (zc_config_insert_json(z_loan(config), "transport/shared_memory/enabled", "true") < 0) {
        printf("Error enabling Shared Memory");
        exit(-1);
    }

    printf("Opening session...\n");
    z_owned_session_t s = z_open(z_move(config));
    z_id_t id = z_info_zid(z_loan(s));
    char idstr[33];
    for (int i = 0; i < 16; i++) {
        sprintf(idstr + 2 * i, "%02x", id.id[i]);
    }
    idstr[32] = 0;
    zc_owned_shm_manager_t manager = zc_shm_manager_new(z_loan(s), idstr, N * 1000000);
    if (!z_check(s)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring Publisher on '%s'...\n", args.keyexpr);
    z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(args.keyexpr), NULL);
    if (!z_check(pub)) {
        printf("Unable to declare Publisher for key expression!\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    for (int idx = 0; true; ++idx) {
        zc_owned_shmbuf_t shmbuf = zc_shm_alloc(&manager, 256);
        if (!z_check(shmbuf)) {
            zc_shm_gc(&manager);
            shmbuf = zc_shm_alloc(&manager, 256);
            if (!z_check(shmbuf)) {
                printf("Failed to allocate a SHM buffer, even after GCing\n");
                exit(-1);
            }
        }
        char* buf = (char*)zc_shmbuf_ptr(&shmbuf);
        buf[256] = 0;
        snprintf(buf, 255, "[%4d] %s", idx, args.value);
        size_t len = strlen(buf);
        zc_shmbuf_set_length(&shmbuf, len);
        z_sleep_s(1);
        printf("Putting Data ('%s': '%s')...\n", args.keyexpr, buf);
        z_publisher_put_options_t options = z_publisher_put_options_default();
        options.encoding = z_encoding(Z_ENCODING_PREFIX_TEXT_PLAIN, NULL);
        zc_owned_payload_t payload = zc_shmbuf_into_payload(z_move(shmbuf));
        zc_publisher_put_owned(z_loan(pub), z_move(payload), &options);
    }

    z_undeclare_publisher(z_move(pub));

    z_close(z_move(s));
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
