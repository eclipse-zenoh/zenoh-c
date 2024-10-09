#include <stdio.h>
#include <string.h>

#include "parse_args.h"
#include "zenoh.h"

void parse_args(int argc, char** argv, z_owned_config_t* config);

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
    parse_args(argc, argv, &config);

    z_owned_session_t session;
    z_open(&session, z_move(config), NULL);
    z_view_keyexpr_t ping;
    z_view_keyexpr_from_str_unchecked(&ping, "test/ping");
    z_view_keyexpr_t pong;
    z_view_keyexpr_from_str_unchecked(&pong, "test/pong");
    z_owned_publisher_t pub;
    z_publisher_declare(&pub, z_loan(session), z_loan(pong), NULL);
    z_owned_closure_sample_t respond;
    z_closure(&respond, callback, drop, (void*)&pub);
    z_subscriber_declare_background(z_loan(session), z_loan(ping), z_move(respond), NULL);

    while (1) {
        z_sleep_s(1);
    }

    z_drop(z_move(session));
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
}
