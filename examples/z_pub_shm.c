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

#include "zenoh.h"

#ifdef UNSTABLE
void matching_status_handler(const zc_matching_status_t *matching_status, void *arg) {
    if (matching_status->matching) {
        printf("Subscriber matched\n");
    } else {
        printf("No Subscribers matched\n");
    }
}
#endif

int main(int argc, char **argv) {
    char *keyexpr = "demo/example/zenoh-c-pub";
    char *value = "Pub from C SHM!";
    bool add_matching_listener = false;

    if (argc > 1) keyexpr = argv[1];
    if (argc > 2) value = argv[2];
    if (argc > 3) add_matching_listener = atoi(argv[3]);

    z_owned_config_t config;
    z_config_default(&config);

    // A probing procedure for shared memory is performed upon session opening. To enable `z_pub_shm` to operate
    // over shared memory (and to not fallback on network mode), shared memory needs to be enabled also on the
    // subscriber side. By doing so, the probing procedure will succeed and shared memory will operate as expected.
    if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_SHARED_MEMORY_KEY, "true") < 0) {
        printf(
            "Couldn't insert value `true` in configuration at `%s`. This is likely because `%s` expects a "
            "JSON-serialized value\n",
            Z_CONFIG_SHARED_MEMORY_KEY, Z_CONFIG_SHARED_MEMORY_KEY);
        exit(-1);
    }

    if (argc > 4) {
        if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_CONNECT_KEY, argv[4]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[4], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config)) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring Publisher on '%s'...\n", keyexpr);
    z_owned_publisher_t pub;
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);
    if (z_declare_publisher(&pub, z_loan(s), z_loan(ke), NULL) < 0) {
        printf("Unable to declare Publisher for key expression!\n");
        exit(-1);
    }
#ifdef UNSTABLE
    zc_owned_matching_listener_t listener;
    if (add_matching_listener) {
        zc_owned_closure_matching_status_t callback;
        z_closure(&callback, matching_status_handler, NULL, NULL);
        zc_publisher_matching_listener_declare(&listener, z_loan(pub), z_move(callback));
    }
#else
    if (add_matching_listener) {
        printf("To enable matching listener you must compile Zenoh-c with unstable feature support!\n");
        exit(-1);
    }
#endif

    printf("Creating POSIX SHM Provider...\n");
    const size_t total_size = 4096;
    const size_t buf_ok_size = total_size / 4;

    z_alloc_alignment_t alignment = {0};
    z_owned_memory_layout_t layout;
    z_memory_layout_new(&layout, total_size, alignment);

    z_owned_shm_provider_t provider;
    z_posix_shm_provider_new(&provider, z_loan(layout));

    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);

        z_buf_layout_alloc_result_t alloc;
        z_shm_provider_alloc_gc_defrag_blocking(&alloc, z_loan(provider), buf_ok_size, alignment);
        if (z_check(alloc.buf)) {
            {
                uint8_t *buf = z_shm_mut_data_mut(z_loan_mut(alloc.buf));
                sprintf((char *)buf, "[%4d] %s", idx, value);
                printf("Putting Data ('%s': '%s')...\n", keyexpr, buf);
            }

            z_publisher_put_options_t options;
            z_publisher_put_options_default(&options);

            z_owned_bytes_t payload;
            z_bytes_serialize_from_shm_mut(&payload, &alloc.buf);

            z_publisher_put(z_loan(pub), z_move(payload), &options);
        } else {
            printf("Unexpected failure during SHM buffer allocation...");
            break;
        }
    }

#ifdef UNSTABLE
    if (add_matching_listener) {
        zc_publisher_matching_listener_undeclare(z_move(listener));
    }
#endif

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));

    z_drop(z_move(provider));
    z_drop(z_move(layout));

    return 0;
}