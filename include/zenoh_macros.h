#pragma once
// clang-format off


#ifndef __cplusplus

static inline z_moved_bytes_t z_bytes_move(z_owned_bytes_t* x) { return (z_moved_bytes_t){x}; }
static inline z_moved_bytes_writer_t z_bytes_writer_move(z_owned_bytes_writer_t* x) { return (z_moved_bytes_writer_t){x}; }
static inline z_moved_closure_hello_t z_closure_hello_move(z_owned_closure_hello_t* x) { return (z_moved_closure_hello_t){x}; }
static inline z_moved_closure_query_t z_closure_query_move(z_owned_closure_query_t* x) { return (z_moved_closure_query_t){x}; }
static inline z_moved_closure_reply_t z_closure_reply_move(z_owned_closure_reply_t* x) { return (z_moved_closure_reply_t){x}; }
static inline z_moved_closure_sample_t z_closure_sample_move(z_owned_closure_sample_t* x) { return (z_moved_closure_sample_t){x}; }
static inline z_moved_condvar_t z_condvar_move(z_owned_condvar_t* x) { return (z_moved_condvar_t){x}; }
static inline z_moved_config_t z_config_move(z_owned_config_t* x) { return (z_moved_config_t){x}; }
static inline z_moved_encoding_t z_encoding_move(z_owned_encoding_t* x) { return (z_moved_encoding_t){x}; }
static inline z_moved_fifo_handler_query_t z_fifo_handler_query_move(z_owned_fifo_handler_query_t* x) { return (z_moved_fifo_handler_query_t){x}; }
static inline z_moved_fifo_handler_reply_t z_fifo_handler_reply_move(z_owned_fifo_handler_reply_t* x) { return (z_moved_fifo_handler_reply_t){x}; }
static inline z_moved_fifo_handler_sample_t z_fifo_handler_sample_move(z_owned_fifo_handler_sample_t* x) { return (z_moved_fifo_handler_sample_t){x}; }
static inline z_moved_hello_t z_hello_move(z_owned_hello_t* x) { return (z_moved_hello_t){x}; }
static inline z_moved_keyexpr_t z_keyexpr_move(z_owned_keyexpr_t* x) { return (z_moved_keyexpr_t){x}; }
static inline z_moved_mutex_t z_mutex_move(z_owned_mutex_t* x) { return (z_moved_mutex_t){x}; }
static inline z_moved_publisher_t z_publisher_move(z_owned_publisher_t* x) { return (z_moved_publisher_t){x}; }
static inline z_moved_query_t z_query_move(z_owned_query_t* x) { return (z_moved_query_t){x}; }
static inline z_moved_queryable_t z_queryable_move(z_owned_queryable_t* x) { return (z_moved_queryable_t){x}; }
static inline z_moved_reply_t z_reply_move(z_owned_reply_t* x) { return (z_moved_reply_t){x}; }
static inline z_moved_reply_err_t z_reply_err_move(z_owned_reply_err_t* x) { return (z_moved_reply_err_t){x}; }
static inline z_moved_ring_handler_query_t z_ring_handler_query_move(z_owned_ring_handler_query_t* x) { return (z_moved_ring_handler_query_t){x}; }
static inline z_moved_ring_handler_reply_t z_ring_handler_reply_move(z_owned_ring_handler_reply_t* x) { return (z_moved_ring_handler_reply_t){x}; }
static inline z_moved_ring_handler_sample_t z_ring_handler_sample_move(z_owned_ring_handler_sample_t* x) { return (z_moved_ring_handler_sample_t){x}; }
static inline z_moved_sample_t z_sample_move(z_owned_sample_t* x) { return (z_moved_sample_t){x}; }
static inline z_moved_session_t z_session_move(z_owned_session_t* x) { return (z_moved_session_t){x}; }
static inline z_moved_slice_t z_slice_move(z_owned_slice_t* x) { return (z_moved_slice_t){x}; }
static inline z_moved_string_array_t z_string_array_move(z_owned_string_array_t* x) { return (z_moved_string_array_t){x}; }
static inline z_moved_string_t z_string_move(z_owned_string_t* x) { return (z_moved_string_t){x}; }
static inline z_moved_subscriber_t z_subscriber_move(z_owned_subscriber_t* x) { return (z_moved_subscriber_t){x}; }


#define z_loan(this_) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_loan, \
        z_owned_bytes_writer_t : z_bytes_writer_loan, \
        z_owned_closure_hello_t : z_closure_hello_loan, \
        z_owned_closure_query_t : z_closure_query_loan, \
        z_owned_closure_reply_t : z_closure_reply_loan, \
        z_owned_closure_sample_t : z_closure_sample_loan, \
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
        z_owned_reply_err_t : z_reply_err_loan, \
        z_owned_reply_t : z_reply_loan, \
        z_owned_ring_handler_query_t : z_ring_handler_query_loan, \
        z_owned_ring_handler_reply_t : z_ring_handler_reply_loan, \
        z_owned_ring_handler_sample_t : z_ring_handler_sample_loan, \
        z_owned_sample_t : z_sample_loan, \
        z_owned_session_t : z_session_loan, \
        z_owned_slice_t : z_slice_loan, \
        z_owned_string_array_t : z_string_array_loan, \
        z_owned_string_t : z_string_loan, \
        z_owned_subscriber_t : z_subscriber_loan, \
        z_view_keyexpr_t : z_view_keyexpr_loan, \
        z_view_slice_t : z_view_slice_loan, \
        z_view_string_t : z_view_string_loan \
    )(&this_)

