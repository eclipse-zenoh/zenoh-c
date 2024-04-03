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

use crate::zcu_closure_matching_status_call;
use crate::zcu_owned_closure_matching_status_t;
use std::ops::{Deref, DerefMut};
use zenoh::prelude::SessionDeclarations;
use zenoh::{
    prelude::{Priority, Value},
    publication::MatchingListener,
    publication::Publisher,
    sample::AttachmentBuilder,
};
use zenoh_protocol::core::CongestionControl;
use zenoh_util::core::{zresult::ErrNo, SyncResolve};

use crate::attachment::{
    insert_in_attachment_builder, z_attachment_check, z_attachment_iterate, z_attachment_null,
    z_attachment_t,
};
use libc::c_void;

use crate::{
    impl_guarded_transmute, z_congestion_control_t, z_encoding_default, z_encoding_t, z_keyexpr_t,
    z_owned_keyexpr_t, z_priority_t, z_session_t, zc_owned_payload_t, GuardedTransmute,
    UninitializedKeyExprError, LOG_INVALID_SESSION,
};

/// Options passed to the :c:func:`z_declare_publisher` function.
///
/// Members:
///     z_congestion_control_t congestion_control: The congestion control to apply when routing messages from this publisher.
///     z_priority_t priority: The priority of messages from this publisher.
#[repr(C)]
pub struct z_publisher_options_t {
    pub congestion_control: z_congestion_control_t,
    pub priority: z_priority_t,
}

/// Constructs the default value for :c:type:`z_publisher_options_t`.
#[no_mangle]
pub extern "C" fn z_publisher_options_default() -> z_publisher_options_t {
    z_publisher_options_t {
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
    }
}

/// An owned zenoh publisher.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[cfg(not(target_arch = "arm"))]
#[repr(C, align(8))]
pub struct z_owned_publisher_t([u64; 7]);

#[cfg(target_arch = "arm")]
#[repr(C, align(4))]
pub struct z_owned_publisher_t([u32; 8]);

impl_guarded_transmute!(noderefs Option<Publisher<'_>>, z_owned_publisher_t);

impl<'a> From<Option<Publisher<'a>>> for z_owned_publisher_t {
    fn from(val: Option<Publisher>) -> Self {
        val.transmute()
    }
}
impl Deref for z_owned_publisher_t {
    type Target = Option<Publisher<'static>>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl DerefMut for z_owned_publisher_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl z_owned_publisher_t {
    pub fn null() -> Self {
        None.into()
    }
}

/// Declares a publisher for the given key expression.
///
/// Data can be put and deleted with this publisher with the help of the
/// :c:func:`z_publisher_put` and :c:func:`z_publisher_delete` functions.
///
/// Parameters:
///     session: The zenoh session.
///     keyexpr: The key expression to publish.
///     options: additional options for the publisher.
///
/// Returns:
///    A :c:type:`z_owned_publisherr_t`.
///
///    To check if the publisher decalration succeeded and if the publisher is still valid,
///    you may use `z_publisher_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
///
///    Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.
///    To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.
///    After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.
///
/// Example:
///    Declaring a publisher passing `NULL` for the options:
///
///    .. code-block:: C
///
///       z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(expr), NULL);
///
///    is equivalent to initializing and passing the default publisher options:
///
///    .. code-block:: C
///
///       z_publisher_options_t opts = z_publisher_options_default();
///       z_owned_publisher_t sub = z_declare_publisher(z_loan(s), z_keyexpr(expr), &opts);
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_declare_publisher(
    session: z_session_t,
    keyexpr: z_keyexpr_t,
    options: Option<&z_publisher_options_t>,
) -> z_owned_publisher_t {
    match session.upgrade() {
        Some(s) => {
            let keyexpr = keyexpr.deref().as_ref().map(|s| s.clone().into_owned());
            if let Some(key_expr) = keyexpr {
                let mut p = s.declare_publisher(key_expr);
                if let Some(options) = options {
                    p = p
                        .congestion_control(options.congestion_control.into())
                        .priority(options.priority.into());
                }
                match p.res_sync() {
                    Err(e) => {
                        log::error!("{}", e);
                        None
                    }
                    Ok(publisher) => Some(publisher),
                }
            } else {
                log::error!("{}", UninitializedKeyExprError);
                None
            }
        }
        None => {
            log::debug!("{}", LOG_INVALID_SESSION);
            None
        }
    }
    .into()
}

/// Constructs a null safe-to-drop value of 'z_owned_publisher_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_null() -> z_owned_publisher_t {
    z_owned_publisher_t::null()
}

