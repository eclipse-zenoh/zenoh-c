//
// Copyright (c) 2026 ZettaScale Technology
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

#undef NDEBUG
#include <assert.h>

#include "zenoh.h"

static void count_zid(const z_id_t* zid, void* context) {
    assert(zid->id[0] == 1);
    ++*(int*)context;
}

static void count_log(zc_log_severity_t severity, const z_loaned_string_t* msg, void* context) {
    assert(severity == ZC_LOG_SEVERITY_INFO);
    assert(z_string_len(msg) == 3);
    ++*(int*)context;
}

static void count_drop(void* context) { ++*(int*)context; }

// z_call forwards every argument after the closure, whatever their number.
static void test_call(void) {
    int calls = 0;

    z_owned_closure_zid_t zid_closure;
    z_closure(&zid_closure, count_zid, count_drop, &calls);
    z_id_t zid = {{1}};
    z_call(z_loan(zid_closure), &zid);

    zc_owned_closure_log_t log_closure;
    z_closure(&log_closure, count_log, NULL, &calls);
    z_view_string_t msg;
    z_view_string_from_str(&msg, "msg");
    z_call(z_loan(log_closure), ZC_LOG_SEVERITY_INFO, z_loan(msg));

    assert(calls == 2);
    z_drop(z_move(zid_closure));
    z_drop(z_move(log_closure));
    assert(calls == 3);  // count_drop shares the counter with count_zid
}

int main(void) {
    test_call();
    return 0;
}
