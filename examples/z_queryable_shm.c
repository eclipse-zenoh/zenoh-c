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

#define DEFAULT_KEYEXPR "demo/example/zenoh-c-queryable"
#define DEFAULT_VALUE "Queryable from C SHM!"
z_view_keyexpr_t ke;
const char *value;

typedef struct {
    const char *keyexpr;
    const z_loaned_shm_provider_t *provider;
} context_t;

struct args_t {
    char *keyexpr;  // -k
    char *value;    // -p
    bool complete;  // --complete
};
struct args_t parse_args(int argc, char **argv, z_owned_config_t *config);

void query_handler(z_loaned_query_t *query, void *context) {
    context_t *handler_context = (context_t *)context;

    z_view_string_t key_string;
    z_keyexpr_as_view_string(z_query_keyexpr(query), &key_string);

    z_view_string_t params;
    z_query_parameters(query, &params);

    const z_loaned_bytes_t *payload = z_query_payload(query);
    if (payload != NULL && z_bytes_len(payload) > 0) {
        const z_loaned_shm_t *shm = NULL;
        char *payload_type = z_bytes_as_loaned_shm(payload, &shm) == Z_OK ? "SHM" : "RAW";

        z_owned_string_t payload_string;
        z_bytes_to_string(payload, &payload_string);

        printf(">> [Queryable ] Received Query '%.*s?%.*s' with value '%.*s' [%s]\n",
               (int)z_string_len(z_loan(key_string)), z_string_data(z_loan(key_string)),
               (int)z_string_len(z_loan(params)), z_string_data(z_loan(params)),
               (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string)), payload_type);
        z_drop(z_move(payload_string));
    } else {
        printf(">> [Queryable ] Received Query '%.*s?%.*s'\n", (int)z_string_len(z_loan(key_string)),
               z_string_data(z_loan(key_string)), (int)z_string_len(z_loan(params)), z_string_data(z_loan(params)));
    }

    printf("Allocating Shared Memory Buffer...\n");
    size_t value_len = strlen(value) + 1;  // + NULL terminator
    z_alloc_alignment_t alignment = {0};
    z_buf_layout_alloc_result_t alloc;
    z_shm_provider_alloc_gc_defrag_blocking(&alloc, handler_context->provider, value_len, alignment);
    if (alloc.status == ZC_BUF_LAYOUT_ALLOC_STATUS_OK) {
        {
            uint8_t *buf = z_shm_mut_data_mut(z_loan_mut(alloc.buf));
            memcpy(buf, value, value_len);
            printf(">> [Queryable] Responding ('%s': '%s')...\n", handler_context->keyexpr, buf);
        }

        z_query_reply_options_t options;
        z_query_reply_options_default(&options);

        z_owned_bytes_t reply_payload;
        z_bytes_from_shm_mut(&reply_payload, z_move(alloc.buf));

        z_view_keyexpr_t reply_keyexpr;
        z_view_keyexpr_from_str(&reply_keyexpr, handler_context->keyexpr);

        z_query_reply(query, z_loan(reply_keyexpr), z_move(reply_payload), &options);

    } else {
        printf("Unexpected failure during SHM buffer allocation...");
        exit(-1);
    }
}

int main(int argc, char **argv) {
    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);
    value = args.value;

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    if (z_view_keyexpr_from_str(&ke, args.keyexpr)) {
        printf("%s is not a valid key expression", args.keyexpr);
        exit(-1);
    }

    printf("Creating POSIX SHM Provider...\n");
    const size_t total_size = 4096;
    z_alloc_alignment_t alignment = {0};
    z_owned_memory_layout_t layout;
    z_memory_layout_new(&layout, total_size, alignment);
    z_owned_shm_provider_t provider;
    z_posix_shm_provider_new(&provider, z_loan(layout));

    printf("Declaring Queryable on '%s'...\n", args.keyexpr);
    z_owned_closure_query_t callback;
    context_t context = (context_t){.keyexpr = args.keyexpr, .provider = z_loan(provider)};
    z_closure(&callback, query_handler, NULL, (void *)&context);
    z_owned_queryable_t qable;

    z_queryable_options_t opts;
    z_queryable_options_default(&opts);
    opts.complete = args.complete;

    if (z_declare_queryable(z_loan(s), &qable, z_loan(ke), z_move(callback), &opts) < 0) {
        printf("Unable to create queryable.\n");
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

    z_drop(z_move(qable));
    z_drop(z_move(s));
    z_drop(z_move(layout));
    z_drop(z_move(provider));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_queryable_shm [OPTIONS]\n\n\
    Options:\n\
        -k <KEYEXPR> (optional, string, default='%s'): The key expression matching queries to reply to\n\
        -p <PAYLOAD> (optional, string, default='%s'): The value to reply to queries with\n\
        --complete (optional, flag to indicate whether queryable is complete or not)",
        DEFAULT_KEYEXPR, DEFAULT_VALUE);
    printf(COMMON_HELP);
    printf(
        "\
        -h: print help\n");
}

struct args_t parse_args(int argc, char **argv, z_owned_config_t *config) {
    if (parse_opt(argc, argv, "h", false)) {
        print_help();
        exit(1);
    }
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", (char *), (char *)DEFAULT_KEYEXPR);
    _Z_PARSE_ARG(args.value, "p", (char *), (char *)DEFAULT_VALUE);
    _Z_CHECK_FLAG(args.complete, "complete");

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
