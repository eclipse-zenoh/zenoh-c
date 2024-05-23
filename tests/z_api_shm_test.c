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

#include "zenoh.h"

#define ASSERT_OK(result) \
    if (result != Z_OK) { \
        assert(false);    \
        return result;    \
    }

#define ASSERT_TRUE(expr) \
    if (!expr) {          \
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
    z_shm_copy(&immut2, z_loan(immut));
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

bool test_allocation(const z_loaned_shared_memory_provider_t* provider, size_t size, z_alloc_alignment_t alignment) {
    z_owned_buf_alloc_result_t alloc;

    ASSERT_OK(z_shared_memory_provider_alloc_gc(&alloc, provider, size, alignment));
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

int run_provider() {
    const size_t total_size = 4096;
    const size_t buf_ok_size = 1024;
    const size_t buf_err_size = 8192;

    z_alloc_alignment_t alignment = {4};

    z_owned_memory_layout_t layout;
    ASSERT_OK(z_memory_layout_new(&layout, total_size, alignment));
    ASSERT_CHECK(layout);

    z_owned_shared_memory_provider_t provider;
    ASSERT_OK(z_posix_shared_memory_provider_new(&provider, z_loan(layout)));
    ASSERT_CHECK(provider);

    // test allocation OK
    for (int i = 0; i < 100; ++i) {
        ASSERT_TRUE(test_allocation(z_loan(provider), buf_ok_size, alignment));
    }

    // test allocation ERROR
    ASSERT_TRUE(!test_allocation(z_loan(provider), buf_err_size, alignment));

    // OK layouted allocations
    {
        // make OK allocation layout
        z_owned_alloc_layout_t alloc_layout;
        ASSERT_OK(z_alloc_layout_new(&alloc_layout, z_loan(provider), buf_ok_size, alignment));
        ASSERT_CHECK(alloc_layout);
        // test layouted allocation OK
        for (int i = 0; i < 100; ++i) {
            ASSERT_TRUE(test_layouted_allocation(z_loan(alloc_layout)));
        }
        z_drop(z_move(alloc_layout));
        ASSERT_CHECK_ERR(alloc_layout);
    }

    // ERR layouted allocation
    {
        // make ERR allocation layout
        z_owned_alloc_layout_t alloc_layout;
        ASSERT_OK(z_alloc_layout_new(&alloc_layout, z_loan(provider), buf_err_size, alignment));
        ASSERT_CHECK(alloc_layout);
        // test layouted allocation ERROR
        ASSERT_TRUE(!test_layouted_allocation(z_loan(alloc_layout)));
        z_drop(z_move(alloc_layout));
        ASSERT_CHECK_ERR(alloc_layout);
    }

    // additional functions
    z_shared_memory_provider_defragment(z_loan(provider));
    z_shared_memory_provider_garbage_collect(z_loan(provider));

    z_drop(z_move(provider));
    ASSERT_CHECK_ERR(provider);

    z_drop(z_move(layout));
    ASSERT_CHECK_ERR(layout);

    return 0;
}

int run_default_client_storage() {
    z_owned_shared_memory_client_storage_t storage;
    ASSERT_OK(z_shared_memory_client_storage_new_default(&storage));
    ASSERT_CHECK(storage);
    z_drop(z_move(storage));
    ASSERT_CHECK_ERR(storage);

    return 0;
}

int run_global_client_storage() {
    z_owned_shared_memory_client_storage_t storage;
    ASSERT_OK(z_ref_shared_memory_client_storage_global(&storage));
    ASSERT_CHECK(storage);
    z_drop(z_move(storage));
    ASSERT_CHECK_ERR(storage);

    return 0;
}

int run_client_storage() {
    zc_owned_shared_memory_client_list_t list;
    ASSERT_OK(zc_shared_memory_client_list_new(&list));
    ASSERT_CHECK(list);

    z_owned_shared_memory_client_t client;
    ASSERT_OK(z_posix_shared_memory_client_new(&client));
    ASSERT_CHECK(client);

    ASSERT_OK(zc_shared_memory_client_list_add_client(Z_SHM_POSIX_PROTOCOL_ID, z_move(client), z_loan_mut(list)));
    ASSERT_CHECK_ERR(client);

    z_owned_shared_memory_client_storage_t storage;
    ASSERT_OK(z_shared_memory_client_storage_new(&storage, z_loan(list), true));
    ASSERT_CHECK(storage);

    z_drop(z_move(storage));
    ASSERT_CHECK_ERR(storage);

    z_drop(z_move(list));
    ASSERT_CHECK_ERR(list);
    return 0;
}

int main() {
    ASSERT_OK(run_provider());
    ASSERT_OK(run_default_client_storage());
    ASSERT_OK(run_global_client_storage());
    ASSERT_OK(run_client_storage());
    return 0;
}
