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

const char *const SEM_NAME = "/z_int_test_qerror_sync_sem";
sem_t *sem;

const char *const keyexpr = "test/key";
const char *const ERROR = "error message";

void query_handler(const z_query_t *query, void *context) {
    printf("query_handler\n");
    z_value_t value;

    value.payload = z_bytes_new(ERROR);

    z_query_reply_error(query, &value, NULL);

    exit(0);
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

void reply_handler(z_owned_reply_t *reply, void *arg) {
    printf("reply_handler\n");

    assert(!z_reply_is_ok(reply));

    z_value_t value = z_reply_err(reply);
    ASSERT_STR_BYTES_EQUAL(ERROR, value.payload);

    exit(0);
}

int run_get() {
    SEM_WAIT(sem);

    z_owned_config_t config = z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        perror("Unable to open session!");
        return -1;
    }

    z_get_options_t opts = z_get_options_default();
    z_owned_closure_reply_t closure_reply = z_closure(reply_handler, NULL, &s);
    z_get(z_loan(s), z_keyexpr(keyexpr), "", z_move(closure_reply), &opts);
    printf("z_get\n");

    sleep(10);

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
