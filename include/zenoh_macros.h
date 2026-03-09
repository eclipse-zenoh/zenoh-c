#pragma once
// clang-format off


#ifndef __cplusplus

static inline z_moved_bytes_t* z_bytes_move(z_owned_bytes_t* x) { return (z_moved_bytes_t*)(x); }
static inline z_moved_bytes_writer_t* z_bytes_writer_move(z_owned_bytes_writer_t* x) { return (z_moved_bytes_writer_t*)(x); }
static inline z_moved_cancellation_token_t* z_cancellation_token_move(z_owned_cancellation_token_t* x) { return (z_moved_cancellation_token_t*)(x); }
static inline z_moved_chunk_alloc_result_t* z_chunk_alloc_result_move(z_owned_chunk_alloc_result_t* x) { return (z_moved_chunk_alloc_result_t*)(x); }
static inline z_moved_closure_hello_t* z_closure_hello_move(z_owned_closure_hello_t* x) { return (z_moved_closure_hello_t*)(x); }
static inline z_moved_closure_matching_status_t* z_closure_matching_status_move(z_owned_closure_matching_status_t* x) { return (z_moved_closure_matching_status_t*)(x); }
static inline z_moved_closure_query_t* z_closure_query_move(z_owned_closure_query_t* x) { return (z_moved_closure_query_t*)(x); }
static inline z_moved_closure_reply_t* z_closure_reply_move(z_owned_closure_reply_t* x) { return (z_moved_closure_reply_t*)(x); }
static inline z_moved_closure_sample_t* z_closure_sample_move(z_owned_closure_sample_t* x) { return (z_moved_closure_sample_t*)(x); }
static inline z_moved_closure_zid_t* z_closure_zid_move(z_owned_closure_zid_t* x) { return (z_moved_closure_zid_t*)(x); }
static inline z_moved_condvar_t* z_condvar_move(z_owned_condvar_t* x) { return (z_moved_condvar_t*)(x); }
static inline z_moved_config_t* z_config_move(z_owned_config_t* x) { return (z_moved_config_t*)(x); }
static inline z_moved_encoding_t* z_encoding_move(z_owned_encoding_t* x) { return (z_moved_encoding_t*)(x); }
static inline z_moved_fifo_handler_query_t* z_fifo_handler_query_move(z_owned_fifo_handler_query_t* x) { return (z_moved_fifo_handler_query_t*)(x); }
static inline z_moved_fifo_handler_reply_t* z_fifo_handler_reply_move(z_owned_fifo_handler_reply_t* x) { return (z_moved_fifo_handler_reply_t*)(x); }
static inline z_moved_fifo_handler_sample_t* z_fifo_handler_sample_move(z_owned_fifo_handler_sample_t* x) { return (z_moved_fifo_handler_sample_t*)(x); }
static inline z_moved_hello_t* z_hello_move(z_owned_hello_t* x) { return (z_moved_hello_t*)(x); }
static inline z_moved_keyexpr_t* z_keyexpr_move(z_owned_keyexpr_t* x) { return (z_moved_keyexpr_t*)(x); }
static inline z_moved_liveliness_token_t* z_liveliness_token_move(z_owned_liveliness_token_t* x) { return (z_moved_liveliness_token_t*)(x); }
static inline z_moved_matching_listener_t* z_matching_listener_move(z_owned_matching_listener_t* x) { return (z_moved_matching_listener_t*)(x); }
static inline z_moved_memory_layout_t* z_memory_layout_move(z_owned_memory_layout_t* x) { return (z_moved_memory_layout_t*)(x); }
static inline z_moved_mutex_t* z_mutex_move(z_owned_mutex_t* x) { return (z_moved_mutex_t*)(x); }
static inline z_moved_precomputed_layout_t* z_precomputed_layout_move(z_owned_precomputed_layout_t* x) { return (z_moved_precomputed_layout_t*)(x); }
static inline z_moved_ptr_in_segment_t* z_ptr_in_segment_move(z_owned_ptr_in_segment_t* x) { return (z_moved_ptr_in_segment_t*)(x); }
static inline z_moved_publisher_t* z_publisher_move(z_owned_publisher_t* x) { return (z_moved_publisher_t*)(x); }
static inline z_moved_querier_t* z_querier_move(z_owned_querier_t* x) { return (z_moved_querier_t*)(x); }
static inline z_moved_query_t* z_query_move(z_owned_query_t* x) { return (z_moved_query_t*)(x); }
static inline z_moved_queryable_t* z_queryable_move(z_owned_queryable_t* x) { return (z_moved_queryable_t*)(x); }
static inline z_moved_reply_t* z_reply_move(z_owned_reply_t* x) { return (z_moved_reply_t*)(x); }
static inline z_moved_reply_err_t* z_reply_err_move(z_owned_reply_err_t* x) { return (z_moved_reply_err_t*)(x); }
static inline z_moved_ring_handler_query_t* z_ring_handler_query_move(z_owned_ring_handler_query_t* x) { return (z_moved_ring_handler_query_t*)(x); }
static inline z_moved_ring_handler_reply_t* z_ring_handler_reply_move(z_owned_ring_handler_reply_t* x) { return (z_moved_ring_handler_reply_t*)(x); }
static inline z_moved_ring_handler_sample_t* z_ring_handler_sample_move(z_owned_ring_handler_sample_t* x) { return (z_moved_ring_handler_sample_t*)(x); }
static inline z_moved_sample_t* z_sample_move(z_owned_sample_t* x) { return (z_moved_sample_t*)(x); }
static inline z_moved_session_t* z_session_move(z_owned_session_t* x) { return (z_moved_session_t*)(x); }
static inline z_moved_shared_shm_provider_t* z_shared_shm_provider_move(z_owned_shared_shm_provider_t* x) { return (z_moved_shared_shm_provider_t*)(x); }
static inline z_moved_shm_client_t* z_shm_client_move(z_owned_shm_client_t* x) { return (z_moved_shm_client_t*)(x); }
static inline z_moved_shm_client_storage_t* z_shm_client_storage_move(z_owned_shm_client_storage_t* x) { return (z_moved_shm_client_storage_t*)(x); }
static inline z_moved_shm_t* z_shm_move(z_owned_shm_t* x) { return (z_moved_shm_t*)(x); }
static inline z_moved_shm_mut_t* z_shm_mut_move(z_owned_shm_mut_t* x) { return (z_moved_shm_mut_t*)(x); }
static inline z_moved_shm_provider_t* z_shm_provider_move(z_owned_shm_provider_t* x) { return (z_moved_shm_provider_t*)(x); }
static inline z_moved_slice_t* z_slice_move(z_owned_slice_t* x) { return (z_moved_slice_t*)(x); }
static inline z_moved_string_array_t* z_string_array_move(z_owned_string_array_t* x) { return (z_moved_string_array_t*)(x); }
static inline z_moved_string_t* z_string_move(z_owned_string_t* x) { return (z_moved_string_t*)(x); }
static inline z_moved_subscriber_t* z_subscriber_move(z_owned_subscriber_t* x) { return (z_moved_subscriber_t*)(x); }
static inline z_moved_task_t* z_task_move(z_owned_task_t* x) { return (z_moved_task_t*)(x); }
static inline zc_moved_closure_log_t* zc_closure_log_move(zc_owned_closure_log_t* x) { return (zc_moved_closure_log_t*)(x); }
static inline zc_moved_concurrent_close_handle_t* zc_concurrent_close_handle_move(zc_owned_concurrent_close_handle_t* x) { return (zc_moved_concurrent_close_handle_t*)(x); }
static inline zc_moved_shm_client_list_t* zc_shm_client_list_move(zc_owned_shm_client_list_t* x) { return (zc_moved_shm_client_list_t*)(x); }
static inline ze_moved_advanced_publisher_t* ze_advanced_publisher_move(ze_owned_advanced_publisher_t* x) { return (ze_moved_advanced_publisher_t*)(x); }
static inline ze_moved_advanced_subscriber_t* ze_advanced_subscriber_move(ze_owned_advanced_subscriber_t* x) { return (ze_moved_advanced_subscriber_t*)(x); }
static inline ze_moved_closure_miss_t* ze_closure_miss_move(ze_owned_closure_miss_t* x) { return (ze_moved_closure_miss_t*)(x); }
static inline ze_moved_publication_cache_t* ze_publication_cache_move(ze_owned_publication_cache_t* x) { return (ze_moved_publication_cache_t*)(x); }
static inline ze_moved_querying_subscriber_t* ze_querying_subscriber_move(ze_owned_querying_subscriber_t* x) { return (ze_moved_querying_subscriber_t*)(x); }
static inline ze_moved_sample_miss_listener_t* ze_sample_miss_listener_move(ze_owned_sample_miss_listener_t* x) { return (ze_moved_sample_miss_listener_t*)(x); }
static inline ze_moved_serializer_t* ze_serializer_move(ze_owned_serializer_t* x) { return (ze_moved_serializer_t*)(x); }


#define z_loan(this_) \
    _Generic((this_), \
        z_owned_bytes_t : z_bytes_loan, \
        z_owned_bytes_writer_t : z_bytes_writer_loan, \
        z_owned_cancellation_token_t : z_cancellation_token_loan, \
        z_owned_closure_hello_t : z_closure_hello_loan, \
        z_owned_closure_matching_status_t : z_closure_matching_status_loan, \
        z_owned_closure_query_t : z_closure_query_loan, \
        z_owned_closure_reply_t : z_closure_reply_loan, \
        z_owned_closure_sample_t : z_closure_sample_loan, \
        z_owned_closure_zid_t : z_closure_zid_loan, \
        z_owned_condvar_t : z_condvar_loan, \
        z_owned_config_t : z_config_loan, \
        z_owned_encoding_t : z_encoding_loan, \
        z_owned_fifo_handler_query_t : z_fifo_handler_query_loan, \
        z_owned_fifo_handler_reply_t : z_fifo_handler_reply_loan, \
        z_owned_fifo_handler_sample_t : z_fifo_handler_sample_loan, \
        z_owned_hello_t : z_hello_loan, \
        z_owned_keyexpr_t : z_keyexpr_loan, \
        z_owned_liveliness_token_t : z_liveliness_token_loan, \
        z_owned_memory_layout_t : z_memory_layout_loan, \
        z_owned_precomputed_layout_t : z_precomputed_layout_loan, \
        z_owned_ptr_in_segment_t : z_ptr_in_segment_loan, \
        z_owned_publisher_t : z_publisher_loan, \
        z_owned_querier_t : z_querier_loan, \
        z_owned_query_t : z_query_loan, \
        z_owned_queryable_t : z_queryable_loan, \
        z_owned_reply_err_t : z_reply_err_loan, \
        z_owned_reply_t : z_reply_loan, \
        z_owned_ring_handler_query_t : z_ring_handler_query_loan, \
        z_owned_ring_handler_reply_t : z_ring_handler_reply_loan, \
        z_owned_ring_handler_sample_t : z_ring_handler_sample_loan, \
        z_owned_sample_t : z_sample_loan, \
        z_owned_session_t : z_session_loan, \
        z_owned_shared_shm_provider_t : z_shared_shm_provider_loan, \
        z_owned_shm_client_storage_t : z_shm_client_storage_loan, \
        z_owned_shm_t : z_shm_loan, \
        z_owned_shm_mut_t : z_shm_mut_loan, \
        z_owned_shm_provider_t : z_shm_provider_loan, \
        z_owned_slice_t : z_slice_loan, \
        z_owned_string_array_t : z_string_array_loan, \
        z_owned_string_t : z_string_loan, \
        z_owned_subscriber_t : z_subscriber_loan, \
        z_view_keyexpr_t : z_view_keyexpr_loan, \
        z_view_slice_t : z_view_slice_loan, \
        z_view_string_t : z_view_string_loan, \
        zc_owned_closure_log_t : zc_closure_log_loan, \
        zc_owned_shm_client_list_t : zc_shm_client_list_loan, \
        ze_owned_advanced_publisher_t : ze_advanced_publisher_loan, \
        ze_owned_advanced_subscriber_t : ze_advanced_subscriber_loan, \
        ze_owned_closure_miss_t : ze_closure_miss_loan, \
        ze_owned_publication_cache_t : ze_publication_cache_loan, \
        ze_owned_querying_subscriber_t : ze_querying_subscriber_loan, \
        ze_owned_serializer_t : ze_serializer_loan \
    )(&this_)

#define z_loan_mut(this_) \
    _Generic((this_), \
        z_owned_bytes_t : z_bytes_loan_mut, \
        z_owned_bytes_writer_t : z_bytes_writer_loan_mut, \
        z_owned_cancellation_token_t : z_cancellation_token_loan_mut, \
        z_owned_closure_hello_t : z_closure_hello_loan_mut, \
        z_owned_closure_query_t : z_closure_query_loan_mut, \
        z_owned_closure_reply_t : z_closure_reply_loan_mut, \
        z_owned_closure_sample_t : z_closure_sample_loan_mut, \
        z_owned_condvar_t : z_condvar_loan_mut, \
        z_owned_config_t : z_config_loan_mut, \
        z_owned_encoding_t : z_encoding_loan_mut, \
        z_owned_hello_t : z_hello_loan_mut, \
        z_owned_mutex_t : z_mutex_loan_mut, \
        z_owned_publisher_t : z_publisher_loan_mut, \
        z_owned_querier_t : z_querier_loan_mut, \
        z_owned_query_t : z_query_loan_mut, \
        z_owned_reply_err_t : z_reply_err_loan_mut, \
        z_owned_reply_t : z_reply_loan_mut, \
        z_owned_sample_t : z_sample_loan_mut, \
        z_owned_session_t : z_session_loan_mut, \
        z_owned_shm_t : z_shm_loan_mut, \
        z_owned_shm_mut_t : z_shm_mut_loan_mut, \
        z_owned_string_array_t : z_string_array_loan_mut, \
        zc_owned_shm_client_list_t : zc_shm_client_list_loan_mut, \
        ze_owned_advanced_publisher_t : ze_advanced_publisher_loan_mut, \
        ze_owned_serializer_t : ze_serializer_loan_mut \
    )(&this_)

