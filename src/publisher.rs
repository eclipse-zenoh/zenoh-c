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
use crate::transmute::TransmuteFromHandle;
use crate::transmute::TransmuteIntoHandle;
use crate::transmute::TransmuteRef;
use crate::transmute::TransmuteUninitPtr;
use crate::z_owned_encoding_t;
use crate::z_owned_source_info_t;
use crate::z_timestamp_t;
use crate::zcu_closure_matching_status_call;
use crate::zcu_closure_matching_status_loan;
use crate::zcu_locality_default;
use crate::zcu_locality_t;
use crate::zcu_owned_closure_matching_status_t;
use std::mem::MaybeUninit;
use std::ptr;
use zenoh::core::Wait;
use zenoh::handlers::DefaultHandler;
use zenoh::prelude::SessionDeclarations;
use zenoh::publisher::CongestionControl;
use zenoh::sample::QoSBuilderTrait;
use zenoh::sample::SampleBuilderTrait;
use zenoh::sample::TimestampBuilderTrait;
use zenoh::sample::ValueBuilderTrait;
use zenoh::{publisher::MatchingListener, publisher::Priority, publisher::Publisher};

use crate::{
    z_congestion_control_t, z_loaned_keyexpr_t, z_loaned_session_t, z_owned_bytes_t, z_priority_t,
};

/// Options passed to the `z_declare_publisher()` function.
#[repr(C)]
pub struct z_publisher_options_t {
    /// The congestion control to apply when routing messages from this publisher.
    pub congestion_control: z_congestion_control_t,
    /// The priority of messages from this publisher.
    pub priority: z_priority_t,
    /// If true, Zenoh will not wait to batch this message with others to reduce the bandwith
    pub is_express: bool,
    /// The allowed destination for thsi publisher.
    pub allowed_destination: zcu_locality_t,
}

/// Constructs the default value for `z_publisher_options_t`.
#[no_mangle]
pub extern "C" fn z_publisher_options_default(this: &mut z_publisher_options_t) {
    *this = z_publisher_options_t {
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
        is_express: false,
        allowed_destination: zcu_locality_default(),
    };
}

pub use crate::opaque_types::z_owned_publisher_t;
decl_transmute_owned!(Option<Publisher<'static>>, z_owned_publisher_t);
pub use crate::opaque_types::z_loaned_publisher_t;
decl_transmute_handle!(Publisher<'static>, z_loaned_publisher_t);

validate_equivalence!(z_owned_publisher_t, z_loaned_publisher_t);

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
    this: *mut MaybeUninit<z_owned_publisher_t>,
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    options: Option<&z_publisher_options_t>,
) -> errors::z_error_t {
    let this = this.transmute_uninit_ptr();
    let session = session.transmute_ref();
    let key_expr = key_expr.transmute_ref().clone().into_owned();
    let mut p = session.declare_publisher(key_expr);
    if let Some(options) = options {
        p = p
            .congestion_control(options.congestion_control.into())
            .priority(options.priority.into())
            .express(options.is_express)
            .allowed_destination(options.allowed_destination.into());
    }
    match p.wait() {
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

/// Constructs a publisher in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_null(this: *mut MaybeUninit<z_owned_publisher_t>) {
    let this = this.transmute_uninit_ptr();
    Inplace::empty(this);
}

/// Returns ``true`` if publisher is valid, ``false`` otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_publisher_check(this: &z_owned_publisher_t) -> bool {
    this.transmute_ref().is_some()
}

/// Borrows publisher.
#[no_mangle]
pub extern "C" fn z_publisher_loan(this: &z_owned_publisher_t) -> &z_loaned_publisher_t {
    let this = this.transmute_ref();
    let this = unwrap_ref_unchecked(this);
    this.transmute_handle()
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
    let publisher = this.transmute_ref();
    let payload = payload.transmute_mut().extract();

    let mut put = publisher.put(payload);
    if let Some(options) = options {
        if !options.encoding.is_null() {
            let encoding = unsafe { options.encoding.as_mut() }
                .unwrap()
                .transmute_mut()
                .extract();
            put = put.encoding(encoding);
        };
        if !options.source_info.is_null() {
            let source_info = unsafe { options.source_info.as_mut() }
                .unwrap()
                .transmute_mut()
                .extract();
            put = put.source_info(source_info);
        }
        if !options.attachment.is_null() {
            let attachment = unsafe { options.attachment.as_mut() }
                .unwrap()
                .transmute_mut()
                .extract();
            put = put.attachment(attachment);
        }
        if !options.timestamp.is_null() {
            let timestamp = *unsafe { options.timestamp.as_mut() }
                .unwrap()
                .transmute_ref();
            put = put.timestamp(Some(timestamp));
        }
    }

    if let Err(e) = put.wait() {
        log::error!("{}", e);
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
    let publisher = publisher.transmute_ref();
    let mut del = publisher.delete();
    if let Some(options) = options {
        if !options.timestamp.is_null() {
            let timestamp = *unsafe { options.timestamp.as_mut() }
                .unwrap()
                .transmute_ref();
            del = del.timestamp(Some(timestamp));
        }
    }
    if let Err(e) = del.wait() {
        log::error!("{}", e);
        errors::Z_EGENERIC
    } else {
        errors::Z_OK
    }
}

/// Returns the key expression of the publisher.
#[no_mangle]
pub extern "C" fn z_publisher_keyexpr(publisher: &z_loaned_publisher_t) -> &z_loaned_keyexpr_t {
    let publisher = publisher.transmute_ref();
    publisher.key_expr().transmute_handle()
}

/// Sets congestion control for the publisher
#[no_mangle]
pub extern "C" fn z_publisher_set_congestion_control(
    publisher: &mut z_loaned_publisher_t,
    congestion_control: z_congestion_control_t,
) {
    let publisher = publisher.transmute_mut();
    publisher.set_congestion_control(congestion_control.into())
}

/// Sets priority for the publisher
#[no_mangle]
pub extern "C" fn z_publisher_set_priority(
    publisher: &mut z_loaned_publisher_t,
    priority: z_priority_t,
) {
    let publisher = publisher.transmute_mut();
    publisher.set_priority(priority.into())
}

/// Sets allowed destination for the publisher
#[no_mangle]
pub extern "C" fn z_publisher_set_allowed_destination(
    publisher: &mut z_loaned_publisher_t,
    destination: zcu_locality_t,
) {
    let publisher = publisher.transmute_mut();
    publisher.set_allowed_destination(destination.into())
}

pub use crate::opaque_types::zcu_owned_matching_listener_t;
decl_transmute_owned!(
    Option<MatchingListener<'static, DefaultHandler>>,
    zcu_owned_matching_listener_t
);

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
    this: *mut MaybeUninit<zcu_owned_matching_listener_t>,
    publisher: &z_loaned_publisher_t,
    callback: &mut zcu_owned_closure_matching_status_t,
) -> errors::z_error_t {
    let this = this.transmute_uninit_ptr();
    let mut closure = zcu_owned_closure_matching_status_t::empty();
    std::mem::swap(callback, &mut closure);
    let publisher = publisher.transmute_ref();
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
        Ok(_) => {
            Inplace::empty(this);
            errors::Z_OK
        }
        Err(e) => {
            log::error!("{}", e);
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
    if let Some(p) = this.transmute_mut().extract().take() {
        if let Err(e) = p.undeclare().wait() {
            log::error!("{}", e);
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
    if let Some(p) = this.transmute_mut().extract().take() {
        if let Err(e) = p.undeclare().wait() {
            log::error!("{}", e);
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
