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

use crate::errors;
use crate::transmute::unwrap_ref_unchecked;
use crate::transmute::Inplace;
use crate::transmute::TransmuteCopy;
use crate::transmute::TransmuteFromHandle;
use crate::transmute::TransmuteIntoHandle;
use crate::transmute::TransmuteRef;
use crate::transmute::TransmuteUninitPtr;
use crate::z_owned_encoding_t;
use crate::zcu_closure_matching_status_call;
use crate::zcu_owned_closure_matching_status_t;
use std::mem::MaybeUninit;
use std::ptr;
use zenoh::handlers::DefaultHandler;
use zenoh::prelude::SessionDeclarations;
use zenoh::publication::CongestionControl;
use zenoh::sample::QoSBuilderTrait;
use zenoh::sample::SampleBuilderTrait;
use zenoh::sample::ValueBuilderTrait;
use zenoh::{
    prelude::Priority,
    publication::MatchingListener,
    publication::Publisher,
};

use zenoh::prelude::SyncResolve;

use crate::{
    z_congestion_control_t, z_keyexpr_t,
    z_priority_t, z_session_t, z_owned_bytes_t
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

pub use crate::opaque_types::z_owned_publisher_t;
decl_transmute_owned!(Option<Publisher<'static>>, z_owned_publisher_t);
pub use crate::opaque_types::z_publisher_t;
decl_transmute_copy!(&'static Publisher<'static>, z_publisher_t);

/// Declares a publisher for the given key expression.
///
/// Data can be put and deleted with this publisher with the help of the
/// :c:func:`z_publisher_put` and :c:func:`z_publisher_delete` functions.
///
/// Parameters:
///     session: The zenoh session.
///     key_expr: The key expression to publish.
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
    key_expr: z_keyexpr_t,
    options: Option<&z_publisher_options_t>,
    this: *mut MaybeUninit<z_owned_publisher_t>,
) -> errors::ZCError {
    let this = this.transmute_uninit_ptr();
    let session = session.transmute_copy();
    let key_expr = key_expr.transmute_ref().clone().into_owned();
    let mut p = session.declare_publisher(key_expr);
    if let Some(options) = options {
        p = p
            .congestion_control(options.congestion_control.into())
            .priority(options.priority.into());
    }
    match p.res_sync() {
        Err(e) => {
            log::error!("{}", e);
            Inplace::empty(this);
            errors::Z_EGENERIC
        }
        Ok(publisher) => {
            Inplace::init(this, Some(publisher));
            errors::Z_OK
        }
    }
}

/// Constructs a null safe-to-drop value of 'z_owned_publisher_t' type
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_null(this: *mut MaybeUninit<z_owned_publisher_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Returns ``true`` if `pub` is valid.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_publisher_check(pbl: &'static z_owned_publisher_t) -> bool {
    pbl.transmute_ref().is_some()
}

/// Returns a :c:type:`z_publisher_t` loaned from `p`.
#[no_mangle]
pub extern "C" fn z_publisher_loan(p: &'static z_owned_publisher_t) -> z_publisher_t {
    let p = p.transmute_ref();
    let p = unwrap_ref_unchecked(p);
    p.transmute_copy()
}

/// Options passed to the :c:func:`z_publisher_put` function.
///
/// Members:
///     z_owned_encoding_t encoding: The encoding of the payload.
///    z_owned_bytes_t attachment: The attachment to attach to the publication.
#[repr(C)]
pub struct z_publisher_put_options_t {
    pub encoding: *mut z_owned_encoding_t,
    pub attachment: *mut z_owned_bytes_t,
}

/// Constructs the default value for :c:type:`z_publisher_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_put_options_default() -> z_publisher_put_options_t {
    z_publisher_put_options_t {
        encoding: ptr::null_mut(),
        attachment: ptr::null_mut(),
    }
}

/// Sends a `PUT` message onto the publisher's key expression, transfering the payload ownership.
///
/// This is avoids copies when transfering data that was either:
/// - `zc_sample_payload_rcinc`'d from a sample, when forwarding samples from a subscriber/query to a publisher
/// - constructed from a `zc_owned_shmbuf_t`
///
/// The payload and all owned options fields are consumed upon function return.
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
    payload: &mut z_owned_bytes_t,
    options: z_publisher_put_options_t,
) -> errors::ZCError {
    let publisher = publisher.transmute_copy();
    let payload = match payload.transmute_mut().extract() {
        Some(p) => p,
        None => {
            log::debug!("Attempted to put with a null payload");
            return errors::Z_EINVAL;
        }
    };
    
    let mut put = publisher.put(payload);

    if !options.encoding.is_null() {
        let encoding = unsafe{ *options.encoding }.transmute_mut().extract();
        put = put.encoding(encoding);
    };
    if !options.attachment.is_null() {
        let attachment = unsafe { *options.attachment }.transmute_mut().extract();
        put = put.attachment(attachment);
    }

    if let Err(e) = put.res_sync() {
        log::error!("{}", e);
        errors::Z_EGENERIC
    } else {
        errors::Z_OK
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
    _options: z_publisher_delete_options_t,
) -> errors::ZCError {
    let publisher = publisher.transmute_copy();
   
    if let Err(e) =  publisher.delete().res_sync() {
        log::error!("{}", e);
        errors::Z_EGENERIC
    } else {
        errors::Z_OK
    }
}

/// Returns the key expression of the publisher
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_keyexpr(publisher: z_publisher_t) -> z_keyexpr_t {
    let publisher = publisher.transmute_copy();
    publisher.key_expr().transmute_handle()
}

pub use crate::opaque_types::zcu_owned_matching_listener_t;
decl_transmute_owned!(Option<MatchingListener<'static, DefaultHandler>>, zcu_owned_matching_listener_t);

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
    this: *mut MaybeUninit<zcu_owned_matching_listener_t>
) -> errors::ZCError {
    let this = this.transmute_uninit_ptr();
    let mut closure = zcu_owned_closure_matching_status_t::empty();
    std::mem::swap(callback, &mut closure);
    let publisher = publisher.transmute_copy();
    let listener = publisher
        .matching_listener()
        .callback_mut(move |matching_status| {
            let status = zcu_matching_status_t {
                matching: matching_status.matching_subscribers(),
            };
            zcu_closure_matching_status_call(&closure, &status);
        })
        .res();
    match listener {
        Ok(_) => {
            Inplace::empty(this);
            errors::Z_OK
        },
        Err(e) => {
            log::error!("{}", e);
            errors::Z_EGENERIC
        } 
    }
}

/// Undeclares the given :c:type:`z_owned_publisher_t`, droping it and invalidating it for double-drop safety.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_undeclare_publisher(publisher: &mut z_owned_publisher_t) -> errors::ZCError {
    if let Some(p) = publisher.transmute_mut().extract().take() {
        if let Err(e) = p.undeclare().res_sync() {
            log::error!("{}", e);
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}
