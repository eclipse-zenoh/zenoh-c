#pragma once

// clang-format off
#ifndef __cplusplus


#define z_loan(x) \
    _Generic((x), \
        z_owned_alloc_layout_t : z_alloc_layout_loan, \
        z_owned_bytes_t : z_bytes_loan, \
        z_owned_bytes_writer_t : z_bytes_writer_loan, \
        z_owned_chunk_alloc_result_t : z_chunk_alloc_result_loan, \
        z_owned_closure_hello_t : z_closure_hello_loan, \
        z_owned_closure_query_t : z_closure_query_loan, \
        z_owned_closure_reply_t : z_closure_reply_loan, \
        z_owned_closure_sample_t : z_closure_sample_loan, \
        z_owned_closure_zid_t : z_closure_zid_loan, \
        z_owned_condvar_t : z_condvar_loan, \
        z_owned_config_t : z_config_loan, \
        z_owned_encoding_t : z_encoding_loan, \
        z_owned_encoding_t : z_encoding_loan, \
        z_owned_fifo_handler_query_t : z_fifo_handler_query_loan, \
        z_owned_fifo_handler_reply_t : z_fifo_handler_reply_loan, \
        z_owned_fifo_handler_sample_t : z_fifo_handler_sample_loan, \
        z_owned_hello_t : z_hello_loan, \
        z_owned_keyexpr_t : z_keyexpr_loan, \
        z_owned_memory_layout_t : z_memory_layout_loan, \
        z_owned_publisher_t : z_publisher_loan, \
        z_owned_query_t : z_query_loan, \
        z_owned_queryable_t : z_queryable_loan, \
        z_owned_reply_err_t : z_reply_err_loan, \
        z_owned_reply_t : z_reply_loan, \
        z_owned_ring_handler_query_t : z_ring_handler_query_loan, \
        z_owned_ring_handler_reply_t : z_ring_handler_reply_loan, \
        z_owned_ring_handler_sample_t : z_ring_handler_sample_loan, \
        z_owned_sample_t : z_sample_loan, \
        z_owned_session_t : z_session_loan, \
        z_owned_shm_client_storage_t : z_shm_client_storage_loan, \
        z_owned_shm_t : z_shm_loan, \
        z_owned_shm_mut_t : z_shm_mut_loan, \
        z_owned_shm_provider_t : z_shm_provider_loan, \
        z_owned_slice_t : z_slice_loan, \
        z_owned_source_info_t : z_source_info_loan, \
        z_owned_string_array_t : z_string_array_loan, \
        z_owned_string_t : z_string_loan, \
        z_owned_subscriber_t : z_subscriber_loan, \
        z_view_keyexpr_t : z_view_keyexpr_loan, \
        z_view_slice_t : z_view_slice_loan, \
        z_view_string_t : z_view_string_loan, \
        zc_owned_closure_matching_status_t : zc_closure_matching_status_loan, \
        zc_owned_liveliness_token_t : zc_liveliness_token_loan, \
        zc_owned_shm_client_list_t : zc_shm_client_list_loan, \
        ze_owned_querying_subscriber_t : ze_querying_subscriber_loan \
    )(&x)

#define z_loan_mut(x) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_loan_mut, \
        z_owned_bytes_writer_t : z_bytes_writer_loan_mut, \
        z_owned_condvar_t : z_condvar_loan_mut, \
        z_owned_config_t : z_config_loan_mut, \
        z_owned_encoding_t : z_encoding_loan_mut, \
        z_owned_mutex_t : z_mutex_loan_mut, \
        z_owned_publisher_t : z_publisher_loan_mut, \
        z_owned_shm_t : z_shm_loan_mut, \
        z_owned_shm_mut_t : z_shm_mut_loan_mut, \
        z_owned_string_array_t : z_string_array_loan_mut, \
        zc_owned_shm_client_list_t : zc_shm_client_list_loan_mut \
    )(&x)

#define z_drop(x) \
    _Generic((x), \
        z_moved_alloc_layout_t : z_alloc_layout_drop, \
        z_moved_bytes_t : z_bytes_drop, \
        z_moved_bytes_writer_t : z_bytes_writer_drop, \
        z_moved_chunk_alloc_result_t : z_chunk_alloc_result_drop, \
        z_moved_closure_hello_t : z_closure_hello_drop, \
        z_moved_closure_query_t : z_closure_query_drop, \
        z_moved_closure_reply_t : z_closure_reply_drop, \
        z_moved_closure_sample_t : z_closure_sample_drop, \
        z_moved_closure_zid_t : z_closure_zid_drop, \
        z_moved_condvar_t : z_condvar_drop, \
        z_moved_config_t : z_config_drop, \
        z_moved_fifo_handler_query_t : z_fifo_handler_query_drop, \
        z_moved_fifo_handler_reply_t : z_fifo_handler_reply_drop, \
        z_moved_fifo_handler_sample_t : z_fifo_handler_sample_drop, \
        z_moved_hello_t : z_hello_drop, \
        z_moved_keyexpr_t : z_keyexpr_drop, \
        z_moved_memory_layout_t : z_memory_layout_drop, \
        z_moved_mutex_t : z_mutex_drop, \
        z_moved_publisher_t : z_publisher_drop, \
        z_moved_query_t : z_query_drop, \
        z_moved_queryable_t : z_queryable_drop, \
        z_moved_reply_t : z_reply_drop, \
        z_moved_reply_err_t : z_reply_err_drop, \
        z_moved_ring_handler_query_t : z_ring_handler_query_drop, \
        z_moved_ring_handler_reply_t : z_ring_handler_reply_drop, \
        z_moved_ring_handler_sample_t : z_ring_handler_sample_drop, \
        z_moved_sample_t : z_sample_drop, \
        z_moved_session_t : z_session_drop, \
        z_moved_shm_client_t : z_shm_client_drop, \
        z_moved_shm_client_storage_t : z_shm_client_storage_drop, \
        z_moved_shm_t : z_shm_drop, \
        z_moved_shm_mut_t : z_shm_mut_drop, \
        z_moved_shm_provider_t : z_shm_provider_drop, \
        z_moved_slice_t : z_slice_drop, \
        z_moved_source_info_t : z_source_info_drop, \
        z_moved_string_array_t : z_string_array_drop, \
        z_moved_string_t : z_string_drop, \
        z_moved_subscriber_t : z_subscriber_drop, \
        zc_moved_closure_matching_status_t : zc_closure_matching_status_drop, \
        zc_moved_liveliness_token_t : zc_liveliness_token_drop, \
        zc_moved_shm_client_list_t : zc_shm_client_list_drop, \
        zc_moved_matching_listener_t : zcu_publisher_matching_listener_drop, \
        ze_moved_publication_cache_t : ze_publication_cache_drop, \
        ze_moved_querying_subscriber_t : ze_querying_subscriber_drop \
    )(x)

