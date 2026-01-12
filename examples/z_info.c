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

#include "parse_args.h"
#include "zenoh.h"

void print_zid(const z_id_t* id, void* ctx) {
    z_owned_string_t str;
    z_id_to_string(id, &str);
    printf("%.*s\n", (int)z_string_len(z_loan(str)), z_string_data(z_loan(str)));
    z_drop(z_move(str));
}

#if defined(Z_FEATURE_UNSTABLE_API)
void print_transport(z_loaned_transport_t* transport, void* ctx) {
    z_id_t zid = z_transport_zid(transport);
    printf(" transport to zid: ");
    print_zid(&zid, NULL);
}

void print_link(z_loaned_link_t* link, void* ctx) {
    printf(" link\n");
}
#endif

void parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    parse_args(argc, argv, &config);

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_id_t self_id = z_info_zid(z_loan(s));
    printf("own id: ");
    print_zid(&self_id, NULL);

    printf("routers ids:\n");
    z_owned_closure_zid_t callback;
    z_closure(&callback, print_zid, NULL, NULL);
    z_info_routers_zid(z_loan(s), z_move(callback));

    // `callback` has been `z_move`d just above, so it's safe to reuse the variable,
    // we'll just have to make sure we `z_move` it again to avoid mem-leaks.
    printf("peers ids:\n");
    z_owned_closure_zid_t callback2;
    z_closure(&callback2, print_zid, NULL, NULL);
    z_info_peers_zid(z_loan(s), z_move(callback2));

    #if defined(Z_FEATURE_UNSTABLE_API)
    // Get transports
    printf("transports:\n");
    z_owned_closure_transport_t callback3;
    z_closure(&callback3, print_transport, NULL, NULL);
    z_info_transports(z_loan(s), z_move(callback3));

    // Get links
    printf("links:\n");
    z_owned_closure_link_t callback4;
    z_closure(&callback4, print_link, NULL, NULL);
    z_info_links(z_loan(s), z_move(callback4));
    #endif

    z_drop(z_move(s));
}

void print_help() {
    printf(
        "\
    Usage: z_info [OPTIONS]\n\n\
    Options:\n");
    printf(COMMON_HELP);
}

void parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
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
}
