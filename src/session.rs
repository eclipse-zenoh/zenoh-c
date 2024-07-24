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

use std::{mem::MaybeUninit, sync::Arc};

use zenoh::{prelude::*, session::Session};

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::z_loaned_shm_client_storage_t;
use crate::{
    opaque_types::{z_loaned_session_t, z_owned_session_t},
    result,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_owned_config_t, zc_init_logger,
};
decl_c_type!(
    owned(z_owned_session_t, Option<Arc<Session>>),
    loaned(z_loaned_session_t, Arc<Session>)
);

/// Borrows session.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_session_loan(this: &z_owned_session_t) -> &z_loaned_session_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Constructs a Zenoh session in its gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_session_null(this: &mut MaybeUninit<z_owned_session_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Constructs and opens a new Zenoh session.
///
/// @return 0 in case of success, negative error code otherwise (in this case the session will be in its gravestone state).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_open(
    this: &mut MaybeUninit<z_owned_session_t>,
    config: &mut z_owned_config_t,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    if cfg!(feature = "logger-autoinit") {
        zc_init_logger();
    }
    let Some(config) = config.as_rust_type_mut().take() else {
        tracing::error!("Config not provided");
        this.write(None);
        return result::Z_EINVAL;
    };
    match zenoh::open(config).wait() {
        Ok(s) => {
            this.write(Some(Arc::new(s)));
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("Error opening session: {}", e);
            this.write(None);
            result::Z_ENETWORK
        }
    }
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// Constructs and opens a new Zenoh session with specified client storage.
///
/// @return 0 in case of success, negative error code otherwise (in this case the session will be in its gravestone state).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_open_with_custom_shm_clients(
    this: &mut MaybeUninit<z_owned_session_t>,
    config: &mut z_owned_config_t,
    shm_clients: &z_loaned_shm_client_storage_t,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    if cfg!(feature = "logger-autoinit") {
        zc_init_logger();
    }
    let Some(config) = config.as_rust_type_mut().take() else {
        tracing::error!("Config not provided");
        this.write(None);
        return result::Z_EINVAL;
    };
    match zenoh::open(config)
        .with_shm_clients(shm_clients.as_rust_type_ref().clone())
        .wait()
    {
        Ok(s) => {
            this.write(Some(Arc::new(s)));
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("Error opening session: {}", e);
            this.write(None);
            result::Z_ENETWORK
        }
    }
}

/// Returns ``true`` if `session` is valid, ``false`` otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_session_check(this: &z_owned_session_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Closes a zenoh session. This alos drops and invalidates `session`.
///
/// @return 0 in  case of success, a negative value if an error occured while closing the session,
/// the remaining reference count (number of shallow copies) of the session otherwise, saturating at i8::MAX.
#[no_mangle]
pub extern "C" fn z_close(this: &mut z_owned_session_t) -> result::z_result_t {
    let session = this.as_rust_type_mut();
    let Some(s) = session.take() else {
        return result::Z_EINVAL;
    };
    let s = match Arc::try_unwrap(s) {
        Ok(s) => s,
        Err(s) => {
            return (Arc::strong_count(&s) - 1).min(i8::MAX as usize) as i8;
        }
    };
    match s.close().wait() {
        Err(e) => {
            tracing::error!("Error closing session: {}", e);
            result::Z_EGENERIC
        }
        Ok(_) => result::Z_OK,
    }
}

/// Frees memory and invalidates the session.
///
/// This will also close the session if it does not have any clones left.
#[no_mangle]
pub extern "C" fn z_session_drop(this: &mut z_owned_session_t) {
    *this.as_rust_type_mut() = None;
}

/// Constructs an owned shallow copy of the session in provided uninitialized memory location.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_session_clone(
    dst: &mut MaybeUninit<z_owned_session_t>,
    this: &z_loaned_session_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(Some(this.as_rust_type_ref().clone()));
}