/// Returns ``true`` if `pub` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_publisher_check(pbl: &z_owned_publisher_t) -> bool {
    pbl.as_ref().is_some()
}

/// A loaned zenoh publisher.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct z_publisher_t(*const z_owned_publisher_t);

impl<'a> AsRef<Option<Publisher<'a>>> for z_owned_publisher_t {
    fn as_ref(&self) -> &'a Option<Publisher<'a>> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> AsMut<Option<Publisher<'a>>> for z_owned_publisher_t {
    fn as_mut(&mut self) -> &'a mut Option<Publisher<'a>> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> AsRef<Option<Publisher<'a>>> for z_publisher_t {
    fn as_ref(&self) -> &'a Option<Publisher<'a>> {
        unsafe { (*self.0).as_ref() }
    }
}

/// Returns a :c:type:`z_publisher_t` loaned from `p`.
#[no_mangle]
pub extern "C" fn z_publisher_loan(p: &z_owned_publisher_t) -> z_publisher_t {
    z_publisher_t(p)
}

/// Options passed to the :c:func:`z_publisher_put` function.
///
/// Members:
///     z_encoding_t encoding: The encoding of the payload.
///     z_attachment_t attachment: The attachment to attach to the publication.
#[repr(C)]
pub struct z_publisher_put_options_t {
    pub encoding: z_encoding_t,
    pub attachment: z_attachment_t,
}

/// Constructs the default value for :c:type:`z_publisher_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_put_options_default() -> z_publisher_put_options_t {
    z_publisher_put_options_t {
        encoding: z_encoding_default(),
        attachment: z_attachment_null(),
    }
}

/// Sends a `PUT` message onto the publisher's key expression, transfering the payload ownership.
///
/// This is avoids copies when transfering data that was either:
/// - `zc_sample_payload_rcinc`'d from a sample, when forwarding samples from a subscriber/query to a publisher
/// - constructed from a `zc_owned_shmbuf_t`
///
/// The payload's encoding can be sepcified through the options.
///
/// Parameters:
///     session: The zenoh session.
///     payload: The value to put.
///     options: The publisher put options.
/// Returns:
///     ``0`` in case of success, negative values in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_publisher_put(
    publisher: z_publisher_t,
    payload: Option<&mut zc_owned_payload_t>,
    options: Option<&z_publisher_put_options_t>,
) -> i8 {
    if let Some(p) = publisher.as_ref() {
        let Some(payload) = payload.and_then(|p| p.take()) else {
            log::debug!("Attempted to put without a payload");
            return i8::MIN;
        };
        let value: Value = payload.into();
        let put = match options {
            Some(options) => {
                let mut put = p.put(value.encoding(options.encoding.into()));
                if z_attachment_check(&options.attachment) {
                    let mut attachment_builder = AttachmentBuilder::new();
                    z_attachment_iterate(
                        options.attachment,
                        insert_in_attachment_builder,
                        &mut attachment_builder as *mut AttachmentBuilder as *mut c_void,
                    );
                    put = put.with_attachment(attachment_builder.build());
                };
                put
            }
            None => p.put(value),
        };
        if let Err(e) = put.res_sync() {
            log::error!("{}", e);
            e.errno().get()
        } else {
            0
        }
    } else {
        i8::MIN
    }
}

/// Represents the set of options that can be applied to the delete operation by a previously declared publisher,
/// whenever issued via :c:func:`z_publisher_delete`.
#[repr(C)]
pub struct z_publisher_delete_options_t {
    __dummy: u8,
}

