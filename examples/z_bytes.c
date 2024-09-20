//
// Copyright (c) 2024 ZettaScale Technology
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
//

#include <ctype.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <zenoh.h>

#undef NDEBUG

typedef struct kv_pair_t {
    int32_t key;
    z_owned_string_t value;
} kv_pair_t;

typedef struct kv_it {
    kv_pair_t *current;
    kv_pair_t *end;
} kv_it;

typedef struct int32_it {
    int32_t *current;
    int32_t *end;
} int32_it;

static bool kv_pairs_iter(z_owned_bytes_t *kv_pair, void *context);
static bool int32_iter(z_owned_bytes_t *b, void *context);
static void print_slice_data(z_view_slice_t *slice);

int main(void) {
    z_owned_bytes_t payload;

    // Number types: uint8, uint16, uint32, uint64, int8, int16, int32, int64, float, double
    uint32_t input_u32 = 123456;
    uint32_t output_u32 = 0;
    z_bytes_serialize_from_uint32(&payload, input_u32);
    z_bytes_deserialize_into_uint32(z_loan(payload), &output_u32);
    assert(input_u32 == output_u32);
    z_drop(z_move(payload));
    // Corresponding encoding to be used in operations options like `z_put()`, `z_get()`, etc.
    // encoding = z_encoding_zenoh_uint32();

    // String, also work with and z_owned_string_t
    const char *input_str = "test";
    z_owned_string_t output_string;
    z_bytes_serialize_from_str(&payload, input_str);
    z_bytes_deserialize_into_string(z_loan(payload), &output_string);
    assert(strncmp(input_str, z_string_data(z_loan(output_string)), strlen(input_str)) == 0);
    z_drop(z_move(payload));
    z_drop(z_move(output_string));
    // Corresponding encoding to be used in operations options like `z_put()`, `z_get()`, etc.
    // encoding = z_encoding_zenoh_string();

    // Bytes, also work with z_owned_slice_t
    const uint8_t input_bytes[] = {1, 2, 3, 4};
    z_owned_slice_t output_bytes;
    z_bytes_serialize_from_buf(&payload, input_bytes, sizeof(input_bytes));
    z_bytes_deserialize_into_slice(z_loan(payload), &output_bytes);
    assert(memcmp(input_bytes, z_slice_data(z_loan(output_bytes)), sizeof(input_bytes)) == 0);
    z_drop(z_move(payload));
    z_drop(z_move(output_bytes));
    // Corresponding encoding to be used in operations options like `z_put()`, `z_get()`, etc.
    // encoding = z_encoding_zenoh_bytes(); // (the default value)

    // Writer reader
    uint8_t input_writer[] = {0, 1, 2, 3, 4};
    uint8_t output_reader[5] = {0};
    z_bytes_empty(&payload);
    z_bytes_writer_t writer = z_bytes_get_writer(z_bytes_loan_mut(&payload));
    z_bytes_writer_write_all(&writer, input_writer, 3);
    z_bytes_writer_write_all(&writer, input_writer + 3, 2);
    z_bytes_reader_t reader = z_bytes_get_reader(z_bytes_loan(&payload));
    z_bytes_reader_read(&reader, output_reader, sizeof(output_reader));
    assert(0 == memcmp(input_writer, output_reader, sizeof(output_reader)));
    z_drop(z_move(payload));

    // Bytes iterator
    int32_t input_values[] = {0, 1, 2, 3, 4};
    int32_t output_values[5] = {0};
    int32_it values_iter = (int32_it){.current = input_values, .end = input_values + 5};
    z_bytes_from_iter(&payload, int32_iter, (void *)(&values_iter));
    z_bytes_iterator_t it = z_bytes_get_iterator(z_bytes_loan(&payload));

    z_owned_bytes_t current_item;
    size_t i = 0;
    while (z_bytes_iterator_next(&it, &current_item)) {
        z_bytes_deserialize_into_int32(z_loan(current_item), &output_values[i]);
        z_bytes_drop(z_bytes_move(&current_item));
        i++;
    }
    for (size_t i = 0; i < 5; ++i) {
        assert(input_values[i] == output_values[i]);
    }
    z_drop(z_move(payload));

    // Key value pairs
    kv_pair_t kv_pairs_input[2];
    kv_pairs_input[0].key = 0;
    z_string_copy_from_str(&kv_pairs_input[0].value, "value_0");
    kv_pairs_input[1].key = 1;
    z_string_copy_from_str(&kv_pairs_input[1].value, "value_1");
    kv_it it_kv = {.current = kv_pairs_input, .end = kv_pairs_input + 2};
    z_bytes_from_iter(&payload, kv_pairs_iter, (void *)&it_kv);

    kv_pair_t kv_pairs_output[2];
    size_t out_idx = 0;
    z_bytes_iterator_t kv_pairs_iterator = z_bytes_get_iterator(z_loan(payload));
    z_owned_bytes_t kv, first, second;
    while (z_bytes_iterator_next(&kv_pairs_iterator, &kv)) {
        assert(out_idx < 2);
        z_bytes_deserialize_into_pair(z_loan(kv), &first, &second);
        z_bytes_deserialize_into_int32(z_loan(first), &kv_pairs_output[out_idx].key);
        z_bytes_deserialize_into_string(z_loan(second), &kv_pairs_output[out_idx].value);
        z_bytes_drop(z_bytes_move(&first));
        z_bytes_drop(z_bytes_move(&second));
        z_bytes_drop(z_bytes_move(&kv));
        out_idx++;
    }
    assert(out_idx == 2);
    for (size_t i = 0; i < 2; i++) {
        assert(kv_pairs_output[i].key == kv_pairs_input[i].key);
        assert(z_string_len(z_loan(kv_pairs_output[i].value)) == z_string_len(z_loan(kv_pairs_input[i].value)));
        assert(strncmp(z_string_data(z_loan(kv_pairs_output[i].value)), z_string_data(z_loan(kv_pairs_input[i].value)),
                       z_string_len(z_loan(kv_pairs_output[i].value))) == 0);
    }

    z_drop(z_move(payload));
    for (size_t i = 0; i < 2; i++) {
        z_drop(z_move(kv_pairs_output[i].value));
        z_drop(z_move(kv_pairs_input[i].value));
    }

    // Slice iterator
    values_iter = (int32_it){.current = input_values, .end = input_values + 5};
    z_bytes_from_iter(&payload, int32_iter, (void *)(&values_iter));

    z_bytes_slice_iterator_t slice_iter = z_bytes_get_slice_iterator(z_bytes_loan(&payload));
    z_view_slice_t curr_slice;
    while (z_bytes_slice_iterator_next(&slice_iter, &curr_slice)) {
        printf("slice len: %d, slice data: '", (int)z_slice_len(z_view_slice_loan(&curr_slice)));
        print_slice_data(&curr_slice);
        printf("'\n");
    }
    z_drop(z_move(payload));

    return 0;
}

static bool int32_iter(z_owned_bytes_t *b, void *context) {
    int32_it *it = (int32_it *)(context);
    if (it->current == it->end) {
        return false;
    }
    z_bytes_serialize_from_int32(b, *it->current);
    it->current++;
    return true;
}

bool kv_pairs_iter(z_owned_bytes_t *kv_pair, void *context) {
    kv_it *it = (kv_it *)(context);
    if (it->current == it->end) {
        return false;
    }
    z_owned_bytes_t k, v;
    z_bytes_serialize_from_int32(&k, it->current->key);
    z_bytes_serialize_from_str(&v, z_string_data(z_loan(it->current->value)));
    z_bytes_from_pair(kv_pair, z_move(k), z_move(v));
    it->current++;
    return true;
};

static void print_slice_data(z_view_slice_t *slice) {
    for (size_t i = 0; i < z_slice_len(z_view_slice_loan(slice)); i++) {
        printf("0x%02x ", z_slice_data(z_view_slice_loan(slice))[i]);
    }
}
