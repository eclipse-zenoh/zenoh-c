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

#define URL1 "foo/bar/1"
#define URL2 "foo/bar/2"

void test_publisher() {
    z_owned_config_t config = z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    assert(z_check(s));
    z_owned_publisher_t pub1 = z_declare_publisher(z_loan(s), z_keyexpr(URL1), NULL);
    z_owned_publisher_t pub2 = z_declare_publisher(z_loan(s), z_keyexpr(URL2), NULL);
    z_drop(z_move(pub1));
    z_drop(z_move(pub2));
    z_drop(z_move(s));
}

// void data_handler(const z_sample_t *sample, void *arg) {}

// void test_pull_subscriber() {
//     z_owned_config_t config = z_config_default();
//     z_owned_session_t s = z_open(z_move(config));
//     z_owned_closure_sample_t callback = z_closure(data_handler);
//     z_owned_pull_subscriber_t sub = z_declare_pull_subscriber(z_loan(s), z_keyexpr(URL), z_move(callback), NULL);
//     assert(z_check(sub));
//     z_drop(z_move(sub));
//     assert(!z_check(sub));
//     z_drop(z_move(sub));
//     assert(!z_check(sub));
//     z_drop(z_move(s));
// }

// void test_subscriber() {
//     z_owned_config_t config = z_config_default();
//     z_owned_session_t s = z_open(z_move(config));
//     z_owned_closure_sample_t callback = z_closure(data_handler);
//     z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(URL), z_move(callback), NULL);
//     assert(z_check(sub));
//     z_drop(z_move(sub));
//     assert(!z_check(sub));
//     z_drop(z_move(sub));
//     assert(!z_check(sub));
//     z_drop(z_move(s));
// }

// void query_handler(const z_query_t *query, void *context) {}

// void test_queryable() {
//     z_owned_config_t config = z_config_default();
//     z_owned_session_t s = z_open(z_move(config));
//     z_owned_closure_query_t callback = z_closure(query_handler);
//     z_owned_queryable_t queryable = z_declare_queryable(z_loan(s), z_keyexpr(URL), z_move(callback), NULL);
//     assert(z_check(queryable));
//     z_drop(z_move(queryable));
//     assert(!z_check(queryable));
//     z_drop(z_move(queryable));
//     assert(!z_check(queryable));
//     z_drop(z_move(s));
// }

int main(int argc, char **argv) {
    test_publisher();
    // test_pull_subscriber();
    // test_subscriber();
    // test_queryable();

    return 0;
}