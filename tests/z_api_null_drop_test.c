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

#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

int main(int argc, char **argv) {

    //
    // Check that all null functions exists
    //
    z_owned_session_t session_null_1 = z_session_null();
    z_owned_publisher_t publisher_null_1 = z_publisher_null();
    z_owned_keyexpr_t keyexpr_null_1 = z_keyexpr_null();
    z_owned_config_t config_null_1 = z_config_null();
    z_owned_scouting_config_t scouting_config_null_1 = z_scouting_config_null();
    z_owned_pull_subscriber_t pull_subscriber_null_1 = z_pull_subscriber_null();
    z_owned_subscriber_t subscriber_null_1 = z_subscriber_null();
    z_owned_queryable_t queryable_null_1 = z_queryable_null();
    z_owned_encoding_t encoding_null_1 = z_encoding_null();
    z_owned_reply_t reply_null_1 = z_reply_null();
    z_owned_hello_t hello_null_1 = z_hello_null();
    z_owned_closure_sample_t closure_sample_null_1 = z_closure_sample_null();
    z_owned_closure_query_t closure_query_null_1 = z_closure_query_null();
    z_owned_closure_reply_t closure_reply_null_1 = z_closure_reply_null();
    z_owned_closure_hello_t closure_hello_null_1 = z_closure_hello_null();
    z_owned_closure_zid_t closure_zid_null_1 = z_closure_zid_null();
    z_owned_reply_channel_closure_t reply_channel_closure_null_1 = z_reply_channel_closure_null();
    z_owned_reply_channel_t reply_channel_null_1 = z_reply_channel_null();

    //
    // Test that they actually make invalid value (where applicable)
    //
    assert(!z_check(session_null_1));
    assert(!z_check(publisher_null_1));
    assert(!z_check(keyexpr_null_1));
    assert(!z_check(config_null_1));
    assert(!z_check(scouting_config_null_1));
    assert(!z_check(pull_subscriber_null_1));
    assert(!z_check(subscriber_null_1));
    assert(!z_check(queryable_null_1));
    assert(!z_check(encoding_null_1));
    assert(!z_check(reply_null_1));
    assert(!z_check(hello_null_1));

    //
    // Test that z_null macro defined for all types
    //
    z_owned_session_t session_null_2;
    z_owned_publisher_t publisher_null_2;
    z_owned_keyexpr_t keyexpr_null_2;
    z_owned_config_t config_null_2;
    z_owned_scouting_config_t scouting_config_null_2;
    z_owned_pull_subscriber_t pull_subscriber_null_2;
    z_owned_subscriber_t subscriber_null_2;
    z_owned_queryable_t queryable_null_2;
    z_owned_encoding_t encoding_null_2;
    z_owned_reply_t reply_null_2;
    z_owned_hello_t hello_null_2;
    z_owned_closure_sample_t closure_sample_null_2;
    z_owned_closure_query_t closure_query_null_2;
    z_owned_closure_reply_t closure_reply_null_2;
    z_owned_closure_hello_t closure_hello_null_2;
    z_owned_closure_zid_t closure_zid_null_2;
    z_owned_reply_channel_closure_t reply_channel_closure_null_2;
    z_owned_reply_channel_t reply_channel_null_2;

    z_null(&session_null_2);
    z_null(&publisher_null_2);
    z_null(&keyexpr_null_2);
    z_null(&config_null_2);
    z_null(&scouting_config_null_2);
    z_null(&pull_subscriber_null_2);
    z_null(&subscriber_null_2);
    z_null(&queryable_null_2);
    z_null(&encoding_null_2);
    z_null(&reply_null_2);
    z_null(&hello_null_2);
    z_null(&closure_sample_null_2);
    z_null(&closure_query_null_2);
    z_null(&closure_reply_null_2);
    z_null(&closure_hello_null_2);
    z_null(&closure_zid_null_2);
    z_null(&reply_channel_closure_null_2);
    z_null(&reply_channel_null_2);

    //
    // Test that null macro works the same as direct call
    //
    assert(!z_check(session_null_2));
    assert(!z_check(publisher_null_2));
    assert(!z_check(keyexpr_null_2));
    assert(!z_check(config_null_2));
    assert(!z_check(scouting_config_null_2));
    assert(!z_check(pull_subscriber_null_2));
    assert(!z_check(subscriber_null_2));
    assert(!z_check(queryable_null_2));
    assert(!z_check(encoding_null_2));
    assert(!z_check(reply_null_2));
    assert(!z_check(hello_null_2));

    //
    // Test drop null and double drop it
    //
    for (int i = 0; i < 2; ++i) {
        z_drop(&session_null_1);
        z_drop(&publisher_null_1);
        z_drop(&keyexpr_null_1);
        z_drop(&config_null_1);
        z_drop(&scouting_config_null_1);
        z_drop(&pull_subscriber_null_1);
        z_drop(&subscriber_null_1);
        z_drop(&queryable_null_1);
        z_drop(&encoding_null_1);
        z_drop(&reply_null_1);
        z_drop(&hello_null_1);
        z_drop(&closure_sample_null_1);
        z_drop(&closure_query_null_1);
        z_drop(&closure_reply_null_1);
        z_drop(&closure_hello_null_1);
        z_drop(&closure_zid_null_1);
        z_drop(&reply_channel_closure_null_1);
        z_drop(&reply_channel_null_1);

        z_drop(&session_null_2);
        z_drop(&publisher_null_2);
        z_drop(&keyexpr_null_2);
        z_drop(&config_null_2);
        z_drop(&scouting_config_null_2);
        z_drop(&pull_subscriber_null_2);
        z_drop(&subscriber_null_2);
        z_drop(&queryable_null_2);
        z_drop(&encoding_null_2);
        z_drop(&reply_null_2);
        z_drop(&hello_null_2);
        z_drop(&closure_sample_null_2);
        z_drop(&closure_query_null_2);
        z_drop(&closure_reply_null_2);
        z_drop(&closure_hello_null_2);
        z_drop(&closure_zid_null_2);
        z_drop(&reply_channel_closure_null_2);
        z_drop(&reply_channel_null_2);
    }

    return 0;
}
