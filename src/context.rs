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

use libc::c_void;
use std::fmt::Debug;

/// A trait for implementing droppable contexts
pub trait DroppableContext: Debug {
    fn get(&self) -> *mut c_void;
}

/// A non-tread-safe droppable context.
/// Contexts are idiomatically used in C together with callback interfaces to deliver associated state
/// information to each callback.
///
/// This is a non-thread-safe context - zenoh-c guarantees that associated callbacks that share the same
/// zc_context_t instance will never be executed concurrently. In other words, all the callbacks associated
/// with this context data are not required to be thread-safe.
///
/// NOTE: Remember that the same callback interfaces associated with different zc_context_t instances can
/// still be executed concurrently. The exact behavior depends on user's application, but we strongly
/// discourage our users from pinning to some specific behavior unless they _really_ understand what they
/// are doing.
///
/// Once moved to zenoh-c ownership, this context is guaranteed to execute delete_fn when deleted. The
/// delete_fn is guaranteed to be executed only once at some point of time after the last associated
/// callback call returns.
/// NOTE: if user doesn't pass the instance of this context to zenoh-c, the delete_fn callback won't
/// be executed.
#[derive(Debug)]
#[repr(C)]
pub struct zc_context_t {
    context: *mut c_void,
    delete_fn: unsafe extern "C" fn(*mut c_void),
}

impl From<zc_context_t> for Context {
    fn from(value: zc_context_t) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Context(zc_context_t);
impl DroppableContext for Context {
    fn get(&self) -> *mut c_void {
        self.0.context
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            (self.0.delete_fn)(self.0.context);
        }
    }
}

/// A tread-safe droppable context.
/// Contexts are idiomatically used in C together with callback interfaces to deliver associated state
/// information to each callback.
///
/// This is a thread-safe context - the associated callbacks may be executed concurrently with the same
/// zc_context_t instance. In other words, all the callbacks associated with this context data MUST be
/// thread-safe.
///
/// Once moved to zenoh-c ownership, this context is guaranteed to execute delete_fn when deleted.The
/// delete_fn is guaranteed to be executed only once at some point of time after the last associated
/// callback call returns.
/// NOTE: if user doesn't pass the instance of this context to zenoh-c, the delete_fn callback won't
/// be executed.
#[derive(Debug)]
#[repr(C)]
pub struct zc_threadsafe_context_t {
    context: zc_threadsafe_context_data_t,
    delete_fn: unsafe extern "C" fn(*mut c_void),
}

impl From<zc_threadsafe_context_t> for ThreadsafeContext {
    fn from(value: zc_threadsafe_context_t) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct zc_threadsafe_context_data_t {
    ptr: *mut c_void,
}
unsafe impl Send for zc_threadsafe_context_data_t {}
unsafe impl Sync for zc_threadsafe_context_data_t {}

#[derive(Debug)]
#[repr(transparent)]
pub struct ThreadsafeContext(zc_threadsafe_context_t);
impl DroppableContext for ThreadsafeContext {
    fn get(&self) -> *mut c_void {
        self.0.context.ptr
    }
}
impl Drop for ThreadsafeContext {
    fn drop(&mut self) {
        unsafe {
            (self.0.delete_fn)(self.0.context.ptr);
        }
    }
}
