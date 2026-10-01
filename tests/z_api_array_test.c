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

#include <stddef.h>
#include <stdio.h>
#include <string.h>

#undef NDEBUG
#include <assert.h>

#include "zenoh.h"

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

static size_t pointer_calls;
static size_t length_calls;

static z_owned_string_t* array_pointer(z_owned_string_t* arr) {
    ++pointer_calls;
    return arr;
}

static size_t array_length(void) {
    ++length_calls;
    return 3;
}

static void test_array_arguments(void) {
    z_owned_string_t arr[3];
    for (size_t i = 0; i < 3; ++i) init_string(&arr[i]);
    z_drop_array(z_move_array(array_pointer(arr)), array_length());
    assert(pointer_calls == 1);
    assert(length_calls == 1);
    for (size_t i = 0; i < 3; ++i) assert(!z_internal_check(arr[i]));
    z_drop_array((z_moved_string_t*)NULL, 0);
}

int main(void) {
    test_array_arguments();
    TEST_ARRAY(z_owned_string_t, init_string)
    TEST_ARRAY(z_owned_bytes_t, init_bytes)
    return 0;
}
