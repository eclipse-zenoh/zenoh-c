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
#include "zenoh/net.h"
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <string.h>

void reply_handler(const zn_source_info *info, const zn_sample *sample, const void *arg) {
    printf(">> [Reply handler] received (%.*s, %.*s)\n",
        sample->key.len, sample->key.val,
        sample->value.len, sample->value.val);
}

int main(int argc, char** argv) {
    char *uri = "/demo/example/**";
    if (argc > 1) {
        uri = argv[1];
    }
    ZNProperties *config = zn_config_peer();
    if (argc > 2) {
        zn_properties_add(config, ZN_PEER_KEY, argv[2]);
    }

    printf("Openning session...\n");
    ZNSession *s = zn_open(config);
    if (s == 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    printf("Sending Query '%s'...\n", uri);
    zn_query(s, zn_rname(uri), "", zn_query_target_default(), zn_query_consolidation_default(), reply_handler, NULL);

    sleep(1);

    zn_close(s);
    return 0;
}