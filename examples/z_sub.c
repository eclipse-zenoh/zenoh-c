/*
 * Copyright (c) 2017, 2020 ADLINK Technology Inc.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
 * which is available at https://www.apache.org/licenses/LICENSE-2.0.
 *
 * SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
 *
 * Contributors:
 *   ADLINK zenoh team, <zenoh@adlink-labs.tech>
 */
#include <stdio.h>
#if defined(WIN32) || defined(_WIN32) || defined(__WIN32) && !defined(__CYGWIN__)
#include <windows.h>
#define sleep(x) Sleep(x * 1000)
#else
#include <unistd.h>
#endif
#include "zenoh.h"

void data_handler(const z_sample_t *sample, const void *arg __attribute__((unused)))
{
    printf(">> [Subscriber] Received ('%.*s': '%.*s')\n",
           (int)sample->key.suffix.len, sample->key.suffix.start, (int)sample->value.len, sample->value.start);
}

int main(int argc, char **argv)
{
    z_init_logger();

    char *expr = "/demo/example/**";
    if (argc > 1)
    {
        expr = argv[1];
    }
    z_owned_config_t config = z_config_default();
    if (argc > 2)
    {
        z_config_insert_json(z_loan(config), Z_CONFIG_CONNECT_KEY, argv[2]);
    }

    printf("Openning session...\n");
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s))
    {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring Subscriber on '%s'...\n", expr);
    z_owned_subscriber_t sub = z_subscribe(z_loan(s), z_expr(expr), z_subinfo_default(), data_handler, NULL);
    if (!z_check(sub))
    {
        printf("Unable to declare subscriber.\n");
        exit(-1);
    }

    printf("Enter 'q' to quit...\n");
    char c = 0;
    while (c != 'q')
    {
        c = getchar();
        if (c == -1)
        {
            sleep(1);
        }
    }

    z_subscriber_close(z_move(sub));
    z_close(z_move(s));
    return 0;
}
