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

use std::ops::{Deref, DerefMut};

use zenoh::{
    prelude::{Priority, Value},
    publication::Publisher,
};
use zenoh_protocol_core::CongestionControl;
use zenoh_util::core::SyncResolve;

use crate::{
    z_congestion_control, z_encoding_t, z_keyexpr_t, z_priority, z_session_t, LOG_INVALID_SESSION,
};

/// The options for a publisher.
///
/// Note that `local_routing` has 3 legal values: 0 which disables it, 1 which enables it, and -1 which leaves it up to the session.
/// Other values will behave like -1, but are considered UB.
#[repr(C)]
pub struct z_publisher_options_t {
    pub local_routing: i8,
    pub congestion_control: z_congestion_control,
    pub priority: z_priority,
}
#[no_mangle]
pub extern "C" fn z_publisher_options_default() -> z_publisher_options_t {
    z_publisher_options_t {
        local_routing: -1,
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
    }
}

#[repr(C)]
pub struct z_owned_publisher_t {
    pub _align: [u64; 1],
    pub _padding: [usize; 6],
}
impl<'a> From<Option<Publisher<'a>>> for z_owned_publisher_t {
    fn from(val: Option<Publisher>) -> Self {
        unsafe { std::mem::transmute(val) }
    }
}
impl Deref for z_owned_publisher_t {
    type Target = Option<Publisher<'static>>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for z_owned_publisher_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

/// Declares a publication for the given key expression, returning `true` on success.
///
/// Written resources that match the given key will only be sent on the network
/// if matching subscribers exist in the system.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_declare_publisher(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    options: Option<&z_publisher_options_t>,
) -> z_owned_publisher_t {
    match session.as_ref() {
        Some(s) => {
            let mut p = s.declare_publisher(keyexpr);
            if let Some(options) = options {
                p = p
                    .congestion_control(options.congestion_control.into())
                    .priority(options.priority.into());
                match options.local_routing {
                    0 => p = p.local_routing(false),
                    1 => p = p.local_routing(true),
                    _ => {}
                }
            }
            match p.res_sync() {
                Err(e) => {
                    log::error!("{}", e);
                    None
                }
                Ok(publisher) => Some(publisher),
            }
        }
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            None
        }
    }
    .into()
}

#[repr(C)]
pub struct z_publisher_put_options_t {
    pub encoding: z_encoding_t,
}

/// Sends a `PUT` message onto the publisher's key expression.
///
/// Returns 0 if successful.
///
/// You may specify the payload's encoding through the options.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_publisher_put(
    publisher: &z_owned_publisher_t,
    payload: *const u8,
    len: usize,
    options: Option<&z_publisher_put_options_t>,
) -> bool {
    if let Some(p) = publisher.deref() {
        let value: Value = std::slice::from_raw_parts(payload, len).into();
        let put = match options {
            Some(options) => p.put(value.encoding(options.encoding.into())),
            None => p.put(value),
        };
        if let Err(e) = put.res_sync() {
            log::error!("{}", e);
            true
        } else {
            false
        }
    } else {
        true
    }
}
/// Sends a `DELETE` message onto the publisher's key expression.
///
/// Returns 0 if successful.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_publisher_delete(publisher: &z_owned_publisher_t) -> bool {
    if let Some(p) = publisher.deref() {
        if let Err(e) = p.delete().res_sync() {
            log::error!("{}", e);
            true
        } else {
            false
        }
    } else {
        true
    }
}

/// Undeclares a publication for the given key expression.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_undeclare_publisher(publisher: &mut z_owned_publisher_t) {
    if let Some(p) = publisher.take() {
        if let Err(e) = p.undeclare().res_sync() {
            log::error!("{}", e)
        }
    }
}
