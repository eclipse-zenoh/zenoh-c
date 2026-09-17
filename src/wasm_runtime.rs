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

// wasm32 without atomics (no threads at all) can't use zenoh-c's normal synchronous C API: every
// `z_*` function that calls `.wait()` internally relies on `block_in_place`, which panics there.
// This module gives the embedder (e.g. an RMW layer driven by a JS/Emscripten main loop) a
// non-blocking alternative: create one `zc_runtime_t`, drive it with `zc_runtime_pump_once()`
// from an external loop, and poll long-running operations (currently just session open) instead
// of awaiting them synchronously.
#![cfg(target_arch = "wasm32")]

use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc};

use zenoh::Session;

use crate::{
    result,
    transmute::{RustTypeRefUninit, TakeRustType},
    z_moved_config_t, z_owned_session_t,
};

/// A single-threaded Tokio runtime owned by the embedder, used to drive zenoh's async internals
/// on targets with no real threads (e.g. `wasm32-unknown-emscripten` without `-pthread`).
///
/// The embedder must call `zc_runtime_pump_once()` repeatedly, from its own externally-driven
/// loop (e.g. `emscripten_set_main_loop_arg`), for anything to actually happen: no zenoh-c call
/// on this target performs real I/O by itself.
///
/// Also owns a `LocalSet`: some internal zenoh tasks (e.g. `zenoh-ext`'s `AdvancedSubscriber`
/// garbage-collector) are spawned with `tokio::task::spawn_local()` on this target rather than a
/// regular `spawn()`, since it has no `Send` bound to satisfy on a single-threaded runtime.
/// `spawn_local()` panics unless a `LocalSet` is active *at the point it's called* -- which, for
/// a synchronous zenoh-c call like `ze_declare_advanced_subscriber()`, is whenever the embedder
/// happens to call it, not just from inside this module's own pumped futures. So the `LocalSet`
/// is entered once, permanently, for this runtime's whole lifetime (`_enter_guard`), in addition
/// to being polled via `LocalSet::run_until()` inside `pump_once_local()` so that tasks spawned
/// onto it actually make progress.
pub struct zc_runtime_t {
    runtime: tokio::runtime::Runtime,
    // Struct fields drop in declaration order: this goes before `local_set` so the thread-local
    // "current LocalSet" pointer is cleared before `local_set` itself is torn down. Doesn't
    // actually borrow `local_set` (`LocalEnterGuard` just clones an `Rc`), so storing both here
    // is fine regardless, but the order still reads as the intended cleanup sequence.
    _enter_guard: tokio::task::LocalEnterGuard,
    local_set: tokio::task::LocalSet,
    // A future that never completes, reused across calls: `pump_once()` only runs its "ready
    // local tasks" pass (the actual point of `zc_runtime_pump_once()` -- draining the TX
    // pipeline, delivering subscriber callbacks, etc.) when the top-level future it's given
    // does *not* resolve this pump.
    idle: Pin<Box<dyn Future<Output = ()>>>,
}

/// Wraps `future` so that polling it also polls any tasks spawned onto `local_set` via
/// `spawn_local()`, then drives it forward by one non-blocking pump on `runtime`. Returns
/// whether `future` completed this pump, mirroring `Runtime::pump_once()`.
///
/// A free function, not a method on `zc_runtime_t`, so callers can pass a future borrowed from
/// one of that struct's own fields without the borrow checker treating this as re-borrowing the
/// whole struct.
fn pump_once_local(
    runtime: &tokio::runtime::Runtime,
    local_set: &tokio::task::LocalSet,
    future: Pin<&mut dyn Future<Output = ()>>,
) -> bool {
    let wrapped = local_set.run_until(future);
    let mut wrapped: Pin<Box<dyn Future<Output = ()>>> = Box::pin(wrapped);
    runtime.pump_once(wrapped.as_mut())
}

/// Creates a new single-threaded runtime for driving zenoh on targets without real threads.
///
/// Returns NULL on failure. The returned pointer must eventually be passed to
/// `zc_runtime_free()`.
#[no_mangle]
pub extern "C" fn zc_runtime_new() -> *mut zc_runtime_t {
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    else {
        return std::ptr::null_mut();
    };
    let local_set = tokio::task::LocalSet::new();
    let _enter_guard = local_set.enter();
    Box::into_raw(Box::new(zc_runtime_t {
        runtime,
        _enter_guard,
        local_set,
        idle: Box::pin(std::future::pending()),
    }))
}

