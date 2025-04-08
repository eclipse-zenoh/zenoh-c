//
// Copyright (c) 2025 ZettaScale Technology
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
#include <stdatomic.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

void data_handler(z_loaned_sample_t *sample, void *arg) {
    atomic_int *val = (atomic_int *)arg;
    atomic_fetch_add(val, 1);
    z_sleep_s(5);
    atomic_fetch_add(val, 1);
}

void query_handler(z_loaned_query_t *query, void *arg) {
    atomic_int *val = (atomic_int *)arg;
    atomic_fetch_add(val, 1);
    z_sleep_s(5);
    atomic_fetch_add(val, 1);

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/query_callbacks_drop");
    z_owned_bytes_t p;
    z_bytes_copy_from_str(&p, "reply");
    z_query_reply(query, z_loan(ke), z_move(p), NULL);
}

void reply_handler(z_loaned_reply_t *reply, void *arg) {
    atomic_int *val = (atomic_int *)arg;
    atomic_fetch_add(val, 1);
    z_sleep_s(5);
    atomic_fetch_add(val, 1);
}

void test_pub_sub() {
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/callbacks_drop");

    z_owned_config_t c1, c2;
    z_owned_session_t s1, s2;
    z_config_default(&c1);
    z_config_default(&c2);

    assert(z_open(&s1, z_move(c1), NULL) == Z_OK);
    assert(z_open(&s2, z_move(c2), NULL) == Z_OK);

    atomic_int val = ATOMIC_VAR_INIT(0);
    z_owned_subscriber_t sub;
    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, (void *)&val);
    assert(z_declare_subscriber(z_loan(s1), &sub, z_loan(ke), z_move(callback), NULL) == Z_OK);
    z_sleep_s(1);
    z_owned_bytes_t p;
    z_bytes_copy_from_str(&p, "data");
    assert(z_put(z_loan(s2), z_loan(ke), z_move(p), NULL) == Z_OK);

    z_sleep_s(1);
    z_drop(z_move(sub));
    int out = 0;
    out = atomic_load(&val);
    assert(out == 2);

    z_drop(z_move(s1));
    z_drop(z_move(s2));
}

void test_query_reply() {
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/query_callbacks_drop");

    z_owned_config_t c1, c2;
    z_owned_session_t s1, s2;
    z_config_default(&c1);
    z_config_default(&c2);

    assert(z_open(&s1, z_move(c1), NULL) == Z_OK);
    assert(z_open(&s2, z_move(c2), NULL) == Z_OK);

    atomic_int val = ATOMIC_VAR_INIT(0);
    z_owned_queryable_t q;
    z_owned_closure_query_t q_callback;
    z_closure(&q_callback, query_handler, NULL, (void *)&val);

    assert(z_declare_queryable(z_loan(s1), &q, z_loan(ke), z_move(q_callback), NULL) == Z_OK);
    z_sleep_s(1);

    z_owned_closure_reply_t r_callback;
    z_closure(&r_callback, reply_handler, NULL, (void *)&val);
    assert(z_get(z_loan(s2), z_loan(ke), "", z_move(r_callback), NULL) == Z_OK);

    z_sleep_s(1);
    z_drop(z_move(q));
    int out = 0;
    out = atomic_load(&val);
    assert(out == 2);

    z_sleep_s(1);
    z_drop(z_move(s2));
    out = atomic_load(&val);
    assert(out == 4);

    z_drop(z_move(s1));
}

int main(int argc, char **argv) {
    test_pub_sub();
    test_query_reply();
}
