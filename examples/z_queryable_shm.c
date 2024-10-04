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

#include "zenoh.h"

const char *keyexpr = "demo/example/zenoh-c-queryable";
const char *value = "Queryable from C SHM!";
z_view_keyexpr_t ke;

void query_handler(z_loaned_query_t *query, void *context) {
    z_loaned_shm_provider_t *provider = (z_loaned_shm_provider_t *)context;

    z_view_string_t key_string;
    z_keyexpr_as_view_string(z_query_keyexpr(query), &key_string);

    z_view_string_t params;
    z_query_parameters(query, &params);

    const z_loaned_bytes_t *payload = z_query_payload(query);
    if (payload != NULL && z_bytes_len(payload) > 0) {
        const z_loaned_shm_t *shm = NULL;
        char *payload_type = z_bytes_to_loaned_shm(payload, &shm) == Z_OK ? "SHM" : "RAW";

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

    size_t value_len = strlen(value) + 1;  // + NULL terminator
    z_alloc_alignment_t alignment = {0};
    z_buf_layout_alloc_result_t alloc;
    z_shm_provider_alloc_gc_defrag_blocking(&alloc, provider, value_len, alignment);
    if (alloc.status == ZC_BUF_LAYOUT_ALLOC_STATUS_OK) {
        {
            uint8_t *buf = z_shm_mut_data_mut(z_loan_mut(alloc.buf));
            memcpy(buf, value, value_len);
            printf("Putting Data ('%s': '%s')...\n", keyexpr, buf);
        }

        z_query_reply_options_t options;
        z_query_reply_options_default(&options);

        z_owned_bytes_t reply_payload;
        z_bytes_from_shm_mut(&reply_payload, z_move(alloc.buf));

        z_view_keyexpr_t reply_keyexpr;
        z_view_keyexpr_from_str(&reply_keyexpr, (const char *)context);

        z_query_reply(query, z_loan(reply_keyexpr), z_move(reply_payload), &options);

    } else {
        printf("Unexpected failure during SHM buffer allocation...");
        exit(-1);
    }
}

int main(int argc, char **argv) {
    if (argc > 1) {
        keyexpr = argv[1];
    }

    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    z_config_default(&config);
    if (argc > 2) {
        if (zc_config_insert_json5(z_loan_mut(config), Z_CONFIG_CONNECT_KEY, argv[2]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[2], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    if (z_view_keyexpr_from_str(&ke, keyexpr)) {
        printf("%s is not a valid key expression", keyexpr);
        exit(-1);
    }

    printf("Creating POSIX SHM Provider...\n");
    const size_t total_size = 4096;
    z_alloc_alignment_t alignment = {0};
    z_owned_memory_layout_t layout;
    z_memory_layout_new(&layout, total_size, alignment);
    z_owned_shm_provider_t provider;
    z_posix_shm_provider_new(&provider, z_loan(layout));

    printf("Declaring Queryable on '%s'...\n", keyexpr);
    z_owned_closure_query_t callback;
    z_closure(&callback, query_handler, (void *)z_loan(provider), (void *)keyexpr);
    z_owned_queryable_t qable;

    if (z_declare_queryable(&qable, z_loan(s), z_loan(ke), z_move(callback), NULL) < 0) {
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

    z_undeclare_queryable(z_move(qable));
    z_drop(z_move(s));
    z_drop(z_move(layout));
    z_drop(z_move(provider));
    return 0;
}
