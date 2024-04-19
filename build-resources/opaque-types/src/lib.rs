use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use zenoh::config::Config;
use zenoh::config::ZenohId;
use zenoh::encoding::Encoding;
use zenoh::handlers::DefaultHandler;
use zenoh::key_expr::KeyExpr;
use zenoh::bytes::{ZBytes, ZBytesReader};
use zenoh::publication::MatchingListener;
use zenoh::publication::Publisher;
use zenoh::query::Reply;
use zenoh::queryable::Query;
use zenoh::queryable::Queryable;
use zenoh::sample::Sample;
use zenoh::session::Session;
use zenoh::time::Timestamp;
use zenoh::value::Value;

// Disabled due to dependency on z_session_t. To be reworked as for autogeneration this dependency is cicrular.
// pub struct FetchingSubscriberWrapper {
//     fetching_subscriber: zenoh_ext::FetchingSubscriber<'static, ()>,
//     session: z_session_t,
// }

#[macro_export]
macro_rules! get_opaque_type_data {
    ($src_type:ty, $name:ident) => {
        const _: () = {
            const fn get_num_digits(n: usize) -> usize {
                let mut out = 0;
                let mut res = n;
                while res > 0 {
                    out += 1;
                    res = res / 10;
                }
                if out == 0 {
                    out = 1;
                }
                out
            }

            const fn write_str(src: &[u8], mut dst: [u8; MSG_LEN], offset: usize) -> [u8; MSG_LEN] {
                let mut i = 0;
                while i < src.len() {
                    dst[i + offset] = src[i];
                    i += 1;
                }
                dst
            }

            const fn write_num(src: usize, mut dst: [u8; MSG_LEN], offset: usize) -> [u8; MSG_LEN] {
                let mut i = 0;
                let num_digits = get_num_digits(src) as u32;
                while i < num_digits {
                    dst[i as usize + offset] = b'0' + ((src / 10u32.pow(num_digits - i - 1) as usize) % 10) as u8;
                    i += 1;
                }
                dst
            }

            const DST_NAME: &str = stringify!($name);
            const ALIGN: usize = std::mem::align_of::<$src_type>();
            const SIZE: usize = std::mem::size_of::<$src_type>();
            const TYPE_TOKEN: [u8; 6] = *b"type: ";
            const ALIGN_TOKEN: [u8; 9] = *b", align: ";
            const SIZE_TOKEN: [u8; 8] = *b", size: ";
            const SIZE_NUM_DIGITS: usize = get_num_digits(SIZE);
            const ALIGN_NUM_DIGITS: usize = get_num_digits(ALIGN);
            const MSG_LEN: usize = TYPE_TOKEN.len() + ALIGN_TOKEN.len() + SIZE_TOKEN.len() + SIZE_NUM_DIGITS + ALIGN_NUM_DIGITS + DST_NAME.len();
            const TYPE_OFFSET: usize = TYPE_TOKEN.len();
            const ALIGN_OFFSET: usize = TYPE_OFFSET + DST_NAME.len() + ALIGN_TOKEN.len();
            const SIZE_OFFSET: usize = ALIGN_OFFSET + ALIGN_NUM_DIGITS + SIZE_TOKEN.len();
            let mut msg: [u8; MSG_LEN] = [b' '; MSG_LEN];

            msg = write_str(&TYPE_TOKEN, msg, 0);
            msg = write_str(&DST_NAME.as_bytes(), msg, TYPE_OFFSET);
            msg = write_str(&ALIGN_TOKEN, msg, ALIGN_OFFSET - ALIGN_TOKEN.len());
            msg = write_num(ALIGN, msg, ALIGN_OFFSET);
            msg = write_str(&SIZE_TOKEN, msg, SIZE_OFFSET - SIZE_TOKEN.len());
            msg = write_num(SIZE, msg, SIZE_OFFSET);

            panic!("{}", unsafe {
                std::str::from_utf8_unchecked(msg.as_slice())
            });
        };
    }
}

