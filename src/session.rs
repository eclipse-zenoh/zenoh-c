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
use crate::config::*;
use zenoh::prelude::sync::SyncResolve;
use zenoh::Session;

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
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct z_owned_session_t([usize; 3]);

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
        unsafe { (&*self.0).as_ref() }
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

/// Opens a zenoh session. Should the session opening fail, `z_check`ing the returned value will return `false`.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_open(config: &mut z_owned_config_t) -> z_owned_session_t {
    fn ok(session: Session) -> z_owned_session_t {
        unsafe { std::mem::transmute(Some(session)) }
    }

    fn err() -> z_owned_session_t {
        unsafe { std::mem::transmute(None::<Session>) }
    }

    let config = match config.as_mut().take() {
        Some(c) => c,
        None => {
            log::error!("Config not provided");
            return err();
        }
    };
    match zenoh::open(*config).res() {
        Ok(s) => ok(s),
        Err(e) => {
            log::error!("Error opening session: {}", e);
            err()
        }
    }
}

/// Returns `true` if `session` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_session_check(session: &z_owned_session_t) -> bool {
    session.as_ref().is_some()
}

/// Closes a zenoh session. This drops and invalidates `session` for double-drop safety.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_close(session: &mut z_owned_session_t) {
    session.as_mut().take().map(|s| s.close().res());
}
