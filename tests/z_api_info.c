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

#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "zenoh.h"

#if defined(Z_FEATURE_UNSTABLE_API)

#define MAX_CAPTURE 16

typedef struct context_t {
    int transport_count;
    int link_count;
    int transport_event_count;
    int link_event_count;
    z_owned_transport_t transports[MAX_CAPTURE];
    z_owned_link_t links[MAX_CAPTURE];
    z_owned_transport_event_t transport_events[MAX_CAPTURE];
    z_owned_link_event_t link_events[MAX_CAPTURE];
} context_t;

void init_context(context_t* ctx) {
    ctx->transport_count = 0;
    ctx->link_count = 0;
    ctx->transport_event_count = 0;
    ctx->link_event_count = 0;
    for (int i = 0; i < MAX_CAPTURE; i++) {
        z_internal_transport_null(&ctx->transports[i]);
        z_internal_link_null(&ctx->links[i]);
        z_internal_transport_event_null(&ctx->transport_events[i]);
        z_internal_link_event_null(&ctx->link_events[i]);
    }
}

void print_transport(const z_loaned_transport_t* transport) {
    z_id_t zid = z_transport_zid(transport);
    z_owned_string_t zid_str;
    z_id_to_string(&zid, &zid_str);
    printf("zid=%.*s\n", (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)));
    z_drop(z_move(zid_str));
}

void print_link(const z_loaned_link_t* link) {
    z_id_t zid = z_link_zid(link);
    z_owned_string_t zid_str;
    z_id_to_string(&zid, &zid_str);
    printf("zid=%.*s\n", (int)z_string_len(z_loan(zid_str)), z_string_data(z_loan(zid_str)));
    z_drop(z_move(zid_str));
}

void print_transport_event(const z_loaned_transport_event_t* event) {
    z_sample_kind_t kind = z_transport_event_kind(event);
    const char* kind_str = (kind == Z_SAMPLE_KIND_PUT) ? "PUT" : "DELETE";
    const z_loaned_transport_t* transport = z_transport_event_transport(event);
    printf("kind=%s, ", kind_str);
    print_transport(transport);
}

void print_link_event(const z_loaned_link_event_t* event) {
    z_sample_kind_t kind = z_link_event_kind(event);
    const char* kind_str = (kind == Z_SAMPLE_KIND_PUT) ? "PUT" : "DELETE";
    const z_loaned_link_t* link = z_link_event_link(event);
    printf("kind=%s, ", kind_str);
    print_link(link);
}

void print_context(context_t* ctx) {
    for (int i = 0; i < ctx->transport_count; i++) {
        printf("Transport %d: ", i);
        print_transport(z_loan(ctx->transports[i]));
    }
    for (int i = 0; i < ctx->link_count; i++) {
        printf("Link %d: ", i);
        print_link(z_loan(ctx->links[i]));
    }
    for (int i = 0; i < ctx->transport_event_count; i++) {
        printf("Transport Event %d: ", i);
        print_transport_event(z_loan(ctx->transport_events[i]));
    }
    for (int i = 0; i < ctx->link_event_count; i++) {
        printf("Link Event %d: ", i);
        print_link_event(z_loan(ctx->link_events[i]));
    }
}

void drop_context(context_t* ctx) {
    for (int i = 0; i < MAX_CAPTURE; i++) {
        z_drop(z_move(ctx->transports[i]));
        z_drop(z_move(ctx->links[i]));
        z_drop(z_move(ctx->transport_events[i]));
        z_drop(z_move(ctx->link_events[i]));
    }
}

// Callback that captures transports into an array
void capture_transports(z_loaned_transport_t* transport, void* arg) {
    context_t* ctx = (context_t*)arg;
    assert(ctx->transport_count < MAX_CAPTURE && "Exceeded maximum transport capture limit");
    z_take_from_loaned(&ctx->transports[ctx->transport_count], transport);
    ctx->transport_count++;
}

// Callback that captures links into an array
void capture_links(z_loaned_link_t* link, void* arg) {
    context_t* ctx = (context_t*)arg;
    assert(ctx->link_count < MAX_CAPTURE && "Exceeded maximum link capture limit");
    z_take_from_loaned(&ctx->links[ctx->link_count], link);
    ctx->link_count++;
}

