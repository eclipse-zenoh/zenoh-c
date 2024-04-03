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

int run_publisher() {
    SEM_WAIT(sem);

    z_owned_config_t config = z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        perror("Unable to open session!");
        return -1;
    }

    z_publisher_options_t publisher_options = z_publisher_options_default();
    publisher_options.priority = Z_PRIORITY_DATA;
    publisher_options.congestion_control = Z_CONGESTION_CONTROL_BLOCK;
    z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(keyexpr), &publisher_options);
    if (!z_check(pub)) {
        perror("Unable to declare Publisher for key expression!");
        return -1;
    }

    for (int i = 0; i < values_count; ++i) {
        z_publisher_put_options_t options = z_publisher_put_options_default();
        options.encoding = z_encoding(Z_ENCODING_PREFIX_TEXT_PLAIN, NULL);
        zc_owned_payload_t payload = zc_payload_encode_from_string(values[i]);
        z_publisher_put(z_loan(pub), z_move(payload), &options);
    }

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));
    return 0;
}

void data_handler(const z_sample_t *sample, void *arg) {
    static int val_num = 0;
    z_owned_str_t keystr = z_keyexpr_to_string(z_sample_keyexpr(sample));
    if (strcmp(keyexpr, z_loan(keystr))) {
        perror("Unexpected key received");
        exit(-1);
    }
    z_drop(z_move(keystr));

    z_owned_str_t payload = zc_payload_decode_into_string(z_sample_payload(sample));
    if (strcmp(values[val_num], z_loan(payload))) {
        perror("Unexpected value received");
        z_drop(z_move(payload));
        exit(-1);
    }
    z_drop(z_move(payload));

    if (z_qos_get_congestion_control(z_sample_qos(sample)) != Z_CONGESTION_CONTROL_BLOCK ||
        z_qos_get_priority(z_sample_qos(sample)) != Z_PRIORITY_DATA) {
        perror("Unexpected QoS values");
        exit(-1);
    }

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
