//
// Copyright (c) 2025 ZettaScale Technology
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
#include <stdio.h>
#include <string.h>

#include "parse_args.h"
#include "zenoh.h"

#define DEFAULT_KEYEXPR "demo/example/**"
#define STORAGE_NUM_BINS 256

struct node_t;
typedef struct node_t node_t;

struct node_t {
    z_owned_sample_t sample;
    node_t* next;
    node_t* prev;
};

typedef struct {
    node_t** nodes;
    size_t num_bins;
} storage_t;

typedef struct {
    size_t bin;
    node_t* node;
} storage_iterator_t;

storage_t storage;
z_owned_mutex_t storage_mutex;

struct args_t {
    char* keyexpr;  // -k, --key
    bool complete;  // --complete
};

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config);
const char* kind_to_str(z_sample_kind_t kind);

storage_t storage_new(size_t num_bins);
void storage_insert(storage_t* storage, const z_loaned_sample_t* sample);
z_owned_sample_t* storage_find(storage_t* storage, const z_loaned_string_t* key);
void storage_remove(storage_t* storage, const z_loaned_keyexpr_t* keyexpr);
void storage_drop(storage_t* storage);
storage_iterator_t storage_begin(storage_t* storage);
bool storage_is_end(storage_t* storage, storage_iterator_t it);
z_owned_sample_t* storage_iterator_value(storage_iterator_t it);
storage_iterator_t storage_iterator_next(storage_t* storage, storage_iterator_t it);

void sub_handler(z_loaned_sample_t* sample, void* arg) {
    z_view_string_t key_string;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key_string);

    z_owned_string_t payload_string;
    z_bytes_to_string(z_sample_payload(sample), &payload_string);

    printf(">> [Subscriber] Received %s ('%.*s': '%.*s')\n", kind_to_str(z_sample_kind(sample)),
           (int)z_string_len(z_loan(key_string)), z_string_data(z_loan(key_string)),
           (int)z_string_len(z_loan(payload_string)), z_string_data(z_loan(payload_string)));
    z_drop(z_move(payload_string));
    z_mutex_lock(z_loan_mut(storage_mutex));
    switch (z_sample_kind(sample)) {
        case Z_SAMPLE_KIND_PUT:
            storage_insert(&storage, sample);
            break;
        case Z_SAMPLE_KIND_DELETE:
            storage_remove(&storage, z_sample_keyexpr(sample));
            break;
    }
    z_mutex_unlock(z_loan_mut(storage_mutex));
}

void query_handler(z_loaned_query_t* query, void* context) {
    z_mutex_lock(z_loan_mut(storage_mutex));
    storage_iterator_t it = storage_begin(&storage);
    while (!storage_is_end(&storage, it)) {
        const z_loaned_sample_t* sample = z_loan(*storage_iterator_value(it));
        if (z_keyexpr_intersects(z_query_keyexpr(query), z_sample_keyexpr(sample))) {
            z_owned_bytes_t payload;
            z_bytes_clone(&payload, z_sample_payload(sample));
            z_query_reply(query, z_sample_keyexpr(sample), z_move(payload), NULL);
        }
        it = storage_iterator_next(&storage, it);
    }
    z_mutex_unlock(z_loan_mut(storage_mutex));
}

int main(int argc, char** argv) {
    zc_init_log_from_env_or("error");

    z_mutex_init(&storage_mutex);
    storage = storage_new(STORAGE_NUM_BINS);

    z_owned_config_t config;
    struct args_t args = parse_args(argc, argv, &config);
    z_view_keyexpr_t ke;
    z_view_keyexpr_from_str(&ke, args.keyexpr);

    printf("Opening session...\n");
    z_owned_session_t s;
    if (z_open(&s, z_move(config), NULL) < 0) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_owned_closure_sample_t sub_callback;
    z_closure(&sub_callback, sub_handler, NULL, NULL);
    printf("Declaring Subscriber on '%s'...\n", args.keyexpr);
    z_owned_subscriber_t sub;
    if (z_declare_subscriber(z_loan(s), &sub, z_loan(ke), z_move(sub_callback), NULL) < 0) {
        printf("Unable to declare subscriber.\n");
        exit(-1);
    }

    printf("Declaring Queryable on '%s'...\n", args.keyexpr);
    z_owned_closure_query_t query_callback;
    z_closure(&query_callback, query_handler, NULL, (void*)args.keyexpr);
    z_owned_queryable_t qable;

    z_queryable_options_t opts;
    z_queryable_options_default(&opts);
    opts.complete = args.complete;

    if (z_declare_queryable(z_loan(s), &qable, z_loan(ke), z_move(query_callback), &opts) < 0) {
        printf("Unable to create queryable.\n");
        exit(-1);
    }

    printf("Press CTRL-C to quit...\n");
    while (1) {
        z_sleep_s(1);
    }

    z_drop(z_move(qable));
    z_drop(z_move(sub));
    z_drop(z_move(s));
    z_mutex_drop(z_move(storage_mutex));
    storage_drop(&storage);
    return 0;
}

void print_help() {
    printf(
        "\
    Usage: z_storage [OPTIONS]\n\n\
    Options:\n\
        -k, --key <KEYEXPR> (optional, string, default='%s'): The selection of resources to store\n\
        --complete (optional): Declare the storage as complete w.r.t. the key expression",
        DEFAULT_KEYEXPR);
    printf(COMMON_HELP);
}