#define z_loan_mut(this_) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_loan_mut, \
        z_owned_bytes_writer_t : z_bytes_writer_loan_mut, \
        z_owned_condvar_t : z_condvar_loan_mut, \
        z_owned_config_t : z_config_loan_mut, \
        z_owned_encoding_t : z_encoding_loan_mut, \
        z_owned_mutex_t : z_mutex_loan_mut, \
        z_owned_publisher_t : z_publisher_loan_mut, \
        z_owned_string_array_t : z_string_array_loan_mut \
    )(&this_)

#define z_drop(this_) \
    _Generic((x), \
        z_moved_bytes_t : z_bytes_drop, \
        z_moved_bytes_writer_t : z_bytes_writer_drop, \
        z_moved_closure_hello_t : z_closure_hello_drop, \
        z_moved_closure_query_t : z_closure_query_drop, \
        z_moved_closure_reply_t : z_closure_reply_drop, \
        z_moved_closure_sample_t : z_closure_sample_drop, \
        z_moved_condvar_t : z_condvar_drop, \
        z_moved_config_t : z_config_drop, \
        z_moved_encoding_t : z_encoding_drop, \
        z_moved_fifo_handler_query_t : z_fifo_handler_query_drop, \
        z_moved_fifo_handler_reply_t : z_fifo_handler_reply_drop, \
        z_moved_fifo_handler_sample_t : z_fifo_handler_sample_drop, \
        z_moved_hello_t : z_hello_drop, \
        z_moved_keyexpr_t : z_keyexpr_drop, \
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
        z_moved_slice_t : z_slice_drop, \
        z_moved_string_array_t : z_string_array_drop, \
        z_moved_string_t : z_string_drop, \
        z_moved_subscriber_t : z_subscriber_drop \
    )(this_)

#define z_move(this_) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_move, \
        z_owned_bytes_writer_t : z_bytes_writer_move, \
        z_owned_closure_hello_t : z_closure_hello_move, \
        z_owned_closure_query_t : z_closure_query_move, \
        z_owned_closure_reply_t : z_closure_reply_move, \
        z_owned_closure_sample_t : z_closure_sample_move, \
        z_owned_condvar_t : z_condvar_move, \
        z_owned_config_t : z_config_move, \
        z_owned_encoding_t : z_encoding_move, \
        z_owned_fifo_handler_query_t : z_fifo_handler_query_move, \
        z_owned_fifo_handler_reply_t : z_fifo_handler_reply_move, \
        z_owned_fifo_handler_sample_t : z_fifo_handler_sample_move, \
        z_owned_hello_t : z_hello_move, \
        z_owned_keyexpr_t : z_keyexpr_move, \
        z_owned_mutex_t : z_mutex_move, \
        z_owned_publisher_t : z_publisher_move, \
        z_owned_query_t : z_query_move, \
        z_owned_queryable_t : z_queryable_move, \
        z_owned_reply_t : z_reply_move, \
        z_owned_reply_err_t : z_reply_err_move, \
        z_owned_ring_handler_query_t : z_ring_handler_query_move, \
        z_owned_ring_handler_reply_t : z_ring_handler_reply_move, \
        z_owned_ring_handler_sample_t : z_ring_handler_sample_move, \
        z_owned_sample_t : z_sample_move, \
        z_owned_session_t : z_session_move, \
        z_owned_slice_t : z_slice_move, \
        z_owned_string_array_t : z_string_array_move, \
        z_owned_string_t : z_string_move, \
        z_owned_subscriber_t : z_subscriber_move \
    )(&this_)

#define z_take(this_, x) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_take, \
        z_owned_bytes_writer_t : z_bytes_writer_take, \
        z_owned_closure_hello_t : z_closure_hello_take, \
        z_owned_closure_query_t : z_closure_query_take, \
        z_owned_closure_reply_t : z_closure_reply_take, \
        z_owned_closure_sample_t : z_closure_sample_take, \
        z_owned_condvar_t : z_condvar_take, \
        z_owned_config_t : z_config_take, \
        z_owned_encoding_t : z_encoding_take, \
        z_owned_fifo_handler_query_t : z_fifo_handler_query_take, \
        z_owned_fifo_handler_reply_t : z_fifo_handler_reply_take, \
        z_owned_fifo_handler_sample_t : z_fifo_handler_sample_take, \
        z_owned_hello_t : z_hello_take, \
        z_owned_keyexpr_t : z_keyexpr_take, \
        z_owned_mutex_t : z_mutex_take, \
        z_owned_publisher_t : z_publisher_take, \
        z_owned_query_t : z_query_take, \
        z_owned_queryable_t : z_queryable_take, \
        z_owned_reply_t : z_reply_take, \
        z_owned_reply_err_t : z_reply_err_take, \
        z_owned_ring_handler_query_t : z_ring_handler_query_take, \
        z_owned_ring_handler_reply_t : z_ring_handler_reply_take, \
        z_owned_ring_handler_sample_t : z_ring_handler_sample_take, \
        z_owned_sample_t : z_sample_take, \
        z_owned_session_t : z_session_take, \
        z_owned_slice_t : z_slice_take, \
        z_owned_string_array_t : z_string_array_take, \
        z_owned_string_t : z_string_take, \
        z_owned_subscriber_t : z_subscriber_take \
    )(&this_, x)