#define z_drop(this_) \
    _Generic((this_), \
        z_moved_bytes_t* : z_bytes_drop, \
        z_moved_bytes_writer_t* : z_bytes_writer_drop, \
        z_moved_cancellation_token_t* : z_cancellation_token_drop, \
        z_moved_chunk_alloc_result_t* : z_chunk_alloc_result_drop, \
        z_moved_closure_hello_t* : z_closure_hello_drop, \
        z_moved_closure_matching_status_t* : z_closure_matching_status_drop, \
        z_moved_closure_query_t* : z_closure_query_drop, \
        z_moved_closure_reply_t* : z_closure_reply_drop, \
        z_moved_closure_sample_t* : z_closure_sample_drop, \
        z_moved_closure_zid_t* : z_closure_zid_drop, \
        z_moved_condvar_t* : z_condvar_drop, \
        z_moved_config_t* : z_config_drop, \
        z_moved_encoding_t* : z_encoding_drop, \
        z_moved_fifo_handler_query_t* : z_fifo_handler_query_drop, \
        z_moved_fifo_handler_reply_t* : z_fifo_handler_reply_drop, \
        z_moved_fifo_handler_sample_t* : z_fifo_handler_sample_drop, \
        z_moved_hello_t* : z_hello_drop, \
        z_moved_keyexpr_t* : z_keyexpr_drop, \
        z_moved_liveliness_token_t* : z_liveliness_token_drop, \
        z_moved_matching_listener_t* : z_matching_listener_drop, \
        z_moved_memory_layout_t* : z_memory_layout_drop, \
        z_moved_mutex_t* : z_mutex_drop, \
        z_moved_precomputed_layout_t* : z_precomputed_layout_drop, \
        z_moved_ptr_in_segment_t* : z_ptr_in_segment_drop, \
        z_moved_publisher_t* : z_publisher_drop, \
        z_moved_querier_t* : z_querier_drop, \
        z_moved_query_t* : z_query_drop, \
        z_moved_queryable_t* : z_queryable_drop, \
        z_moved_reply_t* : z_reply_drop, \
        z_moved_reply_err_t* : z_reply_err_drop, \
        z_moved_ring_handler_query_t* : z_ring_handler_query_drop, \
        z_moved_ring_handler_reply_t* : z_ring_handler_reply_drop, \
        z_moved_ring_handler_sample_t* : z_ring_handler_sample_drop, \
        z_moved_sample_t* : z_sample_drop, \
        z_moved_session_t* : z_session_drop, \
        z_moved_shared_shm_provider_t* : z_shared_shm_provider_drop, \
        z_moved_shm_client_t* : z_shm_client_drop, \
        z_moved_shm_client_storage_t* : z_shm_client_storage_drop, \
        z_moved_shm_t* : z_shm_drop, \
        z_moved_shm_mut_t* : z_shm_mut_drop, \
        z_moved_shm_provider_t* : z_shm_provider_drop, \
        z_moved_slice_t* : z_slice_drop, \
        z_moved_string_array_t* : z_string_array_drop, \
        z_moved_string_t* : z_string_drop, \
        z_moved_subscriber_t* : z_subscriber_drop, \
        z_moved_task_t* : z_task_drop, \
        zc_moved_closure_log_t* : zc_closure_log_drop, \
        zc_moved_concurrent_close_handle_t* : zc_concurrent_close_handle_drop, \
        zc_moved_shm_client_list_t* : zc_shm_client_list_drop, \
        ze_moved_advanced_publisher_t* : ze_advanced_publisher_drop, \
        ze_moved_advanced_subscriber_t* : ze_advanced_subscriber_drop, \
        ze_moved_closure_miss_t* : ze_closure_miss_drop, \
        ze_moved_publication_cache_t* : ze_publication_cache_drop, \
        ze_moved_querying_subscriber_t* : ze_querying_subscriber_drop, \
        ze_moved_sample_miss_listener_t* : ze_sample_miss_listener_drop, \
        ze_moved_serializer_t* : ze_serializer_drop \
    )(this_)

#define z_move(this_) \
    _Generic((this_), \
        z_owned_bytes_t : z_bytes_move, \
        z_owned_bytes_writer_t : z_bytes_writer_move, \
        z_owned_cancellation_token_t : z_cancellation_token_move, \
        z_owned_chunk_alloc_result_t : z_chunk_alloc_result_move, \
        z_owned_closure_hello_t : z_closure_hello_move, \
        z_owned_closure_matching_status_t : z_closure_matching_status_move, \
        z_owned_closure_query_t : z_closure_query_move, \
        z_owned_closure_reply_t : z_closure_reply_move, \
        z_owned_closure_sample_t : z_closure_sample_move, \
        z_owned_closure_zid_t : z_closure_zid_move, \
        z_owned_condvar_t : z_condvar_move, \
        z_owned_config_t : z_config_move, \
        z_owned_encoding_t : z_encoding_move, \
        z_owned_fifo_handler_query_t : z_fifo_handler_query_move, \
        z_owned_fifo_handler_reply_t : z_fifo_handler_reply_move, \
        z_owned_fifo_handler_sample_t : z_fifo_handler_sample_move, \
        z_owned_hello_t : z_hello_move, \
        z_owned_keyexpr_t : z_keyexpr_move, \
        z_owned_liveliness_token_t : z_liveliness_token_move, \
        z_owned_matching_listener_t : z_matching_listener_move, \
        z_owned_memory_layout_t : z_memory_layout_move, \
        z_owned_mutex_t : z_mutex_move, \
        z_owned_precomputed_layout_t : z_precomputed_layout_move, \
        z_owned_ptr_in_segment_t : z_ptr_in_segment_move, \
        z_owned_publisher_t : z_publisher_move, \
        z_owned_querier_t : z_querier_move, \
        z_owned_query_t : z_query_move, \
        z_owned_queryable_t : z_queryable_move, \
        z_owned_reply_t : z_reply_move, \
        z_owned_reply_err_t : z_reply_err_move, \
        z_owned_ring_handler_query_t : z_ring_handler_query_move, \
        z_owned_ring_handler_reply_t : z_ring_handler_reply_move, \
        z_owned_ring_handler_sample_t : z_ring_handler_sample_move, \
        z_owned_sample_t : z_sample_move, \
        z_owned_session_t : z_session_move, \
        z_owned_shared_shm_provider_t : z_shared_shm_provider_move, \
        z_owned_shm_client_t : z_shm_client_move, \
        z_owned_shm_client_storage_t : z_shm_client_storage_move, \
        z_owned_shm_t : z_shm_move, \
        z_owned_shm_mut_t : z_shm_mut_move, \
        z_owned_shm_provider_t : z_shm_provider_move, \
        z_owned_slice_t : z_slice_move, \
        z_owned_string_array_t : z_string_array_move, \
        z_owned_string_t : z_string_move, \
        z_owned_subscriber_t : z_subscriber_move, \
        z_owned_task_t : z_task_move, \
        zc_owned_closure_log_t : zc_closure_log_move, \
        zc_owned_concurrent_close_handle_t : zc_concurrent_close_handle_move, \
        zc_owned_shm_client_list_t : zc_shm_client_list_move, \
        ze_owned_advanced_publisher_t : ze_advanced_publisher_move, \
        ze_owned_advanced_subscriber_t : ze_advanced_subscriber_move, \
        ze_owned_closure_miss_t : ze_closure_miss_move, \
        ze_owned_publication_cache_t : ze_publication_cache_move, \
        ze_owned_querying_subscriber_t : ze_querying_subscriber_move, \
        ze_owned_sample_miss_listener_t : ze_sample_miss_listener_move, \
        ze_owned_serializer_t : ze_serializer_move \
    )(&this_)

#define z_internal_null(this_) \
    _Generic((this_), \
        z_owned_bytes_t* : z_internal_bytes_null, \
        z_owned_bytes_writer_t* : z_internal_bytes_writer_null, \
        z_owned_cancellation_token_t* : z_internal_cancellation_token_null, \
        z_owned_chunk_alloc_result_t* : z_internal_chunk_alloc_result_null, \
        z_owned_closure_hello_t* : z_internal_closure_hello_null, \
        z_owned_closure_matching_status_t* : z_internal_closure_matching_status_null, \
        z_owned_closure_query_t* : z_internal_closure_query_null, \
        z_owned_closure_reply_t* : z_internal_closure_reply_null, \
        z_owned_closure_sample_t* : z_internal_closure_sample_null, \
        z_owned_closure_zid_t* : z_internal_closure_zid_null, \
        z_owned_condvar_t* : z_internal_condvar_null, \
        z_owned_config_t* : z_internal_config_null, \
        z_owned_encoding_t* : z_internal_encoding_null, \
        z_owned_fifo_handler_query_t* : z_internal_fifo_handler_query_null, \
        z_owned_fifo_handler_reply_t* : z_internal_fifo_handler_reply_null, \
        z_owned_fifo_handler_sample_t* : z_internal_fifo_handler_sample_null, \
        z_owned_hello_t* : z_internal_hello_null, \
        z_owned_keyexpr_t* : z_internal_keyexpr_null, \
        z_owned_liveliness_token_t* : z_internal_liveliness_token_null, \
        z_owned_matching_listener_t* : z_internal_matching_listener_null, \
        z_owned_memory_layout_t* : z_internal_memory_layout_null, \
        z_owned_mutex_t* : z_internal_mutex_null, \
        z_owned_precomputed_layout_t* : z_internal_precomputed_layout_null, \
        z_owned_ptr_in_segment_t* : z_internal_ptr_in_segment_null, \
        z_owned_publisher_t* : z_internal_publisher_null, \
        z_owned_querier_t* : z_internal_querier_null, \
        z_owned_query_t* : z_internal_query_null, \
        z_owned_queryable_t* : z_internal_queryable_null, \
        z_owned_reply_err_t* : z_internal_reply_err_null, \
        z_owned_reply_t* : z_internal_reply_null, \
        z_owned_ring_handler_query_t* : z_internal_ring_handler_query_null, \
        z_owned_ring_handler_reply_t* : z_internal_ring_handler_reply_null, \
        z_owned_ring_handler_sample_t* : z_internal_ring_handler_sample_null, \
        z_owned_sample_t* : z_internal_sample_null, \
        z_owned_session_t* : z_internal_session_null, \
        z_owned_shared_shm_provider_t* : z_internal_shared_shm_provider_null, \
        z_owned_shm_client_t* : z_internal_shm_client_null, \
        z_owned_shm_client_storage_t* : z_internal_shm_client_storage_null, \
        z_owned_shm_mut_t* : z_internal_shm_mut_null, \
        z_owned_shm_t* : z_internal_shm_null, \
        z_owned_shm_provider_t* : z_internal_shm_provider_null, \
        z_owned_slice_t* : z_internal_slice_null, \
        z_owned_string_array_t* : z_internal_string_array_null, \
        z_owned_string_t* : z_internal_string_null, \
        z_owned_subscriber_t* : z_internal_subscriber_null, \
        z_owned_task_t* : z_internal_task_null, \
        zc_owned_closure_log_t* : zc_internal_closure_log_null, \
        zc_owned_concurrent_close_handle_t* : zc_internal_concurrent_close_handle_null, \
        zc_owned_shm_client_list_t* : zc_internal_shm_client_list_null, \
        ze_owned_advanced_publisher_t* : ze_internal_advanced_publisher_null, \
        ze_owned_advanced_subscriber_t* : ze_internal_advanced_subscriber_null, \
        ze_owned_closure_miss_t* : ze_internal_closure_miss_null, \
        ze_owned_publication_cache_t* : ze_internal_publication_cache_null, \
        ze_owned_querying_subscriber_t* : ze_internal_querying_subscriber_null, \
        ze_owned_sample_miss_listener_t* : ze_internal_sample_miss_listener_null, \
        ze_owned_serializer_t* : ze_internal_serializer_null \
    )(this_)

static inline void z_bytes_take(z_owned_bytes_t* this_, z_moved_bytes_t* x) { *this_ = x->_this; z_internal_bytes_null(&x->_this); }
static inline void z_bytes_writer_take(z_owned_bytes_writer_t* this_, z_moved_bytes_writer_t* x) { *this_ = x->_this; z_internal_bytes_writer_null(&x->_this); }
static inline void z_cancellation_token_take(z_owned_cancellation_token_t* this_, z_moved_cancellation_token_t* x) { *this_ = x->_this; z_internal_cancellation_token_null(&x->_this); }
static inline void z_chunk_alloc_result_take(z_owned_chunk_alloc_result_t* this_, z_moved_chunk_alloc_result_t* x) { *this_ = x->_this; z_internal_chunk_alloc_result_null(&x->_this); }
static inline void z_closure_hello_take(z_owned_closure_hello_t* this_, z_moved_closure_hello_t* x) { *this_ = x->_this; z_internal_closure_hello_null(&x->_this); }
static inline void z_closure_matching_status_take(z_owned_closure_matching_status_t* closure_, z_moved_closure_matching_status_t* x) { *closure_ = x->_this; z_internal_closure_matching_status_null(&x->_this); }
static inline void z_closure_query_take(z_owned_closure_query_t* closure_, z_moved_closure_query_t* x) { *closure_ = x->_this; z_internal_closure_query_null(&x->_this); }
static inline void z_closure_reply_take(z_owned_closure_reply_t* closure_, z_moved_closure_reply_t* x) { *closure_ = x->_this; z_internal_closure_reply_null(&x->_this); }
static inline void z_closure_sample_take(z_owned_closure_sample_t* closure_, z_moved_closure_sample_t* x) { *closure_ = x->_this; z_internal_closure_sample_null(&x->_this); }
static inline void z_closure_zid_take(z_owned_closure_zid_t* closure_, z_moved_closure_zid_t* x) { *closure_ = x->_this; z_internal_closure_zid_null(&x->_this); }
static inline void z_condvar_take(z_owned_condvar_t* this_, z_moved_condvar_t* x) { *this_ = x->_this; z_internal_condvar_null(&x->_this); }
static inline void z_config_take(z_owned_config_t* this_, z_moved_config_t* x) { *this_ = x->_this; z_internal_config_null(&x->_this); }
static inline void z_encoding_take(z_owned_encoding_t* this_, z_moved_encoding_t* x) { *this_ = x->_this; z_internal_encoding_null(&x->_this); }
static inline void z_fifo_handler_query_take(z_owned_fifo_handler_query_t* this_, z_moved_fifo_handler_query_t* x) { *this_ = x->_this; z_internal_fifo_handler_query_null(&x->_this); }
static inline void z_fifo_handler_reply_take(z_owned_fifo_handler_reply_t* this_, z_moved_fifo_handler_reply_t* x) { *this_ = x->_this; z_internal_fifo_handler_reply_null(&x->_this); }
static inline void z_fifo_handler_sample_take(z_owned_fifo_handler_sample_t* this_, z_moved_fifo_handler_sample_t* x) { *this_ = x->_this; z_internal_fifo_handler_sample_null(&x->_this); }
static inline void z_hello_take(z_owned_hello_t* this_, z_moved_hello_t* x) { *this_ = x->_this; z_internal_hello_null(&x->_this); }
static inline void z_keyexpr_take(z_owned_keyexpr_t* this_, z_moved_keyexpr_t* x) { *this_ = x->_this; z_internal_keyexpr_null(&x->_this); }
static inline void z_liveliness_token_take(z_owned_liveliness_token_t* this_, z_moved_liveliness_token_t* x) { *this_ = x->_this; z_internal_liveliness_token_null(&x->_this); }
static inline void z_matching_listener_take(z_owned_matching_listener_t* this_, z_moved_matching_listener_t* x) { *this_ = x->_this; z_internal_matching_listener_null(&x->_this); }
static inline void z_memory_layout_take(z_owned_memory_layout_t* this_, z_moved_memory_layout_t* x) { *this_ = x->_this; z_internal_memory_layout_null(&x->_this); }
static inline void z_mutex_take(z_owned_mutex_t* this_, z_moved_mutex_t* x) { *this_ = x->_this; z_internal_mutex_null(&x->_this); }
static inline void z_precomputed_layout_take(z_owned_precomputed_layout_t* this_, z_moved_precomputed_layout_t* x) { *this_ = x->_this; z_internal_precomputed_layout_null(&x->_this); }
static inline void z_ptr_in_segment_take(z_owned_ptr_in_segment_t* this_, z_moved_ptr_in_segment_t* x) { *this_ = x->_this; z_internal_ptr_in_segment_null(&x->_this); }
static inline void z_publisher_take(z_owned_publisher_t* this_, z_moved_publisher_t* x) { *this_ = x->_this; z_internal_publisher_null(&x->_this); }
static inline void z_querier_take(z_owned_querier_t* this_, z_moved_querier_t* x) { *this_ = x->_this; z_internal_querier_null(&x->_this); }
static inline void z_query_take(z_owned_query_t* this_, z_moved_query_t* x) { *this_ = x->_this; z_internal_query_null(&x->_this); }
static inline void z_queryable_take(z_owned_queryable_t* this_, z_moved_queryable_t* x) { *this_ = x->_this; z_internal_queryable_null(&x->_this); }
static inline void z_reply_take(z_owned_reply_t* this_, z_moved_reply_t* x) { *this_ = x->_this; z_internal_reply_null(&x->_this); }
static inline void z_reply_err_take(z_owned_reply_err_t* this_, z_moved_reply_err_t* x) { *this_ = x->_this; z_internal_reply_err_null(&x->_this); }
static inline void z_ring_handler_query_take(z_owned_ring_handler_query_t* this_, z_moved_ring_handler_query_t* x) { *this_ = x->_this; z_internal_ring_handler_query_null(&x->_this); }
static inline void z_ring_handler_reply_take(z_owned_ring_handler_reply_t* this_, z_moved_ring_handler_reply_t* x) { *this_ = x->_this; z_internal_ring_handler_reply_null(&x->_this); }
static inline void z_ring_handler_sample_take(z_owned_ring_handler_sample_t* this_, z_moved_ring_handler_sample_t* x) { *this_ = x->_this; z_internal_ring_handler_sample_null(&x->_this); }
static inline void z_sample_take(z_owned_sample_t* this_, z_moved_sample_t* x) { *this_ = x->_this; z_internal_sample_null(&x->_this); }
static inline void z_session_take(z_owned_session_t* this_, z_moved_session_t* x) { *this_ = x->_this; z_internal_session_null(&x->_this); }
static inline void z_shared_shm_provider_take(z_owned_shared_shm_provider_t* this_, z_moved_shared_shm_provider_t* x) { *this_ = x->_this; z_internal_shared_shm_provider_null(&x->_this); }
static inline void z_shm_client_take(z_owned_shm_client_t* this_, z_moved_shm_client_t* x) { *this_ = x->_this; z_internal_shm_client_null(&x->_this); }
static inline void z_shm_client_storage_take(z_owned_shm_client_storage_t* this_, z_moved_shm_client_storage_t* x) { *this_ = x->_this; z_internal_shm_client_storage_null(&x->_this); }
static inline void z_shm_take(z_owned_shm_t* this_, z_moved_shm_t* x) { *this_ = x->_this; z_internal_shm_null(&x->_this); }
static inline void z_shm_mut_take(z_owned_shm_mut_t* this_, z_moved_shm_mut_t* x) { *this_ = x->_this; z_internal_shm_mut_null(&x->_this); }
static inline void z_shm_provider_take(z_owned_shm_provider_t* this_, z_moved_shm_provider_t* x) { *this_ = x->_this; z_internal_shm_provider_null(&x->_this); }
static inline void z_slice_take(z_owned_slice_t* this_, z_moved_slice_t* x) { *this_ = x->_this; z_internal_slice_null(&x->_this); }
static inline void z_string_array_take(z_owned_string_array_t* this_, z_moved_string_array_t* x) { *this_ = x->_this; z_internal_string_array_null(&x->_this); }
static inline void z_string_take(z_owned_string_t* this_, z_moved_string_t* x) { *this_ = x->_this; z_internal_string_null(&x->_this); }
static inline void z_subscriber_take(z_owned_subscriber_t* this_, z_moved_subscriber_t* x) { *this_ = x->_this; z_internal_subscriber_null(&x->_this); }
static inline void z_task_take(z_owned_task_t* this_, z_moved_task_t* x) { *this_ = x->_this; z_internal_task_null(&x->_this); }
static inline void zc_closure_log_take(zc_owned_closure_log_t* closure_, zc_moved_closure_log_t* x) { *closure_ = x->_this; zc_internal_closure_log_null(&x->_this); }
static inline void zc_concurrent_close_handle_take(zc_owned_concurrent_close_handle_t* this_, zc_moved_concurrent_close_handle_t* x) { *this_ = x->_this; zc_internal_concurrent_close_handle_null(&x->_this); }
static inline void zc_shm_client_list_take(zc_owned_shm_client_list_t* this_, zc_moved_shm_client_list_t* x) { *this_ = x->_this; zc_internal_shm_client_list_null(&x->_this); }
static inline void ze_advanced_publisher_take(ze_owned_advanced_publisher_t* this_, ze_moved_advanced_publisher_t* x) { *this_ = x->_this; ze_internal_advanced_publisher_null(&x->_this); }
static inline void ze_advanced_subscriber_take(ze_owned_advanced_subscriber_t* this_, ze_moved_advanced_subscriber_t* x) { *this_ = x->_this; ze_internal_advanced_subscriber_null(&x->_this); }
static inline void ze_closure_miss_take(ze_owned_closure_miss_t* closure_, ze_moved_closure_miss_t* x) { *closure_ = x->_this; ze_internal_closure_miss_null(&x->_this); }
static inline void ze_publication_cache_take(ze_owned_publication_cache_t* this_, ze_moved_publication_cache_t* x) { *this_ = x->_this; ze_internal_publication_cache_null(&x->_this); }
static inline void ze_querying_subscriber_take(ze_owned_querying_subscriber_t* this_, ze_moved_querying_subscriber_t* x) { *this_ = x->_this; ze_internal_querying_subscriber_null(&x->_this); }
static inline void ze_sample_miss_listener_take(ze_owned_sample_miss_listener_t* this_, ze_moved_sample_miss_listener_t* x) { *this_ = x->_this; ze_internal_sample_miss_listener_null(&x->_this); }
static inline void ze_serializer_take(ze_owned_serializer_t* this_, ze_moved_serializer_t* x) { *this_ = x->_this; ze_internal_serializer_null(&x->_this); }


