#![allow(unused_doc_comments)]
#![allow(dead_code)]
#![allow(deprecated)]
use core::ffi::c_void;
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use std::sync::Arc;
use std::{
    sync::{Condvar, Mutex, MutexGuard},
    thread::JoinHandle,
};

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use zenoh::shm::{
    zshm, zshmmut, ChunkAllocResult, ChunkDescriptor, MemoryLayout, PosixShmProviderBackend,
    PrecomputedLayout, ProtocolID, PtrInSegment, ShmClient, ShmClientStorage, ShmProvider,
    ShmProviderBackend, WithProtocolID, ZLayoutError, ZShm, ZShmMut,
};
use zenoh::{
    bytes::{Encoding, ZBytes, ZBytesReader, ZBytesSliceIterator, ZBytesWriter},
    config::Config,
    handlers::{FifoChannelHandler, RingChannelHandler},
    key_expr::KeyExpr,
    liveliness::LivelinessToken,
    matching::MatchingListener,
    pubsub::{Publisher, Subscriber},
    query::{Querier, Query, Queryable, Reply, ReplyError},
    sample::Sample,
    scouting::Hello,
    session::{Session, ZenohId},
    time::Timestamp,
};
#[cfg(feature = "unstable")]
use zenoh::{
    cancellation::CancellationToken,
    internal::builders::close::NolocalJoinHandle,
    sample::SourceInfo,
    session::{
        EntityGlobalId, Link, LinkEvent, LinkEventsListener, Transport, TransportEvent,
        TransportEventsListener,
    },
};

#[macro_export]
macro_rules! get_opaque_type_data {
    ($src_type:ty, $name:ident) => {
        const _: () = {
            use const_format::concatcp;
            const DST_NAME: &str = stringify!($name);
            const ALIGN: usize = std::mem::align_of::<$src_type>();
            const SIZE: usize = std::mem::size_of::<$src_type>();
            const INFO_MESSAGE: &str =
                concatcp!("type: ", DST_NAME, ", align: ", ALIGN, ", size: ", SIZE);
            #[cfg(feature = "panic")]
            panic!("{}", INFO_MESSAGE);
        };
    };
}

/// A Zenoh data.
///
/// To minimize copies and reallocations, Zenoh may provide data in several separate buffers.
get_opaque_type_data!(ZBytes, z_owned_bytes_t);
/// A loaned Zenoh data.
get_opaque_type_data!(ZBytes, z_loaned_bytes_t);

pub struct CSlice {
    _data: *const u8,
    _len: usize,
    _drop: Option<extern "C" fn(data: *mut c_void, context: *mut c_void)>,
    _context: *mut c_void,
}

get_opaque_type_data!(CSlice, z_owned_slice_t);
/// A contiguous sequence of bytes owned by some other entity.
get_opaque_type_data!(CSlice, z_view_slice_t);
/// A loaned sequence of bytes.
get_opaque_type_data!(CSlice, z_loaned_slice_t);

/// The wrapper type for strings allocated by Zenoh.
get_opaque_type_data!(CSlice, z_owned_string_t);
/// The view over a string.
get_opaque_type_data!(CSlice, z_view_string_t);
/// A loaned string.
get_opaque_type_data!(CSlice, z_loaned_string_t);

/// An array of maybe-owned non-null terminated strings.
///
get_opaque_type_data!(Vec<CSlice>, z_owned_string_array_t);
/// A loaned string array.
get_opaque_type_data!(Vec<CSlice>, z_loaned_string_array_t);

/// An owned Zenoh sample.
///
/// This is a read only type that can only be constructed by cloning a `z_loaned_sample_t`.
/// Like all owned types, it should be freed using z_drop or z_sample_drop.
get_opaque_type_data!(Option<Sample>, z_owned_sample_t);
/// A loaned Zenoh sample.
get_opaque_type_data!(Sample, z_loaned_sample_t);