#define z_null(this_) \
    _Generic((x), \
        z_owned_bytes_t* : z_bytes_null, \
        z_owned_bytes_writer_t* : z_bytes_writer_null, \
        z_owned_closure_hello_t* : z_closure_hello_null, \
        z_owned_closure_query_t* : z_closure_query_null, \
        z_owned_closure_reply_t* : z_closure_reply_null, \
        z_owned_closure_sample_t* : z_closure_sample_null, \
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
        z_owned_reply_err_t* : z_reply_err_null, \
        z_owned_reply_t* : z_reply_null, \
        z_owned_ring_handler_query_t* : z_ring_handler_query_null, \
        z_owned_ring_handler_reply_t* : z_ring_handler_reply_null, \
        z_owned_ring_handler_sample_t* : z_ring_handler_sample_null, \
        z_owned_sample_t* : z_sample_null, \
        z_owned_session_t* : z_session_null, \
        z_owned_slice_t* : z_slice_null, \
        z_owned_string_array_t* : z_string_array_null, \
        z_owned_string_t* : z_string_null, \
        z_owned_subscriber_t* : z_subscriber_null, \
        z_owned_task_t* : z_task_null, \
        z_view_keyexpr_t* : z_view_keyexpr_null, \
        z_view_slice_t* : z_view_slice_null, \
        z_view_string_t* : z_view_string_null \
    )(this_)

#define z_check(this_) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_check, \
        z_owned_bytes_writer_t : z_bytes_writer_check, \
        z_owned_closure_hello_t : z_closure_hello_check, \
        z_owned_closure_query_t : z_closure_query_check, \
        z_owned_closure_reply_t : z_closure_reply_check, \
        z_owned_closure_sample_t : z_closure_sample_check, \
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
        z_owned_reply_err_t : z_reply_err_check, \
        z_owned_ring_handler_query_t : z_ring_handler_query_check, \
        z_owned_ring_handler_reply_t : z_ring_handler_reply_check, \
        z_owned_ring_handler_sample_t : z_ring_handler_sample_check, \
        z_owned_sample_t : z_sample_check, \
        z_owned_session_t : z_session_check, \
        z_owned_slice_t : z_slice_check, \
        z_owned_string_array_t : z_string_array_check, \
        z_owned_string_t : z_string_check, \
        z_owned_subscriber_t : z_subscriber_check, \
        z_owned_task_t : z_task_check, \
        z_view_keyexpr_t : z_view_keyexpr_check, \
        z_view_slice_t : z_view_slice_check, \
        z_view_string_t : z_view_string_check \
    )(&this_)

#define z_call(closure, hello) \
    _Generic((x), \
        const z_loaned_closure_hello_t* : z_closure_hello_call, \
        const z_loaned_closure_query_t* : z_closure_query_call, \
        const z_loaned_closure_reply_t* : z_closure_reply_call, \
        const z_loaned_closure_sample_t* : z_closure_sample_call \
    )(closure, hello)

#define z_closure(x, callback, dropper, ctx) \
    {{(x)->context = (void*)(ctx); (x)->call = (callback); (x)->drop = (dropper);}}

#define z_try_recv(this_, query) \
    _Generic((x), \
        const z_loaned_fifo_handler_query_t* : z_fifo_handler_query_try_recv, \
        const z_loaned_fifo_handler_reply_t* : z_fifo_handler_reply_try_recv, \
        const z_loaned_fifo_handler_sample_t* : z_fifo_handler_sample_try_recv, \
        const z_loaned_ring_handler_query_t* : z_ring_handler_query_try_recv, \
        const z_loaned_ring_handler_reply_t* : z_ring_handler_reply_try_recv, \
        const z_loaned_ring_handler_sample_t* : z_ring_handler_sample_try_recv \
    )(this_, query)

#define z_recv(this_, query) \
    _Generic((x), \
        const z_loaned_fifo_handler_query_t* : z_fifo_handler_query_recv, \
        const z_loaned_fifo_handler_reply_t* : z_fifo_handler_reply_recv, \
        const z_loaned_fifo_handler_sample_t* : z_fifo_handler_sample_recv, \
        const z_loaned_ring_handler_query_t* : z_ring_handler_query_recv, \
        const z_loaned_ring_handler_reply_t* : z_ring_handler_reply_recv, \
        const z_loaned_ring_handler_sample_t* : z_ring_handler_sample_recv \
    )(this_, query)
#else  // #ifndef __cplusplus


