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

#define DEFAULT_KEYEXPR "demo/example/zenoh-c-pub"
#define DEFAULT_VALUE "Pub from C!"
#define DEFAULT_HISTORY 1

struct args_t {
    char* keyexpr;   // -k, --key
    char* value;     // -p, --payload
    size_t history;  // -i, --history
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

    if (zc_config_insert_json5(z_loan_mut(config), Z_CONFIG_ADD_TIMESTAMP_KEY, "true") < 0) {
        printf("Unable to configure timestamps!\n");
        exit(-1);
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring AdvancedPublisher on '%s'...\n", args.keyexpr);
    ze_owned_advanced_publisher_t pub;
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, args.keyexpr);

    ze_advanced_publisher_options_t pub_opts;
    ze_advanced_publisher_options_default(&pub_opts);
    ze_advanced_publisher_cache_options_default(&pub_opts.cache);  // or pub_opts.cache.is_enabled = true;
    pub_opts.cache.max_samples = args.history;
    pub_opts.publisher_detection = true;
    ze_advanced_publisher_sample_miss_detection_options_default(&pub_opts.sample_miss_detection);
    // or pub_opts.sample_miss_detection = true
    pub_opts.sample_miss_detection.heartbeat_period_ms = 500;
    pub_opts.sample_miss_detection.heartbeat_mode = ZE_ADVANCED_PUBLISHER_HEARTBEAT_MODE_PERIODIC;
    // if not set, publisher will retransmit samples based on periodic queries from advanced subscriber

    if (ze_declare_advanced_publisher(z_loan(s), &pub, z_loan(ke), &pub_opts) < 0) {
        printf("Unable to declare AdvancedPublisher for key expression!\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    char buf[256] = {0};
    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);
        sprintf(buf, "[%4d] %s", idx, args.value);
        printf("Put Data ('%s': '%s')...\n", args.keyexpr, buf);
        ze_advanced_publisher_put_options_t options;
        ze_advanced_publisher_put_options_default(&options);

        z_owned_bytes_t payload;
        z_bytes_copy_from_str(&payload, buf);
        ze_advanced_publisher_put(z_loan(pub), z_move(payload), &options);
    }

    z_drop(z_move(pub));
    z_drop(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_advanced_pub [OPTIONS]\n\n\
    Options:\n\
        -k, --key <KEYEXPR> (optional, string, default='%s'): The key expression to write to\n\
        -p, --payload <PAYLOAD> (optional, string, default='%s'): The value to write\n\
        -i, --history <HISTORY_SIZE> (optional, string, default=%d): The number of publications to keep in cache\n",
        DEFAULT_KEYEXPR, DEFAULT_VALUE, DEFAULT_HISTORY);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", "key", (char*), (char*)DEFAULT_KEYEXPR);
    _Z_PARSE_ARG(args.value, "p", "payload", (char*), (char*)DEFAULT_VALUE);
    _Z_PARSE_ARG(args.history, "i", "hisotry", atoi, DEFAULT_HISTORY);

    parse_zenoh_common_args(argc, argv, config);
    const char* unknown_arg = check_unknown_opts(argc, argv);
    if (unknown_arg) {
        printf("Unknown option %s\n", unknown_arg);
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