#define z_alloc_layout_move(x) (z_moved_alloc_layout_t){&x}
#define z_bytes_move(x) (z_moved_bytes_t){&x}
#define z_bytes_writer_move(x) (z_moved_bytes_writer_t){&x}
#define z_chunk_alloc_result_move(x) (z_moved_chunk_alloc_result_t){&x}
#define z_closure_hello_move(x) (z_moved_closure_hello_t){&x}
#define z_closure_query_move(x) (z_moved_closure_query_t){&x}
#define z_closure_reply_move(x) (z_moved_closure_reply_t){&x}
#define z_closure_sample_move(x) (z_moved_closure_sample_t){&x}
#define z_closure_zid_move(x) (z_moved_closure_zid_t){&x}
#define z_condvar_move(x) (z_moved_condvar_t){&x}
#define z_config_move(x) (z_moved_config_t){&x}
#define z_fifo_handler_query_move(x) (z_moved_fifo_handler_query_t){&x}
#define z_fifo_handler_reply_move(x) (z_moved_fifo_handler_reply_t){&x}
#define z_fifo_handler_sample_move(x) (z_moved_fifo_handler_sample_t){&x}
#define z_hello_move(x) (z_moved_hello_t){&x}
#define z_keyexpr_move(x) (z_moved_keyexpr_t){&x}
#define z_memory_layout_move(x) (z_moved_memory_layout_t){&x}
#define z_mutex_move(x) (z_moved_mutex_t){&x}
#define z_publisher_move(x) (z_moved_publisher_t){&x}
#define z_query_move(x) (z_moved_query_t){&x}
#define z_queryable_move(x) (z_moved_queryable_t){&x}
#define z_reply_move(x) (z_moved_reply_t){&x}
#define z_reply_err_move(x) (z_moved_reply_err_t){&x}
#define z_ring_handler_query_move(x) (z_moved_ring_handler_query_t){&x}
#define z_ring_handler_reply_move(x) (z_moved_ring_handler_reply_t){&x}
#define z_ring_handler_sample_move(x) (z_moved_ring_handler_sample_t){&x}
#define z_sample_move(x) (z_moved_sample_t){&x}
#define z_session_move(x) (z_moved_session_t){&x}
#define z_shm_client_move(x) (z_moved_shm_client_t){&x}
#define z_shm_client_storage_move(x) (z_moved_shm_client_storage_t){&x}
#define z_shm_move(x) (z_moved_shm_t){&x}
#define z_shm_mut_move(x) (z_moved_shm_mut_t){&x}
#define z_shm_provider_move(x) (z_moved_shm_provider_t){&x}
#define z_slice_move(x) (z_moved_slice_t){&x}
#define z_source_info_move(x) (z_moved_source_info_t){&x}
#define z_string_array_move(x) (z_moved_string_array_t){&x}
#define z_string_move(x) (z_moved_string_t){&x}
#define z_subscriber_move(x) (z_moved_subscriber_t){&x}
#define zc_closure_matching_status_move(x) (zc_moved_closure_matching_status_t){&x}
#define zc_liveliness_token_move(x) (zc_moved_liveliness_token_t){&x}
#define zc_shm_client_list_move(x) (zc_moved_shm_client_list_t){&x}
#define zcu_publisher_matching_listener_move(x) (zc_moved_matching_listener_t){&x}
#define ze_publication_cache_move(x) (ze_moved_publication_cache_t){&x}
#define ze_querying_subscriber_move(x) (ze_moved_querying_subscriber_t){&x}
#define z_move(x) \
    _Generic((x), \
        z_owned_alloc_layout_t : (z_moved_alloc_layout_t){(z_owned_alloc_layout_t*)&x}, \
        z_owned_bytes_t : (z_moved_bytes_t){(z_owned_bytes_t*)&x}, \
        z_owned_bytes_writer_t : (z_moved_bytes_writer_t){(z_owned_bytes_writer_t*)&x}, \
        z_owned_chunk_alloc_result_t : (z_moved_chunk_alloc_result_t){(z_owned_chunk_alloc_result_t*)&x}, \
        z_owned_closure_hello_t : (z_moved_closure_hello_t){(z_owned_closure_hello_t*)&x}, \
        z_owned_closure_query_t : (z_moved_closure_query_t){(z_owned_closure_query_t*)&x}, \
        z_owned_closure_reply_t : (z_moved_closure_reply_t){(z_owned_closure_reply_t*)&x}, \
        z_owned_closure_sample_t : (z_moved_closure_sample_t){(z_owned_closure_sample_t*)&x}, \
        z_owned_closure_zid_t : (z_moved_closure_zid_t){(z_owned_closure_zid_t*)&x}, \
        z_owned_condvar_t : (z_moved_condvar_t){(z_owned_condvar_t*)&x}, \
        z_owned_config_t : (z_moved_config_t){(z_owned_config_t*)&x}, \
        z_owned_fifo_handler_query_t : (z_moved_fifo_handler_query_t){(z_owned_fifo_handler_query_t*)&x}, \
        z_owned_fifo_handler_reply_t : (z_moved_fifo_handler_reply_t){(z_owned_fifo_handler_reply_t*)&x}, \
        z_owned_fifo_handler_sample_t : (z_moved_fifo_handler_sample_t){(z_owned_fifo_handler_sample_t*)&x}, \
        z_owned_hello_t : (z_moved_hello_t){(z_owned_hello_t*)&x}, \
        z_owned_keyexpr_t : (z_moved_keyexpr_t){(z_owned_keyexpr_t*)&x}, \
        z_owned_memory_layout_t : (z_moved_memory_layout_t){(z_owned_memory_layout_t*)&x}, \
        z_owned_mutex_t : (z_moved_mutex_t){(z_owned_mutex_t*)&x}, \
        z_owned_publisher_t : (z_moved_publisher_t){(z_owned_publisher_t*)&x}, \
        z_owned_query_t : (z_moved_query_t){(z_owned_query_t*)&x}, \
        z_owned_queryable_t : (z_moved_queryable_t){(z_owned_queryable_t*)&x}, \
        z_owned_reply_t : (z_moved_reply_t){(z_owned_reply_t*)&x}, \
        z_owned_reply_err_t : (z_moved_reply_err_t){(z_owned_reply_err_t*)&x}, \
        z_owned_ring_handler_query_t : (z_moved_ring_handler_query_t){(z_owned_ring_handler_query_t*)&x}, \
        z_owned_ring_handler_reply_t : (z_moved_ring_handler_reply_t){(z_owned_ring_handler_reply_t*)&x}, \
        z_owned_ring_handler_sample_t : (z_moved_ring_handler_sample_t){(z_owned_ring_handler_sample_t*)&x}, \
        z_owned_sample_t : (z_moved_sample_t){(z_owned_sample_t*)&x}, \
        z_owned_session_t : (z_moved_session_t){(z_owned_session_t*)&x}, \
        z_owned_shm_client_t : (z_moved_shm_client_t){(z_owned_shm_client_t*)&x}, \
        z_owned_shm_client_storage_t : (z_moved_shm_client_storage_t){(z_owned_shm_client_storage_t*)&x}, \
        z_owned_shm_t : (z_moved_shm_t){(z_owned_shm_t*)&x}, \
        z_owned_shm_mut_t : (z_moved_shm_mut_t){(z_owned_shm_mut_t*)&x}, \
        z_owned_shm_provider_t : (z_moved_shm_provider_t){(z_owned_shm_provider_t*)&x}, \
        z_owned_slice_t : (z_moved_slice_t){(z_owned_slice_t*)&x}, \
        z_owned_source_info_t : (z_moved_source_info_t){(z_owned_source_info_t*)&x}, \
        z_owned_string_array_t : (z_moved_string_array_t){(z_owned_string_array_t*)&x}, \
        z_owned_string_t : (z_moved_string_t){(z_owned_string_t*)&x}, \
        z_owned_subscriber_t : (z_moved_subscriber_t){(z_owned_subscriber_t*)&x}, \
        zc_owned_closure_matching_status_t : (zc_moved_closure_matching_status_t){(zc_owned_closure_matching_status_t*)&x}, \
        zc_owned_liveliness_token_t : (zc_moved_liveliness_token_t){(zc_owned_liveliness_token_t*)&x}, \
        zc_owned_shm_client_list_t : (zc_moved_shm_client_list_t){(zc_owned_shm_client_list_t*)&x}, \
        zc_owned_matching_listener_t : (zc_moved_matching_listener_t){(zc_owned_matching_listener_t*)&x}, \
        ze_owned_publication_cache_t : (ze_moved_publication_cache_t){(ze_owned_publication_cache_t*)&x}, \
        ze_owned_querying_subscriber_t : (ze_moved_querying_subscriber_t){(ze_owned_querying_subscriber_t*)&x} \
    )

