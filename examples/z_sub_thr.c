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
#include <sys/time.h>
#include "zenoh.h"

#define N 100000

volatile unsigned long long int count = 0;
volatile struct timeval start;
volatile struct timeval stop;

void print_stats(volatile struct timeval *start, volatile struct timeval *stop)
{
    double t0 = start->tv_sec + ((double)start->tv_usec / 1000000.0);
    double t1 = stop->tv_sec + ((double)stop->tv_usec / 1000000.0);
    double thpt = N / (t1 - t0);
    printf("%f msgs/sec\n", thpt);
}

void data_handler(const z_sample_t *sample, const void *arg)
{
    struct timeval tv;
    if (count == 0)
    {
        gettimeofday(&tv, 0);
        start = tv;
        count++;
    }
    else if (count < N)
    {
        count++;
    }
    else
    {
        gettimeofday(&tv, 0);
        stop = tv;
        print_stats(&start, &stop);
        count = 0;
    }
}

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

    z_reskey_t rid = z_rid(z_register_resource(s.borrow, z_rname("/test/thr")));
    z_owned_subscriber_t sub = z_register_subscriber(s.borrow, rid, z_subinfo__default(), data_handler, NULL);
    if (sub.borrow == 0)
    {
        printf("Unable to declare subscriber.\n");
        exit(-1);
    }

    char c = 0;
    while (c != 'q')
    {
        c = fgetc(stdin);
    }

    z_unregister_subscriber(sub);
    z_close(s);
    return 0;
}