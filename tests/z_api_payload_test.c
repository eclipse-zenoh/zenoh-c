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

void test_reader_seek() {
    uint8_t data[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    uint8_t data_out[10] = {0};

    z_owned_bytes_t payload;
    z_bytes_encode_from_slice(&payload, data, 10);

    z_owned_bytes_reader_t reader;
    z_bytes_reader_new(&reader, z_loan(payload));
    assert(z_bytes_reader_tell(z_loan_mut(reader)) == 0);

    assert(0 == z_bytes_reader_seek(z_loan_mut(reader), 5, SEEK_CUR));
    assert(z_bytes_reader_tell(z_loan_mut(reader)) == 5);

    assert(0 == z_bytes_reader_seek(z_loan_mut(reader), 7, SEEK_SET));
    assert(z_bytes_reader_tell(z_loan_mut(reader)) == 7);

    assert(0 == z_bytes_reader_seek(z_loan_mut(reader), -1, SEEK_END));
    assert(z_bytes_reader_tell(z_loan_mut(reader)) == 9);

    assert(z_bytes_reader_seek(z_loan_mut(reader), 20, SEEK_SET) < 0);

    assert(0 == z_bytes_reader_seek(z_loan_mut(reader), 5, SEEK_SET));
    assert(z_bytes_reader_tell(z_loan_mut(reader)) == 5);

    assert(z_bytes_reader_seek(z_loan_mut(reader), 10, SEEK_CUR) < 0);
    assert(z_bytes_reader_seek(z_loan_mut(reader), 10, SEEK_END) < 0);
    assert(z_bytes_reader_seek(z_loan_mut(reader), -20, SEEK_END) < 0);

    z_drop(z_move(reader));
    z_drop(z_move(payload));
}

void test_reader_read() {
    uint8_t data[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    uint8_t data_out[10] = {0};

    z_owned_bytes_t payload;
    z_bytes_encode_from_slice(&payload, data, 10);
    z_owned_bytes_reader_t reader;
    z_bytes_reader_new(&reader, z_loan(payload));

    assert(5 == z_bytes_reader_read(z_loan_mut(reader), data_out, 5));

    z_bytes_reader_seek(z_loan_mut(reader), 2, SEEK_CUR);
    assert(2 == z_bytes_reader_read(z_loan_mut(reader), data_out + 7, 2));

    z_bytes_reader_seek(z_loan_mut(reader), 5, SEEK_SET);
    assert(2 == z_bytes_reader_read(z_loan_mut(reader), data_out + 5, 2));

    z_bytes_reader_seek(z_loan_mut(reader), -1, SEEK_END);
    assert(1 == z_bytes_reader_read(z_loan_mut(reader), data_out + 9, 10));

    assert(0 == z_bytes_reader_read(z_loan_mut(reader), data_out, 10));

    assert(!memcmp(data, data_out, 10));

    z_drop(z_move(reader));
    z_drop(z_move(payload));
}

int main(int argc, char **argv) {
    test_reader_read();
}
