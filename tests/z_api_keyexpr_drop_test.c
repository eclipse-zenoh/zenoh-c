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

void test_publisher() {
    z_owned_config_t config = z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    assert(z_check(s));
    char keyexpr[256];
    strncpy(keyexpr, "foo/bar", 256);
    z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(keyexpr), NULL);
    // strncpy(keyexpr, "baz/quax", 256);
    z_keyexpr_t pub_keyexpr = z_publisher_keyexpr(z_loan(pub));
    z_owned_str_t pub_keyexpr_str = z_keyexpr_to_string(pub_keyexpr);
    assert(strcmp(z_loan(pub_keyexpr_str), "foo/bar") == 0);
    z_drop(z_move(pub_keyexpr_str));
    z_drop(z_move(pub));
    z_drop(z_move(s));
}

void data_handler(const z_sample_t *sample, void *arg) {}

// void test_pull_subscriber() {
//     z_owned_config_t config = z_config_default();
//     z_owned_session_t s = z_open(z_move(config));
//     z_owned_closure_sample_t callback = z_closure(data_handler);
//     char keyexpr[256];
//     strncpy(keyexpr, "foo/bar", 256);
//     z_owned_pull_subscriber_t sub = z_declare_pull_subscriber(z_loan(s), z_keyexpr(keyexpr), z_move(callback), NULL);
//     strncpy(keyexpr, "baz/quax", 256);
//     z_drop(z_move(sub));
//     z_drop(z_move(s));
// }

void test_subscriber() {
    z_owned_config_t config = z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    z_owned_closure_sample_t callback = z_closure(data_handler);
    char keyexpr[256];
    strncpy(keyexpr, "foo/bar", 256);
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(keyexpr), z_move(callback), NULL);
    strncpy(keyexpr, "baz/quax", 256);
    z_keyexpr_t sub_keyexpr = z_subscriber_keyexpr(z_loan(sub));
    z_owned_str_t sub_keyexpr_str = z_keyexpr_to_string(sub_keyexpr);
    assert(strcmp(z_loan(sub_keyexpr_str), "foo/bar") == 0);
    z_drop(z_move(sub_keyexpr_str));
    z_drop(z_move(sub));
    z_drop(z_move(s));
}

// void query_handler(const z_query_t *query, void *context) {}

// void test_queryable() {
//     z_owned_config_t config = z_config_default();
//     z_owned_session_t s = z_open(z_move(config));
//     z_owned_closure_query_t callback = z_closure(query_handler);
//     char keyexpr[256];
//     strncpy(keyexpr, "foo/bar", 256);
//     z_owned_queryable_t queryable = z_declare_queryable(z_loan(s), z_keyexpr(keyexpr), z_move(callback), NULL);
//     strncpy(keyexpr, "baz/quax", 256);
//     z_drop(z_move(queryable));
//     z_drop(z_move(s));
// }

int main(int argc, char **argv) {
    test_publisher();
    // test_pull_subscriber();
    test_subscriber();
    // test_queryable();

    return 0;
}