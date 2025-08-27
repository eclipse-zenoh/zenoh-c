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
#![allow(non_camel_case_types)]

pub const FEATURES: &str = prebindgen_proc_macro::features!();
pub const PREBINDGEN_OUT_DIR: &str = prebindgen_proc_macro::prebindgen_out_dir!();

use std::{cmp::min, slice};

use libc::c_void;
use prebindgen_proc_macro::prebindgen;
// Reexport opaque types (z_owned_xxx_t, etc)
pub use zenoh_ffi_opaque_types::opaque_types::*;

use crate::transmute::{LoanedCTypeRef, TakeRustType};

#[macro_use]
mod transmute;

mod collections;
pub use crate::collections::*;
mod result;
pub use crate::result::*;
mod config;
pub use crate::config::*;
#[cfg(feature = "unstable")]
mod close;
#[cfg(feature = "unstable")]
pub use crate::close::*;
mod encoding;
pub use crate::encoding::*;
mod commons;
pub use crate::commons::*;
mod zbytes;
pub use crate::zbytes::*;
mod keyexpr;
pub use crate::keyexpr::*;
mod info;
pub use crate::info::*;
mod get;
pub use crate::get::*;
mod querier;
pub use crate::querier::*;
mod queryable;
pub use crate::queryable::*;
mod put;
pub use crate::put::*;
mod scouting;
pub use crate::scouting::*;
mod session;
pub use crate::session::*;
mod subscriber;
pub use crate::subscriber::*;
mod publisher;
pub use crate::publisher::*;
mod closures;
pub use closures::*;
mod platform;
pub use platform::*;
mod liveliness;
pub use liveliness::*;
mod matching;
pub use matching::*;
#[cfg(feature = "unstable")]
mod publication_cache;
#[cfg(feature = "unstable")]
pub use publication_cache::*;
#[cfg(feature = "unstable")]
mod querying_subscriber;
#[cfg(feature = "unstable")]
pub use querying_subscriber::*;
#[cfg(feature = "unstable")]
mod advanced_subscriber;
#[cfg(feature = "unstable")]
pub use advanced_subscriber::*;
#[cfg(feature = "unstable")]
mod advanced_publisher;
#[cfg(feature = "unstable")]
pub use advanced_publisher::*;
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
pub mod context;
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
pub use context::*;
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
pub mod shm;
#[cfg(all(feature = "shared-memory", feature = "unstable"))]
pub use shm::*;

mod serialization;
pub use serialization::*;

// This is the entry point for zenoh-c
// When compiling normal Rust executable, it includes rusty entry point `lang_start` that internally
// calls `std::rt::init()` that is intended to initialize some of the internals for Rust and std.
// However, when Rust library is used with non-rusty executable (this is exactly zenoh-c's case),
// there will be no rusty `lang_start` entry point and our rusty code will run without some initialization
// that is done there. In most of the cases this is OK, but sometimes lack of this initialization bites us.
// This entry point is made as a replacement for our case and contains some necessary init.
#[cfg(unix)]
#[ctor::ctor]
fn alternative_rusty_entry_point() {
    // See
    // https://github.com/eclipse-zenoh/zenoh-c/issues/294#issuecomment-2783671006
    // and
    // https://github.com/rust-lang/rust/issues/62569
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    }

    // TODO: do we need to call zenoh::try_init_log_from_env(); here???
}

/// Initializes the zenoh runtime logger, using rust environment settings.
/// E.g.: `RUST_LOG=info` will enable logging at info level. Similarly, you can set the variable to `error` or `debug`.
///
/// Note that if the environment variable is not set, then logging will not be enabled.
/// See <https://docs.rs/env_logger/latest/env_logger/index.html> for accepted filter format.
#[prebindgen]
pub fn zc_try_init_log_from_env() {
    zenoh::try_init_log_from_env();
}

