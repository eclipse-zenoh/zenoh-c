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
#include <time.h>
#include "zenoh.h"

#define N 100000

volatile unsigned long long int count = 0;
volatile clock_t start;
volatile clock_t stop;

void print_stats(volatile clock_t *start, volatile clock_t *stop)
{
    clock_t elapsed = stop - start;
    double thpt = N * (double)CLOCKS_PER_SEC / (double)(elapsed);
    printf("%f msgs/sec\n", thpt);
}

void data_handler(const z_sample_t *sample, const void *arg)
{
    if (count == 0)
    {
        start = clock();
        count++;
    }
    else if (count < N)
    {
        count++;
    }
    else
    {
        stop = clock();
        print_stats(&start, &stop);
        count = 0;
    }
}

int main(int argc, char **argv)
{
    z_init_logger();

    z_owned_config_t config = z_config_default();
    if (argc > 1)
    {
        z_config_set(z_loan(config), ZN_CONFIG_PEER_KEY, argv[1]);
    }

    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s))
    {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_keyexpr_t rid = z_declare_expr(z_loan(s), z_expr("/test/thr"));
    z_owned_subscriber_t sub = z_subscribe(z_loan(s), rid, z_subinfo_default(), data_handler, NULL);
    if (!z_check(sub))
    {
        printf("Unable to create subscriber.\n");
        exit(-1);
    }

    char c = 0;
    while (c != 'q')
    {
        c = fgetc(stdin);
    }

    z_subscriber_close(z_move(sub));
    z_undeclare_expr(z_loan(s), rid);
    z_close(z_move(s));
    return 0;
}