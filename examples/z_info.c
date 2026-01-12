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
    z_whatami_t whatami = z_transport_whatami(transport);
    bool is_qos = z_transport_is_qos(transport);
    bool is_multicast = z_transport_is_multicast(transport);

    printf("  Transport { zid: ");
    print_zid(&zid, NULL);

    printf("  whatami: ");
    z_view_string_t whatami_str;
    z_whatami_to_view_string(whatami, &whatami_str);
    printf("%.*s", (int)z_string_len(z_loan(whatami_str)), z_string_data(z_loan(whatami_str)));

    printf(", is_qos: %s", is_qos ? "true" : "false");
    printf(", is_multicast: %s", is_multicast ? "true" : "false");

    #if defined(Z_FEATURE_SHARED_MEMORY)
    bool is_shm = z_transport_is_shm(transport);
    printf(", is_shm: %s", is_shm ? "true" : "false");
    #endif

    printf(" }\n");
}

void print_link(z_loaned_link_t* link, void* ctx) {
    z_id_t zid = z_link_zid(link);
    z_owned_string_t src, dst, group, auth_id;
    z_link_src(link, &src);
    z_link_dst(link, &dst);
    z_link_group(link, &group);
    z_link_auth_identifier(link, &auth_id);

    uint16_t mtu = z_link_mtu(link);
    bool is_streamed = z_link_is_streamed(link);

    uint8_t min_prio = 0, max_prio = 0;
    bool has_priorities = z_link_priorities(link, &min_prio, &max_prio);
    z_reliability_t reliability;
    bool has_reliability = z_link_reliability(link, &reliability);

    printf("  Link { zid: ");
    print_zid(&zid, NULL);

    printf("  src: %.*s", (int)z_string_len(z_loan(src)), z_string_data(z_loan(src)));
    printf(", dst: %.*s", (int)z_string_len(z_loan(dst)), z_string_data(z_loan(dst)));

    if (z_string_len(z_loan(group)) > 0) {
        printf(", group: %.*s", (int)z_string_len(z_loan(group)), z_string_data(z_loan(group)));
    }

    printf(", mtu: %u", mtu);
    printf(", is_streamed: %s", is_streamed ? "true" : "false");

    z_owned_string_array_t interfaces;
    z_link_interfaces(link, &interfaces);
    size_t interfaces_len = z_string_array_len(z_loan(interfaces));
    if (interfaces_len > 0) {
        printf(", interfaces: [");
        for (size_t i = 0; i < interfaces_len; i++) {
            const z_loaned_string_t* iface = z_string_array_get(z_loan(interfaces), i);
            if (i > 0) printf(", ");
            printf("%.*s", (int)z_string_len(iface), z_string_data(iface));
        }
        printf("]");
    }
    z_drop(z_move(interfaces));

    if (z_string_len(z_loan(auth_id)) > 0) {
        printf(", auth_id: %.*s", (int)z_string_len(z_loan(auth_id)), z_string_data(z_loan(auth_id)));
    }

    if (has_priorities) {
        printf(", priorities: (%u, %u)", min_prio, max_prio);
    }

    if (has_reliability) {
        printf(", reliability: %s", reliability == Z_RELIABILITY_RELIABLE ? "Reliable" : "BestEffort");
    }

    printf(" }\n");

    z_drop(z_move(src));
    z_drop(z_move(dst));
    z_drop(z_move(group));
    z_drop(z_move(auth_id));
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
