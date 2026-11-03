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
    size_t len_old, len_new;

    strcpy(keyexpr, "a/**/**/c");
    len_old = len_new = strlen(keyexpr);
    printf("'%s', len = %zu -> ", keyexpr, len_old);
    err = z_keyexpr_canonize(keyexpr, &len_new);
    printf("'%s', len = %zu, err = %d\n", keyexpr, len_new, err);
    assert(err == 0);
    assert(len_new == len_old - 3);
    assert(strcmp(keyexpr, "a/**/c") == 0);

    strcpy(keyexpr, "a/**/**/c");
    printf("'%s' -> ", keyexpr);
    err = z_keyexpr_canonize_null_terminated(keyexpr);
    printf("'%s', err = %d\n", keyexpr, err);
    assert(err == 0);
    assert(strcmp(keyexpr, "a/**/c") == 0);

    strcpy(keyexpr, "a/**/**/c");
    z_view_keyexpr_t key_expr_canonized;
    z_view_keyexpr_from_str_autocanonize(&key_expr_canonized, keyexpr);
    assert(z_view_keyexpr_is_empty(&key_expr_canonized) == false);
    assert(strcmp(keyexpr, "a/**/c") == 0);
    z_view_string_t key_exp_canonized_bytes;
    z_keyexpr_as_view_string(z_loan(key_expr_canonized), &key_exp_canonized_bytes);
    assert(z_string_len(z_loan(key_exp_canonized_bytes)) == len_new);
    assert(strncmp(z_string_data(z_loan(key_exp_canonized_bytes)), "a/**/c", len_new) == 0);

    strcpy(keyexpr, "a/**/**/c");
    len_new = len_old;
    int8_t res = z_view_keyexpr_from_substr_autocanonize(&key_expr_canonized, keyexpr, &len_new);
    assert(res == 0);
    assert(len_new == len_old - 3);
    assert(strncmp(keyexpr, "a/**/c", len_new) == 0);
    z_keyexpr_as_view_string(z_loan(key_expr_canonized), &key_exp_canonized_bytes);
    assert(z_string_len(z_loan(key_exp_canonized_bytes)) == len_new);
    assert(strncmp(z_string_data(z_loan(key_exp_canonized_bytes)), "a/**/c", len_new) == 0);
}

void includes() {
    z_view_keyexpr_t foobar, foostar;
    z_view_keyexpr_from_str(&foobar, "foo/bar");
    z_view_keyexpr_from_str(&foostar, "foo/*");

    assert(z_keyexpr_includes(z_loan(foostar), z_loan(foobar)) == true);
    assert(z_keyexpr_includes(z_loan(foobar), z_loan(foostar)) == false);
}

void intersects() {
    z_view_keyexpr_t foobar, foostar, barstar;
    z_view_keyexpr_from_str(&foobar, "foo/bar");
    z_view_keyexpr_from_str(&foostar, "foo/*");
    z_view_keyexpr_from_str(&barstar, "bar/*");

    assert(z_keyexpr_intersects(z_loan(foostar), z_loan(foobar)) == true);
    assert(z_keyexpr_intersects(z_loan(barstar), z_loan(foobar)) == false);
}

void undeclare() {
    z_owned_config_t config;
    z_config_default(&config);
    z_owned_session_t s;
    z_open(&s, z_move(config), NULL);

    z_view_keyexpr_t view_ke;
    z_view_keyexpr_from_str(&view_ke, "test/thr");
    z_owned_keyexpr_t ke;
    z_declare_keyexpr(z_loan(s), &ke, z_loan(view_ke));
    assert(z_internal_keyexpr_check(&ke));
    z_undeclare_keyexpr(z_loan(s), z_move(ke));
    assert(!z_internal_keyexpr_check(&ke));
}

