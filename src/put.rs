//
// Copyright (c) 2017, 2022 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//
use std::mem::MaybeUninit;

use zenoh::{
    bytes::EncodingBuilderTrait,
    qos::{CongestionControl, Priority, QoSBuilderTrait},
    sample::{SampleBuilderTrait, TimestampBuilderTrait},
    Wait,
};

#[cfg(feature = "unstable")]
use crate::z_moved_source_info_t;
#[cfg(feature = "unstable")]
use crate::z_reliability_t;
use crate::{
    commons::*,
    result,
    transmute::{IntoRustType, RustTypeRef, TakeRustType},
    z_loaned_keyexpr_t, z_loaned_session_t, z_moved_bytes_t, z_moved_encoding_t, z_timestamp_t,
};

/// Options passed to the `z_put()` function.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_put_options_t {
    /// The encoding of the message.
    pub encoding: z_moved_encoding_t,
    /// The congestion control to apply when routing this message.
    pub congestion_control: z_congestion_control_t,
    /// The priority of this message.
    pub priority: z_priority_t,
    /// If true, Zenoh will not wait to batch this operation with others to reduce the bandwith.
    pub is_express: bool,
    /// The timestamp of this message.
    pub timestamp: Option<&'static mut z_timestamp_t>,
    /// The allowed destination of this message.
    #[cfg(feature = "unstable")]
    pub allowed_destination: zc_locality_t,
    /// The reliability to apply to this message.
    #[cfg(feature = "unstable")]
    pub reliability: z_reliability_t,
    /// The source info for the message.
    #[cfg(feature = "unstable")]
    pub source_info: z_moved_source_info_t,
    /// The attachment to this message.
    pub attachment: z_moved_bytes_t,
}

/// Constructs the default value for `z_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_put_options_default(this: &mut MaybeUninit<z_put_options_t>) {
    this.write(z_put_options_t {
        encoding: None.into(),
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
        is_express: false,
        timestamp: None,
        #[cfg(feature = "unstable")]
        allowed_destination: zc_locality_default(),
        #[cfg(feature = "unstable")]
        reliability: z_reliability_default(),
        #[cfg(feature = "unstable")]
        source_info: None.into(),
        attachment: None.into(),
    });
}

/// Publishes data on specified key expression.
///
/// @param session: The Zenoh session.
/// @param key_expr: The key expression to publish to.
/// @param payload: The value to put (consumed upon function return).
/// @param options: The put options (all owned values will be consumed upon function return).
///
/// @return 0 in case of success, negative error values in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_put(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    payload: z_moved_bytes_t,
    options: Option<&mut z_put_options_t>,
) -> result::z_result_t {
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let Some(payload) = payload.into_rust_type() else {
        return result::Z_EINVAL;
    };

    let mut put = session.put(key_expr, payload);
    if let Some(options) = options {
        if let Some(encoding) = options.encoding.take_rust_type() {
            put = put.encoding(encoding);
        };
        #[cfg(feature = "unstable")]
        if let Some(source_info) = options.source_info.take_rust_type() {
            put = put.source_info(source_info);
        };
        if let Some(attachment) = options.attachment.take_rust_type() {
            put = put.attachment(attachment);
        }
        if let Some(timestamp) = options.timestamp.as_ref() {
            put = put.timestamp(Some(timestamp.into_rust_type()));
        }
        put = put.priority(options.priority.into());
        put = put.congestion_control(options.congestion_control.into());
        put = put.express(options.is_express);
        #[cfg(feature = "unstable")]
        {
            put = put
                .allowed_destination(options.allowed_destination.into())
                .reliability(options.reliability.into());
        }
    }

    if let Err(e) = put.wait() {
        tracing::error!("{}", e);
        result::Z_EGENERIC
    } else {
        result::Z_OK
    }
}

/// Options passed to the `z_delete()` function.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_delete_options_t {
    /// The congestion control to apply when routing this delete message.
    pub congestion_control: z_congestion_control_t,
    /// The priority of the delete message.
    pub priority: z_priority_t,
    /// If true, Zenoh will not wait to batch this operation with others to reduce the bandwith.
    pub is_express: bool,
    /// The timestamp of this message.
    pub timestamp: Option<&'static mut z_timestamp_t>,
    /// The allowed destination of this message.
    #[cfg(feature = "unstable")]
    pub allowed_destination: zc_locality_t,
    #[cfg(feature = "unstable")]
    /// The reliability to apply to this message.
    pub reliability: z_reliability_t,
}

/// Constructs the default value for `z_delete_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_delete_options_default(this: &mut MaybeUninit<z_delete_options_t>) {
    this.write(z_delete_options_t {
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
        is_express: false,
        timestamp: None,
        #[cfg(feature = "unstable")]
        allowed_destination: zc_locality_default(),
        #[cfg(feature = "unstable")]
        reliability: z_reliability_default(),
    });
}

/// Sends request to delete data on specified key expression (used when working with <a href="https://zenoh.io/docs/manual/abstractions/#storage"> Zenoh storages </a>).
///
/// @param session: The zenoh session.
/// @param key_expr: The key expression to delete.
/// @param options: The delete options.
///
/// @return 0 in case of success, negative values in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_delete(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    options: Option<&mut z_delete_options_t>,
) -> result::z_result_t {
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let mut del = session.delete(key_expr);
    if let Some(options) = options {
        if let Some(timestamp) = options.timestamp.as_ref() {
            del = del.timestamp(Some(timestamp.into_rust_type()));
        }
        del = del
            .congestion_control(options.congestion_control.into())
            .priority(options.priority.into())
            .express(options.is_express);

        #[cfg(feature = "unstable")]
        {
            del = del
                .allowed_destination(options.allowed_destination.into())
                .reliability(options.reliability.into());
        }
    }

    match del.wait() {
        Err(e) => {
            tracing::error!("{}", e);
            result::Z_EGENERIC
        }
        Ok(()) => result::Z_OK,
    }
}
