//
// Copyright (c) 2022 ZettaScale Technology
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

#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

#define URL "demo/example"

void test_session() {
    z_owned_config_t config;
    z_config_default(&config);
    assert(z_internal_check(config));
    z_owned_session_t session;
    z_open(&session, z_move(config), NULL);
    assert(z_internal_check(session));
    z_close(z_move(session), NULL);
    assert(!z_internal_check(session));
    z_close(z_move(session), NULL);
    assert(!z_internal_check(session));
}

void test_publisher() {
    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    z_open(&s, z_move(config), NULL);

    z_owned_keyexpr_t keyexpr;
    z_keyexpr_from_str(&keyexpr, URL);

    z_owned_publisher_t pub;
    z_declare_publisher(&pub, z_loan(s), z_loan(keyexpr), NULL);
    assert(z_internal_check(pub));
    z_undeclare_publisher(z_move(pub));
    assert(!z_internal_check(pub));
    z_undeclare_publisher(z_move(pub));
    assert(!z_internal_check(pub));
    z_close(z_move(s), NULL);
}

void test_keyexpr() {
    z_owned_keyexpr_t keyexpr;
    z_keyexpr_from_str(&keyexpr, URL);

    assert(z_internal_check(keyexpr));
    z_drop(z_move(keyexpr));
    assert(!z_internal_check(keyexpr));
    z_drop(z_move(keyexpr));
    assert(!z_internal_check(keyexpr));
}

void test_config() {
    z_owned_config_t config;
    z_config_default(&config);
    assert(z_internal_check(config));
    z_drop(z_move(config));
    assert(!z_internal_check(config));
    z_drop(z_move(config));
    assert(!z_internal_check(config));
}

void data_handler(z_loaned_sample_t *sample, void *arg) {}

void test_subscriber() {
    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    z_open(&s, z_move(config), NULL);
    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, NULL);

    z_view_keyexpr_t keyexpr;
    z_view_keyexpr_from_str(&keyexpr, URL);
    z_owned_subscriber_t sub;
    z_declare_subscriber(&sub, z_loan(s), z_loan(keyexpr), z_move(callback), NULL);
    assert(z_internal_check(sub));
    z_undeclare_subscriber(z_move(sub));
    assert(!z_internal_check(sub));
    z_undeclare_subscriber(z_move(sub));
    assert(!z_internal_check(sub));
    z_close(z_move(s), NULL);
}

void query_handler(z_loaned_query_t *query, void *context) {}

void test_queryable() {
    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    z_open(&s, z_move(config), NULL);
    z_owned_closure_query_t callback;
    z_closure(&callback, query_handler, NULL, NULL);

    z_view_keyexpr_t keyexpr;
    z_view_keyexpr_from_str(&keyexpr, URL);
    z_owned_queryable_t queryable;
    z_declare_queryable(&queryable, z_loan(s), z_loan(keyexpr), z_move(callback), NULL);
    assert(z_internal_check(queryable));
    z_undeclare_queryable(z_move(queryable));
    assert(!z_internal_check(queryable));
    z_undeclare_queryable(z_move(queryable));
    assert(!z_internal_check(queryable));
    z_close(z_move(s), NULL);
}

int main(int argc, char **argv) {
    zc_try_init_log_from_env();
    test_session();
    test_publisher();
    test_keyexpr();
    test_config();
    test_subscriber();
    test_queryable();

    return 0;
}
