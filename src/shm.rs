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

use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

use libc::c_char;
use zenoh::{
    buffers::ZBuf,
    shm::{SharedMemoryBuf, SharedMemoryManager},
};

use crate::{z_session_t, zc_owned_payload_t, zc_payload_null};

#[repr(C)]
pub struct zc_owned_shm_manager_t(usize);
impl From<Option<Box<UnsafeCell<zenoh::shm::SharedMemoryManager>>>> for zc_owned_shm_manager_t {
    fn from(value: Option<Box<UnsafeCell<zenoh::shm::SharedMemoryManager>>>) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
impl Deref for zc_owned_shm_manager_t {
    type Target = Option<Box<UnsafeCell<zenoh::shm::SharedMemoryManager>>>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for zc_owned_shm_manager_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

#[no_mangle]
pub extern "C" fn zc_shm_manager_new(
    session: z_session_t,
    id: *const c_char,
    size: usize,
) -> zc_owned_shm_manager_t {
    let _ = session; // This function will likely need the session in the future, so we start doing the association now
    (move || {
        let len = unsafe { libc::strlen(id) };
        let id = unsafe { std::str::from_utf8(std::slice::from_raw_parts(id as *const u8, len)) }
            .ok()?
            .to_owned();
        Some(Box::new(UnsafeCell::new(
            SharedMemoryManager::make(id, size).ok()?,
        )))
    })()
    .into()
}

#[no_mangle]
pub extern "C" fn zc_shm_manager_drop(manager: &mut zc_owned_shm_manager_t) {
    manager.take();
}

#[no_mangle]
pub extern "C" fn zc_shm_manager_check(manager: &zc_owned_shm_manager_t) -> bool {
    manager.is_some()
}

/// Runs a garbage collection pass on the SHM manager.
///
/// Returns the number of bytes that have been freed by the pass.
///
/// # Safety
/// Calling this function concurrently with other shm functions on the same manager is UB.
#[no_mangle]
pub unsafe extern "C" fn zc_shm_gc(manager: &zc_owned_shm_manager_t) -> usize {
    if let Some(shm) = manager.as_ref() {
        unsafe { (*shm.get()).garbage_collect() }
    } else {
        0
    }
}

/// Runs a defragmentation pass on the SHM manager.
///
/// Note that this doesn't trigger a garbage collection pass, nor does it move currently allocated data.
///
/// # Safety
/// Calling this function concurrently with other shm functions on the same manager is UB.
#[no_mangle]
pub unsafe extern "C" fn zc_shm_defrag(manager: &zc_owned_shm_manager_t) -> usize {
    if let Some(shm) = manager.as_ref() {
        unsafe { (*shm.get()).defragment() }
    } else {
        0
    }
}

#[repr(C)]
#[derive(Default)]
pub struct zc_owned_shmbuf_t([usize; 9]);
impl From<UnsafeCell<Option<SharedMemoryBuf>>> for zc_owned_shmbuf_t {
    fn from(value: UnsafeCell<Option<SharedMemoryBuf>>) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
impl Deref for zc_owned_shmbuf_t {
    type Target = UnsafeCell<Option<SharedMemoryBuf>>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for zc_owned_shmbuf_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

/// Allocates a buffer of size `capacity` in the manager's memory.
///
/// # Safety
/// Calling this function concurrently with other shm functions on the same manager is UB.
#[no_mangle]
pub unsafe extern "C" fn zc_shm_alloc(
    manager: &zc_owned_shm_manager_t,
    capacity: usize,
) -> zc_owned_shmbuf_t {
    manager
        .as_ref()
        .map(|shm| unsafe {
            match (*shm.get()).alloc(capacity) {
                Ok(buf) => std::mem::transmute(buf),
                Err(_) => Default::default(),
            }
        })
        .unwrap_or_default()
}

/// Drops the SHM buffer, decrementing its backing reference counter.
#[no_mangle]
pub extern "C" fn zc_shmbuf_drop(buf: &mut zc_owned_shmbuf_t) {
    buf.get_mut().take();
}

/// Constructs an owned payload from an owned SHM buffer.
#[no_mangle]
pub extern "C" fn zc_shmbuf_into_payload(buf: &mut zc_owned_shmbuf_t) -> zc_owned_payload_t {
    match buf.get_mut().take() {
        Some(buf) => ZBuf::from(buf).try_into().unwrap_or_default(),
        None => zc_payload_null(),
    }
}

/// Returns the start of the SHM buffer.
#[no_mangle]
pub unsafe extern "C" fn zc_shmbuf_ptr(buf: &zc_owned_shmbuf_t) -> *const u8 {
    match &*buf.get() {
        None => std::ptr::null(),
        Some(buf) => buf.as_slice().as_ptr(),
    }
}

/// Returns the capacity of the SHM buffer.
#[no_mangle]
pub unsafe extern "C" fn zc_shmbuf_capacity(buf: &zc_owned_shmbuf_t) -> usize {
    match &*buf.get() {
        None => 0,
        Some(buf) => buf.info.length,
    }
}

/// Returns the length of the SHM buffer.
///
/// Note that when constructing an SHM buffer, length is defaulted to its capacity.
#[no_mangle]
pub unsafe extern "C" fn zc_shmbuf_length(buf: &zc_owned_shmbuf_t) -> usize {
    match &*buf.get() {
        None => 0,
        Some(buf) => buf.len,
    }
}

/// Sets the length of the SHM buffer.
///
/// This lets Zenoh know how much of the data to write over the network when sending the value to non-SHM-compatible neighboors.
#[no_mangle]
pub unsafe extern "C" fn zc_shmbuf_set_length(buf: &zc_owned_shmbuf_t, len: usize) {
    if let Some(buf) = &mut *buf.get() {
        buf.len = len
    }
}
