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

int main(int argc, char **argv) {
    char *keyexpr = "demo/example/zenoh-c-pub";
    char *value = "Pub from C!";

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
    z_view_keyexpr(&ke, keyexpr);
    z_owned_publisher_t pub;
    if (z_declare_publisher(&pub, z_loan(s), z_loan(ke), NULL)) {
        printf("Unable to declare Publisher for key expression!\n");
        exit(-1);
    }

    z_publisher_put_options_t options;
    z_publisher_put_options_default(&options);

    // allocate attachment map
    z_owned_slice_map_t map;
    z_slice_map_new(&map);
    z_view_slice_t src_key, src_value;
    z_view_slice_from_str(&src_key, "source");
    z_view_slice_from_str(&src_value, "C");
    // add some value
    z_slice_map_insert_by_alias(z_loan_mut(map), z_loan(src_key), z_loan(src_value));
    // allocate attachment and payload
    z_owned_bytes_t attachment;
    z_owned_bytes_t payload;

    char buf[256];
    char buf_ind[16];
    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);

        // add some other attachment value
        sprintf(buf_ind, "%d", idx);
        z_view_slice_t index_key, index_value;
        z_view_slice_from_str(&index_key, "index");
        z_view_slice_from_str(&index_value, buf_ind);
        z_slice_map_insert_by_alias(z_loan_mut(map), z_loan(index_key), z_loan(index_value));
        z_bytes_encode_from_bytes_map(&attachment, z_loan(map));
        options.attachment = &attachment;

        sprintf(buf, "[%4d] %s", idx, value);
        printf("Putting Data ('%s': '%s')...\n", keyexpr, buf);
        z_view_str_t payload_str;
        z_view_str_wrap(&payload_str, buf);
        
        z_bytes_encode_from_string(&payload, z_loan(payload_str));
        z_publisher_put(z_loan(pub), z_move(payload), &options);
    }

    z_undeclare_publisher(z_move(pub));

    z_close(z_move(s));
    z_drop(z_move(map));

    return 0;
}
