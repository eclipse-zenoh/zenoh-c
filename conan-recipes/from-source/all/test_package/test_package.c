#include "zenoh.h"

int main(int argc, char **argv) {
    (void)argc;
    (void)argv;
    z_owned_config_t config = z_config_default();
    return EXIT_SUCCESS;
}