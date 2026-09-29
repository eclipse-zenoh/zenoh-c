//
// Copyright (c) 2026 ZettaScale Technology
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

#pragma once

#include <stdio.h>
#include <stdlib.h>

#include "zenoh.h"

static inline void z_test_require_ok(z_result_t result, const char* operation) {
    if (result != Z_OK) {
        fprintf(stderr, "%s failed: %d\n", operation, (int)result);
        abort();
    }
}

static inline void z_test_isolated_config(z_owned_config_t* config) {
    z_test_require_ok(z_config_default(config), "Create test config");
    z_test_require_ok(zc_config_insert_json5(z_loan_mut(*config), "mode", "\"peer\""), "Set peer mode");
    z_test_require_ok(zc_config_insert_json5(z_loan_mut(*config), "scouting/multicast/enabled", "false"),
                      "Disable multicast scouting");
    z_test_require_ok(zc_config_insert_json5(z_loan_mut(*config), "scouting/gossip/enabled", "false"),
                      "Disable gossip scouting");
    z_test_require_ok(zc_config_insert_json5(z_loan_mut(*config), "listen/endpoints", "[]"), "Clear listeners");
    z_test_require_ok(zc_config_insert_json5(z_loan_mut(*config), "connect/endpoints", "[]"), "Clear connectors");
}

// Open a private pair without discovering other tests or routers on the host.
// Keep the successful listener open while connecting; probing and releasing a
// free port first would let another process claim it before z_open binds it.
static inline void z_test_open_session_pair(z_owned_session_t* listener, z_owned_session_t* connector) {
    for (unsigned attempt = 0; attempt < 16; ++attempt) {
        // Avoid the default ephemeral port ranges on Linux, macOS and Windows.
        unsigned port = 20000u + z_random_u32() % 12768u;
        char endpoints[40];
        snprintf(endpoints, sizeof(endpoints), "[\"tcp/127.0.0.1:%u\"]", port);
        z_owned_config_t config;
        z_test_isolated_config(&config);
        z_test_require_ok(zc_config_insert_json5(z_loan_mut(config), "listen/endpoints", endpoints), "Set listener");
        z_test_require_ok(zc_config_insert_json5(z_loan_mut(config), "listen/timeout_ms", "0"),
                          "Disable listen retries");
        z_test_require_ok(zc_config_insert_json5(z_loan_mut(config), "listen/exit_on_failure", "true"),
                          "Reject failed listener");
        z_result_t result = z_open(listener, z_move(config), NULL);
        if (result != Z_OK) {
            z_drop(z_move(*listener));
            if (attempt == 15) z_test_require_ok(result, "Open isolated test listener after 16 attempts");
            continue;
        }

        z_test_isolated_config(&config);
        z_test_require_ok(zc_config_insert_json5(z_loan_mut(config), "connect/endpoints", endpoints), "Set connector");
        z_test_require_ok(zc_config_insert_json5(z_loan_mut(config), "connect/timeout_ms", "5000"),
                          "Bound connection wait");
        z_test_require_ok(zc_config_insert_json5(z_loan_mut(config), "connect/exit_on_failure", "true"),
                          "Require successful connection");
        result = z_open(connector, z_move(config), NULL);
        if (result != Z_OK) {
            z_drop(z_move(*connector));
            z_drop(z_move(*listener));
            z_test_require_ok(result, "Connect isolated test sessions");
        }
        return;
    }
}