#if defined(Z_FEATURE_UNSTABLE_API)
void relation_to() {
    z_view_keyexpr_t foobar, foostar, barstar;
    z_view_keyexpr_from_str(&foobar, "foo/bar");
    z_view_keyexpr_from_str(&foostar, "foo/*");
    z_view_keyexpr_from_str(&barstar, "bar/*");

    assert(z_keyexpr_relation_to(z_loan(foostar), z_loan(foobar)) == Z_KEYEXPR_INTERSECTION_LEVEL_INCLUDES);
    assert(z_keyexpr_relation_to(z_loan(foobar), z_loan(foostar)) == Z_KEYEXPR_INTERSECTION_LEVEL_INTERSECTS);
    assert(z_keyexpr_relation_to(z_loan(foostar), z_loan(foostar)) == Z_KEYEXPR_INTERSECTION_LEVEL_EQUALS);
    assert(z_keyexpr_relation_to(z_loan(barstar), z_loan(foobar)) == Z_KEYEXPR_INTERSECTION_LEVEL_DISJOINT);
}
#endif

static void assert_empty_keyexpr_string(const z_loaned_keyexpr_t* source) {
    z_view_string_t text;
    z_view_keyexpr_t roundtrip;
    z_keyexpr_as_view_string(source, &text);
    assert(z_string_len(z_loan(text)) == 0);
    assert(z_view_keyexpr_from_substr(&roundtrip, z_string_data(z_loan(text)), z_string_len(z_loan(text))) == Z_EINVAL);
    assert(z_view_keyexpr_is_empty(&roundtrip));
}

static void keyexpr_string_views(void) {
    const char raw[] = {'d', 'e', 'm', 'o', '/', '*', '*'};
    z_owned_keyexpr_t owned_key;
    assert(z_keyexpr_from_substr(&owned_key, raw, sizeof(raw)) == Z_OK);
    z_view_keyexpr_t key_view;
    assert(z_view_keyexpr_from_substr(&key_view, raw, sizeof(raw)) == Z_OK);
    z_view_string_t text;
    z_keyexpr_as_view_string(z_loan(owned_key), &text);
    assert(z_string_len(z_loan(text)) == sizeof(raw));
    assert(memcmp(z_string_data(z_loan(text)), raw, sizeof(raw)) == 0);
    z_keyexpr_as_view_string(z_loan(key_view), &text);
    assert(z_string_data(z_loan(text)) == raw);

    z_view_keyexpr_t invalid;
    const char* bad[] = {"bad/**x", "a/**/**", "", "\xff"};
    for (size_t i = 0; i < sizeof(bad) / sizeof(bad[0]); ++i) {
        assert(z_view_keyexpr_from_str(&invalid, bad[i]) != Z_OK);
        assert(z_view_keyexpr_is_empty(&invalid));
        assert_empty_keyexpr_string(z_loan(invalid));
    }
    z_view_keyexpr_empty(&invalid);
    assert_empty_keyexpr_string(z_loan(invalid));
    z_owned_keyexpr_t empty_key;
    z_internal_keyexpr_null(&empty_key);
    assert_empty_keyexpr_string(z_loan(empty_key));
    z_drop(z_move(empty_key));

    // A valid key named "dummy" must remain distinct from the empty sentinel.
    assert(z_view_keyexpr_from_str(&key_view, "dummy") == Z_OK);
    z_keyexpr_as_view_string(z_loan(key_view), &text);
    assert(z_string_len(z_loan(text)) == 5);
    assert(memcmp(z_string_data(z_loan(text)), "dummy", 5) == 0);
    assert(z_view_keyexpr_from_substr(&key_view, z_string_data(z_loan(text)), z_string_len(z_loan(text))) == Z_OK);
    z_drop(z_move(owned_key));
}

int main(int argc, char** argv) {
    keyexpr_string_views();
    canonize();
    includes();
    intersects();
    undeclare();
#if defined(Z_FEATURE_UNSTABLE_API)
    relation_to();
#endif
}
