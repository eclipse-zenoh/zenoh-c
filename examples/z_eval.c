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

char *uri = "/demo/example/zenoh-c-eval";
char *value = "Eval from C!";

void query_handler(z_query_t *query, const void *arg)
{
    z_string_t res = z_query__res_name(query);
    z_string_t pred = z_query__predicate(query);
    printf(">> [Query handler] Handling '%s?%s'\n", res.borrow, pred.borrow);
    z_send_reply(query, uri, (const unsigned char *)value, strlen(value));
}

int main(int argc, char **argv)
{
    z_init_logger();

    if (argc > 1)
    {
        uri = argv[1];
    }
    z_owned_config_t config = z_config__default();
    if (argc > 2)
    {
        z_config__insert(config.borrow, ZN_CONFIG_PEER_KEY, z_string__new(argv[2]));
    }

    printf("Openning session...\n");
    z_owned_session_t s = z_open(config);
    if (s.borrow == 0)
    {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring Queryable on '%s'...\n", uri);
    z_owned_reskey_t urikey = z_rname(uri);
    z_owned_queryable_t qable = z_register_queryable(s.borrow, &urikey, ZN_QUERYABLE_EVAL, query_handler, NULL);
    if (qable.borrow == 0)
    {
        printf("Unable to declare queryable.\n");
        exit(-1);
    }

    char c = 0;
    while (c != 'q')
    {
        c = fgetc(stdin);
    }

    z_unregister_queryable(qable);
    z_reskey__free(urikey);
    z_close(s);
    return 0;
}
