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
#include <stdio.h>
#include <string.h>

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_KEYEXPR "demo/example/zenoh-c-pub"
#define DEFAULT_VALUE "Pub from C!"

typedef struct kv_pair_t {
    const char* key;
    const char* value;
} kv_pair_t;

struct args_t {
    char* keyexpr;  // -k
    char* value;    // -v
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

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
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, args.keyexpr);
    z_owned_publisher_t pub;
    if (z_declare_publisher(&pub, z_loan(s), z_loan(ke), NULL)) {
        printf("Unable to declare Publisher for key expression!\n");
        exit(-1);
    }

    z_publisher_put_options_t options;
    z_publisher_put_options_default(&options);

    // allocate attachment data
    kv_pair_t kvs[2];
    kvs[0] = (kv_pair_t){.key = "source", .value = "C"};
    // allocate attachment and payload
    z_owned_bytes_t attachment;
    z_owned_bytes_t payload;

    printf("Press CTRL-C to quit...\n");
    char buf[256];
    char buf_ind[16];
    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);

#if defined(Z_FEATURE_UNSTABLE_API)
        // add some other attachment value
        sprintf(buf_ind, "%d", idx);
        kvs[1] = (kv_pair_t){.key = "index", .value = buf_ind};
        ze_owned_serializer_t serializer;
        ze_serializer_empty(&serializer);
        ze_serializer_serialize_sequence_begin(z_loan_mut(serializer), 2);
        for (size_t i = 0; i < 2; ++i) {
            ze_serializer_serialize_str(z_loan_mut(serializer), kvs[i].key);
            ze_serializer_serialize_str(z_loan_mut(serializer), kvs[i].value);
        }
        ze_serializer_serialize_sequence_end(z_loan_mut(serializer));
        ze_serializer_finish(z_move(serializer), &attachment);
        options.attachment = z_move(attachment);
#endif
        sprintf(buf, "[%4d] %s", idx, args.value);
        printf("Putting Data ('%s': '%s')...\n", args.keyexpr, buf);

        z_bytes_copy_from_str(&payload, buf);
        z_publisher_put(z_loan(pub), z_move(payload), &options);
    }

    z_undeclare_publisher(z_move(pub));

    z_close(z_move(s), NULL);

    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_pub_attachement [OPTIONS]\n\n\
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
