#[rustfmt::skip]
#[allow(clippy::all)]
/// A split buffer that owns all of its data.
///
/// To minimize copies and reallocations, Zenoh may provide you data in split buffers.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_bytes_t {
    _0: [u8; 40],
}
/// A loaned payload.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_bytes_t {
    _0: [u8; 40],
}
/// A contiguous view of bytes owned by some other entity.
///
/// `start` being `null` is considered a gravestone value,
/// and empty slices are represented using a possibly dangling pointer for `start`.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_slice_t {
    _0: [u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_view_slice_t {
    _0: [u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_slice_t {
    _0: [u8; 16],
}
/// The wrapper type for null-terminated string values allocated by zenoh. The instances of `z_owned_str_t`
/// should be released with `z_drop` macro or with `z_str_drop` function and checked to validity with
/// `z_check` and `z_str_check` correspondently
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_str_t {
    _0: [u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_view_str_t {
    _0: [u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_str_t {
    _0: [u8; 16],
}
/// A map of maybe-owned vector of bytes to maybe-owned vector of bytes.
///
/// In Zenoh C, this map is backed by Rust's standard HashMap, with a DoS-resistant hasher
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_slice_map_t {
    _0: [u8; 48],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_slice_map_t {
    _0: [u8; 48],
}
/// An owned sample.
///
/// This is a read only type that can only be constructed by cloning a `z_loaned_sample_t`.
/// Like all owned types, its memory must be freed by passing a mutable reference to it to `z_sample_drop`.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_sample_t {
    _0: [u8; 240],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_sample_t {
    _0: [u8; 240],
}
/// A reader for payload data.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_bytes_reader_t {
    _0: [u8; 24],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_bytes_reader_t {
    _0: [u8; 24],
}
/// The encoding of a payload, in a MIME-like format.
///
/// For wire and matching efficiency, common MIME types are represented using an integer as `prefix`, and a `suffix` may be used to either provide more detail, or in combination with the `Empty` prefix to write arbitrary MIME types.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_encoding_t {
    _0: [u8; 48],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_encoding_t {
    _0: [u8; 48],
}
/// An owned reply to a :c:func:`z_get`.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` (or `z_check(val)` if your compiler supports `_Generic`), which will return `true` if `val` is valid.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_reply_t {
    _0: [u8; 256],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_reply_t {
    _0: [u8; 256],
}
/// A zenoh value.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_value_t {
    _0: [u8; 88],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_value_t {
    _0: [u8; 88],
}
///
/// Queries are atomically reference-counted, letting you extract them from the callback that handed them to you by cloning.
/// `z_loaned_query_t`'s are valid as long as at least one corresponding `z_owned_query_t` exists, including the one owned by Zenoh until the callback returns.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_query_t {
    _0: [u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_query_t {
    _0: [u8; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_queryable_t {
    _0: [u8; 32],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_queryable_t {
    _0: [u8; 32],
}
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
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct ze_owned_querying_subscriber_t {
    _0: [u8; 64],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct ze_loaned_querying_subscriber_t {
    _0: [u8; 64],
}
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
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_keyexpr_t {
    _0: [u8; 32],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_view_keyexpr_t {
    _0: [u8; 32],
}
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
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_keyexpr_t {
    _0: [u8; 32],
}
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
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_session_t {
    _0: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_session_t {
    _0: [u8; 40],
}
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
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_config_t {
    _0: [u8; 1544],
}
/// A loaned zenoh configuration.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_config_t {
    _0: [u8; 1544],
}
/// Represents a Zenoh ID.
///
/// In general, valid Zenoh IDs are LSB-first 128bit unsigned and non-zero integers.
#[derive(Copy, Clone)]
#[repr(C, align(1))]
pub struct z_id_t {
    id: [u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_timestamp_t {
    _0: [u8; 24],
}
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
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_publisher_t {
    _0: [u8; 56],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_publisher_t {
    _0: [u8; 56],
}
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
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct zcu_owned_matching_listener_t {
    _0: [u8; 40],
}
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
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_subscriber_t {
    _0: [u8; 32],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_subscriber_t {
    _0: [u8; 32],
}
/// A liveliness token that can be used to provide the network with information about connectivity to its
/// declarer: when constructed, a PUT sample will be received by liveliness subscribers on intersecting key
/// expressions.
///
/// A DELETE on the token's key expression will be received by subscribers if the token is destroyed, or if connectivity between the subscriber and the token's creator is lost.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct zc_owned_liveliness_token_t {
    _0: [u8; 32],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct zc_loaned_liveliness_token_t {
    _0: [u8; 32],
}
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
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct ze_owned_publication_cache_t {
    _0: [u8; 96],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct ze_loaned_publication_cache_t {
    _0: [u8; 96],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_mutex_t {
    _0: [u8; 24],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_loaned_mutex_t {
    _0: [u8; 24],
}
#[derive(Copy, Clone)]
#[repr(C, align(4))]
pub struct z_owned_condvar_t {
    _0: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C, align(4))]
pub struct z_loaned_condvar_t {
    _0: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct z_owned_task_t {
    _0: [u8; 24],
}
