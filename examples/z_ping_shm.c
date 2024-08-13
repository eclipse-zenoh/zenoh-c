#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

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
    z_mutex_init(&mutex);
    z_condvar_init(&cond);
    z_owned_config_t config;
    if (args.config_path) {
        zc_config_from_file(&config, args.config_path);
    } else {
        z_config_default(&config);
    }

    // A probing procedure for shared memory is performed upon session opening. To operate over shared memory
    // (and to not fallback on network mode), shared memory needs to be enabled in the configuration.
    if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_SHARED_MEMORY_KEY, "true") < 0) {
        printf(
            "Couldn't insert value `true` in configuration at `%s`. This is likely because `%s` expects a "
            "JSON-serialized value\n",
            Z_CONFIG_SHARED_MEMORY_KEY, Z_CONFIG_SHARED_MEMORY_KEY);
        exit(-1);
    }

    z_owned_session_t session;
    z_open(&session, z_move(config));
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

    // Create SHM Provider
    z_alloc_alignment_t alignment = {0};
    z_owned_memory_layout_t layout;
    z_memory_layout_new(&layout, args.size, alignment);
    z_owned_shm_provider_t provider;
    z_posix_shm_provider_new(&provider, z_loan(layout));

    // Allocate SHM Buffer
    z_buf_layout_alloc_result_t alloc;
    z_shm_provider_alloc(&alloc, z_loan(provider), args.size, alignment);
    if (!z_check(alloc.buf)) {
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
    if (z_bytes_serialize_from_shm(&shmbs, z_move(shm)) != Z_OK) {
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
    z_undeclare_subscriber(z_move(sub));
    z_undeclare_publisher(z_move(pub));
    z_drop(z_move(mutex));
    z_close(z_move(session));

    z_drop(z_move(shm));
    z_drop(z_move(provider));
    z_drop(z_move(layout));
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