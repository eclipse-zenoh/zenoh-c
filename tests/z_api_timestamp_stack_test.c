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

#if defined(ZENOHC_BUILD_WITH_UNSTABLE_API) || defined(Z_FEATURE_UNSTABLE_API)

// ── shared state for subscriber callbacks ────────────────────────────────────
// All stack inspection is done inside the callback while the sample is alive.
// The context stores only the results of that inspection.

typedef struct {
    int received;
    int has_stack;           // 1 if sample carried a TimestampStack
    size_t record_count;
    int found_send;
    int found_receive;
    int as_timestamp_ok;     // 1 if at least one non-custom record parsed OK
} SampleCtx;

static void on_sample(z_loaned_sample_t *sample, void *arg) {
    SampleCtx *ctx = (SampleCtx *)arg;
    ctx->received = 1;

    const z_loaned_timestamp_stack_t *stack = z_sample_timestamp_stack(sample);
    ctx->has_stack = (stack != NULL);
    if (!stack) return;

    size_t n = z_timestamp_stack_record_count(stack);
    ctx->record_count = n;
    for (size_t i = 0; i < n; i++) {
        const z_loaned_timestamp_stack_record_t *rec = z_timestamp_stack_record_at(stack, i);
        if (!rec) continue;
        z_interception_point_t pt = z_timestamp_stack_record_point(rec);
        if (pt == Z_INTERCEPTION_POINT_SEND)    ctx->found_send = 1;
        if (pt == Z_INTERCEPTION_POINT_RECEIVE) ctx->found_receive = 1;
        if (!z_timestamp_stack_record_is_custom(rec)) {
            z_timestamp_t ts;
            if (z_timestamp_stack_record_as_timestamp(rec, &ts) == Z_OK)
                ctx->as_timestamp_ok = 1;
        }
    }
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

    SampleCtx ctx = {0};
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
    assert(ctx.has_stack == 0);

    z_undeclare_subscriber(z_subscriber_move(&sub));
    z_session_drop(z_session_move(&s));
}

// ── test_send_receive ─────────────────────────────────────────────────────────

void test_send_receive(void) {
    printf("test_send_receive\n");

    z_owned_session_t s = open_session();
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/ts/c/send_recv");

    SampleCtx ctx = {0};
    z_owned_closure_sample_t cb;
    z_closure_sample(&cb, on_sample, NULL, &ctx);
    z_owned_subscriber_t sub;
    assert(z_declare_subscriber(z_session_loan(&s), &sub, z_view_keyexpr_loan(&ke),
                                z_closure_sample_move(&cb), NULL) == Z_OK);

    z_sleep_ms(50);

    z_owned_timestamp_instrumentation_t instr;
    assert(z_timestamp_instrumentation_new(&instr, /*send=*/true, /*route=*/false, /*receive=*/true) == Z_OK);

    z_put_options_t opts; z_put_options_default(&opts);
    opts.timestamp_instrumentation = z_timestamp_instrumentation_loan(&instr);

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, "hello");
    assert(z_put(z_session_loan(&s), z_view_keyexpr_loan(&ke), z_bytes_move(&payload), &opts) == Z_OK);
    z_sleep_ms(200);

    assert(ctx.received == 1);
    assert(ctx.has_stack == 1);
    assert(ctx.record_count >= 2);
    assert(ctx.found_send);
    assert(ctx.found_receive);

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

    SampleCtx ctx = {0};
    z_owned_closure_sample_t cb;
    z_closure_sample(&cb, on_sample, NULL, &ctx);
    z_owned_subscriber_t sub;
    assert(z_declare_subscriber(z_session_loan(&s), &sub, z_view_keyexpr_loan(&ke),
                                z_closure_sample_move(&cb), NULL) == Z_OK);

    z_sleep_ms(50);

    z_owned_timestamp_instrumentation_t instr;
    assert(z_timestamp_instrumentation_new(&instr, true, false, true) == Z_OK);
    z_put_options_t opts; z_put_options_default(&opts);
    opts.timestamp_instrumentation = z_timestamp_instrumentation_loan(&instr);

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, "t");
    assert(z_put(z_session_loan(&s), z_view_keyexpr_loan(&ke), z_bytes_move(&payload), &opts) == Z_OK);
    z_sleep_ms(200);

    assert(ctx.received == 1);
    assert(ctx.has_stack == 1);
    assert(ctx.as_timestamp_ok == 1);

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

    SampleCtx ctx = {0};
    z_owned_closure_sample_t cb;
    z_closure_sample(&cb, on_sample, NULL, &ctx);
    z_owned_subscriber_t sub;
    assert(z_declare_subscriber(z_session_loan(&s), &sub, z_view_keyexpr_loan(&ke),
                                z_closure_sample_move(&cb), NULL) == Z_OK);

    z_owned_timestamp_instrumentation_t instr;
    assert(z_timestamp_instrumentation_new(&instr, true, false, true) == Z_OK);

    z_publisher_options_t pub_opts; z_publisher_options_default(&pub_opts);
    pub_opts.timestamp_instrumentation = z_timestamp_instrumentation_loan(&instr);

    z_owned_publisher_t pub;
    assert(z_declare_publisher(z_session_loan(&s), &pub, z_view_keyexpr_loan(&ke), &pub_opts) == Z_OK);

    z_sleep_ms(50);

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, "data");
    assert(z_publisher_put(z_publisher_loan(&pub), z_bytes_move(&payload), NULL) == Z_OK);
    z_sleep_ms(200);

    assert(ctx.received == 1);
    assert(ctx.has_stack == 1);
    assert(ctx.record_count >= 2);

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
