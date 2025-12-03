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
    z_owned_config_t config;
    z_config_default(&config);
    if (zc_config_insert_json5(z_loan_mut(config), Z_CONFIG_ADD_TIMESTAMP_KEY, "true") < 0) {
        perror("Unable to configure timestamps!");
        return -1;
    }

    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL), 0) {
        perror("Unable to open session!");
        return -1;
    }

    printf("Declaring AdvancedPublisher on '%s'...\n", keyexpr);
    ze_owned_advanced_publisher_t pub;
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);

    ze_advanced_publisher_options_t pub_opts;
    ze_advanced_publisher_options_default(&pub_opts);
    ze_advanced_publisher_cache_options_default(&pub_opts.cache);
    pub_opts.cache.max_samples = values_count;
    pub_opts.publisher_detection = true;
    pub_opts.sample_miss_detection.is_enabled = true;  // heartbeats are disabled by default

    if (ze_declare_advanced_publisher(z_loan(s), &pub, z_loan(ke), &pub_opts) < 0) {
        printf("Unable to declare AdvancedPublisher for key expression!\n");
        exit(-1);
    }

    // values for cache
    for (int i = 0; i < values_count / 2; ++i) {
        z_owned_bytes_t payload;
        z_bytes_from_static_str(&payload, values[i]);
        ze_advanced_publisher_put(z_loan(pub), z_move(payload), NULL);
    }

    SEM_POST(sem_pub);
    printf("wait: sem_sub\n");
    SEM_WAIT(sem_sub);
    z_sleep_s(1);

    // values for subscribe
    for (int i = values_count / 2; i < values_count; ++i) {
        z_owned_bytes_t payload;
        z_bytes_from_static_str(&payload, values[i]);
        ze_advanced_publisher_put(z_loan(pub), z_move(payload), NULL);
    }

    printf("wait: sem_sub\n");
    SEM_WAIT(sem_sub);

    z_drop(z_move(pub));
    z_drop(z_move(s));

    return 0;
}

void data_handler(z_loaned_sample_t *sample, void *arg) {
    static int val_num = 0;
    z_view_string_t keystr;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &keystr);
    if (strncmp(keyexpr, z_string_data(z_loan(keystr)), z_string_len(z_loan(keystr)))) {
        perror("Unexpected key received");
        exit(-1);
    }
    z_owned_string_t payload_str;
    z_bytes_to_string(z_sample_payload(sample), &payload_str);

    const z_source_info_t *info = z_sample_source_info(sample);
    assert(info != NULL);
    z_owned_string_t id_string;
    z_entity_global_id_t global_id = z_source_info_id(info);
    z_id_t z_id = z_entity_global_id_zid(&global_id);
    z_id_to_string(&z_id, &id_string);

    printf("Received: '%.*s'; id=%.*s; sn=%u\n", (int)z_string_len(z_loan(payload_str)),
           z_string_data(z_loan(payload_str)), (int)z_string_len(z_loan(id_string)), z_string_data(z_loan(id_string)),
           z_source_info_sn(info));
    ASSERT_STR_STRING_EQUAL(values[val_num], z_loan(payload_str));
    z_drop(z_move(payload_str));
    z_drop(z_move(id_string));

    printf("data_handler: %i\n", val_num);
    if (++val_num == values_count) {
        SEM_POST(sem_sub);
        exit(0);
    };
}

int run_subscriber() {
    printf("wait: sem_pub\n");
    SEM_WAIT(sem_pub);

    z_owned_config_t config;
    z_config_default(&config);

    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        perror("Unable to open session!");
        return -1;
    }

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);

    ze_advanced_subscriber_options_t sub_opts;
    ze_advanced_subscriber_options_default(&sub_opts);

    ze_advanced_subscriber_history_options_default(&sub_opts.history);
    sub_opts.history.detect_late_publishers = true;

    ze_advanced_subscriber_recovery_options_default(&sub_opts.recovery);
    ze_advanced_subscriber_last_sample_miss_detection_options_default(&sub_opts.recovery.last_sample_miss_detection);
    sub_opts.recovery.last_sample_miss_detection.periodic_queries_period_ms = 1000;
    sub_opts.subscriber_detection = true;

    z_owned_closure_sample_t callback;
    z_closure(&callback, data_handler, NULL, NULL);
    printf("Declaring AdvancedSubscriber on '%s'...\n", keyexpr);
    ze_owned_advanced_subscriber_t sub;
    if (ze_declare_advanced_subscriber(z_loan(s), &sub, z_loan(ke), z_move(callback), &sub_opts) < 0) {
        printf("Unable to declare advanced subscriber.\n");
        exit(-1);
    }

    SEM_POST(sem_sub);
    z_sleep_s(10);

    z_drop(z_move(sub));
    z_drop(z_move(s));

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
