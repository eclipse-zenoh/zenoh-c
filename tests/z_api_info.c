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

#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

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

// Helper to create an isolated session configuration
// This prevents the session from discovering or connecting to external zenoh nodes
void create_isolated_config_with_mode(z_owned_config_t* config, const char* mode, const char* listen_endpoints,
                                      const char* connect_endpoints) {
    z_config_default(config);

    // Set mode
    zc_config_insert_json5(z_loan_mut(*config), "mode", mode);

    // Disable multicast scouting
    zc_config_insert_json5(z_loan_mut(*config), "scouting/multicast/enabled", "false");

    // Disable gossip scouting
    zc_config_insert_json5(z_loan_mut(*config), "scouting/gossip/enabled", "false");

    // Configure listen endpoints
    zc_config_insert_json5(z_loan_mut(*config), "listen/endpoints", listen_endpoints);

    // Configure connect endpoints
    zc_config_insert_json5(z_loan_mut(*config), "connect/endpoints", connect_endpoints);
}

// Helper to create an isolated session configuration in peer mode
void create_isolated_config(z_owned_config_t* config, const char* listen_endpoints, const char* connect_endpoints) {
    create_isolated_config_with_mode(config, "\"peer\"", listen_endpoints, connect_endpoints);
}

// Helper to configure an isolated session pair that won't connect to external zenoh nodes
// Session 1: listens on a specific port
// Session 2: connects to session 1's port
// Both sessions have scouting disabled to prevent discovery of external nodes
int create_isolated_session_pair(z_owned_session_t* s1, z_owned_session_t* s2) {
    // Create config for session 1: listener on localhost
    z_owned_config_t config1;
    create_isolated_config(&config1, "[\"tcp/127.0.0.1:17447\"]", "[]");

    if (z_open(s1, z_move(config1), NULL) < 0) {
        printf("Unable to open session 1!\n");
        return -1;
    }

    // Give session 1 time to start listening
    sleep(1);

    // Create config for session 2: connects to session 1
    z_owned_config_t config2;
    create_isolated_config(&config2, "[]", "[\"tcp/127.0.0.1:17447\"]");

    if (z_open(s2, z_move(config2), NULL) < 0) {
        printf("Unable to open session 2!\n");
        z_drop(z_move(*s1));
        return -1;
    }

    // Sleep to allow sessions to establish transports
    sleep(1);

    return 0;
}

// Helper to create an isolated session pair with first session in router mode, second in peer mode
// Session 1 (router): listens on a specific port
// Session 2 (peer): connects to session 1's port
void create_router_peer_session_pair(z_owned_session_t* router_session, z_owned_session_t* peer_session) {
    // Create config for router session: listener on localhost
    z_owned_config_t config_router;
    create_isolated_config_with_mode(&config_router, "\"router\"", "[\"tcp/127.0.0.1:17452\"]", "[]");

    z_result_t res = z_open(router_session, z_move(config_router), NULL);
    assert(res == 0);

    // Give router session time to start listening
    sleep(1);

    // Create config for peer session: connects to router
    z_owned_config_t config_peer;
    create_isolated_config_with_mode(&config_peer, "\"peer\"", "[]", "[\"tcp/127.0.0.1:17452\"]");

    res = z_open(peer_session, z_move(config_peer), NULL);
    assert(res == 0);

    // Sleep to allow sessions to establish connection
    sleep(1);
}

// Context for counting ZIDs and storing the first one
typedef struct {
    int count;
    z_id_t first_zid;
} zid_collect_ctx_t;

// Callback for collecting ZIDs
void zid_collect_handler(const z_id_t* zid, void* arg) {
    zid_collect_ctx_t* ctx = (zid_collect_ctx_t*)arg;
    if (ctx->count == 0) {
        ctx->first_zid = *zid;
    }
    ctx->count++;
}

