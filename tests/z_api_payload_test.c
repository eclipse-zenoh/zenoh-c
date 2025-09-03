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

void test_reader_seek(void) {
    uint8_t data[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};

    z_owned_bytes_t payload;
    z_bytes_from_buf(&payload, data, 10, NULL, NULL);

    z_bytes_reader_t reader = z_bytes_get_reader(z_loan(payload));
    assert(z_bytes_reader_tell(&reader) == 0);

    assert(0 == z_bytes_reader_seek(&reader, 5, SEEK_CUR));
    assert(z_bytes_reader_tell(&reader) == 5);

    assert(0 == z_bytes_reader_seek(&reader, 7, SEEK_SET));
    assert(z_bytes_reader_tell(&reader) == 7);

    assert(0 == z_bytes_reader_seek(&reader, -1, SEEK_END));
    assert(z_bytes_reader_tell(&reader) == 9);

    assert(z_bytes_reader_seek(&reader, 20, SEEK_SET) < 0);

    assert(0 == z_bytes_reader_seek(&reader, 5, SEEK_SET));
    assert(z_bytes_reader_tell(&reader) == 5);

    assert(z_bytes_reader_seek(&reader, 10, SEEK_CUR) < 0);
    assert(z_bytes_reader_seek(&reader, 10, SEEK_END) < 0);
    assert(z_bytes_reader_seek(&reader, -20, SEEK_END) < 0);

    z_drop(z_move(payload));
}

