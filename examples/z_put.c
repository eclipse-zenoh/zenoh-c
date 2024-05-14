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
    char *keyexpr = "demo/example/zenoh-c-put";
    char *value = "Put from C!";

    if (argc > 1) keyexpr = argv[1];
    if (argc > 2) value = argv[2];

    z_owned_slice_map_t attachment_map;
    z_slice_map_new(&attachment_map);
    z_view_slice_t map_key, map_value;
    z_view_slice_from_str(&map_key, "hello");
    z_view_slice_from_str(&map_value, "there");
    z_slice_map_insert_by_alias(z_loan_mut(attachment_map), z_loan(map_key), z_loan(map_value));

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

    printf("Putting Data ('%s': '%s')...\n", keyexpr, value);

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_string(&ke, keyexpr);

    z_view_str_t payload_string;
    z_view_str_wrap(&payload_string, value);

    z_owned_bytes_t payload;
    z_bytes_encode_from_string(&payload, z_loan(payload_string));
    
    z_owned_bytes_t attachment;
    z_bytes_encode_from_slice_map(&attachment, z_loan(attachment_map));

    z_put_options_t options;
    z_put_options_default(&options);
    options.attachment = &attachment; // attachement is going to be consumed by z_put, so no need to drop it manually
    
    int res = z_put(z_loan(s), z_loan(ke), z_move(payload), &options);
    if (res < 0) {
        printf("Put failed...\n");
    }

    z_close(z_move(s));
    z_drop(z_move(attachment_map));
    return 0;
}
