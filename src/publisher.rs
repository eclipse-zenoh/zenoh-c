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
    mem::MaybeUninit,
    ptr::{self, null_mut},
};

use zenoh::{
    prelude::*,
    pubsub::{MatchingListener, Publisher},
    qos::{CongestionControl, Priority},
};

use crate::{
    errors,
    transmute::{IntoCType, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit},
    z_congestion_control_t, z_entity_global_id_t, z_loaned_keyexpr_t, z_loaned_session_t,
    z_owned_bytes_t, z_owned_encoding_t, z_owned_source_info_t, z_priority_t, z_timestamp_t,
    zcu_closure_matching_status_call, zcu_closure_matching_status_loan, zcu_locality_default,
    zcu_locality_t, zcu_owned_closure_matching_status_t,
};

/// Options passed to the `z_declare_publisher()` function.
#[repr(C)]
pub struct z_publisher_options_t {
    /// Default encoding for messages put by this publisher.
    pub encoding: *mut z_owned_encoding_t,
    /// The congestion control to apply when routing messages from this publisher.
    pub congestion_control: z_congestion_control_t,
    /// The priority of messages from this publisher.
    pub priority: z_priority_t,
    /// If true, Zenoh will not wait to batch this message with others to reduce the bandwith.
    pub is_express: bool,
    /// The allowed destination for this publisher.
    pub allowed_destination: zcu_locality_t,
}

/// Constructs the default value for `z_publisher_options_t`.
#[no_mangle]
pub extern "C" fn z_publisher_options_default(this: &mut z_publisher_options_t) {
    *this = z_publisher_options_t {
        encoding: null_mut(),
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
        is_express: false,
        allowed_destination: zcu_locality_default(),
    };
}

pub use crate::opaque_types::{z_loaned_publisher_t, z_owned_publisher_t};

decl_c_type!(
    owned(z_owned_publisher_t, Option<Publisher<'static>>),
    loaned(z_loaned_publisher_t, Publisher<'static>)
);

/// Constructs and declares a publisher for the given key expression.
///
/// Data can be put and deleted with this publisher with the help of the
/// `z_publisher_put()` and `z_publisher_delete()` functions.
///
/// @param this_: An unitilized location in memory where publisher will be constructed.
/// @param session: The Zenoh session.
/// @param key_expr: The key expression to publish.
/// @param options: Additional options for the publisher.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_declare_publisher(
    this: &mut MaybeUninit<z_owned_publisher_t>,
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    options: Option<&z_publisher_options_t>,
) -> errors::z_error_t {
    let this = this.as_rust_type_mut_uninit();
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref().clone().into_owned();
    let mut p = session.declare_publisher(key_expr);
    if let Some(options) = options {
        p = p
            .congestion_control(options.congestion_control.into())
            .priority(options.priority.into())
            .express(options.is_express)
            .allowed_destination(options.allowed_destination.into());
        if let Some(encoding) = unsafe { options.encoding.as_mut() } {
            let encoding = std::mem::take(encoding.as_rust_type_mut());
            p = p.encoding(encoding);
        }
    }
    match p.wait() {
        Err(e) => {
            tracing::error!("{}", e);
            this.write(None);
            errors::Z_EGENERIC
        }
        Ok(publisher) => {
            this.write(Some(publisher));
            errors::Z_OK
        }
    }
}

/// Constructs a publisher in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_null(this: &mut MaybeUninit<z_owned_publisher_t>) {
    this.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if publisher is valid, ``false`` otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_publisher_check(this: &z_owned_publisher_t) -> bool {
    this.as_rust_type_ref().is_some()
}

/// Borrows publisher.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_publisher_loan(this: &z_owned_publisher_t) -> &z_loaned_publisher_t {
    this.as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Mutably borrows publisher.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_publisher_loan_mut(
    this: &mut z_owned_publisher_t,
) -> &mut z_loaned_publisher_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// Options passed to the `z_publisher_put()` function.
#[repr(C)]
pub struct z_publisher_put_options_t {
    ///  The encoding of the data to publish.
    pub encoding: *mut z_owned_encoding_t,
    /// The timestamp of the publication.
    pub timestamp: *mut z_timestamp_t,
    /// The source info for the publication.
    pub source_info: *mut z_owned_source_info_t,
    /// The attachment to attach to the publication.
    pub attachment: *mut z_owned_bytes_t,
}

/// Constructs the default value for `z_publisher_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_put_options_default(this: &mut z_publisher_put_options_t) {
    *this = z_publisher_put_options_t {
        encoding: ptr::null_mut(),
        timestamp: ptr::null_mut(),
        source_info: ptr::null_mut(),
        attachment: ptr::null_mut(),
    }
}