/// A split buffer that owns all of its data.
///
/// To minimize copies and reallocations, Zenoh may provide you data in split buffers.
get_opaque_type_data!(Option<ZBytes>, z_owned_bytes_t);
/// A loaned payload.
get_opaque_type_data!(&'static ZBytes, z_bytes_t);

/// A map of maybe-owned vector of bytes to maybe-owned vector of bytes.
///
/// In Zenoh C, this map is backed by Rust's standard HashMap, with a DoS-resistant hasher
get_opaque_type_data!(Option<HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>>, z_owned_slice_map_t);
get_opaque_type_data!(&'static HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>, z_slice_map_t);

/// An owned sample.
///
/// This is a read only type that can only be constructed by cloning a `z_sample_t`.
/// Like all owned types, its memory must be freed by passing a mutable reference to it to `zc_sample_drop`.
get_opaque_type_data!(Option<Sample>, zc_owned_sample_t);
get_opaque_type_data!(&'static Sample, z_sample_t);

/// A reader for payload data.
get_opaque_type_data!(Option<ZBytesReader<'static>>, z_owned_bytes_t_reader_t);
get_opaque_type_data!(&'static ZBytesReader<'static>, z_bytes_reader_t);

/// The encoding of a payload, in a MIME-like format.
///
/// For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
get_opaque_type_data!(&'static Encoding, z_encoding_t);
get_opaque_type_data!(Encoding, z_owned_encoding_t);

/// An owned reply to a :c:func:`z_get`.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
get_opaque_type_data!(Option<Reply>, z_owned_reply_t);
get_opaque_type_data!(&'static Reply, z_reply_t);

/// A zenoh value.
get_opaque_type_data!(Value, z_owned_value_t);
get_opaque_type_data!(&'static Value, z_value_t);

// Loaned variant of a Query received by a Queryable.
///
/// Queries are atomically reference-counted, letting you extract them from the callback that handed them to you by cloning.
/// `z_query_t`'s are valid as long as at least one corresponding `z_owned_query_t` exists, including the one owned by Zenoh until the callback returns.
get_opaque_type_data!(Option<Query>, z_owned_query_t);
get_opaque_type_data!(&'static Query, z_query_t);

/// An owned zenoh queryable.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
get_opaque_type_data!(Option<Queryable<'static, ()>>, z_owned_queryable_t);
get_opaque_type_data!(&'static Queryable<'static, ()>, z_queryable_t);

// get_opaque_type_data!(
//     Option<Box<FetchingSubscriberWrapper>>,
//     ze_owned_querying_subscriber_t
// );
// get_opaque_type_data!(
//     &'static FetchingSubscriberWrapper,
//     ze_querying_subscriber_t
// );

/// A zenoh-allocated key expression.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"key/expression"``.
///    - ``"key/ex*"``.
///
/// Key expressions can be mapped to numerical ids through :c:func:`z_declare_expr`
/// for wire and computation efficiency.
///
/// A `key expression <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Key%20Expressions.md>`_ can be either:
///   - A plain string expression.
///   - A pure numerical id.
///   - The combination of a numerical prefix and a string suffix.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
get_opaque_type_data!(Option<KeyExpr<'static>>, z_owned_keyexpr_t);
/// A loaned key expression.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"key/expression"``.
///    - ``"key/ex*"``.
///
/// Using :c:func:`z_declare_keyexpr` allows zenoh to optimize a key expression,
/// both for local processing and network-wise.
get_opaque_type_data!(&'static KeyExpr<'_>, z_keyexpr_t);

/// An owned zenoh session.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
get_opaque_type_data!(Option<Arc<Session>>, z_owned_session_t);
get_opaque_type_data!(&'static Session, z_session_t);

/// An owned zenoh configuration.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
get_opaque_type_data!(Option<Box<Config>>, z_owned_config_t);
/// A loaned zenoh configuration.
get_opaque_type_data!(&'static Config, z_config_t);

/// Represents a Zenoh ID.
///
/// In general, valid Zenoh IDs are LSB-first 128bit unsigned and non-zero integers.
get_opaque_type_data!(ZenohId, z_id_t);

get_opaque_type_data!(Timestamp, z_timestamp_t);

/// An owned zenoh publisher.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
get_opaque_type_data!(Option<Publisher<'static>>, z_owned_publisher_t);
get_opaque_type_data!(&'static Publisher<'static>, z_publisher_t);

/// An owned zenoh matching listener. Destroying the matching listener cancels the subscription.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
get_opaque_type_data!(Option<MatchingListener<'static, DefaultHandler>>, zcu_owned_matching_listener_t);