static inline z_moved_bytes_t z_bytes_move(z_owned_bytes_t* x) { return z_moved_bytes_t{x}; }
static inline z_moved_bytes_writer_t z_bytes_writer_move(z_owned_bytes_writer_t* x) { return z_moved_bytes_writer_t{x}; }
static inline z_moved_closure_hello_t z_closure_hello_move(z_owned_closure_hello_t* x) { return z_moved_closure_hello_t{x}; }
static inline z_moved_closure_query_t z_closure_query_move(z_owned_closure_query_t* x) { return z_moved_closure_query_t{x}; }
static inline z_moved_closure_reply_t z_closure_reply_move(z_owned_closure_reply_t* x) { return z_moved_closure_reply_t{x}; }
static inline z_moved_closure_sample_t z_closure_sample_move(z_owned_closure_sample_t* x) { return z_moved_closure_sample_t{x}; }
static inline z_moved_condvar_t z_condvar_move(z_owned_condvar_t* x) { return z_moved_condvar_t{x}; }
static inline z_moved_config_t z_config_move(z_owned_config_t* x) { return z_moved_config_t{x}; }
static inline z_moved_encoding_t z_encoding_move(z_owned_encoding_t* x) { return z_moved_encoding_t{x}; }
static inline z_moved_fifo_handler_query_t z_fifo_handler_query_move(z_owned_fifo_handler_query_t* x) { return z_moved_fifo_handler_query_t{x}; }
static inline z_moved_fifo_handler_reply_t z_fifo_handler_reply_move(z_owned_fifo_handler_reply_t* x) { return z_moved_fifo_handler_reply_t{x}; }
static inline z_moved_fifo_handler_sample_t z_fifo_handler_sample_move(z_owned_fifo_handler_sample_t* x) { return z_moved_fifo_handler_sample_t{x}; }
static inline z_moved_hello_t z_hello_move(z_owned_hello_t* x) { return z_moved_hello_t{x}; }
static inline z_moved_keyexpr_t z_keyexpr_move(z_owned_keyexpr_t* x) { return z_moved_keyexpr_t{x}; }
static inline z_moved_mutex_t z_mutex_move(z_owned_mutex_t* x) { return z_moved_mutex_t{x}; }
static inline z_moved_publisher_t z_publisher_move(z_owned_publisher_t* x) { return z_moved_publisher_t{x}; }
static inline z_moved_query_t z_query_move(z_owned_query_t* x) { return z_moved_query_t{x}; }
static inline z_moved_queryable_t z_queryable_move(z_owned_queryable_t* x) { return z_moved_queryable_t{x}; }
static inline z_moved_reply_t z_reply_move(z_owned_reply_t* x) { return z_moved_reply_t{x}; }
static inline z_moved_reply_err_t z_reply_err_move(z_owned_reply_err_t* x) { return z_moved_reply_err_t{x}; }
static inline z_moved_ring_handler_query_t z_ring_handler_query_move(z_owned_ring_handler_query_t* x) { return z_moved_ring_handler_query_t{x}; }
static inline z_moved_ring_handler_reply_t z_ring_handler_reply_move(z_owned_ring_handler_reply_t* x) { return z_moved_ring_handler_reply_t{x}; }
static inline z_moved_ring_handler_sample_t z_ring_handler_sample_move(z_owned_ring_handler_sample_t* x) { return z_moved_ring_handler_sample_t{x}; }
static inline z_moved_sample_t z_sample_move(z_owned_sample_t* x) { return z_moved_sample_t{x}; }
static inline z_moved_session_t z_session_move(z_owned_session_t* x) { return z_moved_session_t{x}; }
static inline z_moved_slice_t z_slice_move(z_owned_slice_t* x) { return z_moved_slice_t{x}; }
static inline z_moved_string_array_t z_string_array_move(z_owned_string_array_t* x) { return z_moved_string_array_t{x}; }
static inline z_moved_string_t z_string_move(z_owned_string_t* x) { return z_moved_string_t{x}; }
static inline z_moved_subscriber_t z_subscriber_move(z_owned_subscriber_t* x) { return z_moved_subscriber_t{x}; }



inline const z_loaned_bytes_t* z_loan(const z_owned_bytes_t& this_) { return z_bytes_loan(&this_); };
inline const z_loaned_bytes_writer_t* z_loan(const z_owned_bytes_writer_t& this_) { return z_bytes_writer_loan(&this_); };
inline const z_loaned_closure_hello_t* z_loan(const z_owned_closure_hello_t& closure) { return z_closure_hello_loan(&closure); };
inline const z_loaned_closure_query_t* z_loan(const z_owned_closure_query_t& closure) { return z_closure_query_loan(&closure); };
inline const z_loaned_closure_reply_t* z_loan(const z_owned_closure_reply_t& closure) { return z_closure_reply_loan(&closure); };
inline const z_loaned_closure_sample_t* z_loan(const z_owned_closure_sample_t& closure) { return z_closure_sample_loan(&closure); };
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
inline const z_loaned_reply_err_t* z_loan(const z_owned_reply_err_t& this_) { return z_reply_err_loan(&this_); };
inline const z_loaned_reply_t* z_loan(const z_owned_reply_t& this_) { return z_reply_loan(&this_); };
inline const z_loaned_ring_handler_query_t* z_loan(const z_owned_ring_handler_query_t& this_) { return z_ring_handler_query_loan(&this_); };
inline const z_loaned_ring_handler_reply_t* z_loan(const z_owned_ring_handler_reply_t& this_) { return z_ring_handler_reply_loan(&this_); };
inline const z_loaned_ring_handler_sample_t* z_loan(const z_owned_ring_handler_sample_t& this_) { return z_ring_handler_sample_loan(&this_); };
inline const z_loaned_sample_t* z_loan(const z_owned_sample_t& this_) { return z_sample_loan(&this_); };
inline const z_loaned_session_t* z_loan(const z_owned_session_t& this_) { return z_session_loan(&this_); };
inline const z_loaned_slice_t* z_loan(const z_owned_slice_t& this_) { return z_slice_loan(&this_); };
inline const z_loaned_string_array_t* z_loan(const z_owned_string_array_t& this_) { return z_string_array_loan(&this_); };
inline const z_loaned_string_t* z_loan(const z_owned_string_t& this_) { return z_string_loan(&this_); };
inline const z_loaned_subscriber_t* z_loan(const z_owned_subscriber_t& this_) { return z_subscriber_loan(&this_); };
inline const z_loaned_keyexpr_t* z_loan(const z_view_keyexpr_t& this_) { return z_view_keyexpr_loan(&this_); };
inline const z_loaned_slice_t* z_loan(const z_view_slice_t& this_) { return z_view_slice_loan(&this_); };
inline const z_loaned_string_t* z_loan(const z_view_string_t& this_) { return z_view_string_loan(&this_); };


