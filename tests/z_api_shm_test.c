//
// Copyright (c) 2023 ZettaScale Technology
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
#include <string.h>

#include "zenoh.h"

#define ASSERT_OK(result) \
    if (result != Z_OK) { \
        assert(false);    \
        return result;    \
    }

#define ASSERT_TRUE(expr) \
    if (!(expr)) {        \
        assert(false);    \
        return -300;      \
    }

#define ASSERT_CHECK(var) \
    if (!z_check(var)) {  \
        assert(false);    \
        return -100;      \
    }

#define ASSERT_CHECK_ERR(var) \
    if (z_check(var)) {       \
        assert(false);        \
        return -200;          \
    }

int test_shm_buffer(z_owned_shm_mut_t* buf) {
    ASSERT_CHECK(*buf);

    { z_loaned_shm_mut_t* loaned = z_loan_mut(*buf); }

    z_owned_shm_t immut;
    z_shm_from_mut(&immut, z_move(*buf));
    ASSERT_CHECK(immut);
    ASSERT_CHECK_ERR(*buf);

    { const z_loaned_shm_t* loaned = z_loan(immut); }

    {
        z_loaned_shm_t* loaned_immut = z_loan_mut(immut);
        z_loaned_shm_mut_t* loaned_mut = z_shm_try_reloan_mut(loaned_immut);
        ASSERT_TRUE(loaned_mut != NULL);
    }

    {
        z_loaned_shm_mut_t* loaned_mut = z_shm_try_mut(&immut);
        ASSERT_TRUE(loaned_mut != NULL);
    }

    z_owned_shm_t immut2;
    z_shm_clone(z_loan(immut), &immut2);
    ASSERT_CHECK(immut2);

    z_owned_shm_mut_t mut;
    z_shm_mut_try_from_immut(&mut, z_move(immut2));
    ASSERT_CHECK_ERR(immut2);
    ASSERT_CHECK_ERR(mut);

    z_shm_mut_try_from_immut(&mut, z_move(immut));
    ASSERT_CHECK(mut);

    z_drop(z_move(mut));
    ASSERT_CHECK_ERR(mut);
}

bool test_layouted_allocation(const z_loaned_alloc_layout_t* alloc_layout) {
    z_owned_buf_alloc_result_t alloc;

    z_alloc_layout_alloc_gc(&alloc, alloc_layout);
    ASSERT_CHECK(alloc);

    z_owned_shm_mut_t shm_buf;
    z_alloc_error_t shm_error;

    ASSERT_OK(z_buf_alloc_result_unwrap(z_move(alloc), &shm_buf, &shm_error));
    if (z_check(shm_buf)) {
        ASSERT_OK(test_shm_buffer(z_move(shm_buf)));
        ASSERT_CHECK_ERR(shm_buf);
        return true;
    } else
        return false;
}

bool test_allocation(const z_loaned_shm_provider_t* provider, size_t size, z_alloc_alignment_t alignment) {
    z_owned_buf_alloc_result_t alloc;

    ASSERT_OK(z_shm_provider_alloc_gc(&alloc, provider, size, alignment));
    ASSERT_CHECK(alloc);

    z_owned_shm_mut_t shm_buf;
    z_alloc_error_t shm_error;

    ASSERT_OK(z_buf_alloc_result_unwrap(z_move(alloc), &shm_buf, &shm_error));
    if (z_check(shm_buf)) {
        ASSERT_OK(test_shm_buffer(z_move(shm_buf)));
        ASSERT_CHECK_ERR(shm_buf);
        return true;
    } else
        return false;
}

int test_provider(z_owned_shm_provider_t* provider, z_alloc_alignment_t alignment, size_t buf_ok_size,
                  size_t buf_err_size) {
    // test allocation OK
    for (int i = 0; i < 100; ++i) {
        ASSERT_TRUE(test_allocation(z_loan(*provider), buf_ok_size, alignment));
    }

    // test allocation ERROR
    if (buf_err_size) {
        ASSERT_TRUE(!test_allocation(z_loan(*provider), buf_err_size, alignment));
    }

    // OK layouted allocations
    {
        // make OK allocation layout
        z_owned_alloc_layout_t alloc_layout;
        ASSERT_OK(z_alloc_layout_new(&alloc_layout, z_loan(*provider), buf_ok_size, alignment));
        ASSERT_CHECK(alloc_layout);
        // test layouted allocation OK
        for (int i = 0; i < 100; ++i) {
            ASSERT_TRUE(test_layouted_allocation(z_loan(alloc_layout)));
        }
        z_drop(z_move(alloc_layout));
        ASSERT_CHECK_ERR(alloc_layout);
    }

    // ERR layouted allocation
    if (buf_err_size) {
        // make ERR allocation layout
        z_owned_alloc_layout_t alloc_layout;
        ASSERT_OK(z_alloc_layout_new(&alloc_layout, z_loan(*provider), buf_err_size, alignment));
        ASSERT_CHECK(alloc_layout);
        // test layouted allocation ERROR
        ASSERT_TRUE(!test_layouted_allocation(z_loan(alloc_layout)));
        z_drop(z_move(alloc_layout));
        ASSERT_CHECK_ERR(alloc_layout);
    }

    // additional functions
    z_shm_provider_defragment(z_loan(*provider));
    z_shm_provider_garbage_collect(z_loan(*provider));

    return Z_OK;
}

