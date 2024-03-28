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

#include "parse_args.h"
#include "zenoh.h"

#define N 1000000

void parse_args(int argc, char **argv, z_owned_config_t *config);

typedef struct {
    volatile unsigned long count;
    volatile unsigned long finished_rounds;
    z_clock_t start;
    z_clock_t first_start;
    bool started;
} z_stats_t;

z_stats_t *z_stats_make() {
    z_stats_t *stats = z_malloc(sizeof(z_stats_t));
    stats->count = 0;
    stats->finished_rounds = 0;
    stats->started = false;
    return stats;
}

void on_sample(const z_sample_t *sample, void *context) {
    z_stats_t *stats = (z_stats_t *)context;
    if (stats->count == 0) {
        stats->start = z_clock_now();
        if (!stats->started) {
            stats->first_start = stats->start;
            stats->started = true;
        }
        stats->count++;
    } else if (stats->count < N) {
        stats->count++;
    } else {
        stats->finished_rounds++;
        printf("%f msg/s\n", 1000.0 * N / z_clock_elapsed_ms(&stats->start));
        stats->count = 0;
    }
}
void drop_stats(void *context) {
    const z_stats_t *stats = (z_stats_t *)context;
    const unsigned long sent_messages = N * stats->finished_rounds + stats->count;
    double elapsed_s = z_clock_elapsed_s(&stats->first_start);
    printf("Stats being dropped after unsubscribing: sent %ld messages over %f seconds (%f msg/s)\n", sent_messages,
           elapsed_s, (double)sent_messages / elapsed_s);
    z_free(context);
}

int main(int argc, char **argv) {
    z_owned_config_t config = z_config_default();
    parse_args(argc, argv, &config);

    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_owned_keyexpr_t ke = z_declare_keyexpr(z_loan(s), z_keyexpr("test/thr"));

    z_stats_t *context = z_stats_make();
    z_owned_closure_sample_t callback = z_closure(on_sample, drop_stats, context);
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_loan(ke), z_move(callback), NULL);
    if (!z_check(sub)) {
        printf("Unable to create subscriber.\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    while (1) {
        z_sleep_s(1);
    }

    z_undeclare_subscriber(z_move(sub));
    z_undeclare_keyexpr(z_loan(s), z_move(ke));
    z_close(z_move(s));
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_sub_thr [OPTIONS]\n\n\
    Options:\n");
    printf(COMMON_HELP);
    printf(
        "\
        -h: print help\n");
}

void parse_args(int argc, char **argv, z_owned_config_t *config) {
    if (parse_opt(argc, argv, "h", false)) {
        print_help();
        exit(1);
    }
    parse_zenoh_common_args(argc, argv, config);
    char *arg = check_unknown_opts(argc, argv);
    if (arg) {
        printf("Unknown option %s\n", arg);
        exit(-1);
    }
    char **pos_args = parse_pos_args(argc, argv, 1);
    if (!pos_args || pos_args[0]) {
        printf("Unexpected positional arguments\n");
        free(pos_args);
        exit(-1);
    }
    free(pos_args);
}