struct args_t parse_args(int argc, char** argv, z_owned_config_t* config) {
    _Z_CHECK_HELP;
    struct args_t args;
    _Z_PARSE_ARG(args.keyexpr, "k", "key", (char*), (char*)DEFAULT_KEYEXPR);
    args.complete = _Z_CHECK_FLAG("complete");
    parse_zenoh_common_args(argc, argv, config);
    const char* unknown_arg = check_unknown_opts(argc, argv);
    if (unknown_arg) {
        printf("Unknown option %s\n", unknown_arg);
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

size_t _storage_hash_string(const z_loaned_string_t* s) {
    size_t hash = 5381;
    size_t len = z_string_len(s);
    const char* data = z_string_data(s);
    for (size_t i = 0; i < len; ++i) {
        hash = ((hash << 5) + hash) + (size_t)data[i];
    }
    return hash;
}

size_t _storage_compare_string(const z_loaned_string_t* s1, const z_loaned_string_t* s2) {
    size_t len1 = z_string_len(s1);
    const char* data1 = z_string_data(s1);
    size_t len2 = z_string_len(s2);
    const char* data2 = z_string_data(s2);
    return len1 == len2 && strncmp(data1, data2, len1) == 0;
}

node_t* _storage_find(node_t* hash_bin, const z_loaned_string_t* key) {
    node_t* node = hash_bin;
    while (node != NULL) {
        z_view_string_t key_string;
        z_keyexpr_as_view_string(z_sample_keyexpr(z_loan(node->sample)), &key_string);
        if (_storage_compare_string(z_loan(key_string), key)) {
            return node;
        }
        node = node->next;
    }
    return NULL;
}

z_owned_sample_t* storage_find(storage_t* storage, const z_loaned_string_t* key) {
    size_t hash = _storage_hash_string(key);
    size_t bin = hash % storage->num_bins;
    node_t* res = _storage_find(storage->nodes[bin], key);
    if (res == NULL) {
        return NULL;
    }
    return &res->sample;
}

void storage_insert(storage_t* storage, const z_loaned_sample_t* sample) {
    z_view_string_t key;
    z_keyexpr_as_view_string(z_sample_keyexpr(sample), &key);

    size_t hash = _storage_hash_string(z_loan(key));
    size_t bin = hash % storage->num_bins;
    node_t* res = _storage_find(storage->nodes[bin], z_loan(key));
    if (res != NULL) {
        z_drop(z_move(res->sample));
    } else if (storage->nodes[bin] == NULL) {
        res = (node_t*)z_malloc(sizeof(node_t));
        res->prev = NULL;
        res->next = NULL;
        storage->nodes[bin] = res;
    } else {
        res = storage->nodes[bin];
        while (res->next != NULL) res = res->next;
        res->next = (node_t*)z_malloc(sizeof(node_t));
        res->next->prev = res;
        res = res->next;
        res->next = NULL;
    }
    z_sample_clone(&res->sample, sample);
}

void storage_remove(storage_t* storage, const z_loaned_keyexpr_t* keyexpr) {
    z_view_string_t key;
    z_keyexpr_as_view_string(keyexpr, &key);

    size_t hash = _storage_hash_string(z_loan(key));
    size_t bin = hash % storage->num_bins;
    node_t* res = _storage_find(storage->nodes[bin], z_loan(key));
    if (res == NULL) {
        return;
    }
    if (res->prev == NULL) {
        storage->nodes[bin] = res->next;
    } else {
        res->prev->next = res->next;
    }
    if (res->next != NULL) {
        res->next->prev = res->prev;
    }

    z_drop(z_move(res->sample));
    z_free(res);
}

storage_t storage_new(size_t num_bins) {
    storage_t storage;
    storage.num_bins = num_bins;
    storage.nodes = (node_t**)z_malloc(sizeof(node_t*) * num_bins);
    for (size_t i = 0; i < num_bins; ++i) {
        storage.nodes[i] = NULL;
    }
    return storage;
}

void storage_drop(storage_t* storage) {
    for (size_t i = 0; i < storage->num_bins; ++i) {
        node_t* node = storage->nodes[i];
        while (node != NULL) {
            z_drop(z_move(node->sample));
            node_t* to_free = node;
            node = node->next;
            z_free(to_free);
        }
    }
    z_free(storage->nodes);
}

storage_iterator_t _storage_first_non_empty_bin(storage_t* storage, size_t start) {
    while (start < storage->num_bins) {
        if (storage->nodes[start] != NULL) {
            return (storage_iterator_t){.bin = start, .node = storage->nodes[start]};
        }
        start++;
    }
    return (storage_iterator_t){.bin = start, .node = NULL};
}

storage_iterator_t storage_begin(storage_t* storage) { return _storage_first_non_empty_bin(storage, 0); }

bool storage_is_end(storage_t* storage, storage_iterator_t it) { return it.bin >= storage->num_bins; }

z_owned_sample_t* storage_iterator_value(storage_iterator_t it) { return &it.node->sample; }

storage_iterator_t storage_iterator_next(storage_t* storage, storage_iterator_t it) {
    if (it.node->next != NULL) {
        return (storage_iterator_t){.bin = it.bin, .node = it.node->next};
    }
    return _storage_first_non_empty_bin(storage, it.bin + 1);
}

const char* kind_to_str(z_sample_kind_t kind) {
    switch (kind) {
        case Z_SAMPLE_KIND_PUT:
            return "PUT";
        case Z_SAMPLE_KIND_DELETE:
            return "DELETE";
        default:
            return "UNKNOWN";
    }
}
