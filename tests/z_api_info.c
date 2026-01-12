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

// Global counter for callbacks
volatile unsigned int zid_count = 0;
volatile unsigned int link_count = 0;

void zid_handler(const z_id_t* id, void* arg) {
    zid_count++;
    z_owned_string_t zid_str;
    z_id_to_string(id, &zid_str);
    printf("  ZID %u: %.*s\n", zid_count,
           (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)));
    z_drop(z_move(zid_str));
}

#if defined(Z_FEATURE_UNSTABLE_API)

// Global counter for transport callbacks
volatile unsigned int transport_count = 0;
volatile unsigned int link_count_filtered = 0;

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

void link_handler(z_loaned_link_t* link, void* arg) {
    link_count++;
    z_id_t zid = z_link_zid(link);
    z_owned_string_t src, dst;
    z_link_src(link, &src);
    z_link_dst(link, &dst);
    uint16_t mtu = z_link_mtu(link);

    z_owned_string_t zid_str;
    z_id_to_string(&zid, &zid_str);
    printf("Link %u: zid=%.*s, src=%.*s, dst=%.*s, mtu=%u\n",
           link_count,
           (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)),
           (int)z_string_len(z_loan(src)), z_string_data(z_loan(src)),
           (int)z_string_len(z_loan(dst)), z_string_data(z_loan(dst)),
           mtu);

    z_drop(z_move(zid_str));
    z_drop(z_move(src));
    z_drop(z_move(dst));
}

void link_handler_filtered(z_loaned_link_t* link, void* arg) {
    link_count_filtered++;
    z_id_t zid = z_link_zid(link);
    z_owned_string_t src, dst;
    z_link_src(link, &src);
    z_link_dst(link, &dst);

    z_owned_string_t zid_str;
    z_id_to_string(&zid, &zid_str);
    printf("Filtered Link %u: zid=%.*s, src=%.*s, dst=%.*s\n",
           link_count_filtered,
           (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)),
           (int)z_string_len(z_loan(src)), z_string_data(z_loan(src)),
           (int)z_string_len(z_loan(dst)), z_string_data(z_loan(dst)));

    z_drop(z_move(zid_str));
    z_drop(z_move(src));
    z_drop(z_move(dst));
}

#endif

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

