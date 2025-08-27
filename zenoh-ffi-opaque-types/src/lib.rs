//
// Copyright (c) 2025 ZettaScale Technology.
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
pub mod opaque_types;
mod probe;

pub const FEATURES: &str = prebindgen_proc_macro::features!();
pub const PREBINDGEN_OUT_DIR: &str = prebindgen_proc_macro::prebindgen_out_dir!();

use crate::opaque_types::z_id_t;

impl From<[u8; 16]> for z_id_t {
    fn from(value: [u8; 16]) -> Self {
        z_id_t { id: value }
    }
}
