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

void close_drop() {
    z_owned_config_t config;
    z_config_default(&config);

    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        perror("Unable to open session!");
        exit(-1);
    }

    z_drop(z_move(s));
}

void close_sync() {
    z_owned_config_t config;
    z_config_default(&config);

    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        perror("Unable to open session!");
        exit(-1);
    }

    z_close_options_t options;
    z_close_options_default(&options);

    if (z_close(z_loan_mut(s), &options) < 0) {
        perror("Error closing session!");
        exit(-1);
    }

    z_drop(z_move(s));
}

void close_concurrent() {
    z_owned_config_t config;
    z_config_default(&config);

    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        perror("Unable to open session!");
        exit(-1);
    }

    zc_owned_concurrent_close_handle_t close_handle;

    z_close_options_t options;
    z_close_options_default(&options);
    options.out_concurrent = &close_handle;

    if (z_close(z_loan_mut(s), &options) < 0) {
        perror("Error starting concurrent session close!");
        exit(-1);
    }

    if (zc_concurrent_close_handle_wait(z_move(close_handle)) < 0) {
        perror("Error closing session!");
        exit(-1);
    }

    z_drop(z_move(s));
}

int main(int argc, char **argv) {
    zc_try_init_log_from_env();
    close_drop();
    close_sync();
    close_concurrent();
}
