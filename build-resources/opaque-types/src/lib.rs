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
use zenoh::scouting::Hello;
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

/// A serialized Zenoh data.
///
/// To minimize copies and reallocations, Zenoh may provide data in several separate buffers.
get_opaque_type_data!(Option<ZBytes>, z_owned_bytes_t);
/// A loaned serialized Zenoh data.
get_opaque_type_data!(ZBytes, z_loaned_bytes_t);


type CSlice = (usize, isize);

/// A contiguous owned sequence of bytes allocated by Zenoh.
get_opaque_type_data!(CSlice, z_owned_slice_t);
/// A contiguous sequence of bytes owned by some other entity.
get_opaque_type_data!(CSlice, z_view_slice_t);
/// A loaned sequence of bytes.
get_opaque_type_data!(CSlice, z_loaned_slice_t);

/// The wrapper type for null-terminated string values allocated by Zenoh.
get_opaque_type_data!(CSlice, z_owned_str_t);
/// The view over a null-terminated string.
get_opaque_type_data!(CSlice, z_view_str_t);
/// A loaned null-terminated string.
get_opaque_type_data!(CSlice, z_loaned_str_t);

/// A map of maybe-owned slices to maybe-owned slices.
///
/// In Zenoh C, this map is backed by Rust's standard HashMap, with a DoS-resistant hasher.
get_opaque_type_data!(Option<HashMap<usize, usize>>, z_owned_slice_map_t);
/// A loaned slice map.
get_opaque_type_data!(HashMap<usize, usize>, z_loaned_slice_map_t);

/// An array of maybe-owned slices.
///
get_opaque_type_data!(Option<Vec<CSlice>>, z_owned_slice_array_t);
/// A loaned slice array.
get_opaque_type_data!(Vec<CSlice>, z_loaned_slice_array_t);

/// An owned Zenoh sample.
///
/// This is a read only type that can only be constructed by cloning a `z_loaned_sample_t`.
/// Like all owned types, it should be freed using z_drop or z_sample_drop.
get_opaque_type_data!(Option<Sample>, z_owned_sample_t);
/// A loaned Zenoh sample.
get_opaque_type_data!(Sample, z_loaned_sample_t);

