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

void query_handler(const z_loaned_query_t *query, void *context) {
    static int value_num = 0;

    z_view_string_t params;
    z_query_parameters(query, &params);

    const z_loaned_bytes_t *attachment = z_query_attachment(query);
    if (attachment == NULL) {
        perror("Missing attachment!");
        exit(-1);
    }

    kv_pair_t kvs_in[2] = {(kv_pair_t){K_CONST, V_CONST}, (kv_pair_t){K_VAR, values[value_num]}};
    kv_it it = {.current = kvs_in, .end = kvs_in + 2};
    if (check_attachment(attachment, &it) != 0) {
        perror("Failed to validate attachment");
        exit(-1);
    }

    z_query_reply_options_t options;
    z_query_reply_options_default(&options);

    z_owned_bytes_t reply_attachment;
    kv_pair_t kvs_out[1] = {(kv_pair_t){K_CONST, V_CONST}};
    kv_it it_out = {.current = kvs_out, .end = kvs_out + 1};
    z_bytes_from_iter(&reply_attachment, create_attachment_iter, (void *)&it_out);

    options.attachment = &reply_attachment;

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
    if (z_open(&s, z_move(config)) < 0) {
        perror("Unable to open session!");
        return -1;
    }

    z_owned_closure_query_t callback;
    z_closure(&callback, query_handler, NULL, keyexpr);

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
    z_close(z_move(s));
    return 0;
}

int run_get() {
    SEM_WAIT(sem);

    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    if (z_open(&s, z_move(config)) < 0) {
        perror("Unable to open session!");
        return -1;
    }

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);

    z_get_options_t opts;
    z_get_options_default(&opts);

    kv_pair_t kvs[2];
    kvs[0] = (kv_pair_t){.key = K_CONST, .value = V_CONST};

    for (int val_num = 0; val_num < values_count; ++val_num) {
        kvs[1] = (kv_pair_t){.key = K_VAR, .value = values[val_num]};

        z_owned_fifo_handler_reply_t handler;
        z_owned_closure_reply_t closure;
        z_fifo_channel_reply_new(&closure, &handler, 16);

        z_owned_bytes_t attachment;
        kv_it it = {.current = kvs, .end = kvs + 2};
        z_bytes_from_iter(&attachment, create_attachment_iter, (void *)&it);

        opts.attachment = &attachment;
        z_get(z_loan(s), z_loan(ke), "", z_move(closure), &opts);
        z_owned_reply_t reply;
        for (z_recv(z_loan(handler), &reply); z_check(reply); z_recv(z_loan(handler), &reply)) {
            assert(z_reply_is_ok(z_loan(reply)));

            const z_loaned_sample_t *sample = z_reply_ok(z_loan(reply));
            z_owned_string_t payload_str;
            z_bytes_deserialize_into_string(z_sample_payload(sample), &payload_str);
            if (strncmp(values[val_num], z_string_data(z_loan(payload_str)), z_string_len(z_loan(payload_str)))) {
                perror("Unexpected value received");
                z_drop(z_move(payload_str));
                exit(-1);
            }

            const z_loaned_bytes_t *received_attachment = z_sample_attachment(sample);
            if (received_attachment == NULL) {
                perror("Missing attachment!");
                exit(-1);
            }

            kv_pair_t kvs_out[1] = {(kv_pair_t){K_CONST, V_CONST}};
            kv_it it_out = {.current = kvs_out, .end = kvs_out + 1};
            if (check_attachment(received_attachment, &it_out) != 0) {
                perror("Failed to validate attachment");
                exit(-1);
            }

            z_drop(z_move(payload_str));
        }
        z_drop(z_move(reply));
        z_drop(z_move(handler));
    }
    z_close(z_move(s));

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
