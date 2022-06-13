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

#define _z_closure_overloader(callback, droper, ctx, ...) \
  {                                                       \
    .call = callback, .drop = droper, .context = ctx      \
  }
#define z_closure(...) _z_closure_overloader(__VA_ARGS__, 0, 0)
#define z_move(x) (&x)
