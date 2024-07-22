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

int main(int argc, char** argv) {
    char* expr = "demo/example/**";
    char* value = NULL;
    switch (argc) {
        default:
        case 3:
            value = argv[2];
        case 2:
            expr = argv[1];
            break;
        case 1:
            value = "Test Value";
            break;
    }
    size_t value_len = value ? strlen(value) : 0;

    z_view_keyexpr_t keyexpr;
    if (z_view_keyexpr_from_str(&keyexpr, expr) < 0) {
        printf("%s is not a valid key expression", expr);
        exit(-1);
    }
    z_owned_config_t config;
    z_config_default(&config);

    // A probing procedure for shared memory is performed upon session opening. To operate over shared memory
    // (and to not fallback on network mode), shared memory needs to be enabled in the configuration.
    if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_SHARED_MEMORY_KEY, "true") < 0) {
        printf(
            "Couldn't insert value `true` in configuration at `%s`. This is likely because `%s` expects a "
            "JSON-serialized value\n",
            argv[4], Z_CONFIG_SHARED_MEMORY_KEY, Z_CONFIG_SHARED_MEMORY_KEY);
        exit(-1);
    }

    if (argc > 3) {
        if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_CONNECT_KEY, argv[3]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[3], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config))) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    // Create SHM Provider
    z_alloc_alignment_t alignment = {0};
    z_owned_memory_layout_t layout;
    z_memory_layout_new(&layout, value_len, alignment);
    z_owned_shm_provider_t provider;
    z_posix_shm_provider_new(&provider, z_loan(layout));

    // Allocate SHM Buffer
    z_buf_layout_alloc_result_t alloc;
    z_shm_provider_alloc(&alloc, z_loan(provider), value_len, alignment);
    if (!z_check(alloc.buf)) {
        printf("Unexpected failure during SHM buffer allocation...");
        return -1;
    }
    // Fill SHM Buffer with data
    uint8_t* data = z_shm_mut_data_mut(z_loan_mut(alloc.buf));
    memcpy(data, value, value_len);
    // Convert mutable SHM Buffer into immutable one (to be able to make it's ref copies)
    z_owned_shm_t shm;
    z_shm_from_mut(&shm, z_move(alloc.buf));
    const z_loaned_shm_t* loaned_shm = z_loan(shm);

    printf("Sending Query '%s'...\n", expr);
    z_owned_fifo_handler_reply_t handler;
    z_owned_closure_reply_t closure;
    z_fifo_channel_reply_new(&closure, &handler, 16);

    z_get_options_t opts;
    z_get_options_default(&opts);

    z_owned_bytes_t payload;
    if (value != NULL) {
        z_bytes_serialize_from_shm_copy(&payload, z_loan(shm));
        opts.payload = &payload;
    }
    z_get(z_loan(s), z_loan(keyexpr), "", z_move(closure),
          z_move(opts));  // here, the send is moved and will be dropped by zenoh when adequate
    z_owned_reply_t reply;

    for (z_recv(z_loan(handler), &reply); z_check(reply); z_recv(z_loan(handler), &reply)) {
        if (z_reply_is_ok(z_loan(reply))) {
            const z_loaned_sample_t* sample = z_reply_ok(z_loan(reply));

            z_view_string_t key_str;
            z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_str);

            z_owned_string_t reply_str;
            z_bytes_deserialize_into_string(z_sample_payload(sample), &reply_str);

            printf(">> Received ('%.*s': '%.*s')\n", (int)z_string_len(z_loan(key_str)), z_string_data(z_loan(key_str)),
                   (int)z_string_len(z_loan(reply_str)), z_string_data(z_loan(reply_str)));
            z_drop(z_move(reply_str));
        } else {
            printf("Received an error\n");
        }
        z_drop(z_move(reply));
    }

    z_drop(z_move(handler));
    z_close(z_move(s));

    z_drop(z_move(shm));
    z_drop(z_move(provider));
    z_drop(z_move(layout));
    return 0;
}