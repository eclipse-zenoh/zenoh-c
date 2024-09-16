//
// Copyright (c) 2024 ZettaScale Technology
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
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

void cb(struct z_loaned_reply_t *reply, void *context) {}
void drop(void *context) {}

void put() {
    z_owned_config_t config;
    z_config_default(&config);

    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        perror("Unable to open session!");
        exit(-1);
    }

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "zenoh/test_put");
    z_put_options_t opts;
    z_put_options_default(&opts);
    z_owned_bytes_t payload, attachment;
    z_bytes_serialize_from_int32(&attachment, 16);
    opts.attachment = z_move(attachment);
    z_bytes_serialize_from_int32(&payload, 16);
    z_put(z_loan(s), z_loan(ke), z_move(payload), &opts);
    assert(!z_internal_check(payload));
    assert(!z_internal_check(attachment));
    z_close(z_move(s), NULL);
}

void get() {
    z_owned_config_t config;
    z_config_default(&config);

    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        perror("Unable to open session!");
        exit(-1);
    }

    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, "zenoh/test_get");
    z_get_options_t opts;
    z_get_options_default(&opts);
    z_owned_bytes_t payload, attachment;
    z_bytes_serialize_from_int32(&attachment, 16);
    opts.payload = z_move(payload);
    z_bytes_serialize_from_int32(&payload, 16);
    opts.attachment = z_move(attachment);
    z_owned_closure_reply_t closure;
    z_closure(&closure, cb, drop, NULL);

    z_get(z_loan(s), z_loan(ke), "", z_move(closure), &opts);
    assert(!z_internal_check(payload));
    assert(!z_internal_check(attachment));
    z_close(z_move(s), NULL);
}

int main(int argc, char **argv) {
    zc_try_init_log_from_env();
    put();
    get();
}