// Test for stable z_info functions (z_info_zid, z_info_routers_zid, z_info_peers_zid)
void test_z_info_stable() {
    printf("=== Testing stable z_info functions ===\n");

    z_owned_session_t router_session, peer_session;
    create_router_peer_session_pair(&router_session, &peer_session);

    // Get ZIDs of both sessions
    z_id_t router_zid = z_info_zid(z_loan(router_session));
    z_id_t peer_zid = z_info_zid(z_loan(peer_session));

    // Verify ZIDs are different
    assert(memcmp(router_zid.id, peer_zid.id, sizeof(router_zid.id)) != 0);
    printf("PASS: Router and peer sessions have different ZIDs\n");

    // Test router session: should have empty routers list
    zid_collect_ctx_t router_routers_ctx = {0};
    z_owned_closure_zid_t callback;
    z_closure(&callback, zid_collect_handler, NULL, &router_routers_ctx);
    z_info_routers_zid(z_loan(router_session), z_move(callback));
    assert(router_routers_ctx.count == 0);
    printf("PASS: Router session has empty routers list\n");

    // Test router session: should have peers list with peer's ZID
    zid_collect_ctx_t router_peers_ctx = {0};
    z_closure(&callback, zid_collect_handler, NULL, &router_peers_ctx);
    z_info_peers_zid(z_loan(router_session), z_move(callback));
    assert(router_peers_ctx.count == 1);
    assert(memcmp(router_peers_ctx.first_zid.id, peer_zid.id, sizeof(peer_zid.id)) == 0);
    printf("PASS: Router session has peers list with peer's ZID\n");

    // Test peer session: should have routers list with router's ZID
    zid_collect_ctx_t peer_routers_ctx = {0};
    z_closure(&callback, zid_collect_handler, NULL, &peer_routers_ctx);
    z_info_routers_zid(z_loan(peer_session), z_move(callback));
    assert(peer_routers_ctx.count == 1);
    assert(memcmp(peer_routers_ctx.first_zid.id, router_zid.id, sizeof(router_zid.id)) == 0);
    printf("PASS: Peer session has routers list with router's ZID\n");

    // Test peer session: should have empty peers list
    zid_collect_ctx_t peer_peers_ctx = {0};
    z_closure(&callback, zid_collect_handler, NULL, &peer_peers_ctx);
    z_info_peers_zid(z_loan(peer_session), z_move(callback));
    assert(peer_peers_ctx.count == 0);
    printf("PASS: Peer session has empty peers list\n");

    printf("\n");
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
    printf("Session 1 transport: zid=%.*s\n", (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)));
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
    z_drop(z_move(s1));

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
    printf("Session 1 link: zid=%.*s\n", (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)));

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

    z_drop(z_move(s1));
    z_drop(z_move(s2));

    return 0;
}

