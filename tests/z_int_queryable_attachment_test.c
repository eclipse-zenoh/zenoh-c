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

void query_handler(const z_loaned_query_t *query, void *context) {
    static int value_num = 0;

    z_view_str_t params;
    z_query_parameters(query, &params);
    const z_loaned_value_t* payload_value = z_query_value(query);

    const z_loaned_bytes_t* attachment = z_query_attachment(query);
    if (attachment == NULL) {
        perror("Missing attachment!");
        exit(-1);
    }

    z_view_slice_t k_const, k_var;
    z_view_slice_from_str(&k_const, K_CONST);
    z_view_slice_from_str(&k_var, K_VAR);

    z_owned_slice_map_t map;
    z_bytes_decode_into_slice_map(attachment, &map);

    const z_loaned_slice_t* v_const = z_slice_map_get(z_loan(map), z_loan(k_const));
    ASSERT_STR_SLICE_EQUAL(V_CONST, v_const);

    const z_loaned_slice_t* v_var = z_slice_map_get(z_loan(map), z_loan(k_var));
    ASSERT_STR_SLICE_EQUAL(values[value_num], v_var); 

    z_owned_slice_map_t reply_map;
    z_slice_map_new(&reply_map);
    z_slice_map_insert_by_copy(z_loan_mut(reply_map), z_loan(k_const), v_const);

    z_query_reply_options_t options;
    z_query_reply_options_default(&options);

    z_owned_bytes_t reply_attachment;
    z_bytes_encode_from_slice_map(&reply_attachment, z_loan(reply_map));
    options.attachment = &reply_attachment;

    z_owned_bytes_t payload;
    z_bytes_encode_from_string(&payload, values[value_num]);

    z_view_keyexpr_t reply_ke;
    z_view_keyexpr_from_string(&reply_ke, (const char *)context);
    z_query_reply(query, z_loan(reply_ke), z_move(payload), &options);
    z_drop(z_move(map));

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
    z_view_keyexpr_from_string(&ke, keyexpr);

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
    z_view_keyexpr_from_string(&ke, keyexpr);

    z_owned_slice_map_t map;
    z_slice_map_new(&map);

    z_view_slice_t k_const, k_var, v_const;
    z_view_slice_from_str(&k_const, K_CONST);
    z_view_slice_from_str(&k_var, K_VAR);
    z_view_slice_from_str(&v_const, V_CONST);

    z_slice_map_insert_by_copy(z_loan_mut(map), z_loan(k_const), z_loan(v_const));

    z_get_options_t opts;
    z_get_options_default(&opts);


    for (int val_num = 0; val_num < values_count; ++val_num) {
        z_view_slice_t v_var;
        z_view_slice_from_str(&v_var, values[val_num]);
        z_slice_map_insert_by_copy(z_loan_mut(map), z_loan(k_var), z_loan(v_var)); // will overwrite any previous value for the same key

        z_owned_reply_channel_t channel;
        zc_reply_fifo_new(&channel, 16);

        z_owned_bytes_t attachment;
        z_bytes_encode_from_slice_map(&attachment, z_loan(map));
        opts.attachment = &attachment;
        z_get(z_loan(s), z_loan(ke), "", z_move(channel.send), &opts);
        z_owned_reply_t reply;
        for (z_call(channel.recv, &reply); z_check(reply); z_call(channel.recv, &reply)) {
            assert(z_reply_is_ok(z_loan(reply)));

            const z_loaned_sample_t* sample = z_reply_ok(z_loan(reply));
            z_owned_str_t payload_str;
            z_bytes_decode_into_string(z_sample_payload(sample), &payload_str);
            if (strncmp(values[val_num], z_str_data(z_loan(payload_str)), z_str_len(z_loan(payload_str)))) {
                perror("Unexpected value received");
                z_drop(z_move(payload_str));
                exit(-1);
            }

            const z_loaned_bytes_t* received_attachment = z_sample_attachment(sample);
            if (received_attachment == NULL) {
                perror("Missing attachment!");
                exit(-1);
            }
            z_owned_slice_map_t received_map;
            z_bytes_decode_into_slice_map(received_attachment, &received_map);
            const z_loaned_slice_t* v_const_get = z_slice_map_get(z_loan(received_map), z_loan(k_const));
            ASSERT_STR_SLICE_EQUAL(V_CONST, v_const_get);

            z_drop(z_move(payload_str));
            z_drop(z_move(received_map));
        }
        z_drop(z_move(reply));
        z_drop(z_move(channel));
    }
    z_close(z_move(s));
    z_drop(z_move(map));

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
