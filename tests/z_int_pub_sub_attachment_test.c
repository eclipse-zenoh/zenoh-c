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
//

#include "z_int_helpers.h"

#ifdef VALID_PLATFORM

#include <string.h>

#include "zenoh.h"

const char *const SEM_NAME = "/z_int_test_sync_sem";
sem_t *sem;

const char *const keyexpr = "test/key";
const char *const values[] = {"test_value_1", "test_value_2", "test_value_3"};
const size_t values_count = sizeof(values) / sizeof(values[0]);

const char *const K_VAR = "k_var";
const char *const K_CONST = "k_const";
const char *const V_CONST = "v const";

typedef struct kv_pair_t {
    z_owned_string_t key;
    z_owned_string_t value;
} kv_pair_t;

void drop_attachment(kv_pair_t *kvp, size_t len) {
    for (size_t i = 0; i < len; i++) {
        z_drop(z_move(kvp[i].key));
        z_drop(z_move(kvp[i].value));
    }
}

z_result_t check_attachment(kv_pair_t *kvs, size_t len, const z_loaned_bytes_t *attachment) {
    if (attachment == NULL) {
        perror("Missing attachment!");
        return -1;
    }

    ze_deserializer_t deserializer = ze_deserializer_from_bytes(attachment);
    size_t received_len = 0;
    ze_deserializer_deserialize_sequence_length(&deserializer, &received_len);
    if (received_len != len) {
        perror("Incorrect attachment size!");
        return -1;
    }
    z_owned_string_t key, value;
    for (size_t i = 0; i < len; i++) {
        ze_deserializer_deserialize_string(&deserializer, &key);
        ze_deserializer_deserialize_string(&deserializer, &value);
        if (strncmp(z_string_data(z_loan(key)), z_string_data(z_loan(kvs[i].key)), z_string_len(z_loan(kvs[i].key))) !=
            0) {
            perror("Incorrect attachment key");
            z_drop(z_move(key));
            return -1;
        }
        z_drop(z_move(key));
        if (strncmp(z_string_data(z_loan(value)), z_string_data(z_loan(kvs[i].value)),
                    z_string_len(z_loan(kvs[i].value))) != 0) {
            perror("Incorrect attachment value");
            z_drop(z_move(value));
            return -1;
        }
        z_drop(z_move(value));
    }

    drop_attachment(kvs, len);
    return 0;
}

int run_publisher() {
    SEM_WAIT(sem);

    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL)) {
        perror("Unable to open session!");
        return -1;
    }

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);
    z_owned_publisher_t pub;
    if (z_declare_publisher(&pub, z_loan(s), z_loan(ke), NULL) < 0) {
        perror("Unable to declare Publisher for key expression!");
        return -1;
    }

    z_publisher_put_options_t options;
    z_publisher_put_options_default(&options);

    for (int i = 0; i < values_count; ++i) {
        kv_pair_t kvs[2];
        z_string_copy_from_str(&kvs[0].key, K_CONST);
        z_string_copy_from_str(&kvs[0].value, V_CONST);
        z_string_copy_from_str(&kvs[1].key, K_VAR);
        z_string_copy_from_str(&kvs[1].value, values[i]);

        z_owned_bytes_t attachment;
        ze_owned_serializer_t serializer;
        ze_serializer_empty(&serializer);
        ze_serializer_serialize_sequence_length(z_loan_mut(serializer), 2);
        for (size_t j = 0; j < 2; j++) {
            ze_serializer_serialize_string(z_loan_mut(serializer), z_loan(kvs[j].key));
            ze_serializer_serialize_string(z_loan_mut(serializer), z_loan(kvs[j].value));
        }
        ze_serializer_finish(z_move(serializer), &attachment);

        options.attachment = z_move(attachment);
        drop_attachment(kvs, 2);
        z_owned_bytes_t payload;
        z_bytes_from_static_str(&payload, values[i]);
        z_publisher_put(z_loan(pub), z_move(payload), &options);
    }

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s), NULL);
    return 0;
}

void data_handler(z_loaned_sample_t *sample, void *arg) {
    static int val_num = 0;
    z_view_string_t keystr;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &keystr);
    if (strncmp(keyexpr, z_string_data(z_loan(keystr)), z_string_len(z_loan(keystr)))) {
        perror("Unexpected key received");
        exit(-1);
    }

    z_owned_string_t payload_str;
    z_bytes_to_string(z_sample_payload(sample), &payload_str);
    if (strncmp(values[val_num], z_string_data(z_loan(payload_str)), z_string_len(z_loan(payload_str)))) {
        perror("Unexpected value received");
        z_drop(z_move(payload_str));
        exit(-1);
    }
    z_drop(z_move(payload_str));

    kv_pair_t kvs[2];
    z_string_copy_from_str(&kvs[0].key, K_CONST);
    z_string_copy_from_str(&kvs[0].value, V_CONST);
    z_string_copy_from_str(&kvs[1].key, K_VAR);
    z_string_copy_from_str(&kvs[1].value, values[val_num]);

    if (check_attachment(kvs, 2, z_sample_attachment(sample)) != 0) {
        perror("Failed to validate attachment!");
        exit(-1);
    }

    if (++val_num == values_count) {
        exit(0);
    };
}

int run_subscriber() {
    z_owned_config_t config;
    z_config_default(&config);

    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        perror("Unable to open session!");
        return -1;
    }

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);

    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, NULL);
    z_owned_subscriber_t sub;

    if (z_declare_subscriber(&sub, z_loan(s), z_loan(ke), z_move(callback), NULL) < 0) {
        perror("Unable to declare subscriber!");
        return -1;
    }

    SEM_POST(sem);
    z_sleep_s(10);

    z_undeclare_subscriber(z_move(sub));
    z_close(z_move(s), NULL);

    return -1;
}

int main() {
    SEM_INIT(sem, SEM_NAME);

    func_ptr_t funcs[] = {run_publisher, run_subscriber};
    assert(run_timeouted_test(funcs, 2, 10) == 0);

    SEM_DROP(sem, SEM_NAME);

    return 0;
}

#else
int main() { return 0; }
#endif  // VALID_PLATFORM