#define z_take(this_, x) \
    _Generic((this_), \
        z_owned_bytes_t* : z_bytes_take, \
        z_owned_bytes_writer_t* : z_bytes_writer_take, \
        z_owned_cancellation_token_t* : z_cancellation_token_take, \
        z_owned_chunk_alloc_result_t* : z_chunk_alloc_result_take, \
        z_owned_closure_hello_t* : z_closure_hello_take, \
        z_owned_closure_matching_status_t* : z_closure_matching_status_take, \
        z_owned_closure_query_t* : z_closure_query_take, \
        z_owned_closure_reply_t* : z_closure_reply_take, \
        z_owned_closure_sample_t* : z_closure_sample_take, \
        z_owned_closure_zid_t* : z_closure_zid_take, \
        z_owned_condvar_t* : z_condvar_take, \
        z_owned_config_t* : z_config_take, \
        z_owned_encoding_t* : z_encoding_take, \
        z_owned_fifo_handler_query_t* : z_fifo_handler_query_take, \
        z_owned_fifo_handler_reply_t* : z_fifo_handler_reply_take, \
        z_owned_fifo_handler_sample_t* : z_fifo_handler_sample_take, \
        z_owned_hello_t* : z_hello_take, \
        z_owned_keyexpr_t* : z_keyexpr_take, \
        z_owned_liveliness_token_t* : z_liveliness_token_take, \
        z_owned_matching_listener_t* : z_matching_listener_take, \
        z_owned_memory_layout_t* : z_memory_layout_take, \
        z_owned_mutex_t* : z_mutex_take, \
        z_owned_precomputed_layout_t* : z_precomputed_layout_take, \
        z_owned_ptr_in_segment_t* : z_ptr_in_segment_take, \
        z_owned_publisher_t* : z_publisher_take, \
        z_owned_querier_t* : z_querier_take, \
        z_owned_query_t* : z_query_take, \
        z_owned_queryable_t* : z_queryable_take, \
        z_owned_reply_t* : z_reply_take, \
        z_owned_reply_err_t* : z_reply_err_take, \
        z_owned_ring_handler_query_t* : z_ring_handler_query_take, \
        z_owned_ring_handler_reply_t* : z_ring_handler_reply_take, \
        z_owned_ring_handler_sample_t* : z_ring_handler_sample_take, \
        z_owned_sample_t* : z_sample_take, \
        z_owned_session_t* : z_session_take, \
        z_owned_shared_shm_provider_t* : z_shared_shm_provider_take, \
        z_owned_shm_client_t* : z_shm_client_take, \
        z_owned_shm_client_storage_t* : z_shm_client_storage_take, \
        z_owned_shm_t* : z_shm_take, \
        z_owned_shm_mut_t* : z_shm_mut_take, \
        z_owned_shm_provider_t* : z_shm_provider_take, \
        z_owned_slice_t* : z_slice_take, \
        z_owned_string_array_t* : z_string_array_take, \
        z_owned_string_t* : z_string_take, \
        z_owned_subscriber_t* : z_subscriber_take, \
        z_owned_task_t* : z_task_take, \
        zc_owned_closure_log_t* : zc_closure_log_take, \
        zc_owned_concurrent_close_handle_t* : zc_concurrent_close_handle_take, \
        zc_owned_shm_client_list_t* : zc_shm_client_list_take, \
        ze_owned_advanced_publisher_t* : ze_advanced_publisher_take, \
        ze_owned_advanced_subscriber_t* : ze_advanced_subscriber_take, \
        ze_owned_closure_miss_t* : ze_closure_miss_take, \
        ze_owned_publication_cache_t* : ze_publication_cache_take, \
        ze_owned_querying_subscriber_t* : ze_querying_subscriber_take, \
        ze_owned_sample_miss_listener_t* : ze_sample_miss_listener_take, \
        ze_owned_serializer_t* : ze_serializer_take \
    )(this_, x)

#define z_take_from_loaned(dst, src) \
    _Generic((dst), \
        z_owned_hello_t* : z_hello_take_from_loaned, \
        z_owned_query_t* : z_query_take_from_loaned, \
        z_owned_reply_t* : z_reply_take_from_loaned, \
        z_owned_sample_t* : z_sample_take_from_loaned \
    )(dst, src)

#define z_internal_check(this_) \
    _Generic((this_), \
        z_owned_bytes_t : z_internal_bytes_check, \
        z_owned_bytes_writer_t : z_internal_bytes_writer_check, \
        z_owned_cancellation_token_t : z_internal_cancellation_token_check, \
        z_owned_chunk_alloc_result_t : z_internal_chunk_alloc_result_check, \
        z_owned_closure_hello_t : z_internal_closure_hello_check, \
        z_owned_closure_matching_status_t : z_internal_closure_matching_status_check, \
        z_owned_closure_query_t : z_internal_closure_query_check, \
        z_owned_closure_reply_t : z_internal_closure_reply_check, \
        z_owned_closure_sample_t : z_internal_closure_sample_check, \
        z_owned_closure_zid_t : z_internal_closure_zid_check, \
        z_owned_condvar_t : z_internal_condvar_check, \
        z_owned_config_t : z_internal_config_check, \
        z_owned_encoding_t : z_internal_encoding_check, \
        z_owned_fifo_handler_query_t : z_internal_fifo_handler_query_check, \
        z_owned_fifo_handler_reply_t : z_internal_fifo_handler_reply_check, \
        z_owned_fifo_handler_sample_t : z_internal_fifo_handler_sample_check, \
        z_owned_hello_t : z_internal_hello_check, \
        z_owned_keyexpr_t : z_internal_keyexpr_check, \
        z_owned_liveliness_token_t : z_internal_liveliness_token_check, \
        z_owned_matching_listener_t : z_internal_matching_listener_check, \
        z_owned_memory_layout_t : z_internal_memory_layout_check, \
        z_owned_mutex_t : z_internal_mutex_check, \
        z_owned_precomputed_layout_t : z_internal_precomputed_layout_check, \
        z_owned_ptr_in_segment_t : z_internal_ptr_in_segment_check, \
        z_owned_publisher_t : z_internal_publisher_check, \
        z_owned_querier_t : z_internal_querier_check, \
        z_owned_query_t : z_internal_query_check, \
        z_owned_queryable_t : z_internal_queryable_check, \
        z_owned_reply_t : z_internal_reply_check, \
        z_owned_reply_err_t : z_internal_reply_err_check, \
        z_owned_ring_handler_query_t : z_internal_ring_handler_query_check, \
        z_owned_ring_handler_reply_t : z_internal_ring_handler_reply_check, \
        z_owned_ring_handler_sample_t : z_internal_ring_handler_sample_check, \
        z_owned_sample_t : z_internal_sample_check, \
        z_owned_session_t : z_internal_session_check, \
        z_owned_shared_shm_provider_t : z_internal_shared_shm_provider_check, \
        z_owned_shm_t : z_internal_shm_check, \
        z_owned_shm_client_t : z_internal_shm_client_check, \
        z_owned_shm_client_storage_t : z_internal_shm_client_storage_check, \
        z_owned_shm_mut_t : z_internal_shm_mut_check, \
        z_owned_shm_provider_t : z_internal_shm_provider_check, \
        z_owned_slice_t : z_internal_slice_check, \
        z_owned_string_array_t : z_internal_string_array_check, \
        z_owned_string_t : z_internal_string_check, \
        z_owned_subscriber_t : z_internal_subscriber_check, \
        z_owned_task_t : z_internal_task_check, \
        zc_owned_closure_log_t : zc_internal_closure_log_check, \
        zc_owned_concurrent_close_handle_t : zc_internal_concurrent_close_handle_check, \
        zc_owned_shm_client_list_t : zc_internal_shm_client_list_check, \
        ze_owned_advanced_publisher_t : ze_internal_advanced_publisher_check, \
        ze_owned_advanced_subscriber_t : ze_internal_advanced_subscriber_check, \
        ze_owned_closure_miss_t : ze_internal_closure_miss_check, \
        ze_owned_publication_cache_t : ze_internal_publication_cache_check, \
        ze_owned_querying_subscriber_t : ze_internal_querying_subscriber_check, \
        ze_owned_sample_miss_listener_t : ze_internal_sample_miss_listener_check, \
        ze_owned_serializer_t : ze_internal_serializer_check \
    )(&this_)

#define z_call(closure, hello) \
    _Generic((closure), \
        const z_loaned_closure_hello_t* : z_closure_hello_call, \
        const z_loaned_closure_matching_status_t* : z_closure_matching_status_call, \
        const z_loaned_closure_query_t* : z_closure_query_call, \
        const z_loaned_closure_reply_t* : z_closure_reply_call, \
        const z_loaned_closure_sample_t* : z_closure_sample_call, \
        const z_loaned_closure_zid_t* : z_closure_zid_call, \
        const ze_loaned_closure_miss_t* : ze_closure_miss_call \
    )(closure, hello)

typedef void(*z_closure_drop_callback_t)(void *context);
typedef void(*z_closure_hello_callback_t)(z_loaned_hello_t *hello, void *context);
typedef void(*z_closure_matching_status_callback_t)(const z_matching_status_t *matching_status, void *context);
typedef void(*z_closure_query_callback_t)(z_loaned_query_t *query, void *context);
typedef void(*z_closure_reply_callback_t)(z_loaned_reply_t *reply, void *context);
typedef void(*z_closure_sample_callback_t)(z_loaned_sample_t *sample, void *context);
typedef void(*z_closure_zid_callback_t)(const z_id_t *z_id, void *context);
typedef void(*zc_closure_log_callback_t)(zc_log_severity_t severity, const z_loaned_string_t *msg, void *context);
typedef void(*ze_closure_miss_callback_t)(const ze_miss_t *matching_status, void *context);

#define z_closure(this_, call, drop, context) \
    _Generic((this_), \
        z_owned_closure_hello_t* : z_closure_hello, \
        z_owned_closure_matching_status_t* : z_closure_matching_status, \
        z_owned_closure_query_t* : z_closure_query, \
        z_owned_closure_reply_t* : z_closure_reply, \
        z_owned_closure_sample_t* : z_closure_sample, \
        z_owned_closure_zid_t* : z_closure_zid, \
        zc_owned_closure_log_t* : zc_closure_log, \
        ze_owned_closure_miss_t* : ze_closure_miss \
    )(this_, call, drop, context)

#define z_try_recv(this_, query) \
    _Generic((this_), \
        const z_loaned_fifo_handler_query_t* : z_fifo_handler_query_try_recv, \
        const z_loaned_fifo_handler_reply_t* : z_fifo_handler_reply_try_recv, \
        const z_loaned_fifo_handler_sample_t* : z_fifo_handler_sample_try_recv, \
        const z_loaned_ring_handler_query_t* : z_ring_handler_query_try_recv, \
        const z_loaned_ring_handler_reply_t* : z_ring_handler_reply_try_recv, \
        const z_loaned_ring_handler_sample_t* : z_ring_handler_sample_try_recv \
    )(this_, query)

#define z_recv(this_, query) \
    _Generic((this_), \
        const z_loaned_fifo_handler_query_t* : z_fifo_handler_query_recv, \
        const z_loaned_fifo_handler_reply_t* : z_fifo_handler_reply_recv, \
        const z_loaned_fifo_handler_sample_t* : z_fifo_handler_sample_recv, \
        const z_loaned_ring_handler_query_t* : z_ring_handler_query_recv, \
        const z_loaned_ring_handler_reply_t* : z_ring_handler_reply_recv, \
        const z_loaned_ring_handler_sample_t* : z_ring_handler_sample_recv \
    )(this_, query)

