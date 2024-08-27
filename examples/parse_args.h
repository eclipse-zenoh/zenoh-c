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
        -m <MODE> (optional, string, default='peer'): The zenoh session mode. [possible values: peer, client, router]\n\
        -e <CONNECT> (optional, string): endpoint to connect to. Repeat option to pass multiple endpoints. If none are given, endpoints will be discovered through multicast-scouting if it is enabled.\n\
            e.g.: '-e tcp/192.168.1.1:7447'\n\
        -l <LISTEN> (optional, string): locator to listen on. Repeat option to pass multiple locators. If none are given, the default configuration will be used.\n\
            e.g.: '-l tcp/192.168.1.1:7447'\n\
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
const char* parse_opt(int argc, char** argv, const char* opt, bool opt_has_value) {
    size_t optlen = strlen(opt);
    for (int i = 1; i < argc; i++) {
        if (argv[i] == NULL) {
            continue;
        }
        size_t len = strlen(argv[i]);
        if (len < 2) {
            continue;
        }
        if (optlen == 1) {
            if (argv[i][0] == '-' && argv[i][1] == opt[0]) {
                argv[i] = NULL;
                if (!opt_has_value) {
                    return (char*)opt;
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
                    return (char*)opt;
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
    return NULL;
}

/**
 * Check if any options remains in `argv`. Must be called after all expected options are parsed
 * @param argc
 * @param argv
 * @returns NULL if no option was found, else the first option string that was found
 */
const char* check_unknown_opts(int argc, char** const argv) {
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
char** parse_pos_args(const int argc, char** argv, const size_t nb_args) {
    char** pos_argv = (char**)calloc(nb_args, sizeof(char*));
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
 * Parse zenoh options that require a JSON-serialized list (-e, -l from common args) and add them to
 * `config`. Prints error message and exits if fails to insert parsed values
 * @param argc
 * @param argv
 * @param opt: option to parse (without `-` or `--` prefix)
 * @param config: address of an owned zenoh configuration
 * @param config_key: zenoh configuration key under which the parsed values will be inserted
 */
void parse_zenoh_json_list_config(int argc, char** argv, const char* opt, const char* config_key,
                                  z_owned_config_t* config) {
    char* buf = (char*)calloc(1, sizeof(char));
    const char* value = parse_opt(argc, argv, opt, true);
    while (value) {
        size_t len_newbuf = strlen(buf) + strlen(value) + 4;  // value + quotes + comma + nullbyte
        char* newbuf = (char*)malloc(len_newbuf);
        snprintf(newbuf, len_newbuf, "%s'%s',", buf, value);
        free(buf);
        buf = newbuf;
        value = parse_opt(argc, argv, opt, true);
    }
    size_t buflen = strlen(buf);
    if (buflen > 0) {
        // remove trailing comma
        buf[buflen - 1] = '\0';
        buflen--;
        // add list delimiters
        size_t json_list_len = buflen + 3;  // buf + brackets + nullbyte
        char* json_list = (char*)malloc(json_list_len);
        snprintf(json_list, json_list_len, "[%s]", buf);
        // insert in config
        if (zc_config_insert_json(z_loan_mut(*config), config_key, json_list) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`\n`%s` is either not a JSON-serialized list of "
                "strings, or values within the list do not respect expected format for `%s`\n",
                json_list, config_key, json_list, config_key);
            free(json_list);
            exit(-1);
        }
        free(json_list);
    }
    free(buf);
}

/**
 * Parse zenoh options that are common to all examples (-c, -m, -e, -l, --no-multicast-scouting) and add them to
 * `config`
 * @param argc
 * @param argv
 * @param config: address of an owned zenoh configuration
 */
void parse_zenoh_common_args(const int argc, char** argv, z_owned_config_t* config) {
    // -c: A configuration file.
    const char* config_file = parse_opt(argc, argv, "c", true);
    if (config_file) {
        zc_config_from_file(config, config_file);
    } else {
        z_config_default(config);
    }
    // -m: The Zenoh session mode [default: peer].
    const char* mode = parse_opt(argc, argv, "m", true);
    if (mode) {
        size_t buflen = strlen(mode) + 3;  // mode + quotes + nullbyte
        char* buf = (char*)malloc(buflen);
        snprintf(buf, buflen, "'%s'", mode);
        if (zc_config_insert_json(z_loan_mut(*config), Z_CONFIG_MODE_KEY, buf) < 0) {
            printf(
                "Couldn't insert value `%s` in configuration at `%s`. Value must be one of: 'client', 'peer' or "
                "'router'\n",
                mode, Z_CONFIG_MODE_KEY);
            free(buf);
            exit(-1);
        }
        free(buf);
    }
    // -e: Endpoint to connect to. Can be repeated
    parse_zenoh_json_list_config(argc, argv, "e", Z_CONFIG_CONNECT_KEY, config);
    // -l: Endpoint to listen on. Can be repeated
    parse_zenoh_json_list_config(argc, argv, "l", Z_CONFIG_LISTEN_KEY, config);
    // --no-multicast-scrouting: Disable the multicast-based scouting mechanism.
    const char* no_multicast_scouting = parse_opt(argc, argv, "no-multicast-scouting", false);
    if (no_multicast_scouting &&
        zc_config_insert_json(z_loan_mut(*config), Z_CONFIG_MULTICAST_SCOUTING_KEY, "false") < 0) {
        printf("Couldn't disable multicast-scouting.\n");
        exit(-1);
    }
}
