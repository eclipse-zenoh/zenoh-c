#define z_loan(x)                                   \
    _Generic((x), z_owned_session_t                 \
             : z_session_loan, z_owned_keyexpr_t    \
             : z_keyexpr_loan, z_owned_config_t     \
             : z_config_loan, z_owned_publisher_t   \
             : z_publisher_loan, z_owned_encoding_t \
             : z_encoding_loan, z_owned_hello_t     \
             : z_hello_loan)(&x)
#define z_drop(x)                                                    \
    _Generic((x), z_owned_session_t                                  \
             : z_close, z_owned_keyexpr_t                            \
             : z_keyexpr_drop, z_owned_config_t                      \
             : z_config_drop, z_owned_scouting_config_t              \
             : z_scouting_config_drop, z_owned_pull_subscriber_t     \
             : z_undeclare_pull_subscriber, z_owned_subscriber_t     \
             : z_undeclare_subscriber, z_owned_queryable_t           \
             : z_undeclare_queryable, z_owned_encoding_t             \
             : z_encoding_drop, z_owned_reply_t                      \
             : z_reply_drop, z_owned_closure_sample_t                \
             : z_closure_sample_drop, z_owned_closure_query_t        \
             : z_closure_query_drop, z_owned_closure_reply_t         \
             : z_closure_reply_drop, z_owned_closure_hello_t         \
             : z_closure_hello_drop, z_owned_reply_channel_closure_t \
             : z_reply_channel_closure_drop, z_owned_reply_channel_t \
             : z_reply_channel_drop)(&x)
#define z_check(x)                                           \
    _Generic((x), z_owned_session_t                          \
             : z_session_check, z_owned_publisher_t          \
             : z_publisher_check, z_owned_keyexpr_t          \
             : z_keyexpr_check, z_keyexpr_t                  \
             : z_keyexpr_is_initialized, z_owned_config_t    \
             : z_config_check, z_owned_scouting_config_t     \
             : z_scouting_config_check, z_bytes_t            \
             : z_bytes_check, z_owned_subscriber_t           \
             : z_subscriber_check, z_owned_pull_subscriber_t \
             : z_pull_subscriber_check, z_owned_queryable_t  \
             : z_queryable_check, z_owned_encoding_t         \
             : z_encoding_check, z_owned_reply_t             \
             : z_reply_check)(&x)

#define z_call(x, ...)                                               \
    _Generic((x), z_owned_closure_sample_t                           \
             : z_closure_sample_call, z_owned_closure_query_t        \
             : z_closure_query_call, z_owned_closure_reply_t         \
             : z_closure_reply_call, z_owned_closure_hello_t         \
             : z_closure_hello_call, z_owned_reply_channel_closure_t \
             : z_reply_channel_closure_call)(&x, __VA_ARGS__)

#define _z_closure_overloader(callback, droper, ctx, ...) \
    { .call = callback, .drop = droper, .context = ctx }
#define z_closure(...) _z_closure_overloader(__VA_ARGS__, NULL, NULL)
#define z_move(x) (&x)
