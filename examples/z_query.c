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
#include <unistd.h>
#include <string.h>
#include "zenoh.h"

int main(int argc, char **argv)
{
    z_init_logger();

    char *uri = "/demo/example/**";
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

    printf("Sending Query '%s'...\n", uri);
    z_owned_reskey_t urikey = z_rname(uri);
    z_reply_data_array_t replies = z_query_collect(s.borrow, &urikey, "", z_query_target__default(), z_query_consolidation__default());

    for (unsigned int i = 0; i < replies.len; ++i)
    {
        printf(">> [Reply handler] received (%s, %.*s)\n",
               replies.val[i].data.key.borrow, (int)replies.val[i].data.value.len, replies.val[i].data.value.val);
    }
    z_reply_data_array__free(replies);
    z_reskey__free(urikey);
    z_close(s);
    return 0;
}