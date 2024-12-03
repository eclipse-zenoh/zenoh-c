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

#define DEFAULT_KEYEXPR "demo/example/zenoh-c-pub"
#define DEFAULT_VALUE "Pub from C!"
#define DEFAULT_ATTACHMENT NULL

struct args_t {
    char* keyexpr;               // -k
    char* value;                 // -p
    char* attachment;            // -a
    bool add_matching_listener;  // --add-matching-listener
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

#if defined(Z_FEATURE_UNSTABLE_API)
void matching_status_handler(const zc_matching_status_t* matching_status, void* arg) {
    if (matching_status->matching) {
        printf("Publisher has matching subscribers.\n");
    } else {
        printf("Publisher has NO MORE matching subscribers.\n");
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
    if (z_declare_publisher(z_loan(s), &pub, z_loan(ke), NULL) < 0) {
        printf("Unable to declare Publisher for key expression!\n");
        exit(-1);
    }

#if defined(Z_FEATURE_UNSTABLE_API)
    if (args.add_matching_listener) {
        zc_owned_closure_matching_status_t callback;
        z_closure(&callback, matching_status_handler, NULL, NULL);
        zc_publisher_declare_background_matching_listener(z_loan(pub), z_move(callback));
    }
#endif

    printf("Press CTRL-C to quit...\n");
    char buf[256] = {};
    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);
        sprintf(buf, "[%4d] %s", idx, args.value);
        printf("Putting Data ('%s': '%s')...\n", args.keyexpr, buf);
        z_publisher_put_options_t options;
        z_publisher_put_options_default(&options);

        z_owned_bytes_t payload;
        z_bytes_copy_from_str(&payload, buf);
        if (args.attachment != NULL) {
            z_owned_bytes_t attachment;
            z_bytes_copy_from_str(&attachment, args.attachment);
            options.attachment = z_move(attachment);
        }
        /// optional encoding
        z_owned_encoding_t encoding;
        z_encoding_clone(&encoding, z_encoding_text_plain());
        options.encoding = z_move(encoding);

        z_publisher_put(z_loan(pub), z_move(payload), &options);
    }

    z_drop(z_move(pub));
    z_drop(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_pub [OPTIONS]\n\n\
    Options:\n\
        -k <KEYEXPR> (optional, string, default='%s'): The key expression to write to\n\
        -p <PAYLOAD> (optional, string, default='%s'): The value to write\n\
        -a <ATTACHMENT> (optional, string, default=NULL): The attachment to add to each put\n"
#if defined(Z_FEATURE_UNSTABLE_API)
        "       --add-matching-listener (optional): Add matching listener\n"
#endif
        ,
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
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", (char*), (char*)DEFAULT_KEYEXPR);
    _Z_PARSE_ARG(args.value, "p", (char*), (char*)DEFAULT_VALUE);
    _Z_PARSE_ARG(args.attachment, "a", (char*), (char*)DEFAULT_ATTACHMENT);
    _Z_CHECK_FLAG(args.add_matching_listener, "add-matching-listener");
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
