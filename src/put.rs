use std::ptr::null_mut;

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
use crate::commons::*;
use crate::errors;
use crate::keyexpr::*;
use crate::transmute::Inplace;
use crate::transmute::TransmuteFromHandle;
use crate::transmute::TransmuteRef;
use crate::z_owned_bytes_t;
use crate::z_session_t;
use zenoh::prelude::{sync::SyncResolve, Priority};
use zenoh::publication::CongestionControl;
use zenoh::sample::QoSBuilderTrait;
use zenoh::sample::SampleBuilderTrait;
use zenoh::sample::ValueBuilderTrait;

/// Options passed to the :c:func:`z_put` function.
///
/// Members:
///     z_encoding_t encoding: The encoding of the payload.
///     z_congestion_control_t congestion_control: The congestion control to apply when routing this message.
///     z_priority_t priority: The priority of this message.
///    z_bytes_t attachment: The attachment to this message.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_put_options_t {
    pub encoding: *mut z_owned_encoding_t,
    pub congestion_control: z_congestion_control_t,
    pub priority: z_priority_t,
    pub attachment: *mut z_owned_bytes_t,
}

/// Constructs the default value for :c:type:`z_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_put_options_default() -> z_put_options_t {
    z_put_options_t {
        encoding: null_mut(),
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
        attachment: null_mut(),
    }
}

/// Put data, transfering its ownership.
///
///
/// The payload's encoding and attachment can be sepcified through the options. These values are consumed upon function
/// return.
///
/// Parameters:
///     session: The zenoh session.
///     key_expr: The key expression to put.
///     payload: The value to put (consumed upon function return).
///     options: The put options.
/// Returns:
///     ``0`` in case of success, negative error values in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_put(
    session: z_session_t,
    key_expr: z_keyexpr_t,
    payload: &mut z_owned_bytes_t,
    options: Option<&mut z_put_options_t>,
) -> errors::z_error_t {
    let session = session.transmute_ref();
    let key_expr = key_expr.transmute_ref();
    let payload = match payload.transmute_mut().extract() {
        Some(p) => p,
        None => {
            log::debug!("Attempted to put with a null payload");
            return errors::Z_EINVAL;
        }
    };

    let mut put = session.put(key_expr, payload);
    if let Some(options) = options {
        if !options.encoding.is_null() {
            let encoding = unsafe { *options.encoding }.transmute_mut().extract();
            put = put.encoding(encoding);
        };
        if !options.attachment.is_null() {
            let attachment = unsafe { *options.attachment }.transmute_mut().extract();
            put = put.attachment(attachment);
        }
    }

    if let Err(e) = put.res_sync() {
        log::error!("{}", e);
        errors::Z_EGENERIC
    } else {
        errors::Z_OK
    }
}

/// Options passed to the :c:func:`z_delete` function.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_delete_options_t {
    pub congestion_control: z_congestion_control_t,
    pub priority: z_priority_t,
}

/// Constructs the default value for :c:type:`z_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_delete_options_default() -> z_delete_options_t {
    z_delete_options_t {
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
    }
}

/// Delete data.
///
/// Parameters:
///     session: The zenoh session.
///     key_expr: The key expression to delete.
///     options: The delete options.
/// Returns:
///     ``0`` in case of success, negative values in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_delete(
    session: z_session_t,
    key_expr: z_keyexpr_t,
    options: Option<&mut z_delete_options_t>,
) -> errors::z_error_t {
    let session = session.transmute_ref();
    let key_expr = key_expr.transmute_ref();
    let mut del = session
        .delete(key_expr);
    if let Some(options) = options {
        del = del.congestion_control(options.congestion_control.into())
            .priority(options.priority.into());
    }

    match del.res_sync() {
        Err(e) => {
            log::error!("{}", e);
            errors::Z_EGENERIC
        }
        Ok(()) => errors::Z_OK,
    }
}
