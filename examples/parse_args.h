//
// Copyright (c) 2024 ZettaScale Technology
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
//

#pragma once

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "zenoh.h"

#define COMMON_HELP \
    "\
        -c <CONFIG> (optional, string): The path to a configuration file for the session. If this option isn't passed, the default configuration will be used.\n\
        -m <MODE> (optional, string, default='peer'): JSON-serialized string of the zenoh session mode. [possible values: 'peer', 'client', 'router']\n\
        -e <CONNECT> (optional, string): JSON-serialized list of locators to connect to. If none are given, endpoints will be discovered through multicast-scouting if it is enabled.\n\
        -l <LISTEN> (optional, string): JSON-serialized list of locators to listen on. If none are given, the default configuration will be used.\n\
        --no-multicast-scouting (optional): By default zenohd replies to multicast scouting messages for being discovered by peers and clients. This option disables this feature.\n\
"

/**
 * Parse an option of format `-f`, `--flag`, `-f <value>` or `--flag <value>` from `argv`. If found, the option and its
 * eventual value are each replaced by NULL in `argv`
 * @param argc: argc passed from `main` function
 * @param argv: argv passed from `main` function
 * @param opt: option to parse (without `-` or `--` prefix)
 * @param opt_has_value: if true, the option is of format `-f <value>` or `--flag <value>` and `value` will be returned
 * if found, else an error message is printed and program will exit. If false, option has no value and a non-null
 * pointer will be returned if option is found.
 * @returns NULL if option was not found, else a non-null value depending on if `opt_has_value`.
 */
char* parse_opt(int argc, char** argv, char* opt, bool opt_has_value) {
    size_t optlen = strlen(opt);
    for (int i = 1; i < argc; i++) {
        if (argv[i] == NULL) {
            continue;
        }
        size_t len = strlen(argv[i]);
        if (len >= 2) {
            if (optlen == 1) {
                if (argv[i][0] == '-' && argv[i][1] == opt[0]) {
                    argv[i] = NULL;
                    if (!opt_has_value) {
                        return opt;
                    } else if (i + 1 < argc && argv[i + 1]) {
                        char* value = argv[i + 1];
                        argv[i + 1] = NULL;
                        return value;
                    } else {
                        printf("Option -%s given without a value\n", opt);
                        exit(-1);
                    }
                }
            } else if (optlen > 1 && len > 3 && argv[i][0] == '-' && argv[i][1] == '-') {
                // Note: support for '--arg=<value>' syntax can be added here
                if (strcmp(argv[i] + 2, opt) == 0) {
                    argv[i] = NULL;
                    if (!opt_has_value) {
                        return opt;
                    } else if (i + 1 < argc && argv[i + 1]) {
                        char* value = argv[i + 1];
                        argv[i + 1] = NULL;
                        return value;
                    } else {
                        printf("Option --%s given without a value\n", opt);
                        exit(-1);
                    }
                }
            }
        }
    }
    return NULL;
}

/**
 * Check if any options remains in `argv`. Must be called after all expected options are parsed
 * @param argc
 * @param argv
 * @returns NULL if no option was found, else the first option string that was found
 */
char* check_unknown_opts(int argc, char** argv) {
    for (int i = 1; i < argc; i++) {
        if (argv[i] && argv[i][0] == '-') {
            return argv[i];
        }
    }
    return NULL;
}

/**
 * Parse positional arguments from `argv`. Must be called after all expected options are parsed, and after checking that
 * no unknown options remain in `argv`
 * @param argc
 * @param argv
 * @param nb_args: number of expected positional arguments
 * @returns NULL if found more positional arguments than `nb_args`. Else an array of found arguments in order, followed
 * by NULL values if found less positional arguments than `nb_args`
 * @note Returned pointer is dynamically allocated and must be freed
 */
char** parse_pos_args(int argc, char** argv, size_t nb_args) {
    char** pos_argv = (char**)malloc(nb_args * sizeof(char*));
    // Initialize pointers to NULL to detect when example is called with number of args < nb_args
    for (int i = 0; i < nb_args; i++) {
        pos_argv[i] = NULL;
    }
    size_t pos_argc = 0;
    for (int i = 1; i < argc; i++) {
        if (argv[i]) {
            pos_argc++;
            if (pos_argc > nb_args) {
                free(pos_argv);
                return NULL;
            }
            pos_argv[pos_argc - 1] = argv[i];
        }
    }
    return pos_argv;
}

/**
 * Parse zenoh options that are common to all examples (-c, -m, -e, -l, --no-multicast-scouting) and add them to
 * `config`
 * @param argc
 * @param argv
 * @param config: address of an owned zenoh configuration
 */
void parse_zenoh_common_args(int argc, char** argv, z_owned_config_t* config) {
    // -c: A configuration file.
    char* config_file = parse_opt(argc, argv, "c", true);
    if (config_file) {
        *config = zc_config_from_file(config_file);
    }
    // -m: The Zenoh session mode [default: peer].
    char* mode = parse_opt(argc, argv, "m", true);
    if (mode && zc_config_insert_json(z_loan(*config), Z_CONFIG_MODE_KEY, mode) < 0) {
        printf(
            "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
            "JSON-serialized string. Value must be one of: 'client', 'peer' or 'router'\n",
            mode, Z_CONFIG_MODE_KEY, Z_CONFIG_MODE_KEY);
        exit(-1);
    }
    // -e: Endpoints to connect to.
    char* connect = parse_opt(argc, argv, "e", true);
    if (connect && zc_config_insert_json(z_loan(*config), Z_CONFIG_CONNECT_KEY, connect) < 0) {
        printf(
            "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
            "JSON-serialized list of strings\n",
            connect, Z_CONFIG_CONNECT_KEY, Z_CONFIG_CONNECT_KEY);
        exit(-1);
    }
    // -l: Endpoints to listen on.
    char* listen = parse_opt(argc, argv, "l", true);
    if (listen && zc_config_insert_json(z_loan(*config), Z_CONFIG_LISTEN_KEY, listen) < 0) {
        printf(
            "Couldn't insert value `%s` in configuration at `%s`. This is likely because `%s` expects a "
            "JSON-serialized list of strings\n",
            listen, Z_CONFIG_LISTEN_KEY, Z_CONFIG_LISTEN_KEY);
        exit(-1);
    }
    // --no-multicast-scrouting: Disable the multicast-based scouting mechanism.
    char* no_multicast_scouting = parse_opt(argc, argv, "no-multicast-scouting", false);
    if (no_multicast_scouting && zc_config_insert_json(z_loan(*config), Z_CONFIG_MULTICAST_SCOUTING_KEY, "false") < 0) {
        printf("Couldn't disable multicast-scouting.\n");
        exit(-1);
    }
}