inline z_loaned_bytes_t* z_loan_mut(z_owned_bytes_t& this_) { return z_bytes_loan_mut(&this_); };
inline z_loaned_bytes_writer_t* z_loan_mut(z_owned_bytes_writer_t& this_) { return z_bytes_writer_loan_mut(&this_); };
inline z_loaned_condvar_t* z_loan_mut(z_owned_condvar_t& this_) { return z_condvar_loan_mut(&this_); };
inline z_loaned_config_t* z_loan_mut(z_owned_config_t& this_) { return z_config_loan_mut(&this_); };
inline z_loaned_encoding_t* z_loan_mut(z_owned_encoding_t& this_) { return z_encoding_loan_mut(&this_); };
inline z_loaned_mutex_t* z_loan_mut(z_owned_mutex_t& this_) { return z_mutex_loan_mut(&this_); };
inline z_loaned_publisher_t* z_loan_mut(z_owned_publisher_t& this_) { return z_publisher_loan_mut(&this_); };
inline z_loaned_string_array_t* z_loan_mut(z_owned_string_array_t& this_) { return z_string_array_loan_mut(&this_); };


inline void z_drop(z_moved_bytes_t this_) { z_bytes_drop(this_); };
inline void z_drop(z_moved_bytes_writer_t this_) { z_bytes_writer_drop(this_); };
inline void z_drop(z_moved_closure_hello_t _closure) { z_closure_hello_drop(_closure); };
inline void z_drop(z_moved_closure_query_t closure) { z_closure_query_drop(closure); };
inline void z_drop(z_moved_closure_reply_t closure) { z_closure_reply_drop(closure); };
inline void z_drop(z_moved_closure_sample_t closure) { z_closure_sample_drop(closure); };
inline void z_drop(z_moved_condvar_t this_) { z_condvar_drop(this_); };
inline void z_drop(z_moved_config_t this_) { z_config_drop(this_); };
inline void z_drop(z_moved_encoding_t this_) { z_encoding_drop(this_); };
inline void z_drop(z_moved_fifo_handler_query_t this_) { z_fifo_handler_query_drop(this_); };
inline void z_drop(z_moved_fifo_handler_reply_t this_) { z_fifo_handler_reply_drop(this_); };
inline void z_drop(z_moved_fifo_handler_sample_t this_) { z_fifo_handler_sample_drop(this_); };
inline void z_drop(z_moved_hello_t this_) { z_hello_drop(this_); };
inline void z_drop(z_moved_keyexpr_t this_) { z_keyexpr_drop(this_); };
inline void z_drop(z_moved_mutex_t this_) { z_mutex_drop(this_); };
inline void z_drop(z_moved_publisher_t this_) { z_publisher_drop(this_); };
inline void z_drop(z_moved_query_t this_) { z_query_drop(this_); };
inline void z_drop(z_moved_queryable_t this_) { z_queryable_drop(this_); };
inline void z_drop(z_moved_reply_t this_) { z_reply_drop(this_); };
inline void z_drop(z_moved_reply_err_t this_) { z_reply_err_drop(this_); };
inline void z_drop(z_moved_ring_handler_query_t this_) { z_ring_handler_query_drop(this_); };
inline void z_drop(z_moved_ring_handler_reply_t this_) { z_ring_handler_reply_drop(this_); };
inline void z_drop(z_moved_ring_handler_sample_t this_) { z_ring_handler_sample_drop(this_); };
inline void z_drop(z_moved_sample_t this_) { z_sample_drop(this_); };
inline void z_drop(z_moved_session_t this_) { z_session_drop(this_); };
inline void z_drop(z_moved_slice_t this_) { z_slice_drop(this_); };
inline void z_drop(z_moved_string_array_t this_) { z_string_array_drop(this_); };
inline void z_drop(z_moved_string_t this_) { z_string_drop(this_); };
inline void z_drop(z_moved_subscriber_t this_) { z_subscriber_drop(this_); };


