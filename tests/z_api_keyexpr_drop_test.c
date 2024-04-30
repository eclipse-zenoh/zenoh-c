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
    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    z_open(&s, z_move(config));
    assert(z_check(s));
    char keyexpr[256];
    strncpy(keyexpr, "foo/bar", 256);
    z_view_keyexpr_t ke;
    z_view_keyexpr_new(&ke, keyexpr);
    z_owned_publisher_t pub;
    z_declare_publisher(&pub, z_loan(s), z_loan(ke), NULL);
    strncpy(keyexpr, "baz/quax", 256);  // Update source string to ensure that the keyexpr is copied into publisher
    z_view_keyexpr_new(&ke, keyexpr);
    const z_loaned_keyexpr_t* pub_ke = z_publisher_keyexpr(z_loan(pub));
    z_owned_str_t pub_keyexpr; 
    z_keyexpr_to_string(pub_ke, &pub_keyexpr);
    assert(strcmp(z_str_data(z_loan(pub_keyexpr)), "foo/bar") == 0);  // Check that publisher keeps the correct keyexpr
    z_drop(z_move(pub_keyexpr));
    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));
}

void data_handler(const z_loaned_sample_t *sample, void *arg) {}

void test_subscriber() {
    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    z_open(&s, z_move(config));
    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, NULL);
    char keyexpr[256];
    strncpy(keyexpr, "foo/bar", 256);
    z_view_keyexpr_t ke;
    z_view_keyexpr_new(&ke, keyexpr);
    z_owned_subscriber_t sub;
    z_declare_subscriber(&sub, z_loan(s), z_loan(ke), z_move(callback), NULL);
    strncpy(keyexpr, "baz/quax", 256);  // Update source string to ensure that the keyexpr is copied into the subscriber
    z_view_keyexpr_new(&ke, keyexpr);
    const z_loaned_keyexpr_t* sub_ke = z_subscriber_keyexpr(z_loan(sub));
    z_owned_str_t sub_keyexpr;
    z_keyexpr_to_string(sub_ke, &sub_keyexpr);
    assert(strcmp(z_str_data(z_loan(sub_keyexpr)), "foo/bar") == 0);  // Check that subscriber keeps the correct keyexpr
    z_drop(z_move(sub_keyexpr));
    z_undeclare_subscriber(z_move(sub));
    z_close(z_move(s));
}

int main(int argc, char **argv) {
    test_publisher();
    test_subscriber();
    // TODO: Make same tests for pull subscriber and queryable when their `keyexpr` getters are implemented
    // test_pull_subscriber();
    // test_queryable();

    return 0;
}