#define z_clone(dst, this_) \
    _Generic((dst), \
        z_owned_bytes_t* : z_bytes_clone, \
        z_owned_cancellation_token_t* : z_cancellation_token_clone, \
        z_owned_config_t* : z_config_clone, \
        z_owned_encoding_t* : z_encoding_clone, \
        z_owned_hello_t* : z_hello_clone, \
        z_owned_keyexpr_t* : z_keyexpr_clone, \
        z_owned_ptr_in_segment_t* : z_ptr_in_segment_clone, \
        z_owned_query_t* : z_query_clone, \
        z_owned_reply_t* : z_reply_clone, \
        z_owned_reply_err_t* : z_reply_err_clone, \
        z_owned_sample_t* : z_sample_clone, \
        z_owned_shared_shm_provider_t* : z_shared_shm_provider_clone, \
        z_owned_shm_client_storage_t* : z_shm_client_storage_clone, \
        z_owned_shm_t* : z_shm_clone, \
        z_owned_slice_t* : z_slice_clone, \
        z_owned_string_array_t* : z_string_array_clone, \
        z_owned_string_t* : z_string_clone \
    )(dst, this_)
#else  // #ifndef __cplusplus


static inline z_moved_bytes_t* z_bytes_move(z_owned_bytes_t* x) { return reinterpret_cast<z_moved_bytes_t*>(x); }
static inline z_moved_bytes_writer_t* z_bytes_writer_move(z_owned_bytes_writer_t* x) { return reinterpret_cast<z_moved_bytes_writer_t*>(x); }
static inline z_moved_cancellation_token_t* z_cancellation_token_move(z_owned_cancellation_token_t* x) { return reinterpret_cast<z_moved_cancellation_token_t*>(x); }
static inline z_moved_chunk_alloc_result_t* z_chunk_alloc_result_move(z_owned_chunk_alloc_result_t* x) { return reinterpret_cast<z_moved_chunk_alloc_result_t*>(x); }
static inline z_moved_closure_hello_t* z_closure_hello_move(z_owned_closure_hello_t* x) { return reinterpret_cast<z_moved_closure_hello_t*>(x); }
static inline z_moved_closure_matching_status_t* z_closure_matching_status_move(z_owned_closure_matching_status_t* x) { return reinterpret_cast<z_moved_closure_matching_status_t*>(x); }
static inline z_moved_closure_query_t* z_closure_query_move(z_owned_closure_query_t* x) { return reinterpret_cast<z_moved_closure_query_t*>(x); }
static inline z_moved_closure_reply_t* z_closure_reply_move(z_owned_closure_reply_t* x) { return reinterpret_cast<z_moved_closure_reply_t*>(x); }
static inline z_moved_closure_sample_t* z_closure_sample_move(z_owned_closure_sample_t* x) { return reinterpret_cast<z_moved_closure_sample_t*>(x); }
static inline z_moved_closure_zid_t* z_closure_zid_move(z_owned_closure_zid_t* x) { return reinterpret_cast<z_moved_closure_zid_t*>(x); }
static inline z_moved_condvar_t* z_condvar_move(z_owned_condvar_t* x) { return reinterpret_cast<z_moved_condvar_t*>(x); }
static inline z_moved_config_t* z_config_move(z_owned_config_t* x) { return reinterpret_cast<z_moved_config_t*>(x); }
static inline z_moved_encoding_t* z_encoding_move(z_owned_encoding_t* x) { return reinterpret_cast<z_moved_encoding_t*>(x); }
static inline z_moved_fifo_handler_query_t* z_fifo_handler_query_move(z_owned_fifo_handler_query_t* x) { return reinterpret_cast<z_moved_fifo_handler_query_t*>(x); }
static inline z_moved_fifo_handler_reply_t* z_fifo_handler_reply_move(z_owned_fifo_handler_reply_t* x) { return reinterpret_cast<z_moved_fifo_handler_reply_t*>(x); }
static inline z_moved_fifo_handler_sample_t* z_fifo_handler_sample_move(z_owned_fifo_handler_sample_t* x) { return reinterpret_cast<z_moved_fifo_handler_sample_t*>(x); }
static inline z_moved_hello_t* z_hello_move(z_owned_hello_t* x) { return reinterpret_cast<z_moved_hello_t*>(x); }
static inline z_moved_keyexpr_t* z_keyexpr_move(z_owned_keyexpr_t* x) { return reinterpret_cast<z_moved_keyexpr_t*>(x); }
static inline z_moved_liveliness_token_t* z_liveliness_token_move(z_owned_liveliness_token_t* x) { return reinterpret_cast<z_moved_liveliness_token_t*>(x); }
static inline z_moved_matching_listener_t* z_matching_listener_move(z_owned_matching_listener_t* x) { return reinterpret_cast<z_moved_matching_listener_t*>(x); }
static inline z_moved_memory_layout_t* z_memory_layout_move(z_owned_memory_layout_t* x) { return reinterpret_cast<z_moved_memory_layout_t*>(x); }
static inline z_moved_mutex_t* z_mutex_move(z_owned_mutex_t* x) { return reinterpret_cast<z_moved_mutex_t*>(x); }
static inline z_moved_precomputed_layout_t* z_precomputed_layout_move(z_owned_precomputed_layout_t* x) { return reinterpret_cast<z_moved_precomputed_layout_t*>(x); }
static inline z_moved_ptr_in_segment_t* z_ptr_in_segment_move(z_owned_ptr_in_segment_t* x) { return reinterpret_cast<z_moved_ptr_in_segment_t*>(x); }
static inline z_moved_publisher_t* z_publisher_move(z_owned_publisher_t* x) { return reinterpret_cast<z_moved_publisher_t*>(x); }
static inline z_moved_querier_t* z_querier_move(z_owned_querier_t* x) { return reinterpret_cast<z_moved_querier_t*>(x); }
static inline z_moved_query_t* z_query_move(z_owned_query_t* x) { return reinterpret_cast<z_moved_query_t*>(x); }
static inline z_moved_queryable_t* z_queryable_move(z_owned_queryable_t* x) { return reinterpret_cast<z_moved_queryable_t*>(x); }
static inline z_moved_reply_t* z_reply_move(z_owned_reply_t* x) { return reinterpret_cast<z_moved_reply_t*>(x); }
static inline z_moved_reply_err_t* z_reply_err_move(z_owned_reply_err_t* x) { return reinterpret_cast<z_moved_reply_err_t*>(x); }
static inline z_moved_ring_handler_query_t* z_ring_handler_query_move(z_owned_ring_handler_query_t* x) { return reinterpret_cast<z_moved_ring_handler_query_t*>(x); }
static inline z_moved_ring_handler_reply_t* z_ring_handler_reply_move(z_owned_ring_handler_reply_t* x) { return reinterpret_cast<z_moved_ring_handler_reply_t*>(x); }
static inline z_moved_ring_handler_sample_t* z_ring_handler_sample_move(z_owned_ring_handler_sample_t* x) { return reinterpret_cast<z_moved_ring_handler_sample_t*>(x); }
static inline z_moved_sample_t* z_sample_move(z_owned_sample_t* x) { return reinterpret_cast<z_moved_sample_t*>(x); }
static inline z_moved_session_t* z_session_move(z_owned_session_t* x) { return reinterpret_cast<z_moved_session_t*>(x); }
static inline z_moved_shared_shm_provider_t* z_shared_shm_provider_move(z_owned_shared_shm_provider_t* x) { return reinterpret_cast<z_moved_shared_shm_provider_t*>(x); }
static inline z_moved_shm_client_t* z_shm_client_move(z_owned_shm_client_t* x) { return reinterpret_cast<z_moved_shm_client_t*>(x); }
static inline z_moved_shm_client_storage_t* z_shm_client_storage_move(z_owned_shm_client_storage_t* x) { return reinterpret_cast<z_moved_shm_client_storage_t*>(x); }
static inline z_moved_shm_t* z_shm_move(z_owned_shm_t* x) { return reinterpret_cast<z_moved_shm_t*>(x); }
static inline z_moved_shm_mut_t* z_shm_mut_move(z_owned_shm_mut_t* x) { return reinterpret_cast<z_moved_shm_mut_t*>(x); }
static inline z_moved_shm_provider_t* z_shm_provider_move(z_owned_shm_provider_t* x) { return reinterpret_cast<z_moved_shm_provider_t*>(x); }
static inline z_moved_slice_t* z_slice_move(z_owned_slice_t* x) { return reinterpret_cast<z_moved_slice_t*>(x); }
static inline z_moved_string_array_t* z_string_array_move(z_owned_string_array_t* x) { return reinterpret_cast<z_moved_string_array_t*>(x); }
static inline z_moved_string_t* z_string_move(z_owned_string_t* x) { return reinterpret_cast<z_moved_string_t*>(x); }
static inline z_moved_subscriber_t* z_subscriber_move(z_owned_subscriber_t* x) { return reinterpret_cast<z_moved_subscriber_t*>(x); }
static inline z_moved_task_t* z_task_move(z_owned_task_t* x) { return reinterpret_cast<z_moved_task_t*>(x); }
static inline zc_moved_closure_log_t* zc_closure_log_move(zc_owned_closure_log_t* x) { return reinterpret_cast<zc_moved_closure_log_t*>(x); }
static inline zc_moved_concurrent_close_handle_t* zc_concurrent_close_handle_move(zc_owned_concurrent_close_handle_t* x) { return reinterpret_cast<zc_moved_concurrent_close_handle_t*>(x); }
static inline zc_moved_shm_client_list_t* zc_shm_client_list_move(zc_owned_shm_client_list_t* x) { return reinterpret_cast<zc_moved_shm_client_list_t*>(x); }
static inline ze_moved_advanced_publisher_t* ze_advanced_publisher_move(ze_owned_advanced_publisher_t* x) { return reinterpret_cast<ze_moved_advanced_publisher_t*>(x); }
static inline ze_moved_advanced_subscriber_t* ze_advanced_subscriber_move(ze_owned_advanced_subscriber_t* x) { return reinterpret_cast<ze_moved_advanced_subscriber_t*>(x); }
static inline ze_moved_closure_miss_t* ze_closure_miss_move(ze_owned_closure_miss_t* x) { return reinterpret_cast<ze_moved_closure_miss_t*>(x); }
static inline ze_moved_publication_cache_t* ze_publication_cache_move(ze_owned_publication_cache_t* x) { return reinterpret_cast<ze_moved_publication_cache_t*>(x); }
static inline ze_moved_querying_subscriber_t* ze_querying_subscriber_move(ze_owned_querying_subscriber_t* x) { return reinterpret_cast<ze_moved_querying_subscriber_t*>(x); }
static inline ze_moved_sample_miss_listener_t* ze_sample_miss_listener_move(ze_owned_sample_miss_listener_t* x) { return reinterpret_cast<ze_moved_sample_miss_listener_t*>(x); }
static inline ze_moved_serializer_t* ze_serializer_move(ze_owned_serializer_t* x) { return reinterpret_cast<ze_moved_serializer_t*>(x); }



inline const z_loaned_bytes_t* z_loan(const z_owned_bytes_t& this_) { return z_bytes_loan(&this_); };
inline const z_loaned_bytes_writer_t* z_loan(const z_owned_bytes_writer_t& this_) { return z_bytes_writer_loan(&this_); };
inline const z_loaned_cancellation_token_t* z_loan(const z_owned_cancellation_token_t& this_) { return z_cancellation_token_loan(&this_); };
inline const z_loaned_closure_hello_t* z_loan(const z_owned_closure_hello_t& closure) { return z_closure_hello_loan(&closure); };
inline const z_loaned_closure_matching_status_t* z_loan(const z_owned_closure_matching_status_t& closure) { return z_closure_matching_status_loan(&closure); };
inline const z_loaned_closure_query_t* z_loan(const z_owned_closure_query_t& closure) { return z_closure_query_loan(&closure); };
inline const z_loaned_closure_reply_t* z_loan(const z_owned_closure_reply_t& closure) { return z_closure_reply_loan(&closure); };
inline const z_loaned_closure_sample_t* z_loan(const z_owned_closure_sample_t& closure) { return z_closure_sample_loan(&closure); };
inline const z_loaned_closure_zid_t* z_loan(const z_owned_closure_zid_t& closure) { return z_closure_zid_loan(&closure); };
inline const z_loaned_condvar_t* z_loan(const z_owned_condvar_t& this_) { return z_condvar_loan(&this_); };
inline const z_loaned_config_t* z_loan(const z_owned_config_t& this_) { return z_config_loan(&this_); };
inline const z_loaned_encoding_t* z_loan(const z_owned_encoding_t& this_) { return z_encoding_loan(&this_); };
inline const z_loaned_fifo_handler_query_t* z_loan(const z_owned_fifo_handler_query_t& this_) { return z_fifo_handler_query_loan(&this_); };
inline const z_loaned_fifo_handler_reply_t* z_loan(const z_owned_fifo_handler_reply_t& this_) { return z_fifo_handler_reply_loan(&this_); };
inline const z_loaned_fifo_handler_sample_t* z_loan(const z_owned_fifo_handler_sample_t& this_) { return z_fifo_handler_sample_loan(&this_); };
inline const z_loaned_hello_t* z_loan(const z_owned_hello_t& this_) { return z_hello_loan(&this_); };
inline const z_loaned_keyexpr_t* z_loan(const z_owned_keyexpr_t& this_) { return z_keyexpr_loan(&this_); };
inline const z_loaned_liveliness_token_t* z_loan(const z_owned_liveliness_token_t& this_) { return z_liveliness_token_loan(&this_); };
inline const z_loaned_memory_layout_t* z_loan(const z_owned_memory_layout_t& this_) { return z_memory_layout_loan(&this_); };
inline const z_loaned_precomputed_layout_t* z_loan(const z_owned_precomputed_layout_t& this_) { return z_precomputed_layout_loan(&this_); };
inline const z_loaned_ptr_in_segment_t* z_loan(const z_owned_ptr_in_segment_t& this_) { return z_ptr_in_segment_loan(&this_); };
inline const z_loaned_publisher_t* z_loan(const z_owned_publisher_t& this_) { return z_publisher_loan(&this_); };
inline const z_loaned_querier_t* z_loan(const z_owned_querier_t& this_) { return z_querier_loan(&this_); };
inline const z_loaned_query_t* z_loan(const z_owned_query_t& this_) { return z_query_loan(&this_); };
inline const z_loaned_queryable_t* z_loan(const z_owned_queryable_t& this_) { return z_queryable_loan(&this_); };
inline const z_loaned_reply_err_t* z_loan(const z_owned_reply_err_t& this_) { return z_reply_err_loan(&this_); };
inline const z_loaned_reply_t* z_loan(const z_owned_reply_t& this_) { return z_reply_loan(&this_); };
inline const z_loaned_ring_handler_query_t* z_loan(const z_owned_ring_handler_query_t& this_) { return z_ring_handler_query_loan(&this_); };
inline const z_loaned_ring_handler_reply_t* z_loan(const z_owned_ring_handler_reply_t& this_) { return z_ring_handler_reply_loan(&this_); };
inline const z_loaned_ring_handler_sample_t* z_loan(const z_owned_ring_handler_sample_t& this_) { return z_ring_handler_sample_loan(&this_); };
inline const z_loaned_sample_t* z_loan(const z_owned_sample_t& this_) { return z_sample_loan(&this_); };
inline const z_loaned_session_t* z_loan(const z_owned_session_t& this_) { return z_session_loan(&this_); };
inline const z_loaned_shared_shm_provider_t* z_loan(const z_owned_shared_shm_provider_t& this_) { return z_shared_shm_provider_loan(&this_); };
inline const z_loaned_shm_client_storage_t* z_loan(const z_owned_shm_client_storage_t& this_) { return z_shm_client_storage_loan(&this_); };
inline const z_loaned_shm_t* z_loan(const z_owned_shm_t& this_) { return z_shm_loan(&this_); };
inline const z_loaned_shm_mut_t* z_loan(const z_owned_shm_mut_t& this_) { return z_shm_mut_loan(&this_); };
inline const z_loaned_shm_provider_t* z_loan(const z_owned_shm_provider_t& this_) { return z_shm_provider_loan(&this_); };
inline const z_loaned_slice_t* z_loan(const z_owned_slice_t& this_) { return z_slice_loan(&this_); };
inline const z_loaned_string_array_t* z_loan(const z_owned_string_array_t& this_) { return z_string_array_loan(&this_); };
inline const z_loaned_string_t* z_loan(const z_owned_string_t& this_) { return z_string_loan(&this_); };
inline const z_loaned_subscriber_t* z_loan(const z_owned_subscriber_t& this_) { return z_subscriber_loan(&this_); };
inline const z_loaned_keyexpr_t* z_loan(const z_view_keyexpr_t& this_) { return z_view_keyexpr_loan(&this_); };
inline const z_loaned_slice_t* z_loan(const z_view_slice_t& this_) { return z_view_slice_loan(&this_); };
inline const z_loaned_string_t* z_loan(const z_view_string_t& this_) { return z_view_string_loan(&this_); };
inline const zc_loaned_closure_log_t* z_loan(const zc_owned_closure_log_t& closure) { return zc_closure_log_loan(&closure); };
inline const zc_loaned_shm_client_list_t* z_loan(const zc_owned_shm_client_list_t& this_) { return zc_shm_client_list_loan(&this_); };
inline const ze_loaned_advanced_publisher_t* z_loan(const ze_owned_advanced_publisher_t& this_) { return ze_advanced_publisher_loan(&this_); };
inline const ze_loaned_advanced_subscriber_t* z_loan(const ze_owned_advanced_subscriber_t& this_) { return ze_advanced_subscriber_loan(&this_); };
inline const ze_loaned_closure_miss_t* z_loan(const ze_owned_closure_miss_t& closure) { return ze_closure_miss_loan(&closure); };
inline const ze_loaned_publication_cache_t* z_loan(const ze_owned_publication_cache_t& this_) { return ze_publication_cache_loan(&this_); };
inline const ze_loaned_querying_subscriber_t* z_loan(const ze_owned_querying_subscriber_t& this_) { return ze_querying_subscriber_loan(&this_); };
inline const ze_loaned_serializer_t* z_loan(const ze_owned_serializer_t& this_) { return ze_serializer_loan(&this_); };


