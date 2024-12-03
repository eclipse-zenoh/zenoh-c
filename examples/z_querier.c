//
// Copyright (c) 2024 ZettaScale Technology
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

#define DEFAULT_SELECTOR "demo/example/**"
#define DEFAULT_VALUE NULL
#define DEFAULT_TIMEOUT_MS 10000

struct args_t {
    char* selector;              // -s
    char* value;                 // -v
    bool add_matching_listener;  // --add-matching-listener
    z_query_target_t target;     // -t
    uint64_t timeout_ms;         // -o
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

#if defined(Z_FEATURE_UNSTABLE_API)
void matching_status_handler(const zc_matching_status_t* matching_status, void* arg) {
    if (matching_status->matching) {
        printf("Querier has matching queryables.\n");
    } else {
        printf("Querier has NO MORE matching queryables.\n");
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

    const char* ke = args.selector;
    size_t ke_len = strlen(ke);
    const char* params = strchr(args.selector, '?');
    if (params != NULL) {
        ke_len = params - ke;
        params += 1;
    }

    z_view_keyexpr_t keyexpr;
    if (z_view_keyexpr_from_str(&keyexpr, ke) < 0) {
        printf("%s is not a valid key expression", ke);
        exit(-1);
    }

    printf("Declaring Querier on '%s'...\n", ke);
    z_owned_querier_t querier;

    z_querier_options_t opts;
    z_querier_options_default(&opts);
    opts.timeout_ms = args.timeout_ms;
    opts.target = args.target;

    if (z_declare_querier(z_loan(s), &querier, z_loan(keyexpr), &opts) < 0) {
        printf("Unable to declare Querier for key expression!\n");
        exit(-1);
    }

#if defined(Z_FEATURE_UNSTABLE_API)
    if (args.add_matching_listener) {
        zc_owned_closure_matching_status_t callback;
        z_closure(&callback, matching_status_handler, NULL, NULL);
        zc_querier_declare_background_matching_listener(z_loan(querier), z_move(callback));
    }
#endif

    printf("Press CTRL-C to quit...\n");
    char buf[256] = {};
    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);
        sprintf(buf, "[%4d] %s", idx, args.value ? args.value : "");
        printf("Querying '%s' with payload '%s'...\n", args.selector, buf);
        z_querier_get_options_t get_options;
        z_querier_get_options_default(&get_options);

        if (args.value) {
            z_owned_bytes_t payload;
            z_bytes_copy_from_str(&payload, buf);
            get_options.payload = z_move(payload);
        }

        z_owned_fifo_handler_reply_t handler;
        z_owned_closure_reply_t closure;
        z_fifo_channel_reply_new(&closure, &handler, 16);

        z_querier_get(z_loan(querier), params, z_move(closure), &get_options);

        z_owned_reply_t reply;
        for (z_result_t res = z_recv(z_loan(handler), &reply); res == Z_OK; res = z_recv(z_loan(handler), &reply)) {
            if (z_reply_is_ok(z_loan(reply))) {
                const z_loaned_sample_t* sample = z_reply_ok(z_loan(reply));

                z_view_string_t key_str;
                z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_str);

                z_owned_string_t reply_str;
                z_bytes_to_string(z_sample_payload(sample), &reply_str);

                printf(">> Received ('%.*s': '%.*s')\n", (int)z_string_len(z_loan(key_str)),
                       z_string_data(z_loan(key_str)), (int)z_string_len(z_loan(reply_str)),
                       z_string_data(z_loan(reply_str)));
                z_drop(z_move(reply_str));
            } else {
                printf("Received an error\n");
            }
            z_drop(z_move(reply));
        }
        z_drop(z_move(handler));
    }

    z_drop(z_move(querier));
    z_drop(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_querier [OPTIONS]\n\n\
    Options:\n\
        -s <SELECTOR> (optional, string, default='%s'): The selection of resources to query\n\
        -p <PAYLOAD> (optional, string): An optional value to put in the query\n\
        -t <TARGET> (optional, BEST_MATCHING | ALL | ALL_COMPLETE): Query target\n\
        -o <TIMEOUT_MS> (optional, number, default = '%d'): Query timeout in milliseconds\n"
#if defined(Z_FEATURE_UNSTABLE_API)
        "--add-matching-listener (optional): Add matching listener\n"
#endif
        ,
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
    const char* selector = parse_opt(argc, argv, "s", true);
    if (!selector) {
        selector = DEFAULT_SELECTOR;
    }
    const char* value = parse_opt(argc, argv, "p", true);
    if (!value) {
        value = DEFAULT_VALUE;
    }
    const char* timeout_arg = parse_opt(argc, argv, "o", true);
    uint64_t timeout_ms = DEFAULT_TIMEOUT_MS;
    if (timeout_arg) {
        timeout_ms = atoi(timeout_arg);
    }
    const char* target_arg = parse_opt(argc, argv, "t", true);
    z_query_target_t target = z_query_target_default();
    if (target_arg) {
        target = parse_query_target(target_arg);
    }
    const char* matching_listener_arg = parse_opt(argc, argv, "add-matching-listener", false);
    bool add_matching_listener = false;
    if (matching_listener_arg) {
        add_matching_listener = true;
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
    return (struct args_t){.selector = (char*)selector,
                           .value = (char*)value,
                           .timeout_ms = timeout_ms,
                           .target = target,
                           .add_matching_listener = add_matching_listener};
}
