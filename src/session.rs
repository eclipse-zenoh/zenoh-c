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
    unwrap_ref_unchecked, Inplace, TransmuteIntoHandle, TransmuteRef, TransmuteUninitPtr,
};
use crate::{errors, z_owned_config_t, zc_init_logger};
use std::mem::MaybeUninit;
use std::sync::Arc;
use zenoh::core::ErrNo;
use zenoh::prelude::sync::SyncResolve;
use zenoh::session::Session;

use crate::opaque_types::z_owned_session_t;
decl_transmute_owned!(Option<Arc<Session>>, z_owned_session_t);

/// A loaned zenoh session.
use crate::opaque_types::z_session_t;
decl_transmute_handle!(Session, z_session_t);

/// Returns a :c:type:`z_session_t` loaned from `s`.
///
/// This handle doesn't increase the refcount of the session, but does allow to do so with `zc_session_rcinc`.
///
/// # Safety
/// The returned `z_session_t` aliases `z_owned_session_t`'s internal allocation,
/// attempting to use it after all owned handles to the session (including publishers, queryables and subscribers)
/// have been destroyed is UB (likely SEGFAULT)
#[no_mangle]
pub extern "C" fn z_session_loan(this: &'static z_owned_session_t) -> z_session_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    let this = this.as_ref();
    this.transmute_handle()
}

/// Constructs a null safe-to-drop value of 'z_owned_session_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_session_null(this: *mut MaybeUninit<z_owned_session_t>) {
    Inplace::empty(this.transmute_uninit_ptr());
}

/// Opens a zenoh session. Should the session opening fail, `z_check` ing the returned value will return `false`.
/// Config value is always consumed upon function return.
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
    let config = match config.transmute_mut().take() {
        Some(c) => c,
        None => {
            log::error!("Config not provided");
            Inplace::empty(this);
            return errors::Z_EINVAL;
        }
    };
    match zenoh::open(*config).res() {
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

/// Returns ``true`` if `session` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_session_check(this: &z_owned_session_t) -> bool {
    this.transmute_ref().is_some()
}

/// Closes a zenoh session. This drops and invalidates `session` for double-drop safety.
///
/// Returns a negative value if an error occured while closing the session.
/// Returns the remaining reference count of the session otherwise, saturating at i8::MAX.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_close(session: &mut z_owned_session_t) -> i8 {
    let session = session.transmute_mut();
    let Some(s) = session.take() else {
        return 0;
    };
    let s = match Arc::try_unwrap(s) {
        Ok(s) => s,
        Err(s) => {
            return (Arc::strong_count(&s) - 1).min(i8::MAX as usize) as i8;
        }
    };
    match s.close().res() {
        Err(e) => e.errno().get(),
        Ok(_) => 0,
    }
}

/// Increments the session's reference count, returning a new owning handle.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn zc_session_rcinc(
    dst: *mut MaybeUninit<z_owned_session_t>,
    src: &z_owned_session_t,
) -> i8 {
    // session.as_ref().as_ref().and_then(|s| s.upgrade()).into()
    let dst = dst.transmute_uninit_ptr();
    let Some(src) = src.transmute_ref() else {
        return -1;
    };
    Inplace::init(dst, Some(src.clone()));
    0
}
