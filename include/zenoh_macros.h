#define z_loan(x) _Generic((x), z_owned_session_t \
                           : z_session_loan,      \
                             z_owned_keyexpr_t    \
                           : z_keyexpr_loan,      \
                             z_owned_config_t     \
                           : z_config_loan,       \
                             z_owned_info_t       \
                           : z_info_loan,         \
                             z_owned_encoding_t   \
                           : z_encoding_loan)(&x)
#define z_drop(x) _Generic((x), z_owned_session_t    \
                           : z_close,                \
                             z_owned_keyexpr_t       \
                           : z_keyexpr_drop,         \
                             z_owned_config_t        \
                           : z_config_drop,          \
                             z_owned_info_t          \
                           : z_info_drop,            \
                             z_owned_subscriber_t    \
                           : z_undeclare_subscriber, \
                             z_owned_queryable_t     \
                           : z_undeclare_queryable,  \
                             z_owned_encoding_t      \
                           : z_encoding_drop)(&x)
#define z_check(x) _Generic((x), z_owned_session_t    \
                            : z_session_check,        \
                              z_owned_keyexpr_t       \
                            : z_keyexpr_check,        \
                              z_keyexpr_t             \
                            : z_loaned_keyexpr_check, \
                              z_owned_config_t        \
                            : z_config_check,         \
                              z_bytes_t               \
                            : z_bytes_check,          \
                              z_owned_info_t          \
                            : z_info_check,           \
                              z_owned_subscriber_t    \
                            : z_subscriber_check,     \
                              z_owned_queryable_t     \
                            : z_queryable_check,      \
                              z_owned_encoding_t      \
                            : z_encoding_check)(&x)

#define z_stateless_closure(callback)         \
  {                                           \
    .call = callback, .drop = 0, .context = 0 \
  }
#define _z_closure_error(callback, drop) "Invalid overload of the closure macro, which either takes a single argument (stateless callback), or 3 (callback, drop, context)"
#define z_stateful_closure(callback, droper, ctx)    \
  {                                                  \
    .call = callback, .drop = droper, .context = ctx \
  }
#define _z_closure_overload_selector_helper(arg1, arg2, arg3, arg4, ...) arg4
#define _z_closure_overload_selector(...) _z_closure_overload_selector_helper(__VA_ARGS__, z_stateful_closure, _z_closure_error, z_stateless_closure)
#define z_closure(...) _z_closure_overload_selector(__VA_ARGS__)(__VA_ARGS__)
#define z_move(x) (&x)
