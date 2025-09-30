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
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>
z_owned_mutex_t mu;

typedef struct z_val_t {
    int val;
    bool dropped;
} z_val_t;

void drop(void *arg) {
    z_mutex_lock(z_loan_mut(mu));
    z_val_t *val = (z_val_t *)arg;
    val->dropped = true;
    z_mutex_unlock(z_loan_mut(mu));
}

void data_handler(z_loaned_sample_t *sample, void *arg) {
    z_val_t *val = (z_val_t *)arg;
    z_mutex_lock(z_loan_mut(mu));
    (val->val)++;
    z_mutex_unlock(z_loan_mut(mu));
    z_sleep_s(5);
    z_mutex_lock(z_loan_mut(mu));
    (val->val)++;
    z_mutex_unlock(z_loan_mut(mu));
}

void query_handler(z_loaned_query_t *query, void *arg) {
    z_val_t *val = (z_val_t *)arg;
    z_mutex_lock(z_loan_mut(mu));
    (val->val)++;
    z_mutex_unlock(z_loan_mut(mu));
    z_sleep_s(5);
    z_mutex_lock(z_loan_mut(mu));
    (val->val)++;
    z_mutex_unlock(z_loan_mut(mu));

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/query_callbacks_drop");
    z_owned_bytes_t p;
    z_bytes_copy_from_str(&p, "reply");
    z_query_reply(query, z_loan(ke), z_move(p), NULL);
}

void reply_handler(z_loaned_reply_t *reply, void *arg) {
    z_val_t *val = (z_val_t *)arg;
    z_mutex_lock(z_loan_mut(mu));
    (val->val)++;
    z_mutex_unlock(z_loan_mut(mu));
    z_sleep_s(5);
    z_mutex_lock(z_loan_mut(mu));
    (val->val)++;
    z_mutex_unlock(z_loan_mut(mu));
}

void test_pub_sub() {
    z_mutex_init(&mu);
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/callbacks_drop");

    z_owned_config_t c1, c2;
    z_owned_session_t s1, s2;
    z_config_default(&c1);
    z_config_default(&c2);

    assert(z_open(&s1, z_move(c1), NULL) == Z_OK);
    assert(z_open(&s2, z_move(c2), NULL) == Z_OK);

    z_val_t val = {0, false};
    z_owned_subscriber_t sub;
    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, drop, (void *)&val);
    assert(z_declare_subscriber(z_loan(s1), &sub, z_loan(ke), z_move(callback), NULL) == Z_OK);
    z_sleep_s(1);
    z_owned_bytes_t p;
    z_bytes_copy_from_str(&p, "data");
    assert(z_put(z_loan(s2), z_loan(ke), z_move(p), NULL) == Z_OK);

    z_sleep_s(1);
    z_drop(z_move(sub));
    z_mutex_lock(z_loan_mut(mu));
    assert(val.dropped);
    assert(val.val == 2);
    z_mutex_unlock(z_loan_mut(mu));

    z_drop(z_move(s1));
    z_drop(z_move(s2));
    z_drop(z_move(mu));
}

void test_query_reply() {
    z_mutex_init(&mu);
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "test/query_callbacks_drop");

    z_owned_config_t c1, c2;
    z_owned_session_t s1, s2;
    z_config_default(&c1);
    z_config_default(&c2);

    assert(z_open(&s1, z_move(c1), NULL) == Z_OK);
    assert(z_open(&s2, z_move(c2), NULL) == Z_OK);

    z_val_t query_val = {0, false};
    z_val_t reply_val = {0, false};

    z_owned_queryable_t q;
    z_owned_closure_query_t q_callback;
    z_closure(&q_callback, query_handler, drop, (void *)&query_val);

    assert(z_declare_queryable(z_loan(s1), &q, z_loan(ke), z_move(q_callback), NULL) == Z_OK);
    z_sleep_s(1);

    z_owned_closure_reply_t r_callback;
    z_closure(&r_callback, reply_handler, drop, (void *)&reply_val);
    assert(z_get(z_loan(s2), z_loan(ke), "", z_move(r_callback), NULL) == Z_OK);

    z_sleep_s(1);
    z_drop(z_move(q));
    z_mutex_lock(z_loan_mut(mu));
    assert(query_val.dropped);
    assert(query_val.val == 2);
    z_mutex_unlock(z_loan_mut(mu));

    z_sleep_s(1);
    z_drop(z_move(s2));
    z_mutex_lock(z_loan_mut(mu));
    assert(reply_val.dropped);
    assert(reply_val.val == 2);
    z_mutex_unlock(z_loan_mut(mu));

    z_drop(z_move(s1));
    z_drop(z_move(mu));
}

int main(int argc, char **argv) {
    test_pub_sub();
    test_query_reply();
}