/// Constructs the default values for the delete operation via a publisher entity.
///
/// Returns:
///   Returns the constructed :c:type:`z_publisher_delete_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_delete_options_default() -> z_publisher_delete_options_t {
    z_publisher_delete_options_t { __dummy: 0 }
}
/// Sends a `DELETE` message onto the publisher's key expression.
///
/// Returns:
///     ``0`` in case of success, ``1`` in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_delete(
    publisher: z_publisher_t,
    _options: *const z_publisher_delete_options_t,
) -> i8 {
    if let Some(p) = publisher.as_ref() {
        if let Err(e) = p.delete().res_sync() {
            log::error!("{}", e);
            e.errno().get()
        } else {
            0
        }
    } else {
        i8::MIN
    }
}

/// Returns the key expression of the publisher
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_keyexpr(publisher: z_publisher_t) -> z_owned_keyexpr_t {
    if let Some(p) = publisher.as_ref() {
        p.key_expr().clone().into()
    } else {
        z_keyexpr_t::null().into()
    }
}

/// An owned zenoh matching listener. Destroying the matching listener cancels the subscription.
///
/// Like most `z_owned_X_t` types, you may obtain an instance of `z_X_t` by loaning it using `z_X_loan(&val)`.  
/// The `z_loan(val)` macro, available if your compiler supports C11's `_Generic`, is equivalent to writing `z_X_loan(&val)`.  
///
/// Like all `z_owned_X_t`, an instance will be destroyed by any function which takes a mutable pointer to said instance, as this implies the instance's inners were moved.  
/// To make this fact more obvious when reading your code, consider using `z_move(val)` instead of `&val` as the argument.  
/// After a move, `val` will still exist, but will no longer be valid. The destructors are double-drop-safe, but other functions will still trust that your `val` is valid.  
///
/// To check if `val` is still valid, you may use `z_X_check(&val)` or `z_check(val)` if your compiler supports `_Generic`, which will return `true` if `val` is valid.
#[repr(C, align(8))]
pub struct zcu_owned_matching_listener_t([u64; 4]);

impl_guarded_transmute!(noderefs
    Option<MatchingListener<'_, ()>>,
    zcu_owned_matching_listener_t
);

impl From<Option<MatchingListener<'_, ()>>> for zcu_owned_matching_listener_t {
    fn from(val: Option<MatchingListener<'_, ()>>) -> Self {
        val.transmute()
    }
}

impl zcu_owned_matching_listener_t {
    pub fn null() -> Self {
        None.into()
    }
}

/// A struct that indicates if there exist Subscribers matching the Publisher's key expression.
///
/// Members:
///   bool matching: true if there exist Subscribers matching the Publisher's key expression.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct zcu_matching_status_t {
    pub matching: bool,
}

/// Register callback for notifying subscribers matching.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn zcu_publisher_matching_listener_callback(
    publisher: z_publisher_t,
    callback: &mut zcu_owned_closure_matching_status_t,
) -> zcu_owned_matching_listener_t {
    let mut closure = zcu_owned_closure_matching_status_t::empty();
    std::mem::swap(callback, &mut closure);
    {
        if let Some(p) = publisher.as_ref() {
            let listener = p
                .matching_listener()
                .callback_mut(move |matching_status| {
                    let status = zcu_matching_status_t {
                        matching: matching_status.matching_subscribers(),
                    };
                    zcu_closure_matching_status_call(&closure, &status);
                })
                .res()
                .unwrap();
            Some(listener)
        } else {
            None
        }
    }
    .into()
}

/// Undeclares the given :c:type:`z_owned_publisher_t`, droping it and invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_undeclare_publisher(publisher: &mut z_owned_publisher_t) -> i8 {
    if let Some(p) = publisher.take() {
        if let Err(e) = p.undeclare().res_sync() {
            log::error!("{}", e);
            return e.errno().get();
        }
    }
    0
}