inline z_loaned_bytes_t* z_loan_mut(z_owned_bytes_t& this_) { return z_bytes_loan_mut(&this_); };
inline z_loaned_bytes_writer_t* z_loan_mut(z_owned_bytes_writer_t& this_) { return z_bytes_writer_loan_mut(&this_); };
inline z_loaned_cancellation_token_t* z_loan_mut(z_owned_cancellation_token_t& this_) { return z_cancellation_token_loan_mut(&this_); };
inline z_loaned_closure_hello_t* z_loan_mut(z_owned_closure_hello_t& closure) { return z_closure_hello_loan_mut(&closure); };
inline z_loaned_closure_query_t* z_loan_mut(z_owned_closure_query_t& closure) { return z_closure_query_loan_mut(&closure); };
inline z_loaned_closure_reply_t* z_loan_mut(z_owned_closure_reply_t& closure) { return z_closure_reply_loan_mut(&closure); };
inline z_loaned_closure_sample_t* z_loan_mut(z_owned_closure_sample_t& closure) { return z_closure_sample_loan_mut(&closure); };
inline z_loaned_condvar_t* z_loan_mut(z_owned_condvar_t& this_) { return z_condvar_loan_mut(&this_); };
inline z_loaned_config_t* z_loan_mut(z_owned_config_t& this_) { return z_config_loan_mut(&this_); };
inline z_loaned_encoding_t* z_loan_mut(z_owned_encoding_t& this_) { return z_encoding_loan_mut(&this_); };
inline z_loaned_hello_t* z_loan_mut(z_owned_hello_t& this_) { return z_hello_loan_mut(&this_); };
inline z_loaned_mutex_t* z_loan_mut(z_owned_mutex_t& this_) { return z_mutex_loan_mut(&this_); };
inline z_loaned_publisher_t* z_loan_mut(z_owned_publisher_t& this_) { return z_publisher_loan_mut(&this_); };
inline z_loaned_querier_t* z_loan_mut(z_owned_querier_t& this_) { return z_querier_loan_mut(&this_); };
inline z_loaned_query_t* z_loan_mut(z_owned_query_t& this_) { return z_query_loan_mut(&this_); };
inline z_loaned_reply_err_t* z_loan_mut(z_owned_reply_err_t& this_) { return z_reply_err_loan_mut(&this_); };
inline z_loaned_reply_t* z_loan_mut(z_owned_reply_t& this_) { return z_reply_loan_mut(&this_); };
inline z_loaned_sample_t* z_loan_mut(z_owned_sample_t& this_) { return z_sample_loan_mut(&this_); };
inline z_loaned_session_t* z_loan_mut(z_owned_session_t& this_) { return z_session_loan_mut(&this_); };
inline z_loaned_shm_t* z_loan_mut(z_owned_shm_t& this_) { return z_shm_loan_mut(&this_); };
inline z_loaned_shm_mut_t* z_loan_mut(z_owned_shm_mut_t& this_) { return z_shm_mut_loan_mut(&this_); };
inline z_loaned_string_array_t* z_loan_mut(z_owned_string_array_t& this_) { return z_string_array_loan_mut(&this_); };
inline zc_loaned_shm_client_list_t* z_loan_mut(zc_owned_shm_client_list_t& this_) { return zc_shm_client_list_loan_mut(&this_); };
inline ze_loaned_advanced_publisher_t* z_loan_mut(ze_owned_advanced_publisher_t& this_) { return ze_advanced_publisher_loan_mut(&this_); };
inline ze_loaned_serializer_t* z_loan_mut(ze_owned_serializer_t& this_) { return ze_serializer_loan_mut(&this_); };


inline void z_drop(z_moved_bytes_t* this_) { z_bytes_drop(this_); };
inline void z_drop(z_moved_bytes_writer_t* this_) { z_bytes_writer_drop(this_); };
inline void z_drop(z_moved_cancellation_token_t* this_) { z_cancellation_token_drop(this_); };
inline void z_drop(z_moved_chunk_alloc_result_t* this_) { z_chunk_alloc_result_drop(this_); };
inline void z_drop(z_moved_closure_hello_t* this_) { z_closure_hello_drop(this_); };
inline void z_drop(z_moved_closure_matching_status_t* closure_) { z_closure_matching_status_drop(closure_); };
inline void z_drop(z_moved_closure_query_t* closure_) { z_closure_query_drop(closure_); };
inline void z_drop(z_moved_closure_reply_t* closure_) { z_closure_reply_drop(closure_); };
inline void z_drop(z_moved_closure_sample_t* closure_) { z_closure_sample_drop(closure_); };
inline void z_drop(z_moved_closure_zid_t* closure_) { z_closure_zid_drop(closure_); };
inline void z_drop(z_moved_condvar_t* this_) { z_condvar_drop(this_); };
inline void z_drop(z_moved_config_t* this_) { z_config_drop(this_); };
inline void z_drop(z_moved_encoding_t* this_) { z_encoding_drop(this_); };
inline void z_drop(z_moved_fifo_handler_query_t* this_) { z_fifo_handler_query_drop(this_); };
inline void z_drop(z_moved_fifo_handler_reply_t* this_) { z_fifo_handler_reply_drop(this_); };
inline void z_drop(z_moved_fifo_handler_sample_t* this_) { z_fifo_handler_sample_drop(this_); };
inline void z_drop(z_moved_hello_t* this_) { z_hello_drop(this_); };
inline void z_drop(z_moved_keyexpr_t* this_) { z_keyexpr_drop(this_); };
inline void z_drop(z_moved_liveliness_token_t* this_) { z_liveliness_token_drop(this_); };
inline void z_drop(z_moved_matching_listener_t* this_) { z_matching_listener_drop(this_); };
inline void z_drop(z_moved_memory_layout_t* this_) { z_memory_layout_drop(this_); };
inline void z_drop(z_moved_mutex_t* this_) { z_mutex_drop(this_); };
inline void z_drop(z_moved_precomputed_layout_t* this_) { z_precomputed_layout_drop(this_); };
inline void z_drop(z_moved_ptr_in_segment_t* this_) { z_ptr_in_segment_drop(this_); };
inline void z_drop(z_moved_publisher_t* this_) { z_publisher_drop(this_); };
inline void z_drop(z_moved_querier_t* this_) { z_querier_drop(this_); };
inline void z_drop(z_moved_query_t* this_) { z_query_drop(this_); };
inline void z_drop(z_moved_queryable_t* this_) { z_queryable_drop(this_); };
inline void z_drop(z_moved_reply_t* this_) { z_reply_drop(this_); };
inline void z_drop(z_moved_reply_err_t* this_) { z_reply_err_drop(this_); };
inline void z_drop(z_moved_ring_handler_query_t* this_) { z_ring_handler_query_drop(this_); };
inline void z_drop(z_moved_ring_handler_reply_t* this_) { z_ring_handler_reply_drop(this_); };
inline void z_drop(z_moved_ring_handler_sample_t* this_) { z_ring_handler_sample_drop(this_); };
inline void z_drop(z_moved_sample_t* this_) { z_sample_drop(this_); };
inline void z_drop(z_moved_session_t* this_) { z_session_drop(this_); };
inline void z_drop(z_moved_shared_shm_provider_t* this_) { z_shared_shm_provider_drop(this_); };
inline void z_drop(z_moved_shm_client_t* this_) { z_shm_client_drop(this_); };
inline void z_drop(z_moved_shm_client_storage_t* this_) { z_shm_client_storage_drop(this_); };
inline void z_drop(z_moved_shm_t* this_) { z_shm_drop(this_); };
inline void z_drop(z_moved_shm_mut_t* this_) { z_shm_mut_drop(this_); };
inline void z_drop(z_moved_shm_provider_t* this_) { z_shm_provider_drop(this_); };
inline void z_drop(z_moved_slice_t* this_) { z_slice_drop(this_); };
inline void z_drop(z_moved_string_array_t* this_) { z_string_array_drop(this_); };
inline void z_drop(z_moved_string_t* this_) { z_string_drop(this_); };
inline void z_drop(z_moved_subscriber_t* this_) { z_subscriber_drop(this_); };
inline void z_drop(z_moved_task_t* this_) { z_task_drop(this_); };
inline void z_drop(zc_moved_closure_log_t* closure_) { zc_closure_log_drop(closure_); };
inline void z_drop(zc_moved_concurrent_close_handle_t* this_) { zc_concurrent_close_handle_drop(this_); };
inline void z_drop(zc_moved_shm_client_list_t* this_) { zc_shm_client_list_drop(this_); };
inline void z_drop(ze_moved_advanced_publisher_t* this_) { ze_advanced_publisher_drop(this_); };
inline void z_drop(ze_moved_advanced_subscriber_t* this_) { ze_advanced_subscriber_drop(this_); };
inline void z_drop(ze_moved_closure_miss_t* closure_) { ze_closure_miss_drop(closure_); };
inline void z_drop(ze_moved_publication_cache_t* this_) { ze_publication_cache_drop(this_); };
inline void z_drop(ze_moved_querying_subscriber_t* this_) { ze_querying_subscriber_drop(this_); };
inline void z_drop(ze_moved_sample_miss_listener_t* this_) { ze_sample_miss_listener_drop(this_); };
inline void z_drop(ze_moved_serializer_t* this_) { ze_serializer_drop(this_); };


inline z_moved_bytes_t* z_move(z_owned_bytes_t& this_) { return z_bytes_move(&this_); };
inline z_moved_bytes_writer_t* z_move(z_owned_bytes_writer_t& this_) { return z_bytes_writer_move(&this_); };
inline z_moved_cancellation_token_t* z_move(z_owned_cancellation_token_t& this_) { return z_cancellation_token_move(&this_); };
inline z_moved_chunk_alloc_result_t* z_move(z_owned_chunk_alloc_result_t& this_) { return z_chunk_alloc_result_move(&this_); };
inline z_moved_closure_hello_t* z_move(z_owned_closure_hello_t& this_) { return z_closure_hello_move(&this_); };
inline z_moved_closure_matching_status_t* z_move(z_owned_closure_matching_status_t& closure_) { return z_closure_matching_status_move(&closure_); };
inline z_moved_closure_query_t* z_move(z_owned_closure_query_t& closure_) { return z_closure_query_move(&closure_); };
inline z_moved_closure_reply_t* z_move(z_owned_closure_reply_t& closure_) { return z_closure_reply_move(&closure_); };
inline z_moved_closure_sample_t* z_move(z_owned_closure_sample_t& closure_) { return z_closure_sample_move(&closure_); };
inline z_moved_closure_zid_t* z_move(z_owned_closure_zid_t& closure_) { return z_closure_zid_move(&closure_); };
inline z_moved_condvar_t* z_move(z_owned_condvar_t& this_) { return z_condvar_move(&this_); };
inline z_moved_config_t* z_move(z_owned_config_t& this_) { return z_config_move(&this_); };
inline z_moved_encoding_t* z_move(z_owned_encoding_t& this_) { return z_encoding_move(&this_); };
inline z_moved_fifo_handler_query_t* z_move(z_owned_fifo_handler_query_t& this_) { return z_fifo_handler_query_move(&this_); };
inline z_moved_fifo_handler_reply_t* z_move(z_owned_fifo_handler_reply_t& this_) { return z_fifo_handler_reply_move(&this_); };
inline z_moved_fifo_handler_sample_t* z_move(z_owned_fifo_handler_sample_t& this_) { return z_fifo_handler_sample_move(&this_); };
inline z_moved_hello_t* z_move(z_owned_hello_t& this_) { return z_hello_move(&this_); };
inline z_moved_keyexpr_t* z_move(z_owned_keyexpr_t& this_) { return z_keyexpr_move(&this_); };
inline z_moved_liveliness_token_t* z_move(z_owned_liveliness_token_t& this_) { return z_liveliness_token_move(&this_); };
inline z_moved_matching_listener_t* z_move(z_owned_matching_listener_t& this_) { return z_matching_listener_move(&this_); };
inline z_moved_memory_layout_t* z_move(z_owned_memory_layout_t& this_) { return z_memory_layout_move(&this_); };
inline z_moved_mutex_t* z_move(z_owned_mutex_t& this_) { return z_mutex_move(&this_); };
inline z_moved_precomputed_layout_t* z_move(z_owned_precomputed_layout_t& this_) { return z_precomputed_layout_move(&this_); };
inline z_moved_ptr_in_segment_t* z_move(z_owned_ptr_in_segment_t& this_) { return z_ptr_in_segment_move(&this_); };
inline z_moved_publisher_t* z_move(z_owned_publisher_t& this_) { return z_publisher_move(&this_); };
inline z_moved_querier_t* z_move(z_owned_querier_t& this_) { return z_querier_move(&this_); };
inline z_moved_query_t* z_move(z_owned_query_t& this_) { return z_query_move(&this_); };
inline z_moved_queryable_t* z_move(z_owned_queryable_t& this_) { return z_queryable_move(&this_); };
inline z_moved_reply_t* z_move(z_owned_reply_t& this_) { return z_reply_move(&this_); };
inline z_moved_reply_err_t* z_move(z_owned_reply_err_t& this_) { return z_reply_err_move(&this_); };
inline z_moved_ring_handler_query_t* z_move(z_owned_ring_handler_query_t& this_) { return z_ring_handler_query_move(&this_); };
inline z_moved_ring_handler_reply_t* z_move(z_owned_ring_handler_reply_t& this_) { return z_ring_handler_reply_move(&this_); };
inline z_moved_ring_handler_sample_t* z_move(z_owned_ring_handler_sample_t& this_) { return z_ring_handler_sample_move(&this_); };
inline z_moved_sample_t* z_move(z_owned_sample_t& this_) { return z_sample_move(&this_); };
inline z_moved_session_t* z_move(z_owned_session_t& this_) { return z_session_move(&this_); };
inline z_moved_shared_shm_provider_t* z_move(z_owned_shared_shm_provider_t& this_) { return z_shared_shm_provider_move(&this_); };
inline z_moved_shm_client_t* z_move(z_owned_shm_client_t& this_) { return z_shm_client_move(&this_); };
inline z_moved_shm_client_storage_t* z_move(z_owned_shm_client_storage_t& this_) { return z_shm_client_storage_move(&this_); };
inline z_moved_shm_t* z_move(z_owned_shm_t& this_) { return z_shm_move(&this_); };
inline z_moved_shm_mut_t* z_move(z_owned_shm_mut_t& this_) { return z_shm_mut_move(&this_); };
inline z_moved_shm_provider_t* z_move(z_owned_shm_provider_t& this_) { return z_shm_provider_move(&this_); };
inline z_moved_slice_t* z_move(z_owned_slice_t& this_) { return z_slice_move(&this_); };
inline z_moved_string_array_t* z_move(z_owned_string_array_t& this_) { return z_string_array_move(&this_); };
inline z_moved_string_t* z_move(z_owned_string_t& this_) { return z_string_move(&this_); };
inline z_moved_subscriber_t* z_move(z_owned_subscriber_t& this_) { return z_subscriber_move(&this_); };
inline z_moved_task_t* z_move(z_owned_task_t& this_) { return z_task_move(&this_); };
inline zc_moved_closure_log_t* z_move(zc_owned_closure_log_t& closure_) { return zc_closure_log_move(&closure_); };
inline zc_moved_concurrent_close_handle_t* z_move(zc_owned_concurrent_close_handle_t& this_) { return zc_concurrent_close_handle_move(&this_); };
inline zc_moved_shm_client_list_t* z_move(zc_owned_shm_client_list_t& this_) { return zc_shm_client_list_move(&this_); };
inline ze_moved_advanced_publisher_t* z_move(ze_owned_advanced_publisher_t& this_) { return ze_advanced_publisher_move(&this_); };
inline ze_moved_advanced_subscriber_t* z_move(ze_owned_advanced_subscriber_t& this_) { return ze_advanced_subscriber_move(&this_); };
inline ze_moved_closure_miss_t* z_move(ze_owned_closure_miss_t& closure_) { return ze_closure_miss_move(&closure_); };
inline ze_moved_publication_cache_t* z_move(ze_owned_publication_cache_t& this_) { return ze_publication_cache_move(&this_); };
inline ze_moved_querying_subscriber_t* z_move(ze_owned_querying_subscriber_t& this_) { return ze_querying_subscriber_move(&this_); };
inline ze_moved_sample_miss_listener_t* z_move(ze_owned_sample_miss_listener_t& this_) { return ze_sample_miss_listener_move(&this_); };
inline ze_moved_serializer_t* z_move(ze_owned_serializer_t& this_) { return ze_serializer_move(&this_); };


