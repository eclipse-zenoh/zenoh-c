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
#include <string.h>

#include "zenoh.h"

#if defined(Z_FEATURE_UNSTABLE_API)

// Callback that takes the first transport from loaned pointer
// Fails if called multiple times (indicating multiple transports exist)
void capture_transport_handler(z_loaned_transport_t* transport, void* arg) {
    z_owned_transport_t* ctx = (z_owned_transport_t*)arg;
    if (z_internal_transport_check(ctx)) {
        printf("FAIL: capture_transport_handler called multiple times - multiple transports detected\n");
        printf("      This may indicate external zenoh sessions are connecting\n");
        exit(1);
    }
    z_take_from_loaned(ctx, transport);
}

// Callback that takes the first link from loaned pointer
// Fails if called multiple times (indicating multiple links exist)
void capture_link_handler(z_loaned_link_t* link, void* arg) {
    z_owned_link_t* ctx = (z_owned_link_t*)arg;
    if (z_internal_link_check(ctx)) {
        printf("FAIL: capture_link_handler called multiple times - multiple links detected\n");
        printf("      This may indicate external zenoh sessions are connecting\n");
        exit(1);
    }
    z_take_from_loaned(ctx, link);
}

#endif

// Helper to configure an isolated session that won't connect to external zenoh nodes
// Session 1: listens on a specific port
// Session 2: connects to session 1's port
// Both sessions have scouting disabled to prevent discovery of external nodes
int create_isolated_session_pair(z_owned_session_t* s1, z_owned_session_t* s2) {
    // Create config for session 1: listener on localhost
    z_owned_config_t config1;
    z_config_default(&config1);
    
    // Set mode to peer
    zc_config_insert_json5(z_loan_mut(config1), "mode", "\"peer\"");
    
    // Disable multicast scouting
    zc_config_insert_json5(z_loan_mut(config1), "scouting/multicast/enabled", "false");
    
    // Disable gossip scouting
    zc_config_insert_json5(z_loan_mut(config1), "scouting/gossip/enabled", "false");
    
    // Listen only on localhost with a specific port
    zc_config_insert_json5(z_loan_mut(config1), "listen/endpoints", "[\"tcp/127.0.0.1:17447\"]");
    
    // Don't connect to anything
    zc_config_insert_json5(z_loan_mut(config1), "connect/endpoints", "[]");
    
    if (z_open(s1, z_move(config1), NULL) < 0) {
        printf("Unable to open session 1!\n");
        return -1;
    }
    
    // Give session 1 time to start listening
    sleep(1);

    // Create config for session 2: connects to session 1
    z_owned_config_t config2;
    z_config_default(&config2);
    
    // Set mode to peer
    zc_config_insert_json5(z_loan_mut(config2), "mode", "\"peer\"");
    
    // Disable multicast scouting
    zc_config_insert_json5(z_loan_mut(config2), "scouting/multicast/enabled", "false");
    
    // Disable gossip scouting
    zc_config_insert_json5(z_loan_mut(config2), "scouting/gossip/enabled", "false");
    
    // Don't listen on any port
    zc_config_insert_json5(z_loan_mut(config2), "listen/endpoints", "[]");
    
    // Connect to session 1
    zc_config_insert_json5(z_loan_mut(config2), "connect/endpoints", "[\"tcp/127.0.0.1:17447\"]");
    
    if (z_open(s2, z_move(config2), NULL) < 0) {
        printf("Unable to open session 2!\n");
        z_drop(z_move(*s1));
        return -1;
    }

    // Sleep to allow sessions to establish transports
    sleep(1);

    return 0;
}

#if defined(Z_FEATURE_UNSTABLE_API)