inline z_moved_bytes_t z_move(z_owned_bytes_t& this_) { return z_bytes_move(&this_); };
inline z_moved_bytes_writer_t z_move(z_owned_bytes_writer_t& this_) { return z_bytes_writer_move(&this_); };
inline z_moved_closure_hello_t z_move(z_owned_closure_hello_t& _closure) { return z_closure_hello_move(&_closure); };
inline z_moved_closure_query_t z_move(z_owned_closure_query_t& closure) { return z_closure_query_move(&closure); };
inline z_moved_closure_reply_t z_move(z_owned_closure_reply_t& closure) { return z_closure_reply_move(&closure); };
inline z_moved_closure_sample_t z_move(z_owned_closure_sample_t& closure) { return z_closure_sample_move(&closure); };
inline z_moved_condvar_t z_move(z_owned_condvar_t& this_) { return z_condvar_move(&this_); };
inline z_moved_config_t z_move(z_owned_config_t& this_) { return z_config_move(&this_); };
inline z_moved_encoding_t z_move(z_owned_encoding_t& this_) { return z_encoding_move(&this_); };
inline z_moved_fifo_handler_query_t z_move(z_owned_fifo_handler_query_t& this_) { return z_fifo_handler_query_move(&this_); };
inline z_moved_fifo_handler_reply_t z_move(z_owned_fifo_handler_reply_t& this_) { return z_fifo_handler_reply_move(&this_); };
inline z_moved_fifo_handler_sample_t z_move(z_owned_fifo_handler_sample_t& this_) { return z_fifo_handler_sample_move(&this_); };
inline z_moved_hello_t z_move(z_owned_hello_t& this_) { return z_hello_move(&this_); };
inline z_moved_keyexpr_t z_move(z_owned_keyexpr_t& this_) { return z_keyexpr_move(&this_); };
inline z_moved_mutex_t z_move(z_owned_mutex_t& this_) { return z_mutex_move(&this_); };
inline z_moved_publisher_t z_move(z_owned_publisher_t& this_) { return z_publisher_move(&this_); };
inline z_moved_query_t z_move(z_owned_query_t& this_) { return z_query_move(&this_); };
inline z_moved_queryable_t z_move(z_owned_queryable_t& this_) { return z_queryable_move(&this_); };
inline z_moved_reply_t z_move(z_owned_reply_t& this_) { return z_reply_move(&this_); };
inline z_moved_reply_err_t z_move(z_owned_reply_err_t& this_) { return z_reply_err_move(&this_); };
inline z_moved_ring_handler_query_t z_move(z_owned_ring_handler_query_t& this_) { return z_ring_handler_query_move(&this_); };
inline z_moved_ring_handler_reply_t z_move(z_owned_ring_handler_reply_t& this_) { return z_ring_handler_reply_move(&this_); };
inline z_moved_ring_handler_sample_t z_move(z_owned_ring_handler_sample_t& this_) { return z_ring_handler_sample_move(&this_); };
inline z_moved_sample_t z_move(z_owned_sample_t& this_) { return z_sample_move(&this_); };
inline z_moved_session_t z_move(z_owned_session_t& this_) { return z_session_move(&this_); };
inline z_moved_slice_t z_move(z_owned_slice_t& this_) { return z_slice_move(&this_); };
inline z_moved_string_array_t z_move(z_owned_string_array_t& this_) { return z_string_array_move(&this_); };
inline z_moved_string_t z_move(z_owned_string_t& this_) { return z_string_move(&this_); };
inline z_moved_subscriber_t z_move(z_owned_subscriber_t& this_) { return z_subscriber_move(&this_); };


inline void z_take(z_owned_bytes_t& this_, z_moved_bytes_t x) {
    z_bytes_take(&this_, x);
};
inline void z_take(z_owned_bytes_writer_t& this_, z_moved_bytes_writer_t x) {
    z_bytes_writer_take(&this_, x);
};
inline void z_take(z_owned_closure_hello_t& _closure, z_moved_closure_hello_t x) {
    z_closure_hello_take(&_closure, x);
};
inline void z_take(z_owned_closure_query_t& closure, z_moved_closure_query_t x) {
    z_closure_query_take(&closure, x);
};
inline void z_take(z_owned_closure_reply_t& closure, z_moved_closure_reply_t x) {
    z_closure_reply_take(&closure, x);
};
inline void z_take(z_owned_closure_sample_t& closure, z_moved_closure_sample_t x) {
    z_closure_sample_take(&closure, x);
};
inline void z_take(z_owned_condvar_t& this_, z_moved_condvar_t x) {
    z_condvar_take(&this_, x);
};
inline void z_take(z_owned_config_t& this_, z_moved_config_t x) {
    z_config_take(&this_, x);
};
inline void z_take(z_owned_encoding_t& this_, z_moved_encoding_t x) {
    z_encoding_take(&this_, x);
};
inline void z_take(z_owned_fifo_handler_query_t& this_, z_moved_fifo_handler_query_t x) {
    z_fifo_handler_query_take(&this_, x);
};
inline void z_take(z_owned_fifo_handler_reply_t& this_, z_moved_fifo_handler_reply_t x) {
    z_fifo_handler_reply_take(&this_, x);
};
inline void z_take(z_owned_fifo_handler_sample_t& this_, z_moved_fifo_handler_sample_t x) {
    z_fifo_handler_sample_take(&this_, x);
};
inline void z_take(z_owned_hello_t& this_, z_moved_hello_t x) {
    z_hello_take(&this_, x);
};
inline void z_take(z_owned_keyexpr_t& this_, z_moved_keyexpr_t x) {
    z_keyexpr_take(&this_, x);
};
inline void z_take(z_owned_mutex_t& this_, z_moved_mutex_t x) {
    z_mutex_take(&this_, x);
};
inline void z_take(z_owned_publisher_t& this_, z_moved_publisher_t x) {
    z_publisher_take(&this_, x);
};
inline void z_take(z_owned_query_t& this_, z_moved_query_t x) {
    z_query_take(&this_, x);
};
inline void z_take(z_owned_queryable_t& this_, z_moved_queryable_t x) {
    z_queryable_take(&this_, x);
};
inline void z_take(z_owned_reply_t& this_, z_moved_reply_t x) {
    z_reply_take(&this_, x);
};
inline void z_take(z_owned_reply_err_t& this_, z_moved_reply_err_t x) {
    z_reply_err_take(&this_, x);
};
inline void z_take(z_owned_ring_handler_query_t& this_, z_moved_ring_handler_query_t x) {
    z_ring_handler_query_take(&this_, x);
};
inline void z_take(z_owned_ring_handler_reply_t& this_, z_moved_ring_handler_reply_t x) {
    z_ring_handler_reply_take(&this_, x);
};
inline void z_take(z_owned_ring_handler_sample_t& this_, z_moved_ring_handler_sample_t x) {
    z_ring_handler_sample_take(&this_, x);
};
inline void z_take(z_owned_sample_t& this_, z_moved_sample_t x) {
    z_sample_take(&this_, x);
};
inline void z_take(z_owned_session_t& this_, z_moved_session_t x) {
    z_session_take(&this_, x);
};
inline void z_take(z_owned_slice_t& this_, z_moved_slice_t x) {
    z_slice_take(&this_, x);
};
inline void z_take(z_owned_string_array_t& this_, z_moved_string_array_t x) {
    z_string_array_take(&this_, x);
};
inline void z_take(z_owned_string_t& this_, z_moved_string_t x) {
    z_string_take(&this_, x);
};
inline void z_take(z_owned_subscriber_t& this_, z_moved_subscriber_t x) {
    z_subscriber_take(&this_, x);
};


