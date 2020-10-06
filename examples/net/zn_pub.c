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
#include "zenoh/net.h"

int main(int argc, char **argv) {
    char *uri = "/demo/example/zenoh-c-pub";
    if (argc > 1) {
        uri = argv[1];
    }
    char *value = "Pub from C!";
    if (argc > 2) {
        value = argv[2];
    }
    char *locator = 0;
    if (argc > 3) {
        locator = argv[3];
    }

    printf("Openning session...\n");
    ZNSession *s = zn_open("peer", locator, 0);
    if (s == 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Declaring Resource '%s'", uri);
    unsigned long rid = zn_declare_resource(s, zn_rname(uri));
    printf(" => RId %lu\n", rid);
    ZNResKey *reskey = zn_rid(rid);

    printf("Declaring Publisher on %lu\n", rid);
    ZNPublisher *pub = zn_declare_publisher(s, reskey);
    if (pub == 0) {
        printf("Unable to declare publisher.\n");
        exit(-1);
    }

    char buf[256];
    for(int idx = 0; 1; ++idx) {
        sleep(1);
        sprintf(buf, "[%4d] %s", idx, value);
        printf("Writing Data ('%lu': '%s')...\n", rid, buf);
        zn_write(s, reskey, buf, strlen(buf));
    }

    zn_undeclare_publisher(pub);
    zn_close(s);
    return 0;
}