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
// Tests for the timestamp instrumentation API (unstable).
//

#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

#ifdef ZENOHC_BUILD_WITH_UNSTABLE_API

// ── shared state for subscriber callbacks ────────────────────────────────────

typedef struct {
    const z_loaned_timestamp_stack_t *stack;
    int received;
} SampleCtx;

static void on_sample(z_loaned_sample_t *sample, void *arg) {
    SampleCtx *ctx = (SampleCtx *)arg;
    ctx->stack = z_sample_timestamp_stack(sample);
    ctx->received = 1;
}

// ── helpers ───────────────────────────────────────────────────────────────────

static z_owned_session_t open_session(void) {
    z_owned_session_t s;
    z_owned_config_t cfg;
    z_config_default(&cfg);
    assert(z_open(&s, z_config_move(&cfg), NULL) == Z_OK);
    return s;
}

// ── test_no_instrumentation ───────────────────────────────────────────────────

void test_no_instrumentation(void) {
    printf("test_no_instrumentation\n");

    z_owned_session_t s = open_session();
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/ts/c/none");

    SampleCtx ctx = {NULL, 0};
    z_owned_closure_sample_t cb;
    z_closure_sample(&cb, on_sample, NULL, &ctx);
    z_owned_subscriber_t sub;
    assert(z_declare_subscriber(z_session_loan(&s), &sub, z_view_keyexpr_loan(&ke),
                                z_closure_sample_move(&cb), NULL) == Z_OK);

    z_sleep_ms(50);

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, "hello");
    assert(z_put(z_session_loan(&s), z_view_keyexpr_loan(&ke), z_bytes_move(&payload), NULL) == Z_OK);
    z_sleep_ms(200);

    assert(ctx.received == 1);
    assert(ctx.stack == NULL);

    z_undeclare_subscriber(z_subscriber_move(&sub));
    z_session_drop(z_session_move(&s));
}

// ── test_send_receive ─────────────────────────────────────────────────────────

void test_send_receive(void) {
    printf("test_send_receive\n");

    z_owned_session_t s = open_session();
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/ts/c/send_recv");

    SampleCtx ctx = {NULL, 0};
    z_owned_closure_sample_t cb;
    z_closure_sample(&cb, on_sample, NULL, &ctx);
    z_owned_subscriber_t sub;
    assert(z_declare_subscriber(z_session_loan(&s), &sub, z_view_keyexpr_loan(&ke),
                                z_closure_sample_move(&cb), NULL) == Z_OK);

    z_sleep_ms(50);

    z_owned_timestamp_instrumentation_t instr;
    assert(z_timestamp_instrumentation_new(&instr, /*send=*/true, /*route=*/false, /*receive=*/true) == Z_OK);

    z_put_options_t opts = z_put_options_default();
    opts.timestamp_instrumentation = z_timestamp_instrumentation_loan(&instr);

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, "hello");
    assert(z_put(z_session_loan(&s), z_view_keyexpr_loan(&ke), z_bytes_move(&payload), &opts) == Z_OK);
    z_sleep_ms(200);

    assert(ctx.received == 1);
    assert(ctx.stack != NULL);

    size_t n = z_timestamp_stack_record_count(ctx.stack);
    assert(n >= 2);

    int found_send = 0, found_receive = 0;
    for (size_t i = 0; i < n; i++) {
        const z_loaned_timestamp_stack_record_t *rec = z_timestamp_stack_record_at(ctx.stack, i);
        assert(rec != NULL);
        z_interception_point_t pt = z_timestamp_stack_record_point(rec);
        if (pt == Z_INTERCEPTION_POINT_SEND) found_send = 1;
        if (pt == Z_INTERCEPTION_POINT_RECEIVE) found_receive = 1;
    }
    assert(found_send);
    assert(found_receive);

    z_timestamp_instrumentation_drop(z_timestamp_instrumentation_move(&instr));
    z_undeclare_subscriber(z_subscriber_move(&sub));
    z_session_drop(z_session_move(&s));
}

// ── test_as_timestamp ─────────────────────────────────────────────────────────

