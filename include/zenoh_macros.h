#pragma once
// clang-format off


#ifndef __cplusplus

static inline z_moved_closure_hello_t* z_closure_hello_move(z_owned_closure_hello_t* x) { return (z_moved_closure_hello_t*)(x); }
static inline z_moved_closure_query_t* z_closure_query_move(z_owned_closure_query_t* x) { return (z_moved_closure_query_t*)(x); }
static inline z_moved_closure_reply_t* z_closure_reply_move(z_owned_closure_reply_t* x) { return (z_moved_closure_reply_t*)(x); }
static inline z_moved_closure_sample_t* z_closure_sample_move(z_owned_closure_sample_t* x) { return (z_moved_closure_sample_t*)(x); }
static inline z_moved_closure_zid_t* z_closure_zid_move(z_owned_closure_zid_t* x) { return (z_moved_closure_zid_t*)(x); }
static inline zc_moved_closure_log_t* zc_closure_log_move(zc_owned_closure_log_t* x) { return (zc_moved_closure_log_t*)(x); }


#define z_loan(closure) \
    _Generic((closure), \
        z_owned_closure_hello_t : z_closure_hello_loan, \
        z_owned_closure_query_t : z_closure_query_loan, \
        z_owned_closure_reply_t : z_closure_reply_loan, \
        z_owned_closure_sample_t : z_closure_sample_loan, \
        z_owned_closure_zid_t : z_closure_zid_loan, \
        zc_owned_closure_log_t : zc_closure_log_loan \
    )(&closure)

#define z_loan_mut(closure) \
    _Generic((closure), \
        z_owned_closure_hello_t : z_closure_hello_loan_mut, \
        z_owned_closure_query_t : z_closure_query_loan_mut, \
        z_owned_closure_reply_t : z_closure_reply_loan_mut, \
        z_owned_closure_sample_t : z_closure_sample_loan_mut \
    )(&closure)

#define z_drop(this_) \
    _Generic((this_), \
        z_moved_closure_hello_t* : z_closure_hello_drop, \
        z_moved_closure_query_t* : z_closure_query_drop, \
        z_moved_closure_reply_t* : z_closure_reply_drop, \
        z_moved_closure_sample_t* : z_closure_sample_drop, \
        z_moved_closure_zid_t* : z_closure_zid_drop, \
        zc_moved_closure_log_t* : zc_closure_log_drop \
    )(this_)

#define z_move(this_) \
    _Generic((this_), \
        z_owned_closure_hello_t : z_closure_hello_move, \
        z_owned_closure_query_t : z_closure_query_move, \
        z_owned_closure_reply_t : z_closure_reply_move, \
        z_owned_closure_sample_t : z_closure_sample_move, \
        z_owned_closure_zid_t : z_closure_zid_move, \
        zc_owned_closure_log_t : zc_closure_log_move \
    )(&this_)

#define z_internal_null(this_) \
    _Generic((this_), \
        z_owned_closure_hello_t* : z_internal_closure_hello_null, \
        z_owned_closure_query_t* : z_internal_closure_query_null, \
        z_owned_closure_reply_t* : z_internal_closure_reply_null, \
        z_owned_closure_sample_t* : z_internal_closure_sample_null, \
        z_owned_closure_zid_t* : z_internal_closure_zid_null, \
        zc_owned_closure_log_t* : zc_internal_closure_log_null \
    )(this_)

static inline void z_closure_hello_take(z_owned_closure_hello_t* this_, z_moved_closure_hello_t* x) { *this_ = x->_this; z_internal_closure_hello_null(&x->_this); }
static inline void z_closure_query_take(z_owned_closure_query_t* closure_, z_moved_closure_query_t* x) { *closure_ = x->_this; z_internal_closure_query_null(&x->_this); }
static inline void z_closure_reply_take(z_owned_closure_reply_t* closure_, z_moved_closure_reply_t* x) { *closure_ = x->_this; z_internal_closure_reply_null(&x->_this); }
static inline void z_closure_sample_take(z_owned_closure_sample_t* closure_, z_moved_closure_sample_t* x) { *closure_ = x->_this; z_internal_closure_sample_null(&x->_this); }
static inline void z_closure_zid_take(z_owned_closure_zid_t* closure_, z_moved_closure_zid_t* x) { *closure_ = x->_this; z_internal_closure_zid_null(&x->_this); }
static inline void zc_closure_log_take(zc_owned_closure_log_t* closure_, zc_moved_closure_log_t* x) { *closure_ = x->_this; zc_internal_closure_log_null(&x->_this); }


#define z_take(this_, x) \
    _Generic((this_), \
        z_owned_closure_hello_t* : z_closure_hello_take, \
        z_owned_closure_query_t* : z_closure_query_take, \
        z_owned_closure_reply_t* : z_closure_reply_take, \
        z_owned_closure_sample_t* : z_closure_sample_take, \
        z_owned_closure_zid_t* : z_closure_zid_take, \
        zc_owned_closure_log_t* : zc_closure_log_take \
    )(this_, x)