/// Sends a `PUT` message onto the publisher's key expression, transfering the payload ownership.
///
///
/// The payload and all owned options fields are consumed upon function return.
///
/// @param this_: The publisher.
/// @param session: The Zenoh session.
/// @param payload: The dat to publish. WIll be consumed.
/// @param options: The publisher put options. All owned fields will be consumed.
///
/// @return 0 in case of success, negative error values in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_publisher_put(
    this: &z_loaned_publisher_t,
    payload: &mut z_owned_bytes_t,
    options: Option<&mut z_publisher_put_options_t>,
) -> errors::z_error_t {
    let publisher = this.as_rust_type_ref();
    let payload = std::mem::take(payload.as_rust_type_mut());

    let mut put = publisher.put(payload);
    if let Some(options) = options {
        if let Some(encoding) = unsafe { options.encoding.as_mut() } {
            let encoding = std::mem::take(encoding.as_rust_type_mut());
            put = put.encoding(encoding);
        };
        if let Some(source_info) = unsafe { options.source_info.as_mut() } {
            let source_info = std::mem::take(source_info.as_rust_type_mut());
            put = put.source_info(source_info);
        };
        if let Some(attachment) = unsafe { options.attachment.as_mut() } {
            let attachment = std::mem::take(attachment.as_rust_type_mut());
            put = put.attachment(attachment);
        }
        if !options.timestamp.is_null() {
            let timestamp = *unsafe { options.timestamp.as_mut() }
                .unwrap()
                .as_rust_type_ref();
            put = put.timestamp(Some(timestamp));
        }
    }

    if let Err(e) = put.wait() {
        tracing::error!("{}", e);
        errors::Z_EGENERIC
    } else {
        errors::Z_OK
    }
}

/// Represents the set of options that can be applied to the delete operation by a previously declared publisher,
/// whenever issued via `z_publisher_delete()`.
#[repr(C)]
pub struct z_publisher_delete_options_t {
    /// The timestamp of this message.
    pub timestamp: *mut z_timestamp_t,
}

/// Constructs the default values for the delete operation via a publisher entity.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_delete_options_default(this: &mut z_publisher_delete_options_t) {
    *this = z_publisher_delete_options_t {
        timestamp: ptr::null_mut(),
    }
}
/// Sends a `DELETE` message onto the publisher's key expression.
///
/// @return 0 in case of success, negative error code in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_delete(
    publisher: &z_loaned_publisher_t,
    options: Option<&z_publisher_delete_options_t>,
) -> errors::z_error_t {
    let publisher = publisher.as_rust_type_ref();
    let mut del = publisher.delete();
    if let Some(options) = options {
        if !options.timestamp.is_null() {
            let timestamp = *unsafe { options.timestamp.as_mut() }
                .unwrap()
                .as_rust_type_ref();
            del = del.timestamp(Some(timestamp));
        }
    }
    if let Err(e) = del.wait() {
        tracing::error!("{}", e);
        errors::Z_EGENERIC
    } else {
        errors::Z_OK
    }
}

/// Returns the ID of the publisher.
#[no_mangle]
pub extern "C" fn z_publisher_id(publisher: &z_loaned_publisher_t) -> z_entity_global_id_t {
    publisher.as_rust_type_ref().id().into_c_type()
}

/// Returns the key expression of the publisher.
#[no_mangle]
pub extern "C" fn z_publisher_keyexpr(publisher: &z_loaned_publisher_t) -> &z_loaned_keyexpr_t {
    publisher
        .as_rust_type_ref()
        .key_expr()
        .as_loaned_c_type_ref()
}

pub use crate::opaque_types::zcu_owned_matching_listener_t;
decl_c_type!(owned(zcu_owned_matching_listener_t, Option<MatchingListener<'static, ()>>));

/// A struct that indicates if there exist Subscribers matching the Publisher's key expression.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct zcu_matching_status_t {
    /// True if there exist Subscribers matching the Publisher's key expression, false otherwise.
    pub matching: bool,
}

/// Constructs matching listener, registering a callback for notifying subscribers matching with a given publisher.
///
/// @param this_: An unitilized memory location where matching listener will be constructed. The matching listener will be automatically dropped when publisher is dropped.
/// @publisher: A publisher to associate with matching listener.
/// @callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber, disconnects or when the first subscriber connects).
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn zcu_publisher_matching_listener_callback(
    this: &mut MaybeUninit<zcu_owned_matching_listener_t>,
    publisher: &'static z_loaned_publisher_t,
    callback: &mut zcu_owned_closure_matching_status_t,
) -> errors::z_error_t {
    let this = this.as_rust_type_mut_uninit();
    let mut closure = zcu_owned_closure_matching_status_t::empty();
    std::mem::swap(callback, &mut closure);
    let publisher = publisher.as_rust_type_ref();
    let listener = publisher
        .matching_listener()
        .callback_mut(move |matching_status| {
            let status = zcu_matching_status_t {
                matching: matching_status.matching_subscribers(),
            };
            zcu_closure_matching_status_call(zcu_closure_matching_status_loan(&closure), &status);
        })
        .wait();
    match listener {
        Ok(listener) => {
            this.write(Some(listener));
            errors::Z_OK
        }
        Err(e) => {
            tracing::error!("{}", e);
            errors::Z_EGENERIC
        }
    }
}

/// Undeclares the given matching listener, droping and invalidating it.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn zcu_publisher_matching_listener_undeclare(
    this: &mut zcu_owned_matching_listener_t,
) -> errors::z_error_t {
    if let Some(p) = this.as_rust_type_mut().take() {
        if let Err(e) = p.undeclare().wait() {
            tracing::error!("{}", e);
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}

/// Undeclares the given publisher, droping and invalidating it.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_undeclare_publisher(this: &mut z_owned_publisher_t) -> errors::z_error_t {
    if let Some(p) = this.as_rust_type_mut().take() {
        if let Err(e) = p.undeclare().wait() {
            tracing::error!("{}", e);
            return errors::Z_EGENERIC;
        }
    }
    errors::Z_OK
}

/// Frees memory and resets publisher to its gravestone state. Also attempts undeclare publisher.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_drop(this: &mut z_owned_publisher_t) {
    z_undeclare_publisher(this);
}
