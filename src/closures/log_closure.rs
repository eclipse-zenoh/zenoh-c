//
// Copyright (c) 2017, 2024 ZettaScale Technology.
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

use libc::c_void;

use crate::{
    transmute::{LoanedCTypeRef, OwnedCTypeRef, TakeRustType},
    z_loaned_string_t,
};

#[repr(C)]
#[derive(PartialOrd, PartialEq)]
/// Severity level of Zenoh log message.
pub enum zc_log_severity_t {
    /// The `trace` level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    TRACE = 0,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    DEBUG = 1,
    /// The "info" level.
    ///
    /// Designates useful information.
    INFO = 2,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    WARN = 3,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    ERROR = 4,
}

impl From<zc_log_severity_t> for tracing::Level {
    fn from(value: zc_log_severity_t) -> Self {
        match value {
            zc_log_severity_t::TRACE => tracing::Level::TRACE,
            zc_log_severity_t::DEBUG => tracing::Level::DEBUG,
            zc_log_severity_t::INFO => tracing::Level::INFO,
            zc_log_severity_t::WARN => tracing::Level::WARN,
            zc_log_severity_t::ERROR => tracing::Level::ERROR,
        }
    }
}

impl From<tracing::Level> for zc_log_severity_t {
    fn from(value: tracing::Level) -> Self {
        match value {
            tracing::Level::TRACE => zc_log_severity_t::TRACE,
            tracing::Level::DEBUG => zc_log_severity_t::DEBUG,
            tracing::Level::INFO => zc_log_severity_t::INFO,
            tracing::Level::WARN => zc_log_severity_t::WARN,
            tracing::Level::ERROR => zc_log_severity_t::ERROR,
        }
    }
}
/// @brief A log-processing closure.
///
/// A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks.
#[repr(C)]
pub struct zc_owned_closure_log_t {
    _context: *mut libc::c_void,
    _call: Option<
        extern "C" fn(
            severity: zc_log_severity_t,
            msg: &z_loaned_string_t,
            context: *mut libc::c_void,
        ),
    >,
    _drop: Option<extern "C" fn(context: *mut libc::c_void)>,
}

/// Loaned closure.
#[repr(C)]
pub struct zc_loaned_closure_log_t {
    _0: [usize; 3],
}

/// Moved closure.
#[repr(C)]
pub struct zc_moved_closure_log_t {
    _this: zc_owned_closure_log_t,
}

decl_c_type!(
    owned(zc_owned_closure_log_t),
    loaned(zc_loaned_closure_log_t),
    moved(zc_moved_closure_log_t),
);

impl Default for zc_owned_closure_log_t {
    fn default() -> Self {
        zc_owned_closure_log_t {
            _context: std::ptr::null_mut(),
            _call: None,
            _drop: None,
        }
    }
}

impl zc_owned_closure_log_t {
    pub fn empty() -> Self {
        zc_owned_closure_log_t {
            _context: std::ptr::null_mut(),
            _call: None,
            _drop: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self._call.is_none() && self._drop.is_none() && self._context.is_null()
    }
}
unsafe impl Send for zc_owned_closure_log_t {}
unsafe impl Sync for zc_owned_closure_log_t {}
impl Drop for zc_owned_closure_log_t {
    fn drop(&mut self) {
        if let Some(drop) = self._drop {
            drop(self._context)
        }
    }
}
/// Constructs a closure in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zc_internal_closure_log_null(
    this_: *mut MaybeUninit<zc_owned_closure_log_t>,
) {
    (*this_).write(zc_owned_closure_log_t::default());
}
/// Calls the closure. Calling an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn zc_closure_log_call(
    closure: &zc_loaned_closure_log_t,
    severity: zc_log_severity_t,
    msg: &z_loaned_string_t,
) {
    let closure = closure.as_owned_c_type_ref();
    match closure._call {
        Some(call) => call(severity, msg, closure._context),
        None => {
            tracing::error!("Attempted to call an uninitialized closure!");
        }
    }
}
/// Drops the closure. Droping an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn zc_closure_log_drop(closure_: &mut zc_moved_closure_log_t) {
    let _ = closure_.take_rust_type();
}

/// Returns ``true`` if closure is valid, ``false`` if it is in gravestone state.
#[no_mangle]
pub extern "C" fn zc_internal_closure_log_check(this_: &zc_owned_closure_log_t) -> bool {
    !this_.is_empty()
}

/// Borrows closure.
#[no_mangle]
pub extern "C" fn zc_closure_log_loan(
    closure: &zc_owned_closure_log_t,
) -> &zc_loaned_closure_log_t {
    closure.as_loaned_c_type_ref()
}

/// @brief Constructs closure.
///
/// Closures are not guaranteed not to be called concurrently.
///
/// It is guaranteed that:
///   - `call` will never be called once `drop` has started.
///   - `drop` will only be called **once**, and **after every** `call` has ended.
///   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
///
/// @param this_: uninitialized memory location where new closure will be constructed.
/// @param call: a closure body.
/// @param drop: an optional function to be called once on closure drop.
/// @param context: closure context.
#[no_mangle]
pub extern "C" fn zc_closure_log(
    this: &mut MaybeUninit<zc_owned_closure_log_t>,
    call: Option<
        extern "C" fn(
            severity: zc_log_severity_t,
            msg: &z_loaned_string_t,
            context: *mut libc::c_void,
        ),
    >,
    drop: Option<extern "C" fn(context: *mut c_void)>,
    context: *mut c_void,
) {
    this.write(zc_owned_closure_log_t {
        _context: context,
        _call: call,
        _drop: drop,
    });
}
