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

int main(int argc, char **argv)
{
    z_init_logger();

    if (argc < 2)
    {
        printf("USAGE:\n\tz_pub_thr <payload-size> [<zenoh-locator>]\n\n");
        exit(-1);
    }
    size_t len = atoi(argv[1]);
    z_owned_config_t config = z_config_default();
    if (argc > 2)
    {
        if (!z_config_insert_json(z_loan(config), Z_CONFIG_CONNECT_KEY, argv[2]))
        {
            printf("Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a JSON-serialized list of strings\n", argv[2], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    z_owned_session_t os = z_open(z_move(config));
    if (!z_check(os))
    {
        printf("Unable to open session!\n");
        exit(-1);
    }
    z_session_t s = z_loan(os);

    char *data = (char *)malloc(len);
    memset(data, 1, len);

    z_keyexpr_t keyexpr = z_declare_expr(s, z_expr("/test/thr"));
    if (!z_declare_publication(s, keyexpr))
    {
        printf("Unable to declare publication.\n");
        exit(-1);
    }

    while (1)
    {
        z_put(s, keyexpr, (const uint8_t *)data, len);
    }
    z_undeclare_publication(s, keyexpr);
    z_undeclare_expr(s, keyexpr);
    z_close(z_move(os));
}