inline void z_null(z_owned_bytes_t* this_) { z_bytes_null(this_); };
inline void z_null(z_owned_bytes_writer_t* this_) { z_bytes_writer_null(this_); };
inline void z_null(z_owned_closure_hello_t* this_) { z_closure_hello_null(this_); };
inline void z_null(z_owned_closure_query_t* this_) { z_closure_query_null(this_); };
inline void z_null(z_owned_closure_reply_t* this_) { z_closure_reply_null(this_); };
inline void z_null(z_owned_closure_sample_t* this_) { z_closure_sample_null(this_); };
inline void z_null(z_owned_condvar_t* this_) { z_condvar_null(this_); };
inline void z_null(z_owned_config_t* this_) { z_config_null(this_); };
inline void z_null(z_owned_encoding_t* this_) { z_encoding_null(this_); };
inline void z_null(z_owned_fifo_handler_query_t* this_) { z_fifo_handler_query_null(this_); };
inline void z_null(z_owned_fifo_handler_reply_t* this_) { z_fifo_handler_reply_null(this_); };
inline void z_null(z_owned_fifo_handler_sample_t* this_) { z_fifo_handler_sample_null(this_); };
inline void z_null(z_owned_hello_t* this_) { z_hello_null(this_); };
inline void z_null(z_owned_keyexpr_t* this_) { z_keyexpr_null(this_); };
inline void z_null(z_owned_mutex_t* this_) { z_mutex_null(this_); };
inline void z_null(z_owned_publisher_t* this_) { z_publisher_null(this_); };
inline void z_null(z_owned_query_t* this_) { z_query_null(this_); };
inline void z_null(z_owned_queryable_t* this_) { z_queryable_null(this_); };
inline void z_null(z_owned_reply_err_t* this_) { z_reply_err_null(this_); };
inline void z_null(z_owned_reply_t* this_) { z_reply_null(this_); };
inline void z_null(z_owned_ring_handler_query_t* this_) { z_ring_handler_query_null(this_); };
inline void z_null(z_owned_ring_handler_reply_t* this_) { z_ring_handler_reply_null(this_); };
inline void z_null(z_owned_ring_handler_sample_t* this_) { z_ring_handler_sample_null(this_); };
inline void z_null(z_owned_sample_t* this_) { z_sample_null(this_); };
inline void z_null(z_owned_session_t* this_) { z_session_null(this_); };
inline void z_null(z_owned_slice_t* this_) { z_slice_null(this_); };
inline void z_null(z_owned_string_array_t* this_) { z_string_array_null(this_); };
inline void z_null(z_owned_string_t* this_) { z_string_null(this_); };
inline void z_null(z_owned_subscriber_t* this_) { z_subscriber_null(this_); };
inline void z_null(z_owned_task_t* this_) { z_task_null(this_); };
inline void z_null(z_view_keyexpr_t* this_) { z_view_keyexpr_null(this_); };
inline void z_null(z_view_slice_t* this_) { z_view_slice_null(this_); };
inline void z_null(z_view_string_t* this_) { z_view_string_null(this_); };


inline bool z_check(const z_owned_bytes_t& this_) { return z_bytes_check(&this_); };
inline bool z_check(const z_owned_bytes_writer_t& this_) { return z_bytes_writer_check(&this_); };
inline bool z_check(const z_owned_closure_hello_t& this_) { return z_closure_hello_check(&this_); };
inline bool z_check(const z_owned_closure_query_t& this_) { return z_closure_query_check(&this_); };
inline bool z_check(const z_owned_closure_reply_t& this_) { return z_closure_reply_check(&this_); };
inline bool z_check(const z_owned_closure_sample_t& this_) { return z_closure_sample_check(&this_); };
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
inline bool z_check(const z_owned_reply_err_t& this_) { return z_reply_err_check(&this_); };
inline bool z_check(const z_owned_ring_handler_query_t& this_) { return z_ring_handler_query_check(&this_); };
inline bool z_check(const z_owned_ring_handler_reply_t& this_) { return z_ring_handler_reply_check(&this_); };
inline bool z_check(const z_owned_ring_handler_sample_t& this_) { return z_ring_handler_sample_check(&this_); };
inline bool z_check(const z_owned_sample_t& this_) { return z_sample_check(&this_); };
inline bool z_check(const z_owned_session_t& this_) { return z_session_check(&this_); };
inline bool z_check(const z_owned_slice_t& this_) { return z_slice_check(&this_); };
inline bool z_check(const z_owned_string_array_t& this_) { return z_string_array_check(&this_); };
inline bool z_check(const z_owned_string_t& this_) { return z_string_check(&this_); };
inline bool z_check(const z_owned_subscriber_t& this_) { return z_subscriber_check(&this_); };
inline bool z_check(const z_owned_task_t& this_) { return z_task_check(&this_); };
inline bool z_check(const z_view_keyexpr_t& this_) { return z_view_keyexpr_check(&this_); };
inline bool z_check(const z_view_slice_t& this_) { return z_view_slice_check(&this_); };
inline bool z_check(const z_view_string_t& this_) { return z_view_string_check(&this_); };


