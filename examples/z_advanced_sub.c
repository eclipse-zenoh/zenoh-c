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

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_KEYEXPR "demo/example/**"

struct args_t {
    char* keyexpr;  // -k, --key
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

const char* kind_to_str(z_sample_kind_t kind);

void data_handler(z_loaned_sample_t* sample, void* arg) {
    z_view_string_t key_string;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_string);
    z_owned_string_t payload_string;
    z_bytes_to_string(z_sample_payload(sample), &payload_string);

    printf(">> [Subscriber] Received %s ('%.*s': '%.*s')\n", kind_to_str(z_sample_kind(sample)),
           (int)z_string_len(z_loan(key_string)), z_string_data(z_loan(key_string)),
           (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string)));
    z_drop(z_move(payload_string));
}

void miss_handler(const ze_miss_t* miss, void* arg) {
    z_id_t id = z_entity_global_id_zid(&miss->source);
    z_owned_string_t id_string;
    z_id_to_string(&id, &id_string);
    printf(">> [Subscriber] Missed %d samples from '%.*s' !!!", miss->nb, (int)z_string_len(z_loan(id_string)),
           z_string_data(z_loan(id_string)));
    z_drop(z_move(id_string));
}

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, args.keyexpr);

    ze_advanced_subscriber_options_t sub_opts;
    ze_advanced_subscriber_options_default(&sub_opts);
    ze_advanced_subscriber_history_options_default(&sub_opts.history);  // or sub_opts.history.is_enabled = true;
    sub_opts.history.detect_late_publishers = true;
    ze_advanced_subscriber_recovery_options_default(&sub_opts.recovery);  // or sub_opts.recovery.is_enabled = true;
    sub_opts.recovery.periodic_queries_period_ms = 1000;
    sub_opts.subscriber_detection = true;

    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, NULL);
    printf("Declaring AdvancedSubscriber on '%s'...\n", args.keyexpr);
    ze_owned_advanced_subscriber_t sub;
    if (ze_declare_advanced_subscriber(z_loan(s), &sub, z_loan(ke), z_move(callback), &sub_opts) < 0) {
        printf("Unable to declare advanced subscriber.\n");
        exit(-1);
    }
    ze_owned_closure_miss_t miss_callback;
    z_closure(&miss_callback, miss_handler, NULL, NULL);
    ze_advanced_subscriber_declare_background_sample_miss_listener(z_loan(sub), z_move(miss_callback));

    printf("Press CTRL-C to quit...\n");
    while (1) {
        z_sleep_s(1);
    }

    z_drop(z_move(sub));
    z_drop(z_move(s));

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
    Usage: z_advanced_sub [OPTIONS]\n\n\
    Options:\n\
        -k, --key <KEYEXPR> (optional, string, default='%s'): The key expression to subscribe to\n",
        DEFAULT_KEYEXPR);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", "key", (char*), (char*)DEFAULT_KEYEXPR);

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
