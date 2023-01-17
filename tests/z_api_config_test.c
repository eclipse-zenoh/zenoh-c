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
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

int main(int argc, char **argv) {
    zc_init_logger();
    const char *peers[] = {"tcp/127.0.0.1", "tcp/192.168.0.1", "tcp/10.0.0.1"};
    z_owned_config_t config = z_config_client(peers, 3);
    const char *key = zc_config_get(z_loan(config), "connect/endpoints");
    assert(strcmp(key, "[\"tcp/127.0.0.1\",\"tcp/192.168.0.1\",\"tcp/10.0.0.1\"]") == 0);
}
