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

void query_handler(const z_loaned_query_t *query, void *context) {
    static int value_num = 0;

    z_owned_str_t key_string;
    z_keyexpr_to_string(z_query_keyexpr(query), &key_string);
    z_view_slice_t params;
    z_query_parameters(query, &params);
    const z_loaned_value_t* payload_value = z_query_value(query);

    z_query_reply_options_t options;
    z_query_reply_options_default(&options);
    z_view_str_t value_str;
    z_view_str_wrap(&value_str, values[value_num]);
    z_owned_bytes_t payload;
    z_bytes_encode_from_string(&payload, z_loan(value_str));

    z_view_keyexpr_t reply_ke;
    z_view_keyexpr_new(&reply_ke, (const char*)context);
    z_query_reply(query, z_loan(reply_ke), z_move(payload), &options);
    z_drop(z_move(key_string));

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

    z_view_keyexpr_t ke;
    z_view_keyexpr_new(&ke, keyexpr);
    z_owned_closure_query_t callback;
    z_closure(&callback, query_handler, NULL, keyexpr);
    z_owned_queryable_t qable;
    z_declare_queryable(&qable, z_loan(s), z_loan(ke), z_move(callback), NULL);
    if (!z_check(qable)) {
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
    if (z_open(&s, z_move(config))) {
        perror("Unable to open session!");
        return -1;
    }

    z_view_keyexpr_t ke;
    z_view_keyexpr_new(&ke, keyexpr);

    for (int val_num = 0; val_num < values_count; ++val_num) {
        z_owned_reply_channel_t channel;
        zc_reply_fifo_new(&channel, 16);
        z_get_options_t opts;
        z_get_options_default(&opts);
        z_get(z_loan(s), z_loan(ke), "", z_move(channel.send), &opts);
        z_owned_reply_t reply;
        for (z_call(channel.recv, &reply); z_check(reply); z_call(channel.recv, &reply)) {
            assert(z_reply_is_ok(z_loan(reply)));

            const z_loaned_sample_t* sample = z_reply_ok(z_loan(reply));
            z_owned_str_t key_string;
            z_keyexpr_to_string(z_sample_keyexpr(sample), &key_string);
            z_owned_str_t payload_string;
            z_bytes_decode_into_string(z_sample_payload(sample), &payload_string);
            if (strcmp(values[val_num], z_str_data(z_loan(payload_string)))) {
                perror("Unexpected value received");
                z_drop(z_move(payload_string));
                exit(-1);
            }

            z_drop(z_move(key_string));
            z_drop(z_move(payload_string));
            z_drop(z_move(reply));
        }
        z_drop(z_move(channel));
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