inline void z_internal_null(z_owned_bytes_t* this_) { z_internal_bytes_null(this_); };
inline void z_internal_null(z_owned_bytes_writer_t* this_) { z_internal_bytes_writer_null(this_); };
inline void z_internal_null(z_owned_cancellation_token_t* this_) { z_internal_cancellation_token_null(this_); };
inline void z_internal_null(z_owned_chunk_alloc_result_t* this_) { z_internal_chunk_alloc_result_null(this_); };
inline void z_internal_null(z_owned_closure_hello_t* this_) { z_internal_closure_hello_null(this_); };
inline void z_internal_null(z_owned_closure_matching_status_t* this_) { z_internal_closure_matching_status_null(this_); };
inline void z_internal_null(z_owned_closure_query_t* this_) { z_internal_closure_query_null(this_); };
inline void z_internal_null(z_owned_closure_reply_t* this_) { z_internal_closure_reply_null(this_); };
inline void z_internal_null(z_owned_closure_sample_t* this_) { z_internal_closure_sample_null(this_); };
inline void z_internal_null(z_owned_closure_zid_t* this_) { z_internal_closure_zid_null(this_); };
inline void z_internal_null(z_owned_condvar_t* this_) { z_internal_condvar_null(this_); };
inline void z_internal_null(z_owned_config_t* this_) { z_internal_config_null(this_); };
inline void z_internal_null(z_owned_encoding_t* this_) { z_internal_encoding_null(this_); };
inline void z_internal_null(z_owned_fifo_handler_query_t* this_) { z_internal_fifo_handler_query_null(this_); };
inline void z_internal_null(z_owned_fifo_handler_reply_t* this_) { z_internal_fifo_handler_reply_null(this_); };
inline void z_internal_null(z_owned_fifo_handler_sample_t* this_) { z_internal_fifo_handler_sample_null(this_); };
inline void z_internal_null(z_owned_hello_t* this_) { z_internal_hello_null(this_); };
inline void z_internal_null(z_owned_keyexpr_t* this_) { z_internal_keyexpr_null(this_); };
inline void z_internal_null(z_owned_liveliness_token_t* this_) { z_internal_liveliness_token_null(this_); };
inline void z_internal_null(z_owned_matching_listener_t* this_) { z_internal_matching_listener_null(this_); };
inline void z_internal_null(z_owned_memory_layout_t* this_) { z_internal_memory_layout_null(this_); };
inline void z_internal_null(z_owned_mutex_t* this_) { z_internal_mutex_null(this_); };
inline void z_internal_null(z_owned_precomputed_layout_t* this_) { z_internal_precomputed_layout_null(this_); };
inline void z_internal_null(z_owned_ptr_in_segment_t* this_) { z_internal_ptr_in_segment_null(this_); };
inline void z_internal_null(z_owned_publisher_t* this_) { z_internal_publisher_null(this_); };
inline void z_internal_null(z_owned_querier_t* this_) { z_internal_querier_null(this_); };
inline void z_internal_null(z_owned_query_t* this_) { z_internal_query_null(this_); };
inline void z_internal_null(z_owned_queryable_t* this_) { z_internal_queryable_null(this_); };
inline void z_internal_null(z_owned_reply_err_t* this_) { z_internal_reply_err_null(this_); };
inline void z_internal_null(z_owned_reply_t* this_) { z_internal_reply_null(this_); };
inline void z_internal_null(z_owned_ring_handler_query_t* this_) { z_internal_ring_handler_query_null(this_); };
inline void z_internal_null(z_owned_ring_handler_reply_t* this_) { z_internal_ring_handler_reply_null(this_); };
inline void z_internal_null(z_owned_ring_handler_sample_t* this_) { z_internal_ring_handler_sample_null(this_); };
inline void z_internal_null(z_owned_sample_t* this_) { z_internal_sample_null(this_); };
inline void z_internal_null(z_owned_session_t* this_) { z_internal_session_null(this_); };
inline void z_internal_null(z_owned_shared_shm_provider_t* this_) { z_internal_shared_shm_provider_null(this_); };
inline void z_internal_null(z_owned_shm_client_t* this_) { z_internal_shm_client_null(this_); };
inline void z_internal_null(z_owned_shm_client_storage_t* this_) { z_internal_shm_client_storage_null(this_); };
inline void z_internal_null(z_owned_shm_mut_t* this_) { z_internal_shm_mut_null(this_); };
inline void z_internal_null(z_owned_shm_t* this_) { z_internal_shm_null(this_); };
inline void z_internal_null(z_owned_shm_provider_t* this_) { z_internal_shm_provider_null(this_); };
inline void z_internal_null(z_owned_slice_t* this_) { z_internal_slice_null(this_); };
inline void z_internal_null(z_owned_string_array_t* this_) { z_internal_string_array_null(this_); };
inline void z_internal_null(z_owned_string_t* this_) { z_internal_string_null(this_); };
inline void z_internal_null(z_owned_subscriber_t* this_) { z_internal_subscriber_null(this_); };
inline void z_internal_null(z_owned_task_t* this_) { z_internal_task_null(this_); };
inline void z_internal_null(zc_owned_closure_log_t* this_) { zc_internal_closure_log_null(this_); };
inline void z_internal_null(zc_owned_concurrent_close_handle_t* this_) { zc_internal_concurrent_close_handle_null(this_); };
inline void z_internal_null(zc_owned_shm_client_list_t* this_) { zc_internal_shm_client_list_null(this_); };
inline void z_internal_null(ze_owned_advanced_publisher_t* this_) { ze_internal_advanced_publisher_null(this_); };
inline void z_internal_null(ze_owned_advanced_subscriber_t* this_) { ze_internal_advanced_subscriber_null(this_); };
inline void z_internal_null(ze_owned_closure_miss_t* this_) { ze_internal_closure_miss_null(this_); };
inline void z_internal_null(ze_owned_publication_cache_t* this_) { ze_internal_publication_cache_null(this_); };
inline void z_internal_null(ze_owned_querying_subscriber_t* this_) { ze_internal_querying_subscriber_null(this_); };
inline void z_internal_null(ze_owned_sample_miss_listener_t* this_) { ze_internal_sample_miss_listener_null(this_); };
inline void z_internal_null(ze_owned_serializer_t* this_) { ze_internal_serializer_null(this_); };

static inline void z_bytes_take(z_owned_bytes_t* this_, z_moved_bytes_t* x) { *this_ = x->_this; z_internal_bytes_null(&x->_this); }
static inline void z_bytes_writer_take(z_owned_bytes_writer_t* this_, z_moved_bytes_writer_t* x) { *this_ = x->_this; z_internal_bytes_writer_null(&x->_this); }
static inline void z_cancellation_token_take(z_owned_cancellation_token_t* this_, z_moved_cancellation_token_t* x) { *this_ = x->_this; z_internal_cancellation_token_null(&x->_this); }
static inline void z_chunk_alloc_result_take(z_owned_chunk_alloc_result_t* this_, z_moved_chunk_alloc_result_t* x) { *this_ = x->_this; z_internal_chunk_alloc_result_null(&x->_this); }
static inline void z_closure_hello_take(z_owned_closure_hello_t* this_, z_moved_closure_hello_t* x) { *this_ = x->_this; z_internal_closure_hello_null(&x->_this); }
static inline void z_closure_matching_status_take(z_owned_closure_matching_status_t* closure_, z_moved_closure_matching_status_t* x) { *closure_ = x->_this; z_internal_closure_matching_status_null(&x->_this); }
static inline void z_closure_query_take(z_owned_closure_query_t* closure_, z_moved_closure_query_t* x) { *closure_ = x->_this; z_internal_closure_query_null(&x->_this); }
static inline void z_closure_reply_take(z_owned_closure_reply_t* closure_, z_moved_closure_reply_t* x) { *closure_ = x->_this; z_internal_closure_reply_null(&x->_this); }
static inline void z_closure_sample_take(z_owned_closure_sample_t* closure_, z_moved_closure_sample_t* x) { *closure_ = x->_this; z_internal_closure_sample_null(&x->_this); }
static inline void z_closure_zid_take(z_owned_closure_zid_t* closure_, z_moved_closure_zid_t* x) { *closure_ = x->_this; z_internal_closure_zid_null(&x->_this); }
static inline void z_condvar_take(z_owned_condvar_t* this_, z_moved_condvar_t* x) { *this_ = x->_this; z_internal_condvar_null(&x->_this); }
static inline void z_config_take(z_owned_config_t* this_, z_moved_config_t* x) { *this_ = x->_this; z_internal_config_null(&x->_this); }
static inline void z_encoding_take(z_owned_encoding_t* this_, z_moved_encoding_t* x) { *this_ = x->_this; z_internal_encoding_null(&x->_this); }
static inline void z_fifo_handler_query_take(z_owned_fifo_handler_query_t* this_, z_moved_fifo_handler_query_t* x) { *this_ = x->_this; z_internal_fifo_handler_query_null(&x->_this); }
static inline void z_fifo_handler_reply_take(z_owned_fifo_handler_reply_t* this_, z_moved_fifo_handler_reply_t* x) { *this_ = x->_this; z_internal_fifo_handler_reply_null(&x->_this); }
static inline void z_fifo_handler_sample_take(z_owned_fifo_handler_sample_t* this_, z_moved_fifo_handler_sample_t* x) { *this_ = x->_this; z_internal_fifo_handler_sample_null(&x->_this); }
static inline void z_hello_take(z_owned_hello_t* this_, z_moved_hello_t* x) { *this_ = x->_this; z_internal_hello_null(&x->_this); }
static inline void z_keyexpr_take(z_owned_keyexpr_t* this_, z_moved_keyexpr_t* x) { *this_ = x->_this; z_internal_keyexpr_null(&x->_this); }
static inline void z_liveliness_token_take(z_owned_liveliness_token_t* this_, z_moved_liveliness_token_t* x) { *this_ = x->_this; z_internal_liveliness_token_null(&x->_this); }
static inline void z_matching_listener_take(z_owned_matching_listener_t* this_, z_moved_matching_listener_t* x) { *this_ = x->_this; z_internal_matching_listener_null(&x->_this); }
static inline void z_memory_layout_take(z_owned_memory_layout_t* this_, z_moved_memory_layout_t* x) { *this_ = x->_this; z_internal_memory_layout_null(&x->_this); }
static inline void z_mutex_take(z_owned_mutex_t* this_, z_moved_mutex_t* x) { *this_ = x->_this; z_internal_mutex_null(&x->_this); }
static inline void z_precomputed_layout_take(z_owned_precomputed_layout_t* this_, z_moved_precomputed_layout_t* x) { *this_ = x->_this; z_internal_precomputed_layout_null(&x->_this); }
static inline void z_ptr_in_segment_take(z_owned_ptr_in_segment_t* this_, z_moved_ptr_in_segment_t* x) { *this_ = x->_this; z_internal_ptr_in_segment_null(&x->_this); }
static inline void z_publisher_take(z_owned_publisher_t* this_, z_moved_publisher_t* x) { *this_ = x->_this; z_internal_publisher_null(&x->_this); }
static inline void z_querier_take(z_owned_querier_t* this_, z_moved_querier_t* x) { *this_ = x->_this; z_internal_querier_null(&x->_this); }
static inline void z_query_take(z_owned_query_t* this_, z_moved_query_t* x) { *this_ = x->_this; z_internal_query_null(&x->_this); }
static inline void z_queryable_take(z_owned_queryable_t* this_, z_moved_queryable_t* x) { *this_ = x->_this; z_internal_queryable_null(&x->_this); }
static inline void z_reply_take(z_owned_reply_t* this_, z_moved_reply_t* x) { *this_ = x->_this; z_internal_reply_null(&x->_this); }
static inline void z_reply_err_take(z_owned_reply_err_t* this_, z_moved_reply_err_t* x) { *this_ = x->_this; z_internal_reply_err_null(&x->_this); }
static inline void z_ring_handler_query_take(z_owned_ring_handler_query_t* this_, z_moved_ring_handler_query_t* x) { *this_ = x->_this; z_internal_ring_handler_query_null(&x->_this); }
static inline void z_ring_handler_reply_take(z_owned_ring_handler_reply_t* this_, z_moved_ring_handler_reply_t* x) { *this_ = x->_this; z_internal_ring_handler_reply_null(&x->_this); }
static inline void z_ring_handler_sample_take(z_owned_ring_handler_sample_t* this_, z_moved_ring_handler_sample_t* x) { *this_ = x->_this; z_internal_ring_handler_sample_null(&x->_this); }
static inline void z_sample_take(z_owned_sample_t* this_, z_moved_sample_t* x) { *this_ = x->_this; z_internal_sample_null(&x->_this); }
static inline void z_session_take(z_owned_session_t* this_, z_moved_session_t* x) { *this_ = x->_this; z_internal_session_null(&x->_this); }
static inline void z_shared_shm_provider_take(z_owned_shared_shm_provider_t* this_, z_moved_shared_shm_provider_t* x) { *this_ = x->_this; z_internal_shared_shm_provider_null(&x->_this); }
static inline void z_shm_client_take(z_owned_shm_client_t* this_, z_moved_shm_client_t* x) { *this_ = x->_this; z_internal_shm_client_null(&x->_this); }
static inline void z_shm_client_storage_take(z_owned_shm_client_storage_t* this_, z_moved_shm_client_storage_t* x) { *this_ = x->_this; z_internal_shm_client_storage_null(&x->_this); }
static inline void z_shm_take(z_owned_shm_t* this_, z_moved_shm_t* x) { *this_ = x->_this; z_internal_shm_null(&x->_this); }
static inline void z_shm_mut_take(z_owned_shm_mut_t* this_, z_moved_shm_mut_t* x) { *this_ = x->_this; z_internal_shm_mut_null(&x->_this); }
static inline void z_shm_provider_take(z_owned_shm_provider_t* this_, z_moved_shm_provider_t* x) { *this_ = x->_this; z_internal_shm_provider_null(&x->_this); }
static inline void z_slice_take(z_owned_slice_t* this_, z_moved_slice_t* x) { *this_ = x->_this; z_internal_slice_null(&x->_this); }
static inline void z_string_array_take(z_owned_string_array_t* this_, z_moved_string_array_t* x) { *this_ = x->_this; z_internal_string_array_null(&x->_this); }
static inline void z_string_take(z_owned_string_t* this_, z_moved_string_t* x) { *this_ = x->_this; z_internal_string_null(&x->_this); }
static inline void z_subscriber_take(z_owned_subscriber_t* this_, z_moved_subscriber_t* x) { *this_ = x->_this; z_internal_subscriber_null(&x->_this); }
static inline void z_task_take(z_owned_task_t* this_, z_moved_task_t* x) { *this_ = x->_this; z_internal_task_null(&x->_this); }
static inline void zc_closure_log_take(zc_owned_closure_log_t* closure_, zc_moved_closure_log_t* x) { *closure_ = x->_this; zc_internal_closure_log_null(&x->_this); }
static inline void zc_concurrent_close_handle_take(zc_owned_concurrent_close_handle_t* this_, zc_moved_concurrent_close_handle_t* x) { *this_ = x->_this; zc_internal_concurrent_close_handle_null(&x->_this); }
static inline void zc_shm_client_list_take(zc_owned_shm_client_list_t* this_, zc_moved_shm_client_list_t* x) { *this_ = x->_this; zc_internal_shm_client_list_null(&x->_this); }
static inline void ze_advanced_publisher_take(ze_owned_advanced_publisher_t* this_, ze_moved_advanced_publisher_t* x) { *this_ = x->_this; ze_internal_advanced_publisher_null(&x->_this); }
static inline void ze_advanced_subscriber_take(ze_owned_advanced_subscriber_t* this_, ze_moved_advanced_subscriber_t* x) { *this_ = x->_this; ze_internal_advanced_subscriber_null(&x->_this); }
static inline void ze_closure_miss_take(ze_owned_closure_miss_t* closure_, ze_moved_closure_miss_t* x) { *closure_ = x->_this; ze_internal_closure_miss_null(&x->_this); }
static inline void ze_publication_cache_take(ze_owned_publication_cache_t* this_, ze_moved_publication_cache_t* x) { *this_ = x->_this; ze_internal_publication_cache_null(&x->_this); }
static inline void ze_querying_subscriber_take(ze_owned_querying_subscriber_t* this_, ze_moved_querying_subscriber_t* x) { *this_ = x->_this; ze_internal_querying_subscriber_null(&x->_this); }
static inline void ze_sample_miss_listener_take(ze_owned_sample_miss_listener_t* this_, ze_moved_sample_miss_listener_t* x) { *this_ = x->_this; ze_internal_sample_miss_listener_null(&x->_this); }
static inline void ze_serializer_take(ze_owned_serializer_t* this_, ze_moved_serializer_t* x) { *this_ = x->_this; ze_internal_serializer_null(&x->_this); }



