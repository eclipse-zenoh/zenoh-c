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
use crate::{config::*, impl_guarded_transmute, zc_init_logger, GuardedTransmute};
use zenoh::prelude::sync::SyncResolve;
use zenoh::Session;
use zenoh_util::core::zresult::ErrNo;

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
#[cfg(not(target_arch = "arm"))]
#[repr(C, align(8))]
pub struct z_owned_session_t([u64; 3]);

#[cfg(target_arch = "arm")]
#[repr(C, align(4))]
pub struct z_owned_session_t([u32; 3]);

impl_guarded_transmute!(Option<Session>, z_owned_session_t);

impl From<Option<Session>> for z_owned_session_t {
    fn from(val: Option<Session>) -> Self {
        val.transmute()
    }
}

impl AsRef<Option<Session>> for z_owned_session_t {
    fn as_ref(&self) -> &Option<Session> {
        unsafe { std::mem::transmute(self) }
    }
}

impl AsMut<Option<Session>> for z_owned_session_t {
    fn as_mut(&mut self) -> &mut Option<Session> {
        unsafe { std::mem::transmute(self) }
    }
}

impl AsRef<Option<Session>> for z_session_t {
    fn as_ref(&self) -> &Option<Session> {
        unsafe { (*self.0).as_ref() }
    }
}

impl From<z_session_t> for &'static z_owned_session_t {
    fn from(val: z_session_t) -> Self {
        unsafe { &*val.0 }
    }
}

impl z_owned_session_t {
    pub fn new(session: Session) -> Self {
        Some(session).into()
    }
    pub fn null() -> Self {
        None::<Session>.into()
    }
}

/// A loaned zenoh session.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct z_session_t(*const z_owned_session_t);

/// Returns a :c:type:`z_session_t` loaned from `s`.
#[no_mangle]
pub extern "C" fn z_session_loan(s: &z_owned_session_t) -> z_session_t {
    z_session_t(s)
}

/// Constructs a null safe-to-drop value of 'z_owned_session_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_session_null() -> z_owned_session_t {
    z_owned_session_t::null()
}

/// Opens a zenoh session. Should the session opening fail, `z_check` ing the returned value will return `false`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_open(config: &mut z_owned_config_t) -> z_owned_session_t {
    if cfg!(feature = "logger-autoinit") {
        zc_init_logger();
    }

    let config = match config.as_mut().take() {
        Some(c) => c,
        None => {
            log::error!("Config not provided");
            return z_owned_session_t::null();
        }
    };
    match zenoh::open(*config).res() {
        Ok(s) => z_owned_session_t::new(s),
        Err(e) => {
            log::error!("Error opening session: {}", e);
            z_owned_session_t::null()
        }
    }
}

/// Returns ``true`` if `session` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_session_check(session: &z_owned_session_t) -> bool {
    session.as_ref().is_some()
}

/// Closes a zenoh session. This drops and invalidates `session` for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_close(session: &mut z_owned_session_t) -> i8 {
    if let Some(Err(e)) = session.as_mut().take().map(|s| s.close().res()) {
        return e.errno().get();
    }
    0
}
