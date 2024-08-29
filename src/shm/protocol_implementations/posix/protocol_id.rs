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

use libc::c_uint;
use zenoh::shm::POSIX_PROTOCOL_ID;

// Protocol identifier for POSIX SHM Protocol
#[no_mangle]
pub static Z_SHM_POSIX_PROTOCOL_ID: c_uint = POSIX_PROTOCOL_ID as c_uint;
