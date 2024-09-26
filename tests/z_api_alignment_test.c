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

#include <stddef.h>
#include <stdio.h>
#include <string.h>

#undef NDEBUG
#include <assert.h>

#include "zenoh.h"

#define SLEEP 1
#define URI "demo/example/**/*"
#define SCOUTING_TIMEOUT "1000"

const char *value = "Test value";

#if defined(Z_FEATURE_UNSTABLE_API)
volatile unsigned int zids = 0;
void zid_handler(const z_id_t *id, void *arg) {
    (void)(arg);
    (void)(id);
    zids++;
}
#endif

volatile unsigned int hellos = 0;
void hello_handler(z_loaned_hello_t *hello, void *arg) {
    (void)(arg);
    (void)(hello);
    hellos++;
}

volatile unsigned int queries = 0;
void query_handler(z_loaned_query_t *query, void *arg) {
    queries++;

    const z_loaned_keyexpr_t *query_ke = z_query_keyexpr(query);
    z_view_string_t k_str;
    z_keyexpr_as_view_string(query_ke, &k_str);
#ifdef ZENOH_PICO
    if (k_str == NULL) {
        k_str = zp_keyexpr_resolve(*(z_loaned_session_t *)arg, z_query_keyexpr(query));
    }
#endif

    z_view_string_t params;
    z_query_parameters(query, &params);
    (void)(params);
    const z_loaned_bytes_t *in_payload = z_query_payload(query);
    (void)(in_payload);
    const z_loaned_encoding_t *encoding = z_query_encoding(query);
    (void)(encoding);
    z_query_reply_options_t _ret_qreply_opt;
    z_query_reply_options_default(&_ret_qreply_opt);

    z_owned_bytes_t payload;
    z_bytes_from_static_str(&payload, value);
    z_query_reply(query, query_ke, z_move(payload), &_ret_qreply_opt);
}

volatile unsigned int replies = 0;
void reply_handler(z_loaned_reply_t *reply, void *arg) {
    replies++;

    if (z_reply_is_ok(reply)) {
        const z_loaned_sample_t *sample = z_reply_ok(reply);

        z_view_string_t k_str;
        z_keyexpr_as_view_string(z_sample_keyexpr(sample), &k_str);
#ifdef ZENOH_PICO
        if (k_str == NULL) {
            k_str = zp_keyexpr_resolve(*(z_loaned_session_t *)arg, sample.keyexpr);
        }
#endif
    } else {
        const z_loaned_reply_err_t *_ret_zerr = z_reply_err(reply);
        (void)(_ret_zerr);
    }
}

volatile unsigned int datas = 0;
void data_handler(z_loaned_sample_t *sample, void *arg) {
    datas++;

    z_view_string_t k_str;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &k_str);
#ifdef ZENOH_PICO
    if (k_str == NULL) {
        k_str = zp_keyexpr_resolve(*(z_loaned_session_t *)arg, sample->keyexpr);
    }
#endif
}

