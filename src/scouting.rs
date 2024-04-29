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
    errors::{self, Z_OK}, transmute::{unwrap_ref_unchecked, Inplace, TransmuteCopy, TransmuteFromHandle, TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr}, z_closure_hello_call, z_config_check, z_config_clone, z_config_default, z_config_drop, z_config_null, z_id_t, z_loaned_config_t, z_owned_closure_hello_t, z_owned_config_t, z_owned_slice_array_t, zc_init_logger, CopyableToCArray, ZVector
};
use async_std::task;
use libc::{c_char, c_ulong};
use std::{borrow::Cow, mem::MaybeUninit, os::raw::c_void};
use zenoh::scouting::Hello;
use zenoh_protocol::core::{whatami::WhatAmIMatcher, WhatAmI};
use zenoh_util::core::AsyncResolve;

pub use crate::opaque_types::z_owned_hello_t;
pub use crate::opaque_types::z_loaned_hello_t;

decl_transmute_owned!(Option<Hello>, z_owned_hello_t);
decl_transmute_handle!(Hello, z_loaned_hello_t);


/// Frees `hello`, invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_drop(this: &mut z_owned_hello_t) {
    Inplace::drop(this.transmute_mut())
}

/// Returns a :c:type:`z_hello_t` loaned from :c:type:`z_owned_hello_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_loan(this: &z_owned_hello_t) -> &z_loaned_hello_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

#[no_mangle]
pub extern "C" fn z_hello_check(this: &z_owned_hello_t) -> bool {
    this.transmute_ref().is_some()
}

#[no_mangle]
pub extern "C" fn z_hello_null(this: *mut MaybeUninit<z_owned_hello_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}


#[no_mangle]
pub extern "C" fn z_hello_zid(this: &z_loaned_hello_t) -> z_id_t {
    this.transmute_ref().zid.transmute_copy()
}

#[no_mangle]
pub extern "C" fn z_hello_whatami(this: &z_loaned_hello_t) -> u8 {
    this.transmute_ref().whatami as u8
}

/// Returns an array of non-owned locators as an array of non-null terminated strings.
/// 
/// The lifetime of locator strings is bound to `this`.
#[no_mangle]
pub extern "C" fn z_hello_locators(this: &z_loaned_hello_t, locators_out: *mut MaybeUninit<z_owned_slice_array_t>) {
    let this = this.transmute_ref();
    let mut locators = ZVector::with_capacity(this.locators.len());
    for l in this.locators.iter() {
        locators.push(Cow::Borrowed(l.as_str().as_bytes()));
    }
    Inplace::init(locators_out.transmute_uninit_ptr(), Some(locators));
}

#[repr(C)]
pub struct z_owned_scouting_config_t {
    _config: z_owned_config_t,
    pub zc_timeout_ms: c_ulong,
    pub zc_what: u8,
}

pub const DEFAULT_SCOUTING_WHAT: u8 = WhatAmI::Router as u8 | WhatAmI::Peer as u8;
pub const DEFAULT_SCOUTING_TIMEOUT: c_ulong = 1000;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_scouting_config_null(this: *mut MaybeUninit<z_owned_scouting_config_t>) {
    let mut _config = MaybeUninit::<z_owned_config_t>::uninit();
    z_config_null(&mut _config as *mut MaybeUninit<z_owned_config_t>);
    let _config = _config.assume_init();

    let config = z_owned_scouting_config_t {
        _config,
        zc_timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
        zc_what: DEFAULT_SCOUTING_WHAT,
    };
    (*this).write(config);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_scouting_config_default(
    this: *mut MaybeUninit<z_owned_scouting_config_t>,
) {
    let mut _config = MaybeUninit::<z_owned_config_t>::uninit();
    z_config_default(&mut _config as *mut MaybeUninit<z_owned_config_t>);
    let _config = _config.assume_init();

    let config = z_owned_scouting_config_t {
        _config,
        zc_timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
        zc_what: DEFAULT_SCOUTING_WHAT,
    };
    (*this).write(config);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_scouting_config_from(
    this: *mut MaybeUninit<z_owned_scouting_config_t>,
    config: &z_loaned_config_t,
) {
    let mut dst = MaybeUninit::uninit();
    z_config_clone(config, &mut dst as *mut _);
    let _config = dst.assume_init();

    let config = z_owned_scouting_config_t {
        _config,
        zc_timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
        zc_what: DEFAULT_SCOUTING_WHAT,
    };
    (*this).write(config);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_scouting_config_check(config: &z_owned_scouting_config_t) -> bool {
    z_config_check(&config._config)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_scouting_config_drop(config: &mut z_owned_scouting_config_t) {
    z_config_drop(&mut config._config)
}

/// Scout for routers and/or peers.
///
/// Parameters:
///     what: A whatami bitmask of zenoh entities kind to scout for.
///     config: A set of properties to configure the scouting.
///     timeout: The time (in milliseconds) that should be spent scouting.
///
/// Returns 0 if successful, negative values upon failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_scout(
    config: &mut z_owned_scouting_config_t,
    callback: &mut z_owned_closure_hello_t,
) -> errors::z_error_t {
    if cfg!(feature = "logger-autoinit") {
        zc_init_logger();
    }
    let what = WhatAmIMatcher::try_from(config.zc_what).unwrap_or(WhatAmI::Router | WhatAmI::Peer);
    #[allow(clippy::unnecessary_cast)] // Required for multi-target
    let timeout = config.zc_timeout_ms as u64;
    let config = match config._config.transmute_mut().extract().take() {
        Some(c) => c,
        None => {
            return errors::Z_EINVAL;
        }
    };
    let mut closure = z_owned_closure_hello_t::empty();
    std::mem::swap(&mut closure, callback);

    task::block_on(async move {
        let scout = zenoh::scout(what, config)
            .callback(move |h| {
                z_closure_hello_call(&closure, h.transmute_handle())
            })
            .res_async()
            .await
            .unwrap();
        async_std::task::sleep(std::time::Duration::from_millis(timeout)).await;
        std::mem::drop(scout);
    });
    Z_OK
}

/// Converts the kind of zenoh entity into a string.
///
/// Parameters:
///     whatami: A whatami bitmask of zenoh entity kind.
///     buf: Buffer to write a null-terminated string to.
///     len: Maximum number of bytes that can be written to the `buf`.
///
/// Returns 0 if successful, negative values if whatami contains an invalid bitmask or `buf` is null,
/// or number of remaining bytes, if the null-terminated string size exceeds `len`.
#[no_mangle]
pub extern "C" fn z_whatami_to_str(whatami: u8, buf: *mut c_char, len: usize) -> i8 {
    if buf.is_null() || len == 0 {
        return errors::Z_EINVAL;
    }
    match WhatAmIMatcher::try_from(whatami) {
        Err(_) => errors::Z_EINVAL,
        Ok(w) => {
            let s = w.to_str();
            let res = s.copy_to_c_array(buf as *mut c_void, len - 1);
            unsafe {
                *((buf as usize + res) as *mut c_char) = 0;
            }
            (s.len() - res) as i8
        }
    }
}