inline void z_take(z_owned_bytes_t* this_, z_moved_bytes_t* x) {
    z_bytes_take(this_, x);
};
inline void z_take(z_owned_bytes_writer_t* this_, z_moved_bytes_writer_t* x) {
    z_bytes_writer_take(this_, x);
};
inline void z_take(z_owned_cancellation_token_t* this_, z_moved_cancellation_token_t* x) {
    z_cancellation_token_take(this_, x);
};
inline void z_take(z_owned_chunk_alloc_result_t* this_, z_moved_chunk_alloc_result_t* x) {
    z_chunk_alloc_result_take(this_, x);
};
inline void z_take(z_owned_closure_hello_t* this_, z_moved_closure_hello_t* x) {
    z_closure_hello_take(this_, x);
};
inline void z_take(z_owned_closure_matching_status_t* closure_, z_moved_closure_matching_status_t* x) {
    z_closure_matching_status_take(closure_, x);
};
inline void z_take(z_owned_closure_query_t* closure_, z_moved_closure_query_t* x) {
    z_closure_query_take(closure_, x);
};
inline void z_take(z_owned_closure_reply_t* closure_, z_moved_closure_reply_t* x) {
    z_closure_reply_take(closure_, x);
};
inline void z_take(z_owned_closure_sample_t* closure_, z_moved_closure_sample_t* x) {
    z_closure_sample_take(closure_, x);
};
inline void z_take(z_owned_closure_zid_t* closure_, z_moved_closure_zid_t* x) {
    z_closure_zid_take(closure_, x);
};
inline void z_take(z_owned_condvar_t* this_, z_moved_condvar_t* x) {
    z_condvar_take(this_, x);
};
inline void z_take(z_owned_config_t* this_, z_moved_config_t* x) {
    z_config_take(this_, x);
};
inline void z_take(z_owned_encoding_t* this_, z_moved_encoding_t* x) {
    z_encoding_take(this_, x);
};
inline void z_take(z_owned_fifo_handler_query_t* this_, z_moved_fifo_handler_query_t* x) {
    z_fifo_handler_query_take(this_, x);
};
inline void z_take(z_owned_fifo_handler_reply_t* this_, z_moved_fifo_handler_reply_t* x) {
    z_fifo_handler_reply_take(this_, x);
};
inline void z_take(z_owned_fifo_handler_sample_t* this_, z_moved_fifo_handler_sample_t* x) {
    z_fifo_handler_sample_take(this_, x);
};
inline void z_take(z_owned_hello_t* this_, z_moved_hello_t* x) {
    z_hello_take(this_, x);
};
inline void z_take(z_owned_keyexpr_t* this_, z_moved_keyexpr_t* x) {
    z_keyexpr_take(this_, x);
};
inline void z_take(z_owned_liveliness_token_t* this_, z_moved_liveliness_token_t* x) {
    z_liveliness_token_take(this_, x);
};
inline void z_take(z_owned_matching_listener_t* this_, z_moved_matching_listener_t* x) {
    z_matching_listener_take(this_, x);
};
inline void z_take(z_owned_memory_layout_t* this_, z_moved_memory_layout_t* x) {
    z_memory_layout_take(this_, x);
};
inline void z_take(z_owned_mutex_t* this_, z_moved_mutex_t* x) {
    z_mutex_take(this_, x);
};
inline void z_take(z_owned_precomputed_layout_t* this_, z_moved_precomputed_layout_t* x) {
    z_precomputed_layout_take(this_, x);
};
inline void z_take(z_owned_ptr_in_segment_t* this_, z_moved_ptr_in_segment_t* x) {
    z_ptr_in_segment_take(this_, x);
};
inline void z_take(z_owned_publisher_t* this_, z_moved_publisher_t* x) {
    z_publisher_take(this_, x);
};
inline void z_take(z_owned_querier_t* this_, z_moved_querier_t* x) {
    z_querier_take(this_, x);
};
inline void z_take(z_owned_query_t* this_, z_moved_query_t* x) {
    z_query_take(this_, x);
};
inline void z_take(z_owned_queryable_t* this_, z_moved_queryable_t* x) {
    z_queryable_take(this_, x);
};
inline void z_take(z_owned_reply_t* this_, z_moved_reply_t* x) {
    z_reply_take(this_, x);
};
inline void z_take(z_owned_reply_err_t* this_, z_moved_reply_err_t* x) {
    z_reply_err_take(this_, x);
};
inline void z_take(z_owned_ring_handler_query_t* this_, z_moved_ring_handler_query_t* x) {
    z_ring_handler_query_take(this_, x);
};
inline void z_take(z_owned_ring_handler_reply_t* this_, z_moved_ring_handler_reply_t* x) {
    z_ring_handler_reply_take(this_, x);
};
inline void z_take(z_owned_ring_handler_sample_t* this_, z_moved_ring_handler_sample_t* x) {
    z_ring_handler_sample_take(this_, x);
};
inline void z_take(z_owned_sample_t* this_, z_moved_sample_t* x) {
    z_sample_take(this_, x);
};
inline void z_take(z_owned_session_t* this_, z_moved_session_t* x) {
    z_session_take(this_, x);
};
inline void z_take(z_owned_shared_shm_provider_t* this_, z_moved_shared_shm_provider_t* x) {
    z_shared_shm_provider_take(this_, x);
};
inline void z_take(z_owned_shm_client_t* this_, z_moved_shm_client_t* x) {
    z_shm_client_take(this_, x);
};
inline void z_take(z_owned_shm_client_storage_t* this_, z_moved_shm_client_storage_t* x) {
    z_shm_client_storage_take(this_, x);
};
inline void z_take(z_owned_shm_t* this_, z_moved_shm_t* x) {
    z_shm_take(this_, x);
};
inline void z_take(z_owned_shm_mut_t* this_, z_moved_shm_mut_t* x) {
    z_shm_mut_take(this_, x);
};
inline void z_take(z_owned_shm_provider_t* this_, z_moved_shm_provider_t* x) {
    z_shm_provider_take(this_, x);
};
inline void z_take(z_owned_slice_t* this_, z_moved_slice_t* x) {
    z_slice_take(this_, x);
};
inline void z_take(z_owned_string_array_t* this_, z_moved_string_array_t* x) {
    z_string_array_take(this_, x);
};
inline void z_take(z_owned_string_t* this_, z_moved_string_t* x) {
    z_string_take(this_, x);
};
inline void z_take(z_owned_subscriber_t* this_, z_moved_subscriber_t* x) {
    z_subscriber_take(this_, x);
};
inline void z_take(z_owned_task_t* this_, z_moved_task_t* x) {
    z_task_take(this_, x);
};
inline void z_take(zc_owned_closure_log_t* closure_, zc_moved_closure_log_t* x) {
    zc_closure_log_take(closure_, x);
};
inline void z_take(zc_owned_concurrent_close_handle_t* this_, zc_moved_concurrent_close_handle_t* x) {
    zc_concurrent_close_handle_take(this_, x);
};
inline void z_take(zc_owned_shm_client_list_t* this_, zc_moved_shm_client_list_t* x) {
    zc_shm_client_list_take(this_, x);
};
inline void z_take(ze_owned_advanced_publisher_t* this_, ze_moved_advanced_publisher_t* x) {
    ze_advanced_publisher_take(this_, x);
};
inline void z_take(ze_owned_advanced_subscriber_t* this_, ze_moved_advanced_subscriber_t* x) {
    ze_advanced_subscriber_take(this_, x);
};
inline void z_take(ze_owned_closure_miss_t* closure_, ze_moved_closure_miss_t* x) {
    ze_closure_miss_take(closure_, x);
};
inline void z_take(ze_owned_publication_cache_t* this_, ze_moved_publication_cache_t* x) {
    ze_publication_cache_take(this_, x);
};
inline void z_take(ze_owned_querying_subscriber_t* this_, ze_moved_querying_subscriber_t* x) {
    ze_querying_subscriber_take(this_, x);
};
inline void z_take(ze_owned_sample_miss_listener_t* this_, ze_moved_sample_miss_listener_t* x) {
    ze_sample_miss_listener_take(this_, x);
};
inline void z_take(ze_owned_serializer_t* this_, ze_moved_serializer_t* x) {
    ze_serializer_take(this_, x);
};


inline void z_take_from_loaned(z_owned_hello_t* dst, z_loaned_hello_t* src) {
    z_hello_take_from_loaned(dst, src);
};
inline void z_take_from_loaned(z_owned_query_t* dst, z_loaned_query_t* src) {
    z_query_take_from_loaned(dst, src);
};
inline void z_take_from_loaned(z_owned_reply_t* dst, z_loaned_reply_t* src) {
    z_reply_take_from_loaned(dst, src);
};
inline void z_take_from_loaned(z_owned_sample_t* dst, z_loaned_sample_t* src) {
    z_sample_take_from_loaned(dst, src);
};


inline bool z_internal_check(const z_owned_bytes_t& this_) { return z_internal_bytes_check(&this_); };
inline bool z_internal_check(const z_owned_bytes_writer_t& this_) { return z_internal_bytes_writer_check(&this_); };
inline bool z_internal_check(const z_owned_cancellation_token_t& this_) { return z_internal_cancellation_token_check(&this_); };
inline bool z_internal_check(const z_owned_chunk_alloc_result_t& this_) { return z_internal_chunk_alloc_result_check(&this_); };
inline bool z_internal_check(const z_owned_closure_hello_t& this_) { return z_internal_closure_hello_check(&this_); };
inline bool z_internal_check(const z_owned_closure_matching_status_t& this_) { return z_internal_closure_matching_status_check(&this_); };
inline bool z_internal_check(const z_owned_closure_query_t& this_) { return z_internal_closure_query_check(&this_); };
inline bool z_internal_check(const z_owned_closure_reply_t& this_) { return z_internal_closure_reply_check(&this_); };
inline bool z_internal_check(const z_owned_closure_sample_t& this_) { return z_internal_closure_sample_check(&this_); };
inline bool z_internal_check(const z_owned_closure_zid_t& this_) { return z_internal_closure_zid_check(&this_); };
inline bool z_internal_check(const z_owned_condvar_t& this_) { return z_internal_condvar_check(&this_); };
inline bool z_internal_check(const z_owned_config_t& this_) { return z_internal_config_check(&this_); };
inline bool z_internal_check(const z_owned_encoding_t& this_) { return z_internal_encoding_check(&this_); };
inline bool z_internal_check(const z_owned_fifo_handler_query_t& this_) { return z_internal_fifo_handler_query_check(&this_); };
inline bool z_internal_check(const z_owned_fifo_handler_reply_t& this_) { return z_internal_fifo_handler_reply_check(&this_); };
inline bool z_internal_check(const z_owned_fifo_handler_sample_t& this_) { return z_internal_fifo_handler_sample_check(&this_); };
inline bool z_internal_check(const z_owned_hello_t& this_) { return z_internal_hello_check(&this_); };
inline bool z_internal_check(const z_owned_keyexpr_t& this_) { return z_internal_keyexpr_check(&this_); };
inline bool z_internal_check(const z_owned_liveliness_token_t& this_) { return z_internal_liveliness_token_check(&this_); };
inline bool z_internal_check(const z_owned_matching_listener_t& this_) { return z_internal_matching_listener_check(&this_); };
inline bool z_internal_check(const z_owned_memory_layout_t& this_) { return z_internal_memory_layout_check(&this_); };
inline bool z_internal_check(const z_owned_mutex_t& this_) { return z_internal_mutex_check(&this_); };
inline bool z_internal_check(const z_owned_precomputed_layout_t& this_) { return z_internal_precomputed_layout_check(&this_); };
inline bool z_internal_check(const z_owned_ptr_in_segment_t& this_) { return z_internal_ptr_in_segment_check(&this_); };
inline bool z_internal_check(const z_owned_publisher_t& this_) { return z_internal_publisher_check(&this_); };
inline bool z_internal_check(const z_owned_querier_t& this_) { return z_internal_querier_check(&this_); };
inline bool z_internal_check(const z_owned_query_t& query) { return z_internal_query_check(&query); };
inline bool z_internal_check(const z_owned_queryable_t& this_) { return z_internal_queryable_check(&this_); };
inline bool z_internal_check(const z_owned_reply_t& this_) { return z_internal_reply_check(&this_); };
inline bool z_internal_check(const z_owned_reply_err_t& this_) { return z_internal_reply_err_check(&this_); };
inline bool z_internal_check(const z_owned_ring_handler_query_t& this_) { return z_internal_ring_handler_query_check(&this_); };
inline bool z_internal_check(const z_owned_ring_handler_reply_t& this_) { return z_internal_ring_handler_reply_check(&this_); };
inline bool z_internal_check(const z_owned_ring_handler_sample_t& this_) { return z_internal_ring_handler_sample_check(&this_); };
inline bool z_internal_check(const z_owned_sample_t& this_) { return z_internal_sample_check(&this_); };
inline bool z_internal_check(const z_owned_session_t& this_) { return z_internal_session_check(&this_); };
inline bool z_internal_check(const z_owned_shared_shm_provider_t& this_) { return z_internal_shared_shm_provider_check(&this_); };
inline bool z_internal_check(const z_owned_shm_t& this_) { return z_internal_shm_check(&this_); };
inline bool z_internal_check(const z_owned_shm_client_t& this_) { return z_internal_shm_client_check(&this_); };
inline bool z_internal_check(const z_owned_shm_client_storage_t& this_) { return z_internal_shm_client_storage_check(&this_); };
inline bool z_internal_check(const z_owned_shm_mut_t& this_) { return z_internal_shm_mut_check(&this_); };
inline bool z_internal_check(const z_owned_shm_provider_t& this_) { return z_internal_shm_provider_check(&this_); };
inline bool z_internal_check(const z_owned_slice_t& this_) { return z_internal_slice_check(&this_); };
inline bool z_internal_check(const z_owned_string_array_t& this_) { return z_internal_string_array_check(&this_); };
inline bool z_internal_check(const z_owned_string_t& this_) { return z_internal_string_check(&this_); };
inline bool z_internal_check(const z_owned_subscriber_t& this_) { return z_internal_subscriber_check(&this_); };
inline bool z_internal_check(const z_owned_task_t& this_) { return z_internal_task_check(&this_); };
inline bool z_internal_check(const zc_owned_closure_log_t& this_) { return zc_internal_closure_log_check(&this_); };
inline bool z_internal_check(const zc_owned_concurrent_close_handle_t& this_) { return zc_internal_concurrent_close_handle_check(&this_); };
inline bool z_internal_check(const zc_owned_shm_client_list_t& this_) { return zc_internal_shm_client_list_check(&this_); };
inline bool z_internal_check(const ze_owned_advanced_publisher_t& this_) { return ze_internal_advanced_publisher_check(&this_); };
inline bool z_internal_check(const ze_owned_advanced_subscriber_t& this_) { return ze_internal_advanced_subscriber_check(&this_); };
inline bool z_internal_check(const ze_owned_closure_miss_t& this_) { return ze_internal_closure_miss_check(&this_); };
inline bool z_internal_check(const ze_owned_publication_cache_t& this_) { return ze_internal_publication_cache_check(&this_); };
inline bool z_internal_check(const ze_owned_querying_subscriber_t& this_) { return ze_internal_querying_subscriber_check(&this_); };
inline bool z_internal_check(const ze_owned_sample_miss_listener_t& this_) { return ze_internal_sample_miss_listener_check(&this_); };
inline bool z_internal_check(const ze_owned_serializer_t& this_) { return ze_internal_serializer_check(&this_); };