/// Initializes the zenoh runtime logger, using rust environment settings or the provided fallback level.
/// E.g.: `RUST_LOG=info` will enable logging at info level. Similarly, you can set the variable to `error` or `debug`.
///
/// Note that if the environment variable is not set, then fallback filter will be used instead.
/// See <https://docs.rs/env_logger/latest/env_logger/index.html> for accepted filter format.
///
/// @param fallback_filter: The fallback filter if the `RUST_LOG` environment variable is not set.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn zc_init_log_from_env_or(fallback_filter: *const libc::c_char) -> result::z_result_t {
    match std::ffi::CStr::from_ptr(fallback_filter).to_str() {
        Ok(s) => {
            zenoh::init_log_from_env_or(s);
            result::Z_OK
        }
        Err(_) => result::Z_EINVAL,
    }
}

/// Initializes the zenoh runtime logger with custom callback.
///
/// @param min_severity: Minimum severity level of log message to be be passed to the `callback`.
/// Messages with lower severity levels will be ignored.
/// @param callback: A closure that will be called with each log message severity level and content.
#[prebindgen]
pub fn zc_init_log_with_callback(
    min_severity: zc_log_severity_t,
    callback: &mut zc_moved_closure_log_t,
) {
    let callback = callback.take_rust_type();
    zenoh_util::log::init_log_with_callback(
        move |meta| min_severity <= (*meta.level()).into(),
        move |record| {
            if let Some(s) = record.message.as_ref() {
                let c = CStringView::new_borrowed_from_slice(s.as_bytes());
                zc_closure_log_call(
                    zc_closure_log_loan(&callback),
                    record.level.into(),
                    c.as_loaned_c_type_ref(),
                );
            }
        },
    );
}

// Test should be runned with `cargo test --no-default-features`
#[test]
#[cfg(not(feature = "default"))]
fn test_no_default_features() {
    assert_eq!(
        zenoh::FEATURES,
        concat!(
            // " zenoh/auth_pubkey",
            // " zenoh/auth_usrpwd",
            // " zenoh/complete_n",
            //" zenoh/shared-memory",
            // " zenoh/stats",
            // " zenoh/transport_multilink",
            // " zenoh/transport_quic",
            // " zenoh/transport_serial",
            // " zenoh/transport_unixpipe",
            // " zenoh/transport_tcp",
            // " zenoh/transport_tls",
            // " zenoh/transport_udp",
            // " zenoh/transport_unixsock-stream",
            // " zenoh/transport_ws",
            // " zenoh/unstable",
            // " zenoh/default",
        )
    );
}

trait CopyableToCArray {
    fn copy_to_c_array(&self, buf: *mut c_void, len: usize) -> usize;
}

impl CopyableToCArray for &[u8] {
    fn copy_to_c_array(&self, buf: *mut c_void, len: usize) -> usize {
        if buf.is_null() || (len == 0 && !self.is_empty()) {
            return 0;
        }

        let max_len = min(len, self.len());
        let b = unsafe { slice::from_raw_parts_mut(buf as *mut u8, max_len) };
        b[0..max_len].copy_from_slice(&self[0..max_len]);
        max_len
    }
}

impl CopyableToCArray for &str {
    fn copy_to_c_array(&self, buf: *mut c_void, len: usize) -> usize {
        self.as_bytes().copy_to_c_array(buf, len)
    }
}

/// Stops all Zenoh tasks and drops all related static variables.
/// All Zenoh-related structures should be properly dropped/undeclared PRIOR to this call.
/// None of Zenoh functionality can be used after this call.
/// Useful to suppress memory leaks messages due to Zenoh static variables (since they are never destroyed due to Rust language design).
#[prebindgen]
pub fn zc_stop_z_runtime() {
    let _z = zenoh_runtime::ZRuntimePoolGuard;
}

#[allow(clippy::missing_safety_doc)]
pub(crate) unsafe fn strlen_or_zero(ptr: *const libc::c_char) -> usize {
    match ptr.is_null() {
        true => 0,
        false => libc::strlen(ptr),
    }
}
