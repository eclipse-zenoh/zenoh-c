#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_PKT_SIZE 8
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

void callback(const z_loaned_sample_t* sample, void* context) { z_condvar_signal(z_loan(cond)); }
void drop(void* context) { z_drop(z_move(cond)); }

struct args_t {
    unsigned int size;             // -s
    unsigned int number_of_pings;  // -n
    unsigned int warmup_ms;        // -w
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

int main(int argc, char** argv) {
    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

    z_mutex_init(&mutex);
    z_condvar_init(&cond);
    z_owned_session_t session;
    z_open(&session, z_move(config), NULL);
    z_view_keyexpr_t ping;
    z_view_keyexpr_from_str_unchecked(&ping, "test/ping");
    z_view_keyexpr_t pong;
    z_view_keyexpr_from_str_unchecked(&pong, "test/pong");
    z_owned_publisher_t pub;
    z_declare_publisher(&pub, z_loan(session), z_loan(ping), NULL);
    z_owned_closure_sample_t respond;
    z_closure(&respond, callback, drop, (void*)(&pub));
    z_owned_subscriber_t sub;
    z_declare_subscriber(&sub, z_loan(session), z_loan(pong), z_move(respond), NULL);
    uint8_t* data = z_malloc(args.size);
    for (int i = 0; i < args.size; i++) {
        data[i] = i % 10;
    }
    z_owned_bytes_t payload;

    z_mutex_lock(z_loan_mut(mutex));
    if (args.warmup_ms) {
        printf("Warming up for %dms...\n", args.warmup_ms);
        z_clock_t warmup_start = z_clock_now();

        unsigned long elapsed_us = 0;
        while (elapsed_us < args.warmup_ms * 1000) {
            z_bytes_from_buf(&payload, data, args.size, NULL, NULL);
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
        z_bytes_from_buf(&payload, data, args.size, NULL, NULL);
        z_clock_t measure_start = z_clock_now();
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
    z_free(data);
    z_undeclare_subscriber(z_move(sub));
    z_undeclare_publisher(z_move(pub));
    z_drop(z_move(mutex));
    z_close(z_move(session), NULL);
}

void print_help() {
    printf(
        "\
    Usage: z_ping [OPTIONS]\n\n\
    Options:\n\
        -n <SAMPLES> (optional, int, default=%d): The number of pings to be attempted\n\
        -s <SIZE> (optional, int, default=%d): The size of the payload embedded in the ping and repeated by the pong\n\
        -w <WARMUP> (optional, int, default=%d): The warmup time in ms during which pings will be emitted but not measured\n",
        DEFAULT_PKT_SIZE, DEFAULT_PING_NB, DEFAULT_WARMUP_MS);
    printf(COMMON_HELP);
    printf(
        "\
        -h: print help\n");
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    if (parse_opt(argc, argv, "h", false)) {
        print_help();
        exit(1);
    }
    const char* arg = parse_opt(argc, argv, "s", true);
    unsigned int size = DEFAULT_PKT_SIZE;
    if (arg) {
        size = atoi(arg);
    }
    arg = parse_opt(argc, argv, "n", true);
    unsigned int number_of_pings = DEFAULT_PING_NB;
    if (arg) {
        number_of_pings = atoi(arg);
    }
    arg = parse_opt(argc, argv, "w", true);
    unsigned int warmup_ms = DEFAULT_WARMUP_MS;
    if (arg) {
        warmup_ms = atoi(arg);
    }
    parse_zenoh_common_args(argc, argv, config);
    arg = check_unknown_opts(argc, argv);
    if (arg) {
        printf("Unknown option %s\n", arg);
        exit(-1);
    }
    char** pos_args = parse_pos_args(argc, argv, 1);
    if (!pos_args || pos_args[0]) {
        printf("Unexpected positional arguments\n");
        free(pos_args);
        exit(-1);
    }
    free(pos_args);
    return (struct args_t){.size = size, .number_of_pings = number_of_pings, .warmup_ms = warmup_ms};
}
