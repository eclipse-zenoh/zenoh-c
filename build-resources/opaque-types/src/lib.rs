use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread::JoinHandle;
use zenoh::config::Config;
use zenoh::config::ZenohId;
use zenoh::encoding::Encoding;
use zenoh::handlers::DefaultHandler;
use zenoh::key_expr::KeyExpr;
use zenoh::bytes::{ZBytes, ZBytesReader};
use zenoh::liveliness::LivelinessToken;
use zenoh::publication::MatchingListener;
use zenoh::publication::Publisher;
use zenoh::query::Reply;
use zenoh::queryable::Query;
use zenoh::queryable::Queryable;
use zenoh::sample::Sample;
use zenoh::session::Session;
use zenoh::subscriber::Subscriber;
use zenoh::time::Timestamp;
use zenoh::value::Value;

#[macro_export]
macro_rules! get_opaque_type_data {
    ($src_type:ty, $name:ident) => {
        const _: () = {
            use const_format::concatcp;
            const DST_NAME: &str = stringify!($name);
            const ALIGN: usize = std::mem::align_of::<$src_type>();
            const SIZE: usize = std::mem::size_of::<$src_type>();
            const INFO_MESSAGE: &str = concatcp!(
                "type: ", DST_NAME, ", align: ", ALIGN, ", size: ", SIZE
            );
            panic!("{}", INFO_MESSAGE);
        };
    }
}

/// A split buffer that owns all of its data.
///
/// To minimize copies and reallocations, Zenoh may provide you data in split buffers.
get_opaque_type_data!(Option<ZBytes>, z_owned_bytes_t);
/// A loaned payload.
get_opaque_type_data!(ZBytes, z_loaned_bytes_t);

/// A contiguous view of bytes owned by some other entity.
///
/// `start` being `null` is considered a gravestone value,
/// and empty slices are represented using a possibly dangling pointer for `start`.
get_opaque_type_data!(Option<Box<[u8]>>, z_owned_slice_t);
get_opaque_type_data!(Option<&'static [u8]>, z_view_slice_t);
get_opaque_type_data!(&'static [u8], z_loaned_slice_t);

/// The wrapper type for null-terminated string values allocated by zenoh. The instances of `z_owned_str_t`
/// should be released with `z_drop` macro or with `z_str_drop` function and checked to validity with
/// `z_check` and `z_str_check` correspondently
get_opaque_type_data!(Option<Box<[u8]>>, z_owned_str_t);
get_opaque_type_data!(Option<&'static [u8]>, z_view_str_t);
get_opaque_type_data!(&'static [u8], z_loaned_str_t);

/// A map of maybe-owned vector of bytes to maybe-owned vector of bytes.
///
/// In Zenoh C, this map is backed by Rust's standard HashMap, with a DoS-resistant hasher
get_opaque_type_data!(Option<HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>>, z_owned_slice_map_t);
get_opaque_type_data!(HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>, z_loaned_slice_map_t);

/// An owned sample.
///
/// This is a read only type that can only be constructed by cloning a `z_loaned_sample_t`.
/// Like all owned types, its memory must be freed by passing a mutable reference to it to `z_sample_drop`.
get_opaque_type_data!(Option<Sample>, z_owned_sample_t);
get_opaque_type_data!(Sample, z_loaned_sample_t);

/// A reader for payload data.
get_opaque_type_data!(Option<ZBytesReader<'static>>, z_owned_bytes_reader_t);
get_opaque_type_data!(ZBytesReader<'static>, z_loaned_bytes_reader_t);

/// The encoding of a payload, in a MIME-like format.
///
/// For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
get_opaque_type_data!(Encoding, z_loaned_encoding_t);
get_opaque_type_data!(Encoding, z_owned_encoding_t);

/// An owned reply to a :c:func:`z_get`.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
get_opaque_type_data!(Option<Reply>, z_owned_reply_t);
get_opaque_type_data!(Reply, z_loaned_reply_t);

/// A zenoh value.
get_opaque_type_data!(Value, z_owned_value_t);
get_opaque_type_data!(Value, z_loaned_value_t);

// Loaned variant of a Query received by a Queryable.
///
/// Queries are atomically reference-counted, letting you extract them from the callback that handed them to you by cloning.
/// `z_loaned_query_t`'s are valid as long as at least one corresponding `z_owned_query_t` exists, including the one owned by Zenoh until the callback returns.
get_opaque_type_data!(Option<Query>, z_owned_query_t);
get_opaque_type_data!(Query, z_loaned_query_t);

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
get_opaque_type_data!(Queryable<'static, ()>, z_loaned_queryable_t);

/// An owned zenoh querying subscriber. Destroying the subscriber cancels the subscription.
///
/// Like most `ze_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
///
/// Like all `ze_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
get_opaque_type_data!(Option<(zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session)>, ze_owned_querying_subscriber_t);
get_opaque_type_data!((zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session), ze_loaned_querying_subscriber_t);

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
get_opaque_type_data!(Option<KeyExpr<'static>>, z_view_keyexpr_t);

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
get_opaque_type_data!(KeyExpr<'_>, z_loaned_keyexpr_t);

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
get_opaque_type_data!(Session, z_loaned_session_t);

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
get_opaque_type_data!(Option<Config>, z_owned_config_t);
/// A loaned zenoh configuration.
get_opaque_type_data!(Config, z_loaned_config_t);

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
get_opaque_type_data!(Publisher<'static>, z_loaned_publisher_t);

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


/// An owned zenoh subscriber. Destroying the subscriber cancels the subscription.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
get_opaque_type_data!(Option<Subscriber<'static, ()>>, z_owned_subscriber_t);
get_opaque_type_data!(Subscriber<'static, ()>, z_loaned_subscriber_t);

/// A liveliness token that can be used to provide the network with information about connectivity to its
/// declarer: when constructed, a PUT sample will be received by liveliness subscribers on intersecting key
/// expressions.
///
/// A DELETE on the token's key expression will be received by subscribers if the token is destroyed, or if connectivity between the subscriber and the token's creator is lost.
get_opaque_type_data!(Option<LivelinessToken<'static>>, zc_owned_liveliness_token_t);
get_opaque_type_data!(LivelinessToken<'static>, zc_loaned_liveliness_token_t);


/// An owned zenoh publication_cache.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
get_opaque_type_data!(Option<zenoh_ext::PublicationCache<'static>>, ze_owned_publication_cache_t);
get_opaque_type_data!(zenoh_ext::PublicationCache<'static>, ze_loaned_publication_cache_t);


get_opaque_type_data!(Option<(Mutex<()>, Option<MutexGuard<'static, ()>>)>, z_owned_mutex_t);
get_opaque_type_data!((Mutex<()>, Option<MutexGuard<'static, ()>>), z_loaned_mutex_t);

get_opaque_type_data!(Option<Condvar>, z_owned_condvar_t);
get_opaque_type_data!(Condvar, z_loaned_condvar_t);

get_opaque_type_data!(Option<JoinHandle<()>>, z_owned_task_t);