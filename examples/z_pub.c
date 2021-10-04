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

    char *uri = "/demo/example/zenoh-c-pub";
    if (argc > 1)
    {
        uri = argv[1];
    }
    char *value = "Pub from C!";
    if (argc > 2)
    {
        value = argv[2];
    }
    z_owned_config_t config = z_config__default();
    if (argc > 3)
    {
        z_config__insert(config.borrow, ZN_CONFIG_PEER_KEY, z_string__new(argv[3]));
    }

    printf("Openning session...\n");
    z_owned_session_t s = z_open(config);
    if (s.borrow == 0)
    {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring Resource '%s'", uri);
    unsigned long rid = z_register_resource(s.borrow, z_rname(uri));
    printf(" => RId %lu\n", rid);
    z_owned_reskey_t reskey = z_rid(rid);

    printf("Declaring Publisher on %lu\n", rid);
    z_owned_publisher_t pub = z_register_publisher(s.borrow, &reskey);
    if (pub.borrow == 0)
    {
        printf("Unable to declare publisher.\n");
        exit(-1);
    }

    char buf[256];
    for (int idx = 0; 1; ++idx)
    {
        sleep(1);
        sprintf(buf, "[%4d] %s", idx, value);
        printf("Writing Data ('%lu': '%s')...\n", rid, buf);
        z_write(s.borrow, &reskey, (const uint8_t *)buf, strlen(buf));
    }
    z_reskey__free(reskey);
    z_unregister_publisher(pub);
    z_close(s);
    return 0;
}