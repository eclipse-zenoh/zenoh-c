#include <stdio.h>
#include <string.h>

#include "zenoh.h"

void callback(const z_loaned_sample_t* sample, void* context) {
    const z_loaned_publisher_t* pub = z_loan(*(z_owned_publisher_t*)context);
#ifdef ZENOH_C  // The z_owned_bytes_t API is exclusive to zenoh-c, but allows avoiding some copies.
    z_owned_bytes_t payload;
    z_bytes_clone(&payload, z_sample_payload(sample));
    z_publisher_put(pub, z_move(payload), NULL);
#endif
}
void drop(void* context) {
    z_owned_publisher_t* pub = (z_owned_publisher_t*)context;
    z_undeclare_publisher(z_move(*pub));
    // A note on lifetimes:
    //  here, `sub` takes ownership of `pub` and will drop it before returning from its own `drop`,
    //  which makes passing a pointer to the stack safe as long as `sub` is dropped in a scope where `pub` is still
    //  valid.
}
struct args_t {
    char* config_path;       // -c
    uint8_t help_requested;  // -h
};
struct args_t parse_args(int argc, char** argv);

int main(int argc, char** argv) {
    struct args_t args = parse_args(argc, argv);
    if (args.help_requested) {
        printf(
            "-c (optional, string): the path to a configuration file for the session. If this option isn't passed, the "
            "default configuration will be used.\n");
        return 1;
    }
    z_owned_config_t config;
    if (args.config_path) {
        zc_config_from_file(&config, args.config_path);
    } else {
        z_config_default(&config);
    }

#if (defined(SHARED_MEMORY) && defined(UNSTABLE))
    // A probing procedure for shared memory is performed upon session opening. To operate over shared memory
    // (and to not fallback on network mode), shared memory needs to be enabled in the configuration.
    if (zc_config_insert_json(z_loan_mut(config), Z_CONFIG_SHARED_MEMORY_KEY, "true") < 0) {
        printf(
            "Couldn't insert value `true` in configuration at `%s`. This is likely because `%s` expects a "
            "JSON-serialized value\n",
            Z_CONFIG_SHARED_MEMORY_KEY, Z_CONFIG_SHARED_MEMORY_KEY);
        exit(-1);
    }
#endif

    z_owned_session_t session;
    z_open(&session, z_move(config));
    z_view_keyexpr_t ping;
    z_view_keyexpr_from_str_unchecked(&ping, "test/ping");
    z_view_keyexpr_t pong;
    z_view_keyexpr_from_str_unchecked(&pong, "test/pong");
    z_owned_publisher_t pub;
    z_declare_publisher(&pub, z_loan(session), z_loan(pong), NULL);
    z_owned_closure_sample_t respond;
    z_closure(&respond, callback, drop, (void*)z_move(pub));
    z_owned_subscriber_t sub;
    z_declare_subscriber(&sub, z_loan(session), z_loan(ping), z_move(respond), NULL);
    while (getchar() != 'q') {
    }
    z_undeclare_subscriber(z_move(sub));
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
    return (struct args_t){.help_requested = 0, .config_path = getopt(argc, argv, 'c')};
}