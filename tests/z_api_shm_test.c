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

#define ASSERT_ERR(result) \
    if (result == Z_OK) {  \
        assert(false);     \
        return result;     \
    }

#define ASSERT_TRUE(expr) \
    if (!(expr)) {        \
        assert(false);    \
        return -300;      \
    }

#define ASSERT_CHECK(var)         \
    if (!z_internal_check(var)) { \
        assert(false);            \
        return -100;              \
    }

#define ASSERT_CHECK_ERR(var)    \
    if (z_internal_check(var)) { \
        assert(false);           \
        return -200;             \
    }

int test_shm_buffer(z_moved_shm_mut_t* mbuf) {
    ASSERT_CHECK(mbuf->_this);
    z_owned_shm_mut_t buf;
    z_take(&buf, mbuf);
    ASSERT_CHECK_ERR(mbuf->_this);
    ASSERT_CHECK(buf);

    {
        z_loaned_shm_mut_t* loaned = z_loan_mut(buf);
    }

    z_owned_shm_t immut;
    z_shm_from_mut(&immut, z_move(buf));
    ASSERT_CHECK(immut);
    ASSERT_CHECK_ERR(buf);

    {
        const z_loaned_shm_t* loaned = z_loan(immut);
    }

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
    z_shm_clone(&immut2, z_loan(immut));
    ASSERT_CHECK(immut2);

    z_owned_shm_mut_t mut;
    ASSERT_TRUE(Z_EUNAVAILABLE == z_shm_mut_try_from_immut(&mut, z_move(immut2), &immut2));
    ASSERT_CHECK(immut2);

    z_drop(z_move(immut2));

    ASSERT_TRUE(Z_OK == z_shm_mut_try_from_immut(&mut, z_move(immut), &immut));
    ASSERT_CHECK(mut);
    ASSERT_CHECK_ERR(immut);

    z_drop(z_move(mut));
    ASSERT_CHECK_ERR(mut);
    return 0;
}

int test_layouted_allocation(const z_loaned_alloc_layout_t* alloc_layout) {
    z_buf_alloc_result_t alloc;

    z_owned_shm_mut_t shm_buf;
    z_alloc_error_t shm_error;

    z_alloc_layout_alloc_gc(&alloc, alloc_layout);
    if (alloc.status == ZC_BUF_ALLOC_STATUS_OK) {
        ASSERT_CHECK(alloc.buf);
        ASSERT_OK(test_shm_buffer(z_move(alloc.buf)));
        ASSERT_CHECK_ERR(alloc.buf);
        return Z_OK;
    } else
        return Z_ENULL;
}

int test_allocation(const z_loaned_shm_provider_t* provider, size_t size, z_alloc_alignment_t alignment) {
    z_buf_layout_alloc_result_t alloc;

    z_owned_shm_mut_t shm_buf;
    z_alloc_error_t shm_error;

    z_shm_provider_alloc_gc(&alloc, provider, size, alignment);
    if (alloc.status == ZC_BUF_LAYOUT_ALLOC_STATUS_OK) {
        ASSERT_CHECK(alloc.buf);
        ASSERT_OK(test_shm_buffer(z_move(alloc.buf)));
        ASSERT_CHECK_ERR(alloc.buf);
        return Z_OK;
    } else
        return Z_ENULL;
}

