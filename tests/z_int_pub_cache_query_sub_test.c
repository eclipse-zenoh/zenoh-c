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

const char *const SEM_NAME_PUB = "/z_int_test_sync_sem_pub";
sem_t *sem_pub;
const char *const SEM_NAME_SUB = "/z_int_test_sync_sem_sub";
sem_t *sem_sub;

const char *const keyexpr = "test/key";
const char *const values[] = {"test_value_1", "test_value_2", "test_value_3",
                              "test_value_4", "test_value_5", "test_value_6"};
const size_t values_count = sizeof(values) / sizeof(values[0]);

int run_publisher() {
    z_owned_config_t config = z_config_default();
    if (zc_config_insert_json(z_loan(config), Z_CONFIG_ADD_TIMESTAMP_KEY, "true") < 0) {
        perror("Unable to configure timestamps!");
        return -1;
    }

    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        perror("Unable to open session!");
        return -1;
    }

    ze_publication_cache_options_t pub_cache_opts = ze_publication_cache_options_default();
    pub_cache_opts.history = 42;

    ze_owned_publication_cache_t pub_cache =
        ze_declare_publication_cache(z_loan(s), z_keyexpr(keyexpr), &pub_cache_opts);
    if (!z_check(pub_cache)) {
        perror("Unable to declare publication cache for key expression!\n");
        return -1;
    }

    z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(keyexpr), NULL);
    if (!z_check(pub)) {
        perror("Unable to declare Publisher for key expression!");
        return -1;
    }

    // values for cache
    for (int i = 0; i < values_count / 2; ++i) {
        z_put(z_loan(s), z_keyexpr(keyexpr), (const uint8_t *)values[i], strlen(values[i]), NULL);
    }

    SEM_POST(sem_pub);
    printf("wait: sem_sub\n");
    SEM_WAIT(sem_sub);

    // values for subscribe
    for (int i = values_count / 2; i < values_count; ++i) {
        z_put(z_loan(s), z_keyexpr(keyexpr), (const uint8_t *)values[i], strlen(values[i]), NULL);
    }

    printf("wait: sem_sub\n");
    SEM_WAIT(sem_sub);

    z_drop(z_move(pub_cache));
    z_drop(z_move(pub));
    z_close(z_move(s));

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

    ASSERT_STR_BYTES_EQUAL(values[val_num], sample->payload);

    printf("data_handler: %i\n", val_num);
    if (++val_num == values_count) {
        SEM_POST(sem_sub);
        exit(0);
    };
}

int run_subscriber() {
    printf("wait: sem_pub\n");
    SEM_WAIT(sem_pub);

    z_owned_config_t config = z_config_default();

    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        perror("Unable to open session!");
        return -1;
    }

    z_owned_closure_sample_t callback = z_closure(data_handler);
    ze_owned_querying_subscriber_t sub =
        ze_declare_querying_subscriber(z_loan(s), z_keyexpr(keyexpr), z_move(callback), NULL);
    if (!z_check(sub)) {
        perror("Unable to declare subscriber!");
        return -1;
    }

    SEM_POST(sem_sub);
    sleep(10);

    z_drop(z_move(sub));
    z_close(z_move(s));

    return -1;
}

int main() {
    SEM_INIT(sem_pub, SEM_NAME_PUB);
    SEM_INIT(sem_sub, SEM_NAME_SUB);

    func_ptr_t funcs[] = {run_publisher, run_subscriber};
    assert(run_timeouted_test(funcs, 2, 10) == 0);

    SEM_DROP(sem_pub, SEM_NAME_PUB);
    SEM_DROP(sem_sub, SEM_NAME_SUB);

    return 0;
}

#else
int main() { return 0; }
#endif  // VALID_PLATFORM