#define z_null(x) \
    _Generic((x), \
        z_owned_alloc_layout_t* : z_alloc_layout_null, \
        z_owned_bytes_t* : z_bytes_null, \
        z_owned_bytes_writer_t* : z_bytes_writer_null, \
        z_owned_chunk_alloc_result_t* : z_chunk_alloc_result_null, \
        z_owned_closure_hello_t* : z_closure_hello_null, \
        z_owned_closure_query_t* : z_closure_query_null, \
        z_owned_closure_reply_t* : z_closure_reply_null, \
        z_owned_closure_sample_t* : z_closure_sample_null, \
        z_owned_closure_zid_t* : z_closure_zid_null, \
        z_owned_condvar_t* : z_condvar_null, \
        z_owned_config_t* : z_config_null, \
        z_owned_encoding_t* : z_encoding_null, \
        z_owned_encoding_t* : z_encoding_null, \
        z_owned_fifo_handler_query_t* : z_fifo_handler_query_null, \
        z_owned_fifo_handler_reply_t* : z_fifo_handler_reply_null, \
        z_owned_fifo_handler_sample_t* : z_fifo_handler_sample_null, \
        z_owned_hello_t* : z_hello_null, \
        z_owned_keyexpr_t* : z_keyexpr_null, \
        z_owned_memory_layout_t* : z_memory_layout_null, \
        z_owned_mutex_t* : z_mutex_null, \
        z_owned_publisher_t* : z_publisher_null, \
        z_owned_query_t* : z_query_null, \
        z_owned_queryable_t* : z_queryable_null, \
        z_owned_reply_err_t* : z_reply_err_null, \
        z_owned_reply_t* : z_reply_null, \
        z_owned_ring_handler_query_t* : z_ring_handler_query_null, \
        z_owned_ring_handler_reply_t* : z_ring_handler_reply_null, \
        z_owned_ring_handler_sample_t* : z_ring_handler_sample_null, \
        z_owned_sample_t* : z_sample_null, \
        z_owned_session_t* : z_session_null, \
        z_owned_shm_client_t* : z_shm_client_null, \
        z_owned_shm_client_storage_t* : z_shm_client_storage_null, \
        z_owned_shm_mut_t* : z_shm_mut_null, \
        z_owned_shm_t* : z_shm_null, \
        z_owned_shm_provider_t* : z_shm_provider_null, \
        z_owned_slice_t* : z_slice_null, \
        z_owned_source_info_t* : z_source_info_null, \
        z_owned_string_array_t* : z_string_array_null, \
        z_owned_string_t* : z_string_null, \
        z_owned_subscriber_t* : z_subscriber_null, \
        z_owned_task_t* : z_task_null, \
        z_view_keyexpr_t* : z_view_keyexpr_null, \
        z_view_slice_t* : z_view_slice_null, \
        z_view_string_t* : z_view_string_null, \
        zc_owned_closure_matching_status_t* : zc_closure_matching_status_null, \
        zc_owned_liveliness_token_t* : zc_liveliness_token_null, \
        zc_owned_shm_client_list_t* : zc_shm_client_list_null, \
        ze_owned_publication_cache_t* : ze_publication_cache_null, \
        ze_owned_querying_subscriber_t* : ze_querying_subscriber_null \
    )(x)

#define z_check(x) \
    _Generic((x), \
        z_owned_alloc_layout_t : z_alloc_layout_check, \
        z_owned_bytes_t : z_bytes_check, \
        z_owned_bytes_writer_t : z_bytes_writer_check, \
        z_owned_chunk_alloc_result_t : z_chunk_alloc_result_check, \
        z_owned_closure_hello_t : z_closure_hello_check, \
        z_owned_closure_query_t : z_closure_query_check, \
        z_owned_closure_reply_t : z_closure_reply_check, \
        z_owned_closure_sample_t : z_closure_sample_check, \
        z_owned_closure_zid_t : z_closure_zid_check, \
        z_owned_condvar_t : z_condvar_check, \
        z_owned_config_t : z_config_check, \
        z_owned_encoding_t : z_encoding_check, \
        z_owned_encoding_t : z_encoding_check, \
        z_owned_fifo_handler_query_t : z_fifo_handler_query_check, \
        z_owned_fifo_handler_reply_t : z_fifo_handler_reply_check, \
        z_owned_fifo_handler_sample_t : z_fifo_handler_sample_check, \
        z_owned_hello_t : z_hello_check, \
        z_owned_keyexpr_t : z_keyexpr_check, \
        z_owned_memory_layout_t : z_memory_layout_check, \
        z_owned_mutex_t : z_mutex_check, \
        z_owned_publisher_t : z_publisher_check, \
        z_owned_query_t : z_query_check, \
        z_owned_queryable_t : z_queryable_check, \
        z_owned_reply_t : z_reply_check, \
        z_owned_reply_err_t : z_reply_err_check, \
        z_owned_ring_handler_query_t : z_ring_handler_query_check, \
        z_owned_ring_handler_reply_t : z_ring_handler_reply_check, \
        z_owned_ring_handler_sample_t : z_ring_handler_sample_check, \
        z_owned_sample_t : z_sample_check, \
        z_owned_session_t : z_session_check, \
        z_owned_shm_t : z_shm_check, \
        z_owned_shm_client_t : z_shm_client_check, \
        z_owned_shm_client_storage_t : z_shm_client_storage_check, \
        z_owned_shm_mut_t : z_shm_mut_check, \
        z_owned_shm_provider_t : z_shm_provider_check, \
        z_owned_slice_t : z_slice_check, \
        z_owned_source_info_t : z_source_info_check, \
        z_owned_string_array_t : z_string_array_check, \
        z_owned_string_t : z_string_check, \
        z_owned_subscriber_t : z_subscriber_check, \
        z_owned_task_t : z_task_check, \
        z_view_keyexpr_t : z_view_keyexpr_check, \
        z_view_slice_t : z_view_slice_check, \
        z_view_string_t : z_view_string_check, \
        zc_owned_closure_matching_status_t : zc_closure_matching_status_check, \
        zc_owned_liveliness_token_t : zc_liveliness_token_check, \
        zc_owned_shm_client_list_t : zc_shm_client_list_check, \
        ze_owned_publication_cache_t : ze_publication_cache_check, \
        ze_owned_querying_subscriber_t : ze_querying_subscriber_check \
    )(&x)

#define z_call(x, ...) \
    _Generic((x), \
        const z_loaned_closure_hello_t* : z_closure_hello_call, \
        const z_loaned_closure_query_t* : z_closure_query_call, \
        const z_loaned_closure_reply_t* : z_closure_reply_call, \
        const z_loaned_closure_sample_t* : z_closure_sample_call, \
        const z_loaned_closure_zid_t* : z_closure_zid_call, \
        const zc_loaned_closure_matching_status_t* : zc_closure_matching_status_call \
    )(x, __VA_ARGS__)

#define z_closure(x, callback, dropper, ctx) \
    {{(x)->context = (void*)(ctx); (x)->call = (callback); (x)->drop = (dropper);}}

