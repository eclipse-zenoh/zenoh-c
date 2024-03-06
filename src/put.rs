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
use crate::LOG_INVALID_SESSION;
use libc::c_void;
use libc::size_t;
use zenoh::prelude::{sync::SyncResolve, Priority, SampleKind};
use zenoh::publication::CongestionControl;
use zenoh::sample::AttachmentBuilder;
use zenoh_util::core::zresult::ErrNo;

use crate::attachment::{
    insert_in_attachment_builder, z_attachment_check, z_attachment_iterate, z_attachment_null,
    z_attachment_t,
};

/// The priority of zenoh messages.
///
///     - **REAL_TIME**
///     - **INTERACTIVE_HIGH**
///     - **INTERACTIVE_LOW**
///     - **DATA_HIGH**
///     - **DATA**
///     - **DATA_LOW**
///     - **BACKGROUND**
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
/// tags{c.z_priority_t, api.options.priority}
pub enum z_priority_t {
    /// tags{c.z_priority_t.real_time, api.options.priority.real_time}
    REAL_TIME = 1,
    /// tags{c.z_priority_t.interactive_high, api.options.priority.interactive_high}
    INTERACTIVE_HIGH = 2,
    /// tags{c.z_priority_t.interactive_low, api.options.priority.interactive_low}
    INTERACTIVE_LOW = 3,
    /// tags{c.z_priority_t.data_high, api.options.priority.data_high}
    DATA_HIGH = 4,
    /// tags{c.z_priority_t.data, api.options.priority.data}
    DATA = 5,
    /// tags{c.z_priority_t.data_low, api.options.priority.data_low}
    DATA_LOW = 6,
    /// tags{c.z_priority_t.background, api.options.priority.background}
    BACKGROUND = 7,
}

impl From<Priority> for z_priority_t {
    fn from(p: Priority) -> Self {
        match p {
            Priority::RealTime => z_priority_t::REAL_TIME,
            Priority::InteractiveHigh => z_priority_t::INTERACTIVE_HIGH,
            Priority::InteractiveLow => z_priority_t::INTERACTIVE_LOW,
            Priority::DataHigh => z_priority_t::DATA_HIGH,
            Priority::Data => z_priority_t::DATA,
            Priority::DataLow => z_priority_t::DATA_LOW,
            Priority::Background => z_priority_t::BACKGROUND,
        }
    }
}

impl From<z_priority_t> for Priority {
    fn from(p: z_priority_t) -> Self {
        match p {
            z_priority_t::REAL_TIME => Priority::RealTime,
            z_priority_t::INTERACTIVE_HIGH => Priority::InteractiveHigh,
            z_priority_t::INTERACTIVE_LOW => Priority::InteractiveLow,
            z_priority_t::DATA_HIGH => Priority::DataHigh,
            z_priority_t::DATA => Priority::Data,
            z_priority_t::DATA_LOW => Priority::DataLow,
            z_priority_t::BACKGROUND => Priority::Background,
        }
    }
}

/// The kind of congestion control.
///
///     - **BLOCK**
///     - **DROP**
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
/// tags{c.z_congestion_control_t, api.options.congestion_control}
pub enum z_congestion_control_t {
    /// tags{c.z_congestion_control_t.block, api.options.congestion_control.block}
    BLOCK,
    /// tags{c.z_congestion_control_t.drop, api.options.congestion_control.drop}
    DROP,
}

impl From<CongestionControl> for z_congestion_control_t {
    fn from(cc: CongestionControl) -> Self {
        match cc {
            CongestionControl::Block => z_congestion_control_t::BLOCK,
            CongestionControl::Drop => z_congestion_control_t::DROP,
        }
    }
}

impl From<z_congestion_control_t> for CongestionControl {
    fn from(cc: z_congestion_control_t) -> Self {
        match cc {
            z_congestion_control_t::BLOCK => CongestionControl::Block,
            z_congestion_control_t::DROP => CongestionControl::Drop,
        }
    }
}

/// Options passed to the :c:func:`z_put` function.
///
/// Members:
///     z_encoding_t encoding: The encoding of the payload.
///     z_congestion_control_t congestion_control: The congestion control to apply when routing this message.
///     z_priority_t priority: The priority of this message.
///     z_attachment_t attachment: The attachment to this message.
#[repr(C)]
#[allow(non_camel_case_types)]
/// tags{c.z_put_options_t}
pub struct z_put_options_t {
    /// tags{c.z_put_options_t.encoding, api.put.encoding.set}
    pub encoding: z_encoding_t,
    /// tags{c.z_put_options_t.congestion_control, api.put.congestion_control.set}
    pub congestion_control: z_congestion_control_t,
    /// tags{c.z_put_options_t.priority, api.put.priority.set}
    pub priority: z_priority_t,
    /// tags{c.z_put_options_t.attachment, api.put.attachment.set}
    pub attachment: z_attachment_t,
}

/// Constructs the default value for :c:type:`z_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_put_options_default}
pub extern "C" fn z_put_options_default() -> z_put_options_t {
    z_put_options_t {
        encoding: z_encoding_default(),
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
        attachment: z_attachment_null(),
    }
}

/// Put data.
///
/// The payload's encoding can be sepcified through the options.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression to put.
///     payload: The value to put.
///     len: The length of the value to put.
///     options: The put options.
/// Returns:
///     ``0`` in case of success, negative values in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_put, api.session.put}
pub unsafe extern "C" fn z_put(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    payload: *const u8,
    len: size_t,
    opts: Option<&z_put_options_t>,
) -> i8 {
    match session.upgrade() {
        Some(s) => {
            let mut res = s
                .put(keyexpr, std::slice::from_raw_parts(payload, len))
                .kind(SampleKind::Put);
            if let Some(opts) = opts {
                res = res
                    .encoding(opts.encoding)
                    .congestion_control(opts.congestion_control.into())
                    .priority(opts.priority.into());
                if z_attachment_check(&opts.attachment) {
                    let mut attachment_builder = AttachmentBuilder::new();
                    z_attachment_iterate(
                        opts.attachment,
                        insert_in_attachment_builder,
                        &mut attachment_builder as *mut AttachmentBuilder as *mut c_void,
                    );
                    res = res.with_attachment(attachment_builder.build());
                };
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
/// tags{c.zc_put_owned, api.session.put}
pub extern "C" fn zc_put_owned(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    payload: Option<&mut zc_owned_payload_t>,
    opts: Option<&z_put_options_t>,
) -> i8 {
    match session.upgrade() {
        Some(s) => {
            if let Some(payload) = payload.and_then(|p| p.take()) {
                let mut res = s.put(keyexpr, payload).kind(SampleKind::Put);
                if let Some(opts) = opts {
                    res = res
                        .encoding(opts.encoding)
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
            } else {
                log::debug!("zc_payload_null was provided as payload for put");
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
/// tags{c.z_delete_options_t}
pub struct z_delete_options_t {
    /// tags{c.z_delete_options_t.congestion_control, api.delete.congestion_control.set}
    pub congestion_control: z_congestion_control_t,
    /// tags{c.z_delete_options_t.priority, api.delete.priority.set}
    pub priority: z_priority_t,
}

/// Constructs the default value for :c:type:`z_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
/// tags{c.z_delete_options_default}
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
/// tags{c.z_delete, api.session.delete}
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
