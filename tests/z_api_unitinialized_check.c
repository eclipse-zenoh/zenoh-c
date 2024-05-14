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

#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

int main(int argc, char **argv) {
    z_owned_keyexpr_t owned_keyexpr;
    z_keyexpr_from_string(&owned_keyexpr, NULL);
    assert(!z_check(owned_keyexpr));

    z_view_keyexpr_t keyexpr;
    z_view_keyexpr_from_string(&keyexpr, NULL);
    assert(!z_check(keyexpr));
}
