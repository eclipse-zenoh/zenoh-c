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
#include <string.h>
#if defined(WIN32) || defined(_WIN32) || defined(__WIN32) && !defined(__CYGWIN__)
#include <windows.h>
#define sleep(x) Sleep(x * 1000)
#else
#include <unistd.h>
#endif
#include "zenoh.h"

char *expr = "/demo/example/zenoh-c-eval";
char *value = "Eval from C!";

void query_handler(const z_query_t *query, const void *arg)
{
    z_bytes_t res = z_query_key_expr(query).suffix;
    z_bytes_t pred = z_query_predicate(query);
    printf(">> [Queryable ] Received Query '%.*s%.*s'\n", (int)res.len, res.start, (int)pred.len, pred.start);
    z_send_reply(query, expr, (const unsigned char *)value, strlen(value));
}

int main(int argc, char **argv)
{
    z_init_logger();

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

    printf("Creating Queryable on '%s'...\n", expr);
    z_owned_queryable_t qable = z_queryable_new(z_loan(s), z_expr(expr), Z_QUERYABLE_EVAL, query_handler, NULL);
    if (!z_check(qable))
    {
        printf("Unable to create queryable.\n");
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

    z_queryable_close(z_move(qable));
    z_close(z_move(s));
    return 0;
}
