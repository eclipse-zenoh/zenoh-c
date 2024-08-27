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

int main(int argc, char **argv) {
    if (argc < 2) {
        printf("USAGE:\n\tz_pub_thr <payload-size> [<zenoh-locator>]\n\n");
        exit(-1);
    }

    char *keyexpr = "test/thr";
    size_t len = atoi(argv[1]);

    z_owned_config_t config;
    z_config_default(&config);

    if (argc > 2) {
        if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_CONNECT_KEY, argv[2]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[2], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    z_owned_session_t s;
    if (z_open(&s, z_move(config)) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_publisher_options_t options;
    z_publisher_options_default(&options);
    options.congestion_control = Z_CONGESTION_CONTROL_BLOCK;

    z_owned_publisher_t pub;
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);
    if (z_declare_publisher(&pub, z_loan(s), z_loan(ke), &options)) {
        printf("Unable to declare publisher for key expression!\n");
        exit(-1);
    }

    printf("Creating POSIX SHM Provider...\n");
    z_alloc_alignment_t alignment = {0};
    z_owned_memory_layout_t layout;
    z_memory_layout_new(&layout, len, alignment);
    z_owned_shm_provider_t provider;
    z_posix_shm_provider_new(&provider, z_loan(layout));

    printf("Allocating single SHM buffer\n");
    z_buf_layout_alloc_result_t alloc;
    z_shm_provider_alloc(&alloc, z_loan(provider), len, alignment);
    if (alloc.status != ZC_BUF_LAYOUT_ALLOC_STATUS_OK) {
        printf("Unexpected failure during SHM buffer allocation...\n");
        return -1;
    }
    memset(z_shm_mut_data_mut(z_loan_mut(alloc.buf)), 1, len);
    z_owned_shm_t shm;
    z_shm_from_mut(&shm, z_move(alloc.buf));

    z_owned_bytes_t shmbs;
    if (z_bytes_serialize_from_shm(&shmbs, z_move(shm)) != Z_OK) {
        printf("Unexpected failure during SHM buffer serialization...\n");
        return -1;
    }

    while (1) {
        z_owned_bytes_t payload;
        z_bytes_clone(&payload, z_loan(shmbs));
        z_publisher_put(z_loan(pub), z_move(payload), NULL);
    }

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));

    z_drop(z_move(shm));
    z_drop(z_move(provider));
    z_drop(z_move(layout));
}
