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

void fprintlocators(FILE *stream, const z_str_array_t *locs) {
    fprintf(stream, "[");
    for (unsigned int i = 0; i < locs->len; i++) {
        fprintf(stream, "\"");
        fprintf(stream, "%s", locs->val[i]);
        fprintf(stream, "\"");
        if (i < locs->len - 1) {
            fprintf(stream, ", ");
        }
    }
    fprintf(stream, "]");
}

void fprinthello(FILE *stream, const z_hello_t hello) {
    fprintf(stream, "Hello { pid: ");
    fprintpid(stream, hello.pid);
    fprintf(stream, ", whatami: ");
    fprintwhatami(stream, hello.whatami);
    fprintf(stream, ", locators: ");
    fprintlocators(stream, &hello.locators);
    fprintf(stream, " }");
}

void callback(z_owned_hello_t *hello, void *context) {
    z_hello_t lhello = z_loan(*hello);
    fprinthello(stdout, lhello);
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
    z_owned_scouting_config_t config = z_scouting_config_default();
    z_owned_closure_hello_t closure = z_closure(callback, drop, context);
    printf("Scouting...\n");
    z_scout(z_move(config), z_move(closure));
    z_sleep_s(1);
    return 0;
}