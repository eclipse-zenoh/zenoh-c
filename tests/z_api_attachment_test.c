//
// Copyright (c) 2023 ZettaScale Technology
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

#include "z_int_helpers.h"
#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

void writting_through_map_by_alias_read_by_get() {
    // Writing
    z_owned_bytes_map_t map = z_slice_map_new();
    z_slice_map_insert_by_alias(&map, z_slice_from_str("k1"), z_slice_from_str("v1"));
    z_slice_map_insert_by_alias(&map, z_slice_from_str("k2"), z_slice_from_str("v2"));
   z_bytes_t attachment = z_slice_map_as_attachment(&map);

    // Elements check

    assert(z_slice_map_len(&map) == 2);
    assert(z_attachment_len(attachment) == 2);
    assert(!z_attachment_is_empty(attachment));

    z_slice_t a1 = z_attachment_get(attachment, z_slice_from_str("k1"));
    ASSERT_STR_BYTES_EQUAL("v1", a1);

    z_slice_t a2 = z_attachment_get(attachment, z_slice_from_str("k2"));
    ASSERT_STR_BYTES_EQUAL("v2", a2);

    z_slice_t a_non = z_attachment_get(attachment, z_slice_from_str("k_non"));
    assert(a_non.start == NULL);
    assert(a_non.len == 0);

    z_drop(z_move(map));
}

int8_t _attachment_reader(z_slice_t key, z_slice_t value, void* ctx) {
    assert((size_t)ctx == 42);
    if (!strncmp(key.start, "k1", key.len)) {
        assert(!strncmp(value.start, "v1", value.len));
    }
    if (!strncmp(key.start, "k2", key.len)) {
        assert(!strncmp(value.start, "v2", value.len));
    }
    return 24;
}

void writting_through_map_by_copy_read_by_iter() {
    // Writing
    z_owned_bytes_map_t map = z_slice_map_new();
    z_slice_map_insert_by_copy(&map, z_slice_from_str("k1"), z_slice_from_str("v1"));
    z_slice_map_insert_by_copy(&map, z_slice_from_str("k2"), z_slice_from_str("v2"));
   z_bytes_t attachment = z_slice_map_as_attachment(&map);

    // Elements check
    assert(z_slice_map_len(&map) == 2);
    assert(z_attachment_len(attachment) == 2);
    assert(!z_attachment_is_empty(attachment));

    int res = z_attachment_iterate(attachment, _attachment_reader, (void*)42);
    assert(res == 24);

    z_drop(z_move(map));
}

int8_t _iteration_driver(const void* data, z_attachment_iter_body_t body, void* ctx) {
    int8_t ret = 0;
    ret = body(z_slice_from_str("k1"), z_slice_from_str("v1"), ctx);
    if (ret) {
        return ret;
    }
    ret = body(z_slice_from_str("k2"), z_slice_from_str("v2"), ctx);
    return ret;
}

void writting_no_map_read_by_get() {
   z_bytes_t attachment = {.data = NULL, .iteration_driver = &_iteration_driver};

    // Elements check
    assert(z_attachment_len(attachment) == 2);
    assert(!z_attachment_is_empty(attachment));

    z_slice_t a1 = z_attachment_get(attachment, z_slice_from_str("k1"));
    ASSERT_STR_BYTES_EQUAL("v1", a1);

    z_slice_t a2 = z_attachment_get(attachment, z_slice_from_str("k2"));
    ASSERT_STR_BYTES_EQUAL("v2", a2);

    z_slice_t a_non = z_attachment_get(attachment, z_slice_from_str("k_non"));
    assert(a_non.start == NULL);
    assert(a_non.len == 0);
}

void invalid_attachment_safety() {
   z_bytes_t attachment = z_attachment_null();
    assert(z_attachment_is_empty(attachment));
    assert(z_attachment_len(attachment) == 0);

    z_slice_t a_non = z_attachment_get(attachment, z_slice_from_str("k_non"));
    assert(a_non.start == NULL);
    assert(a_non.len == 0);

    int res = z_attachment_iterate(attachment, _attachment_reader, NULL);
    assert(res != 0);
}

int main(int argc, char** argv) {
    writting_through_map_by_alias_read_by_get();
    writting_through_map_by_copy_read_by_iter();
    writting_no_map_read_by_get();
    invalid_attachment_safety();
}
