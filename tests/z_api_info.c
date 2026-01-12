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

// Structure to capture transport ZID in callback
typedef struct {
    z_id_t first_zid;
    unsigned int count;
} transport_context_t;

// Structure to capture link ZID in callback
typedef struct {
    z_id_t first_zid;
    unsigned int count;
} link_context_t;

// Structure to hold owned transport for filtering
typedef struct {
    z_owned_transport_t transport;
    bool captured;
} transport_capture_t;

// Callback that captures the first transport ZID and counts all
void capture_transport_handler(z_loaned_transport_t* transport, void* arg) {
    transport_context_t* ctx = (transport_context_t*)arg;
    if (ctx->count == 0) {
        ctx->first_zid = z_transport_zid(transport);
    }
    ctx->count++;
}

// Callback that clones the first transport (for use in filtering)
void clone_transport_handler(z_loaned_transport_t* transport, void* arg) {
    transport_capture_t* ctx = (transport_capture_t*)arg;
    if (!ctx->captured) {
        z_transport_clone(&ctx->transport, transport);
        ctx->captured = true;
    }
}

// Callback that captures the first link ZID and counts all
void capture_link_handler(z_loaned_link_t* link, void* arg) {
    link_context_t* ctx = (link_context_t*)arg;
    if (ctx->count == 0) {
        ctx->first_zid = z_link_zid(link);
    }
    ctx->count++;
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

    // Capture transport ZID from session 1
    transport_context_t ctx1;
    memset(&ctx1.first_zid, 0, sizeof(ctx1.first_zid));
    ctx1.count = 0;
    
    z_owned_closure_transport_t callback;
    z_closure(&callback, capture_transport_handler, NULL, &ctx1);
    z_info_transports(z_loan(s1), z_move(callback));

    // Verify transport was captured
    if (ctx1.count == 0) {
        printf("FAIL: No transport found from session 1\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Print captured transport info
    z_owned_string_t zid_str;
    z_id_to_string(&ctx1.first_zid, &zid_str);
    printf("Session 1 transport: zid=%.*s\n",
           (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)));
    z_drop(z_move(zid_str));

    // Verify the captured transport matches session 2's ZID
    z_id_t s2_zid = z_info_zid(z_loan(s2));
    
    if (memcmp(s2_zid.id, ctx1.first_zid.id, sizeof(s2_zid.id)) == 0) {
        printf("PASS: Session 1's transport connects to session 2 (ZIDs match)\n\n");
    } else {
        printf("FAIL: Captured transport ZID doesn't match session 2's ZID\n");
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
    if (create_isolated_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    // Capture link info from session 1
    link_context_t ctx;
    memset(&ctx.first_zid, 0, sizeof(ctx.first_zid));
    ctx.count = 0;
    
    z_owned_closure_link_t callback;
    z_closure(&callback, capture_link_handler, NULL, &ctx);
    z_info_links(z_loan(s1), z_move(callback), NULL);

    if (ctx.count == 0) {
        printf("FAIL: No link found from session 1\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Print captured link info
    z_owned_string_t zid_str;
    z_id_to_string(&ctx.first_zid, &zid_str);
    printf("Session 1 link: zid=%.*s, count=%u\n",
           (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)),
           ctx.count);

    // Verify link connects to session 2
    z_id_t s2_zid = z_info_zid(z_loan(s2));
    if (memcmp(s2_zid.id, ctx.first_zid.id, sizeof(s2_zid.id)) == 0) {
        printf("PASS: Session 1's link connects to session 2 (ZIDs match)\n\n");
    } else {
        printf("FAIL: Captured link ZID doesn't match session 2's ZID\n");
        z_drop(z_move(zid_str));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    z_drop(z_move(zid_str));

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

    // First, clone a transport to use as filter
    transport_capture_t transport_ctx;
    z_internal_transport_null(&transport_ctx.transport);
    transport_ctx.captured = false;
    
    z_owned_closure_transport_t transport_callback;
    z_closure(&transport_callback, clone_transport_handler, NULL, &transport_ctx);
    z_info_transports(z_loan(s1), z_move(transport_callback));

    if (!transport_ctx.captured) {
        printf("FAIL: No transport captured for filtering\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Now filter links by the captured transport
    link_context_t link_ctx;
    memset(&link_ctx.first_zid, 0, sizeof(link_ctx.first_zid));
    link_ctx.count = 0;
    
    z_info_links_options_t options;
    z_info_links_options_default(&options);
    options.transport = z_transport_move(&transport_ctx.transport);
    
    z_owned_closure_link_t link_callback;
    z_closure(&link_callback, capture_link_handler, NULL, &link_ctx);
    z_info_links(z_loan(s1), z_move(link_callback), &options);

    if (link_ctx.count > 0) {
        z_owned_string_t zid_str;
        z_id_to_string(&link_ctx.first_zid, &zid_str);
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

    z_close(z_loan_mut(s1), NULL);
    z_drop(z_move(s1));
    z_close(z_loan_mut(s2), NULL);
    z_drop(z_move(s2));

    return 0;
}

int test_isolation() {
    printf("=== Testing session isolation (no external connections) ===\n");
    z_owned_session_t s1, s2;
    if (create_isolated_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    // Count transports from session 1 - should be exactly 1 (only s2)
    transport_context_t ctx1;
    memset(&ctx1.first_zid, 0, sizeof(ctx1.first_zid));
    ctx1.count = 0;
    
    z_owned_closure_transport_t callback;
    z_closure(&callback, capture_transport_handler, NULL, &ctx1);
    z_info_transports(z_loan(s1), z_move(callback));

    if (ctx1.count != 1) {
        printf("FAIL: Session 1 should have exactly 1 transport (to session 2), got %u\n", ctx1.count);
        printf("      This may indicate external zenoh sessions are connecting\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Count transports from session 2 - should be exactly 1 (only s1)
    transport_context_t ctx2;
    memset(&ctx2.first_zid, 0, sizeof(ctx2.first_zid));
    ctx2.count = 0;
    
    z_closure(&callback, capture_transport_handler, NULL, &ctx2);
    z_info_transports(z_loan(s2), z_move(callback));

    if (ctx2.count != 1) {
        printf("FAIL: Session 2 should have exactly 1 transport (to session 1), got %u\n", ctx2.count);
        printf("      This may indicate external zenoh sessions are connecting\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    printf("PASS: Both sessions have exactly 1 transport each (properly isolated)\n\n");

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
    // Test isolation first
    if (test_isolation() != 0) {
        return -1;
    }

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
