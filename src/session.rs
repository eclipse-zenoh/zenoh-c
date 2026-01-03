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
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use zenoh::shm::ShmProviderState;
use zenoh::{Session, Wait};
use zenoh_ffi_opaque_types::opaque_types::{z_loaned_session_t, z_owned_session_t};

use crate::{
    result::{self, z_result_t},
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_moved_config_t, z_moved_session_t,
};
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
use crate::{
    shm::provider::{shared_shm_provider::SharedShmProvider, shm_provider::CSHMProvider},
    z_loaned_shm_client_storage_t, z_owned_shared_shm_provider_t,
};
#[cfg(feature = "unstable")]
use crate::{z_entity_global_id_t, zc_owned_concurrent_close_handle_t};

decl_c_type!(
    owned(z_owned_session_t, z_moved_session_t, option Session),
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

/// Moves session.
#[prebindgen("move")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_session_move(this_: &mut z_owned_session_t) -> &mut z_moved_session_t {
    std::mem::transmute(this_)
}

// Mutably borrows session.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_session_loan_mut(this_: &mut z_owned_session_t) -> &mut z_loaned_session_t {
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

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Session's provider state.
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum z_shm_provider_state {
    /// Provider is disabled by configuration.
    DISABLED,
    /// Provider is concurrently-initializing.
    INITIALIZING,
    /// Provider is ready.
    READY,
    /// Error initializing provider.
    ERROR,
}

#[cfg(all(feature = "shared-memory", feature = "unstable"))]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Each session's runtime may create its own provider to manage internal optimizations.  
/// This method exposes that provider so it can also be accessed at the application level.
///
/// Note that the provider may not be immediately available or may be disabled via configuration.
/// Provider initialization is concurrent and triggered by access events (both transport-internal and through this API).
///
/// To use this provider, both *shared_memory* and *transport_optimization* config sections must be enabled.
///
/// @param out_provider: A [`z_owned_shared_shm_provider_t`](z_owned_shared_shm_provider_t) object that will be
/// initialized from Session's provider if it exists. Initialized only if the returned value is `Z_OK`.
/// @param out_state: A [`z_shm_provider_state`](z_shm_provider_state) that indicates the status of the provider.
/// Always initialized by this function.
/// @return 0 in case if provider is avalable, negative error code otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_obtain_shm_provider(
    this: &z_loaned_session_t,
    out_provider: &mut MaybeUninit<z_owned_shared_shm_provider_t>,
    out_state: &mut MaybeUninit<z_shm_provider_state>,
) -> z_result_t {
    let s = this.as_rust_type_ref();
    match s.get_shm_provider() {
        ShmProviderState::Disabled => {
            out_state.write(z_shm_provider_state::DISABLED);
            result::Z_EUNAVAILABLE
        }
        ShmProviderState::Initializing => {
            out_state.write(z_shm_provider_state::INITIALIZING);
            result::Z_EUNAVAILABLE
        }
        ShmProviderState::Ready(provider) => {
            out_state.write(z_shm_provider_state::READY);
            out_provider
                .as_rust_type_mut_uninit()
                .write(Some(SharedShmProvider(CSHMProvider::SharedPosix(provider))));
            result::Z_OK
        }
        ShmProviderState::Error => {
            out_state.write(z_shm_provider_state::ERROR);
            result::Z_EUNAVAILABLE
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
    this: &mut std::mem::MaybeUninit<z_owned_session_t>,
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
    internal_out_concurrent:
        Option<&'static mut std::mem::MaybeUninit<zc_owned_concurrent_close_handle_t>>,

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

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the ID of the session.
#[no_mangle]
pub extern "C" fn z_session_id(session: &z_loaned_session_t) -> z_entity_global_id_t {
    use crate::transmute::IntoCType;
    session.as_rust_type_ref().id().into_c_type()
}
