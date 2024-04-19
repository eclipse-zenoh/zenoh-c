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
use crate::keyexpr::*;
use crate::session::*;
use crate::z_owned_bytes_t;
use crate::LOG_INVALID_SESSION;
use libc::c_void;
use zenoh::encoding;
use zenoh::prelude::{sync::SyncResolve, Priority, SampleKind};
use zenoh::publication::CongestionControl;
use zenoh::sample::AttachmentBuilder;
use zenoh::sample::QoSBuilderTrait;
use zenoh::sample::SampleBuilderTrait;
use zenoh::sample::ValueBuilderTrait;

use crate::attachment::{
    insert_in_attachment_builder, z_attachment_check, z_attachment_iterate, z_attachment_null,
   z_bytes_t,
};

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
    pub encoding: z_encoding_t,
    pub congestion_control: z_congestion_control_t,
    pub priority: z_priority_t,
    pub attachment:z_bytes_t,
}

/// Constructs the default value for :c:type:`z_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_put_options_default() -> z_put_options_t {
    z_put_options_t {
        encoding: z_encoding_default(),
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
        attachment: z_attachment_null(),
    }
}

/// Put data, transfering the buffer ownership.
///
/// This is avoids copies when transfering data that was either:
/// - `zc_sample_payload_rcinc`'d from a sample, when forwarding samples from a subscriber/query to a publisher
/// - constructed from a `zc_owned_shmbuf_t`
///
/// The payload's encoding can be sepcified through the options.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression to put.
///     payload: The value to put.
///     options: The put options.
/// Returns:
///     ``0`` in case of success, negative values in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_put(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    payload: Option<&mut z_owned_bytes_t>,
    opts: Option<&z_put_options_t>,
) -> i8 {
    match session.upgrade() {
        Some(s) => {
            if let Some(payload) = payload.and_then(|p| p.take()) {
                let mut res = s.put(keyexpr, payload);
                if let Some(opts) = opts {
                    res = res
                        .encoding(**opts.encoding)
                        .congestion_control(opts.congestion_control.into())
                        .priority(opts.priority.into());
                    if z_attachment_check(&opts.attachment) {
                        let mut attachment_builder = AttachmentBuilder::new();
                        z_attachment_iterate(
                            opts.attachment,
                            insert_in_attachment_builder,
                            &mut attachment_builder as *mut AttachmentBuilder as *mut c_void,
                        );
                        res = res.attachment(attachment_builder.build());
                    };
                }
                match res.res_sync() {
                    Err(e) => {
                        log::error!("{}", e);
                        e.errno().get()
                    }
                    Ok(()) => 0,
                }
            } else {
                log::debug!("z_bytes_null was provided as payload for put");
                i8::MIN
            }
        }
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            i8::MIN
        }
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
///     keyexpr: The key expression to delete.
///     options: The put options.
/// Returns:
///     ``0`` in case of success, negative values in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_delete(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    opts: Option<&z_delete_options_t>,
) -> i8 {
    match session.upgrade() {
        Some(s) => {
            let mut res = s.delete(keyexpr);
            if let Some(opts) = opts {
                res = res
                    .congestion_control(opts.congestion_control.into())
                    .priority(opts.priority.into());
            }
            match res.res_sync() {
                Err(e) => {
                    log::error!("{}", e);
                    e.errno().get()
                }
                Ok(()) => 0,
            }
        }
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            i8::MIN
        }
    }
}
