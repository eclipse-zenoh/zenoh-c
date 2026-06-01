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
// Demonstrates opt-in end-to-end latency instrumentation.
//
// Build with ZENOH_UNSTABLE=ON:
//   cmake -B build -DZENOH_UNSTABLE=ON -G Ninja && cmake --build build -j4
// Run:
//   ./build/examples/z_timestamp_instrumentation
//

#include <stdio.h>
#include <string.h>
#include <time.h>

#include "zenoh.h"

#ifdef ZENOHC_BUILD_WITH_UNSTABLE_API

static void print_stack(const z_loaned_timestamp_stack_t *stack) {
    if (stack == NULL) {
        printf("  (no timestamp stack)\n");
        return;
    }
    size_t n = z_timestamp_stack_record_count(stack);
    for (size_t i = 0; i < n; i++) {
        const z_loaned_timestamp_stack_record_t *rec = z_timestamp_stack_record_at(stack, i);
        z_interception_point_t pt = z_timestamp_stack_record_point(rec);
        const char *pt_name =
            pt == Z_INTERCEPTION_POINT_SEND    ? "SEND   " :
            pt == Z_INTERCEPTION_POINT_ROUTE   ? "ROUTE  " :
            pt == Z_INTERCEPTION_POINT_RECEIVE ? "RECEIVE" : "UNKNOWN";

        size_t ts_len = 0;
        const uint8_t *ts_bytes = z_timestamp_stack_record_timestamp(rec, &ts_len);
        printf("  %s  custom=%-5s  ts_bytes=%zu", pt_name,
               z_timestamp_stack_record_is_custom(rec) ? "true" : "false", ts_len);

        if (!z_timestamp_stack_record_is_custom(rec)) {
            z_timestamp_t ts;
            if (z_timestamp_stack_record_as_timestamp(rec, &ts) == Z_OK) {
                printf("  ntp64=%llu", (unsigned long long)z_timestamp_ntp64_time(&ts));
            }
        }
        printf("\n");
        (void)ts_bytes;
    }
}

typedef struct { const z_loaned_timestamp_stack_t *stack; } SampleCtx;

static void on_sample(z_loaned_sample_t *sample, void *arg) {
    SampleCtx *ctx = (SampleCtx *)arg;
    z_owned_string_t keystr;
    z_keyexpr_to_string(z_sample_keyexpr(sample), &keystr);
    printf("Received sample on '%.*s':\n",
           (int)z_string_len(z_string_loan(&keystr)),
           z_string_data(z_string_loan(&keystr)));
    z_string_drop(z_string_move(&keystr));
    ctx->stack = z_sample_timestamp_stack(sample);
    print_stack(ctx->stack);
}

int main(void) {
    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t session;
    if (z_open(&session, z_config_move(&config), NULL) != Z_OK) {
        fprintf(stderr, "Failed to open session\n");
        return 1;
    }

    // ── Example 1: put/subscribe with send+receive instrumentation ────────────
    printf("\n── put/subscribe with send+receive instrumentation ─────────────────\n");
    {
        z_view_keyexpr_t ke;
        z_view_keyexpr_from_str(&ke, "demo/ts/hello");

        SampleCtx ctx = {NULL};
        z_owned_closure_sample_t cb;
        z_closure_sample(&cb, on_sample, NULL, &ctx);
        z_owned_subscriber_t sub;
        z_declare_subscriber(z_session_loan(&session), &sub, z_view_keyexpr_loan(&ke),
                             z_closure_sample_move(&cb), NULL);
        z_sleep_ms(50);

        z_owned_timestamp_instrumentation_t instr;
        z_timestamp_instrumentation_new(&instr, /*send=*/true, /*route=*/false, /*receive=*/true);

        z_put_options_t opts = z_put_options_default();
        opts.timestamp_instrumentation = z_timestamp_instrumentation_loan(&instr);

        z_owned_bytes_t payload;
        z_bytes_from_static_str(&payload, "world");
        z_put(z_session_loan(&session), z_view_keyexpr_loan(&ke), z_bytes_move(&payload), &opts);
        z_sleep_ms(200);

        z_timestamp_instrumentation_drop(z_timestamp_instrumentation_move(&instr));
        z_undeclare_subscriber(z_subscriber_move(&sub));
    }

    // ── Example 2: publisher with default instrumentation ────────────────────
    printf("\n── publisher with default instrumentation ───────────────────────────\n");
    {
        z_view_keyexpr_t ke;
        z_view_keyexpr_from_str(&ke, "demo/ts/pub");

        SampleCtx ctx = {NULL};
        z_owned_closure_sample_t cb;
        z_closure_sample(&cb, on_sample, NULL, &ctx);
        z_owned_subscriber_t sub;
        z_declare_subscriber(z_session_loan(&session), &sub, z_view_keyexpr_loan(&ke),
                             z_closure_sample_move(&cb), NULL);

        z_owned_timestamp_instrumentation_t instr;
        z_timestamp_instrumentation_new(&instr, true, false, true);
        z_publisher_options_t pub_opts = z_publisher_options_default();
        pub_opts.timestamp_instrumentation = z_timestamp_instrumentation_loan(&instr);

        z_owned_publisher_t pub;
        z_declare_publisher(z_session_loan(&session), &pub, z_view_keyexpr_loan(&ke), &pub_opts);
        z_sleep_ms(50);

        z_owned_bytes_t p1;
        z_bytes_from_static_str(&p1, "message-1");
        z_publisher_put(z_publisher_loan(&pub), z_bytes_move(&p1), NULL);
        z_sleep_ms(200);

        z_undeclare_publisher(z_publisher_move(&pub));
        z_timestamp_instrumentation_drop(z_timestamp_instrumentation_move(&instr));
        z_undeclare_subscriber(z_subscriber_move(&sub));
    }

    z_session_drop(z_session_move(&session));
    printf("\nDone.\n");
    return 0;
}

#else  // ZENOHC_BUILD_WITH_UNSTABLE_API not set

int main(void) {
    fprintf(stderr, "This example requires ZENOH_UNSTABLE=ON\n");
    return 1;
}

#endif
