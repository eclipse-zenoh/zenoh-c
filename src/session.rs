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

use crate::transmute::{unwrap_ref_unchecked, Inplace, TransmuteCopy, TransmuteRef};
use crate::{config::*, z_owned_config_t, zc_init_logger};
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::sync::{Arc, Weak};
use zenoh::core::ErrNo;
use zenoh::prelude::sync::SyncResolve;
use zenoh::session::Session;

/// An owned zenoh session.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
use crate::opaque_types::z_owned_session_t;
decl_transmute_owned!(Option<Arc<Session>>, z_owned_session_t);

/// A loaned zenoh session.
use crate::opaque_types::z_session_t;
decl_transmute_copy!(&'static Session, z_session_t);

/// Returns a :c:type:`z_session_t` loaned from `s`.
///
/// This handle doesn't increase the refcount of the session, but does allow to do so with `zc_session_rcinc`.
///
/// # Safety
/// The returned `z_session_t` aliases `z_owned_session_t`'s internal allocation,
/// attempting to use it after all owned handles to the session (including publishers, queryables and subscribers)
/// have been destroyed is UB (likely SEGFAULT)
#[no_mangle]
pub extern "C" fn z_session_loan(s: &z_owned_session_t) -> z_session_t {
    let s = s.transmute_ref();
    let s = unwrap_ref_unchecked(s);
    let s = s.as_ref();
    s.transmute_copy()
}

/// Constructs a null safe-to-drop value of 'z_owned_session_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_session_null(s: *mut MaybeUninit<z_owned_session_t>) {
    Inplace::empty(z_owned_session_t::transmute_uninit_ptr(s));
}

/// Opens a zenoh session. Should the session opening fail, `z_check` ing the returned value will return `false`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_open(
    this: *mut MaybeUninit<z_owned_session_t>,
    config: &mut z_owned_config_t,
) -> i8 {
    let this = z_owned_session_t::transmute_uninit_ptr(this);
    if cfg!(feature = "logger-autoinit") {
        zc_init_logger();
    }
    let config = match config.as_mut().take() {
        Some(c) => c,
        None => {
            log::error!("Config not provided");
            Inplace::empty(this);
            return -1;
        }
    };
    match zenoh::open(*config).res() {
        Ok(s) => {
            Inplace::init(this, Some(Arc::new(s)));
            0
        }
        Err(e) => {
            log::error!("Error opening session: {}", e);
            Inplace::empty(this);
            -1
        }
    }
}

/// Returns ``true`` if `session` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_session_check(session: &z_owned_session_t) -> bool {
    session.transmute_ref().is_some()
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
    let dst = z_owned_session_t::transmute_uninit_ptr(dst);
    let Some(src) = src.transmute_ref() else {
        return -1;
    };
    Inplace::init(dst, Some(src.clone()));
    0
}
