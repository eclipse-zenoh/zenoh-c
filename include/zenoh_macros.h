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
#define z_free(x) _Generic((x), z_owned_session_t     \
                           : z_close,                 \
                             z_owned_keyexpr_t        \
                           : z_keyexpr_free,          \
                             z_owned_config_t         \
                           : z_config_free,           \
                             z_owned_info_t           \
                           : z_info_free,             \
                             z_owned_subscriber_t     \
                           : z_unregister_subscriber, \
                             z_owned_encoding_t       \
                           : z_encoding_free)(&x)
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
                              z_owned_encoding_t      \
                            : z_encoding_check)(&x)

#define z_move(x) (&x)
