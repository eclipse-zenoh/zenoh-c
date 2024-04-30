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

#undef NDEBUG
#include <assert.h> 

void test_reader() {
    uint8_t data[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    uint8_t data_out[10] = {0};
    z_view_slice_t slice;
    z_view_slice_wrap(&slice, data, 10);

    z_owned_bytes_t payload;
    z_bytes_encode_from_slice(&payload, z_loan(slice));
    z_owned_bytes_reader_t reader;
    z_bytes_reader_new(&reader, z_loan(payload));
    assert(z_bytes_reader_tell(z_loan_mut(reader)) == 0);

    z_bytes_reader_read(z_loan_mut(reader), data_out, 5);
    assert(z_bytes_reader_tell(z_loan_mut(reader)) == 5);
    z_bytes_reader_seek(z_loan_mut(reader), 2, SEEK_CUR);
    assert(z_bytes_reader_tell(z_loan_mut(reader)) == 7);
    z_bytes_reader_read(z_loan_mut(reader), data_out + 7, 2);
    z_bytes_reader_seek(z_loan_mut(reader), 5, SEEK_SET);
    assert(z_bytes_reader_tell(z_loan_mut(reader)) == 7);
    z_bytes_reader_read(z_loan_mut(reader), data_out + 5, 2);

    z_bytes_reader_seek(z_loan_mut(reader), -1, SEEK_END);
    assert(z_bytes_reader_tell(z_loan_mut(reader)) == 9);
    z_bytes_reader_read(z_loan_mut(reader), data_out + 9, 1);
    assert(memcmp(data, data_out, 10));
}

int main(int argc, char **argv) {
    test_reader();
}
