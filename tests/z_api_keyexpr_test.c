//
// Copyright (c) 2022 ZettaScale Technology
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

#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

void canonize() {
    char keyexpr[256];
    int8_t err;
    uintptr_t len_old, len_new;

    strcpy(keyexpr, "a/**/**/c");
    len_old = len_new = strlen(keyexpr);
    printf("'%s', len = %lu -> ", keyexpr, len_old);
    err = z_keyexpr_canonize(keyexpr, &len_new);
    printf("'%s', len = %lu, err = %d\n", keyexpr, len_new, err);
    assert(err == 0);
    assert(len_new == len_old - 3);
    assert(strcmp(keyexpr, "a/**/c*/c") == 0);  // string not truncated, it's ok

    strcpy(keyexpr, "a/**/**/c");
    printf("'%s' -> ", keyexpr);
    err = z_keyexpr_canonize_null_terminated(keyexpr);
    printf("'%s', err = %d\n", keyexpr, err);
    assert(err == 0);
    assert(strcmp(keyexpr, "a/**/c") == 0);
}

void includes() {
    z_keyexpr_t nul = z_keyexpr(NULL);
    z_keyexpr_t foobar = z_keyexpr("foo/bar");
    z_keyexpr_t foostar = z_keyexpr("foo/*");
    assert(z_keyexpr_includes(foostar, foobar) == 0);
    assert(z_keyexpr_includes(foobar, foostar) == -1);
    assert(z_keyexpr_includes(nul, foobar) < -1);
    assert(z_keyexpr_includes(foobar, nul) < -1);
}

void intersects() {
    z_keyexpr_t nul = z_keyexpr(NULL);
    z_keyexpr_t foobar = z_keyexpr("foo/bar");
    z_keyexpr_t foostar = z_keyexpr("foo/*");
    z_keyexpr_t barstar = z_keyexpr("bar/*");
    assert(z_keyexpr_intersects(foostar, foobar) == 0);
    assert(z_keyexpr_intersects(barstar, foobar) == -1);
    assert(z_keyexpr_intersects(nul, foobar) < -1);
    assert(z_keyexpr_intersects(foobar, nul) < -1);
}

void undeclare() {
    z_owned_config_t config = z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    z_owned_keyexpr_t ke = z_declare_keyexpr(z_loan(s), z_keyexpr("test/thr"));
    assert(z_keyexpr_check(&ke));
    z_undeclare_keyexpr(z_loan(s), &ke);
    assert(!z_keyexpr_check(&ke));
}

int main(int argc, char **argv) {
    canonize();
    includes();
    intersects();
    undeclare();
}