/// A reader for serialized data.
get_opaque_type_data!(Option<ZBytesReader<'static>>, z_owned_bytes_reader_t);
/// A loaned reader for serialized data.
get_opaque_type_data!(ZBytesReader<'static>, z_loaned_bytes_reader_t);

/// The <a href="https://zenoh.io/docs/manual/abstractions/#encoding"> encoding </a> of Zenoh data.
get_opaque_type_data!(Encoding, z_owned_encoding_t);
/// A loaned Zenoh encoding.
get_opaque_type_data!(Encoding, z_loaned_encoding_t);

/// An owned reply from a Queryable to a `z_get()`.
get_opaque_type_data!(Option<Reply>, z_owned_reply_t);
/// A loaned reply.
get_opaque_type_data!(Reply, z_loaned_reply_t);

/// A Zenoh value - a compination of payload and its encoding.
get_opaque_type_data!(Value, z_owned_value_t);
/// A loaned Zenoh value.
get_opaque_type_data!(Value, z_loaned_value_t);

/// An owned Zenoh query received by a queryable.
///
/// Queries are atomically reference-counted, letting you extract them from the callback that handed them to you by cloning.
get_opaque_type_data!(Option<Query>, z_owned_query_t);
/// A loaned Zenoh query.
get_opaque_type_data!(Query, z_loaned_query_t);

/// An owned Zenoh <a href="https://zenoh.io/docs/manual/abstractions/#queryable"> queryable </a>.
/// 
/// Responds to queries sent via `z_get()` with intersecting key expression.
get_opaque_type_data!(Option<Queryable<'static, ()>>, z_owned_queryable_t);
/// A loaned Zenoh queryable.
get_opaque_type_data!(Queryable<'static, ()>, z_loaned_queryable_t);

/// An owned Zenoh querying subscriber. 
/// 
/// In addition to receiving the data it is subscribed to,
/// it also will fetch data from a Queryable at startup and peridodically (using  `ze_querying_subscriber_get()`).
get_opaque_type_data!(Option<(zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session)>, ze_owned_querying_subscriber_t);
/// A loaned Zenoh querying subscriber.
get_opaque_type_data!((zenoh_ext::FetchingSubscriber<'static, ()>, &'static Session), ze_loaned_querying_subscriber_t);

/// A Zenoh-allocated <a href="https://zenoh.io/docs/manual/abstractions/#key-expression"> key expression </a>.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"key/expression"``.
///    - ``"key/ex*"``.
///
/// Key expressions can be mapped to numerical ids through `z_declare_keyexpr`
/// for wire and computation efficiency.
///
/// Internally key expressiobn can be either:
///   - A plain string expression.
///   - A pure numerical id.
///   - The combination of a numerical prefix and a string suffix.
get_opaque_type_data!(Option<KeyExpr<'static>>, z_owned_keyexpr_t);
/// A user allocated string, viewed as a key expression.
get_opaque_type_data!(Option<KeyExpr<'static>>, z_view_keyexpr_t);

/// A loaned key expression.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"key/expression"``.
///    - ``"key/ex*"``.
///
/// Using `z_declare_keyexpr` allows Zenoh to optimize a key expression,
/// both for local processing and network-wise.
get_opaque_type_data!(KeyExpr<'_>, z_loaned_keyexpr_t);

/// An owned Zenoh session.
get_opaque_type_data!(Option<Arc<Session>>, z_owned_session_t);
/// A loaned Zenoh session.
get_opaque_type_data!(Arc<Session>, z_loaned_session_t);

/// An owned Zenoh configuration.
get_opaque_type_data!(Option<Config>, z_owned_config_t);
/// A loaned Zenoh configuration.
get_opaque_type_data!(Config, z_loaned_config_t);

/// A Zenoh ID.
///
/// In general, valid Zenoh IDs are LSB-first 128bit unsigned and non-zero integers.
get_opaque_type_data!(ZenohId, z_id_t);

/// A Zenoh <a href="https://zenoh.io/docs/manual/abstractions/#timestamp"> timestamp </a>.
/// 
/// It consists of a time generated by a Hybrid Logical Clock (HLC) in NPT64 format and a unique zenoh identifier.
get_opaque_type_data!(Timestamp, z_timestamp_t);

/// An owned Zenoh <a href="https://zenoh.io/docs/manual/abstractions/#publisher"> publisher </a>.
get_opaque_type_data!(Option<Publisher<'static>>, z_owned_publisher_t);
/// A loaned Zenoh publisher.
get_opaque_type_data!(Publisher<'static>, z_loaned_publisher_t);

/// An owned Zenoh matching listener.
/// 
/// A listener that sends notifications when the [`MatchingStatus`] of a publisher changes.
/// Dropping the corresponding publisher, also drops matching listener.
get_opaque_type_data!(Option<MatchingListener<'static, DefaultHandler>>, zcu_owned_matching_listener_t);


/// An owned Zenoh <a href="https://zenoh.io/docs/manual/abstractions/#subscriber"> subscriber </a>.
/// 
/// Receives data from publication on intersecting key expressions.
/// Destroying the subscriber cancels the subscription.
get_opaque_type_data!(Option<Subscriber<'static, ()>>, z_owned_subscriber_t);
/// A loaned Zenoh subscriber.
get_opaque_type_data!(Subscriber<'static, ()>, z_loaned_subscriber_t);

/// A liveliness token that can be used to provide the network with information about connectivity to its
/// declarer: when constructed, a PUT sample will be received by liveliness subscribers on intersecting key
/// expressions.
///
/// A DELETE on the token's key expression will be received by subscribers if the token is destroyed, or if connectivity between the subscriber and the token's creator is lost.
get_opaque_type_data!(Option<LivelinessToken<'static>>, zc_owned_liveliness_token_t);
get_opaque_type_data!(LivelinessToken<'static>, zc_loaned_liveliness_token_t);


/// An owned Zenoh publication cache.
///
/// Used to store publications on intersecting key expressions. Can be queried later via `z_get()` to retrieve this data
/// (for example by `ze_owned_querying_subscriber_t`).
get_opaque_type_data!(Option<zenoh_ext::PublicationCache<'static>>, ze_owned_publication_cache_t);
/// A loaned Zenoh publication cache.
get_opaque_type_data!(zenoh_ext::PublicationCache<'static>, ze_loaned_publication_cache_t);

/// An owned mutex.
get_opaque_type_data!(Option<(Mutex<()>, Option<MutexGuard<'static, ()>>)>, z_owned_mutex_t);
/// A loaned mutex.
get_opaque_type_data!((Mutex<()>, Option<MutexGuard<'static, ()>>), z_loaned_mutex_t);

/// An owned conditional variable.
/// 
/// Used in combination with `z_owned_mutex_t` to wake up thread when certain conditions are met.
get_opaque_type_data!(Option<Condvar>, z_owned_condvar_t);
/// A loaned conditional variable.
get_opaque_type_data!(Condvar, z_loaned_condvar_t);

/// An owned Zenoh task.
get_opaque_type_data!(Option<JoinHandle<()>>, z_owned_task_t);

/// An owned Zenoh-allocated hello message returned by a Zenoh entity to a scout message sent with `z_scout()`.
get_opaque_type_data!(Option<Hello>, z_owned_hello_t);
/// A loaned hello message.
get_opaque_type_data!(Hello, z_loaned_hello_t);