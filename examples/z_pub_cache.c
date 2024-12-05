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
#define DEFAULT_HISTORY 1

struct args_t {
    char* keyexpr;         // -k, --key
    char* value;           // -v, --value
    unsigned int history;  // -i, --history
    bool complete;         // -o, --complete
    char* prefix;          // -x, --prefix
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

    ze_publication_cache_options_t pub_cache_opts;
    ze_publication_cache_options_default(&pub_cache_opts);
    pub_cache_opts.history = args.history;
    pub_cache_opts.queryable_complete = args.complete;
    z_view_keyexpr_t prefix;
    if (args.prefix != NULL) {
        z_view_keyexpr_from_str(&prefix, args.prefix);
        pub_cache_opts.queryable_prefix = z_loan(prefix);
    }

    printf("Declaring publication cache on '%s'...\n", args.keyexpr);
    ze_owned_publication_cache_t pub_cache;
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, args.keyexpr);

    if (ze_declare_publication_cache(z_loan(s), &pub_cache, z_loan(ke), &pub_cache_opts) != Z_OK) {
        printf("Unable to declare publication cache for key expression!\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    char buf[256];
    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);
        sprintf(buf, "[%4d] %s", idx, args.value);
        printf("Putting Data ('%s': '%s')...\n", args.keyexpr, buf);
        z_owned_bytes_t payload;
        z_bytes_copy_from_str(&payload, buf);

        z_put(z_loan(s), z_loan(ke), z_move(payload), NULL);
    }

    z_drop(z_move(pub_cache));
    z_drop(z_move(s));

    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_pub_cache [OPTIONS]\n\n\
    Options:\n\
        -k, --key <KEYEXPR> (optional, string, default='%s'): The key expression to write to\n\
        -v, --value <VALUE> (optional, string, default='%s'): The value to write\n\
        -i, --history <HISTORY> (optional, int, default='%d'): The number of publications to keep in cache\n\
        -o, --complete (optional): Set `complete` option to true. This means that this queryable is ultimate data source, no need to scan other queryables.\n\
        -x, --prefix <PREFIX> (optional, string, default=NULL):  An optional queryable prefix\n.",
        DEFAULT_KEYEXPR, DEFAULT_VALUE, DEFAULT_HISTORY);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", "key", (char*), (char*)DEFAULT_KEYEXPR);
    _Z_PARSE_ARG(args.value, "v", "value", (char*), (char*)DEFAULT_VALUE);
    _Z_PARSE_ARG(args.history, "i", "history", atoi, DEFAULT_HISTORY);
    args.complete = _Z_CHECK_FLAG("o") || _Z_CHECK_FLAG("complete");
    _Z_PARSE_ARG(args.prefix, "x", "prefix", (char*), NULL);

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
