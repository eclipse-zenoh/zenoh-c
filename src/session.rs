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
use prebindgen_proc_macro::prebindgen;
use zenoh::{Session, Wait};

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::z_loaned_shm_client_storage_t;
#[cfg(feature = "unstable")]
use crate::zc_owned_concurrent_close_handle_t;
use crate::{
    opaque_types::{z_loaned_session_t, z_owned_session_t},
    result::{self, z_result_t},
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_moved_config_t, z_moved_session_t,
};
decl_c_type!(
    owned(z_owned_session_t, option Session),
    loaned(z_loaned_session_t),
);

/// Borrows session.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_session_loan(this_: &z_owned_session_t) -> &z_loaned_session_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

// Mutably borrows session.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_session_loan_mut(
    this_: &mut z_owned_session_t,
) -> &mut z_loaned_session_t {
    this_
        .as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// Constructs a Zenoh session in its gravestone state.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub fn z_internal_session_null(this_: &mut std::mem::MaybeUninit<z_owned_session_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Options passed to the `z_open()` function.
#[prebindgen]
#[repr(C)]
pub struct z_open_options_t {
    _dummy: u8,
}

/// Constructs the default value for `z_open_options_t`.
#[prebindgen]
pub fn z_open_options_default(this_: &mut std::mem::MaybeUninit<z_open_options_t>) {
    this_.write(z_open_options_t { _dummy: 0 });
}

/// Constructs and opens a new Zenoh session.
///
/// @return 0 in case of success, negative error code otherwise (in this case the session will be in its gravestone state).
#[allow(clippy::missing_safety_doc)]
#[prebindgen]
pub fn z_open(
    this: &mut std::mem::MaybeUninit<z_owned_session_t>,
    config: &mut z_moved_config_t,
    _options: Option<&z_open_options_t>,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let Some(config) = config.take_rust_type() else {
        crate::report_error!("Config not provided");
        this.write(None);
        return result::Z_EINVAL;
    };
    match zenoh::open(config).wait() {
        Ok(s) => {
            this.write(Some(s));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("Error opening session: {}", e);
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
#[prebindgen]
pub fn z_open_with_custom_shm_clients(
    this: &mut MaybeUninit<z_owned_session_t>,
    config: &mut z_moved_config_t,
    shm_clients: &z_loaned_shm_client_storage_t,
) -> z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let Some(config) = config.take_rust_type() else {
        crate::report_error!("Config not provided");
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
            crate::report_error!("Error opening session: {}", e);
            this.write(None);
            result::Z_ENETWORK
        }
    }
}

/// Returns ``true`` if `session` is valid, ``false`` otherwise.
#[allow(clippy::missing_safety_doc)]
#[prebindgen]
pub fn z_internal_session_check(this_: &z_owned_session_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Options passed to the `z_close()` function.
#[prebindgen]
#[repr(C)]
pub struct z_close_options_t {
    #[cfg(feature = "unstable")]
    #[doc(hidden)]
    /// The timeout for close operation in milliseconds. 0 means default close timeout which is 10 seconds.
    internal_timeout_ms: u32,

    #[cfg(feature = "unstable")]
    #[doc(hidden)]
    /// An optional uninitialized concurrent close handle. If set, the close operation will be executed
    /// concurrently in separate task, and this handle will be initialized to be used for controlling
    /// it's execution.
    internal_out_concurrent: Option<&'static mut std::mem::MaybeUninit<zc_owned_concurrent_close_handle_t>>,

    #[cfg(not(feature = "unstable"))]
    _dummy: u8,
}

/// Constructs the default value for `z_close_options_t`.
#[prebindgen]
#[allow(unused)]
pub fn z_close_options_default(this_: &mut std::mem::MaybeUninit<z_close_options_t>) {
    this_.write(z_close_options_t {
        #[cfg(feature = "unstable")]
        internal_timeout_ms: 0,
        #[cfg(feature = "unstable")]
        internal_out_concurrent: None,
        #[cfg(not(feature = "unstable"))]
        _dummy: 0,
    });
}

/// Closes Zenoh session. This also drops all the closure callbacks remaining from not yet dropped or undeclared Zenoh entites (subscribers, queriers, etc).
/// After this operation, all calls for network operations for entites declared on this session will return a error.
///
/// @return `0` in case of success, a negative value if an error occured while closing the session.
#[prebindgen]
pub fn z_close(
    session: &mut z_loaned_session_t,
    #[allow(unused)] options: Option<&mut z_close_options_t>,
) -> z_result_t {
    #[allow(unused_mut)]
    let mut close_builder = session.as_rust_type_mut().close();

    #[cfg(feature = "unstable")]
    if let Some(options) = options {
        if options.internal_timeout_ms != 0 {
            close_builder = close_builder.timeout(core::time::Duration::from_millis(
                options.internal_timeout_ms as u64,
            ))
        }

        if let Some(close_handle) = &mut options.internal_out_concurrent {
            let handle = close_builder.in_background().wait();
            close_handle.as_rust_type_mut_uninit().write(Some(handle));
            return result::Z_OK;
        }
    }

    match close_builder.wait() {
        Err(e) => {
            crate::report_error!("Error closing session: {}", e);
            result::Z_EGENERIC
        }
        Ok(_) => result::Z_OK,
    }
}

/// Checks if zenoh session is closed.
///
/// @return `true` if session is closed, `false` otherwise.
#[prebindgen]
pub fn z_session_is_closed(session: &z_loaned_session_t) -> bool {
    let s = session.as_rust_type_ref();
    s.is_closed()
}

/// Closes and invalidates the session.
#[prebindgen]
pub fn z_session_drop(this_: &mut z_moved_session_t) {
    let _ = this_.take_rust_type();
}
