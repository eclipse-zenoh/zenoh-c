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

#undef NDEBUG
#include <assert.h>

#include "zenoh.h"

// fill v with invalid values
// set v to null
// check if v it is null
// make sure that drop on null does not crash
// make sure that double drop on null does not crash
// fill v with invalid values again
//
// set v1 to null
// move v to v1
// make sure that v is null now
#define TEST(name)                    \
    {                                 \
        name v;                       \
        memset(&v, -1, sizeof(v));    \
        z_internal_null(&v);          \
        assert(!z_internal_check(v)); \
        z_drop(z_move(v));            \
        z_drop(z_move(v));            \
        name v1;                      \
        z_internal_null(&v1);         \
        memset(&v, -1, sizeof(v));    \
        z_take(&v1, z_move(v));       \
        assert(!z_internal_check(v)); \
    }

#define TEST_TAKE_MUT(name)                     \
    {                                           \
        name v;                                 \
        name v1;                                \
        z_internal_null(&v1);                   \
        memset(&v, -1, sizeof(v));              \
        z_take_from_loaned(&v1, z_loan_mut(v)); \
        z_drop(z_move(v));                      \
        assert(!z_internal_check(v));           \
    }

// build an array of two live values and one null
// drop the whole array through z_move_array / z_drop_array
// make sure that every element is null now, and that dropping again is a no-op
#define TEST_ARRAY(name, init)                 \
    {                                          \
        name arr[3];                           \
        init(&arr[0]);                         \
        init(&arr[1]);                         \
        z_internal_null(&arr[2]);              \
        assert(z_internal_check(arr[0]));      \
        z_drop_array(z_move_array(arr), 3);    \
        for (size_t i = 0; i < 3; ++i) {       \
            assert(!z_internal_check(arr[i])); \
        }                                      \
        z_drop_array(z_move_array(arr), 3);    \
    }

static void init_string(z_owned_string_t* s) { z_string_copy_from_str(s, "abc"); }
static void init_bytes(z_owned_bytes_t* b) { z_bytes_copy_from_str(b, "abc"); }

int main(void) {
    TEST_ARRAY(z_owned_string_t, init_string)
    TEST_ARRAY(z_owned_bytes_t, init_bytes)

    TEST(z_owned_session_t)
    TEST(z_owned_keyexpr_t)
    TEST(z_owned_config_t)
    TEST(z_owned_hello_t)
    TEST(z_owned_closure_sample_t)
    TEST(z_owned_closure_query_t)
    TEST(z_owned_closure_reply_t)
    TEST(z_owned_closure_hello_t)
#if defined(Z_FEATURE_UNSTABLE_API_t)
    TEST(z_owned_closure_zid_t)
#endif
    TEST(z_owned_string_t)
    TEST(z_owned_string_array_t)
    TEST(z_owned_sample_t)
    TEST(z_owned_query_t)
    TEST(z_owned_slice_t)
    TEST(z_owned_bytes_t)
    TEST(z_owned_bytes_writer_t)
    TEST(z_owned_encoding_t)
    TEST(z_owned_publisher_t)
    TEST(z_owned_subscriber_t)
    TEST(z_owned_queryable_t)
    TEST(z_owned_reply_t)

    TEST(ze_owned_serializer_t)
    // Double drop not supported for these types
    // TEST(z_owned_task_t)
    // TEST(z_owned_mutex_t)
    // TEST(z_owned_condvar_t)

    TEST_TAKE_MUT(z_owned_sample_t)
    TEST_TAKE_MUT(z_owned_query_t)
    TEST_TAKE_MUT(z_owned_reply_t)
    TEST_TAKE_MUT(z_owned_hello_t)

    return 0;
}