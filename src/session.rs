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

use crate::transmute::{
    unwrap_ref_unchecked, Inplace, TransmuteFromHandle, TransmuteIntoHandle, TransmuteRef,
    TransmuteUninitPtr,
};
use crate::{errors, z_owned_config_t, zc_init_logger};
use std::mem::MaybeUninit;
use std::sync::Arc;
use zenoh::core::Wait;
use zenoh::session::Session;

use crate::opaque_types::z_owned_session_t;
decl_transmute_owned!(Option<Arc<Session>>, z_owned_session_t);

use crate::opaque_types::z_loaned_session_t;
decl_transmute_handle!(Arc<Session>, z_loaned_session_t);
validate_equivalence!(z_owned_session_t, z_loaned_session_t);

/// Borrows session.
#[no_mangle]
pub extern "C" fn z_session_loan(this: &z_owned_session_t) -> &z_loaned_session_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
}

/// Constructs a Zenoh session in its gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_session_null(this: *mut MaybeUninit<z_owned_session_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Constructs and opens a new Zenoh session.
///
/// @return 0 in case of success, negative error code otherwise (in this case the session will be in its gravestone state).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_open(
    this: *mut MaybeUninit<z_owned_session_t>,
    config: &mut z_owned_config_t,
) -> errors::z_error_t {
    let this = this.transmute_uninit_ptr();
    if cfg!(feature = "logger-autoinit") {
        zc_init_logger();
    }
    let config = match config.transmute_mut().extract() {
        Some(c) => c,
        None => {
            log::error!("Config not provided");
            Inplace::empty(this);
            return errors::Z_EINVAL;
        }
    };
    match zenoh::open(config).wait() {
        Ok(s) => {
            Inplace::init(this, Some(Arc::new(s)));
            errors::Z_OK
        }
        Err(e) => {
            log::error!("Error opening session: {}", e);
            Inplace::empty(this);
            errors::Z_ENETWORK
        }
    }
}

/// Returns ``true`` if `session` is valid, ``false`` otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_session_check(this: &z_owned_session_t) -> bool {
    this.transmute_ref().is_some()
}

/// Closes a zenoh session. This alos drops and invalidates `session`.
///
/// @return 0 in  case of success, a negative value if an error occured while closing the session,
/// the remaining reference count (number of shallow copies) of the session otherwise, saturating at i8::MAX.
#[no_mangle]
pub extern "C" fn z_close(this: &mut z_owned_session_t) -> errors::z_error_t {
    let session = this.transmute_mut();
    let Some(s) = session.take() else {
        return errors::Z_EINVAL;
    };
    let s = match Arc::try_unwrap(s) {
        Ok(s) => s,
        Err(s) => {
            return (Arc::strong_count(&s) - 1).min(i8::MAX as usize) as i8;
        }
    };
    match s.close().wait() {
        Err(e) => {
            log::error!("Error closing session: {}", e);
            errors::Z_EGENERIC
        }
        Ok(_) => errors::Z_OK,
    }
}

/// Frees memory and invalidates the session.
///
/// This will also close the session if it does not have any clones left.
#[no_mangle]
pub extern "C" fn z_session_drop(this: &mut z_owned_session_t) {
    let _ = this.transmute_mut().extract().take();
}

/// Constructs an owned shallow copy of the session in provided uninitialized memory location.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn zc_session_clone(
    this: &z_loaned_session_t,
    dst: *mut MaybeUninit<z_owned_session_t>,
) {
    let dst = dst.transmute_uninit_ptr();
    let src = this.transmute_ref();
    Inplace::init(dst, Some(src.clone()));
}