#define z_try_recv(x, ...) \
    _Generic((x), \
        const z_loaned_fifo_handler_query_t* : z_fifo_handler_query_try_recv, \
        const z_loaned_fifo_handler_reply_t* : z_fifo_handler_reply_try_recv, \
        const z_loaned_fifo_handler_sample_t* : z_fifo_handler_sample_try_recv, \
        const z_loaned_ring_handler_query_t* : z_ring_handler_query_try_recv, \
        const z_loaned_ring_handler_reply_t* : z_ring_handler_reply_try_recv, \
        const z_loaned_ring_handler_sample_t* : z_ring_handler_sample_try_recv \
    )(x, __VA_ARGS__)

#define z_recv(x, ...) \
    _Generic((x), \
        const z_loaned_fifo_handler_query_t* : z_fifo_handler_query_recv, \
        const z_loaned_fifo_handler_reply_t* : z_fifo_handler_reply_recv, \
        const z_loaned_fifo_handler_sample_t* : z_fifo_handler_sample_recv, \
        const z_loaned_ring_handler_query_t* : z_ring_handler_query_recv, \
        const z_loaned_ring_handler_reply_t* : z_ring_handler_reply_recv, \
        const z_loaned_ring_handler_sample_t* : z_ring_handler_sample_recv \
    )(x, __VA_ARGS__)
#else  // #ifndef __cplusplus



inline const z_loaned_alloc_layout_t* z_loan(const z_owned_alloc_layout_t& this_) { return z_alloc_layout_loan(&this_); };
inline const z_loaned_bytes_t* z_loan(const z_owned_bytes_t& this_) { return z_bytes_loan(&this_); };
inline const z_loaned_bytes_writer_t* z_loan(const z_owned_bytes_writer_t& this_) { return z_bytes_writer_loan(&this_); };
inline const z_loaned_chunk_alloc_result_t* z_loan(const z_owned_chunk_alloc_result_t& this_) { return z_chunk_alloc_result_loan(&this_); };
inline const z_loaned_closure_hello_t* z_loan(const z_owned_closure_hello_t& closure) { return z_closure_hello_loan(&closure); };
inline const z_loaned_closure_query_t* z_loan(const z_owned_closure_query_t& closure) { return z_closure_query_loan(&closure); };
inline const z_loaned_closure_reply_t* z_loan(const z_owned_closure_reply_t& closure) { return z_closure_reply_loan(&closure); };
inline const z_loaned_closure_sample_t* z_loan(const z_owned_closure_sample_t& closure) { return z_closure_sample_loan(&closure); };
inline const z_loaned_closure_zid_t* z_loan(const z_owned_closure_zid_t& closure) { return z_closure_zid_loan(&closure); };
inline const z_loaned_condvar_t* z_loan(const z_owned_condvar_t& this_) { return z_condvar_loan(&this_); };
inline const z_loaned_config_t* z_loan(const z_owned_config_t& this_) { return z_config_loan(&this_); };
inline const z_loaned_encoding_t* z_loan(const z_owned_encoding_t& this_) { return z_encoding_loan(&this_); };
inline const z_loaned_encoding_t* z_loan(const z_owned_encoding_t& this_) { return z_encoding_loan(&this_); };
inline const z_loaned_fifo_handler_query_t* z_loan(const z_owned_fifo_handler_query_t& this_) { return z_fifo_handler_query_loan(&this_); };
inline const z_loaned_fifo_handler_reply_t* z_loan(const z_owned_fifo_handler_reply_t& this_) { return z_fifo_handler_reply_loan(&this_); };
inline const z_loaned_fifo_handler_sample_t* z_loan(const z_owned_fifo_handler_sample_t& this_) { return z_fifo_handler_sample_loan(&this_); };
inline const z_loaned_hello_t* z_loan(const z_owned_hello_t& this_) { return z_hello_loan(&this_); };
inline const z_loaned_keyexpr_t* z_loan(const z_owned_keyexpr_t& this_) { return z_keyexpr_loan(&this_); };
inline const z_loaned_memory_layout_t* z_loan(const z_owned_memory_layout_t& this_) { return z_memory_layout_loan(&this_); };
inline const z_loaned_publisher_t* z_loan(const z_owned_publisher_t& this_) { return z_publisher_loan(&this_); };
inline const z_loaned_query_t* z_loan(const z_owned_query_t& this_) { return z_query_loan(&this_); };
inline const z_loaned_queryable_t* z_loan(const z_owned_queryable_t& this_) { return z_queryable_loan(&this_); };
inline const z_loaned_reply_err_t* z_loan(const z_owned_reply_err_t& this_) { return z_reply_err_loan(&this_); };
inline const z_loaned_reply_t* z_loan(const z_owned_reply_t& this_) { return z_reply_loan(&this_); };
inline const z_loaned_ring_handler_query_t* z_loan(const z_owned_ring_handler_query_t& this_) { return z_ring_handler_query_loan(&this_); };
inline const z_loaned_ring_handler_reply_t* z_loan(const z_owned_ring_handler_reply_t& this_) { return z_ring_handler_reply_loan(&this_); };
inline const z_loaned_ring_handler_sample_t* z_loan(const z_owned_ring_handler_sample_t& this_) { return z_ring_handler_sample_loan(&this_); };
inline const z_loaned_sample_t* z_loan(const z_owned_sample_t& this_) { return z_sample_loan(&this_); };
inline const z_loaned_session_t* z_loan(const z_owned_session_t& this_) { return z_session_loan(&this_); };
inline const z_loaned_shm_client_storage_t* z_loan(const z_owned_shm_client_storage_t& this_) { return z_shm_client_storage_loan(&this_); };
inline const z_loaned_shm_t* z_loan(const z_owned_shm_t& this_) { return z_shm_loan(&this_); };
inline const z_loaned_shm_mut_t* z_loan(const z_owned_shm_mut_t& this_) { return z_shm_mut_loan(&this_); };
inline const z_loaned_shm_provider_t* z_loan(const z_owned_shm_provider_t& this_) { return z_shm_provider_loan(&this_); };
inline const z_loaned_slice_t* z_loan(const z_owned_slice_t& this_) { return z_slice_loan(&this_); };
inline const z_loaned_source_info_t* z_loan(const z_owned_source_info_t& this_) { return z_source_info_loan(&this_); };
inline const z_loaned_string_array_t* z_loan(const z_owned_string_array_t& this_) { return z_string_array_loan(&this_); };
inline const z_loaned_string_t* z_loan(const z_owned_string_t& this_) { return z_string_loan(&this_); };
inline const z_loaned_subscriber_t* z_loan(const z_owned_subscriber_t& this_) { return z_subscriber_loan(&this_); };
inline const z_loaned_keyexpr_t* z_loan(const z_view_keyexpr_t& this_) { return z_view_keyexpr_loan(&this_); };
inline const z_loaned_slice_t* z_loan(const z_view_slice_t& this_) { return z_view_slice_loan(&this_); };
inline const z_loaned_string_t* z_loan(const z_view_string_t& this_) { return z_view_string_loan(&this_); };
inline const zc_loaned_closure_matching_status_t* z_loan(const zc_owned_closure_matching_status_t& closure) { return zc_closure_matching_status_loan(&closure); };
inline const zc_loaned_liveliness_token_t* z_loan(const zc_owned_liveliness_token_t& this_) { return zc_liveliness_token_loan(&this_); };
inline const zc_loaned_shm_client_list_t* z_loan(const zc_owned_shm_client_list_t& this_) { return zc_shm_client_list_loan(&this_); };
inline const ze_loaned_querying_subscriber_t* z_loan(const ze_owned_querying_subscriber_t& this_) { return ze_querying_subscriber_loan(&this_); };