void test_as_timestamp(void) {
    printf("test_as_timestamp\n");

    z_owned_session_t s = open_session();
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/ts/c/as_ts");

    SampleCtx ctx = {NULL, 0};
    z_owned_closure_sample_t cb;
    z_closure_sample(&cb, on_sample, NULL, &ctx);
    z_owned_subscriber_t sub;
    assert(z_declare_subscriber(z_session_loan(&s), &sub, z_view_keyexpr_loan(&ke),
                                z_closure_sample_move(&cb), NULL) == Z_OK);

    z_sleep_ms(50);

    z_owned_timestamp_instrumentation_t instr;
    assert(z_timestamp_instrumentation_new(&instr, true, false, true) == Z_OK);
    z_put_options_t opts = z_put_options_default();
    opts.timestamp_instrumentation = z_timestamp_instrumentation_loan(&instr);

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, "t");
    assert(z_put(z_session_loan(&s), z_view_keyexpr_loan(&ke), z_bytes_move(&payload), &opts) == Z_OK);
    z_sleep_ms(200);

    assert(ctx.received == 1);
    assert(ctx.stack != NULL);

    size_t n = z_timestamp_stack_record_count(ctx.stack);
    for (size_t i = 0; i < n; i++) {
        const z_loaned_timestamp_stack_record_t *rec = z_timestamp_stack_record_at(ctx.stack, i);
        if (!z_timestamp_stack_record_is_custom(rec)) {
            z_timestamp_t ts;
            assert(z_timestamp_stack_record_as_timestamp(rec, &ts) == Z_OK);
            (void)ts;  // copy type, no drop needed
        }
    }

    z_timestamp_instrumentation_drop(z_timestamp_instrumentation_move(&instr));
    z_undeclare_subscriber(z_subscriber_move(&sub));
    z_session_drop(z_session_move(&s));
}

// ── test_publisher_default ────────────────────────────────────────────────────

void test_publisher_default(void) {
    printf("test_publisher_default\n");

    z_owned_session_t s = open_session();
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/ts/c/pub_default");

    SampleCtx ctx = {NULL, 0};
    z_owned_closure_sample_t cb;
    z_closure_sample(&cb, on_sample, NULL, &ctx);
    z_owned_subscriber_t sub;
    assert(z_declare_subscriber(z_session_loan(&s), &sub, z_view_keyexpr_loan(&ke),
                                z_closure_sample_move(&cb), NULL) == Z_OK);

    z_owned_timestamp_instrumentation_t instr;
    assert(z_timestamp_instrumentation_new(&instr, true, false, true) == Z_OK);

    z_publisher_options_t pub_opts = z_publisher_options_default();
    pub_opts.timestamp_instrumentation = z_timestamp_instrumentation_loan(&instr);

    z_owned_publisher_t pub;
    assert(z_declare_publisher(z_session_loan(&s), &pub, z_view_keyexpr_loan(&ke), &pub_opts) == Z_OK);

    z_sleep_ms(50);

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, "data");
    assert(z_publisher_put(z_publisher_loan(&pub), z_bytes_move(&payload), NULL) == Z_OK);
    z_sleep_ms(200);

    assert(ctx.received == 1);
    assert(ctx.stack != NULL);
    assert(z_timestamp_stack_record_count(ctx.stack) >= 2);

    z_undeclare_publisher(z_publisher_move(&pub));
    z_timestamp_instrumentation_drop(z_timestamp_instrumentation_move(&instr));
    z_undeclare_subscriber(z_subscriber_move(&sub));
    z_session_drop(z_session_move(&s));
}

// ── test_invalid_instrumentation ─────────────────────────────────────────────

void test_invalid_instrumentation(void) {
    printf("test_invalid_instrumentation\n");
    z_owned_timestamp_instrumentation_t instr;
    int rc = z_timestamp_instrumentation_new(&instr, false, false, false);
    assert(rc != Z_OK);
}

// ── main ──────────────────────────────────────────────────────────────────────

int main(void) {
    test_no_instrumentation();
    test_send_receive();
    test_as_timestamp();
    test_publisher_default();
    test_invalid_instrumentation();
    printf("All timestamp stack tests passed.\n");
    return 0;
}

#else  // ZENOHC_BUILD_WITH_UNSTABLE_API not set

int main(void) {
    printf("Timestamp stack tests skipped (ZENOH_UNSTABLE not enabled).\n");
    return 0;
}

#endif