inline void z_call(const z_loaned_closure_hello_t* closure, z_loaned_hello_t* hello) {
    z_closure_hello_call(closure, hello);
};
inline void z_call(const z_loaned_closure_matching_status_t* closure, const z_matching_status_t* mathing_status) {
    z_closure_matching_status_call(closure, mathing_status);
};
inline void z_call(const z_loaned_closure_query_t* closure, z_loaned_query_t* query) {
    z_closure_query_call(closure, query);
};
inline void z_call(const z_loaned_closure_reply_t* closure, z_loaned_reply_t* reply) {
    z_closure_reply_call(closure, reply);
};
inline void z_call(const z_loaned_closure_sample_t* closure, z_loaned_sample_t* sample) {
    z_closure_sample_call(closure, sample);
};
inline void z_call(const z_loaned_closure_zid_t* closure, const z_id_t* z_id) {
    z_closure_zid_call(closure, z_id);
};
inline void z_call(const ze_loaned_closure_miss_t* closure, const ze_miss_t* mathing_status) {
    ze_closure_miss_call(closure, mathing_status);
};

extern "C" using z_closure_drop_callback_t = void(void* context);
extern "C" using z_closure_hello_callback_t = void(z_loaned_hello_t *hello, void *context);
extern "C" using z_closure_matching_status_callback_t = void(const z_matching_status_t *matching_status, void *context);
extern "C" using z_closure_query_callback_t = void(z_loaned_query_t *query, void *context);
extern "C" using z_closure_reply_callback_t = void(z_loaned_reply_t *reply, void *context);
extern "C" using z_closure_sample_callback_t = void(z_loaned_sample_t *sample, void *context);
extern "C" using z_closure_zid_callback_t = void(const z_id_t *z_id, void *context);
extern "C" using zc_closure_log_callback_t = void(zc_log_severity_t severity, const z_loaned_string_t *msg, void *context);
extern "C" using ze_closure_miss_callback_t = void(const ze_miss_t *matching_status, void *context);

inline void z_closure(z_owned_closure_hello_t* this_, z_closure_hello_callback_t* call,
    z_closure_drop_callback_t* drop, void* context) {
    z_closure_hello(this_, call, drop, context);
};
inline void z_closure(z_owned_closure_matching_status_t* this_, z_closure_matching_status_callback_t* call,
    z_closure_drop_callback_t* drop, void* context) {
    z_closure_matching_status(this_, call, drop, context);
};
inline void z_closure(z_owned_closure_query_t* this_, z_closure_query_callback_t* call,
    z_closure_drop_callback_t* drop, void* context) {
    z_closure_query(this_, call, drop, context);
};
inline void z_closure(z_owned_closure_reply_t* this_, z_closure_reply_callback_t* call,
    z_closure_drop_callback_t* drop, void* context) {
    z_closure_reply(this_, call, drop, context);
};
inline void z_closure(z_owned_closure_sample_t* this_, z_closure_sample_callback_t* call,
    z_closure_drop_callback_t* drop, void* context) {
    z_closure_sample(this_, call, drop, context);
};
inline void z_closure(z_owned_closure_zid_t* this_, z_closure_zid_callback_t* call,
    z_closure_drop_callback_t* drop, void* context) {
    z_closure_zid(this_, call, drop, context);
};
inline void z_closure(zc_owned_closure_log_t* this_, zc_closure_log_callback_t* call,
    z_closure_drop_callback_t* drop, void* context) {
    zc_closure_log(this_, call, drop, context);
};
inline void z_closure(ze_owned_closure_miss_t* this_, ze_closure_miss_callback_t* call,
    z_closure_drop_callback_t* drop, void* context) {
    ze_closure_miss(this_, call, drop, context);
};


inline z_result_t z_try_recv(const z_loaned_fifo_handler_query_t* this_, z_owned_query_t* query) {
    return z_fifo_handler_query_try_recv(this_, query);
};
inline z_result_t z_try_recv(const z_loaned_fifo_handler_reply_t* this_, z_owned_reply_t* reply) {
    return z_fifo_handler_reply_try_recv(this_, reply);
};
inline z_result_t z_try_recv(const z_loaned_fifo_handler_sample_t* this_, z_owned_sample_t* sample) {
    return z_fifo_handler_sample_try_recv(this_, sample);
};
inline z_result_t z_try_recv(const z_loaned_ring_handler_query_t* this_, z_owned_query_t* query) {
    return z_ring_handler_query_try_recv(this_, query);
};
inline z_result_t z_try_recv(const z_loaned_ring_handler_reply_t* this_, z_owned_reply_t* reply) {
    return z_ring_handler_reply_try_recv(this_, reply);
};
inline z_result_t z_try_recv(const z_loaned_ring_handler_sample_t* this_, z_owned_sample_t* sample) {
    return z_ring_handler_sample_try_recv(this_, sample);
};


inline z_result_t z_recv(const z_loaned_fifo_handler_query_t* this_, z_owned_query_t* query) {
    return z_fifo_handler_query_recv(this_, query);
};
inline z_result_t z_recv(const z_loaned_fifo_handler_reply_t* this_, z_owned_reply_t* reply) {
    return z_fifo_handler_reply_recv(this_, reply);
};
inline z_result_t z_recv(const z_loaned_fifo_handler_sample_t* this_, z_owned_sample_t* sample) {
    return z_fifo_handler_sample_recv(this_, sample);
};
inline z_result_t z_recv(const z_loaned_ring_handler_query_t* this_, z_owned_query_t* query) {
    return z_ring_handler_query_recv(this_, query);
};
inline z_result_t z_recv(const z_loaned_ring_handler_reply_t* this_, z_owned_reply_t* reply) {
    return z_ring_handler_reply_recv(this_, reply);
};
inline z_result_t z_recv(const z_loaned_ring_handler_sample_t* this_, z_owned_sample_t* sample) {
    return z_ring_handler_sample_recv(this_, sample);
};


inline void z_clone(z_owned_bytes_t* dst, const z_loaned_bytes_t* this_) {
    z_bytes_clone(dst, this_);
};
inline void z_clone(z_owned_cancellation_token_t* dst, const z_loaned_cancellation_token_t* this_) {
    z_cancellation_token_clone(dst, this_);
};
inline void z_clone(z_owned_config_t* dst, const z_loaned_config_t* this_) {
    z_config_clone(dst, this_);
};
inline void z_clone(z_owned_encoding_t* dst, const z_loaned_encoding_t* this_) {
    z_encoding_clone(dst, this_);
};
inline void z_clone(z_owned_hello_t* dst, const z_loaned_hello_t* this_) {
    z_hello_clone(dst, this_);
};
inline void z_clone(z_owned_keyexpr_t* dst, const z_loaned_keyexpr_t* this_) {
    z_keyexpr_clone(dst, this_);
};
inline void z_clone(z_owned_ptr_in_segment_t* out, const z_loaned_ptr_in_segment_t* this_) {
    z_ptr_in_segment_clone(out, this_);
};
inline void z_clone(z_owned_query_t* dst, const z_loaned_query_t* this_) {
    z_query_clone(dst, this_);
};
inline void z_clone(z_owned_reply_t* dst, const z_loaned_reply_t* this_) {
    z_reply_clone(dst, this_);
};
inline void z_clone(z_owned_reply_err_t* dst, const z_loaned_reply_err_t* this_) {
    z_reply_err_clone(dst, this_);
};
inline void z_clone(z_owned_sample_t* dst, const z_loaned_sample_t* this_) {
    z_sample_clone(dst, this_);
};
inline void z_clone(z_owned_shared_shm_provider_t* dst, const z_loaned_shared_shm_provider_t* this_) {
    z_shared_shm_provider_clone(dst, this_);
};
inline void z_clone(z_owned_shm_client_storage_t* this_, const z_loaned_shm_client_storage_t* from) {
    z_shm_client_storage_clone(this_, from);
};
inline void z_clone(z_owned_shm_t* out, const z_loaned_shm_t* this_) {
    z_shm_clone(out, this_);
};
inline void z_clone(z_owned_slice_t* dst, const z_loaned_slice_t* this_) {
    z_slice_clone(dst, this_);
};
inline void z_clone(z_owned_string_array_t* dst, const z_loaned_string_array_t* this_) {
    z_string_array_clone(dst, this_);
};
inline void z_clone(z_owned_string_t* dst, const z_loaned_string_t* this_) {
    z_string_clone(dst, this_);
};

template<class T> struct z_loaned_to_owned_type_t {};
template<class T> struct z_owned_to_loaned_type_t {};
template<> struct z_loaned_to_owned_type_t<z_loaned_bytes_t> { typedef z_owned_bytes_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_bytes_t> { typedef z_loaned_bytes_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_bytes_writer_t> { typedef z_owned_bytes_writer_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_bytes_writer_t> { typedef z_loaned_bytes_writer_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_cancellation_token_t> { typedef z_owned_cancellation_token_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_cancellation_token_t> { typedef z_loaned_cancellation_token_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_hello_t> { typedef z_owned_closure_hello_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_hello_t> { typedef z_loaned_closure_hello_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_matching_status_t> { typedef z_owned_closure_matching_status_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_matching_status_t> { typedef z_loaned_closure_matching_status_t type; };
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
template<> struct z_loaned_to_owned_type_t<z_loaned_liveliness_token_t> { typedef z_owned_liveliness_token_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_liveliness_token_t> { typedef z_loaned_liveliness_token_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_memory_layout_t> { typedef z_owned_memory_layout_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_memory_layout_t> { typedef z_loaned_memory_layout_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_precomputed_layout_t> { typedef z_owned_precomputed_layout_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_precomputed_layout_t> { typedef z_loaned_precomputed_layout_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_ptr_in_segment_t> { typedef z_owned_ptr_in_segment_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_ptr_in_segment_t> { typedef z_loaned_ptr_in_segment_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_publisher_t> { typedef z_owned_publisher_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_publisher_t> { typedef z_loaned_publisher_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_querier_t> { typedef z_owned_querier_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_querier_t> { typedef z_loaned_querier_t type; };
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
template<> struct z_loaned_to_owned_type_t<z_loaned_shared_shm_provider_t> { typedef z_owned_shared_shm_provider_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_shared_shm_provider_t> { typedef z_loaned_shared_shm_provider_t type; };
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
template<> struct z_loaned_to_owned_type_t<z_loaned_string_array_t> { typedef z_owned_string_array_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_string_array_t> { typedef z_loaned_string_array_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_string_t> { typedef z_owned_string_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_string_t> { typedef z_loaned_string_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_subscriber_t> { typedef z_owned_subscriber_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_subscriber_t> { typedef z_loaned_subscriber_t type; };
template<> struct z_loaned_to_owned_type_t<zc_loaned_closure_log_t> { typedef zc_owned_closure_log_t type; };
template<> struct z_owned_to_loaned_type_t<zc_owned_closure_log_t> { typedef zc_loaned_closure_log_t type; };
template<> struct z_loaned_to_owned_type_t<zc_loaned_shm_client_list_t> { typedef zc_owned_shm_client_list_t type; };
template<> struct z_owned_to_loaned_type_t<zc_owned_shm_client_list_t> { typedef zc_loaned_shm_client_list_t type; };
template<> struct z_loaned_to_owned_type_t<ze_loaned_advanced_publisher_t> { typedef ze_owned_advanced_publisher_t type; };
template<> struct z_owned_to_loaned_type_t<ze_owned_advanced_publisher_t> { typedef ze_loaned_advanced_publisher_t type; };
template<> struct z_loaned_to_owned_type_t<ze_loaned_advanced_subscriber_t> { typedef ze_owned_advanced_subscriber_t type; };
template<> struct z_owned_to_loaned_type_t<ze_owned_advanced_subscriber_t> { typedef ze_loaned_advanced_subscriber_t type; };
template<> struct z_loaned_to_owned_type_t<ze_loaned_closure_miss_t> { typedef ze_owned_closure_miss_t type; };
template<> struct z_owned_to_loaned_type_t<ze_owned_closure_miss_t> { typedef ze_loaned_closure_miss_t type; };
template<> struct z_loaned_to_owned_type_t<ze_loaned_publication_cache_t> { typedef ze_owned_publication_cache_t type; };
template<> struct z_owned_to_loaned_type_t<ze_owned_publication_cache_t> { typedef ze_loaned_publication_cache_t type; };
template<> struct z_loaned_to_owned_type_t<ze_loaned_querying_subscriber_t> { typedef ze_owned_querying_subscriber_t type; };
template<> struct z_owned_to_loaned_type_t<ze_owned_querying_subscriber_t> { typedef ze_loaned_querying_subscriber_t type; };
template<> struct z_loaned_to_owned_type_t<ze_loaned_serializer_t> { typedef ze_owned_serializer_t type; };
template<> struct z_owned_to_loaned_type_t<ze_owned_serializer_t> { typedef ze_loaned_serializer_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_mutex_t> { typedef z_owned_mutex_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_mutex_t> { typedef z_loaned_mutex_t type; };
#endif  // #ifndef __cplusplus