inline z_loaned_bytes_t* z_loan_mut(z_owned_bytes_t& this_) { return z_bytes_loan_mut(&this_); };
inline z_loaned_bytes_writer_t* z_loan_mut(z_owned_bytes_writer_t& this_) { return z_bytes_writer_loan_mut(&this_); };
inline z_loaned_condvar_t* z_loan_mut(z_owned_condvar_t& this_) { return z_condvar_loan_mut(&this_); };
inline z_loaned_config_t* z_loan_mut(z_owned_config_t& this_) { return z_config_loan_mut(&this_); };
inline z_loaned_encoding_t* z_loan_mut(z_owned_encoding_t& this_) { return z_encoding_loan_mut(&this_); };
inline z_loaned_mutex_t* z_loan_mut(z_owned_mutex_t& this_) { return z_mutex_loan_mut(&this_); };
inline z_loaned_publisher_t* z_loan_mut(z_owned_publisher_t& this_) { return z_publisher_loan_mut(&this_); };
inline z_loaned_shm_t* z_loan_mut(z_owned_shm_t& this_) { return z_shm_loan_mut(&this_); };
inline z_loaned_shm_mut_t* z_loan_mut(z_owned_shm_mut_t& this_) { return z_shm_mut_loan_mut(&this_); };
inline z_loaned_string_array_t* z_loan_mut(z_owned_string_array_t& this_) { return z_string_array_loan_mut(&this_); };
inline zc_loaned_shm_client_list_t* z_loan_mut(zc_owned_shm_client_list_t& this_) { return zc_shm_client_list_loan_mut(&this_); };


inline void z_drop(z_moved_alloc_layout_t this_) { return z_alloc_layout_drop(this_); };
inline void z_drop(z_moved_bytes_t this_) { return z_bytes_drop(this_); };
inline void z_drop(z_moved_bytes_writer_t this_) { return z_bytes_writer_drop(this_); };
inline void z_drop(z_moved_chunk_alloc_result_t this_) { return z_chunk_alloc_result_drop(this_); };
inline void z_drop(z_moved_closure_hello_t _closure) { return z_closure_hello_drop(_closure); };
inline void z_drop(z_moved_closure_query_t closure) { return z_closure_query_drop(closure); };
inline void z_drop(z_moved_closure_reply_t closure) { return z_closure_reply_drop(closure); };
inline void z_drop(z_moved_closure_sample_t closure) { return z_closure_sample_drop(closure); };
inline void z_drop(z_moved_closure_zid_t closure) { return z_closure_zid_drop(closure); };
inline void z_drop(z_moved_condvar_t this_) { return z_condvar_drop(this_); };
inline void z_drop(z_moved_config_t this_) { return z_config_drop(this_); };
inline void z_drop(z_moved_fifo_handler_query_t this_) { return z_fifo_handler_query_drop(this_); };
inline void z_drop(z_moved_fifo_handler_reply_t this_) { return z_fifo_handler_reply_drop(this_); };
inline void z_drop(z_moved_fifo_handler_sample_t this_) { return z_fifo_handler_sample_drop(this_); };
inline void z_drop(z_moved_hello_t this_) { return z_hello_drop(this_); };
inline void z_drop(z_moved_keyexpr_t this_) { return z_keyexpr_drop(this_); };
inline void z_drop(z_moved_memory_layout_t this_) { return z_memory_layout_drop(this_); };
inline void z_drop(z_moved_mutex_t this_) { return z_mutex_drop(this_); };
inline void z_drop(z_moved_publisher_t this_) { return z_publisher_drop(this_); };
inline void z_drop(z_moved_query_t this_) { return z_query_drop(this_); };
inline void z_drop(z_moved_queryable_t this_) { return z_queryable_drop(this_); };
inline void z_drop(z_moved_reply_t this_) { return z_reply_drop(this_); };
inline void z_drop(z_moved_reply_err_t this_) { return z_reply_err_drop(this_); };
inline void z_drop(z_moved_ring_handler_query_t this_) { return z_ring_handler_query_drop(this_); };
inline void z_drop(z_moved_ring_handler_reply_t this_) { return z_ring_handler_reply_drop(this_); };
inline void z_drop(z_moved_ring_handler_sample_t this_) { return z_ring_handler_sample_drop(this_); };
inline void z_drop(z_moved_sample_t this_) { return z_sample_drop(this_); };
inline void z_drop(z_moved_session_t this_) { return z_session_drop(this_); };
inline void z_drop(z_moved_shm_client_t this_) { return z_shm_client_drop(this_); };
inline void z_drop(z_moved_shm_client_storage_t this_) { return z_shm_client_storage_drop(this_); };
inline void z_drop(z_moved_shm_t this_) { return z_shm_drop(this_); };
inline void z_drop(z_moved_shm_mut_t this_) { return z_shm_mut_drop(this_); };
inline void z_drop(z_moved_shm_provider_t this_) { return z_shm_provider_drop(this_); };
inline void z_drop(z_moved_slice_t this_) { return z_slice_drop(this_); };
inline void z_drop(z_moved_source_info_t this_) { return z_source_info_drop(this_); };
inline void z_drop(z_moved_string_array_t this_) { return z_string_array_drop(this_); };
inline void z_drop(z_moved_string_t this_) { return z_string_drop(this_); };
inline void z_drop(z_moved_subscriber_t this_) { return z_subscriber_drop(this_); };
inline void z_drop(zc_moved_closure_matching_status_t closure) { return zc_closure_matching_status_drop(closure); };
inline void z_drop(zc_moved_liveliness_token_t this_) { return zc_liveliness_token_drop(this_); };
inline void z_drop(zc_moved_shm_client_list_t this_) { return zc_shm_client_list_drop(this_); };
inline void z_drop(zc_moved_matching_listener_t this_) { return zcu_publisher_matching_listener_drop(this_); };
inline void z_drop(ze_moved_publication_cache_t this_) { return ze_publication_cache_drop(this_); };
inline void z_drop(ze_moved_querying_subscriber_t this_) { return ze_querying_subscriber_drop(this_); };


