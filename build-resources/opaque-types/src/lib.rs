use std::sync::Arc;
use zenoh::buffers::{ZBuf, ZBufReader};
use zenoh::config::Config;
use zenoh::encoding::Encoding;
use zenoh::key_expr::KeyExpr;
use zenoh::query::Reply;
use zenoh::queryable::Query;
use zenoh::queryable::Queryable;
use zenoh::sample::Sample;
use zenoh::session::Session;
use zenoh::value::Value;

// Disabled due to dependency on z_session_t. To be reworked as for autogeneration this dependency is cicrular.
// pub struct FetchingSubscriberWrapper {
//     fetching_subscriber: zenoh_ext::FetchingSubscriber<'static, ()>,
//     session: z_session_t,
// }

#[macro_export]
macro_rules! get_opaque_type_data {
    ($src_type:ty, $expr:expr) => {
        const _: () = {
            let align = std::mem::align_of::<$src_type>();
            let size = std::mem::size_of::<$src_type>();
            let mut msg: [u8; 61] =
                *b"type:                                 , align:    , size:    ";
            let mut i = 0;
            while i < 4 {
                msg[i as usize + 46] = b'0' + ((align / 10u32.pow(3 - i) as usize) % 10) as u8;
                msg[i as usize + 57] = b'0' + ((size / 10u32.pow(3 - i) as usize) % 10) as u8;
                i += 1;
            }
            let mut i: usize = 0;
            while i < $expr.len() {
                msg[i as usize + 5] = $expr.as_bytes()[i];
                i += 1;
            }
            panic!("{}", unsafe {
                std::str::from_utf8_unchecked(msg.as_slice())
            });
        };
    };
}

/// A split buffer that owns all of its data.
///
/// To minimize copies and reallocations, Zenoh may provide you data in split buffers.
get_opaque_type_data!(Option<ZBuf>, "z_owned_buffer_t");
get_opaque_type_data!(&'static ZBuf, "z_buffer_t");

/// An owned sample.
///
/// This is a read only type that can only be constructed by cloning a `z_sample_t`.
/// Like all owned types, its memory must be freed by passing a mutable reference to it to `zc_sample_drop`.
get_opaque_type_data!(Option<Sample>, "zc_owned_sample_t");

get_opaque_type_data!(&'static Sample, "z_sample_t");

/// A reader for payload data.
get_opaque_type_data!(Option<ZBufReader<'static>>, "zc_owned_payload_reader");
get_opaque_type_data!(&'static ZBufReader, "zc_payload_reader");

get_opaque_type_data!(&'static Encoding, "z_encoding_t");
get_opaque_type_data!(Encoding, "z_owned_encoding_t");

get_opaque_type_data!(Option<Reply>, "z_owned_reply_t");

get_opaque_type_data!(Value, "z_owned_value_t");
get_opaque_type_data!(&'static Value, "z_value_t");

get_opaque_type_data!(Option<Query>, "z_owned_query_t");
get_opaque_type_data!(&'static Query, "z_query_t");

get_opaque_type_data!(Option<Queryable<'static, ()>>, "z_owned_queryable_t");
get_opaque_type_data!(&'static Queryable<'static, ()>, "z_queryable_t");

// get_opaque_type_data!(
//     Option<Box<FetchingSubscriberWrapper>>,
//     "ze_owned_querying_subscriber_t"
// );
// get_opaque_type_data!(
//     &'static FetchingSubscriberWrapper,
//     "ze_querying_subscriber_t"
// );

get_opaque_type_data!(Option<KeyExpr<'static>>, "z_owned_keyexpr_t");
get_opaque_type_data!(&'static KeyExpr<'_>, "z_keyexpr_t");

get_opaque_type_data!(Option<Arc<Session>>, "z_owned_session_t");
get_opaque_type_data!(&'static Session, "z_session_t");

get_opaque_type_data!(Option<Box<Config>>, "z_owned_config_t");
get_opaque_type_data!(&'static Config, "z_config_t");