inline void z_call(const z_loaned_closure_hello_t* closure, const z_loaned_hello_t* hello) {
    z_closure_hello_call(closure, hello);
};
inline void z_call(const z_loaned_closure_query_t* closure, const z_loaned_query_t* query) {
    z_closure_query_call(closure, query);
};
inline void z_call(const z_loaned_closure_reply_t* closure, const z_loaned_reply_t* reply) {
    z_closure_reply_call(closure, reply);
};
inline void z_call(const z_loaned_closure_sample_t* closure, const z_loaned_sample_t* sample) {
    z_closure_sample_call(closure, sample);
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
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_query_t> { typedef z_owned_closure_query_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_query_t> { typedef z_loaned_closure_query_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_reply_t> { typedef z_owned_closure_reply_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_reply_t> { typedef z_loaned_closure_reply_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_closure_sample_t> { typedef z_owned_closure_sample_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_closure_sample_t> { typedef z_loaned_closure_sample_t type; };
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
template<> struct z_loaned_to_owned_type_t<z_loaned_slice_t> { typedef z_owned_slice_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_slice_t> { typedef z_loaned_slice_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_string_array_t> { typedef z_owned_string_array_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_string_array_t> { typedef z_loaned_string_array_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_string_t> { typedef z_owned_string_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_string_t> { typedef z_loaned_string_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_subscriber_t> { typedef z_owned_subscriber_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_subscriber_t> { typedef z_loaned_subscriber_t type; };
template<> struct z_loaned_to_owned_type_t<z_loaned_mutex_t> { typedef z_owned_mutex_t type; };
template<> struct z_owned_to_loaned_type_t<z_owned_mutex_t> { typedef z_loaned_mutex_t type; };
#endif  // #ifndef __cplusplus

static inline void z_bytes_take(z_owned_bytes_t* this_, z_moved_bytes_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_bytes_writer_take(z_owned_bytes_writer_t* this_, z_moved_bytes_writer_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_closure_hello_take(z_owned_closure_hello_t* _closure, z_moved_closure_hello_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_closure_query_take(z_owned_closure_query_t* closure, z_moved_closure_query_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_closure_reply_take(z_owned_closure_reply_t* closure, z_moved_closure_reply_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_closure_sample_take(z_owned_closure_sample_t* closure, z_moved_closure_sample_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_condvar_take(z_owned_condvar_t* this_, z_moved_condvar_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_config_take(z_owned_config_t* this_, z_moved_config_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_encoding_take(z_owned_encoding_t* this_, z_moved_encoding_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_fifo_handler_query_take(z_owned_fifo_handler_query_t* this_, z_moved_fifo_handler_query_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_fifo_handler_reply_take(z_owned_fifo_handler_reply_t* this_, z_moved_fifo_handler_reply_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_fifo_handler_sample_take(z_owned_fifo_handler_sample_t* this_, z_moved_fifo_handler_sample_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_hello_take(z_owned_hello_t* this_, z_moved_hello_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_keyexpr_take(z_owned_keyexpr_t* this_, z_moved_keyexpr_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_mutex_take(z_owned_mutex_t* this_, z_moved_mutex_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_publisher_take(z_owned_publisher_t* this_, z_moved_publisher_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_query_take(z_owned_query_t* this_, z_moved_query_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_queryable_take(z_owned_queryable_t* this_, z_moved_queryable_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_reply_take(z_owned_reply_t* this_, z_moved_reply_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_reply_err_take(z_owned_reply_err_t* this_, z_moved_reply_err_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_ring_handler_query_take(z_owned_ring_handler_query_t* this_, z_moved_ring_handler_query_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_ring_handler_reply_take(z_owned_ring_handler_reply_t* this_, z_moved_ring_handler_reply_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_ring_handler_sample_take(z_owned_ring_handler_sample_t* this_, z_moved_ring_handler_sample_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_sample_take(z_owned_sample_t* this_, z_moved_sample_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_session_take(z_owned_session_t* this_, z_moved_session_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_slice_take(z_owned_slice_t* this_, z_moved_slice_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_string_array_take(z_owned_string_array_t* this_, z_moved_string_array_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_string_take(z_owned_string_t* this_, z_moved_string_t x) { *this_ = *x._ptr; z_null(x._ptr); }
static inline void z_subscriber_take(z_owned_subscriber_t* this_, z_moved_subscriber_t x) { *this_ = *x._ptr; z_null(x._ptr); }


