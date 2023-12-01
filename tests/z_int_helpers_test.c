#include <string.h>

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

int all_success() {
    func_ptr_t funcs[] = {run_success, run_success, run_success};
    assert(run_timeouted_test(funcs, 3, 10) == 0);
}

int all_failed() {
    func_ptr_t funcs[] = {run_failed, run_failed, run_failed};
    assert(run_timeouted_test(funcs, 3, 10) == -1);
}

int first_failed() {
    func_ptr_t funcs[] = {run_failed, run_success, run_success};
    assert(run_timeouted_test(funcs, 3, 10) == -1);
}

int last_failed() {
    func_ptr_t funcs[] = {run_success, run_success, run_failed};
    assert(run_timeouted_test(funcs, 3, 10) == -1);
}

int all_hanged() {
    func_ptr_t funcs[] = {run_hanged, run_hanged, run_hanged};
    assert(run_timeouted_test(funcs, 3, 1) == -1);
}

int one_hanged() {
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
}

#endif  // VALID_PLATFORM
