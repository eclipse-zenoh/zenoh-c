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

    z_owned_config_t config = z_config_default();
    if (argc > 1)
    {
        z_config_set(z_loan(config), ZN_CONFIG_PEER_KEY, argv[1]);
    }

    printf("Openning session...\n");
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s))
    {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_owned_info_t ops = z_info(z_loan(s));
    z_info_t ps = z_loan(ops);
    z_owned_string_t prop = z_info_get(ps, ZN_INFO_PID_KEY);
    printf("info_pid : %s\n", z_loan(prop));
    z_string_free(z_move(prop));

    prop = z_info_get(ps, ZN_INFO_ROUTER_PID_KEY);
    printf("info_router_pid : %s\n", z_loan(prop));
    z_string_free(z_move(prop));

    prop = z_info_get(ps, ZN_INFO_PEER_PID_KEY);
    printf("info_peer_pid : %s\n", z_loan(prop));
    z_string_free(z_move(prop));

    z_info_free(z_move(ops));
    z_close(z_move(s));
}