int test_z_info_transports() {
    printf("=== Testing z_info_transports ===\n");
    z_owned_session_t s1, s2;
    if (create_isolated_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    // Capture transport from session 1
    z_owned_transport_t transport;
    z_internal_transport_null(&transport);
    
    z_owned_closure_transport_t callback;
    z_closure(&callback, capture_transport_handler, NULL, &transport);
    z_info_transports(z_loan(s1), z_move(callback));

    // Verify transport was captured
    if (!z_internal_transport_check(&transport)) {
        printf("FAIL: No transport found from session 1\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Get ZID from captured transport
    z_id_t captured_zid = z_transport_zid(z_loan(transport));
    z_owned_string_t zid_str;
    z_id_to_string(&captured_zid, &zid_str);
    printf("Session 1 transport: zid=%.*s\n",
           (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)));
    z_drop(z_move(zid_str));

    // Verify the captured transport matches session 2's ZID
    z_id_t s2_zid = z_info_zid(z_loan(s2));
    
    if (memcmp(s2_zid.id, captured_zid.id, sizeof(s2_zid.id)) == 0) {
        printf("PASS: Session 1's transport connects to session 2 (ZIDs match)\n\n");
    } else {
        printf("FAIL: Captured transport ZID doesn't match session 2's ZID\n");
        z_drop(z_move(transport));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Cleanup
    z_drop(z_move(transport));
    z_close(z_loan_mut(s1), NULL);
    z_drop(z_move(s1));

    z_close(z_loan_mut(s2), NULL);
    z_drop(z_move(s2));

    return 0;
}

int test_z_info_links() {
    printf("=== Testing z_info_links ===\n");
    z_owned_session_t s1, s2;
    if (create_isolated_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    // Capture link from session 1
    z_owned_link_t link;
    z_internal_link_null(&link);
    
    z_owned_closure_link_t callback;
    z_closure(&callback, capture_link_handler, NULL, &link);
    z_info_links(z_loan(s1), z_move(callback), NULL);

    if (!z_internal_link_check(&link)) {
        printf("FAIL: No link found from session 1\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Get ZID from captured link
    z_id_t captured_zid = z_link_zid(z_loan(link));
    z_owned_string_t zid_str;
    z_id_to_string(&captured_zid, &zid_str);
    printf("Session 1 link: zid=%.*s\n",
           (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)));

    // Verify link connects to session 2
    z_id_t s2_zid = z_info_zid(z_loan(s2));
    if (memcmp(s2_zid.id, captured_zid.id, sizeof(s2_zid.id)) == 0) {
        printf("PASS: Session 1's link connects to session 2 (ZIDs match)\n\n");
    } else {
        printf("FAIL: Captured link ZID doesn't match session 2's ZID\n");
        z_drop(z_move(zid_str));
        z_drop(z_move(link));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    z_drop(z_move(zid_str));
    z_drop(z_move(link));

    z_close(z_loan_mut(s1), NULL);
    z_drop(z_move(s1));
    z_close(z_loan_mut(s2), NULL);
    z_drop(z_move(s2));

    return 0;
}

int test_z_info_links_filtered() {
    printf("=== Testing z_info_links with transport filter ===\n");
    z_owned_session_t s1, s2;
    if (create_isolated_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    // Capture a transport to use as filter
    z_owned_transport_t transport;
    z_internal_transport_null(&transport);
    
    z_owned_closure_transport_t transport_callback;
    z_closure(&transport_callback, capture_transport_handler, NULL, &transport);
    z_info_transports(z_loan(s1), z_move(transport_callback));

    if (!z_internal_transport_check(&transport)) {
        printf("FAIL: No transport captured for filtering\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Now capture links filtered by the transport
    z_owned_link_t link;
    z_internal_link_null(&link);
    
    z_info_links_options_t options;
    z_info_links_options_default(&options);
    options.transport = z_transport_move(&transport);
    
    z_owned_closure_link_t link_callback;
    z_closure(&link_callback, capture_link_handler, NULL, &link);
    z_info_links(z_loan(s1), z_move(link_callback), &options);

    if (z_internal_link_check(&link)) {
        z_id_t link_zid = z_link_zid(z_loan(link));
        z_owned_string_t zid_str;
        z_id_to_string(&link_zid, &zid_str);
        printf("Filtered link: zid=%.*s\n",
               (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)));
        z_drop(z_move(zid_str));
        printf("PASS: z_info_links with transport filter works\n\n");
    } else {
        printf("FAIL: No link found for filtered transport\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    z_drop(z_move(link));

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
    // Test transports
    if (test_z_info_transports() != 0) {
        return -1;
    }

    // Test links
    if (test_z_info_links() != 0) {
        return -1;
    }

    // Test filtered links
    if (test_z_info_links_filtered() != 0) {
        return -1;
    }

    printf("\nAll tests completed successfully!\n");
#else
    printf("Skipping tests: Z_FEATURE_UNSTABLE_API not enabled\n");
#endif

    return 0;
}
