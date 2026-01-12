//
// Copyright (c) 2024 ZettaScale Technology
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
#include <unistd.h>
#include <assert.h>

#include "zenoh.h"

#if defined(Z_FEATURE_UNSTABLE_API)

// Global counter for transport callbacks
volatile unsigned int transport_count = 0;

void transport_handler(z_loaned_transport_t* transport, void* arg) {
    transport_count++;

    // Get transport information
    z_id_t zid = z_transport_zid(transport);
    z_whatami_t whatami = z_transport_whatami(transport);
    bool is_qos = z_transport_is_qos(transport);

    // Convert ZID to string for printing
    z_owned_string_t zid_str;
    z_id_to_string(&zid, &zid_str);
    printf("Transport %u: zid=%.*s, whatami=%d, qos=%s\n",
           transport_count,
           (int)z_string_len(z_loan(zid_str)),
           z_string_data(z_loan(zid_str)),
           whatami,
           is_qos ? "true" : "false");
    z_drop(z_move(zid_str));
}

int create_session_pair(z_owned_session_t* s1, z_owned_session_t* s2) {
    // Create and open first session
    z_owned_config_t config1;
    z_config_default(&config1);
    if (z_open(s1, z_move(config1), NULL) < 0) {
        printf("Unable to open session 1!\n");
        return -1;
    }

    // Create and open second session
    z_owned_config_t config2;
    z_config_default(&config2);
    if (z_open(s2, z_move(config2), NULL) < 0) {
        printf("Unable to open session 2!\n");
        z_drop(z_move(*s1));
        return -1;
    }

    // Sleep to allow sessions to establish transports
    sleep(1);

    return 0;
}

int test_z_info_transports() {
    z_owned_session_t s1, s2;
    if (create_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    // Get transport information from first session
    printf("Transports from session 1:\n");
    transport_count = 0;
    z_owned_closure_transport_t callback;
    z_closure(&callback, transport_handler, NULL, NULL);
    z_info_transports(z_loan(s1), z_move(callback));

    unsigned int expected_transports = 1;  // At least one transport to s2
    if (transport_count >= expected_transports) {
        printf("PASS: Received %u transport callback(s) from session 1\n", transport_count);
    } else {
        printf("FAIL: Expected at least %u transport(s) from session 1, got %u\n",
               expected_transports, transport_count);
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Get transport information from second session
    printf("\nTransports from session 2:\n");
    transport_count = 0;
    z_closure(&callback, transport_handler, NULL, NULL);
    z_info_transports(z_loan(s2), z_move(callback));

    if (transport_count >= expected_transports) {
        printf("PASS: Received %u transport callback(s) from session 2\n", transport_count);
    } else {
        printf("FAIL: Expected at least %u transport(s) from session 2, got %u\n",
               expected_transports, transport_count);
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Cleanup
    z_close(z_loan_mut(s1), NULL);
    z_drop(z_move(s1));

    z_close(z_loan_mut(s2), NULL);
    z_drop(z_move(s2));

    return 0;
}
#endif

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

#if defined(Z_FEATURE_UNSTABLE_API)
    if (test_z_info_transports() != 0) {
        return -1;
    }

    printf("\nTest completed successfully!\n");
#else
    printf("Skipping z_info_transports test: Z_FEATURE_UNSTABLE_API not enabled\n");
#endif

    return 0;
}
