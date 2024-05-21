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
use crate::{
    errors::{self, Z_OK},
    transmute::{
        unwrap_ref_unchecked, Inplace, TransmuteCopy, TransmuteFromHandle, TransmuteIntoHandle,
        TransmuteRef, TransmuteUninitPtr,
    },
    z_closure_hello_call, z_id_t, z_owned_closure_hello_t, z_owned_config_t, z_owned_slice_array_t,
    z_view_str_t, zc_init_logger, CSlice, ZVector,
};
use async_std::task;
use libc::c_ulong;
use std::mem::MaybeUninit;
use zenoh::scouting::Hello;
use zenoh_protocol::core::{whatami::WhatAmIMatcher, WhatAmI};
use zenoh_util::core::AsyncResolve;

pub use crate::opaque_types::z_loaned_hello_t;
pub use crate::opaque_types::z_owned_hello_t;

decl_transmute_owned!(Option<Hello>, z_owned_hello_t);
decl_transmute_handle!(Hello, z_loaned_hello_t);

validate_equivalence!(z_owned_hello_t, z_loaned_hello_t);

/// Frees memory and resets hello message to its gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_drop(this: &mut z_owned_hello_t) {
    Inplace::drop(this.transmute_mut())
}

/// Borrows hello message.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_loan(this: &z_owned_hello_t) -> &z_loaned_hello_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

/// Returns ``true`` if `hello message` is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
pub extern "C" fn z_hello_check(this: &z_owned_hello_t) -> bool {
    this.transmute_ref().is_some()
}

/// Constructs hello message in a gravestone state.
#[no_mangle]
pub extern "C" fn z_hello_null(this: *mut MaybeUninit<z_owned_hello_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Returns id of Zenoh entity that transmitted hello message.
#[no_mangle]
pub extern "C" fn z_hello_zid(this: &z_loaned_hello_t) -> z_id_t {
    this.transmute_ref().zid.transmute_copy()
}

/// Returns type of Zenoh entity that transmitted hello message.
#[no_mangle]
pub extern "C" fn z_hello_whatami(this: &z_loaned_hello_t) -> z_whatami_t {
    match this.transmute_ref().whatami {
        WhatAmI::Router => z_whatami_t::Z_WHATAMI_ROUTER,
        WhatAmI::Peer => z_whatami_t::Z_WHATAMI_PEER,
        WhatAmI::Client => z_whatami_t::Z_WHATAMI_CLIENT,
    }
}

/// Constructs an array of non-owned locators (in the form non-null terminated strings) of Zenoh entity that sent hello message.
///
/// The lifetime of locator strings is bound to `this_`.
#[no_mangle]
pub extern "C" fn z_hello_locators(
    this: &z_loaned_hello_t,
    locators_out: *mut MaybeUninit<z_owned_slice_array_t>,
) {
    let this = this.transmute_ref();
    let mut locators = ZVector::with_capacity(this.locators.len());
    for l in this.locators.iter() {
        locators.push(CSlice::new_borrowed_from_slice(l.as_str().as_bytes()));
    }
    Inplace::init(locators_out.transmute_uninit_ptr(), Some(locators));
}

/// Options to pass to `z_scout()`.
#[derive(Clone)]
#[repr(C)]
pub struct z_scout_options_t {
    /// The maximum duration in ms the scouting can take.
    pub zc_timeout_ms: c_ulong,
    /// Type of entities to scout for.
    pub zc_what: z_whatami_t,
}

impl Default for z_scout_options_t {
    fn default() -> Self {
        z_scout_options_t {
            zc_timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
            zc_what: DEFAULT_SCOUTING_WHAT,
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum z_whatami_t {
    Z_WHATAMI_ROUTER = 0x01,
    Z_WHATAMI_PEER = 0x02,
    Z_WHATAMI_CLIENT = 0x04,
    Z_WHATAMI_ROUTER_PEER = 0x01 | 0x02,
    Z_WHATAMI_ROUTER_CLIENT = 0x01 | 0x04,
    Z_WHATAMI_PEER_CLIENT = 0x02 | 0x04,
    Z_WHATAMI_ROUTER_PEER_CLIENT = 0x01 | 0x02 | 0x04,
}

pub const DEFAULT_SCOUTING_WHAT: z_whatami_t = z_whatami_t::Z_WHATAMI_ROUTER_PEER;
pub const DEFAULT_SCOUTING_TIMEOUT: c_ulong = 1000;

/// Constructs the default values for the scouting operation.
#[no_mangle]
pub extern "C" fn z_scout_options_default(this: &mut z_scout_options_t) {
    *this = z_scout_options_t::default();
}

/// Scout for routers and/or peers.
///
/// @param config: A set of properties to configure scouting session.
/// @param callback: A closure that will be called on each hello message received from discoverd Zenoh entities.
/// @param options: A set of scouting options
///
/// @return 0 if successful, negative error values upon failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_scout(
    config: &mut z_owned_config_t,
    callback: &mut z_owned_closure_hello_t,
    options: Option<&z_scout_options_t>,
) -> errors::z_error_t {
    if cfg!(feature = "logger-autoinit") {
        zc_init_logger();
    }
    let options = options.cloned().unwrap_or_default();
    let what =
        WhatAmIMatcher::try_from(options.zc_what as u8).unwrap_or(WhatAmI::Router | WhatAmI::Peer);
    #[allow(clippy::unnecessary_cast)] // Required for multi-target
    let timeout = options.zc_timeout_ms as u64;
    let config = match config.transmute_mut().extract().take() {
        Some(c) => c,
        None => {
            return errors::Z_EINVAL;
        }
    };
    let mut closure = z_owned_closure_hello_t::empty();
    std::mem::swap(&mut closure, callback);

    task::block_on(async move {
        let scout = zenoh::scout(what, config)
            .callback(move |h| z_closure_hello_call(&closure, h.transmute_handle()))
            .res_async()
            .await
            .unwrap();
        async_std::task::sleep(std::time::Duration::from_millis(timeout)).await;
        std::mem::drop(scout);
    });
    Z_OK
}

/// Constructs a non-owned non-null-terminated string from the kind of zenoh entity.
///
/// The string has static storage (i.e. valid until the end of the program).
/// @param whatami: A whatami bitmask of zenoh entity kind.
/// @param str_out: An unitialized memory location where strring will be constructed.
/// @param len: Maximum number of bytes that can be written to the `buf`.
///
/// @return 0 if successful, negative error values if whatami contains an invalid bitmask.
#[no_mangle]
pub extern "C" fn z_whatami_to_str(
    whatami: z_whatami_t,
    str_out: *mut MaybeUninit<z_view_str_t>,
) -> errors::z_error_t {
    match WhatAmIMatcher::try_from(whatami as u8) {
        Err(_) => errors::Z_EINVAL,
        Ok(w) => {
            let s = w.to_str();
            let slice = CSlice::new_borrowed_from_slice(s.as_bytes());
            Inplace::init(str_out.transmute_uninit_ptr(), slice);
            errors::Z_OK
        }
    }
}
