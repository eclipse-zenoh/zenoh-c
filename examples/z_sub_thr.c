//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
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

void data_handler(const z_sample_t sample, const void *arg)
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
        if (!z_config_insert_json(z_loan(config), Z_CONFIG_CONNECT_KEY, argv[1]))
        {
            printf("Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a JSON-serialized list of strings\n", argv[1], Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
            exit(-1);
        }
    }

    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s))
    {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_owned_keyexpr_t ke = z_declare_keyexpr(z_loan(s), z_keyexpr("/test/thr"));
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_loan(ke), data_handler, NULL);
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

    z_undeclare_subscriber(z_move(sub));
    z_undeclare_keyexpr(z_move(ke));
    z_close(z_move(s));
    return 0;
}
