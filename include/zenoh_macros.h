#pragma once

// clang-format off
#ifndef __cplusplus


#define z_loan(x) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_loan, \
        z_owned_bytes_writer_t : z_bytes_writer_loan, \
        z_owned_closure_hello_t : z_closure_hello_loan, \
        z_owned_closure_owned_query_t : z_closure_owned_query_loan, \
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
        z_owned_publisher_t : z_publisher_loan, \
        z_owned_query_t : z_query_loan, \
        z_owned_queryable_t : z_queryable_loan, \
        z_owned_reply_t : z_reply_loan, \
        z_owned_ring_handler_query_t : z_ring_handler_query_loan, \
        z_owned_ring_handler_reply_t : z_ring_handler_reply_loan, \
        z_owned_ring_handler_sample_t : z_ring_handler_sample_loan, \
        z_owned_sample_t : z_sample_loan, \
        z_owned_session_t : z_session_loan, \
        z_owned_slice_t : z_slice_loan, \
        z_owned_slice_map_t : z_slice_map_loan, \
        z_owned_string_array_t : z_string_array_loan, \
        z_owned_string_t : z_string_loan, \
        z_owned_subscriber_t : z_subscriber_loan, \
        z_owned_value_t : z_value_loan, \
        z_view_keyexpr_t : z_view_keyexpr_loan, \
        z_view_slice_t : z_view_slice_loan, \
        z_view_string_t : z_view_string_loan, \
        zcu_owned_closure_matching_status_t : zcu_closure_matching_status_loan, \
        ze_owned_querying_subscriber_t : ze_querying_subscriber_loan \
    )(&x)

#define z_loan_mut(x) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_loan_mut, \
        z_owned_bytes_writer_t : z_bytes_writer_loan_mut, \
        z_owned_condvar_t : z_condvar_loan_mut, \
        z_owned_config_t : z_config_loan_mut, \
        z_owned_mutex_t : z_mutex_loan_mut, \
        z_owned_slice_map_t : z_slice_map_loan_mut, \
        z_owned_string_array_t : z_string_array_loan_mut \
    )(&x)

#define z_drop(x) \
    _Generic((x), \
        z_owned_bytes_t* : z_bytes_drop, \
        z_owned_bytes_writer_t* : z_bytes_writer_drop, \
        z_owned_closure_hello_t* : z_closure_hello_drop, \
        z_owned_closure_owned_query_t* : z_closure_owned_query_drop, \
        z_owned_closure_query_t* : z_closure_query_drop, \
        z_owned_closure_reply_t* : z_closure_reply_drop, \
        z_owned_closure_sample_t* : z_closure_sample_drop, \
        z_owned_closure_zid_t* : z_closure_zid_drop, \
        z_owned_condvar_t* : z_condvar_drop, \
        z_owned_config_t* : z_config_drop, \
        z_owned_encoding_t* : z_encoding_drop, \
        z_owned_fifo_handler_query_t* : z_fifo_handler_query_drop, \
        z_owned_fifo_handler_reply_t* : z_fifo_handler_reply_drop, \
        z_owned_fifo_handler_sample_t* : z_fifo_handler_sample_drop, \
        z_owned_hello_t* : z_hello_drop, \
        z_owned_keyexpr_t* : z_keyexpr_drop, \
        z_owned_mutex_t* : z_mutex_drop, \
        z_owned_publisher_t* : z_publisher_drop, \
        z_owned_query_t* : z_query_drop, \
        z_owned_queryable_t* : z_queryable_drop, \
        z_owned_reply_t* : z_reply_drop, \
        z_owned_ring_handler_query_t* : z_ring_handler_query_drop, \
        z_owned_ring_handler_reply_t* : z_ring_handler_reply_drop, \
        z_owned_ring_handler_sample_t* : z_ring_handler_sample_drop, \
        z_owned_sample_t* : z_sample_drop, \
        z_owned_session_t* : z_session_drop, \
        z_owned_slice_t* : z_slice_drop, \
        z_owned_slice_map_t* : z_slice_map_drop, \
        z_owned_string_array_t* : z_string_array_drop, \
        z_owned_string_t* : z_string_drop, \
        z_owned_subscriber_t* : z_subscriber_drop, \
        z_owned_value_t* : z_value_drop, \
        zc_owned_liveliness_token_t* : zc_liveliness_token_drop, \
        zcu_owned_closure_matching_status_t* : zcu_closure_matching_status_drop, \
        ze_owned_publication_cache_t* : ze_publication_cache_drop, \
        ze_owned_querying_subscriber_t* : ze_querying_subscriber_drop \
    )(x)

