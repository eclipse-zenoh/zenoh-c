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
pub use liveliness::*;
#[cfg(feature = "shared-memory")]
mod shm;

trait GuardedTransmute<D> {
    fn transmute(self) -> D;
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
