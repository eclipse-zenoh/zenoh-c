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

mod collections;
use std::cmp::min;
use std::slice;

pub use crate::collections::*;
mod config;
pub use crate::config::*;
mod commons;
pub use crate::commons::*;
mod keyexpr;
pub use crate::keyexpr::*;
mod info;
pub use crate::info::*;
mod get;
pub use crate::get::*;
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
mod pull_subscriber;
pub use crate::pull_subscriber::*;
mod publisher;
pub use crate::publisher::*;
mod closures;
pub use closures::*;
mod liveliness;
use libc::c_void;
pub use liveliness::*;
mod publication_cache;
pub use publication_cache::*;
mod querying_subscriber;
pub use querying_subscriber::*;
pub mod attachment;
pub use platform::*;
pub mod platform;
#[cfg(feature = "shared-memory")]
pub mod shm;
#[cfg(feature = "shared-memory")]
pub use crate::shm::*;

trait GuardedTransmute<D> {
    fn transmute(self) -> D;
    fn transmute_ref(&self) -> &D;
    fn transmute_mut(&mut self) -> &mut D;
}

#[macro_export]
macro_rules! decl_rust_copy_type {
    (zenoh:($zenoh_type:ty), c:($c_type:ty)) => {
        impl_guarded_transmute!($zenoh_type, $c_type);
        impl_guarded_transmute!($c_type, $zenoh_type);

        impl From<$zenoh_type> for $c_type {
            fn from(value: $zenoh_type) -> Self {
                value.transmute()
            }
        }
        impl From<$c_type> for $zenoh_type {
            fn from(value: $c_type) -> Self {
                value.transmute()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_guarded_transmute {
    ($src_type:ty, $dst_type:ty) => {
        const _: () = {
            let src = std::mem::align_of::<$src_type>();
            let dst = std::mem::align_of::<$dst_type>();
            if src != dst {
                let mut msg: [u8; 20] = *b"src:     , dst:     ";
                let mut i = 0;
                while i < 4 {
                    msg[i as usize + 5] = b'0' + ((src / 10u32.pow(3 - i) as usize) % 10) as u8;
                    msg[i as usize + 16] = b'0' + ((dst / 10u32.pow(3 - i) as usize) % 10) as u8;
                    i += 1;
                }
                panic!("{}", unsafe {
                    std::str::from_utf8_unchecked(msg.as_slice())
                });
            }
        };

        impl $crate::GuardedTransmute<$dst_type> for $src_type {
            fn transmute(self) -> $dst_type {
                unsafe { std::mem::transmute::<$src_type, $dst_type>(self) }
            }

            fn transmute_ref(&self) -> &$dst_type {
                unsafe { std::mem::transmute::<&$src_type, &$dst_type>(self) }
            }

            fn transmute_mut(&mut self) -> &mut $dst_type {
                unsafe { std::mem::transmute::<&mut $src_type, &mut $dst_type>(self) }
            }
        }
    };
}

pub(crate) const LOG_INVALID_SESSION: &str = "Invalid session";

/// Initialises the zenoh runtime logger.
///
/// Note that unless you built zenoh-c with the `logger-autoinit` feature disabled,
/// this will be performed automatically by `z_open` and `z_scout`.
#[no_mangle]
pub extern "C" fn zc_init_logger() {
    let _ = env_logger::try_init();
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
            " zenoh/shared-memory",
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
            " zenoh/unstable",
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
