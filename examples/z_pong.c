#include <stdio.h>
#include <string.h>

#include "zenoh.h"

void callback(const z_sample_t* sample, void* context) {
    z_publisher_t pub = z_loan(*(z_owned_publisher_t*)context);
#ifdef ZENOH_C  // The zc_owned_payload_t API is exclusive to zenoh-c, but allows avoiding some copies.
    zc_owned_payload_t payload = zc_sample_payload_rcinc(sample);
    zc_publisher_put_owned(pub, z_move(payload), NULL);
#else
    z_publisher_put(pub, sample->payload.start, sample->payload.len, NULL);
#endif
}
void drop(void* context) {
    z_owned_publisher_t* pub = (z_owned_publisher_t*)context;
    z_drop(pub);
    // A note on lifetimes:
    //  here, `sub` takes ownership of `pub` and will drop it before returning from its own `drop`,
    //  which makes passing a pointer to the stack safe as long as `sub` is dropped in a scope where `pub` is still
    //  valid.
}
struct args_t {
    char* config_path;             // -c
    uint8_t help_requested;        // -h
};
struct args_t parse_args(int argc, char** argv);

int main(int argc, char** argv) {
    struct args_t args = parse_args(argc, argv);
    if (args.help_requested) {
        printf("-c (optional, string): the path to a configuration file for the session. If this option isn't passed, the default configuration will be used.\n");
        return 1;
    }
    z_owned_config_t config = args.config_path ? zc_config_from_file(args.config_path) : z_config_default();
    z_owned_session_t session = z_open(z_move(config));
    z_keyexpr_t ping = z_keyexpr_unchecked("test/ping");
    z_keyexpr_t pong = z_keyexpr_unchecked("test/pong");
    z_owned_publisher_t pub = z_declare_publisher(z_loan(session), pong, NULL);
    z_owned_closure_sample_t respond = z_closure(callback, drop, (void*)z_move(pub));
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(session), ping, z_move(respond), NULL);
    while (getchar() != 'q') {
    }
    z_drop(z_move(sub));
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