#ifndef ZENOH_H
#define ZENOH_H
#include "zenoh-gen.h"

#define z_borrow(x) _Generic((x), z_owned_session_t \
                             : z_session_borrow,    \
                               z_owned_reskey_t     \
                             : z_reskey_borrow,     \
                               z_owned_config_t     \
                             : z_config_borrow,     \
                               z_owned_string_t     \
                             : z_string_borrow,     \
                               z_owned_bytes_t      \
                             : z_bytes_borrow,      \
                               z_owned_info_t       \
                             : z_info_borrow)(&x)
#define z_free(x) _Generic((x), z_owned_session_t \
                           : z_close,             \
                             z_owned_reskey_t     \
                           : z_reskey_free,       \
                             z_owned_config_t     \
                           : z_config_free,       \
                             z_owned_string_t     \
                           : z_string_free,       \
                             z_owned_bytes_t      \
                           : z_bytes_free,        \
                             z_owned_info_t       \
                           : z_info_free,         \
                             z_owned_subscriber_t \
                           : z_unregister_subscriber)(&x)
#define z_check(x) _Generic((x), z_owned_session_t \
                            : z_session_check,     \
                              z_owned_reskey_t     \
                            : z_reskey_check,      \
                              z_owned_config_t     \
                            : z_config_check,      \
                              z_owned_publisher_t  \
                            : z_publisher_check,   \
                              z_owned_string_t     \
                            : z_string_check,      \
                              z_owned_bytes_t      \
                            : z_bytes_check,       \
                              z_owned_info_t       \
                            : z_info_check,        \
                              z_owned_subscriber_t \
                            : z_subscriber_check)(&x)
#endif