int main(int argc, char **argv) {
    setbuf(stdout, NULL);

#ifdef ZENOH_C
    zc_try_init_log_from_env();
#endif

    z_view_keyexpr_t key_demo_example, key_demo_example_a, key_demo_example_starstar;
    z_view_keyexpr_from_str(&key_demo_example, "demo/example");
    z_view_keyexpr_from_str(&key_demo_example_a, "demo/example/a");
    z_view_keyexpr_from_str(&key_demo_example_starstar, "demo/example/**");
    bool _ret_bool = z_view_keyexpr_is_empty(&key_demo_example);
    assert(_ret_bool == false);

    _ret_bool = z_keyexpr_includes(z_loan(key_demo_example_starstar), z_loan(key_demo_example_a));
    assert(_ret_bool);
#ifdef ZENOH_PICO
    _ret_bool = zp_keyexpr_includes_null_terminated("demo/example/**", "demo/example/a");
    assert(_ret_int == 0);
#endif
    _ret_bool = z_keyexpr_intersects(z_loan(key_demo_example_starstar), z_loan(key_demo_example_a));
    assert(_ret_bool);
#ifdef ZENOH_PICO
    _ret_bool = zp_keyexpr_intersect_null_terminated("demo/example/**", "demo/example/a");
    assert(_ret_int == 0);
#endif
    _ret_bool = z_keyexpr_equals(z_loan(key_demo_example_starstar), z_loan(key_demo_example));
    assert(!_ret_bool);
#ifdef ZENOH_PICO
    _ret_bool = zp_keyexpr_equals_null_terminated("demo/example/**", "demo/example");
    assert(_ret_int == -1);
#endif

    z_sleep_s(SLEEP);

    size_t keyexpr_len = strlen(URI);
    char *keyexpr_str = (char *)z_malloc(keyexpr_len + 1);
    memcpy(keyexpr_str, URI, keyexpr_len);
    keyexpr_str[keyexpr_len] = '\0';
    int8_t _ret_int8 = z_keyexpr_is_canon(keyexpr_str, keyexpr_len);
    assert(_ret_int8 < 0);

#ifdef ZENOH_PICO
    _ret_int8 = zp_keyexpr_is_canon_null_terminated(keyexpr_str);
    assert(_ret_int8 < 0);
#endif
    _ret_int8 = z_keyexpr_canonize(keyexpr_str, &keyexpr_len);
    assert(_ret_int8 == 0);
    assert(strlen(URI) == keyexpr_len);
    printf("keyexpr: %s", keyexpr_str);
#ifdef ZENOH_PICO
    _ret_int8 = zp_keyexpr_canonize_null_terminated(keyexpr_str);
    assert(_ret_int8 == 0);
    assert(strlen(URI) == keyexpr_len);
#endif

    z_sleep_s(SLEEP);

    z_owned_config_t _ret_config;
    z_config_default(&_ret_config);
    assert(z_internal_check(_ret_config));
    z_drop(z_move(_ret_config));
#ifdef ZENOH_PICO
    _ret_int8 = zp_config_insert(z_loan(_ret_config), Z_CONFIG_PEER_KEY, z_string_make(argv[1]));
    assert(_ret_int8 == 0);
    const char *_ret_cstr = zp_config_get(z_loan(_ret_config), Z_CONFIG_PEER_KEY);
    assert(strlen(_ret_cstr) == strlen(argv[1]));
    assert(strncmp(_ret_cstr, argv[1], strlen(_ret_cstr)) == 0);
#endif

#ifdef ZENOH_PICO
    z_owned_scouting_config_t _ret_sconfig;
    z_scouting_config_default(&_ret_sconfig);
    assert(z_internal_check(_ret_sconfig));
    _ret_int8 =
        zp_scouting_config_insert(z_loan(_ret_sconfig), Z_CONFIG_SCOUTING_TIMEOUT_KEY, z_string_make(SCOUTING_TIMEOUT));
    assert(_ret_int8 == 0);
    _ret_cstr = zp_scouting_config_get(z_loan(_ret_sconfig), Z_CONFIG_SCOUTING_TIMEOUT_KEY);
    assert(strlen(_ret_cstr) == strlen(SCOUTING_TIMEOUT));
    assert(strncmp(_ret_cstr, SCOUTING_TIMEOUT, strlen(_ret_cstr)) == 0);
    z_drop(z_move(_ret_sconfig));
#endif

    z_sleep_s(SLEEP);
    z_config_default(&_ret_config);
    z_owned_closure_hello_t _ret_closure_hello;
    z_closure(&_ret_closure_hello, hello_handler, NULL, NULL);
    _ret_int8 = z_scout(z_move(_ret_config), z_move(_ret_closure_hello), NULL);
    assert(_ret_int8 == 0);
    assert(hellos == 1);

    z_sleep_s(atoi(SCOUTING_TIMEOUT) / 1000);
    z_sleep_s(SLEEP);

    z_owned_session_t s1;
    assert(0 == z_open(&s1, z_move(_ret_config), NULL));
    assert(z_internal_check(s1));

#if defined(Z_FEATURE_UNSTABLE_API)
    z_id_t _ret_zid = z_info_zid(z_loan(s1));
    z_owned_string_t str;
    z_id_to_string(&_ret_zid, &str);
    printf("Session 1 with PID: 0x%.*s\n", (int)z_string_len(z_loan(str)), z_string_data(z_loan(str)));
    z_drop(z_move(str));

    z_owned_closure_zid_t _ret_closure_zid;
    z_closure(&_ret_closure_zid, zid_handler, NULL, NULL);
    _ret_int8 = z_info_peers_zid(z_loan(s1), z_move(_ret_closure_zid));
    assert(_ret_int8 == 0);
    z_sleep_s(SLEEP);
    assert(zids == 0);

    z_closure(&_ret_closure_zid, zid_handler, NULL, NULL);
    _ret_int8 = z_info_routers_zid(z_loan(s1), z_move(_ret_closure_zid));
    assert(_ret_int8 == 0);

    z_sleep_s(SLEEP);
    assert(zids == 1);
#endif

#ifdef ZENOH_PICO
    zp_task_read_options_t _ret_read_opt = zp_task_read_options_default();
    zp_start_read_task(z_loan(s1), &_ret_read_opt);
    zp_task_lease_options_t _ret_lease_opt = zp_task_lease_options_default();
    zp_start_lease_task(z_loan(s1), &_ret_lease_opt);
#endif

    z_sleep_s(SLEEP);

    z_config_default(&_ret_config);
#ifdef ZENOH_PICO
    _ret_int8 = zp_config_insert(z_loan(_ret_config), Z_CONFIG_PEER_KEY, z_string_make(argv[1]));
    assert(_ret_int8 == 0);
    _ret_cstr = zp_config_get(z_loan(_ret_config), Z_CONFIG_PEER_KEY);
    assert(strlen(_ret_cstr) == strlen(argv[1]));
    assert(strncmp(_ret_cstr, argv[1], strlen(_ret_cstr)) == 0);
#endif

    z_owned_session_t s2;
    assert(0 == z_open(&s2, z_move(_ret_config), NULL));
    assert(z_internal_check(s2));

#if defined(Z_FEATURE_UNSTABLE_API)
    _ret_zid = z_info_zid(z_loan(s2));
    z_id_to_string(&_ret_zid, &str);
    printf("Session 2 with PID: 0x%.*s\n", (int)z_string_len(z_loan(str)), z_string_data(z_loan(str)));
    z_drop(z_move(str));
#endif

#ifdef ZENOH_PICO
    zp_start_read_task(z_loan(s2), NULL);
    zp_start_lease_task(z_loan(s2), NULL);
#endif

    z_sleep_s(SLEEP);

    const z_loaned_session_t *ls1 = z_loan(s1);
    z_owned_closure_sample_t _ret_closure_sample;
    z_closure(&_ret_closure_sample, data_handler, NULL, (void *)ls1);
    z_subscriber_options_t _ret_sub_opt;
    z_subscriber_options_default(&_ret_sub_opt);

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, keyexpr_str);
    z_owned_subscriber_t _ret_sub;
    z_declare_subscriber(&_ret_sub, z_loan(s2), z_loan(ke), z_move(_ret_closure_sample), &_ret_sub_opt);
    assert(z_internal_check(_ret_sub));

    z_sleep_s(SLEEP);

    char s1_res[64];
    snprintf(s1_res, 64, "%s/chunk/%d", keyexpr_str, 1);
    z_view_keyexpr_t s1_key;
    z_view_keyexpr_from_str(&s1_key, s1_res);
    z_owned_keyexpr_t _ret_expr;
    z_declare_keyexpr(&_ret_expr, z_loan(s1), z_loan(s1_key));
    assert(z_internal_check(_ret_expr));
    z_put_options_t _ret_put_opt;
    z_put_options_default(&_ret_put_opt);
    _ret_put_opt.congestion_control = Z_CONGESTION_CONTROL_BLOCK;
    // TODO: set encoding option

    z_owned_bytes_t payload;
    z_bytes_copy_from_str(&payload, value);
    _ret_int8 = z_put(z_loan(s1), z_loan(_ret_expr), z_move(payload), &_ret_put_opt);
    assert(_ret_int8 == 0);

    z_sleep_s(SLEEP);
    assert(datas == 1);

    z_delete_options_t _ret_delete_opt;
    z_delete_options_default(&_ret_delete_opt);
    _ret_int8 = z_delete(z_loan(s1), z_loan(_ret_expr), &_ret_delete_opt);
    assert(_ret_int8 == 0);

    z_sleep_s(SLEEP);
    assert(datas == 2);

    _ret_int8 = z_undeclare_keyexpr(z_move(_ret_expr), z_loan(s1));
    assert(_ret_int8 == 0);
    assert(!z_internal_check(_ret_expr));

    _ret_int8 = z_undeclare_subscriber(z_move(_ret_sub));
    assert(_ret_int8 == 0);

    // TODO: test for pull subscriber

    z_owned_closure_query_t _ret_closure_query;
    z_closure(&_ret_closure_query, query_handler, NULL, (void *)ls1);
    z_queryable_options_t _ret_qle_opt;
    z_queryable_options_default(&_ret_qle_opt);
    z_owned_queryable_t qle;
    z_declare_queryable(&qle, z_loan(s1), z_loan(s1_key), z_move(_ret_closure_query), &_ret_qle_opt);
    assert(z_internal_check(qle));

    z_sleep_s(SLEEP);

    const z_loaned_session_t *ls2 = z_loan(s2);
    z_owned_closure_reply_t _ret_closure_reply;
    z_closure(&_ret_closure_reply, reply_handler, NULL, &ls2);
    z_get_options_t _ret_get_opt;
    z_get_options_default(&_ret_get_opt);
    _ret_get_opt.target = z_query_target_default();
    _ret_get_opt.consolidation = z_query_consolidation_auto();
    _ret_get_opt.consolidation = z_query_consolidation_default();
    _ret_get_opt.consolidation = z_query_consolidation_latest();
    _ret_get_opt.consolidation = z_query_consolidation_monotonic();
    _ret_get_opt.consolidation = z_query_consolidation_none();
    _ret_int8 = z_get(z_loan(s2), z_loan(s1_key), "", z_move(_ret_closure_reply), &_ret_get_opt);
    assert(_ret_int8 == 0);

    z_sleep_s(SLEEP);
    assert(queries == 1);
    assert(replies == 1);

    _ret_int8 = z_undeclare_queryable(z_move(qle));
    assert(_ret_int8 == 0);

#ifdef ZENOH_PICO
    zp_stop_read_task(z_loan(s1));
    zp_stop_lease_task(z_loan(s1));
#endif

    _ret_int8 = z_close(z_move(s1), NULL);
    assert(_ret_int8 == 0);

#ifdef ZENOH_PICO
    zp_stop_read_task(z_loan(s2));
    zp_stop_lease_task(z_loan(s2));
#endif
    _ret_int8 = z_close(z_move(s2), NULL);
    assert(_ret_int8 == 0);

    z_sleep_s(SLEEP * 5);

    z_free(keyexpr_str);

    return 0;
}