#define z_move(x) (&x)

#define z_null(x) \
    _Generic((x), \
        z_owned_bytes_t* : z_bytes_null, \
        z_owned_bytes_writer_t* : z_bytes_writer_null, \
        z_owned_closure_hello_t* : z_closure_hello_null, \
        z_owned_closure_query_t* : z_closure_query_null, \
        z_owned_closure_reply_t* : z_closure_reply_null, \
        z_owned_closure_sample_t* : z_closure_sample_null, \
        z_owned_closure_zid_t* : z_closure_zid_null, \
        z_owned_condvar_t* : z_condvar_null, \
        z_owned_config_t* : z_config_null, \
        z_owned_encoding_t* : z_encoding_null, \
        z_owned_fifo_handler_query_t* : z_fifo_handler_query_null, \
        z_owned_fifo_handler_reply_t* : z_fifo_handler_reply_null, \
        z_owned_fifo_handler_sample_t* : z_fifo_handler_sample_null, \
        z_owned_hello_t* : z_hello_null, \
        z_owned_keyexpr_t* : z_keyexpr_null, \
        z_owned_mutex_t* : z_mutex_null, \
        z_owned_publisher_t* : z_publisher_null, \
        z_owned_query_t* : z_query_null, \
        z_owned_queryable_t* : z_queryable_null, \
        z_owned_reply_t* : z_reply_null, \
        z_owned_ring_handler_query_t* : z_ring_handler_query_null, \
        z_owned_ring_handler_reply_t* : z_ring_handler_reply_null, \
        z_owned_ring_handler_sample_t* : z_ring_handler_sample_null, \
        z_owned_sample_t* : z_sample_null, \
        z_owned_session_t* : z_session_null, \
        z_owned_slice_map_t* : z_slice_map_null, \
        z_owned_slice_t* : z_slice_null, \
        z_owned_string_array_t* : z_string_array_null, \
        z_owned_string_t* : z_string_null, \
        z_owned_subscriber_t* : z_subscriber_null, \
        z_owned_task_t* : z_task_null, \
        z_owned_value_t* : z_value_null, \
        z_view_keyexpr_t* : z_view_keyexpr_null, \
        z_view_slice_t* : z_view_slice_null, \
        z_view_string_t* : z_view_string_null, \
        zc_owned_liveliness_token_t* : zc_liveliness_token_null, \
        zcu_owned_closure_matching_status_t* : zcu_closure_matching_status_null, \
        ze_owned_publication_cache_t* : ze_publication_cache_null, \
        ze_owned_querying_subscriber_t* : ze_querying_subscriber_null \
    )(x)