// Callback that captures transport events into an array
void capture_transport_events(z_loaned_transport_event_t* event, void* arg) {
    context_t* ctx = (context_t*)arg;
    assert(ctx->transport_event_count < MAX_CAPTURE && "Exceeded maximum transport event capture limit");
    z_take_from_loaned(&ctx->transport_events[ctx->transport_event_count], event);
    ctx->transport_event_count++;
}

// Callback that captures link events into an array
void capture_link_events(z_loaned_link_event_t* event, void* arg) {
    context_t* ctx = (context_t*)arg;
    assert(ctx->link_event_count < MAX_CAPTURE && "Exceeded maximum link event capture limit");
    z_take_from_loaned(&ctx->link_events[ctx->link_event_count], event);
    ctx->link_event_count++;
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

// Helper to create an isolated session pair (router + peer) that won't connect to external zenoh nodes
// Session 1 (router): listens on a specific port
// Session 2 (peer): connects to session 1's port
// Both sessions have scouting disabled to prevent discovery of external nodes
void create_session_pair(z_owned_session_t* s1, z_owned_session_t* s2) {
    // Create config for router session: listener on localhost
    z_owned_config_t config1;
    create_isolated_config_with_mode(&config1, "\"router\"", "[\"tcp/127.0.0.1:17447\"]", "[]");

    z_result_t res = z_open(s1, z_move(config1), NULL);
    assert(res == 0);

    // Give router session time to start listening
    sleep(1);

    // Create config for peer session: connects to router
    z_owned_config_t config2;
    create_isolated_config_with_mode(&config2, "\"peer\"", "[]", "[\"tcp/127.0.0.1:17447\"]");

    res = z_open(s2, z_move(config2), NULL);
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
    create_session_pair(&router_session, &peer_session);

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

    // Cleanup
    z_drop(z_move(router_session));
    z_drop(z_move(peer_session));

    printf("\n");
}

#if defined(Z_FEATURE_UNSTABLE_API)

void test_z_info_transports_and_links() {
    printf("=== Testing z_info_transports and z_info_links ===\n");
    z_owned_session_t s1, s2;
    create_session_pair(&s1, &s2);

    // Capture transport from session 1
    context_t ctx;
    init_context(&ctx);

    z_owned_closure_transport_t capture_transport_callback;
    z_closure(&capture_transport_callback, capture_transports, NULL, &ctx);
    z_info_transports(z_loan(s1), z_move(capture_transport_callback));
    print_context(&ctx);

    // Verify that only one transport is captured
    assert(ctx.transport_count == 1 && "Expected exactly one transport captured from session 1");
    // Verify the captured transport matches session 2's ZID
    z_id_t captured_zid = z_transport_zid(z_loan(ctx.transports[0]));
    z_id_t s2_zid = z_info_zid(z_loan(s2));
    assert(memcmp(&captured_zid, &s2_zid, sizeof(z_id_t)) == 0 &&
           "Captured transport ZID doesn't match session 2's ZID");
    printf("PASS: Session 1's transport connects to session 2 (ZIDs match)\n\n");

    z_owned_closure_link_t capture_link_callback;
    z_closure(&capture_link_callback, capture_links, NULL, &ctx);
    z_info_links(z_loan(s1), z_move(capture_link_callback), NULL);
    print_context(&ctx);

    // Verify that only one link is captured
    assert(ctx.link_count == 1 && "Expected exactly one link captured from session 1");
    // Verify the captured link matches session 2's ZID
    z_id_t captured_link_zid = z_link_zid(z_loan(ctx.links[0]));
    assert(memcmp(&captured_link_zid, &s2_zid, sizeof(z_id_t)) == 0 &&
           "Captured link ZID doesn't match session 2's ZID");
    printf("PASS: Session 1's link connects to session 2 (ZIDs match)\n\n");

    // Cleanup
    drop_context(&ctx);
    z_drop(z_move(s1));
    z_drop(z_move(s2));
}

void test_z_info_links_filtered() {
    printf("=== Testing z_info_links with transport filter ===\n");
    z_owned_session_t s1, s2;
    create_session_pair(&s1, &s2);

    // Capture transports from s1 and s2
    context_t ctx_s1, ctx_s2;
    init_context(&ctx_s1);
    init_context(&ctx_s2);

    z_owned_closure_transport_t transport_callback;
    z_closure(&transport_callback, capture_transports, NULL, &ctx_s1);
    z_info_transports(z_loan(s1), z_move(transport_callback));

    assert(ctx_s1.transport_count == 1 && "Expected exactly one transport from session 1");

    z_closure(&transport_callback, capture_transports, NULL, &ctx_s2);
    z_info_transports(z_loan(s2), z_move(transport_callback));

    assert(ctx_s2.transport_count == 1 && "Expected exactly one transport from session 2");

    // Test 1: Filter links on s1 by s1's transport (should find links)
    context_t ctx_links;
    init_context(&ctx_links);

    z_info_links_options_t options;
    z_info_links_options_default(&options);
    options.transport = z_transport_move(&ctx_s1.transports[0]);

    z_owned_closure_link_t link_callback;
    z_closure(&link_callback, capture_links, NULL, &ctx_links);
    z_info_links(z_loan(s1), z_move(link_callback), &options);

    assert(ctx_links.link_count == 1 && "Expected exactly one link when filtering s1 by s1's transport");
    print_context(&ctx_links);
    printf("PASS: Filtered link by s1's transport\n");
    drop_context(&ctx_links);

    // Test 2: Filter links on s1 by s2's transport (should find no links)
    init_context(&ctx_links);

    z_info_links_options_default(&options);
    options.transport = z_transport_move(&ctx_s2.transports[0]);

    z_closure(&link_callback, capture_links, NULL, &ctx_links);
    z_info_links(z_loan(s1), z_move(link_callback), &options);
    print_context(&ctx_links);

    assert(ctx_links.link_count == 0 && "Expected no links when filtering s1 by s2's transport");
    printf("PASS: No links found when filtering s1 by s2's transport (as expected)\n\n");

    // Cleanup
    drop_context(&ctx_links);
    drop_context(&ctx_s1);
    drop_context(&ctx_s2);
    z_drop(z_move(s1));
    z_drop(z_move(s2));
}

void test_transport_events() {
    printf("=== Test: Transport events (no history) ===\n");

    // Session 1
    z_owned_session_t s1;
    z_owned_config_t cfg1;
    create_isolated_config(&cfg1, "[\"tcp/127.0.0.1:17448\"]", "[]");
    assert(z_open(&s1, z_move(cfg1), NULL) >= 0 && "Unable to open session 1");

    context_t ctx;
    init_context(&ctx);

    // Declare listener
    z_owned_transport_events_listener_t listener;
    z_owned_closure_transport_event_t callback;
    z_closure(&callback, capture_transport_events, NULL, &ctx);
    z_transport_events_listener_options_t opts;
    z_transport_events_listener_options_default(&opts);
    opts.history = false;

    assert(z_declare_transport_events_listener(z_loan(s1), &listener, z_move(callback), &opts) == 0 &&
           "Unable to declare transport events listener");

    // Should have no events yet (no history)
    assert(ctx.transport_event_count == 0 && "Expected 0 events before connection");

    // Session 2 connects
    z_owned_session_t s2;
    z_owned_config_t cfg2;
    create_isolated_config(&cfg2, "[]", "[\"tcp/127.0.0.1:17448\"]");
    assert(z_open(&s2, z_move(cfg2), NULL) >= 0 && "Unable to open session 2");

    sleep(2);

    // Should have 1 PUT event (transport added)
    assert(ctx.transport_event_count == 1 && "Expected 1 event after connection");
    assert(z_transport_event_kind(z_loan(ctx.transport_events[0])) == Z_SAMPLE_KIND_PUT &&
           "Expected PUT event for transport added");

    // Verify ZID from the transport in the event
    const z_loaned_transport_t* event_transport = z_transport_event_transport(z_loan(ctx.transport_events[0]));
    z_id_t event_zid = z_transport_zid(event_transport);
    z_id_t s2_zid = z_info_zid(z_loan(s2));
    assert(memcmp(&event_zid, &s2_zid, sizeof(z_id_t)) == 0 && "Transport ZID doesn't match session 2's ZID");
    print_context(&ctx);

    z_drop(z_move(s2));
    sleep(2);

    // Should have 2 events now (1 PUT + 1 DELETE)
    assert(ctx.transport_event_count == 2 && "Expected 2 events after disconnection");
    assert(z_transport_event_kind(z_loan(ctx.transport_events[1])) == Z_SAMPLE_KIND_DELETE &&
           "Expected DELETE event for transport removed");

    printf("PASS\n\n");

    // Cleanup
    z_undeclare_transport_events_listener(z_move(listener));
    drop_context(&ctx);
    z_drop(z_move(s1));
}

void test_transport_events_history() {
    printf("=== Test: Transport events with history ===\n");

    z_owned_session_t s1, s2;
    create_session_pair(&s1, &s2);

    context_t ctx;
    init_context(&ctx);

    z_owned_transport_events_listener_t listener;
    z_owned_closure_transport_event_t callback;
    z_closure(&callback, capture_transport_events, NULL, &ctx);
    z_transport_events_listener_options_t opts;
    z_transport_events_listener_options_default(&opts);
    opts.history = true;

    assert(z_declare_transport_events_listener(z_loan(s1), &listener, z_move(callback), &opts) == 0 &&
           "Unable to declare transport events listener");

    sleep(1);

    // Should have 1 PUT event from history (existing transport)
    assert(ctx.transport_event_count == 1 && "Expected 1 history event");
    assert(z_transport_event_kind(z_loan(ctx.transport_events[0])) == Z_SAMPLE_KIND_PUT &&
           "Expected PUT event for existing transport");
    print_context(&ctx);

    printf("PASS\n\n");

    // Cleanup
    z_undeclare_transport_events_listener(z_move(listener));
    drop_context(&ctx);
    z_drop(z_move(s1));
    z_drop(z_move(s2));
}

void test_transport_events_background() {
    printf("=== Test: Transport events (background) ===\n");

    z_owned_session_t s1;
    z_owned_config_t cfg1;
    create_isolated_config(&cfg1, "[\"tcp/127.0.0.1:17449\"]", "[]");
    assert(z_open(&s1, z_move(cfg1), NULL) >= 0 && "Unable to open session 1");

    context_t ctx;
    init_context(&ctx);

    z_owned_closure_transport_event_t callback;
    z_closure(&callback, capture_transport_events, NULL, &ctx);
    z_transport_events_listener_options_t opts;
    z_transport_events_listener_options_default(&opts);

    assert(z_declare_background_transport_events_listener(z_loan(s1), z_move(callback), &opts) == 0 &&
           "Unable to declare background transport events listener");

    z_owned_session_t s2;
    z_owned_config_t cfg2;
    create_isolated_config(&cfg2, "[]", "[\"tcp/127.0.0.1:17449\"]");
    assert(z_open(&s2, z_move(cfg2), NULL) >= 0 && "Unable to open session 2");

    sleep(2);

    // Should have 1 PUT event (transport added)
    assert(ctx.transport_event_count == 1 && "Expected 1 event after connection");
    assert(z_transport_event_kind(z_loan(ctx.transport_events[0])) == Z_SAMPLE_KIND_PUT &&
           "Expected PUT event for transport added");
    print_context(&ctx);

    printf("PASS\n\n");

    // Cleanup
    drop_context(&ctx);
    z_drop(z_move(s1));
    z_drop(z_move(s2));
}

void test_link_events() {
    printf("=== Test: Link events (no history) ===\n");

    // Session 1
    z_owned_session_t s1;
    z_owned_config_t cfg1;
    create_isolated_config(&cfg1, "[\"tcp/127.0.0.1:17450\"]", "[]");
    assert(z_open(&s1, z_move(cfg1), NULL) >= 0 && "Unable to open session 1");

    context_t ctx;
    init_context(&ctx);

    // Declare listener
    z_owned_link_events_listener_t listener;
    z_owned_closure_link_event_t callback;
    z_closure(&callback, capture_link_events, NULL, &ctx);
    z_link_events_listener_options_t opts;
    z_link_events_listener_options_default(&opts);
    opts.history = false;

    assert(z_declare_link_events_listener(z_loan(s1), &listener, z_move(callback), &opts) == 0 &&
           "Unable to declare link events listener");

    // Should have no events yet (no history)
    assert(ctx.link_event_count == 0 && "Expected 0 events before connection");

    // Session 2 connects
    z_owned_session_t s2;
    z_owned_config_t cfg2;
    create_isolated_config(&cfg2, "[]", "[\"tcp/127.0.0.1:17450\"]");
    assert(z_open(&s2, z_move(cfg2), NULL) >= 0 && "Unable to open session 2");

    sleep(2);

    // Should have 1 PUT event (link added)
    assert(ctx.link_event_count == 1 && "Expected 1 event after connection");
    assert(z_link_event_kind(z_loan(ctx.link_events[0])) == Z_SAMPLE_KIND_PUT && "Expected PUT event for link added");

    // Verify ZID from the link in the event
    const z_loaned_link_t* event_link = z_link_event_link(z_loan(ctx.link_events[0]));
    z_id_t link_zid = z_link_zid(event_link);
    z_id_t s2_zid = z_info_zid(z_loan(s2));
    assert(memcmp(&link_zid, &s2_zid, sizeof(z_id_t)) == 0 && "Link ZID doesn't match session 2's ZID");
    print_context(&ctx);

    z_drop(z_move(s2));
    sleep(2);

    // Should have 2 events now (1 PUT + 1 DELETE)
    assert(ctx.link_event_count == 2 && "Expected 2 events after disconnection");
    assert(z_link_event_kind(z_loan(ctx.link_events[1])) == Z_SAMPLE_KIND_DELETE &&
           "Expected DELETE event for link removed");

    printf("PASS\n\n");

    // Cleanup
    z_undeclare_link_events_listener(z_move(listener));
    drop_context(&ctx);
    z_drop(z_move(s1));
}

void test_link_events_history() {
    printf("=== Test: Link events with history ===\n");

    z_owned_session_t s1, s2;
    create_session_pair(&s1, &s2);

    context_t ctx;
    init_context(&ctx);

    z_owned_link_events_listener_t listener;
    z_owned_closure_link_event_t callback;
    z_closure(&callback, capture_link_events, NULL, &ctx);
    z_link_events_listener_options_t opts;
    z_link_events_listener_options_default(&opts);
    opts.history = true;

    assert(z_declare_link_events_listener(z_loan(s1), &listener, z_move(callback), &opts) == 0 &&
           "Unable to declare link events listener");

    sleep(1);

    // Should have 1 PUT event from history (existing link)
    assert(ctx.link_event_count == 1 && "Expected 1 history event");
    assert(z_link_event_kind(z_loan(ctx.link_events[0])) == Z_SAMPLE_KIND_PUT &&
           "Expected PUT event for existing link");
    print_context(&ctx);

    printf("PASS\n\n");

    // Cleanup
    z_undeclare_link_events_listener(z_move(listener));
    drop_context(&ctx);
    z_drop(z_move(s1));
    z_drop(z_move(s2));
}

void test_link_events_background() {
    printf("=== Test: Link events (background) ===\n");

    z_owned_session_t s1;
    z_owned_config_t cfg1;
    create_isolated_config(&cfg1, "[\"tcp/127.0.0.1:17451\"]", "[]");
    assert(z_open(&s1, z_move(cfg1), NULL) >= 0 && "Unable to open session 1");

    context_t ctx;
    init_context(&ctx);

    z_owned_closure_link_event_t callback;
    z_closure(&callback, capture_link_events, NULL, &ctx);
    z_link_events_listener_options_t opts;
    z_link_events_listener_options_default(&opts);

    assert(z_declare_background_link_events_listener(z_loan(s1), z_move(callback), &opts) == 0 &&
           "Unable to declare background link events listener");

    z_owned_session_t s2;
    z_owned_config_t cfg2;
    create_isolated_config(&cfg2, "[]", "[\"tcp/127.0.0.1:17451\"]");
    assert(z_open(&s2, z_move(cfg2), NULL) >= 0 && "Unable to open session 2");

    sleep(2);

    assert(ctx.link_event_count == 1 && "Expected 1 link event");
    assert(z_link_event_kind(z_loan(ctx.link_events[0])) == Z_SAMPLE_KIND_PUT && "Expected PUT event for link added");
    print_context(&ctx);

    printf("PASS\n\n");

    // Cleanup
    drop_context(&ctx);
    z_drop(z_move(s1));
    z_drop(z_move(s2));
}

void test_link_events_filtered() {
    printf("=== Test: Link events with transport filter ===\n");

    z_owned_session_t s1, s2;
    create_session_pair(&s1, &s2);

    // Capture transports from s1 and s2
    context_t ctx_s1, ctx_s2;
    init_context(&ctx_s1);
    init_context(&ctx_s2);

    z_owned_closure_transport_t transport_callback;
    z_closure(&transport_callback, capture_transports, NULL, &ctx_s1);
    z_info_transports(z_loan(s1), z_move(transport_callback));

    assert(ctx_s1.transport_count == 1 && "Expected exactly one transport from session 1");

    z_closure(&transport_callback, capture_transports, NULL, &ctx_s2);
    z_info_transports(z_loan(s2), z_move(transport_callback));

    assert(ctx_s2.transport_count == 1 && "Expected exactly one transport from session 2");

    // Test 1: Listen for link events on s1 filtered by s1's transport (should receive events)
    context_t ctx1;
    init_context(&ctx1);

    z_owned_link_events_listener_t listener1;
    z_owned_closure_link_event_t callback1;
    z_closure(&callback1, capture_link_events, NULL, &ctx1);
    z_link_events_listener_options_t opts1;
    z_link_events_listener_options_default(&opts1);
    opts1.history = true;
    opts1.transport = z_transport_move(&ctx_s1.transports[0]);

    assert(z_declare_link_events_listener(z_loan(s1), &listener1, z_move(callback1), &opts1) == 0 &&
           "Unable to declare filtered link events listener");

    sleep(1);

    assert(ctx1.link_event_count >= 1 && "Expected at least 1 event with matching transport filter");
    assert(z_link_event_kind(z_loan(ctx1.link_events[0])) == Z_SAMPLE_KIND_PUT &&
           "Expected PUT event for existing link");
    print_context(&ctx1);
    printf("PASS: Received %d event(s) with matching transport filter\n", ctx1.link_event_count);

    z_undeclare_link_events_listener(z_move(listener1));

    // Test 2: Listen for link events on s1 filtered by s2's transport (should receive no events)
    context_t ctx2;
    init_context(&ctx2);

    z_owned_link_events_listener_t listener2;
    z_owned_closure_link_event_t callback2;
    z_closure(&callback2, capture_link_events, NULL, &ctx2);
    z_link_events_listener_options_t opts2;
    z_link_events_listener_options_default(&opts2);
    opts2.history = true;
    opts2.transport = z_transport_move(&ctx_s2.transports[0]);

    assert(z_declare_link_events_listener(z_loan(s1), &listener2, z_move(callback2), &opts2) == 0 &&
           "Unable to declare filtered link events listener (test 2)");

    sleep(1);

    assert(ctx2.link_event_count == 0 && "Expected 0 events with non-matching transport filter");
    printf("PASS: Received 0 events with non-matching transport filter (as expected)\n\n");

    // Cleanup
    z_undeclare_link_events_listener(z_move(listener2));
    drop_context(&ctx_s1);
    drop_context(&ctx_s2);
    drop_context(&ctx1);
    drop_context(&ctx2);
    z_drop(z_move(s1));
    z_drop(z_move(s2));
}

#endif

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    // Test stable z_info functions (always run, not dependent on unstable API)
    test_z_info_stable();

#if defined(Z_FEATURE_UNSTABLE_API)
    // Test transports and links
    test_z_info_transports_and_links();

    // Test filtered links
    test_z_info_links_filtered();

    // Test transport events listener (no history)
    test_transport_events();

    // Test transport events listener (with history)
    test_transport_events_history();

    // Test transport events listener (background)
    test_transport_events_background();

    // Test link events listener (no history)
    test_link_events();

    // Test link events listener (with history)
    test_link_events_history();

    // Test link events listener (background)
    test_link_events_background();

    // Test link events listener with transport filter
    test_link_events_filtered();

    printf("\nAll tests completed successfully!\n");
#else
    printf("Skipping tests: Z_FEATURE_UNSTABLE_API not enabled\n");
#endif

    return 0;
}
