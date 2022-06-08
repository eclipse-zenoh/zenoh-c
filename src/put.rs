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
use libc::{c_int, size_t};
use zenoh::prelude::{sync::SyncResolve, Priority, SampleKind};
use zenoh::publication::CongestionControl;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_priority {
    REAL_TIME = 1,
    INTERACTIVE_HIGH = 2,
    INTERACTIVE_LOW = 3,
    DATA_HIGH = 4,
    DATA = 5,
    DATA_LOW = 6,
    BACKGROUND = 7,
}

impl From<Priority> for z_priority {
    fn from(p: Priority) -> Self {
        match p {
            Priority::RealTime => z_priority::REAL_TIME,
            Priority::InteractiveHigh => z_priority::INTERACTIVE_HIGH,
            Priority::InteractiveLow => z_priority::INTERACTIVE_LOW,
            Priority::DataHigh => z_priority::DATA_HIGH,
            Priority::Data => z_priority::DATA,
            Priority::DataLow => z_priority::DATA_LOW,
            Priority::Background => z_priority::BACKGROUND,
        }
    }
}

impl From<z_priority> for Priority {
    fn from(p: z_priority) -> Self {
        match p {
            z_priority::REAL_TIME => Priority::RealTime,
            z_priority::INTERACTIVE_HIGH => Priority::InteractiveHigh,
            z_priority::INTERACTIVE_LOW => Priority::InteractiveLow,
            z_priority::DATA_HIGH => Priority::DataHigh,
            z_priority::DATA => Priority::Data,
            z_priority::DATA_LOW => Priority::DataLow,
            z_priority::BACKGROUND => Priority::Background,
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum z_congestion_control {
    BLOCK,
    DROP,
}

impl From<CongestionControl> for z_congestion_control {
    fn from(cc: CongestionControl) -> Self {
        match cc {
            CongestionControl::Block => z_congestion_control::BLOCK,
            CongestionControl::Drop => z_congestion_control::DROP,
        }
    }
}

impl From<z_congestion_control> for CongestionControl {
    fn from(cc: z_congestion_control) -> Self {
        match cc {
            z_congestion_control::BLOCK => CongestionControl::Block,
            z_congestion_control::DROP => CongestionControl::Drop,
        }
    }
}

/// Options passed to the :c:func:`z_put_ext` function.
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_put_options_t {
    encoding: z_encoding_t,
    congestion_control: z_congestion_control,
    priority: z_priority,
}

/// Constructs the default value for write options
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_put_options_default() -> z_put_options_t {
    z_put_options_t {
        encoding: z_encoding_default(),
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
    }
}

/// Write data with extended options.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression to write.
///     payload: The value to write.
///     len: The length of the value to write.
///     options: The write options
/// Returns:
///     ``0`` in case of success, ``1`` in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_put(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    payload: *const u8,
    len: size_t,
    mut opts: *const z_put_options_t,
) -> c_int {
    const fn ok() -> c_int {
        true as c_int
    }

    const fn err() -> c_int {
        false as c_int
    }

    match session.as_ref() {
        Some(s) => {
            let default = z_put_options_default();
            if opts.is_null() {
                opts = &default;
            }
            match s
                .put(keyexpr, std::slice::from_raw_parts(payload, len as usize))
                .encoding((*opts).encoding)
                .kind(SampleKind::Put)
                .congestion_control((*opts).congestion_control.into())
                .priority((*opts).priority.into())
                .res_sync()
            {
                Err(e) => {
                    log::error!("{}", e);
                    err()
                }
                Ok(()) => ok(),
            }
        }
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            err()
        }
    }
}
