#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_PING_NB 100
#define DEFAULT_WARMUP_MS 1000
#define PING_TIMEOUT_SEC 1

#define handle_error_en(en, msg) \
    do {                         \
        errno = en;              \
        perror(msg);             \
        exit(EXIT_FAILURE);      \
    } while (0)

z_owned_condvar_t cond;
z_owned_mutex_t mutex;

void callback(z_loaned_sample_t* sample, void* context) { z_condvar_signal(z_loan(cond)); }
void drop(void* context) { z_drop(z_move(cond)); }

struct args_t {
    unsigned int size;             // positional_0
    unsigned int number_of_pings;  // -n, --samples
    unsigned int warmup_ms;        // -w, --warmup
    bool no_express;               // --no-express
};

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");
    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

    z_mutex_init(&mutex);
    z_condvar_init(&cond);

    z_owned_session_t session;
    if (z_open(&session, z_move(config), NULL) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_view_keyexpr_t ping;
    z_view_keyexpr_from_str_unchecked(&ping, "test/ping");
    z_view_keyexpr_t pong;
    z_view_keyexpr_from_str_unchecked(&pong, "test/pong");
    z_publisher_options_t opts;
    z_publisher_options_default(&opts);
    opts.is_express = !args.no_express;

    z_owned_publisher_t pub;
    if (z_declare_publisher(z_loan(session), &pub, z_loan(ping), &opts) < 0) {
        printf("Unable to declare publisher for key expression!\n");
        exit(-1);
    }

    z_owned_closure_sample_t respond;
    z_closure(&respond, callback, drop, (void*)(&pub));

    z_owned_subscriber_t sub;
    if (z_declare_subscriber(z_loan(session), &sub, z_loan(pong), z_move(respond), NULL) < 0) {
        printf("Unable to declare subscriber for key expression!\n");
        exit(-1);
    }

    // Create SHM Provider
    z_owned_shm_provider_t provider;
    z_shm_provider_default_new(&provider, args.size);

    // Allocate SHM Buffer
    z_buf_layout_alloc_result_t alloc;
    z_shm_provider_alloc(&alloc, z_loan(provider), args.size);
    if (alloc.status != ZC_BUF_LAYOUT_ALLOC_STATUS_OK) {
        printf("Unexpected failure during SHM buffer allocation...");
        return -1;
    }
    // Fill SHM Buffer with data
    uint8_t* data = z_shm_mut_data_mut(z_loan_mut(alloc.buf));
    for (int i = 0; i < args.size; i++) {
        data[i] = i % 10;
    }
    // Convert mutable SHM Buffer into immutable one (to be able to make it's ref copies)
    z_owned_shm_t shm;
    z_shm_from_mut(&shm, z_move(alloc.buf));

    z_owned_bytes_t shmbs;
    if (z_bytes_from_shm(&shmbs, z_move(shm)) != Z_OK) {
        printf("Unexpected failure during SHM buffer serialization...\n");
        return -1;
    }

    z_mutex_lock(z_loan_mut(mutex));
    if (args.warmup_ms) {
        printf("Warming up for %dms...\n", args.warmup_ms);
        z_clock_t warmup_start = z_clock_now();

        unsigned long elapsed_us = 0;
        while (elapsed_us < args.warmup_ms * 1000) {
            z_owned_bytes_t payload;
            z_bytes_clone(&payload, z_loan(shmbs));
            z_publisher_put(z_loan(pub), z_move(payload), NULL);
            int s = z_condvar_wait(z_loan(cond), z_loan_mut(mutex));
            if (s != 0) {
                handle_error_en(s, "z_condvar_wait");
            }
            elapsed_us = z_clock_elapsed_us(&warmup_start);
        }
    }
    unsigned long* results = z_malloc(sizeof(unsigned long) * args.number_of_pings);
    for (int i = 0; i < args.number_of_pings; i++) {
        z_clock_t measure_start = z_clock_now();
        z_owned_bytes_t payload;
        z_bytes_clone(&payload, z_loan(shmbs));
        z_publisher_put(z_loan(pub), z_move(payload), NULL);
        int s = z_condvar_wait(z_loan(cond), z_loan_mut(mutex));
        if (s != 0) {
            handle_error_en(s, "z_condvar_wait");
        }
        results[i] = z_clock_elapsed_us(&measure_start);
    }
    for (int i = 0; i < args.number_of_pings; i++) {
        printf("%d bytes: seq=%d rtt=%luµs, lat=%luµs\n", args.size, i, results[i], results[i] / 2);
    }
    z_mutex_unlock(z_loan_mut(mutex));
    z_free(results);
    z_drop(z_move(sub));
    z_drop(z_move(pub));
    z_drop(z_move(mutex));
    z_drop(z_move(session));

    z_drop(z_move(shm));
    z_drop(z_move(provider));
}

void print_help() {
    printf(
        "\
    Usage: z_ping [OPTIONS] <PAYLOAD_SIZE>\n\n\
    Arguments:\n\
        <PAYLOAD_SIZE> (required, number): Size of the payload to publish\n\n\
    Options:\n\
        -n, --samples <SAMPLES> (optional, int, default=%d): The number of pings to be attempted\n\
        -w, --warmup <WARMUP> (optional, int, default=%d): The warmup time in ms during which pings will be emitted but not measured\n\
        --no-express (optional): Disable message batching.\n",
        DEFAULT_PING_NB, DEFAULT_WARMUP_MS);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.number_of_pings, "n", "samples", atoi, DEFAULT_PING_NB);
    _Z_PARSE_ARG(args.warmup_ms, "w", "warmup", atoi, DEFAULT_WARMUP_MS);
    args.no_express = _Z_CHECK_FLAG("no-express");

    parse_zenoh_common_args(argc, argv, config);
    const char* arg = check_unknown_opts(argc, argv);
    if (arg) {
        printf("Unknown option %s\n", arg);
        exit(-1);
    }
    char** pos_args = parse_pos_args(argc, argv, 1);
    if (!pos_args[0]) {
        printf("<PAYLOAD_SIZE> argument is required\n");
        free(pos_args);
        exit(-1);
    }
    args.size = atoi(pos_args[0]);
    free(pos_args);
    return args;
}