void test_reader_read(void) {
    uint8_t data[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    uint8_t data_out[10] = {0};

    z_owned_bytes_t payload;
    z_bytes_from_buf(&payload, data, 10, NULL, NULL);
    z_bytes_reader_t reader = z_bytes_get_reader(z_loan(payload));

    assert(5 == z_bytes_reader_read(&reader, data_out, 5));
    assert(5 == z_bytes_reader_remaining(&reader));

    z_bytes_reader_seek(&reader, 2, SEEK_CUR);
    assert(2 == z_bytes_reader_read(&reader, data_out + 7, 2));
    assert(1 == z_bytes_reader_remaining(&reader));

    z_bytes_reader_seek(&reader, 5, SEEK_SET);
    assert(2 == z_bytes_reader_read(&reader, data_out + 5, 2));
    assert(3 == z_bytes_reader_remaining(&reader));

    z_bytes_reader_seek(&reader, -1, SEEK_END);
    assert(1 == z_bytes_reader_read(&reader, data_out + 9, 10));
    assert(0 == z_bytes_reader_remaining(&reader));

    assert(0 == z_bytes_reader_read(&reader, data_out, 10));

    assert(!memcmp(data, data_out, 10));

    z_drop(z_move(payload));
}

void test_writer(void) {
    uint8_t data[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    uint8_t data_out[10] = {0};

    z_owned_bytes_t payload;

    z_owned_bytes_writer_t writer;
    z_bytes_writer_empty(&writer);

    assert(z_bytes_writer_write_all(z_loan_mut(writer), data, 3) == 0);
    assert(z_bytes_writer_write_all(z_loan_mut(writer), data + 3, 5) == 0);
    assert(z_bytes_writer_write_all(z_loan_mut(writer), data + 8, 2) == 0);
    z_bytes_writer_finish(z_move(writer), &payload);

    z_bytes_reader_t reader = z_bytes_get_reader(z_loan(payload));

    assert(10 == z_bytes_reader_read(&reader, data_out, 10));
    assert(0 == memcmp(data, data_out, 10));

    z_drop(z_move(payload));
}

void test_append(void) {
    uint8_t data[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    uint8_t data_out[10] = {0};

    z_owned_bytes_t payload;
    z_bytes_copy_from_buf(&payload, data, 5);
    z_owned_bytes_writer_t writer;
    z_bytes_writer_empty(&writer);
    z_bytes_writer_append(z_loan_mut(writer), z_move(payload));
    {
        z_owned_bytes_t b;
        z_bytes_copy_from_buf(&b, data + 5, 5);
        assert(z_bytes_writer_append(z_loan_mut(writer), z_bytes_move(&b)) == 0);
    }
    z_bytes_writer_finish(z_move(writer), &payload);

    z_bytes_reader_t reader = z_bytes_get_reader(z_loan(payload));
    z_bytes_reader_read(&reader, data_out, 10);

    assert(!memcmp(data, data_out, 10));

    uint8_t d;
    assert(0 == z_bytes_reader_read(&reader, &d, 1));  // we reached the end of the payload

    z_drop(z_move(payload));
}

void custom_deleter(void *data, void *context) {
    (void)data;
    size_t *cnt = (size_t *)context;
    (*cnt)++;
}

bool z_check_and_drop_payload(z_owned_bytes_t *payload, uint8_t *data, size_t len) {
    z_owned_slice_t out;
    z_bytes_to_slice(z_loan(*payload), &out);
    z_drop(z_move(*payload));
    bool res = memcmp(data, z_slice_data(z_loan(out)), len) == 0;
    z_drop(z_move(out));

    return res;
}

void test_slice(void) {
    uint8_t data[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};

    size_t cnt = 0;
    z_owned_bytes_t payload;
    z_bytes_from_buf(&payload, data, 10, custom_deleter, (void *)&cnt);

    z_owned_slice_t out;
    z_bytes_to_slice(z_loan(payload), &out);

    assert(cnt == 0);
    z_drop(z_move(payload));
    assert(cnt == 1);

    assert(!memcmp(data, z_slice_data(z_loan(out)), 10));
    z_slice_drop(z_slice_move(&out));

    z_owned_bytes_t payload2;
    z_owned_slice_t s;
    z_slice_copy_from_buf(&s, data, 10);
    z_bytes_copy_from_slice(&payload2, z_loan(s));
    assert(z_internal_slice_check(&s));
    z_slice_drop(z_slice_move(&s));
    assert(z_check_and_drop_payload(&payload2, data, 10));

    z_owned_bytes_t payload3;
    z_slice_copy_from_buf(&s, data, 10);
    z_bytes_from_slice(&payload3, z_slice_move(&s));
    assert(!z_internal_slice_check(&s));
    assert(z_check_and_drop_payload(&payload3, data, 10));

    z_owned_bytes_t payload4;
    z_bytes_copy_from_buf(&payload4, data, 10);
    assert(z_check_and_drop_payload(&payload4, data, 10));

    z_owned_bytes_t payload5;
    z_bytes_from_static_buf(&payload5, data, 10);
    assert(z_check_and_drop_payload(&payload5, data, 10));
}

#define TEST_ARITHMETIC(TYPE, EXT, VAL)              \
    {                                                \
        TYPE in = VAL, out;                          \
        z_owned_bytes_t payload;                     \
        ze_serialize_##EXT(&payload, in);            \
        ze_deserialize_##EXT(z_loan(payload), &out); \
        assert(in == out);                           \
        z_drop(z_move(payload));                     \
    }

void test_arithmetic(void) {
    TEST_ARITHMETIC(uint8_t, uint8, 5);
    TEST_ARITHMETIC(uint16_t, uint16, 1000);
    TEST_ARITHMETIC(uint32_t, uint32, 51000000);
    TEST_ARITHMETIC(uint64_t, uint64, 1000000000005);

    TEST_ARITHMETIC(int8_t, int8, 5);
    TEST_ARITHMETIC(int16_t, int16, -1000);
    TEST_ARITHMETIC(int32_t, int32, 51000000);
    TEST_ARITHMETIC(int64_t, int64, -1000000000005);

    TEST_ARITHMETIC(float, float, 10.1f);
    TEST_ARITHMETIC(double, double, -105.001);
}

bool check_slice(const z_loaned_bytes_t *b, const uint8_t *data, size_t len) {
    z_bytes_slice_iterator_t it = z_bytes_get_slice_iterator(b);
    uint8_t *data_out = (uint8_t *)malloc(len);
    z_view_slice_t v;
    size_t pos = 0;
    while (z_bytes_slice_iterator_next(&it, &v)) {
        const uint8_t *slice_data = z_slice_data(z_loan(v));
        size_t slice_len = z_slice_len(z_loan(v));
        memcpy(data_out + pos, slice_data, slice_len);
        pos += slice_len;
    }
    assert(pos == len);
    assert(memcmp(data, data_out, len) == 0);
    free(data_out);
    return true;
}

void test_slices(void) {
    uint8_t data[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    z_owned_bytes_t payload;
    z_bytes_copy_from_buf(&payload, data, 10);
    assert(check_slice(z_loan(payload), data, 10));

#if defined(Z_FEATURE_UNSTABLE_API)
    z_view_slice_t view;
    assert(z_bytes_get_contiguous_view(z_loan(payload), &view) == Z_OK);
    assert(z_slice_len(z_loan(view)) == 10);
    assert(memcmp(data, z_slice_data(z_loan(view)), 10) == 0);
#endif

    z_drop(z_move(payload));

    z_owned_bytes_writer_t writer;
    // possible multiple slices
    z_bytes_writer_empty(&writer);

    for (size_t i = 0; i < 10; i++) {
        z_owned_bytes_t b;
        z_bytes_copy_from_buf(&b, data + i, 1);
        z_bytes_writer_append(z_loan_mut(writer), z_bytes_move(&b));
    }
    z_bytes_writer_finish(z_move(writer), &payload);
    assert(check_slice(z_loan(payload), data, 10));
#if defined(Z_FEATURE_UNSTABLE_API)
    assert(z_bytes_get_contiguous_view(z_loan(payload), &view) != Z_OK);
#endif
    z_drop(z_move(payload));
}

void test_serialize_simple(void) {
    z_owned_bytes_t b;
    ze_owned_serializer_t serializer;
    ze_serializer_empty(&serializer);

    ze_serializer_serialize_double(z_loan_mut(serializer), 0.5);
    ze_serializer_serialize_int32(z_loan_mut(serializer), -1111);
    ze_serializer_serialize_str(z_loan_mut(serializer), "abc");
    ze_serializer_finish(z_move(serializer), &b);

    double d;
    int32_t i;
    z_owned_string_t s;

    ze_deserializer_t deserializer = ze_deserializer_from_bytes(z_loan(b));
    assert(!ze_deserializer_is_done(&deserializer));
    assert(ze_deserializer_deserialize_double(&deserializer, &d) == 0);
    assert(ze_deserializer_deserialize_int32(&deserializer, &i) == 0);
    assert(ze_deserializer_deserialize_string(&deserializer, &s) == 0);
    assert(ze_deserializer_is_done(&deserializer));

    assert(d == 0.5);
    assert(i == -1111);
    assert(strncmp("abc", z_string_data(z_loan(s)), z_string_len(z_loan(s))) == 0);

    z_drop(z_move(s));
    z_drop(z_move(b));
}

void test_serialize_sequence(void) {
    uint32_t input[6] = {1, 2, 3, 100, 10000, 100000};
    z_owned_bytes_t b;
    ze_owned_serializer_t serializer;
    ze_serializer_empty(&serializer);

    ze_serializer_serialize_sequence_length(z_loan_mut(serializer), 6);
    for (size_t i = 0; i < 6; ++i) {
        ze_serializer_serialize_uint32(z_loan_mut(serializer), input[i]);
    }
    ze_serializer_finish(z_move(serializer), &b);

    ze_deserializer_t deserializer = ze_deserializer_from_bytes(z_loan(b));
    assert(!ze_deserializer_is_done(&deserializer));
    size_t len = 0;
    assert(ze_deserializer_deserialize_sequence_length(&deserializer, &len) == 0);
    assert(len == 6);
    for (size_t i = 0; i < 6; i++) {
        uint32_t u = 0;
        assert(ze_deserializer_deserialize_uint32(&deserializer, &u) == 0);
        assert(u == input[i]);
    }
    assert(ze_deserializer_is_done(&deserializer));
    z_drop(z_move(b));
}

int main(void) {
    test_reader_seek();
    test_reader_read();
    test_writer();
    test_slice();
    test_arithmetic();
    test_append();
    test_slices();
    test_serialize_simple();
    test_serialize_sequence();
}
