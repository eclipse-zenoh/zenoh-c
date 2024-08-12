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
    const char *key;
    const char *value;
} kv_pair_t;

typedef struct kv_it {
    kv_pair_t *current;
    kv_pair_t *end;
} kv_it;

bool create_attachment_iter(z_owned_bytes_t *kv_pair, void *context) {
    kv_it *it = (kv_it *)(context);
    if (it->current == it->end) {
        return false;
    }
    z_owned_bytes_t k, v;
    z_bytes_serialize_from_str(&k, it->current->key);
    z_bytes_serialize_from_str(&v, it->current->value);
    z_bytes_from_pair(kv_pair, z_move(k), z_move(v));
    it->current++;
    return true;
};

z_result_t check_attachment(const z_loaned_bytes_t *attachment, kv_it *it) {
    z_bytes_iterator_t iter = z_bytes_get_iterator(attachment);
    while (it->current != it->end) {
        z_owned_bytes_t kv, k, v;
        if (!z_bytes_iterator_next(&iter, &kv)) {
            perror("Not enough elements in the attachment\n");
            return -1;
        }
        if (z_bytes_deserialize_into_pair(z_loan(kv), &k, &v) != 0) {
            perror("Can not deserialize attachment elemnt into kv-pair\n");
            return -1;
        }
        z_owned_string_t k_str, v_str;
        z_bytes_deserialize_into_string(z_loan(k), &k_str);
        z_bytes_deserialize_into_string(z_loan(v), &v_str);

        if (strncmp(it->current->key, z_string_data(z_loan(k_str)), z_string_len(z_loan(k_str))) != 0) {
            perror("Incorrect attachment key\n");
            return -1;
        }
        if (strncmp(it->current->value, z_string_data(z_loan(v_str)), z_string_len(z_loan(v_str))) != 0) {
            perror("Incorrect attachment value\n");
            return -1;
        }
        z_drop(z_move(k_str));
        z_drop(z_move(v_str));
        z_drop(z_move(k));
        z_drop(z_move(v));
        z_drop(z_move(kv));
        it->current++;
    }
    return 0;
};

int run_publisher() {
    SEM_WAIT(sem);

    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    if (z_open(&s, z_move(config))) {
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

    kv_pair_t kvs[2];
    kvs[0] = (kv_pair_t){.key = K_CONST, .value = V_CONST};
    for (int i = 0; i < values_count; ++i) {
        kvs[1] = (kv_pair_t){.key = K_VAR, .value = values[i]};

        z_owned_bytes_t attachment;
        kv_it it = {kvs, kvs + 2};
        z_bytes_from_iter(&attachment, create_attachment_iter, (void *)&it);

        options.attachment = z_move(attachment);

        z_owned_bytes_t payload;
        z_bytes_from_static_str(&payload, values[i]);
        z_publisher_put(z_loan(pub), z_move(payload), &options);
    }

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));
    return 0;
}

void data_handler(const z_loaned_sample_t *sample, void *arg) {
    static int val_num = 0;
    z_view_string_t keystr;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &keystr);
    if (strncmp(keyexpr, z_string_data(z_loan(keystr)), z_string_len(z_loan(keystr)))) {
        perror("Unexpected key received");
        exit(-1);
    }

    z_owned_string_t payload_str;
    z_bytes_deserialize_into_string(z_sample_payload(sample), &payload_str);
    if (strncmp(values[val_num], z_string_data(z_loan(payload_str)), z_string_len(z_loan(payload_str)))) {
        perror("Unexpected value received");
        z_drop(z_move(payload_str));
        exit(-1);
    }
    z_drop(z_move(payload_str));
    const z_loaned_bytes_t *attachment = z_sample_attachment(sample);
    if (attachment == NULL) {
        perror("Missing attachment!");
        exit(-1);
    }

    kv_pair_t kvs[2] = {(kv_pair_t){K_CONST, V_CONST}, (kv_pair_t){K_VAR, values[val_num]}};
    kv_it it = {kvs, kvs + 2};

    if (check_attachment(attachment, &it) != 0) {
        perror("Failed to validate attachment");
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
    if (z_open(&s, z_move(config)) < 0) {
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
    z_close(z_move(s));

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
