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
//
#include <stdio.h>
#include <string.h>
#include "zenoh.h"
#if defined(WIN32) || defined(_WIN32) || defined(__WIN32) && !defined(__CYGWIN__)
#include <windows.h>
#define sleep(x) Sleep(x * 1000)
#else
#include <unistd.h>
#endif

int main(int argc, char **argv)
{
    char *keyexpr = "demo/example/zenoh-c-pub";
    char *value = "Pub from C!";

    z_init_logger();

    if (argc > 1)
        keyexpr = argv[1];
    if (argc > 2)
        value = argv[2];

    z_owned_config_t config = z_config_default();
    if (argc > 3)
    {
        if (!z_config_insert_json(z_loan(config), Z_CONFIG_CONNECT_KEY, argv[3]))
        {
            printf("Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a JSON-serialized list of strings\n", argv[3], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    printf("Opening session...\n");
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s))
    {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring key expression '%s'...", keyexpr);
    z_owned_keyexpr_t ke = z_declare_keyexpr(z_loan(s), z_keyexpr(keyexpr));
    if (!z_check(ke))
    {
        printf("Unable to declare key expression!\n");
        exit(-1);
    }

    // @TODO: declare publisher

    char buf[256];
    for (int idx = 0; 1; ++idx)
    {
        sleep(1);
        sprintf(buf, "[%4d] %s", idx, value);
        printf("Putting Data ('%s': '%s')...\n", keyexpr, buf);
        z_put(z_loan(s), z_loan(ke), (const uint8_t *)buf, strlen(buf), NULL);
    }

    // @TODO: undeclare publisher

    z_undeclare_keyexpr(z_loan(s), z_move(ke));
    z_close(z_move(s));
    return 0;
}
