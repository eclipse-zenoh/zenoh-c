//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>

#include <stdio.h>

#include "zenoh.h"

void fprintpid(FILE *stream, z_id_t pid) {
    z_owned_string_t str;
    z_id_to_string(&pid, &str);
    fprintf(stream, "%.*s", (int)z_string_len(z_loan(str)), z_string_data(z_loan(str)));
    z_drop(z_move(str));
}

void fprintwhatami(FILE *stream, z_whatami_t whatami) {
    z_view_string_t whatami_str;
    z_whatami_to_view_string(whatami, &whatami_str);
    fprintf(stream, "%.*s", (int)z_string_len(z_loan(whatami_str)), z_string_data(z_loan(whatami_str)));
}

void fprintlocators(FILE *stream, const z_loaned_string_array_t *locs) {
    fprintf(stream, "[");
    for (unsigned int i = 0; i < z_string_array_len(locs); i++) {
        const z_loaned_string_t *loc = z_string_array_get(locs, i);
        fprintf(stream, "%.*s", (int)z_string_len(loc), z_string_data(loc));
        if (i < z_string_array_len(locs) - 1) {
            fprintf(stream, ", ");
        }
    }
    fprintf(stream, "]");
}

void fprinthello(FILE *stream, const z_loaned_hello_t *hello) {
    fprintf(stream, "Hello { pid: ");

    fprintpid(stream, z_hello_zid(hello));
    fprintf(stream, ", whatami: ");
    fprintwhatami(stream, z_hello_whatami(hello));

    fprintf(stream, ", locators: ");
    z_owned_string_array_t locators;
    z_hello_locators(hello, &locators);
    fprintlocators(stream, z_loan(locators));
    z_string_array_drop(z_move(locators));

    fprintf(stream, " }");
}

void callback(z_loaned_hello_t *hello, void *context) {
    fprinthello(stdout, hello);
    fprintf(stdout, "\n");
    (*(int *)context)++;
}

void drop(void *context) {
    printf("Dropping scout\n");
    int count = *(int *)context;
    z_free(context);
    if (!count) {
        printf("Did not find any zenoh process.\n");
    }
}

int main(int argc, char **argv) {
    zc_init_log_from_env_or("error");

    int *context = z_malloc(sizeof(int));
    *context = 0;
    z_owned_config_t config;
    z_config_default(&config);

    z_owned_closure_hello_t closure;
    z_closure(&closure, callback, drop, context);
    printf("Scouting...\n");
    z_scout(z_move(config), z_move(closure), NULL);
    z_sleep_s(1);
    return 0;
}