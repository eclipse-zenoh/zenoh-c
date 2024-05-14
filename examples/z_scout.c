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
    int len = 0;
    for (int i = 0; i < 16; i++) {
        if (pid.id[i]) {
            len = i + 1;
        }
    }
    if (!len) {
        fprintf(stream, "None");
    } else {
        fprintf(stream, "Some(");
        for (unsigned int i = 0; i < len; i++) {
            fprintf(stream, "%02X", (int)pid.id[i]);
        }
        fprintf(stream, ")");
    }
}

void fprintwhatami(FILE *stream, unsigned int whatami) {
    char buf[64];
    z_whatami_to_str(whatami, buf, sizeof(buf));
    fprintf(stream, "%s", buf);
}

void fprintlocators(FILE *stream, const z_loaned_slice_array_t *locs) {
    fprintf(stream, "[");
    for (unsigned int i = 0; i < z_slice_array_len(locs); i++) {
        fprintf(stream, "\"");
        const z_loaned_slice_t *loc = z_slice_array_get(locs, i);
        fprintf(stream, "%.*s", (int)z_slice_len(loc), (const char*)z_slice_data(loc));
        fprintf(stream, "\"");
        if (i < z_slice_array_len(locs) - 1) {
            fprintf(stream, ", ");
        }
    }
    fprintf(stream, "]");
}

void fprinthello(FILE *stream, const z_loaned_hello_t* hello) {
    fprintf(stream, "Hello { pid: ");
    fprintpid(stream, z_hello_zid(hello));
    fprintf(stream, ", whatami: ");
    fprintwhatami(stream, z_hello_whatami(hello));

    fprintf(stream, ", locators: ");
    z_owned_slice_array_t locators;
    z_hello_locators(hello, &locators);
    fprintlocators(stream, z_loan(locators));
    z_slice_array_drop(z_move(locators));

    fprintf(stream, " }");
}

void callback(const z_loaned_hello_t *hello, void *context) {
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