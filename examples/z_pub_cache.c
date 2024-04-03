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

    z_owned_config_t config = z_config_default();
    if (argc > 3) {
        if (zc_config_insert_json(z_loan(config), Z_CONFIG_CONNECT_KEY, argv[3]) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
                "JSON-serialized list of strings\n",
                argv[3], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    if (zc_config_insert_json(z_loan(config), Z_CONFIG_ADD_TIMESTAMP_KEY, "true") < 0) {
        printf("Unable to configure timestamps!\n");
        exit(-1);
    }

    printf("Opening session...\n");
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    ze_publication_cache_options_t pub_cache_opts = ze_publication_cache_options_default();
    pub_cache_opts.history = 42;
    pub_cache_opts.queryable_complete = false;

    printf("Declaring publication cache on '%s'...\n", keyexpr);
    ze_owned_publication_cache_t pub_cache =
        ze_declare_publication_cache(z_loan(s), z_keyexpr(keyexpr), &pub_cache_opts);
    if (!z_check(pub_cache)) {
        printf("Unable to declare publication cache for key expression!\n");
        exit(-1);
    }

    char buf[256];
    for (int idx = 0; 1; ++idx) {
        z_sleep_s(1);
        sprintf(buf, "[%4d] %s", idx, value);
        printf("Putting Data ('%s': '%s')...\n", keyexpr, buf);
        zc_owned_payload_t payload = zc_payload_encode_from_string(buf);
        z_put(z_loan(s), z_keyexpr(keyexpr), z_move(payload), NULL);
    }

    z_drop(z_move(pub_cache));
    z_close(z_move(s));

    return 0;
}
