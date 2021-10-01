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
#include "zenoh.h"

int main(int argc, char **argv)
{
    z_init_logger();

    z_owned_config_t config = z_config__default();
    if (argc > 1)
    {
        z_config__insert(config.borrow, ZN_CONFIG_PEER_KEY, z_string__new(argv[1]));
    }

    printf("Openning session...\n");
    z_owned_session_t s = z_open(config);
    if (s.borrow == 0)
    {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_info_t ps = z_info(s.borrow);
    z_string_t prop = z_info__get(ps.borrow, ZN_INFO_PID_KEY);
    printf("info_pid : %s\n", prop.start);
    z_string__free(prop);

    prop = z_info__get(ps.borrow, ZN_INFO_ROUTER_PID_KEY);
    printf("info_router_pid : %s\n", prop.start);
    z_string__free(prop);

    prop = z_info__get(ps.borrow, ZN_INFO_PEER_PID_KEY);
    printf("info_peer_pid : %s\n", prop.start);
    z_string__free(prop);

    z_info__free(ps);
    z_close(s);
}