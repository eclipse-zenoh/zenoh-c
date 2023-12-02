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

void main() {
    all_success();
    all_failed();
    first_failed();
    last_failed();
    all_hanged();
    one_hanged();
}

#endif  // VALID_PLATFORM