#define z_check(x) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_check, \
        z_owned_bytes_writer_t : z_bytes_writer_check, \
        z_owned_closure_hello_t : z_closure_hello_check, \
        z_owned_closure_query_t : z_closure_query_check, \
        z_owned_closure_reply_t : z_closure_reply_check, \
        z_owned_closure_sample_t : z_closure_sample_check, \
        z_owned_closure_zid_t : z_closure_zid_check, \
        z_owned_condvar_t : z_condvar_check, \
        z_owned_config_t : z_config_check, \
        z_owned_encoding_t : z_encoding_check, \
        z_owned_fifo_handler_query_t : z_fifo_handler_query_check, \
        z_owned_fifo_handler_reply_t : z_fifo_handler_reply_check, \
        z_owned_fifo_handler_sample_t : z_fifo_handler_sample_check, \
        z_owned_hello_t : z_hello_check, \
        z_owned_keyexpr_t : z_keyexpr_check, \
        z_owned_mutex_t : z_mutex_check, \
        z_owned_publisher_t : z_publisher_check, \
        z_owned_query_t : z_query_check, \
        z_owned_queryable_t : z_queryable_check, \
        z_owned_reply_t : z_reply_check, \
        z_owned_ring_handler_query_t : z_ring_handler_query_check, \
        z_owned_ring_handler_reply_t : z_ring_handler_reply_check, \
        z_owned_ring_handler_sample_t : z_ring_handler_sample_check, \
        z_owned_sample_t : z_sample_check, \
        z_owned_session_t : z_session_check, \
        z_owned_slice_t : z_slice_check, \
        z_owned_slice_map_t : z_slice_map_check, \
        z_owned_string_array_t : z_string_array_check, \
        z_owned_string_t : z_string_check, \
        z_owned_subscriber_t : z_subscriber_check, \
        z_owned_task_t : z_task_check, \
        z_owned_value_t : z_value_check, \
        z_view_keyexpr_t : z_view_keyexpr_check, \
        z_view_slice_t : z_view_slice_check, \
        z_view_string_t : z_view_string_check, \
        zc_owned_liveliness_token_t : zc_liveliness_token_check, \
        zcu_owned_closure_matching_status_t : zcu_closure_matching_status_check, \
        ze_owned_publication_cache_t : ze_publication_cache_check, \
        ze_owned_querying_subscriber_t : ze_querying_subscriber_check \
    )(&x)

