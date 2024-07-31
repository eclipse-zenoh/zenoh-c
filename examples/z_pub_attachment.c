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

#include "zenoh.h"

typedef struct kv_pair_t {
    const char* key;
    const char* value;
} kv_pair_t;

typedef struct kv_it {
    kv_pair_t* current;
    kv_pair_t* end;
} kv_it;

bool create_attachment_iter(z_owned_bytes_t* kv_pair, void* context) {
    kv_it* it = (kv_it*)(context);
    if (it->current == it->end) {
        return false;
    }
    z_owned_bytes_t k, v;
    z_bytes_serialize_from_str(&k, it->current->key);
    z_bytes_serialize_from_str(&v, it->current->value);
    z_bytes_from_pair(kv_pair, z_move(k), z_move(v));
    it->current++;
    return true;
};

int main(int argc, char** argv) {
    char* keyexpr = "demo/example/zenoh-c-pub";
    char* value = "Pub from C!";

    if (argc > 1) keyexpr = argv[1];
    if (argc > 2) value = argv[2];

    z_owned_config_t config;
    z_config_default(&config);
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
    if (z_open(&s, z_move(config)) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring Publisher on '%s'...\n", keyexpr);
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);
    z_owned_publisher_t pub;
    if (z_declare_publisher(&pub, z_loan(s), z_loan(ke), NULL)) {
        printf("Unable to declare Publisher for key expression!\n");
        exit(-1);
    }

    z_publisher_put_options_t options;
    z_publisher_put_options_default(&options);

    // allocate attachment data
    kv_pair_t kvs[2];
    kvs[0] = (kv_pair_t){.key = "source", .value = "C"};
    // allocate attachment and payload
    z_owned_bytes_t attachment;
    z_owned_bytes_t payload;

    char buf[256];
    char buf_ind[16];
    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);

        // add some other attachment value
        sprintf(buf_ind, "%d", idx);
        kvs[1] = (kv_pair_t){.key = "index", .value = buf_ind};
        kv_it it = {.current = kvs, .end = kvs + 2};
        z_bytes_from_iter(&attachment, create_attachment_iter, (void*)&it);
        options.attachment = &attachment;

        sprintf(buf, "[%4d] %s", idx, value);
        printf("Putting Data ('%s': '%s')...\n", keyexpr, buf);

        z_bytes_serialize_from_str(&payload, buf);
        z_publisher_put(z_loan(pub), z_move(payload), &options);
    }

    z_undeclare_publisher(z_move(pub));

    z_close(z_move(s));

    return 0;
}
