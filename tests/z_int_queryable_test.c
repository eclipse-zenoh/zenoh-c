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

void query_handler(const z_query_t *query, void *context) {
    static int value_num = 0;

    z_owned_str_t keystr = z_keyexpr_to_string(z_query_keyexpr(query));
    z_bytes_t pred = z_query_parameters(query);
    z_value_t payload_value = z_query_value(query);

    z_query_reply_options_t options = z_query_reply_options_default();
    options.encoding = z_encoding(Z_ENCODING_PREFIX_TEXT_PLAIN, NULL);
    z_query_reply(query, z_keyexpr((const char *)context), (const uint8_t *)values[value_num],
                  strlen(values[value_num]), &options);
    z_drop(z_move(keystr));

    if (++value_num == values_count) {
        exit(0);
    }
}

int run_queryable() {
    z_owned_config_t config = z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        perror("Unable to open session!");
        return -1;
    }

    z_owned_closure_query_t callback = z_closure(query_handler, NULL, keyexpr);
    z_owned_queryable_t qable = z_declare_queryable(z_loan(s), z_keyexpr(keyexpr), z_move(callback), NULL);
    if (!z_check(qable)) {
        printf("Unable to create queryable.\n");
        return -1;
    }

    SEM_POST(sem);
    sleep(10);

    z_drop(z_move(qable));
    z_close(z_move(s));
    return 0;
}

int run_get() {
    SEM_WAIT(sem);

    z_owned_config_t config = z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        perror("Unable to open session!");
        return -1;
    }

    for (int val_num = 0; val_num < values_count; ++val_num) {
        z_owned_reply_channel_t channel = zc_reply_fifo_new(16);
        z_get_options_t opts = z_get_options_default();
        z_get(z_loan(s), z_keyexpr(keyexpr), "", z_move(channel.send), &opts);
        z_owned_reply_t reply = z_reply_null();
        for (z_call(channel.recv, &reply); z_check(reply); z_call(channel.recv, &reply)) {
            assert(z_reply_is_ok(&reply));

            z_sample_t sample = z_reply_ok(&reply);
            z_owned_str_t keystr = z_keyexpr_to_string(sample.keyexpr);

            ASSERT_STR_BYTES_EQUAL(values[val_num], sample.payload);

            z_drop(z_move(keystr));
        }
        z_drop(z_move(reply));
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