typedef struct {
    uint8_t* bytes;
    bool* busy_flags;
    size_t count;
    size_t available;
} test_provider_context;

void delete_fn(void* context) {
    test_provider_context* c = (test_provider_context*)context;
    free(c->bytes);
    free(c->busy_flags);
    c->bytes = NULL;
    c->busy_flags = NULL;
}
void alloc_fn(struct z_owned_chunk_alloc_result_t* result, const struct z_loaned_memory_layout_t* layout,
              void* context) {
    assert(context);
    assert(layout);
    assert(result);

    // check size and alignment
    size_t size = 0;
    z_alloc_alignment_t alignment;
    z_memory_layout_get_data(&size, &alignment, layout);
    assert(size == 1);
    assert(alignment.pow == 0);

    // perform allocation
    test_provider_context* c = (test_provider_context*)context;
    for (int i = 0; i < c->count; ++i) {
        if (!c->busy_flags[i]) {
            c->busy_flags[i] = true;
            c->available--;

            z_allocated_chunk_t chunk;
            chunk.data = &c->bytes[i];
            uint64_t ptr = (uint64_t)(chunk.data);
            chunk.descriptpr.chunk = ptr & 0xFFFFFFFF;
            chunk.descriptpr.len = 1;
            chunk.descriptpr.segment = (ptr >> 32) & 0xFFFFFFFF;

            z_chunk_alloc_result_new_ok(result, chunk);
            return;
        }
    }
    z_chunk_alloc_result_new_error(result, Z_ALLOC_ERROR_OUT_OF_MEMORY);
}
void free_fn(const struct z_chunk_descriptor_t* chunk, void* context) {
    assert(context);
    assert(chunk);

    assert(chunk->len == 1);

    // restore data ptr from chunk descriptor
    void* data = (void*)(((uint64_t)chunk->chunk) | ((((uint64_t)chunk->segment) << 32) & 0xFFFFFFFF00000000));

    // calc index from data ptr
    test_provider_context* c = (test_provider_context*)context;
    int64_t index = (int64_t)data - (int64_t)c->bytes;
    assert(index >= 0);
    assert(index < c->count);

    // mark this entry as free
    c->busy_flags[index] = false;
    c->available++;
}
size_t defragment_fn(void* context) {
    assert(context);
    return Z_OK;
}
size_t available_fn(void* context) {
    assert(context);

    test_provider_context* c = (test_provider_context*)context;
    return c->available;
}
void layout_for_fn(struct z_owned_memory_layout_t* layout, void* context) {
    assert(context);
    assert(layout);

    assert(z_check(*layout));

    // check size and alignment
    size_t size = 0;
    z_alloc_alignment_t alignment;
    z_memory_layout_get_data(&size, &alignment, z_loan(*layout));

    if (size != 1 || alignment.pow != 0) {
        z_memory_layout_drop(layout);
    }
}

int run_c_provider() {
    const z_protocol_id_t id = 100500;
    const size_t size = 1024;

    // init test context
    test_provider_context test_context;
    test_context.available = size;
    test_context.count = size;
    test_context.busy_flags = malloc(sizeof(bool) * size);
    test_context.bytes = malloc(sizeof(uint8_t) * size);
    zc_context_t context = {&test_context, &delete_fn};

    // init callbacks
    zc_shm_provider_backend_callbacks_t callbacks = {&alloc_fn, &free_fn, &defragment_fn, &available_fn,
                                                     &layout_for_fn};
    // create provider
    z_owned_shm_provider_t provider;
    z_shm_provider_new(&provider, id, context, callbacks);
    ASSERT_CHECK(provider)

    // test provider
    z_alloc_alignment_t alignment = {0};
    ASSERT_OK(test_provider(&provider, alignment, 1, 0));

    // drop provider
    z_drop(z_move(provider));
    ASSERT_CHECK_ERR(provider);

    // check that delete_fn executed
    ASSERT_TRUE(test_context.busy_flags == NULL);
    ASSERT_TRUE(test_context.bytes == NULL);

    return Z_OK;
}