inline z_moved_alloc_layout_t z_move(z_moved_alloc_layout_t this_) { return (&this_); };
inline z_moved_bytes_t z_move(z_moved_bytes_t this_) { return (&this_); };
inline z_moved_bytes_writer_t z_move(z_moved_bytes_writer_t this_) { return (&this_); };
inline z_moved_chunk_alloc_result_t z_move(z_moved_chunk_alloc_result_t this_) { return (&this_); };
inline z_moved_closure_hello_t z_move(z_moved_closure_hello_t _closure) { return (&_closure); };
inline z_moved_closure_query_t z_move(z_moved_closure_query_t closure) { return (&closure); };
inline z_moved_closure_reply_t z_move(z_moved_closure_reply_t closure) { return (&closure); };
inline z_moved_closure_sample_t z_move(z_moved_closure_sample_t closure) { return (&closure); };
inline z_moved_closure_zid_t z_move(z_moved_closure_zid_t closure) { return (&closure); };
inline z_moved_condvar_t z_move(z_moved_condvar_t this_) { return (&this_); };
inline z_moved_config_t z_move(z_moved_config_t this_) { return (&this_); };
inline z_moved_fifo_handler_query_t z_move(z_moved_fifo_handler_query_t this_) { return (&this_); };
inline z_moved_fifo_handler_reply_t z_move(z_moved_fifo_handler_reply_t this_) { return (&this_); };
inline z_moved_fifo_handler_sample_t z_move(z_moved_fifo_handler_sample_t this_) { return (&this_); };
inline z_moved_hello_t z_move(z_moved_hello_t this_) { return (&this_); };
inline z_moved_keyexpr_t z_move(z_moved_keyexpr_t this_) { return (&this_); };
inline z_moved_memory_layout_t z_move(z_moved_memory_layout_t this_) { return (&this_); };
inline z_moved_mutex_t z_move(z_moved_mutex_t this_) { return (&this_); };
inline z_moved_publisher_t z_move(z_moved_publisher_t this_) { return (&this_); };
inline z_moved_query_t z_move(z_moved_query_t this_) { return (&this_); };
inline z_moved_queryable_t z_move(z_moved_queryable_t this_) { return (&this_); };
inline z_moved_reply_t z_move(z_moved_reply_t this_) { return (&this_); };
inline z_moved_reply_err_t z_move(z_moved_reply_err_t this_) { return (&this_); };
inline z_moved_ring_handler_query_t z_move(z_moved_ring_handler_query_t this_) { return (&this_); };
inline z_moved_ring_handler_reply_t z_move(z_moved_ring_handler_reply_t this_) { return (&this_); };
inline z_moved_ring_handler_sample_t z_move(z_moved_ring_handler_sample_t this_) { return (&this_); };
inline z_moved_sample_t z_move(z_moved_sample_t this_) { return (&this_); };
inline z_moved_session_t z_move(z_moved_session_t this_) { return (&this_); };
inline z_moved_shm_client_t z_move(z_moved_shm_client_t this_) { return (&this_); };
inline z_moved_shm_client_storage_t z_move(z_moved_shm_client_storage_t this_) { return (&this_); };
inline z_moved_shm_t z_move(z_moved_shm_t this_) { return (&this_); };
inline z_moved_shm_mut_t z_move(z_moved_shm_mut_t this_) { return (&this_); };
inline z_moved_shm_provider_t z_move(z_moved_shm_provider_t this_) { return (&this_); };
inline z_moved_slice_t z_move(z_moved_slice_t this_) { return (&this_); };
inline z_moved_source_info_t z_move(z_moved_source_info_t this_) { return (&this_); };
inline z_moved_string_array_t z_move(z_moved_string_array_t this_) { return (&this_); };
inline z_moved_string_t z_move(z_moved_string_t this_) { return (&this_); };
inline z_moved_subscriber_t z_move(z_moved_subscriber_t this_) { return (&this_); };
inline zc_moved_closure_matching_status_t z_move(zc_moved_closure_matching_status_t closure) { return (&closure); };
inline zc_moved_liveliness_token_t z_move(zc_moved_liveliness_token_t this_) { return (&this_); };
inline zc_moved_shm_client_list_t z_move(zc_moved_shm_client_list_t this_) { return (&this_); };
inline zc_moved_matching_listener_t z_move(zc_moved_matching_listener_t this_) { return (&this_); };
inline ze_moved_publication_cache_t z_move(ze_moved_publication_cache_t this_) { return (&this_); };
inline ze_moved_querying_subscriber_t z_move(ze_moved_querying_subscriber_t this_) { return (&this_); };


inline void z_null(z_owned_alloc_layout_t* this_) { return z_alloc_layout_null(this_); };
inline void z_null(z_owned_bytes_t* this_) { return z_bytes_null(this_); };
inline void z_null(z_owned_bytes_writer_t* this_) { return z_bytes_writer_null(this_); };
inline void z_null(z_owned_chunk_alloc_result_t* this_) { return z_chunk_alloc_result_null(this_); };
inline void z_null(z_owned_closure_hello_t* this_) { return z_closure_hello_null(this_); };
inline void z_null(z_owned_closure_query_t* this_) { return z_closure_query_null(this_); };
inline void z_null(z_owned_closure_reply_t* this_) { return z_closure_reply_null(this_); };
inline void z_null(z_owned_closure_sample_t* this_) { return z_closure_sample_null(this_); };
inline void z_null(z_owned_closure_zid_t* this_) { return z_closure_zid_null(this_); };
inline void z_null(z_owned_condvar_t* this_) { return z_condvar_null(this_); };
inline void z_null(z_owned_config_t* this_) { return z_config_null(this_); };
inline void z_null(z_owned_encoding_t* this_) { return z_encoding_null(this_); };
inline void z_null(z_owned_encoding_t* this_) { return z_encoding_null(this_); };
inline void z_null(z_owned_fifo_handler_query_t* this_) { return z_fifo_handler_query_null(this_); };
inline void z_null(z_owned_fifo_handler_reply_t* this_) { return z_fifo_handler_reply_null(this_); };
inline void z_null(z_owned_fifo_handler_sample_t* this_) { return z_fifo_handler_sample_null(this_); };
inline void z_null(z_owned_hello_t* this_) { return z_hello_null(this_); };
inline void z_null(z_owned_keyexpr_t* this_) { return z_keyexpr_null(this_); };
inline void z_null(z_owned_memory_layout_t* this_) { return z_memory_layout_null(this_); };
inline void z_null(z_owned_mutex_t* this_) { return z_mutex_null(this_); };
inline void z_null(z_owned_publisher_t* this_) { return z_publisher_null(this_); };
inline void z_null(z_owned_query_t* this_) { return z_query_null(this_); };
inline void z_null(z_owned_queryable_t* this_) { return z_queryable_null(this_); };
inline void z_null(z_owned_reply_err_t* this_) { return z_reply_err_null(this_); };
inline void z_null(z_owned_reply_t* this_) { return z_reply_null(this_); };
inline void z_null(z_owned_ring_handler_query_t* this_) { return z_ring_handler_query_null(this_); };
inline void z_null(z_owned_ring_handler_reply_t* this_) { return z_ring_handler_reply_null(this_); };
inline void z_null(z_owned_ring_handler_sample_t* this_) { return z_ring_handler_sample_null(this_); };
inline void z_null(z_owned_sample_t* this_) { return z_sample_null(this_); };
inline void z_null(z_owned_session_t* this_) { return z_session_null(this_); };
inline void z_null(z_owned_shm_client_t* this_) { return z_shm_client_null(this_); };
inline void z_null(z_owned_shm_client_storage_t* this_) { return z_shm_client_storage_null(this_); };
inline void z_null(z_owned_shm_mut_t* this_) { return z_shm_mut_null(this_); };
inline void z_null(z_owned_shm_t* this_) { return z_shm_null(this_); };
inline void z_null(z_owned_shm_provider_t* this_) { return z_shm_provider_null(this_); };
inline void z_null(z_owned_slice_t* this_) { return z_slice_null(this_); };
inline void z_null(z_owned_source_info_t* this_) { return z_source_info_null(this_); };
inline void z_null(z_owned_string_array_t* this_) { return z_string_array_null(this_); };
inline void z_null(z_owned_string_t* this_) { return z_string_null(this_); };
inline void z_null(z_owned_subscriber_t* this_) { return z_subscriber_null(this_); };
inline void z_null(z_owned_task_t* this_) { return z_task_null(this_); };
inline void z_null(z_view_keyexpr_t* this_) { return z_view_keyexpr_null(this_); };
inline void z_null(z_view_slice_t* this_) { return z_view_slice_null(this_); };
inline void z_null(z_view_string_t* this_) { return z_view_string_null(this_); };
inline void z_null(zc_owned_closure_matching_status_t* this_) { return zc_closure_matching_status_null(this_); };
inline void z_null(zc_owned_liveliness_token_t* this_) { return zc_liveliness_token_null(this_); };
inline void z_null(zc_owned_shm_client_list_t* this_) { return zc_shm_client_list_null(this_); };
inline void z_null(ze_owned_publication_cache_t* this_) { return ze_publication_cache_null(this_); };
inline void z_null(ze_owned_querying_subscriber_t* this_) { return ze_querying_subscriber_null(this_); };


