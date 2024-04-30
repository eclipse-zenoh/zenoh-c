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

    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    if (z_open(&s, z_move(config))) {
        perror("Unable to open session!");
        return -1;
    }

    z_view_keyexpr_t ke;
    z_view_keyexpr_new(&ke, keyexpr);
    z_owned_publisher_t pub;
    if (z_declare_publisher(&pub, z_loan(s), z_loan(ke), NULL) < 0) {
        perror("Unable to declare Publisher for key expression!");
        return -1;
    }

    z_owned_slice_map_t map;
    z_slice_map_new(&map);
    z_view_slice_t k_const, v_const;
    z_view_slice_from_str(&k_const, K_CONST);
    z_view_slice_from_str(&v_const, V_CONST);
    z_slice_map_insert_by_copy(z_loan_mut(map), z_loan(k_const), z_loan(v_const));

    z_publisher_put_options_t options;
    z_publisher_put_options_default(&options);
    z_owned_bytes_t attachment;
    z_bytes_encode_from_slice_map(&attachment, z_loan(map));
    options.attachment = &attachment;
    for (int i = 0; i < values_count; ++i) {
        z_view_slice_t k_var, v_var;
        z_view_slice_from_str(&k_var, K_VAR);
        z_view_slice_from_str(&v_var, values[i]);
        z_slice_map_insert_by_copy(z_loan_mut(map), z_loan(k_var), z_loan(v_var));
        z_owned_bytes_t payload;
        z_bytes_encode_from_slice(&payload, z_loan(v_var));
        z_publisher_put(z_loan(pub), z_move(payload), &options);
    }

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));
    z_drop(z_move(map));
    return 0;
}

void data_handler(const z_loaned_sample_t *sample, void *arg) {
    static int val_num = 0;
    z_owned_str_t keystr;
    z_keyexpr_to_string(z_sample_keyexpr(sample), &keystr);
    if (strcmp(keyexpr, z_str_data(z_loan(keystr)))) {
        perror("Unexpected key received");
        exit(-1);
    }
    z_drop(z_move(keystr));

    z_owned_str_t payload_str;
    z_bytes_decode_into_string(z_sample_payload(sample), &payload_str);
    if (strcmp(values[val_num], z_str_data(z_loan(payload_str)))) {
        perror("Unexpected value received");
        z_drop(z_move(payload_str));
        exit(-1);
    }
    z_drop(z_move(payload_str));
    const z_loaned_bytes_t* attachment = z_sample_attachment(sample);
    if (attachment == NULL) {
        perror("Missing attachment!");
        exit(-1);
    }
    z_drop(z_move(keystr));

    z_owned_slice_map_t map;
    z_bytes_decode_into_slice_map(attachment, &map);

    z_view_slice_t k_const, k_var;
    z_view_slice_from_str(&k_const, K_CONST);
    z_view_slice_from_str(&k_var, K_CONST);

    const z_loaned_slice_t* v_const = z_slice_map_get(z_loan(map), z_loan(k_const));
    ASSERT_STR_SLICE_EQUAL(V_CONST, v_const);

    const z_loaned_slice_t* v_var = z_slice_map_get(z_loan(map), z_loan(k_var));
    ASSERT_STR_SLICE_EQUAL(values[val_num], v_var);

    z_drop(z_move(map));
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
    z_view_keyexpr_new(&ke, keyexpr);

    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, NULL);
    z_owned_subscriber_t sub;
   ;
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
