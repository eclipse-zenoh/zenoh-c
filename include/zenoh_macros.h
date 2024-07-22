#pragma once

// clang-format off
#ifndef __cplusplus


#define z_loan(x) \
    _Generic((x), \
        z_owned_closure_hello_t : z_closure_hello_loan, \
        z_owned_closure_query_t : z_closure_query_loan, \
        z_owned_closure_reply_t : z_closure_reply_loan, \
        z_owned_closure_sample_t : z_closure_sample_loan, \
        z_owned_closure_zid_t : z_closure_zid_loan \
    )(&x)

#define z_loan_mut(x) \
    _Generic((x) \
    )(&x)

#define z_drop(x) \
    _Generic((x), \
        z_owned_closure_hello_t* : z_closure_hello_drop, \
        z_owned_closure_query_t* : z_closure_query_drop, \
        z_owned_closure_reply_t* : z_closure_reply_drop, \
        z_owned_closure_sample_t* : z_closure_sample_drop, \
        z_owned_closure_zid_t* : z_closure_zid_drop \
    )(x)

#define z_move(x) (&x)

#define z_null(x) \
    _Generic((x), \
        z_owned_closure_hello_t* : z_closure_hello_null, \
        z_owned_closure_query_t* : z_closure_query_null, \
        z_owned_closure_reply_t* : z_closure_reply_null, \
        z_owned_closure_sample_t* : z_closure_sample_null, \
        z_owned_closure_zid_t* : z_closure_zid_null \
    )(x)

#define z_check(x) \
    _Generic((x), \
        z_owned_closure_hello_t : z_closure_hello_check, \
        z_owned_closure_query_t : z_closure_query_check, \
        z_owned_closure_reply_t : z_closure_reply_check, \
        z_owned_closure_sample_t : z_closure_sample_check, \
        z_owned_closure_zid_t : z_closure_zid_check \
    )(&x)

#define z_call(x) \
    _Generic((x) \
    )(x)

#define z_closure(x, callback, dropper, ctx) \
    {{(x)->context = (void*)(ctx); (x)->call = (callback); (x)->drop = (dropper);}}

#define z_try_recv(x) \
    _Generic((x) \
    )(x)

#define z_recv(x) \
    _Generic((x) \
    )(x)
#else  // #ifndef __cplusplus



inline const z_loaned_closure_hello_t* z_loan(const z_owned_closure_hello_t& closure) { return z_closure_hello_loan(&closure); };
inline const z_loaned_closure_query_t* z_loan(const z_owned_closure_query_t& closure) { return z_closure_query_loan(&closure); };
inline const z_loaned_closure_reply_t* z_loan(const z_owned_closure_reply_t& closure) { return z_closure_reply_loan(&closure); };
inline const z_loaned_closure_sample_t* z_loan(const z_owned_closure_sample_t& closure) { return z_closure_sample_loan(&closure); };
inline const z_loaned_closure_zid_t* z_loan(const z_owned_closure_zid_t& closure) { return z_closure_zid_loan(&closure); };




inline void z_drop(z_owned_closure_hello_t* closure) { return z_closure_hello_drop(closure); };
inline void z_drop(z_owned_closure_query_t* closure) { return z_closure_query_drop(closure); };
inline void z_drop(z_owned_closure_reply_t* closure) { return z_closure_reply_drop(closure); };
inline void z_drop(z_owned_closure_sample_t* closure) { return z_closure_sample_drop(closure); };
inline void z_drop(z_owned_closure_zid_t* closure) { return z_closure_zid_drop(closure); };


inline z_owned_closure_hello_t* z_move(z_owned_closure_hello_t& closure) { return (&closure); };
inline z_owned_closure_query_t* z_move(z_owned_closure_query_t& closure) { return (&closure); };
inline z_owned_closure_reply_t* z_move(z_owned_closure_reply_t& closure) { return (&closure); };
inline z_owned_closure_sample_t* z_move(z_owned_closure_sample_t& closure) { return (&closure); };
inline z_owned_closure_zid_t* z_move(z_owned_closure_zid_t& closure) { return (&closure); };


inline void z_null(z_owned_closure_hello_t* this_) { return z_closure_hello_null(this_); };
inline void z_null(z_owned_closure_query_t* this_) { return z_closure_query_null(this_); };
inline void z_null(z_owned_closure_reply_t* this_) { return z_closure_reply_null(this_); };
inline void z_null(z_owned_closure_sample_t* this_) { return z_closure_sample_null(this_); };
inline void z_null(z_owned_closure_zid_t* this_) { return z_closure_zid_null(this_); };


inline bool z_check(const z_owned_closure_hello_t& this_) { return z_closure_hello_check(&this_); };
inline bool z_check(const z_owned_closure_query_t& this_) { return z_closure_query_check(&this_); };
inline bool z_check(const z_owned_closure_reply_t& this_) { return z_closure_reply_check(&this_); };
inline bool z_check(const z_owned_closure_sample_t& this_) { return z_closure_sample_check(&this_); };
inline bool z_check(const z_owned_closure_zid_t& this_) { return z_closure_zid_check(&this_); };









template<class T> struct z_loaned_to_owned_type_t {};
template<class T> struct z_owned_to_loaned_type_t {};
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
#endif  // #ifndef __cplusplus