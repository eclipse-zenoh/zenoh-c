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

int run_publisher() {
    SEM_WAIT(sem);

    z_owned_config_t config = z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        perror("Unable to open session!");
        return -1;
    }

    z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(keyexpr), NULL);
    if (!z_check(pub)) {
        perror("Unable to declare Publisher for key expression!");
        return -1;
    }

    z_owned_bytes_map_t map = z_bytes_map_new();
    z_bytes_map_insert_by_copy(&map, z_bytes_new(K_CONST), z_bytes_new(V_CONST));

    z_publisher_put_options_t options = z_publisher_put_options_default();
    options.encoding = z_encoding(Z_ENCODING_PREFIX_TEXT_PLAIN, NULL);
    options.attachment = z_bytes_map_as_attachment(&map);
    for (int i = 0; i < values_count; ++i) {
        z_bytes_map_insert_by_copy(&map, z_bytes_new(K_VAR), z_bytes_new(values[i]));
        z_publisher_put(z_loan(pub), (const uint8_t *)values[i], strlen(values[i]), &options);
    }

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));
    z_drop(z_move(map));
    return 0;
}

void data_handler(const z_sample_t *sample, void *arg) {
    static int val_num = 0;
    z_owned_str_t keystr = z_keyexpr_to_string(sample->keyexpr);
    if (strcmp(keyexpr, z_loan(keystr))) {
        perror("Unexpected key received");
        exit(-1);
    }
    z_drop(z_move(keystr));

    if (strncmp(values[val_num], (const char *)sample->payload.start, (int)sample->payload.len)) {
        perror("Unexpected value received");
        exit(-1);
    }

    z_bytes_t v_const = z_attachment_get(sample->attachment, z_bytes_new(K_CONST));
    ASSERT_STR_BYTES_EQUAL(V_CONST, v_const);

    z_bytes_t v_var = z_attachment_get(sample->attachment, z_bytes_new(K_VAR));
    ASSERT_STR_BYTES_EQUAL(values[val_num], v_var);

    if (++val_num == values_count) {
        exit(0);
    };
}

int run_subscriber() {
    z_owned_config_t config = z_config_default();

    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        perror("Unable to open session!");
        return -1;
    }

    z_owned_closure_sample_t callback = z_closure(data_handler);
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_keyexpr(keyexpr), z_move(callback), NULL);
    if (!z_check(sub)) {
        perror("Unable to declare subscriber!");
        return -1;
    }

    SEM_POST(sem);
    sleep(10);

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