int test_z_info_links_filtered() {
    printf("=== Testing z_info_links with transport filter ===\n");
    z_owned_session_t s1, s2;
    if (create_isolated_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    // Capture transport from s1 (transport to s2)
    z_owned_transport_t transport_s1;
    z_internal_transport_null(&transport_s1);

    z_owned_closure_transport_t transport_callback;
    z_closure(&transport_callback, capture_transport_handler, NULL, &transport_s1);
    z_info_transports(z_loan(s1), z_move(transport_callback));

    if (!z_internal_transport_check(&transport_s1)) {
        printf("FAIL: No transport captured from session 1\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Capture transport from s2 (transport to s1)
    z_owned_transport_t transport_s2;
    z_internal_transport_null(&transport_s2);

    z_closure(&transport_callback, capture_transport_handler, NULL, &transport_s2);
    z_info_transports(z_loan(s2), z_move(transport_callback));

    if (!z_internal_transport_check(&transport_s2)) {
        printf("FAIL: No transport captured from session 2\n");
        z_drop(z_move(transport_s1));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Test 1: Filter links on s1 by s1's transport (should find links)
    z_owned_link_t link;
    z_internal_link_null(&link);

    z_info_links_options_t options;
    z_info_links_options_default(&options);
    options.transport = z_transport_move(&transport_s1);

    z_owned_closure_link_t link_callback;
    z_closure(&link_callback, capture_link_handler, NULL, &link);
    z_info_links(z_loan(s1), z_move(link_callback), &options);

    if (z_internal_link_check(&link)) {
        z_id_t link_zid = z_link_zid(z_loan(link));
        z_owned_string_t zid_str;
        z_id_to_string(&link_zid, &zid_str);
        printf("PASS: Filtered link by s1's transport: zid=%.*s\n", (int)z_string_len(z_loan(zid_str)),
               z_string_data(z_loan(zid_str)));
        z_drop(z_move(zid_str));
        z_drop(z_move(link));
    } else {
        printf("FAIL: No link found when filtering s1 by s1's transport\n");
        z_drop(z_move(transport_s2));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Test 2: Filter links on s1 by s2's transport (should find no links)
    z_internal_link_null(&link);

    z_info_links_options_default(&options);
    options.transport = z_transport_move(&transport_s2);

    z_closure(&link_callback, capture_link_handler, NULL, &link);
    z_info_links(z_loan(s1), z_move(link_callback), &options);

    if (!z_internal_link_check(&link)) {
        printf("PASS: No links found when filtering s1 by s2's transport (as expected)\n\n");
    } else {
        printf("FAIL: Found link when filtering s1 by s2's transport (should be none)\n");
        z_drop(z_move(link));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    z_drop(z_move(s1));
    z_drop(z_move(s2));

    return 0;
}

// ========================================
// Transport Events Listener Tests
// ========================================

// Context for transport event handler
typedef struct {
    int added_count;
    int removed_count;
    z_owned_transport_t last_transport;
} transport_event_ctx_t;

// Handler for transport events
// NOTE: Caller must drop and nullify ctx->last_transport before expecting a new PUT event
void transport_event_handler(z_loaned_transport_event_t* event, void* arg) {
    transport_event_ctx_t* ctx = (transport_event_ctx_t*)arg;
    z_sample_kind_t kind = z_transport_event_kind(event);
    z_loaned_transport_t* transport = z_transport_event_transport_mut(event);

    if (kind == Z_SAMPLE_KIND_PUT) {
        // Verify last_transport is null before taking a new one
        assert(!z_internal_transport_check(&ctx->last_transport) &&
               "last_transport must be nullified before receiving new event");
        ctx->added_count++;
        z_transport_take_from_loaned(&ctx->last_transport, transport);
    } else {
        ctx->removed_count++;
    }
}

int test_transport_events_sync() {
    printf("=== Test: Transport events (sync, no history) ===\n");

    // Session 1
    z_owned_session_t s1;
    z_owned_config_t cfg1;
    create_isolated_config(&cfg1, "[\"tcp/127.0.0.1:17448\"]", "[]");
    if (z_open(&s1, z_move(cfg1), NULL) < 0) {
        printf("FAIL: Unable to open session 1\n");
        return -1;
    }

    transport_event_ctx_t ctx = {0};
    z_internal_transport_null(&ctx.last_transport);

    // Declare listener
    z_owned_transport_events_listener_t listener;
    z_owned_closure_transport_event_t callback;
    z_closure(&callback, transport_event_handler, NULL, &ctx);
    z_transport_events_listener_options_t opts;
    z_transport_events_listener_options_default(&opts);
    opts.history = false;

    if (z_declare_transport_events_listener(z_loan(s1), &listener, z_move(callback), &opts) != 0) {
        printf("FAIL: Unable to declare transport events listener\n");
        z_drop(z_move(s1));
        return -1;
    }

    // Should have no events yet (no history)
    if (ctx.added_count != 0) {
        printf("FAIL: Expected 0 events before connection, got %d\n", ctx.added_count);
        z_undeclare_transport_events_listener(z_move(listener));
        z_drop(z_move(s1));
        return -1;
    }

    // Session 2 connects
    z_owned_session_t s2;
    z_owned_config_t cfg2;
    create_isolated_config(&cfg2, "[]", "[\"tcp/127.0.0.1:17448\"]");
    if (z_open(&s2, z_move(cfg2), NULL) < 0) {
        printf("FAIL: Unable to open session 2\n");
        z_undeclare_transport_events_listener(z_move(listener));
        z_drop(z_move(s1));
        return -1;
    }

    sleep(2);

    if (ctx.added_count != 1) {
        printf("FAIL: Expected 1 added event, got %d\n", ctx.added_count);
        z_drop(z_move(ctx.last_transport));
        z_undeclare_transport_events_listener(z_move(listener));
        z_drop(z_move(s2));
        z_drop(z_move(s1));
        return -1;
    }

    // Verify ZID
    z_id_t event_zid = z_transport_zid(z_loan(ctx.last_transport));
    z_id_t s2_zid = z_info_zid(z_loan(s2));
    if (memcmp(event_zid.id, s2_zid.id, sizeof(s2_zid.id)) != 0) {
        printf("FAIL: Transport ZID doesn't match session 2's ZID\n");
        z_drop(z_move(ctx.last_transport));
        z_undeclare_transport_events_listener(z_move(listener));
        z_drop(z_move(s2));
        z_drop(z_move(s1));
        return -1;
    }

    z_drop(z_move(ctx.last_transport));
    z_drop(z_move(s2));
    sleep(1);

    if (ctx.removed_count != 1) {
        printf("FAIL: Expected 1 removed event, got %d\n", ctx.removed_count);
        z_undeclare_transport_events_listener(z_move(listener));
        z_drop(z_move(s1));
        return -1;
    }

    printf("PASS\n\n");
    z_undeclare_transport_events_listener(z_move(listener));
    z_drop(z_move(s1));
    return 0;
}

int test_transport_events_history() {
    printf("=== Test: Transport events with history ===\n");

    z_owned_session_t s1, s2;
    if (create_isolated_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    transport_event_ctx_t ctx = {0};
    z_internal_transport_null(&ctx.last_transport);

    z_owned_transport_events_listener_t listener;
    z_owned_closure_transport_event_t callback;
    z_closure(&callback, transport_event_handler, NULL, &ctx);
    z_transport_events_listener_options_t opts;
    z_transport_events_listener_options_default(&opts);
    opts.history = true;

    if (z_declare_transport_events_listener(z_loan(s1), &listener, z_move(callback), &opts) != 0) {
        printf("FAIL: Unable to declare transport events listener\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    sleep(1);

    if (ctx.added_count != 1) {
        printf("FAIL: Expected 1 history event, got %d\n", ctx.added_count);
        z_drop(z_move(ctx.last_transport));
        z_undeclare_transport_events_listener(z_move(listener));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    printf("PASS\n\n");
    z_drop(z_move(ctx.last_transport));
    z_undeclare_transport_events_listener(z_move(listener));
    z_drop(z_move(s1));
    z_drop(z_move(s2));
    return 0;
}

int test_transport_events_background() {
    printf("=== Test: Transport events (background) ===\n");

    z_owned_session_t s1;
    z_owned_config_t cfg1;
    create_isolated_config(&cfg1, "[\"tcp/127.0.0.1:17449\"]", "[]");
    if (z_open(&s1, z_move(cfg1), NULL) < 0) {
        printf("FAIL: Unable to open session 1\n");
        return -1;
    }

    transport_event_ctx_t ctx = {0};
    z_internal_transport_null(&ctx.last_transport);

    z_owned_closure_transport_event_t callback;
    z_closure(&callback, transport_event_handler, NULL, &ctx);
    z_transport_events_listener_options_t opts;
    z_transport_events_listener_options_default(&opts);

    if (z_declare_background_transport_events_listener(z_loan(s1), z_move(callback), &opts) != 0) {
        printf("FAIL: Unable to declare background transport events listener\n");
        z_drop(z_move(s1));
        return -1;
    }

    z_owned_session_t s2;
    z_owned_config_t cfg2;
    create_isolated_config(&cfg2, "[]", "[\"tcp/127.0.0.1:17449\"]");
    if (z_open(&s2, z_move(cfg2), NULL) < 0) {
        printf("FAIL: Unable to open session 2\n");
        z_drop(z_move(s1));
        return -1;
    }

    sleep(2);

    if (ctx.added_count != 1) {
        printf("FAIL: Expected 1 added event, got %d\n", ctx.added_count);
        z_drop(z_move(ctx.last_transport));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    printf("PASS\n\n");
    z_drop(z_move(ctx.last_transport));
    z_drop(z_move(s1));
    z_drop(z_move(s2));
    return 0;
}

// ========================================
// Link Events Listener Tests
// ========================================

// Context for link event handler
typedef struct {
    int added_count;
    int removed_count;
    z_owned_link_t last_link;
} link_event_ctx_t;

// Handler for link events
// NOTE: Caller must drop and nullify ctx->last_link before expecting a new PUT event
void link_event_handler(z_loaned_link_event_t* event, void* arg) {
    link_event_ctx_t* ctx = (link_event_ctx_t*)arg;
    z_sample_kind_t kind = z_link_event_kind(event);
    z_loaned_link_t* link = z_link_event_link_mut(event);

    if (kind == Z_SAMPLE_KIND_PUT) {
        // Verify last_link is null before taking a new one
        assert(!z_internal_link_check(&ctx->last_link) && "last_link must be nullified before receiving new event");
        ctx->added_count++;
        z_link_take_from_loaned(&ctx->last_link, link);
    } else {
        ctx->removed_count++;
    }
}

int test_link_events_sync() {
    printf("=== Test: Link events (sync, no history) ===\n");

    // Session 1
    z_owned_session_t s1;
    z_owned_config_t cfg1;
    create_isolated_config(&cfg1, "[\"tcp/127.0.0.1:17450\"]", "[]");
    if (z_open(&s1, z_move(cfg1), NULL) < 0) {
        printf("FAIL: Unable to open session 1\n");
        return -1;
    }

    link_event_ctx_t ctx = {0};
    z_internal_link_null(&ctx.last_link);

    // Declare listener
    z_owned_link_events_listener_t listener;
    z_owned_closure_link_event_t callback;
    z_closure(&callback, link_event_handler, NULL, &ctx);
    z_link_events_listener_options_t opts;
    z_link_events_listener_options_default(&opts);
    opts.history = false;

    if (z_declare_link_events_listener(z_loan(s1), &listener, z_move(callback), &opts) != 0) {
        printf("FAIL: Unable to declare link events listener\n");
        z_drop(z_move(s1));
        return -1;
    }

    // Should have no events yet (no history)
    if (ctx.added_count != 0) {
        printf("FAIL: Expected 0 events before connection, got %d\n", ctx.added_count);
        z_undeclare_link_events_listener(z_move(listener));
        z_drop(z_move(s1));
        return -1;
    }

    // Session 2 connects
    z_owned_session_t s2;
    z_owned_config_t cfg2;
    create_isolated_config(&cfg2, "[]", "[\"tcp/127.0.0.1:17450\"]");
    if (z_open(&s2, z_move(cfg2), NULL) < 0) {
        printf("FAIL: Unable to open session 2\n");
        z_undeclare_link_events_listener(z_move(listener));
        z_drop(z_move(s1));
        return -1;
    }

    sleep(2);

    if (ctx.added_count != 1) {
        printf("FAIL: Expected 1 added event, got %d\n", ctx.added_count);
        z_drop(z_move(ctx.last_link));
        z_undeclare_link_events_listener(z_move(listener));
        z_drop(z_move(s2));
        z_drop(z_move(s1));
        return -1;
    }

    // Verify ZID from link matches session 2's ZID
    z_id_t link_zid = z_link_zid(z_loan(ctx.last_link));
    z_id_t s2_zid = z_info_zid(z_loan(s2));
    if (memcmp(link_zid.id, s2_zid.id, sizeof(s2_zid.id)) != 0) {
        printf("FAIL: Link ZID doesn't match session 2's ZID\n");
        z_drop(z_move(ctx.last_link));
        z_undeclare_link_events_listener(z_move(listener));
        z_drop(z_move(s2));
        z_drop(z_move(s1));
        return -1;
    }

    z_drop(z_move(ctx.last_link));
    z_drop(z_move(s2));
    sleep(1);

    // Note: A transport may have multiple links (e.g., bidirectional),
    // so we check for at least 1 removed event
    if (ctx.removed_count < 1) {
        printf("FAIL: Expected at least 1 removed event, got %d\n", ctx.removed_count);
        z_undeclare_link_events_listener(z_move(listener));
        z_drop(z_move(s1));
        return -1;
    }

    printf("PASS (received %d removed events)\n\n", ctx.removed_count);
    z_undeclare_link_events_listener(z_move(listener));
    z_drop(z_move(s1));
    return 0;
}

int test_link_events_history() {
    printf("=== Test: Link events with history ===\n");

    z_owned_session_t s1, s2;
    if (create_isolated_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    link_event_ctx_t ctx = {0};
    z_internal_link_null(&ctx.last_link);

    z_owned_link_events_listener_t listener;
    z_owned_closure_link_event_t callback;
    z_closure(&callback, link_event_handler, NULL, &ctx);
    z_link_events_listener_options_t opts;
    z_link_events_listener_options_default(&opts);
    opts.history = true;

    if (z_declare_link_events_listener(z_loan(s1), &listener, z_move(callback), &opts) != 0) {
        printf("FAIL: Unable to declare link events listener\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    sleep(1);

    if (ctx.added_count != 1) {
        printf("FAIL: Expected 1 history event, got %d\n", ctx.added_count);
        z_drop(z_move(ctx.last_link));
        z_undeclare_link_events_listener(z_move(listener));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    printf("PASS\n\n");
    z_drop(z_move(ctx.last_link));
    z_undeclare_link_events_listener(z_move(listener));
    z_drop(z_move(s1));
    z_drop(z_move(s2));
    return 0;
}

int test_link_events_background() {
    printf("=== Test: Link events (background) ===\n");

    z_owned_session_t s1;
    z_owned_config_t cfg1;
    create_isolated_config(&cfg1, "[\"tcp/127.0.0.1:17451\"]", "[]");
    if (z_open(&s1, z_move(cfg1), NULL) < 0) {
        printf("FAIL: Unable to open session 1\n");
        return -1;
    }

    link_event_ctx_t ctx = {0};
    z_internal_link_null(&ctx.last_link);

    z_owned_closure_link_event_t callback;
    z_closure(&callback, link_event_handler, NULL, &ctx);
    z_link_events_listener_options_t opts;
    z_link_events_listener_options_default(&opts);

    if (z_declare_background_link_events_listener(z_loan(s1), z_move(callback), &opts) != 0) {
        printf("FAIL: Unable to declare background link events listener\n");
        z_drop(z_move(s1));
        return -1;
    }

    z_owned_session_t s2;
    z_owned_config_t cfg2;
    create_isolated_config(&cfg2, "[]", "[\"tcp/127.0.0.1:17451\"]");
    if (z_open(&s2, z_move(cfg2), NULL) < 0) {
        printf("FAIL: Unable to open session 2\n");
        z_drop(z_move(s1));
        return -1;
    }

    sleep(2);

    if (ctx.added_count != 1) {
        printf("FAIL: Expected 1 added event, got %d\n", ctx.added_count);
        z_drop(z_move(ctx.last_link));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    printf("PASS\n\n");
    z_drop(z_move(ctx.last_link));
    z_drop(z_move(s1));
    z_drop(z_move(s2));
    return 0;
}

int test_link_events_filtered() {
    printf("=== Test: Link events with transport filter ===\n");

    z_owned_session_t s1, s2;
    if (create_isolated_session_pair(&s1, &s2) != 0) {
        return -1;
    }

    // Capture transport from s1 (transport to s2)
    z_owned_transport_t transport_s1;
    z_internal_transport_null(&transport_s1);

    z_owned_closure_transport_t transport_callback;
    z_closure(&transport_callback, capture_transport_handler, NULL, &transport_s1);
    z_info_transports(z_loan(s1), z_move(transport_callback));

    if (!z_internal_transport_check(&transport_s1)) {
        printf("FAIL: No transport captured from session 1\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Capture transport from s2 (transport to s1)
    z_owned_transport_t transport_s2;
    z_internal_transport_null(&transport_s2);

    z_closure(&transport_callback, capture_transport_handler, NULL, &transport_s2);
    z_info_transports(z_loan(s2), z_move(transport_callback));

    if (!z_internal_transport_check(&transport_s2)) {
        printf("FAIL: No transport captured from session 2\n");
        z_drop(z_move(transport_s1));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    // Test 1: Listen for link events on s1 filtered by s1's transport (should receive events)
    link_event_ctx_t ctx1 = {0};
    z_internal_link_null(&ctx1.last_link);

    z_owned_link_events_listener_t listener1;
    z_owned_closure_link_event_t callback1;
    z_closure(&callback1, link_event_handler, NULL, &ctx1);
    z_link_events_listener_options_t opts1;
    z_link_events_listener_options_default(&opts1);
    opts1.history = true;
    opts1.transport = z_transport_move(&transport_s1);

    if (z_declare_link_events_listener(z_loan(s1), &listener1, z_move(callback1), &opts1) != 0) {
        printf("FAIL: Unable to declare filtered link events listener\n");
        z_drop(z_move(transport_s2));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    sleep(1);

    if (ctx1.added_count < 1) {
        printf("FAIL: Expected at least 1 event with matching transport filter, got %d\n", ctx1.added_count);
        z_drop(z_move(ctx1.last_link));
        z_undeclare_link_events_listener(z_move(listener1));
        z_drop(z_move(transport_s2));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }
    printf("PASS: Received %d event(s) with matching transport filter\n", ctx1.added_count);

    z_drop(z_move(ctx1.last_link));
    z_undeclare_link_events_listener(z_move(listener1));

    // Test 2: Listen for link events on s1 filtered by s2's transport (should receive no events)
    link_event_ctx_t ctx2 = {0};
    z_internal_link_null(&ctx2.last_link);

    z_owned_link_events_listener_t listener2;
    z_owned_closure_link_event_t callback2;
    z_closure(&callback2, link_event_handler, NULL, &ctx2);
    z_link_events_listener_options_t opts2;
    z_link_events_listener_options_default(&opts2);
    opts2.history = true;
    opts2.transport = z_transport_move(&transport_s2);

    if (z_declare_link_events_listener(z_loan(s1), &listener2, z_move(callback2), &opts2) != 0) {
        printf("FAIL: Unable to declare filtered link events listener (test 2)\n");
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }

    sleep(1);

    if (ctx2.added_count != 0) {
        printf("FAIL: Expected 0 events with non-matching transport filter, got %d\n", ctx2.added_count);
        z_drop(z_move(ctx2.last_link));
        z_undeclare_link_events_listener(z_move(listener2));
        z_drop(z_move(s1));
        z_drop(z_move(s2));
        return -1;
    }
    printf("PASS: Received 0 events with non-matching transport filter (as expected)\n\n");

    z_undeclare_link_events_listener(z_move(listener2));
    z_drop(z_move(s1));
    z_drop(z_move(s2));
    return 0;
}

#endif

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    // Test stable z_info functions (always run, not dependent on unstable API)
    test_z_info_stable();

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

    // Test transport events listener (sync, no history)
    if (test_transport_events_sync() != 0) {
        return -1;
    }

    // Test transport events listener (with history)
    if (test_transport_events_history() != 0) {
        return -1;
    }

    // Test transport events listener (background)
    if (test_transport_events_background() != 0) {
        return -1;
    }

    // Test link events listener (sync, no history)
    if (test_link_events_sync() != 0) {
        return -1;
    }

    // Test link events listener (with history)
    if (test_link_events_history() != 0) {
        return -1;
    }

    // Test link events listener (background)
    if (test_link_events_background() != 0) {
        return -1;
    }

    // Test link events listener with transport filter
    if (test_link_events_filtered() != 0) {
        return -1;
    }

    printf("\nAll tests completed successfully!\n");
#else
    printf("Skipping tests: Z_FEATURE_UNSTABLE_API not enabled\n");
#endif

    return 0;
}
