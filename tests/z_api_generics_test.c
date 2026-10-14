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

#include <stdlib.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

static void payload_drop(void* data, void* context) {
    free(data);
    ++*(int*)context;
}

static void test_owned_arrays(void) {
    int drops = 0;
    z_owned_bytes_t payloads[3];
    for (size_t i = 0; i < 3; ++i) {
        uint8_t* data = (uint8_t*)malloc(1);
        assert(data != NULL);
        *data = (uint8_t)i;
        assert(z_bytes_from_buf(&payloads[i], data, 1, payload_drop, &drops) == Z_OK);
        assert(z_internal_check(payloads[i]));
    }

    z_moved_bytes_t* moved = z_move_array(payloads);
    z_drop_array(moved, 3);
    assert(drops == 3);
    for (size_t i = 0; i < 3; ++i) assert(!z_internal_check(payloads[i]));
    z_drop_array(z_move_array(payloads), 3);
    assert(drops == 3);
    z_moved_bytes_t* empty = NULL;
    z_drop_array(empty, 0);
}

static void test_zid_string(void) {
    z_id_t zid = {0};
    zid.id[0] = 1;
    char buf[Z_ID_STR_LEN];
    assert(z_id_as_str(&zid, &buf) == buf);
    assert(strcmp(buf, "1") == 0);

    memset(zid.id, 0xff, sizeof(zid.id));
    assert(z_id_as_str(&zid, &buf) == buf);
    assert(strlen(buf) == Z_ID_STR_LEN - 1);
    z_owned_string_t text;
    z_id_to_string(&zid, &text);
    assert(z_string_len(z_loan(text)) == strlen(buf));
    assert(memcmp(z_string_data(z_loan(text)), buf, strlen(buf)) == 0);
    z_drop(z_move(text));
}

int main(void) {
    test_owned_arrays();
    test_zid_string();
    return 0;
}
