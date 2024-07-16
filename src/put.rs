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
use std::ptr::null_mut;

use zenoh::{
    prelude::*,
    qos::{CongestionControl, Priority},
};

use crate::{
    commons::*, encoding::*, errors, keyexpr::*, transmute::RustTypeRef, z_loaned_session_t,
    z_owned_bytes_t, z_timestamp_t,
};

/// Options passed to the `z_put()` function.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_put_options_t {
    /// The encoding of the message.
    pub encoding: *mut z_owned_encoding_t,
    /// The congestion control to apply when routing this message.
    pub congestion_control: z_congestion_control_t,
    /// The priority of this message.
    pub priority: z_priority_t,
    /// If true, Zenoh will not wait to batch this operation with others to reduce the bandwith.
    pub is_express: bool,
    /// The timestamp of this message.
    pub timestamp: *mut z_timestamp_t,
    /// The allowed destination of this message.
    pub allowed_destination: zcu_locality_t,
    /// The source info for the message.
    pub source_info: *mut z_owned_source_info_t,
    /// The attachment to this message.
    pub attachment: *mut z_owned_bytes_t,
}

/// Constructs the default value for `z_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_put_options_default(this: &mut z_put_options_t) {
    *this = z_put_options_t {
        encoding: null_mut(),
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
        is_express: false,
        timestamp: null_mut(),
        allowed_destination: zcu_locality_default(),
        source_info: null_mut(),
        attachment: null_mut(),
    };
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
    payload: &mut z_owned_bytes_t,
    options: Option<&mut z_put_options_t>,
) -> errors::z_error_t {
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let payload = std::mem::take(payload.as_rust_type_mut());

    let mut put = session.put(key_expr, payload);
    if let Some(options) = options {
        if let Some(encoding) = unsafe { options.encoding.as_mut() } {
            let encoding = std::mem::take(encoding.as_rust_type_mut());
            put = put.encoding(encoding);
        };
        if let Some(source_info) = unsafe { options.source_info.as_mut() } {
            let source_info = std::mem::take(source_info.as_rust_type_mut());
            put = put.source_info(source_info);
        };
        if let Some(attachment) = unsafe { options.attachment.as_mut() } {
            let attachment = std::mem::take(attachment.as_rust_type_mut());
            put = put.attachment(attachment);
        }
        if !options.timestamp.is_null() {
            let timestamp = *unsafe { options.timestamp.as_ref() }
                .unwrap()
                .as_rust_type_ref();
            put = put.timestamp(Some(timestamp));
        }
        put = put.priority(options.priority.into());
        put = put.congestion_control(options.congestion_control.into());
        put = put.express(options.is_express);
        put = put.allowed_destination(options.allowed_destination.into());
    }

    if let Err(e) = put.wait() {
        tracing::error!("{}", e);
        errors::Z_EGENERIC
    } else {
        errors::Z_OK
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
    pub timestamp: *mut z_timestamp_t,
    /// The allowed destination of this message.
    pub allowed_destination: zcu_locality_t,
}

/// Constructs the default value for `z_delete_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_delete_options_default(this: *mut z_delete_options_t) {
    *this = z_delete_options_t {
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
        is_express: false,
        timestamp: null_mut(),
        allowed_destination: zcu_locality_default(),
    };
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
) -> errors::z_error_t {
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let mut del = session.delete(key_expr);
    if let Some(options) = options {
        if !options.timestamp.is_null() {
            let timestamp = *unsafe { options.timestamp.as_ref() }
                .unwrap()
                .as_rust_type_ref();
            del = del.timestamp(Some(timestamp));
        }
        del = del
            .congestion_control(options.congestion_control.into())
            .priority(options.priority.into())
            .express(options.is_express)
            .allowed_destination(options.allowed_destination.into());
    }

    match del.wait() {
        Err(e) => {
            tracing::error!("{}", e);
            errors::Z_EGENERIC
        }
        Ok(()) => errors::Z_OK,
    }
}
