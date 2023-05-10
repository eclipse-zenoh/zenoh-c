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

use std::cell::UnsafeCell;

use libc::c_char;
use zenoh::shm::SharedMemoryManager;

use crate::z_session_t;

#[repr(C)]
pub struct zc_owned_shm_manager_t(Option<Box<UnsafeCell<zenoh::shm::SharedMemoryManager>>>);

#[no_mangle]
pub extern "C" fn zc_shm_manager_new(
    session: z_session_t,
    id: *const c_char,
    size: usize,
) -> zc_owned_shm_manager_t {
    let _ = session; // This function will likely need the session in the future, so we start doing the association now
    zc_owned_shm_manager_t((move || {
        let len = unsafe { libc::strlen(id) };
        let id = unsafe { std::str::from_utf8(std::slice::from_raw_parts(id as *const u8, len)) }
            .ok()?
            .to_owned();
        Some(Box::new(UnsafeCell::new(
            SharedMemoryManager::make(id, size).ok()?,
        )))
    })())
}

#[repr(C)]
#[derive(Default)]
pub struct zc_owned_shmbuf_t([usize; 9]);

#[no_mangle]
pub extern "C" fn zc_shm_alloc(shm: &zc_owned_shm_manager_t, capacity: usize) -> zc_owned_shmbuf_t {
    shm.0
        .as_ref()
        .map(|shm| unsafe {
            match (*shm.get()).alloc(capacity) {
                Ok(buf) => std::mem::transmute(buf),
                Err(_) => Default::default(),
            }
        })
        .unwrap_or_default()
}