int test_provider(z_owned_shm_provider_t* provider, z_alloc_alignment_t alignment, size_t buf_ok_size,
                  size_t buf_err_size) {
    // test allocation OK
    for (int i = 0; i < 100; ++i) {
        ASSERT_OK(test_allocation(z_loan(*provider), buf_ok_size, alignment));
    }

    // test allocation ERROR
    if (buf_err_size) {
        ASSERT_ERR(test_allocation(z_loan(*provider), buf_err_size, alignment));
    }

    // OK layouted allocations
    {
        // make OK allocation layout
        z_owned_alloc_layout_t alloc_layout;
        ASSERT_OK(z_alloc_layout_new(&alloc_layout, z_loan(*provider), buf_ok_size, alignment));
        ASSERT_CHECK(alloc_layout);
        // test layouted allocation OK
        for (int i = 0; i < 100; ++i) {
            ASSERT_OK(test_layouted_allocation(z_loan(alloc_layout)));
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
        ASSERT_ERR(test_layouted_allocation(z_loan(alloc_layout)));
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
    z_protocol_id_t id;
} test_provider_context;

void delete_fn(void* context) {
    test_provider_context* c = (test_provider_context*)context;
    free(c->bytes);
    free(c->busy_flags);
    c->bytes = NULL;
    c->busy_flags = NULL;
}
void deref_segemnt_fn(void* context) {}
void alloc_fn(struct z_owned_chunk_alloc_result_t* result, const struct z_loaned_memory_layout_t* layout,
              void* context) {
    assert(context);
    assert(layout);
    assert(result);

    // check size and alignment
    size_t size = 0;
    z_alloc_alignment_t alignment;
    z_memory_layout_get_data(layout, &size, &alignment);
    assert(size == 1);
    assert(alignment.pow == 0);

    // perform allocation
    test_provider_context* c = (test_provider_context*)context;
    for (int i = 0; i < c->count; ++i) {
        if (!c->busy_flags[i]) {
            c->busy_flags[i] = true;
            c->available--;

            z_owned_ptr_in_segment_t ptr;
            zc_threadsafe_context_t segment = {{NULL}, &deref_segemnt_fn};
            z_ptr_in_segment_new(&ptr, &c->bytes[i], segment);

            z_allocated_chunk_t chunk;
            chunk.ptr = z_move(ptr);
            uint64_t data_ptr = (uint64_t)(&c->bytes[i]);
            chunk.descriptpr.chunk = data_ptr & 0xFFFFFFFF;
            chunk.descriptpr.len = 1;
            chunk.descriptpr.segment = (data_ptr >> 32) & 0xFFFFFFFF;

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
    return 0;
}
size_t available_fn(void* context) {
    assert(context);

    test_provider_context* c = (test_provider_context*)context;
    return c->available;
}
void layout_for_fn(struct z_owned_memory_layout_t* layout, void* context) {
    assert(context);
    assert(layout);

    assert(z_internal_check(*layout));

    // check size and alignment
    size_t size = 0;
    z_alloc_alignment_t alignment;
    z_memory_layout_get_data(z_loan(*layout), &size, &alignment);

    if (size != 1 || alignment.pow != 0) {
        z_memory_layout_drop(z_move(*layout));
    }
}
z_protocol_id_t id_fn(void* context) {
    assert(context);

    test_provider_context* c = (test_provider_context*)context;
    return c->id;
}

int run_c_provider() {
    const size_t size = 1024;

    // init test context
    test_provider_context test_context;
    test_context.available = size;
    test_context.count = size;
    test_context.busy_flags = (bool*)malloc(sizeof(bool) * size);
    test_context.bytes = (uint8_t*)malloc(sizeof(uint8_t) * size);
    test_context.id = 100500;
    zc_context_t context = {&test_context, &delete_fn};

    // init callbacks
    zc_shm_provider_backend_callbacks_t callbacks = {&alloc_fn,     &free_fn,       &defragment_fn,
                                                     &available_fn, &layout_for_fn, &id_fn};
    // create provider
    z_owned_shm_provider_t provider;
    z_shm_provider_new(&provider, context, callbacks);
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
    z_shm_client_storage_new_default(&storage);

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
    z_ref_shm_client_storage_global(&storage);

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
    zc_shm_client_list_new(&list);
    ASSERT_CHECK(list);

    // create POSIX SHM Client
    z_owned_shm_client_t client;
    z_posix_shm_client_new(&client);
    ASSERT_CHECK(client);

    // add client to the list
    ASSERT_OK(zc_shm_client_list_add_client(z_loan_mut(list), z_move(client)));
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

z_protocol_id_t client_id_fn(void* context) { return 100500; }

int run_c_client() {
    // create client list
    zc_owned_shm_client_list_t list;
    zc_shm_client_list_new(&list);
    ASSERT_CHECK(list);

    // create C SHM Client
    zc_threadsafe_context_t context = {NULL, &delete_client_fn};
    zc_shm_client_callbacks_t callbacks = {&attach_fn, &client_id_fn};
    z_owned_shm_client_t client;
    z_shm_client_new(&client, context, callbacks);
    ASSERT_CHECK(client);

    // add client to the list
    ASSERT_OK(zc_shm_client_list_add_client(z_loan_mut(list), z_move(client)));
    ASSERT_CHECK_ERR(client);

    // create client storage from the list
    z_owned_shm_client_storage_t storage;
    ASSERT_OK(z_shm_client_storage_new(&storage, z_loan(list), true));
    ASSERT_CHECK(storage);

    // test client storage
    ASSERT_OK(test_client_storage(&storage));

    // drop the client storage
    z_drop(z_move(storage));
    ASSERT_CHECK_ERR(storage);

    // drop the client list
    z_drop(z_move(list));
    ASSERT_CHECK_ERR(list);
    return Z_OK;
}

int run_cleanup() {
    zc_cleanup_orphaned_shm_segments();
    return Z_OK;
}

int main() {
    ASSERT_OK(run_posix_provider());
    ASSERT_OK(run_c_provider());
    ASSERT_OK(run_default_client_storage());
    ASSERT_OK(run_global_client_storage());
    ASSERT_OK(run_client_storage());
    ASSERT_OK(run_c_client());
    ASSERT_OK(run_cleanup());
    return Z_OK;
}
