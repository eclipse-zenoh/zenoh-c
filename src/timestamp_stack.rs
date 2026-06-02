//
// Copyright (c) 2026 ZettaScale Technology
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

//! @warning This API has been marked as unstable: it works as advertised, but it may be changed
//! in a future release.
//!
//! Timestamp instrumentation — opt-in end-to-end latency measurement for Zenoh messages.

use std::{mem::MaybeUninit, ptr::null, sync::Arc};

use libc::c_void;
use zenoh::timestamp_stack::{
    InterceptionPoint, SessionTimestampCallback, TimestampInstrumentation,
    TimestampStack as RustTimestampStack, TsStackContext,
};

// Re-export so that put.rs / get.rs / publisher.rs can import via `timestamp_stack::`.
pub use crate::opaque_types::z_loaned_timestamp_instrumentation_t;
use crate::{
    get::{z_loaned_reply_err_t, z_loaned_reply_t},
    opaque_types::{
        z_loaned_sample_t, z_loaned_timestamp_stack_record_t, z_loaned_timestamp_stack_t,
        z_moved_timestamp_instrumentation_t, z_owned_timestamp_instrumentation_t, z_timestamp_t,
    },
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
};

// ── z_interception_point_t ────────────────────────────────────────────────────

/// @warning This API has been marked as unstable.
///
/// Which interception point a timestamp record was captured at.
/// New variants may be added in future releases; treat unknown values as `UNKNOWN`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum z_interception_point_t {
    Send = 0,
    Route = 1,
    Receive = 2,
    Unknown = 255,
}

impl From<InterceptionPoint> for z_interception_point_t {
    fn from(v: InterceptionPoint) -> Self {
        match v {
            InterceptionPoint::Send => Self::Send,
            InterceptionPoint::Route => Self::Route,
            InterceptionPoint::Receive => Self::Receive,
            _ => Self::Unknown,
        }
    }
}

// ── z_owned_timestamp_instrumentation_t ──────────────────────────────────────
// Types are defined in opaque_types (generated at build time with correct sizes).
// TimestampInstrumentation has no niche, so z_loaned_t (1 byte) ≠ z_owned_t (2 bytes).
// Use decl_c_type_inequal! to skip the loaned==owned size assertion.
decl_c_type_inequal!(
    owned(z_owned_timestamp_instrumentation_t, option TimestampInstrumentation),
    loaned(z_loaned_timestamp_instrumentation_t, TimestampInstrumentation),
);

/// @warning This API has been marked as unstable.
///
/// Constructs a timestamp instrumentation config specifying which points to record.
///
/// @param this_: An uninitialized location to write the result to.
/// @param send: Record timestamps at the SEND point.
/// @param route: Record timestamps at the ROUTE point.
/// @param receive: Record timestamps at the RECEIVE point.
/// @return 0 on success, negative error code if all flags are false.
#[no_mangle]
pub extern "C" fn z_timestamp_instrumentation_new(
    this_: &mut MaybeUninit<z_owned_timestamp_instrumentation_t>,
    send: bool,
    route: bool,
    receive: bool,
) -> crate::result::z_result_t {
    let this = this_.as_rust_type_mut_uninit();
    match TimestampInstrumentation::new(send, route, receive) {
        Ok(instr) => {
            this.write(Some(instr));
            crate::result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            this.write(None);
            crate::result::Z_EINVAL
        }
    }
}