/// A reader for payload.
get_opaque_type_data!(ZBytesReader<'static>, z_bytes_reader_t);

/// An owned writer for payload.
get_opaque_type_data!(Option<ZBytesWriter>, z_owned_bytes_writer_t);
/// An loaned writer for payload.
get_opaque_type_data!(ZBytesWriter, z_loaned_bytes_writer_t);

/// An iterator over slices of serialized data.
get_opaque_type_data!(ZBytesSliceIterator<'static>, z_bytes_slice_iterator_t);

/// The <a href="https://zenoh.io/docs/manual/abstractions/#encoding"> encoding </a> of Zenoh data.
get_opaque_type_data!(Encoding, z_owned_encoding_t);
/// A loaned Zenoh encoding.
get_opaque_type_data!(Encoding, z_loaned_encoding_t);

/// An owned reply from a Queryable to a `z_get()`.
get_opaque_type_data!(Option<Reply>, z_owned_reply_t);
/// A loaned reply.
get_opaque_type_data!(Reply, z_loaned_reply_t);

/// A Zenoh reply error - a combination of reply error payload and its encoding.
get_opaque_type_data!(ReplyError, z_owned_reply_err_t);
/// A loaned Zenoh reply error.
get_opaque_type_data!(ReplyError, z_loaned_reply_err_t);

/// An owned Zenoh query received by a queryable.
///
/// Queries are atomically reference-counted, letting you extract them from the callback that handed them to you by cloning.
get_opaque_type_data!(Option<Query>, z_owned_query_t);
/// A loaned Zenoh query.
get_opaque_type_data!(Query, z_loaned_query_t);

/// An owned Zenoh <a href="https://zenoh.io/docs/manual/abstractions/#queryable"> queryable </a>.
///
/// Responds to queries sent via `z_get()` with intersecting key expression.
get_opaque_type_data!(Option<Queryable<()>>, z_owned_queryable_t);
/// A loaned Zenoh queryable.
get_opaque_type_data!(Queryable<()>, z_loaned_queryable_t);

/// An owned Zenoh querier.
///
/// Sends queries to matching queryables.
get_opaque_type_data!(Option<Querier>, z_owned_querier_t);
/// A loaned Zenoh queryable.
get_opaque_type_data!(Querier, z_loaned_querier_t);

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned Zenoh querying subscriber.
///
/// In addition to receiving the data it is subscribed to,
/// it also will fetch data from a Queryable at startup and peridodically (using  `ze_querying_subscriber_get()`).
get_opaque_type_data!(
    Option<(zenoh_ext::FetchingSubscriber<()>, &'static Session)>,
    ze_owned_querying_subscriber_t
);
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned Zenoh querying subscriber.
get_opaque_type_data!(
    (zenoh_ext::FetchingSubscriber<()>, &'static Session),
    ze_loaned_querying_subscriber_t
);

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned Zenoh advanced subscriber.
///
/// In addition to receiving the data it is subscribed to,
/// it is also able to receive notifications regarding missed samples and/or automatically recover them.
get_opaque_type_data!(
    Option<zenoh_ext::AdvancedSubscriber<()>>,
    ze_owned_advanced_subscriber_t
);
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned Zenoh advanced subscriber.
get_opaque_type_data!(
    zenoh_ext::AdvancedSubscriber<()>,
    ze_loaned_advanced_subscriber_t
);
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned Zenoh sample miss listener. Missed samples can only be detected from advanced publishers, enabling sample miss detection.
///
/// A listener that sends notification when the advanced subscriber misses a sample .
/// Dropping the corresponding subscriber, also drops the listener.
get_opaque_type_data!(
    Option<zenoh_ext::SampleMissListener<()>>,
    ze_owned_sample_miss_listener_t
);

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned Zenoh advanced publisher.
///
/// In addition to publishing the data,
/// it also maintains the storage, allowing matching subscribers to retrive missed samples.
get_opaque_type_data!(
    Option<zenoh_ext::AdvancedPublisher<'static>>,
    ze_owned_advanced_publisher_t
);
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned Zenoh advanced publisher.
get_opaque_type_data!(
    zenoh_ext::AdvancedPublisher<'static>,
    ze_loaned_advanced_publisher_t
);
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
get_opaque_type_data!(KeyExpr<'static>, z_loaned_keyexpr_t);

/// An owned Zenoh session.
get_opaque_type_data!(Option<Session>, z_owned_session_t);
/// A loaned Zenoh session.
get_opaque_type_data!(Session, z_loaned_session_t);

#[cfg(feature = "unstable")]
/// An owned Close handle
get_opaque_type_data!(
    Option<NolocalJoinHandle<zenoh::Result<()>>>,
    zc_owned_concurrent_close_handle_t
);

/// An owned Zenoh configuration.
get_opaque_type_data!(Option<Config>, z_owned_config_t);
/// A loaned Zenoh configuration.
get_opaque_type_data!(Config, z_loaned_config_t);

/// @brief A Zenoh ID.
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

/// @brief An owned Zenoh matching listener.
///
/// A listener that sends notifications when the [`MatchingStatus`] of a publisher or querier changes.
/// Dropping the corresponding publisher, also drops matching listener.
get_opaque_type_data!(Option<MatchingListener<()>>, z_owned_matching_listener_t);

/// An owned Zenoh <a href="https://zenoh.io/docs/manual/abstractions/#subscriber"> subscriber </a>.
///
/// Receives data from publication on intersecting key expressions.
/// Destroying the subscriber cancels the subscription.
get_opaque_type_data!(Option<Subscriber<()>>, z_owned_subscriber_t);
/// A loaned Zenoh subscriber.
get_opaque_type_data!(Subscriber<()>, z_loaned_subscriber_t);

/// @brief A liveliness token that can be used to provide the network with information about connectivity to its
/// declarer: when constructed, a PUT sample will be received by liveliness subscribers on intersecting key
/// expressions.
///
/// A DELETE on the token's key expression will be received by subscribers if the token is destroyed, or if connectivity between the subscriber and the token's creator is lost.
get_opaque_type_data!(Option<LivelinessToken>, z_owned_liveliness_token_t);
/// @brief A loaned liveliness token.
get_opaque_type_data!(LivelinessToken, z_loaned_liveliness_token_t);

/// @brief An Transport structure returned by Zenoh connectivity API.
///
/// Represents a remote zenoh node connected to this node. Only one transport per remote node exists.
/// Each transport can have multiple corresponding `z_owned_link_t` which represent
/// actual established data links with various protocols.
#[cfg(feature = "unstable")]
get_opaque_type_data!(Option<Transport>, z_owned_transport_t);
#[cfg(feature = "unstable")]
//// @brief A loaned Transport structure.
get_opaque_type_data!(Transport, z_loaned_transport_t);

/// @brief A Zenoh link structure returned by Zenoh connectivity API.
///
/// Represents an actual data link with a remote zenoh node over a specific protocol.
#[cfg(feature = "unstable")]
get_opaque_type_data!(Option<Link>, z_owned_link_t);
#[cfg(feature = "unstable")]
//// @brief A loaned Link structure.
get_opaque_type_data!(Link, z_loaned_link_t);

/// @brief The event notifyting about addition or removal of a transport `z_owned_transport_t`
///
/// Used in Zenoh connectivity API to notify about connecting or disconnecting to remote zenoh nodes.
#[cfg(feature = "unstable")]
get_opaque_type_data!(Option<TransportEvent>, z_owned_transport_event_t);
#[cfg(feature = "unstable")]
/// @brief A loaned TransportEvent structure.
get_opaque_type_data!(TransportEvent, z_loaned_transport_event_t);

/// @brief The event notifyting about addition or removal of a link `z_owned_link_t`
///
/// Used in Zenoh connectivity API to notify about establishment or break of data links with remote zenoh nodes.
#[cfg(feature = "unstable")]
get_opaque_type_data!(Option<LinkEvent>, z_owned_link_event_t);
#[cfg(feature = "unstable")]
/// @brief A loaned LinkEvent structure.
get_opaque_type_data!(LinkEvent, z_loaned_link_event_t);

/// @brief An listener for transport events.
///
/// Used in Zenoh connectivity API to get notified about connecting or disconnecting to remote zenoh nodes.
#[cfg(feature = "unstable")]
get_opaque_type_data!(
    Option<TransportEventsListener<()>>,
    z_owned_transport_events_listener_t
);
#[cfg(feature = "unstable")]
/// @brief A loaned TransportEventsListener structure.
get_opaque_type_data!(
    TransportEventsListener<()>,
    z_loaned_transport_events_listener_t
);

/// @brief An listener for link events.
///
/// Used in Zenoh connectivity API to get notified about establishment or break of data links with remote zenoh nodes.
#[cfg(feature = "unstable")]
get_opaque_type_data!(
    Option<LinkEventsListener<()>>,
    z_owned_link_events_listener_t
);
#[cfg(feature = "unstable")]
/// @brief A loaned LinkEventsListener structure.
get_opaque_type_data!(LinkEventsListener<()>, z_loaned_link_events_listener_t);

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned Zenoh publication cache.
///
/// Used to store publications on intersecting key expressions. Can be queried later via `z_get()` to retrieve this data
/// (for example by `ze_owned_querying_subscriber_t`).
get_opaque_type_data!(
    Option<zenoh_ext::PublicationCache>,
    ze_owned_publication_cache_t
);
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned Zenoh publication cache.
get_opaque_type_data!(zenoh_ext::PublicationCache, ze_loaned_publication_cache_t);

/// An owned mutex.
get_opaque_type_data!(
    Option<(Mutex<()>, Option<MutexGuard<'static, ()>>)>,
    z_owned_mutex_t
);
/// A loaned mutex.
get_opaque_type_data!(
    (Mutex<()>, Option<MutexGuard<'static, ()>>),
    z_loaned_mutex_t
);

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

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned SHM Client.
get_opaque_type_data!(Option<Arc<dyn ShmClient>>, z_owned_shm_client_t);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned list of SHM Clients.
get_opaque_type_data!(Option<Vec<Arc<dyn ShmClient>>>, zc_owned_shm_client_list_t);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned list of SHM Clients.
get_opaque_type_data!(Vec<Arc<dyn ShmClient>>, zc_loaned_shm_client_list_t);

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned SHM Client Storage
get_opaque_type_data!(Option<Arc<ShmClientStorage>>, z_owned_shm_client_storage_t);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// A loaned SHM Client Storage.
get_opaque_type_data!(Arc<ShmClientStorage>, z_loaned_shm_client_storage_t);

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned MemoryLayout.
get_opaque_type_data!(Option<MemoryLayout>, z_owned_memory_layout_t);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned MemoryLayout.
get_opaque_type_data!(MemoryLayout, z_loaned_memory_layout_t);

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned ChunkAllocResult.
get_opaque_type_data!(Option<ChunkAllocResult>, z_owned_chunk_alloc_result_t);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned ZShm slice.
get_opaque_type_data!(Option<ZShm>, z_owned_shm_t);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned ZShm slice.
get_opaque_type_data!(zshm, z_loaned_shm_t);

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned ZShmMut slice.
get_opaque_type_data!(Option<ZShmMut>, z_owned_shm_mut_t);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned ZShmMut slice.
get_opaque_type_data!(zshmmut, z_loaned_shm_mut_t);

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
#[derive(Debug)]
#[repr(C)]
struct DummyCallbacks {
    alloc_fn: unsafe extern "C" fn(),
    free_fn: unsafe extern "C" fn(),
    defragment_fn: unsafe extern "C" fn() -> usize,
    available_fn: unsafe extern "C" fn() -> usize,
    layout_for_fn: unsafe extern "C" fn(),
    id_fn: unsafe extern "C" fn(),
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
#[derive(Debug)]
#[repr(C)]
struct DummyContext {
    context: *mut c_void,
    delete_fn: unsafe extern "C" fn(*mut c_void),
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
#[derive(Debug)]
struct DummySHMProviderBackend {
    context: DummyContext,
    callbacks: DummyCallbacks,
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
impl WithProtocolID for DummySHMProviderBackend {
    fn id(&self) -> ProtocolID {
        0
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
impl ShmProviderBackend for DummySHMProviderBackend {
    fn alloc(&self, _layout: &MemoryLayout) -> ChunkAllocResult {
        todo!()
    }

    fn free(&self, _chunk: &ChunkDescriptor) {
        todo!()
    }

    fn defragment(&self) -> usize {
        todo!()
    }

    fn available(&self) -> usize {
        todo!()
    }

    fn layout_for(&self, _layout: MemoryLayout) -> Result<MemoryLayout, ZLayoutError> {
        todo!()
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
type DummySHMProvider = ShmProvider<DummySHMProviderBackend>;

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
type PosixSHMProvider = ShmProvider<PosixShmProviderBackend>;

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
type SharedPosixSHMProvider = Arc<PosixSHMProvider>;

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
enum CDummySHMProvider {
    Posix(PosixSHMProvider),
    SharedPosix(SharedPosixSHMProvider),
    Dynamic(DummySHMProvider),
    DynamicThreadsafe(DummySHMProvider),
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
struct DummySharedShmProvider(CDummySHMProvider);

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned ShmProvider.
get_opaque_type_data!(Option<CDummySHMProvider>, z_owned_shm_provider_t);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned ShmProvider.
get_opaque_type_data!(CDummySHMProvider, z_loaned_shm_provider_t);

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned shared ShmProvider.
get_opaque_type_data!(
    Option<DummySharedShmProvider>,
    z_owned_shared_shm_provider_t
);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned shared ShmProvider.
get_opaque_type_data!(DummySharedShmProvider, z_loaned_shared_shm_provider_t);

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
type PosixPrecomputedLayout = PrecomputedLayout<'static, PosixShmProviderBackend, MemoryLayout>;

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
type DummyDynamicPrecomputedLayout =
    PrecomputedLayout<'static, DummySHMProviderBackend, MemoryLayout>;

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
type DummyDynamicPrecomputedLayoutThreadSafe =
    PrecomputedLayout<'static, DummySHMProviderBackend, MemoryLayout>;

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
enum CSHMLayout {
    Posix(PosixPrecomputedLayout),
    Dynamic(DummyDynamicPrecomputedLayout),
    DynamicThreadSafe(DummyDynamicPrecomputedLayout),
}
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned ShmProvider's PrecomputedLayout.
get_opaque_type_data!(Option<CSHMLayout>, z_owned_precomputed_layout_t);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned ShmProvider's PrecomputedLayout.
get_opaque_type_data!(CSHMLayout, z_loaned_precomputed_layout_t);

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A pointer in SHM Segment.
get_opaque_type_data!(Option<PtrInSegment>, z_owned_ptr_in_segment_t);
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned pointer in SHM Segment.
get_opaque_type_data!(PtrInSegment, z_loaned_ptr_in_segment_t);

/// An owned Zenoh fifo sample handler.
get_opaque_type_data!(
    Option<FifoChannelHandler<Sample>>,
    z_owned_fifo_handler_sample_t
);
/// An loaned Zenoh fifo sample handler.
get_opaque_type_data!(FifoChannelHandler<Sample>, z_loaned_fifo_handler_sample_t);

/// An owned Zenoh ring sample handler.
get_opaque_type_data!(
    Option<RingChannelHandler<Sample>>,
    z_owned_ring_handler_sample_t
);
/// An loaned Zenoh ring sample handler.
get_opaque_type_data!(RingChannelHandler<Sample>, z_loaned_ring_handler_sample_t);

/// An owned Zenoh fifo query handler.
get_opaque_type_data!(
    Option<FifoChannelHandler<Query>>,
    z_owned_fifo_handler_query_t
);
/// An loaned Zenoh fifo query handler.
get_opaque_type_data!(FifoChannelHandler<Query>, z_loaned_fifo_handler_query_t);

/// An owned Zenoh ring query handler.
get_opaque_type_data!(
    Option<RingChannelHandler<Query>>,
    z_owned_ring_handler_query_t
);
/// An loaned Zenoh ring query handler.
get_opaque_type_data!(RingChannelHandler<Query>, z_loaned_ring_handler_query_t);

/// An owned Zenoh fifo reply handler.
get_opaque_type_data!(
    Option<FifoChannelHandler<Reply>>,
    z_owned_fifo_handler_reply_t
);
/// An loaned Zenoh fifo reply handler.
get_opaque_type_data!(FifoChannelHandler<Reply>, z_loaned_fifo_handler_reply_t);

/// An owned Zenoh ring reply handler.
get_opaque_type_data!(
    Option<RingChannelHandler<Reply>>,
    z_owned_ring_handler_reply_t
);
/// An loaned Zenoh ring reply handler.
get_opaque_type_data!(RingChannelHandler<Reply>, z_loaned_ring_handler_reply_t);

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A source info.
get_opaque_type_data!(SourceInfo, z_source_info_t);
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An entity gloabal id.
get_opaque_type_data!(EntityGlobalId, z_entity_global_id_t);

/// @brief An owned Zenoh serializer.
get_opaque_type_data!(Option<zenoh_ext::ZSerializer>, ze_owned_serializer_t);
/// @brief A loaned Zenoh serializer.
get_opaque_type_data!(zenoh_ext::ZSerializer, ze_loaned_serializer_t);
/// @brief A Zenoh serializer.
get_opaque_type_data!(zenoh_ext::ZDeserializer<'static>, ze_deserializer_t);

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned cancellation token, which can be used to interrupt GET queries.
get_opaque_type_data!(Option<CancellationToken>, z_owned_cancellation_token_t);

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned cancellation token, which can be used to interrupt GET queries.
get_opaque_type_data!(CancellationToken, z_loaned_cancellation_token_t);
