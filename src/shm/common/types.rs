//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use prebindgen_proc_macro::prebindgen;

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Unique protocol identifier.
/// Here is a contract: it is up to user to make sure that incompatible ShmClient
/// and ShmProviderBackend implementations will never use the same ProtocolID.
#[prebindgen]
#[allow(non_camel_case_types)]
pub type z_protocol_id_t = u32;

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Unique segment identifier.
#[prebindgen]
#[allow(non_camel_case_types)]
pub type z_segment_id_t = u32;

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Chunk id within it's segment.
#[prebindgen]
#[allow(non_camel_case_types)]
pub type z_chunk_id_t = u32;