#define z_call(x, ...) \
    _Generic((x), \
        const z_loaned_closure_hello_t* : z_closure_hello_call, \
        const z_loaned_closure_owned_query_t* : z_closure_owned_query_call, \
        const z_loaned_closure_query_t* : z_closure_query_call, \
        const z_loaned_closure_reply_t* : z_closure_reply_call, \
        const z_loaned_closure_sample_t* : z_closure_sample_call, \
        const z_loaned_closure_zid_t* : z_closure_zid_call, \
        const zcu_loaned_closure_matching_status_t* : zcu_closure_matching_status_call \
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



inline const z_loaned_bytes_t* z_loan(const z_owned_bytes_t& this_) { return z_bytes_loan(&this_); };
inline const z_loaned_bytes_writer_t* z_loan(const z_owned_bytes_writer_t& this_) { return z_bytes_writer_loan(&this_); };
inline const z_loaned_closure_hello_t* z_loan(const z_owned_closure_hello_t& closure) { return z_closure_hello_loan(&closure); };
inline const z_loaned_closure_owned_query_t* z_loan(const z_owned_closure_owned_query_t& closure) { return z_closure_owned_query_loan(&closure); };
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
inline const z_loaned_publisher_t* z_loan(const z_owned_publisher_t& this_) { return z_publisher_loan(&this_); };
inline const z_loaned_query_t* z_loan(const z_owned_query_t& this_) { return z_query_loan(&this_); };
inline const z_loaned_queryable_t* z_loan(const z_owned_queryable_t& this_) { return z_queryable_loan(&this_); };
inline const z_loaned_reply_t* z_loan(const z_owned_reply_t& this_) { return z_reply_loan(&this_); };
inline const z_loaned_ring_handler_query_t* z_loan(const z_owned_ring_handler_query_t& this_) { return z_ring_handler_query_loan(&this_); };
inline const z_loaned_ring_handler_reply_t* z_loan(const z_owned_ring_handler_reply_t& this_) { return z_ring_handler_reply_loan(&this_); };
inline const z_loaned_ring_handler_sample_t* z_loan(const z_owned_ring_handler_sample_t& this_) { return z_ring_handler_sample_loan(&this_); };
inline const z_loaned_sample_t* z_loan(const z_owned_sample_t& this_) { return z_sample_loan(&this_); };
inline const z_loaned_session_t* z_loan(const z_owned_session_t& this_) { return z_session_loan(&this_); };
inline const z_loaned_slice_t* z_loan(const z_owned_slice_t& this_) { return z_slice_loan(&this_); };
inline const z_loaned_slice_map_t* z_loan(const z_owned_slice_map_t& this_) { return z_slice_map_loan(&this_); };
inline const z_loaned_string_array_t* z_loan(const z_owned_string_array_t& this_) { return z_string_array_loan(&this_); };
inline const z_loaned_string_t* z_loan(const z_owned_string_t& this_) { return z_string_loan(&this_); };
inline const z_loaned_subscriber_t* z_loan(const z_owned_subscriber_t& this_) { return z_subscriber_loan(&this_); };
inline const z_loaned_value_t* z_loan(const z_owned_value_t& this_) { return z_value_loan(&this_); };
inline const z_loaned_keyexpr_t* z_loan(const z_view_keyexpr_t& this_) { return z_view_keyexpr_loan(&this_); };
inline const z_loaned_slice_t* z_loan(const z_view_slice_t& this_) { return z_view_slice_loan(&this_); };
inline const z_loaned_string_t* z_loan(const z_view_string_t& this_) { return z_view_string_loan(&this_); };
inline const zcu_loaned_closure_matching_status_t* z_loan(const zcu_owned_closure_matching_status_t& closure) { return zcu_closure_matching_status_loan(&closure); };
inline const ze_loaned_querying_subscriber_t* z_loan(const ze_owned_querying_subscriber_t& this_) { return ze_querying_subscriber_loan(&this_); };


inline z_loaned_bytes_t* z_loan_mut(z_owned_bytes_t& this_) { return z_bytes_loan_mut(&this_); };
inline z_loaned_bytes_writer_t* z_loan_mut(z_owned_bytes_writer_t& this_) { return z_bytes_writer_loan_mut(&this_); };
inline z_loaned_condvar_t* z_loan_mut(z_owned_condvar_t& this_) { return z_condvar_loan_mut(&this_); };
inline z_loaned_config_t* z_loan_mut(z_owned_config_t& this_) { return z_config_loan_mut(&this_); };
inline z_loaned_mutex_t* z_loan_mut(z_owned_mutex_t& this_) { return z_mutex_loan_mut(&this_); };
inline z_loaned_slice_map_t* z_loan_mut(z_owned_slice_map_t& this_) { return z_slice_map_loan_mut(&this_); };
inline z_loaned_string_array_t* z_loan_mut(z_owned_string_array_t& this_) { return z_string_array_loan_mut(&this_); };


inline void z_drop(z_owned_bytes_t* this_) { return z_bytes_drop(this_); };
inline void z_drop(z_owned_bytes_writer_t* this_) { return z_bytes_writer_drop(this_); };
inline void z_drop(z_owned_closure_hello_t* closure) { return z_closure_hello_drop(closure); };
inline void z_drop(z_owned_closure_owned_query_t* closure) { return z_closure_owned_query_drop(closure); };
inline void z_drop(z_owned_closure_query_t* closure) { return z_closure_query_drop(closure); };
inline void z_drop(z_owned_closure_reply_t* closure) { return z_closure_reply_drop(closure); };
inline void z_drop(z_owned_closure_sample_t* closure) { return z_closure_sample_drop(closure); };
inline void z_drop(z_owned_closure_zid_t* closure) { return z_closure_zid_drop(closure); };
inline void z_drop(z_owned_condvar_t* this_) { return z_condvar_drop(this_); };
inline void z_drop(z_owned_config_t* this_) { return z_config_drop(this_); };
inline void z_drop(z_owned_encoding_t* this_) { return z_encoding_drop(this_); };
inline void z_drop(z_owned_fifo_handler_query_t* this_) { return z_fifo_handler_query_drop(this_); };
inline void z_drop(z_owned_fifo_handler_reply_t* this_) { return z_fifo_handler_reply_drop(this_); };
inline void z_drop(z_owned_fifo_handler_sample_t* this_) { return z_fifo_handler_sample_drop(this_); };
inline void z_drop(z_owned_hello_t* this_) { return z_hello_drop(this_); };
inline void z_drop(z_owned_keyexpr_t* this_) { return z_keyexpr_drop(this_); };
inline void z_drop(z_owned_mutex_t* this_) { return z_mutex_drop(this_); };
inline void z_drop(z_owned_publisher_t* this_) { return z_publisher_drop(this_); };
inline void z_drop(z_owned_query_t* this_) { return z_query_drop(this_); };
inline void z_drop(z_owned_queryable_t* this_) { return z_queryable_drop(this_); };
inline void z_drop(z_owned_reply_t* this_) { return z_reply_drop(this_); };
inline void z_drop(z_owned_ring_handler_query_t* this_) { return z_ring_handler_query_drop(this_); };
inline void z_drop(z_owned_ring_handler_reply_t* this_) { return z_ring_handler_reply_drop(this_); };
inline void z_drop(z_owned_ring_handler_sample_t* this_) { return z_ring_handler_sample_drop(this_); };
inline void z_drop(z_owned_sample_t* this_) { return z_sample_drop(this_); };
inline void z_drop(z_owned_session_t* this_) { return z_session_drop(this_); };
inline void z_drop(z_owned_slice_t* this_) { return z_slice_drop(this_); };
inline void z_drop(z_owned_slice_map_t* this_) { return z_slice_map_drop(this_); };
inline void z_drop(z_owned_string_array_t* this_) { return z_string_array_drop(this_); };
inline void z_drop(z_owned_string_t* this_) { return z_string_drop(this_); };
inline void z_drop(z_owned_subscriber_t* this_) { return z_subscriber_drop(this_); };
inline void z_drop(z_owned_value_t* this_) { return z_value_drop(this_); };
inline void z_drop(zc_owned_liveliness_token_t* this_) { return zc_liveliness_token_drop(this_); };
inline void z_drop(zcu_owned_closure_matching_status_t* closure) { return zcu_closure_matching_status_drop(closure); };
inline void z_drop(ze_owned_publication_cache_t* this_) { return ze_publication_cache_drop(this_); };
inline void z_drop(ze_owned_querying_subscriber_t* this_) { return ze_querying_subscriber_drop(this_); };


inline z_owned_bytes_t* z_move(z_owned_bytes_t& this_) { return (&this_); };
inline z_owned_bytes_writer_t* z_move(z_owned_bytes_writer_t& this_) { return (&this_); };
inline z_owned_closure_hello_t* z_move(z_owned_closure_hello_t& closure) { return (&closure); };
inline z_owned_closure_owned_query_t* z_move(z_owned_closure_owned_query_t& closure) { return (&closure); };
inline z_owned_closure_query_t* z_move(z_owned_closure_query_t& closure) { return (&closure); };
inline z_owned_closure_reply_t* z_move(z_owned_closure_reply_t& closure) { return (&closure); };
inline z_owned_closure_sample_t* z_move(z_owned_closure_sample_t& closure) { return (&closure); };
inline z_owned_closure_zid_t* z_move(z_owned_closure_zid_t& closure) { return (&closure); };
inline z_owned_condvar_t* z_move(z_owned_condvar_t& this_) { return (&this_); };
inline z_owned_config_t* z_move(z_owned_config_t& this_) { return (&this_); };
inline z_owned_encoding_t* z_move(z_owned_encoding_t& this_) { return (&this_); };
inline z_owned_fifo_handler_query_t* z_move(z_owned_fifo_handler_query_t& this_) { return (&this_); };
inline z_owned_fifo_handler_reply_t* z_move(z_owned_fifo_handler_reply_t& this_) { return (&this_); };
inline z_owned_fifo_handler_sample_t* z_move(z_owned_fifo_handler_sample_t& this_) { return (&this_); };
inline z_owned_hello_t* z_move(z_owned_hello_t& this_) { return (&this_); };
inline z_owned_keyexpr_t* z_move(z_owned_keyexpr_t& this_) { return (&this_); };
inline z_owned_mutex_t* z_move(z_owned_mutex_t& this_) { return (&this_); };
inline z_owned_publisher_t* z_move(z_owned_publisher_t& this_) { return (&this_); };
inline z_owned_query_t* z_move(z_owned_query_t& this_) { return (&this_); };
inline z_owned_queryable_t* z_move(z_owned_queryable_t& this_) { return (&this_); };
inline z_owned_reply_t* z_move(z_owned_reply_t& this_) { return (&this_); };
inline z_owned_ring_handler_query_t* z_move(z_owned_ring_handler_query_t& this_) { return (&this_); };
inline z_owned_ring_handler_reply_t* z_move(z_owned_ring_handler_reply_t& this_) { return (&this_); };
inline z_owned_ring_handler_sample_t* z_move(z_owned_ring_handler_sample_t& this_) { return (&this_); };
inline z_owned_sample_t* z_move(z_owned_sample_t& this_) { return (&this_); };
inline z_owned_session_t* z_move(z_owned_session_t& this_) { return (&this_); };
inline z_owned_slice_t* z_move(z_owned_slice_t& this_) { return (&this_); };
inline z_owned_slice_map_t* z_move(z_owned_slice_map_t& this_) { return (&this_); };
inline z_owned_string_array_t* z_move(z_owned_string_array_t& this_) { return (&this_); };
inline z_owned_string_t* z_move(z_owned_string_t& this_) { return (&this_); };
inline z_owned_subscriber_t* z_move(z_owned_subscriber_t& this_) { return (&this_); };
inline z_owned_value_t* z_move(z_owned_value_t& this_) { return (&this_); };
inline zc_owned_liveliness_token_t* z_move(zc_owned_liveliness_token_t& this_) { return (&this_); };
inline zcu_owned_closure_matching_status_t* z_move(zcu_owned_closure_matching_status_t& closure) { return (&closure); };
inline ze_owned_publication_cache_t* z_move(ze_owned_publication_cache_t& this_) { return (&this_); };
inline ze_owned_querying_subscriber_t* z_move(ze_owned_querying_subscriber_t& this_) { return (&this_); };


inline void z_null(z_owned_bytes_t* this_) { return z_bytes_null(this_); };
inline void z_null(z_owned_bytes_writer_t* this_) { return z_bytes_writer_null(this_); };
inline void z_null(z_owned_closure_hello_t* this_) { return z_closure_hello_null(this_); };
inline void z_null(z_owned_closure_query_t* this_) { return z_closure_query_null(this_); };
inline void z_null(z_owned_closure_reply_t* this_) { return z_closure_reply_null(this_); };
inline void z_null(z_owned_closure_sample_t* this_) { return z_closure_sample_null(this_); };
inline void z_null(z_owned_closure_zid_t* this_) { return z_closure_zid_null(this_); };
inline void z_null(z_owned_condvar_t* this_) { return z_condvar_null(this_); };
inline void z_null(z_owned_config_t* this_) { return z_config_null(this_); };
inline void z_null(z_owned_encoding_t* this_) { return z_encoding_null(this_); };
inline void z_null(z_owned_fifo_handler_query_t* this_) { return z_fifo_handler_query_null(this_); };
inline void z_null(z_owned_fifo_handler_reply_t* this_) { return z_fifo_handler_reply_null(this_); };
inline void z_null(z_owned_fifo_handler_sample_t* this_) { return z_fifo_handler_sample_null(this_); };
inline void z_null(z_owned_hello_t* this_) { return z_hello_null(this_); };
inline void z_null(z_owned_keyexpr_t* this_) { return z_keyexpr_null(this_); };
inline void z_null(z_owned_mutex_t* this_) { return z_mutex_null(this_); };
inline void z_null(z_owned_publisher_t* this_) { return z_publisher_null(this_); };
inline void z_null(z_owned_query_t* this_) { return z_query_null(this_); };
inline void z_null(z_owned_queryable_t* this_) { return z_queryable_null(this_); };
inline void z_null(z_owned_reply_t* this_) { return z_reply_null(this_); };
inline void z_null(z_owned_ring_handler_query_t* this_) { return z_ring_handler_query_null(this_); };
inline void z_null(z_owned_ring_handler_reply_t* this_) { return z_ring_handler_reply_null(this_); };
inline void z_null(z_owned_ring_handler_sample_t* this_) { return z_ring_handler_sample_null(this_); };
inline void z_null(z_owned_sample_t* this_) { return z_sample_null(this_); };
inline void z_null(z_owned_session_t* this_) { return z_session_null(this_); };
inline void z_null(z_owned_slice_map_t* this_) { return z_slice_map_null(this_); };
inline void z_null(z_owned_slice_t* this_) { return z_slice_null(this_); };
inline void z_null(z_owned_string_array_t* this_) { return z_string_array_null(this_); };
inline void z_null(z_owned_string_t* this_) { return z_string_null(this_); };
inline void z_null(z_owned_subscriber_t* this_) { return z_subscriber_null(this_); };
inline void z_null(z_owned_task_t* this_) { return z_task_null(this_); };
inline void z_null(z_owned_value_t* this_) { return z_value_null(this_); };
inline void z_null(z_view_keyexpr_t* this_) { return z_view_keyexpr_null(this_); };
inline void z_null(z_view_slice_t* this_) { return z_view_slice_null(this_); };
inline void z_null(z_view_string_t* this_) { return z_view_string_null(this_); };
inline void z_null(zc_owned_liveliness_token_t* this_) { return zc_liveliness_token_null(this_); };
inline void z_null(zcu_owned_closure_matching_status_t* this_) { return zcu_closure_matching_status_null(this_); };
inline void z_null(ze_owned_publication_cache_t* this_) { return ze_publication_cache_null(this_); };
inline void z_null(ze_owned_querying_subscriber_t* this_) { return ze_querying_subscriber_null(this_); };


inline bool z_check(const z_owned_bytes_t& this_) { return z_bytes_check(&this_); };
inline bool z_check(const z_owned_bytes_writer_t& this_) { return z_bytes_writer_check(&this_); };
inline bool z_check(const z_owned_closure_hello_t& this_) { return z_closure_hello_check(&this_); };
inline bool z_check(const z_owned_closure_query_t& this_) { return z_closure_query_check(&this_); };
inline bool z_check(const z_owned_closure_reply_t& this_) { return z_closure_reply_check(&this_); };
inline bool z_check(const z_owned_closure_sample_t& this_) { return z_closure_sample_check(&this_); };
inline bool z_check(const z_owned_closure_zid_t& this_) { return z_closure_zid_check(&this_); };
inline bool z_check(const z_owned_condvar_t& this_) { return z_condvar_check(&this_); };
inline bool z_check(const z_owned_config_t& this_) { return z_config_check(&this_); };
inline bool z_check(const z_owned_encoding_t& this_) { return z_encoding_check(&this_); };
inline bool z_check(const z_owned_fifo_handler_query_t& this_) { return z_fifo_handler_query_check(&this_); };
inline bool z_check(const z_owned_fifo_handler_reply_t& this_) { return z_fifo_handler_reply_check(&this_); };
inline bool z_check(const z_owned_fifo_handler_sample_t& this_) { return z_fifo_handler_sample_check(&this_); };
inline bool z_check(const z_owned_hello_t& this_) { return z_hello_check(&this_); };
inline bool z_check(const z_owned_keyexpr_t& this_) { return z_keyexpr_check(&this_); };
inline bool z_check(const z_owned_mutex_t& this_) { return z_mutex_check(&this_); };
inline bool z_check(const z_owned_publisher_t& this_) { return z_publisher_check(&this_); };
inline bool z_check(const z_owned_query_t& query) { return z_query_check(&query); };
inline bool z_check(const z_owned_queryable_t& this_) { return z_queryable_check(&this_); };
inline bool z_check(const z_owned_reply_t& this_) { return z_reply_check(&this_); };
inline bool z_check(const z_owned_ring_handler_query_t& this_) { return z_ring_handler_query_check(&this_); };
inline bool z_check(const z_owned_ring_handler_reply_t& this_) { return z_ring_handler_reply_check(&this_); };
inline bool z_check(const z_owned_ring_handler_sample_t& this_) { return z_ring_handler_sample_check(&this_); };
inline bool z_check(const z_owned_sample_t& this_) { return z_sample_check(&this_); };
inline bool z_check(const z_owned_session_t& this_) { return z_session_check(&this_); };
inline bool z_check(const z_owned_slice_t& this_) { return z_slice_check(&this_); };
inline bool z_check(const z_owned_slice_map_t& map) { return z_slice_map_check(&map); };
inline bool z_check(const z_owned_string_array_t& this_) { return z_string_array_check(&this_); };
inline bool z_check(const z_owned_string_t& this_) { return z_string_check(&this_); };
inline bool z_check(const z_owned_subscriber_t& this_) { return z_subscriber_check(&this_); };
inline bool z_check(const z_owned_task_t& this_) { return z_task_check(&this_); };
inline bool z_check(const z_owned_value_t& this_) { return z_value_check(&this_); };
inline bool z_check(const z_view_keyexpr_t& this_) { return z_view_keyexpr_check(&this_); };
inline bool z_check(const z_view_slice_t& this_) { return z_view_slice_check(&this_); };
inline bool z_check(const z_view_string_t& this_) { return z_view_string_check(&this_); };
inline bool z_check(const zc_owned_liveliness_token_t& this_) { return zc_liveliness_token_check(&this_); };
inline bool z_check(const zcu_owned_closure_matching_status_t& this_) { return zcu_closure_matching_status_check(&this_); };
inline bool z_check(const ze_owned_publication_cache_t& this_) { return ze_publication_cache_check(&this_); };
inline bool z_check(const ze_owned_querying_subscriber_t& this_) { return ze_querying_subscriber_check(&this_); };


inline void z_call(const z_loaned_closure_hello_t* closure, const z_loaned_hello_t* hello) {
    return z_closure_hello_call(closure, hello);
};
inline void z_call(const z_loaned_closure_owned_query_t* closure, z_owned_query_t* query) {
    return z_closure_owned_query_call(closure, query);
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
inline void z_call(const zcu_loaned_closure_matching_status_t* closure, const zcu_matching_status_t* mathing_status) {
    return zcu_closure_matching_status_call(closure, mathing_status);
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
    z_owned_closure_owned_query_t* closure,
    void (*call)(z_owned_query_t*, void*),
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
    zcu_owned_closure_matching_status_t* closure,
    void (*call)(const zcu_matching_status_t*, void*),
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
template<> struct z_loaned_to_owned_type_t<z_loaned_bytes_t> { typedef z_owned_bytes_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_bytes_t> { typedef z_loaned_bytes_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_bytes_writer_t> { typedef z_owned_bytes_writer_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_bytes_writer_t> { typedef z_loaned_bytes_writer_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_hello_t> { typedef z_owned_closure_hello_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_hello_t> { typedef z_loaned_closure_hello_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_owned_query_t> { typedef z_owned_closure_owned_query_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_owned_query_t> { typedef z_loaned_closure_owned_query_t type; };
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
template<> struct z_loaned_to_owned_type_t<z_loaned_publisher_t> { typedef z_owned_publisher_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_publisher_t> { typedef z_loaned_publisher_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_query_t> { typedef z_owned_query_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_query_t> { typedef z_loaned_query_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_queryable_t> { typedef z_owned_queryable_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_queryable_t> { typedef z_loaned_queryable_t type; };
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
template<> struct z_loaned_to_owned_type_t<z_loaned_slice_t> { typedef z_owned_slice_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_slice_t> { typedef z_loaned_slice_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_slice_map_t> { typedef z_owned_slice_map_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_slice_map_t> { typedef z_loaned_slice_map_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_string_array_t> { typedef z_owned_string_array_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_string_array_t> { typedef z_loaned_string_array_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_string_t> { typedef z_owned_string_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_string_t> { typedef z_loaned_string_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_subscriber_t> { typedef z_owned_subscriber_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_subscriber_t> { typedef z_loaned_subscriber_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_value_t> { typedef z_owned_value_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_value_t> { typedef z_loaned_value_t type; };
template<> struct z_loaned_to_owned_type_t<zcu_loaned_closure_matching_status_t> { typedef zcu_owned_closure_matching_status_t type; };
template<> struct z_owned_to_loaned_type_t<zcu_owned_closure_matching_status_t> { typedef zcu_loaned_closure_matching_status_t type; };
template<> struct z_loaned_to_owned_type_t<ze_loaned_querying_subscriber_t> { typedef ze_owned_querying_subscriber_t type; };
template<> struct z_owned_to_loaned_type_t<ze_owned_querying_subscriber_t> { typedef ze_loaned_querying_subscriber_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_mutex_t> { typedef z_owned_mutex_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_mutex_t> { typedef z_loaned_mutex_t type; };
#endif  // #ifndef __cplusplus