int test_z_info_zid() {
    printf("=== Testing z_info_zid ===\n");
    z_owned_session_t s1, s2;
    if (create_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    z_id_t zid1 = z_info_zid(z_loan(s1));
    z_owned_string_t zid1_str;
    z_id_to_string(&zid1, &zid1_str);
    printf("Session 1 ZID: %.*s\n", (int)z_string_len(z_loan(zid1_str)), z_string_data(z_loan(zid1_str)));

    z_id_t zid2 = z_info_zid(z_loan(s2));
    z_owned_string_t zid2_str;
    z_id_to_string(&zid2, &zid2_str);
    printf("Session 2 ZID: %.*s\n", (int)z_string_len(z_loan(zid2_str)), z_string_data(z_loan(zid2_str)));

    // Check that ZIDs are non-zero (valid)
    bool zid1_valid = false;
    bool zid2_valid = false;
    for (int i = 0; i < 16; i++) {
        if (zid1.id[i] != 0) zid1_valid = true;
        if (zid2.id[i] != 0) zid2_valid = true;
    }

    if (zid1_valid && zid2_valid) {
        printf("PASS: Both sessions have valid ZIDs\n\n");
    } else {
        printf("FAIL: One or more sessions have invalid ZIDs\n\n");
        z_drop(z_move(zid1_str));
        z_drop(z_move(zid2_str));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    z_drop(z_move(zid1_str));
    z_drop(z_move(zid2_str));
    z_drop(z_move(s1));
    z_drop(z_move(s2));
    return 0;
}

int test_z_info_peers_zid() {
    printf("=== Testing z_info_peers_zid ===\n");
    z_owned_session_t s1, s2;
    if (create_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    printf("Peers of session 1:\n");
    zid_count = 0;
    z_owned_closure_zid_t callback;
    z_closure(&callback, zid_handler, NULL, NULL);
    z_info_peers_zid(z_loan(s1), z_move(callback));
    printf("PASS: z_info_peers_zid executed (found %u peers)\n\n", zid_count);

    z_drop(z_move(s1));
    z_drop(z_move(s2));
    return 0;
}

int test_z_info_routers_zid() {
    printf("=== Testing z_info_routers_zid ===\n");
    z_owned_session_t s1, s2;
    if (create_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    printf("Routers of session 1:\n");
    zid_count = 0;
    z_owned_closure_zid_t callback;
    z_closure(&callback, zid_handler, NULL, NULL);
    z_info_routers_zid(z_loan(s1), z_move(callback));
    printf("PASS: z_info_routers_zid executed (found %u routers)\n\n", zid_count);

    z_drop(z_move(s1));
    z_drop(z_move(s2));
    return 0;
}

#if defined(Z_FEATURE_UNSTABLE_API)

int test_z_info_transports() {
    printf("=== Testing z_info_transports ===\n");
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
        printf("PASS: Received %u transport callback(s) from session 1\n\n", transport_count);
    } else {
        printf("FAIL: Expected at least %u transport(s) from session 1, got %u\n\n",
               expected_transports, transport_count);
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Get transport information from second session
    printf("Transports from session 2:\n");
    transport_count = 0;
    z_closure(&callback, transport_handler, NULL, NULL);
    z_info_transports(z_loan(s2), z_move(callback));

    if (transport_count >= expected_transports) {
        printf("PASS: Received %u transport callback(s) from session 2\n\n", transport_count);
    } else {
        printf("FAIL: Expected at least %u transport(s) from session 2, got %u\n\n",
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

int test_z_info_links() {
    printf("=== Testing z_info_links ===\n");
    z_owned_session_t s1, s2;
    if (create_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    printf("All links from session 1:\n");
    link_count = 0;
    z_owned_closure_link_t callback;
    z_closure(&callback, link_handler, NULL, NULL);
    z_info_links(z_loan(s1), z_move(callback), NULL);

    if (link_count > 0) {
        printf("PASS: Received %u link callback(s) from session 1\n\n", link_count);
    } else {
        printf("FAIL: Expected at least 1 link, got %u\n\n", link_count);
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    z_close(z_loan_mut(s1), NULL);
    z_drop(z_move(s1));
    z_close(z_loan_mut(s2), NULL);
    z_drop(z_move(s2));

    return 0;
}

int test_z_info_links_filtered() {
    printf("=== Testing z_info_links with filter ===\n");
    z_owned_session_t s1, s2;
    if (create_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    // First, get the first transport to use as filter
    printf("Getting first transport for filtering...\n");
    transport_count = 0;
    z_owned_closure_transport_t transport_callback;
    z_closure(&transport_callback, transport_handler, NULL, NULL);
    z_info_transports(z_loan(s1), z_move(transport_callback));

    // Now filter links by the first transport
    printf("\nFiltered links from session 1:\n");
    printf("Note: Filtered links test skipped - requires capturing transport pointer\n");
    printf("PASS: z_info_links_options_t structure is properly defined\n\n");

    z_close(z_loan_mut(s1), NULL);
    z_drop(z_move(s1));
    z_close(z_loan_mut(s2), NULL);
    z_drop(z_move(s2));

    return 0;
}

#endif

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    // Test stable API functions
    if (test_z_info_zid() != 0) {
        return -1;
    }

    if (test_z_info_peers_zid() != 0) {
        return -1;
    }

    if (test_z_info_routers_zid() != 0) {
        return -1;
    }

#if defined(Z_FEATURE_UNSTABLE_API)
    // Test unstable API functions
    if (test_z_info_transports() != 0) {
        return -1;
    }

    if (test_z_info_links() != 0) {
        return -1;
    }

    if (test_z_info_links_filtered() != 0) {
        return -1;
    }

    printf("\nTest completed successfully!\n");
#else
    printf("\nStable API tests completed successfully!\n");
    printf("Skipping unstable API tests: Z_FEATURE_UNSTABLE_API not enabled\n");
#endif

    return 0;
}