inline bool z_check(const z_owned_alloc_layout_t& this_) { return z_alloc_layout_check(&this_); };
inline bool z_check(const z_owned_bytes_t& this_) { return z_bytes_check(&this_); };
inline bool z_check(const z_owned_bytes_writer_t& this_) { return z_bytes_writer_check(&this_); };
inline bool z_check(const z_owned_chunk_alloc_result_t& this_) { return z_chunk_alloc_result_check(&this_); };
inline bool z_check(const z_owned_closure_hello_t& this_) { return z_closure_hello_check(&this_); };
inline bool z_check(const z_owned_closure_query_t& this_) { return z_closure_query_check(&this_); };
inline bool z_check(const z_owned_closure_reply_t& this_) { return z_closure_reply_check(&this_); };
inline bool z_check(const z_owned_closure_sample_t& this_) { return z_closure_sample_check(&this_); };
inline bool z_check(const z_owned_closure_zid_t& this_) { return z_closure_zid_check(&this_); };
inline bool z_check(const z_owned_condvar_t& this_) { return z_condvar_check(&this_); };
inline bool z_check(const z_owned_config_t& this_) { return z_config_check(&this_); };
inline bool z_check(const z_owned_encoding_t& this_) { return z_encoding_check(&this_); };
inline bool z_check(const z_owned_encoding_t& this_) { return z_encoding_check(&this_); };
inline bool z_check(const z_owned_fifo_handler_query_t& this_) { return z_fifo_handler_query_check(&this_); };
inline bool z_check(const z_owned_fifo_handler_reply_t& this_) { return z_fifo_handler_reply_check(&this_); };
inline bool z_check(const z_owned_fifo_handler_sample_t& this_) { return z_fifo_handler_sample_check(&this_); };
inline bool z_check(const z_owned_hello_t& this_) { return z_hello_check(&this_); };
inline bool z_check(const z_owned_keyexpr_t& this_) { return z_keyexpr_check(&this_); };
inline bool z_check(const z_owned_memory_layout_t& this_) { return z_memory_layout_check(&this_); };
inline bool z_check(const z_owned_mutex_t& this_) { return z_mutex_check(&this_); };
inline bool z_check(const z_owned_publisher_t& this_) { return z_publisher_check(&this_); };
inline bool z_check(const z_owned_query_t& query) { return z_query_check(&query); };
inline bool z_check(const z_owned_queryable_t& this_) { return z_queryable_check(&this_); };
inline bool z_check(const z_owned_reply_t& this_) { return z_reply_check(&this_); };
inline bool z_check(const z_owned_reply_err_t& this_) { return z_reply_err_check(&this_); };
inline bool z_check(const z_owned_ring_handler_query_t& this_) { return z_ring_handler_query_check(&this_); };
inline bool z_check(const z_owned_ring_handler_reply_t& this_) { return z_ring_handler_reply_check(&this_); };
inline bool z_check(const z_owned_ring_handler_sample_t& this_) { return z_ring_handler_sample_check(&this_); };
inline bool z_check(const z_owned_sample_t& this_) { return z_sample_check(&this_); };
inline bool z_check(const z_owned_session_t& this_) { return z_session_check(&this_); };
inline bool z_check(const z_owned_shm_t& this_) { return z_shm_check(&this_); };
inline bool z_check(const z_owned_shm_client_t& this_) { return z_shm_client_check(&this_); };
inline bool z_check(const z_owned_shm_client_storage_t& this_) { return z_shm_client_storage_check(&this_); };
inline bool z_check(const z_owned_shm_mut_t& this_) { return z_shm_mut_check(&this_); };
inline bool z_check(const z_owned_shm_provider_t& this_) { return z_shm_provider_check(&this_); };
inline bool z_check(const z_owned_slice_t& this_) { return z_slice_check(&this_); };
inline bool z_check(const z_owned_source_info_t& this_) { return z_source_info_check(&this_); };
inline bool z_check(const z_owned_string_array_t& this_) { return z_string_array_check(&this_); };
inline bool z_check(const z_owned_string_t& this_) { return z_string_check(&this_); };
inline bool z_check(const z_owned_subscriber_t& this_) { return z_subscriber_check(&this_); };
inline bool z_check(const z_owned_task_t& this_) { return z_task_check(&this_); };
inline bool z_check(const z_view_keyexpr_t& this_) { return z_view_keyexpr_check(&this_); };
inline bool z_check(const z_view_slice_t& this_) { return z_view_slice_check(&this_); };
inline bool z_check(const z_view_string_t& this_) { return z_view_string_check(&this_); };
inline bool z_check(const zc_owned_closure_matching_status_t& this_) { return zc_closure_matching_status_check(&this_); };
inline bool z_check(const zc_owned_liveliness_token_t& this_) { return zc_liveliness_token_check(&this_); };
inline bool z_check(const zc_owned_shm_client_list_t& this_) { return zc_shm_client_list_check(&this_); };
inline bool z_check(const ze_owned_publication_cache_t& this_) { return ze_publication_cache_check(&this_); };
inline bool z_check(const ze_owned_querying_subscriber_t& this_) { return ze_querying_subscriber_check(&this_); };


inline void z_call(const z_loaned_closure_hello_t* closure, const z_loaned_hello_t* hello) {
    return z_closure_hello_call(closure, hello);
};
inline void z_call(const z_loaned_closure_query_t* closure, const z_loaned_query_t* query) {
    return z_closure_query_call(closure, query);
};
inline void z_call(const z_loaned_closure_reply_t* closure, const z_loaned_reply_t* reply) {
    return z_closure_reply_call(closure, reply);
};
inline void z_call(const z_loaned_closure_sample_t* closure, const z_loaned_sample_t* sample) {
    return z_closure_sample_call(closure, sample);
};
inline void z_call(const z_loaned_closure_zid_t* closure, const z_id_t* z_id) {
    return z_closure_zid_call(closure, z_id);
};
inline void z_call(const zc_loaned_closure_matching_status_t* closure, const zc_matching_status_t* mathing_status) {
    return zc_closure_matching_status_call(closure, mathing_status);
};