/// @warning This API has been marked as unstable.
///
/// Constructs a null (invalid) timestamp instrumentation.
#[no_mangle]
pub extern "C" fn z_internal_timestamp_instrumentation_null(
    this_: &mut MaybeUninit<z_owned_timestamp_instrumentation_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable.
///
/// Returns ``true`` if the instrumentation config is valid.
#[no_mangle]
pub extern "C" fn z_internal_timestamp_instrumentation_check(
    this_: &z_owned_timestamp_instrumentation_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable.
///
/// Borrows the instrumentation config.
///
/// # Safety
/// Caller must ensure `this_` is a valid, initialized pointer.
#[no_mangle]
pub unsafe extern "C" fn z_timestamp_instrumentation_loan(
    this_: &z_owned_timestamp_instrumentation_t,
) -> &z_loaned_timestamp_instrumentation_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable.
///
/// Drops the instrumentation config.
#[no_mangle]
pub extern "C" fn z_timestamp_instrumentation_drop(
    this_: &mut z_moved_timestamp_instrumentation_t,
) {
    let _ = this_.take_rust_type();
}

// ── z_timestamp_stack_t / z_loaned_timestamp_stack_t ─────────────────────────
// z_loaned_timestamp_stack_t is defined in opaque_types (correct size from build).

decl_c_type!(loaned(z_loaned_timestamp_stack_t, RustTimestampStack),);

/// @warning This API has been marked as unstable.
///
/// Returns the number of records in the timestamp stack.
#[no_mangle]
pub extern "C" fn z_timestamp_stack_record_count(stack: &z_loaned_timestamp_stack_t) -> usize {
    stack.as_rust_type_ref().records().len()
}

/// @warning This API has been marked as unstable.
///
/// Returns the record at the given index, or NULL if out of bounds.
#[no_mangle]
pub extern "C" fn z_timestamp_stack_record_at(
    stack: &z_loaned_timestamp_stack_t,
    index: usize,
) -> *const z_loaned_timestamp_stack_record_t {
    let records = stack.as_rust_type_ref().records();
    match records.get(index) {
        Some(r) => r.as_loaned_c_type_ref() as *const z_loaned_timestamp_stack_record_t,
        None => null(),
    }
}

// ── z_loaned_timestamp_stack_record_t ────────────────────────────────────────
// z_loaned_timestamp_stack_record_t is defined in opaque_types (correct size from build).

decl_c_type!(loaned(
    z_loaned_timestamp_stack_record_t,
    zenoh::timestamp_stack::TimestampStackRecord
),);

/// @warning This API has been marked as unstable.
///
/// Returns the interception point for this record.
#[no_mangle]
pub extern "C" fn z_timestamp_stack_record_point(
    record: &z_loaned_timestamp_stack_record_t,
) -> z_interception_point_t {
    record.as_rust_type_ref().point().into()
}

/// @warning This API has been marked as unstable.
///
/// Returns ``true`` if the timestamp was produced by a user-defined callback.
#[no_mangle]
pub extern "C" fn z_timestamp_stack_record_is_custom(
    record: &z_loaned_timestamp_stack_record_t,
) -> bool {
    record.as_rust_type_ref().is_custom()
}

/// @warning This API has been marked as unstable.
///
/// Returns a pointer to the raw timestamp bytes and sets `*len` to the byte count.
/// The returned pointer is valid for the lifetime of the record. Do not free it.
#[no_mangle]
pub extern "C" fn z_timestamp_stack_record_timestamp(
    record: &z_loaned_timestamp_stack_record_t,
    len: &mut usize,
) -> *const u8 {
    let bytes = record.as_rust_type_ref().timestamp();
    *len = bytes.len();
    bytes.as_ptr()
}

/// @warning This API has been marked as unstable.
///
/// Parses the record as a standard UHLC timestamp.
///
/// Writes the result to `*out` on success. On failure (custom timestamp or malformed bytes),
/// sets `*out` to an invalid timestamp and returns a negative error code.
#[no_mangle]
pub extern "C" fn z_timestamp_stack_record_as_timestamp(
    record: &z_loaned_timestamp_stack_record_t,
    out: &mut MaybeUninit<z_timestamp_t>,
) -> crate::result::z_result_t {
    use crate::transmute::IntoCType;
    match record.as_rust_type_ref().as_timestamp() {
        Some(ts) => {
            out.write(ts.into_c_type());
            crate::result::Z_OK
        }
        None => crate::result::Z_EINVAL,
    }
}

// ── Sample/Reply/ReplyError accessors ────────────────────────────────────────

/// @warning This API has been marked as unstable.
///
/// Returns a loaned pointer to the timestamp stack on a sample, or NULL if not present.
#[no_mangle]
pub extern "C" fn z_sample_timestamp_stack(
    this_: &z_loaned_sample_t,
) -> *const z_loaned_timestamp_stack_t {
    match this_.as_rust_type_ref().timestamp_stack() {
        Some(ts) => ts.as_loaned_c_type_ref() as *const z_loaned_timestamp_stack_t,
        None => null(),
    }
}

/// @warning This API has been marked as unstable.
///
/// Returns a loaned pointer to the timestamp stack on a successful reply's sample,
/// or NULL if not present.
///
/// # Safety
/// Caller must ensure `this_` is a valid, initialized pointer.
#[no_mangle]
pub unsafe extern "C" fn z_reply_timestamp_stack(
    this_: &z_loaned_reply_t,
) -> *const z_loaned_timestamp_stack_t {
    let reply = this_.as_rust_type_ref();
    match reply.result() {
        Ok(sample) => match sample.timestamp_stack() {
            Some(ts) => ts.as_loaned_c_type_ref() as *const z_loaned_timestamp_stack_t,
            None => null(),
        },
        Err(_) => null(),
    }
}

/// @warning This API has been marked as unstable.
///
/// Returns a loaned pointer to the timestamp stack on a reply error,
/// or NULL if not present.
#[no_mangle]
pub extern "C" fn z_reply_err_timestamp_stack(
    this_: &z_loaned_reply_err_t,
) -> *const z_loaned_timestamp_stack_t {
    match this_.as_rust_type_ref().timestamp_stack() {
        Some(ts) => ts.as_loaned_c_type_ref() as *const z_loaned_timestamp_stack_t,
        None => null(),
    }
}

// ── z_owned_session_ts_callback_t ────────────────────────────────────────────

/// @warning This API has been marked as unstable.
///
/// A session-level callback for generating custom timestamps.
///
/// Registered once at `z_open()` time. Receives a context describing the current node
/// and interception point, and returns raw timestamp bytes.
/// Return a zero-length result (set `*len = 0`) to skip stamping this point.
#[repr(C)]
pub struct z_owned_session_ts_callback_t {
    pub _context: *mut c_void,
    pub _call: Option<
        unsafe extern "C" fn(
            zid: *const u8,
            zid_len: usize,
            whatami: u8,
            point: z_interception_point_t,
            out_ts: *mut u8,
            out_len: *mut usize,
            out_capacity: usize,
            context: *mut c_void,
        ),
    >,
    pub _drop: Option<unsafe extern "C" fn(context: *mut c_void)>,
}

/// @warning This API has been marked as unstable.
///
/// Moved session timestamp callback.
#[repr(C)]
pub struct z_moved_session_ts_callback_t {
    pub _this: z_owned_session_ts_callback_t,
}

impl Default for z_owned_session_ts_callback_t {
    fn default() -> Self {
        Self {
            _context: std::ptr::null_mut(),
            _call: None,
            _drop: None,
        }
    }
}

unsafe impl Send for z_owned_session_ts_callback_t {}
unsafe impl Sync for z_owned_session_ts_callback_t {}

impl Drop for z_owned_session_ts_callback_t {
    fn drop(&mut self) {
        if let Some(drop_fn) = self._drop {
            unsafe { drop_fn(self._context) };
        }
    }
}

impl z_owned_session_ts_callback_t {
    pub fn is_empty(&self) -> bool {
        self._call.is_none()
    }

    pub fn into_rust_callback(self) -> Option<SessionTimestampCallback> {
        let call = self._call?;
        // Cast to usize so the closure is Send+Sync (usize is always Send+Sync).
        // Reconstructing the pointer inside the closure is safe because the C caller
        // guarantees the context pointer is valid and thread-safe for the session lifetime.
        let ctx_addr: usize = self._context as usize;
        Some(Arc::new(move |ctx: TsStackContext| {
            let context = ctx_addr as *mut c_void;
            let zid_bytes = ctx.zid.to_le_bytes();
            let whatami: u8 = ctx.whatami as u8;
            let point: z_interception_point_t = ctx.interception_point.into();
            let mut buf = vec![0u8; 128];
            let mut out_len: usize = 0;
            unsafe {
                call(
                    zid_bytes.as_ptr(),
                    zid_bytes.len(),
                    whatami,
                    point,
                    buf.as_mut_ptr(),
                    &mut out_len,
                    buf.len(),
                    context,
                );
            }
            buf.truncate(out_len);
            buf
        }))
    }
}

/// @warning This API has been marked as unstable.
///
/// Constructs a null (empty) session timestamp callback.
#[no_mangle]
pub extern "C" fn z_internal_session_ts_callback_null(
    this_: &mut MaybeUninit<z_owned_session_ts_callback_t>,
) {
    this_.write(z_owned_session_ts_callback_t::default());
}

/// @warning This API has been marked as unstable.
///
/// Returns ``true`` if the callback is non-null.
#[no_mangle]
pub extern "C" fn z_internal_session_ts_callback_check(
    this_: &z_owned_session_ts_callback_t,
) -> bool {
    !this_.is_empty()
}

/// @warning This API has been marked as unstable.
///
/// Drops the session timestamp callback (calls the drop function if set).
#[no_mangle]
pub extern "C" fn z_session_ts_callback_drop(this_: &mut z_moved_session_ts_callback_t) {
    let _ = std::mem::take(&mut this_._this);
}