/// Drops a runtime created by `zc_runtime_new()`. Safe to call with NULL.
#[no_mangle]
pub unsafe extern "C" fn zc_runtime_free(rt: *mut zc_runtime_t) {
    if !rt.is_null() {
        drop(Box::from_raw(rt));
    }
}

/// Drives `rt` forward by one non-blocking step: runs any zenoh-internal tasks that are ready
/// (e.g. delivering a subscriber callback, flushing a batch to the wire), then returns
/// immediately regardless of what became ready.
///
/// Call this repeatedly from an external loop; it never blocks and never sleeps.
#[no_mangle]
pub unsafe extern "C" fn zc_runtime_pump_once(rt: *mut zc_runtime_t) {
    let rt = &mut *rt;
    let _ = pump_once_local(&rt.runtime, &rt.local_set, rt.idle.as_mut());
}

type OpenResult = Result<Session, Box<dyn std::error::Error + Send + Sync>>;

/// A pending, resumable `zenoh::open()` call. Poll it with `zc_open_poll()`.
pub struct zc_open_task_t {
    future: Pin<Box<dyn Future<Output = ()>>>,
    result: Rc<RefCell<Option<OpenResult>>>,
}

/// Starts opening a session without blocking. `config` is consumed either way.
///
/// Returns NULL if `config` was already moved-from; otherwise returns a task that must be
/// polled with `zc_open_poll()` (alongside `zc_runtime_pump_once()`) until it reports the
/// session is ready, then passed to `zc_open_task_free()`.
#[no_mangle]
pub extern "C" fn zc_open_start(config: &mut z_moved_config_t) -> *mut zc_open_task_t {
    let Some(config) = config.take_rust_type() else {
        return std::ptr::null_mut();
    };
    let result: Rc<RefCell<Option<OpenResult>>> = Rc::new(RefCell::new(None));
    let result_for_future = result.clone();
    let future = Box::pin(async move {
        let opened = zenoh::open(config).await;
        *result_for_future.borrow_mut() = Some(opened);
    });
    Box::into_raw(Box::new(zc_open_task_t { future, result }))
}

/// Polls a task started by `zc_open_start()`. Does not itself drive I/O — call
/// `zc_runtime_pump_once()` in the same loop so the underlying connection can actually progress.
///
/// Returns:
/// - `-1` if the session is not open yet: call again after the next `zc_runtime_pump_once()`.
/// - `0` on success: `out_session` is initialized and `task` should be dropped.
/// - a negative `z_result_t` (`< -1`) on failure: `out_session` is left uninitialized and `task`
///   should be dropped; see the log for details.
#[no_mangle]
pub unsafe extern "C" fn zc_open_poll(
    rt: *const zc_runtime_t,
    task: *mut zc_open_task_t,
    out_session: &mut std::mem::MaybeUninit<z_owned_session_t>,
) -> i8 {
    let rt = &*rt;
    let task = &mut *task;
    if !pump_once_local(&rt.runtime, &rt.local_set, task.future.as_mut()) {
        return -1;
    }
    match task.result.borrow_mut().take() {
        Some(Ok(session)) => {
            out_session.as_rust_type_mut_uninit().write(Some(session));
            0
        }
        Some(Err(e)) => {
            crate::report_error!("Error opening session: {}", e);
            result::Z_ENETWORK as i8
        }
        // `future` resolved (pump_once returned true) without ever setting `result`: can't
        // happen barring a bug in the async block above, since it's the only thing that
        // completes the future.
        None => result::Z_EUNAVAILABLE as i8,
    }
}

/// Drops a task returned by `zc_open_start()`, whether or not it ever completed. Safe to call
/// with NULL.
#[no_mangle]
pub unsafe extern "C" fn zc_open_task_free(task: *mut zc_open_task_t) {
    if !task.is_null() {
        drop(Box::from_raw(task));
    }
}

