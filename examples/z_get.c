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
#include "zenoh.h"

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
        if (!z_config_insert_json(z_loan(config), Z_CONFIG_CONNECT_KEY, argv[2]))
        {
            printf("Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a JSON-serialized list of strings\n", argv[2], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    printf("Openning session...\n");
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s))
    {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Sending Query '%s'...\n", expr);
    z_query_target_t target = z_query_target_default();
    target.target = z_target_t_ALL;
    z_owned_reply_data_array_t replies = z_get_collect(
        z_loan(s), z_expr(expr), "",
        target, z_query_consolidation_default());

    for (unsigned int i = 0; i < replies.len; ++i)
    {
        printf(">> Received ('%.*s': '%.*s')\n",
               (int)replies.val[i].data.key.suffix.len, replies.val[i].data.key.suffix.start,
               (int)replies.val[i].data.value.len, replies.val[i].data.value.start);
    }
    z_reply_data_array_free(z_move(replies));
    z_close(z_move(s));
    return 0;
}