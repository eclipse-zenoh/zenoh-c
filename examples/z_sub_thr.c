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

#define N 1000000

typedef struct
{
    volatile unsigned long count;
    volatile unsigned long finished_rounds;
    volatile clock_t start;
    volatile clock_t stop;
    volatile clock_t first_start;
} z_stats_t;

void z_stats_init(z_stats_t *stats)
{
    stats->count = 0;
    stats->finished_rounds = 0;
    stats->first_start = 0;
}

void data_handler(const z_sample_t *sample, const void *arg)
{
    z_stats_t *stats = (z_stats_t *)arg;
    if (stats->count == 0)
    {
        stats->start = clock();
        if (!stats->first_start)
        {
            stats->first_start = stats->start;
        }
        stats->count++;
    }
    else if (stats->count < N)
    {
        stats->count++;
    }
    else
    {
        stats->stop = clock();
        stats->finished_rounds++;
        printf("%f msg/s\n", N * (double)CLOCKS_PER_SEC / (double)(stats->stop - stats->start));
        stats->count = 0;
    }
}
void drop_stats(void *arg)
{
    const clock_t end = clock();
    const z_stats_t *stats = (z_stats_t *)arg;
    const double elapsed = (double)(end - stats->first_start) / (double)CLOCKS_PER_SEC;
    const unsigned long sent_messages = N * stats->finished_rounds + stats->count;
    printf("Stats being dropped after unsubscribing: sent %ld messages over %f seconds (%f msg/s)\n", sent_messages, elapsed, (double)sent_messages / elapsed);
    free(arg);
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

    z_owned_keyexpr_t ke = z_declare_keyexpr(z_loan(s), z_keyexpr("test/thr"));

    z_owned_closure_sample_t callback = {.this_ = malloc(sizeof(z_stats_t)), .call = data_handler, .drop = drop_stats};
    z_stats_init(callback.this_);
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_loan(ke), z_move(callback), NULL);
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
    z_undeclare_keyexpr(z_loan(s), z_move(ke));
    z_close(z_move(s));
    return 0;
}
