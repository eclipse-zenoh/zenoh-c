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

#include "zenoh.h"

const char *const SEM_NAME = "/z_int_test_queryable_sync_sem";
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
#if defined(Z_FEATURE_UNSTABLE_API)
    if (attachment == NULL) {
        perror("Missing attachment!");
        return -1;
    }

    ze_deserializer_t deserializer = ze_deserializer(attachment);
    size_t received_len = 0;
    ze_deserializer_deserialize_sequence_begin(&deserializer, &received_len);
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
    ze_deserializer_deserialize_sequence_end(&deserializer);

    drop_attachment(kvs, len);
#endif
    return 0;
}

void query_handler(z_loaned_query_t *query, void *context) {
    static int value_num = 0;

    z_view_string_t params;
    z_query_parameters(query, &params);
    kv_pair_t kvs_in[2];
    z_string_copy_from_str(&kvs_in[0].key, K_CONST);
    z_string_copy_from_str(&kvs_in[0].value, V_CONST);
    z_string_copy_from_str(&kvs_in[1].key, K_VAR);
    z_string_copy_from_str(&kvs_in[1].value, values[value_num]);

    if (check_attachment(kvs_in, 2, z_query_attachment(query)) != 0) {
        perror("Failed to validate query attachment");
        exit(-1);
    }

    z_query_reply_options_t options;
    z_query_reply_options_default(&options);

#if defined(Z_FEATURE_UNSTABLE_API)
    kv_pair_t kvs[1];
    z_string_copy_from_str(&kvs[0].key, K_CONST);
    z_string_copy_from_str(&kvs[0].value, V_CONST);

    z_owned_bytes_t reply_attachment;
    z_bytes_empty(&reply_attachment);
    ze_serializer_t serializer = ze_serializer(z_loan_mut(reply_attachment));
    ze_serializer_serialize_sequence_begin(&serializer, 1);
    for (size_t i = 0; i < 1; i++) {
        ze_serializer_serialize_string(&serializer, z_loan(kvs[i].key));
        ze_serializer_serialize_string(&serializer, z_loan(kvs[i].value));
    }
    ze_serializer_serialize_sequence_end(&serializer);

    options.attachment = z_move(reply_attachment);
    drop_attachment(kvs, 1);
#endif

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, values[value_num]);

    z_view_keyexpr_t reply_ke;
    z_view_keyexpr_from_str(&reply_ke, (const char *)context);
    z_query_reply(query, z_loan(reply_ke), z_move(payload), &options);

    if (++value_num == values_count) {
        exit(0);
    }
}

int run_queryable() {
    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        perror("Unable to open session!");
        return -1;
    }

    z_owned_closure_query_t callback;
    z_closure(&callback, query_handler, NULL, (void *)keyexpr);

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);

    z_owned_queryable_t qable;
    if (z_declare_queryable(&qable, z_loan(s), z_loan(ke), z_move(callback), NULL) < 0) {
        printf("Unable to create queryable.\n");
        return -1;
    }

    SEM_POST(sem);
    z_sleep_s(10);

    z_undeclare_queryable(z_move(qable));
    z_close(z_move(s), NULL);
    return 0;
}

int run_get() {
    SEM_WAIT(sem);

    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        perror("Unable to open session!");
        return -1;
    }

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);

    z_get_options_t opts;
    z_get_options_default(&opts);

    for (int val_num = 0; val_num < values_count; ++val_num) {
        z_owned_fifo_handler_reply_t handler;
        z_owned_closure_reply_t closure;
        z_fifo_channel_reply_new(&closure, &handler, 16);

#if defined(Z_FEATURE_UNSTABLE_API)
        kv_pair_t kvs[2];
        z_string_copy_from_str(&kvs[0].key, K_CONST);
        z_string_copy_from_str(&kvs[0].value, V_CONST);
        z_string_copy_from_str(&kvs[1].key, K_VAR);
        z_string_copy_from_str(&kvs[1].value, values[val_num]);

        z_owned_bytes_t attachment;
        z_bytes_empty(&attachment);
        ze_serializer_t serializer = ze_serializer(z_loan_mut(attachment));
        ze_serializer_serialize_sequence_begin(&serializer, 2);
        for (size_t i = 0; i < 2; i++) {
            ze_serializer_serialize_string(&serializer, z_loan(kvs[i].key));
            ze_serializer_serialize_string(&serializer, z_loan(kvs[i].value));
        }
        ze_serializer_serialize_sequence_end(&serializer);

        opts.attachment = z_move(attachment);
        drop_attachment(kvs, 2);
#endif
        z_get(z_loan(s), z_loan(ke), "", z_move(closure), &opts);
        z_owned_reply_t reply;
        for (z_result_t res = z_recv(z_loan(handler), &reply); res == Z_OK; res = z_recv(z_loan(handler), &reply)) {
            assert(z_reply_is_ok(z_loan(reply)));

            const z_loaned_sample_t *sample = z_reply_ok(z_loan(reply));
            z_owned_string_t payload_str;
            z_bytes_to_string(z_sample_payload(sample), &payload_str);
            if (strncmp(values[val_num], z_string_data(z_loan(payload_str)), z_string_len(z_loan(payload_str)))) {
                perror("Unexpected value received");
                z_drop(z_move(payload_str));
                exit(-1);
            }

            kv_pair_t kvs_reply[1];
            z_string_copy_from_str(&kvs_reply[0].key, K_CONST);
            z_string_copy_from_str(&kvs_reply[0].value, V_CONST);
            if (check_attachment(kvs_reply, 1, z_sample_attachment(sample)) != 0) {
                perror("Failed to validate reply attachment!");
                exit(-1);
            }

            z_drop(z_move(payload_str));
        }
        z_drop(z_move(reply));
        z_drop(z_move(handler));
    }
    z_close(z_move(s), NULL);

    return 0;
}

int main() {
    SEM_INIT(sem, SEM_NAME);

    func_ptr_t funcs[] = {run_queryable, run_get};
    assert(run_timeouted_test(funcs, 2, 10) == 0);

    SEM_DROP(sem, SEM_NAME);

    return 0;
}

#else
int main() { return 0; }
#endif  // VALID_PLATFORM