int run_posix_provider() {
    const size_t total_size = 4096;
    const size_t buf_ok_size = total_size / 4;
    const size_t buf_err_size = total_size * 2;

    z_alloc_alignment_t alignment = {4};

    z_owned_memory_layout_t layout;
    ASSERT_OK(z_memory_layout_new(&layout, total_size, alignment));
    ASSERT_CHECK(layout);

    z_owned_shm_provider_t provider;
    ASSERT_OK(z_posix_shm_provider_new(&provider, z_loan(layout)));
    ASSERT_CHECK(provider);

    ASSERT_OK(test_provider(&provider, alignment, buf_ok_size, buf_err_size));

    z_drop(z_move(provider));
    ASSERT_CHECK_ERR(provider);

    z_drop(z_move(layout));
    ASSERT_CHECK_ERR(layout);

    return Z_OK;
}

int test_client_storage(z_owned_shm_client_storage_t* storage) {
    ASSERT_CHECK(*storage);

    z_owned_config_t config;
    z_config_default(&config);
    ASSERT_CHECK(config);

    z_owned_session_t session;
    ASSERT_OK(z_open_with_custom_shm_clients(&session, z_move(config), z_loan(*storage)));

    ASSERT_CHECK(session);
    z_drop(z_move(session));
    ASSERT_CHECK_ERR(session);

    return Z_OK;
}

int run_default_client_storage() {
    z_owned_shm_client_storage_t storage;
    ASSERT_OK(z_shm_client_storage_new_default(&storage));

    // test client storage
    ASSERT_OK(test_client_storage(&storage));

    // deref the client storage
    z_drop(z_move(storage));
    ASSERT_CHECK_ERR(storage);

    return Z_OK;
}

int run_global_client_storage() {
    // obtain defaul global client storage
    z_owned_shm_client_storage_t storage;
    ASSERT_OK(z_ref_shm_client_storage_global(&storage));

    // test client storage
    ASSERT_OK(test_client_storage(&storage));

    // deref the client storage
    z_drop(z_move(storage));
    ASSERT_CHECK_ERR(storage);

    return Z_OK;
}

int run_client_storage() {
    // create client list
    zc_owned_shm_client_list_t list;
    ASSERT_OK(zc_shm_client_list_new(&list));
    ASSERT_CHECK(list);

    // create POSIX SHM Client
    z_owned_shm_client_t client;
    ASSERT_OK(z_posix_shm_client_new(&client));
    ASSERT_CHECK(client);

    // add client to the list
    ASSERT_OK(zc_shm_client_list_add_client(Z_SHM_POSIX_PROTOCOL_ID, z_move(client), z_loan_mut(list)));
    ASSERT_CHECK_ERR(client);

    // create client storage from the list
    z_owned_shm_client_storage_t storage;
    ASSERT_OK(z_shm_client_storage_new(&storage, z_loan(list), true));

    // test client storage
    ASSERT_OK(test_client_storage(&storage));

    // deref the client storage
    z_drop(z_move(storage));
    ASSERT_CHECK_ERR(storage);

    // drop the client list
    z_drop(z_move(list));
    ASSERT_CHECK_ERR(list);
    return Z_OK;
}

void delete_client_fn(void* context) { assert(context == NULL); }
void delete_segment_fn(void* context) {}

uint8_t* map_fn(z_chunk_id_t chunk, void* context) {
    return (uint8_t*)((uint64_t)chunk | ((((uint64_t)context) << 32) & 0xFFFFFFFF00000000));
}

bool attach_fn(struct z_shm_segment_t* out_segment, z_segment_id_t id, void* context) {
    assert(context == NULL);
    out_segment->context.context.ptr = (void*)(uint64_t)id;
    out_segment->context.delete_fn = &delete_segment_fn;
    out_segment->callbacks.map_fn = &map_fn;
    return true;
}

int run_c_client() {
    // create client list
    zc_owned_shm_client_list_t list;
    ASSERT_OK(zc_shm_client_list_new(&list));
    ASSERT_CHECK(list);

    // create C SHM Client
    zc_threadsafe_context_t context = {NULL, &delete_client_fn};
    zc_shm_client_callbacks_t callbacks = {&attach_fn};
    z_owned_shm_client_t client;
    ASSERT_OK(z_shm_client_new(&client, context, callbacks));
    ASSERT_CHECK(client);

    // add client to the list
    ASSERT_OK(zc_shm_client_list_add_client(100500, z_move(client), z_loan_mut(list)));
    ASSERT_CHECK_ERR(client);

    // create client storage from the list
    z_owned_shm_client_storage_t storage;
    ASSERT_OK(z_shm_client_storage_new(&storage, z_loan(list), true));
    ASSERT_CHECK(storage);

    // drop the client storage
    z_drop(z_move(storage));
    ASSERT_CHECK_ERR(storage);

    // drop the client list
    z_drop(z_move(list));
    ASSERT_CHECK_ERR(list);
    return Z_OK;
}

int main() {
    ASSERT_OK(run_posix_provider());
    ASSERT_OK(run_c_provider());
    ASSERT_OK(run_default_client_storage());
    ASSERT_OK(run_global_client_storage());
    ASSERT_OK(run_client_storage());
    ASSERT_OK(run_c_client());
    return Z_OK;
}
