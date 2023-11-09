#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include "zenoh.h"

#define DEFAULT_PKT_SIZE 8
#define DEFAULT_PING_NB 100
#define DEFAULT_WARMUP_MS 1000

pthread_cond_t cond;
pthread_mutex_t mutex;
struct timespec ping_timeout = {.tv_sec = 1, .tv_nsec = 0};

void callback(const z_sample_t* sample, void* context) { pthread_cond_signal(&cond); }
void drop(void* context) { pthread_cond_destroy(&cond); }

struct args_t {
    unsigned int size;             // -s
    unsigned int number_of_pings;  // -n
    unsigned int warmup_ms;        // -w
    char* config_path;             // -c
    uint8_t help_requested;        // -h
};
struct args_t parse_args(int argc, char** argv);

int main(int argc, char** argv) {
    struct args_t args = parse_args(argc, argv);
    if (args.help_requested) {
        printf(
            "\
		-n (optional, int, default=%d): the number of pings to be attempted\n\
		-s (optional, int, default=%d): the size of the payload embedded in the ping and repeated by the pong\n\
		-w (optional, int, default=%d): the warmup time in ms during which pings will be emitted but not measured\n\
		-c (optional, string): the path to a configuration file for the session. If this option isn't passed, the default configuration will be used.\n\
		",
            DEFAULT_PKT_SIZE, DEFAULT_PING_NB, DEFAULT_WARMUP_MS);
        return 1;
    }
    pthread_mutex_init(&mutex, NULL);
    pthread_cond_init(&cond, NULL);
    z_owned_config_t config = args.config_path ? zc_config_from_file(args.config_path) : z_config_default();
    z_owned_session_t session = z_open(z_move(config));
    z_keyexpr_t ping = z_keyexpr_unchecked("test/ping");
    z_keyexpr_t pong = z_keyexpr_unchecked("test/pong");
    z_owned_publisher_t pub = z_declare_publisher(z_loan(session), ping, NULL);
    z_owned_closure_sample_t respond = z_closure(callback, drop, (void*)(&pub));
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(session), pong, z_move(respond), NULL);
    uint8_t* data = malloc(args.size);
    for (int i = 0; i < args.size; i++) {
        data[i] = i % 10;
    }
    pthread_mutex_lock(&mutex);
    if (args.warmup_ms) {
        printf("Warming up for %dms...\n", args.warmup_ms);
        struct timespec wmup_start, wmup_stop;
        clock_gettime(CLOCK_MONOTONIC, &wmup_start);
        unsigned long elapsed_us = 0;
        while (elapsed_us < args.warmup_ms * 1000) {
            z_publisher_put(z_loan(pub), data, args.size, NULL);
            pthread_cond_timedwait(&cond, &mutex, &ping_timeout);
            clock_gettime(CLOCK_MONOTONIC, &wmup_stop);
            elapsed_us =
                (1000000 * (wmup_stop.tv_sec - wmup_start.tv_sec) + (wmup_stop.tv_nsec - wmup_start.tv_nsec) / 1000);
        }
    }
    struct timespec t_start, t_stop;
    unsigned long* results = malloc(sizeof(unsigned long) * args.number_of_pings);
    for (int i = 0; i < args.number_of_pings; i++) {
        clock_gettime(CLOCK_MONOTONIC, &t_start);
        z_publisher_put(z_loan(pub), data, args.size, NULL);
        pthread_cond_timedwait(&cond, &mutex, &ping_timeout);
        clock_gettime(CLOCK_MONOTONIC, &t_stop);
        results[i] = (1000000 * (t_stop.tv_sec - t_start.tv_sec) + (t_stop.tv_nsec - t_start.tv_nsec) / 1000);
    }
    for (int i = 0; i < args.number_of_pings; i++) {
        printf("%d bytes: seq=%d rtt=%luµs, lat=%luµs\n", args.size, i, results[i], results[i] / 2);
    }
    pthread_mutex_unlock(&mutex);
    free(results);
    free(data);
    z_drop(z_move(sub));
    z_drop(z_move(pub));
    z_close(z_move(session));
}

char* getopt(int argc, char** argv, char option) {
    for (int i = 0; i < argc; i++) {
        size_t len = strlen(argv[i]);
        if (len >= 2 && argv[i][0] == '-' && argv[i][1] == option) {
            if (len > 2 && argv[i][2] == '=') {
                return argv[i] + 3;
            } else if (i + 1 < argc) {
                return argv[i + 1];
            }
        }
    }
    return NULL;
}

struct args_t parse_args(int argc, char** argv) {
    for (int i = 0; i < argc; i++) {
        if (strcmp(argv[i], "-h") == 0) {
            return (struct args_t){.help_requested = 1};
        }
    }
    char* arg = getopt(argc, argv, 's');
    unsigned int size = DEFAULT_PKT_SIZE;
    if (arg) {
        size = atoi(arg);
    }
    arg = getopt(argc, argv, 'n');
    unsigned int number_of_pings = DEFAULT_PING_NB;
    if (arg) {
        number_of_pings = atoi(arg);
    }
    arg = getopt(argc, argv, 'w');
    unsigned int warmup_ms = DEFAULT_WARMUP_MS;
    if (arg) {
        warmup_ms = atoi(arg);
    }
    return (struct args_t){.help_requested = 0,
                           .size = size,
                           .number_of_pings = number_of_pings,
                           .warmup_ms = warmup_ms,
                           .config_path = getopt(argc, argv, 'c')};
}