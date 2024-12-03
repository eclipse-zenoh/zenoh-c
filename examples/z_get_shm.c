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

#include <stdio.h>
#include <string.h>

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_SELECTOR "demo/example/**"
#define DEFAULT_VALUE NULL
#define DEFAULT_TIMEOUT_MS 10000

struct args_t {
    char* selector;           // -s
    char* value;              // -p
    z_query_target_t target;  // -t
    uint64_t timeout_ms;      // -o
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");
    z_owned_config_t config;

    struct args_t args = parse_args(argc, argv, &config);
    if (!args.value) {
        args.value = "Get from Rust SHM!";
    }

    const char* ke = args.selector;
    size_t ke_len = strlen(ke);
    const char* params = strchr(args.selector, '?');
    if (params != NULL) {
        ke_len = params - ke;
        params += 1;
    }

    size_t value_len = args.value ? strlen(args.value) : 0;

    z_view_keyexpr_t keyexpr;
    if (z_view_keyexpr_from_substr(&keyexpr, ke, ke_len) < 0) {
        printf("%.*s is not a valid key expression", (int)ke_len, ke);
        exit(-1);
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    // Create SHM Provider
    z_alloc_alignment_t alignment = {0};
    z_owned_memory_layout_t layout;
    z_memory_layout_new(&layout, value_len, alignment);
    z_owned_shm_provider_t provider;
    z_posix_shm_provider_new(&provider, z_loan(layout));

    // Allocate SHM Buffer
    z_buf_layout_alloc_result_t alloc;
    z_shm_provider_alloc(&alloc, z_loan(provider), value_len, alignment);
    if (alloc.status != ZC_BUF_LAYOUT_ALLOC_STATUS_OK) {
        printf("Unexpected failure during SHM buffer allocation...");
        return -1;
    }
    // Fill SHM Buffer with data
    uint8_t* data = z_shm_mut_data_mut(z_loan_mut(alloc.buf));
    memcpy(data, args.value, value_len);
    // Convert mutable SHM Buffer into immutable one (to be able to make it's ref copies)
    z_owned_shm_t shm;
    z_shm_from_mut(&shm, z_move(alloc.buf));

    printf("Sending Query '%s'...\n", args.selector);
    z_owned_fifo_handler_reply_t handler;
    z_owned_closure_reply_t closure;
    z_fifo_channel_reply_new(&closure, &handler, 16);

    z_get_options_t opts;
    z_get_options_default(&opts);
    opts.target = args.target;
    opts.timeout_ms = args.timeout_ms;

    z_owned_bytes_t payload;
    if (args.value != NULL) {
        if (z_bytes_from_shm(&payload, z_move(shm)) != Z_OK) {
            printf("Unexpected failure during SHM buffer serialization...\n");
            return -1;
        }
        opts.payload = z_move(payload);
    }
    z_get(z_loan(s), z_loan(keyexpr), params, z_move(closure),
          &opts);  // here, the send is moved and will be dropped by zenoh when adequate
    z_owned_reply_t reply;

    for (z_result_t res = z_recv(z_loan(handler), &reply); res == Z_OK; res = z_recv(z_loan(handler), &reply)) {
        if (z_reply_is_ok(z_loan(reply))) {
            const z_loaned_sample_t* sample = z_reply_ok(z_loan(reply));

            z_view_string_t key_str;
            z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_str);

            z_owned_string_t reply_str;
            z_bytes_to_string(z_sample_payload(sample), &reply_str);

            printf(">> Received ('%.*s': '%.*s')\n", (int)z_string_len(z_loan(key_str)), z_string_data(z_loan(key_str)),
                   (int)z_string_len(z_loan(reply_str)), z_string_data(z_loan(reply_str)));
            z_drop(z_move(reply_str));
        } else {
            printf("Received an error\n");
        }
        z_drop(z_move(reply));
    }

    z_drop(z_move(handler));
    z_drop(z_move(s));

    z_drop(z_move(shm));
    z_drop(z_move(provider));
    z_drop(z_move(layout));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_get [OPTIONS]\n\n\
    Options:\n\
        -s <SELECTOR> (optional, string, default='%s'): The selection of resources to query\n\
        -p <PAYLOAD> (optional, string): An optional value to put in the query\n\
        -t <TARGET> (optional, BEST_MATCHING | ALL | ALL_COMPLETE): Query target\n\
        -o <TIMEOUT_MS> (optional, number, default = '%d'): Query timeout in milliseconds\n",
        DEFAULT_SELECTOR, DEFAULT_TIMEOUT_MS);
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
    struct args_t args;
    _Z_PARSE_ARG(args.selector, "s", (char*), (char*)DEFAULT_SELECTOR);
    _Z_PARSE_ARG(args.value, "p", (char*), (char*)DEFAULT_VALUE);
    _Z_PARSE_ARG(args.timeout_ms, "o", atoi, DEFAULT_TIMEOUT_MS);
    _Z_PARSE_ARG(args.target, "t", parse_query_target, z_query_target_default());

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
