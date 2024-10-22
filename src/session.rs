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

use std::mem::MaybeUninit;

use zenoh::{Session, Wait};

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::z_loaned_shm_client_storage_t;
use crate::{
    opaque_types::{z_loaned_session_t, z_owned_session_t},
    result,
    transmute::{
        LoanedCTypeMut, LoanedCTypeRef, RustTypeMut, RustTypeMutUninit, RustTypeRef, TakeRustType,
    },
    z_moved_config_t, z_moved_session_t,
};
decl_c_type!(
    owned(z_owned_session_t, option Session),
    loaned(z_loaned_session_t),
);

/// Borrows session.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_session_loan(this_: &z_owned_session_t) -> &z_loaned_session_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Mutably borrows session.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_session_loan_mut(
    this_: &mut z_owned_session_t,
) -> &mut z_loaned_session_t {
    this_.as_rust_type_mut().as_loaned_c_type_mut()
}

/// Takes ownership of the mutably borrowed session
#[no_mangle]
pub extern "C" fn z_session_take_loaned(
    dst: &mut MaybeUninit<z_owned_session_t>,
    src: &mut z_loaned_session_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(std::mem::take(src.as_rust_type_mut()));
}

/// Constructs a Zenoh session in its gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_internal_session_null(this_: &mut MaybeUninit<z_owned_session_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Options passed to the `z_open()` function.
#[repr(C)]
pub struct z_open_options_t {
    _dummy: u8,
}

/// Constructs the default value for `z_open_options_t`.
#[no_mangle]
pub extern "C" fn z_open_options_default(this_: &mut MaybeUninit<z_open_options_t>) {
    this_.write(z_open_options_t { _dummy: 0 });
}

/// Constructs and opens a new Zenoh session.
///
/// @return 0 in case of success, negative error code otherwise (in this case the session will be in its gravestone state).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_open(
    this: &mut MaybeUninit<z_owned_session_t>,
    config: &mut z_moved_config_t,
    _options: Option<&z_open_options_t>,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let Some(config) = config.take_rust_type().take() else {
        tracing::error!("Config not provided");
        this.write(None);
        return result::Z_EINVAL;
    };
    match zenoh::open(config).wait() {
        Ok(s) => {
            this.write(Some(s));
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
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs and opens a new Zenoh session with specified client storage.
///
/// @return 0 in case of success, negative error code otherwise (in this case the session will be in its gravestone state).
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_open_with_custom_shm_clients(
    this: &mut MaybeUninit<z_owned_session_t>,
    config: &mut z_moved_config_t,
    shm_clients: &z_loaned_shm_client_storage_t,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let Some(config) = config.take_rust_type() else {
        tracing::error!("Config not provided");
        this.write(None);
        return result::Z_EINVAL;
    };
    match zenoh::open(config)
        .with_shm_clients(shm_clients.as_rust_type_ref().clone())
        .wait()
    {
        Ok(s) => {
            this.write(Some(s));
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
pub extern "C" fn z_internal_session_check(this_: &z_owned_session_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Options passed to the `z_close()` function.
#[repr(C)]
pub struct z_close_options_t {
    _dummy: u8,
}

/// Constructs the default value for `z_close_options_t`.
#[no_mangle]
pub extern "C" fn z_close_options_default(this_: &mut MaybeUninit<z_close_options_t>) {
    this_.write(z_close_options_t { _dummy: 0 });
}

/// Closes zenoh session. This also drops all the closure callbacks remaining from dropped, but not undeclared subscribers.
///
/// @return `0` in case of success, a negative value if an error occured while closing the session.
#[no_mangle]
pub extern "C" fn z_close(
    session: &mut z_loaned_session_t,
    _options: Option<&z_close_options_t>,
) -> result::z_result_t {
    let Some(s) = session.as_rust_type_mut() else {
        return result::Z_ENULL;
    };
    match s.close().wait() {
        Err(e) => {
            tracing::error!("Error closing session: {}", e);
            result::Z_EGENERIC
        }
        Ok(_) => result::Z_OK,
    }
}

/// Checks if zenoh session is closed.
///
/// @return `true` if session is closed, `false` otherwise.
#[no_mangle]
pub extern "C" fn z_session_is_closed(session: &z_loaned_session_t) -> bool {
    let s = session.as_rust_type_ref();
    s.is_closed()
}

/// Closes and invalidates the session.
#[no_mangle]
pub extern "C" fn z_session_drop(this_: &mut z_moved_session_t) {
    let _ = this_.take_rust_type();
}
