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

void writting_by_alias_read_by_get() {
    // Writing
    z_owned_slice_map_t map;
    z_slice_map_new(&map);
    z_view_slice_t k1, k2, v1, v2, k0;
    z_view_slice_from_str(&k0, "k0");
    z_view_slice_from_str(&k1, "k1");
    z_view_slice_from_str(&k2, "k2");
    z_view_slice_from_str(&v1, "v1");
    z_view_slice_from_str(&v2, "v2");
    z_slice_map_insert_by_alias(z_loan_mut(map), z_loan(k1), z_loan(v1));
    z_slice_map_insert_by_alias(z_loan_mut(map), z_loan(k2), z_loan(v2));
    z_owned_bytes_t attachment;
    z_bytes_encode_from_slice_map(&attachment, z_loan(map));

    // Elements check

    assert(z_slice_map_len(z_loan(map)) == 2);
    assert(!z_slice_map_is_empty(z_loan(map)));

    const z_loaned_slice_t* a1 = z_slice_map_get(z_loan(map), z_loan(k1));
    ASSERT_STR_SLICE_EQUAL("v1", a1);

    const z_loaned_slice_t* a2 = z_slice_map_get(z_loan(map), z_loan(k2));
    ASSERT_STR_SLICE_EQUAL("v2", a2);

    const z_loaned_slice_t* a_null = z_slice_map_get(z_loan(map), z_loan(k0));
    assert(a_null == NULL);

    z_drop(z_move(map));
}

bool map_reader(const z_loaned_slice_t* key, const z_loaned_slice_t* value, void* ctx) {
    assert((size_t)ctx == 42);
    if (!strncmp((const char*)z_slice_data(key), "k1", z_slice_len(key))) {
        assert(!strncmp((const char*)z_slice_data(value), "v1", z_slice_len(value)));
    } else if (!strncmp((const char*)z_slice_data(key), "k2", z_slice_len(key))) {
        assert(!strncmp((const char*)z_slice_data(value), "v2", z_slice_len(value)));
    } else {
        assert(false);
    }
    return true;
}

void writting_by_copy_read_by_iter() {
    // Writing
    z_owned_slice_map_t map;
    z_slice_map_new(&map);

    z_view_slice_t k1, k2, v1, v2, k0;
    z_view_slice_from_str(&k0, "k0");
    z_view_slice_from_str(&k1, "k1");
    z_view_slice_from_str(&k2, "k2");
    z_view_slice_from_str(&v1, "v1");
    z_view_slice_from_str(&v2, "v2");
    z_slice_map_insert_by_copy(z_loan_mut(map), z_loan(k1), z_loan(v1));
    z_slice_map_insert_by_copy(z_loan_mut(map), z_loan(k2), z_loan(v2));

    // Elements check
    assert(z_slice_map_len(z_loan(map)) == 2);

    z_slice_map_iterate(z_loan(map), map_reader, (void*)42);

    z_drop(z_move(map));
}

void empty_map_safety() {
    z_owned_slice_map_t map;
    z_slice_map_new(&map);
    assert(z_slice_map_is_empty(z_loan(map)));
    assert(z_slice_map_len(z_loan(map)) == 0);

    z_view_slice_t k0;
    z_view_slice_from_str(&k0, "k0");

    const z_loaned_slice_t* a_null = z_slice_map_get(z_loan(map), z_loan(k0));
    assert(a_null == NULL);

    z_slice_map_iterate(z_loan(map), map_reader, NULL);
}

int main(int argc, char** argv) {
    writting_by_alias_read_by_get();
    writting_by_copy_read_by_iter();
    empty_map_safety();
}
