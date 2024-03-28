#include <stdio.h>
#include <string.h>

#include "parse_args.h"
#include "zenoh.h"

void parse_args(int argc, char** argv, z_owned_config_t* config);

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

int main(int argc, char** argv) {
    z_owned_config_t config = z_config_default();
    parse_args(argc, argv, &config);

    z_owned_session_t session = z_open(z_move(config));
    z_keyexpr_t ping = z_keyexpr_unchecked("test/ping");
    z_keyexpr_t pong = z_keyexpr_unchecked("test/pong");
    z_owned_publisher_t pub = z_declare_publisher(z_loan(session), pong, NULL);
    z_owned_closure_sample_t respond = z_closure(callback, drop, (void*)z_move(pub));
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(session), ping, z_move(respond), NULL);
    while (1) {
        z_sleep_s(1);
    }
    z_drop(z_move(sub));
    z_close(z_move(session));
}

void print_help() {
    printf(
        "\
    Usage: z_pong [OPTIONS]\n\n\
    Options:\n");
    printf(COMMON_HELP);
    printf(
        "\
        -h: print help\n");
}

void parse_args(int argc, char** argv, z_owned_config_t* config) {
    if (parse_opt(argc, argv, "h", false)) {
        print_help();
        exit(1);
    }
    parse_zenoh_common_args(argc, argv, config);
    char* arg = check_unknown_opts(argc, argv);
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
}
