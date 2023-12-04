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

int run_success() { return 0; }

int run_failed() { return 1; }

int run_hanged() {
    while (1) {
        sleep(1000);
    };
    return 0;
}

void all_success() {
    func_ptr_t funcs[] = {run_success, run_success, run_success};
    assert(run_timeouted_test(funcs, 3, 10) == 0);
}

void all_failed() {
    func_ptr_t funcs[] = {run_failed, run_failed, run_failed};
    assert(run_timeouted_test(funcs, 3, 10) == -1);
}

void first_failed() {
    func_ptr_t funcs[] = {run_failed, run_success, run_success};
    assert(run_timeouted_test(funcs, 3, 10) == -1);
}

void last_failed() {
    func_ptr_t funcs[] = {run_success, run_success, run_failed};
    assert(run_timeouted_test(funcs, 3, 10) == -1);
}

void all_hanged() {
    func_ptr_t funcs[] = {run_hanged, run_hanged, run_hanged};
    assert(run_timeouted_test(funcs, 3, 1) == -1);
}

void one_hanged() {
    func_ptr_t funcs[] = {run_success, run_hanged, run_success};
    assert(run_timeouted_test(funcs, 3, 1) == -1);
}

int main() {
    all_success();
    all_failed();
    first_failed();
    last_failed();
    all_hanged();
    one_hanged();

    return 0;
}

#else
int main() { return 0; }
#endif  // VALID_PLATFORM
