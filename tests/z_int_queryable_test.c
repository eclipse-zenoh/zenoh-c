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

const uint32_t TEST_EID = 42;
const uint64_t TEST_SN = 24;
const uint64_t TEST_TS = 401706000;
const uint8_t TEST_ID = 123;

void query_handler(const z_loaned_query_t *query, void *context) {
    static int value_num = 0;

    z_view_string_t params;
    z_query_parameters(query, &params);

    z_query_reply_options_t options;
    z_query_reply_options_default(&options);

    // See https://github.com/eclipse-zenoh/zenoh/issues/1203
    // z_entity_global_id_t entity_global_id;
    // z_entity_global_id_new(&entity_global_id, &self_id, TEST_EID);
    // z_owned_source_info_t source_info;
    // z_source_info_new(&source_info, &entity_global_id, TEST_SN);

    // options.source_info = &source_info;

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

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr);
    z_owned_closure_query_t callback;
    z_closure(&callback, query_handler, NULL, keyexpr);
    z_owned_queryable_t qable;
    ;
    if (z_declare_queryable(&qable, z_loan(s), z_loan(ke), z_move(callback), NULL) != Z_OK) {
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
    z_view_keyexpr_from_str(&ke, keyexpr);

    for (int val_num = 0; val_num < values_count; ++val_num) {
        z_owned_fifo_handler_reply_t handler;
        z_owned_closure_reply_t closure;
        z_fifo_channel_reply_new(&closure, &handler, 16);
        z_get_options_t opts;
        z_get_options_default(&opts);
        z_get(z_loan(s), z_loan(ke), "", z_move(closure), &opts);
        z_owned_reply_t reply;
        for (z_result_t res = z_recv(z_loan(handler), &reply); res == Z_OK; res = z_recv(z_loan(handler), &reply)) {
            assert(z_reply_is_ok(z_loan(reply)));

            const z_loaned_sample_t *sample = z_reply_ok(z_loan(reply));
            z_owned_string_t payload_string;
            z_bytes_deserialize_into_string(z_sample_payload(sample), &payload_string);
            if (strncmp(values[val_num], z_string_data(z_loan(payload_string)), z_string_len(z_loan(payload_string)))) {
                perror("Unexpected value received");
                z_drop(z_move(payload_string));
                exit(-1);
            }

#if defined(UNSTABLE)
            const z_loaned_source_info_t *source_info = z_sample_source_info(sample);
            if (source_info == NULL) {
                perror("Unexpected null source_info");
                exit(-1);
            }
#endif
            // See https://github.com/eclipse-zenoh/zenoh/issues/1203
            // const uint64_t sn = z_source_info_sn(source_info);
            // if (sn != TEST_SN) {
            //     perror("Unexpected sn value");
            //     exit(-1);
            // }
            // const z_entity_global_id_t id = z_source_info_id(source_info);
            // uint32_t eid = z_entity_global_id_eid(&id);
            // if (eid != TEST_EID) {
            //     perror("Unexpected eid value");
            //     exit(-1);
            // }

            const z_timestamp_t *ts = z_sample_timestamp(sample);
            assert(ts == NULL);  // no timestmap was set by queryable

            // See https://github.com/eclipse-zenoh/zenoh/issues/1203
            // z_id_t ts_id = z_timestamp_id(ts);
            // z_id_t gloabl_id = z_entity_global_id_zid(&id);
            //
            // if (memcmp(ts_id.id, gloabl_id.id, sizeof(ts_id.id)) != 0) {
            //     perror("Timestamp id and global id differ");
            //     exit(-1);
            // }
            //
            // if (ts_id.id[0] != TEST_ID) {
            //     perror("Unexpected id value");
            //     exit(-1);
            // }

            z_drop(z_move(payload_string));
            z_drop(z_move(reply));
        }
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