inline void z_closure(
    z_owned_closure_hello_t* closure,
    void (*call)(const z_loaned_hello_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_closure_query_t* closure,
    void (*call)(const z_loaned_query_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_closure_reply_t* closure,
    void (*call)(const z_loaned_reply_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_closure_sample_t* closure,
    void (*call)(const z_loaned_sample_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_closure_zid_t* closure,
    void (*call)(const z_id_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    zc_owned_closure_matching_status_t* closure,
    void (*call)(const zc_matching_status_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};


inline bool z_try_recv(const z_loaned_fifo_handler_query_t* this_, z_owned_query_t* query) {
    return z_fifo_handler_query_try_recv(this_, query);
};
inline bool z_try_recv(const z_loaned_fifo_handler_reply_t* this_, z_owned_reply_t* reply) {
    return z_fifo_handler_reply_try_recv(this_, reply);
};
inline bool z_try_recv(const z_loaned_fifo_handler_sample_t* this_, z_owned_sample_t* sample) {
    return z_fifo_handler_sample_try_recv(this_, sample);
};
inline bool z_try_recv(const z_loaned_ring_handler_query_t* this_, z_owned_query_t* query) {
    return z_ring_handler_query_try_recv(this_, query);
};
inline bool z_try_recv(const z_loaned_ring_handler_reply_t* this_, z_owned_reply_t* reply) {
    return z_ring_handler_reply_try_recv(this_, reply);
};
inline bool z_try_recv(const z_loaned_ring_handler_sample_t* this_, z_owned_sample_t* sample) {
    return z_ring_handler_sample_try_recv(this_, sample);
};


inline bool z_recv(const z_loaned_fifo_handler_query_t* this_, z_owned_query_t* query) {
    return z_fifo_handler_query_recv(this_, query);
};
inline bool z_recv(const z_loaned_fifo_handler_reply_t* this_, z_owned_reply_t* reply) {
    return z_fifo_handler_reply_recv(this_, reply);
};
inline bool z_recv(const z_loaned_fifo_handler_sample_t* this_, z_owned_sample_t* sample) {
    return z_fifo_handler_sample_recv(this_, sample);
};
inline bool z_recv(const z_loaned_ring_handler_query_t* this_, z_owned_query_t* query) {
    return z_ring_handler_query_recv(this_, query);
};
inline bool z_recv(const z_loaned_ring_handler_reply_t* this_, z_owned_reply_t* reply) {
    return z_ring_handler_reply_recv(this_, reply);
};
inline bool z_recv(const z_loaned_ring_handler_sample_t* this_, z_owned_sample_t* sample) {
    return z_ring_handler_sample_recv(this_, sample);
};

template<class T> struct z_loaned_to_owned_type_t {};
template<class T> struct z_owned_to_loaned_type_t {};
template<> struct z_loaned_to_owned_type_t<z_loaned_alloc_layout_t> { typedef z_owned_alloc_layout_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_alloc_layout_t> { typedef z_loaned_alloc_layout_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_bytes_t> { typedef z_owned_bytes_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_bytes_t> { typedef z_loaned_bytes_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_bytes_writer_t> { typedef z_owned_bytes_writer_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_bytes_writer_t> { typedef z_loaned_bytes_writer_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_chunk_alloc_result_t> { typedef z_owned_chunk_alloc_result_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_chunk_alloc_result_t> { typedef z_loaned_chunk_alloc_result_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_hello_t> { typedef z_owned_closure_hello_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_hello_t> { typedef z_loaned_closure_hello_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_query_t> { typedef z_owned_closure_query_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_query_t> { typedef z_loaned_closure_query_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_reply_t> { typedef z_owned_closure_reply_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_reply_t> { typedef z_loaned_closure_reply_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_sample_t> { typedef z_owned_closure_sample_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_sample_t> { typedef z_loaned_closure_sample_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_zid_t> { typedef z_owned_closure_zid_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_zid_t> { typedef z_loaned_closure_zid_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_condvar_t> { typedef z_owned_condvar_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_condvar_t> { typedef z_loaned_condvar_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_config_t> { typedef z_owned_config_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_config_t> { typedef z_loaned_config_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_encoding_t> { typedef z_owned_encoding_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_encoding_t> { typedef z_loaned_encoding_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_fifo_handler_query_t> { typedef z_owned_fifo_handler_query_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_fifo_handler_query_t> { typedef z_loaned_fifo_handler_query_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_fifo_handler_reply_t> { typedef z_owned_fifo_handler_reply_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_fifo_handler_reply_t> { typedef z_loaned_fifo_handler_reply_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_fifo_handler_sample_t> { typedef z_owned_fifo_handler_sample_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_fifo_handler_sample_t> { typedef z_loaned_fifo_handler_sample_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_hello_t> { typedef z_owned_hello_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_hello_t> { typedef z_loaned_hello_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_keyexpr_t> { typedef z_owned_keyexpr_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_keyexpr_t> { typedef z_loaned_keyexpr_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_memory_layout_t> { typedef z_owned_memory_layout_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_memory_layout_t> { typedef z_loaned_memory_layout_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_publisher_t> { typedef z_owned_publisher_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_publisher_t> { typedef z_loaned_publisher_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_query_t> { typedef z_owned_query_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_query_t> { typedef z_loaned_query_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_queryable_t> { typedef z_owned_queryable_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_queryable_t> { typedef z_loaned_queryable_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_reply_err_t> { typedef z_owned_reply_err_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_reply_err_t> { typedef z_loaned_reply_err_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_reply_t> { typedef z_owned_reply_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_reply_t> { typedef z_loaned_reply_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_ring_handler_query_t> { typedef z_owned_ring_handler_query_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_ring_handler_query_t> { typedef z_loaned_ring_handler_query_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_ring_handler_reply_t> { typedef z_owned_ring_handler_reply_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_ring_handler_reply_t> { typedef z_loaned_ring_handler_reply_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_ring_handler_sample_t> { typedef z_owned_ring_handler_sample_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_ring_handler_sample_t> { typedef z_loaned_ring_handler_sample_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_sample_t> { typedef z_owned_sample_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_sample_t> { typedef z_loaned_sample_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_session_t> { typedef z_owned_session_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_session_t> { typedef z_loaned_session_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_shm_client_storage_t> { typedef z_owned_shm_client_storage_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_shm_client_storage_t> { typedef z_loaned_shm_client_storage_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_shm_t> { typedef z_owned_shm_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_shm_t> { typedef z_loaned_shm_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_shm_mut_t> { typedef z_owned_shm_mut_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_shm_mut_t> { typedef z_loaned_shm_mut_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_shm_provider_t> { typedef z_owned_shm_provider_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_shm_provider_t> { typedef z_loaned_shm_provider_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_slice_t> { typedef z_owned_slice_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_slice_t> { typedef z_loaned_slice_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_source_info_t> { typedef z_owned_source_info_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_source_info_t> { typedef z_loaned_source_info_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_string_array_t> { typedef z_owned_string_array_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_string_array_t> { typedef z_loaned_string_array_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_string_t> { typedef z_owned_string_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_string_t> { typedef z_loaned_string_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_subscriber_t> { typedef z_owned_subscriber_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_subscriber_t> { typedef z_loaned_subscriber_t type; };
template<> struct z_loaned_to_owned_type_t<zc_loaned_closure_matching_status_t> { typedef zc_owned_closure_matching_status_t type; };
template<> struct z_owned_to_loaned_type_t<zc_owned_closure_matching_status_t> { typedef zc_loaned_closure_matching_status_t type; };
template<> struct z_loaned_to_owned_type_t<zc_loaned_liveliness_token_t> { typedef zc_owned_liveliness_token_t type; };
template<> struct z_owned_to_loaned_type_t<zc_owned_liveliness_token_t> { typedef zc_loaned_liveliness_token_t type; };
template<> struct z_loaned_to_owned_type_t<zc_loaned_shm_client_list_t> { typedef zc_owned_shm_client_list_t type; };
template<> struct z_owned_to_loaned_type_t<zc_owned_shm_client_list_t> { typedef zc_loaned_shm_client_list_t type; };
template<> struct z_loaned_to_owned_type_t<ze_loaned_querying_subscriber_t> { typedef ze_owned_querying_subscriber_t type; };
template<> struct z_owned_to_loaned_type_t<ze_owned_querying_subscriber_t> { typedef ze_loaned_querying_subscriber_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_mutex_t> { typedef z_owned_mutex_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_mutex_t> { typedef z_loaned_mutex_t type; };
#endif  // #ifndef __cplusplus