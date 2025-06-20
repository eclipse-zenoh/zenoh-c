#include <stdio.h>
#include <string.h>

#include "parse_args.h"
#include "zenoh.h"

struct args_t {
    bool no_express;  // --no-express
};
struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);

void callback(z_loaned_sample_t* sample, void* context) {
    const z_loaned_publisher_t* pub = z_loan(*(z_owned_publisher_t*)context);
    z_owned_bytes_t payload;
    z_bytes_clone(&payload, z_sample_payload(sample));
    z_publisher_put(pub, z_move(payload), NULL);
}
void drop(void* context) {
    z_owned_publisher_t* pub = (z_owned_publisher_t*)context;
    z_drop(z_move(*pub));
    // A note on lifetimes:
    //  here, `sub` takes ownership of `pub` and will drop it before returning from its own `drop`,
    //  which makes passing a pointer to the stack safe as long as `sub` is dropped in a scope where `pub` is still
    //  valid.
}

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);

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
    if (z_declare_publisher(z_loan(session), &pub, z_loan(pong), &opts) < 0) {
        printf("Unable to declare publisher for key expression!\n");
        exit(-1);
    }

    z_owned_closure_sample_t respond;
    z_closure(&respond, callback, drop, (void*)&pub);
    if (z_declare_background_subscriber(z_loan(session), z_loan(ping), z_move(respond), NULL) < 0) {
        printf("Unable to declare background subscriber for key expression!\n");
        exit(-1);
    }

    while (1) {
        z_sleep_s(1);
    }

    z_drop(z_move(session));
}

void print_help() {
    printf(
        "\
    Usage: z_pong [OPTIONS]\n\n\
    Options:\n\
        --no-express (optional): Disable message batching.\n");
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    args.no_express = _Z_CHECK_FLAG("no-express");
    parse_zenoh_common_args(argc, argv, config);
    const char* arg = check_unknown_opts(argc, argv);
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
    return args;
}
