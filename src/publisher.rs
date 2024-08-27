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

use std::mem::MaybeUninit;

#[cfg(feature = "unstable")]
use zenoh::pubsub::MatchingListener;
use zenoh::{
    prelude::*,
    pubsub::Publisher,
    qos::{CongestionControl, Priority},
};

#[cfg(feature = "unstable")]
use crate::z_moved_source_info_t;
#[cfg(feature = "unstable")]
use crate::zc_moved_closure_matching_status_t;
use crate::{
    result::{self},
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_congestion_control_t, z_loaned_keyexpr_t, z_loaned_session_t, z_moved_bytes_t,
    z_moved_encoding_t, z_priority_t, z_timestamp_t,
};
#[cfg(feature = "unstable")]
use crate::{
    transmute::IntoCType, z_entity_global_id_t, zc_closure_matching_status_call,
    zc_closure_matching_status_loan, zc_locality_default, zc_locality_t,
};

/// Options passed to the `z_declare_publisher()` function.
#[repr(C)]
pub struct z_publisher_options_t {
    /// Default encoding for messages put by this publisher.
    pub encoding: Option<&'static mut z_moved_encoding_t>,
    /// The congestion control to apply when routing messages from this publisher.
    pub congestion_control: z_congestion_control_t,
    /// The priority of messages from this publisher.
    pub priority: z_priority_t,
    /// If true, Zenoh will not wait to batch this message with others to reduce the bandwith.
    pub is_express: bool,
    #[cfg(feature = "unstable")]
    /// The allowed destination for this publisher.
    pub allowed_destination: zc_locality_t,
}

/// Constructs the default value for `z_publisher_options_t`.
#[no_mangle]
pub extern "C" fn z_publisher_options_default(this_: &mut MaybeUninit<z_publisher_options_t>) {
    this_.write(z_publisher_options_t {
        encoding: None,
        congestion_control: CongestionControl::default().into(),
        priority: Priority::default().into(),
        is_express: false,
        #[cfg(feature = "unstable")]
        allowed_destination: zc_locality_default(),
    });
}

pub use crate::opaque_types::{z_loaned_publisher_t, z_moved_publisher_t, z_owned_publisher_t};
decl_c_type!(
    owned(z_owned_publisher_t, option Publisher<'static>),
    loaned(z_loaned_publisher_t),
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
    options: Option<&mut z_publisher_options_t>,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref().clone().into_owned();
    let mut p = session.declare_publisher(key_expr);
    if let Some(options) = options {
        p = p
            .congestion_control(options.congestion_control.into())
            .priority(options.priority.into())
            .express(options.is_express);
        #[cfg(feature = "unstable")]
        {
            p = p.allowed_destination(options.allowed_destination.into());
        }
        if let Some(encoding) = options.encoding.take() {
            p = p.encoding(encoding.take_rust_type());
        }
    }
    match p.wait() {
        Err(e) => {
            tracing::error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
        Ok(publisher) => {
            this.write(Some(publisher));
            result::Z_OK
        }
    }
}

/// Constructs a publisher in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_internal_publisher_null(this_: &mut MaybeUninit<z_owned_publisher_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if publisher is valid, ``false`` otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_internal_publisher_check(this_: &z_owned_publisher_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Borrows publisher.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_publisher_loan(this_: &z_owned_publisher_t) -> &z_loaned_publisher_t {
    this_
        .as_rust_type_ref()
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
    pub encoding: Option<&'static mut z_moved_encoding_t>,
    /// The timestamp of the publication.
    pub timestamp: Option<&'static z_timestamp_t>,
    /// The source info for the publication.
    #[cfg(feature = "unstable")]
    pub source_info: Option<&'static mut z_moved_source_info_t>,
    /// The attachment to attach to the publication.
    pub attachment: Option<&'static mut z_moved_bytes_t>,
}

/// Constructs the default value for `z_publisher_put_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_put_options_default(
    this: &mut MaybeUninit<z_publisher_put_options_t>,
) {
    this.write(z_publisher_put_options_t {
        encoding: None,
        timestamp: None,
        #[cfg(feature = "unstable")]
        source_info: None,
        attachment: None,
    });
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
    payload: &mut z_moved_bytes_t,
    options: Option<&mut z_publisher_put_options_t>,
) -> result::z_result_t {
    let publisher = this.as_rust_type_ref();
    let payload = payload.take_rust_type();
    let mut put = publisher.put(payload);
    if let Some(options) = options {
        if let Some(encoding) = options.encoding.take() {
            put = put.encoding(encoding.take_rust_type());
        };
        #[cfg(feature = "unstable")]
        if let Some(source_info) = options.source_info.take() {
            put = put.source_info(source_info.take_rust_type());
        };
        if let Some(attachment) = options.attachment.take() {
            put = put.attachment(attachment.take_rust_type());
        }
        if let Some(timestamp) = options.timestamp {
            put = put.timestamp(Some(*timestamp.as_rust_type_ref()));
        }
    }

    if let Err(e) = put.wait() {
        tracing::error!("{}", e);
        result::Z_EGENERIC
    } else {
        result::Z_OK
    }
}

/// Represents the set of options that can be applied to the delete operation by a previously declared publisher,
/// whenever issued via `z_publisher_delete()`.
#[repr(C)]
pub struct z_publisher_delete_options_t {
    /// The timestamp of this message.
    pub timestamp: Option<&'static z_timestamp_t>,
}

/// Constructs the default values for the delete operation via a publisher entity.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_delete_options_default(
    this: &mut MaybeUninit<z_publisher_delete_options_t>,
) {
    this.write(z_publisher_delete_options_t { timestamp: None });
}
/// Sends a `DELETE` message onto the publisher's key expression.
///
/// @return 0 in case of success, negative error code in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_delete(
    publisher: &z_loaned_publisher_t,
    options: Option<&z_publisher_delete_options_t>,
) -> result::z_result_t {
    let publisher = publisher.as_rust_type_ref();
    let mut del = publisher.delete();
    if let Some(options) = options {
        if let Some(timestamp) = options.timestamp {
            del = del.timestamp(Some(*timestamp.as_rust_type_ref()));
        }
    }
    if let Err(e) = del.wait() {
        tracing::error!("{}", e);
        result::Z_EGENERIC
    } else {
        result::Z_OK
    }
}
#[cfg(feature = "unstable")]
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

#[cfg(feature = "unstable")]
pub use crate::opaque_types::{zc_moved_matching_listener_t, zc_owned_matching_listener_t};
#[cfg(feature = "unstable")]
decl_c_type!(
    owned(zc_owned_matching_listener_t, option MatchingListener<'static, ()>),
);

/// Constructs an empty matching listener
#[no_mangle]
#[cfg(feature = "unstable")]
pub extern "C" fn zc_internal_matching_listener_null(
    this_: &mut MaybeUninit<zc_owned_matching_listener_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Checks the matching listener is for the gravestone state
#[no_mangle]
#[cfg(feature = "unstable")]
pub extern "C" fn zc_internal_matching_listener_check(
    this_: &zc_owned_matching_listener_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

#[cfg(feature = "unstable")]
/// A struct that indicates if there exist Subscribers matching the Publisher's key expression.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct zc_matching_status_t {
    /// True if there exist Subscribers matching the Publisher's key expression, false otherwise.
    pub matching: bool,
}

#[cfg(feature = "unstable")]
/// Constructs matching listener, registering a callback for notifying subscribers matching with a given publisher.
///
/// @param this_: An unitilized memory location where matching listener will be constructed. The matching listener will be automatically dropped when publisher is dropped.
/// @param publisher: A publisher to associate with matching listener.
/// @param callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber, disconnects or when the first subscriber connects).
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn zc_publisher_matching_listener_declare(
    this: &mut MaybeUninit<zc_owned_matching_listener_t>,
    publisher: &'static z_loaned_publisher_t,
    callback: &mut zc_moved_closure_matching_status_t,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let publisher = publisher.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let listener = publisher
        .matching_listener()
        .callback_mut(move |matching_status| {
            let status = zc_matching_status_t {
                matching: matching_status.matching_subscribers(),
            };
            zc_closure_matching_status_call(zc_closure_matching_status_loan(&callback), &status);
        })
        .wait();
    match listener {
        Ok(listener) => {
            this.write(Some(listener));
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

#[cfg(feature = "unstable")]
/// Undeclares the given matching listener, droping and invalidating it.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn zc_publisher_matching_listener_undeclare(
    this: &mut zc_moved_matching_listener_t,
) -> result::z_result_t {
    if let Some(p) = this.take_rust_type() {
        if let Err(e) = p.undeclare().wait() {
            tracing::error!("{}", e);
            return result::Z_EGENERIC;
        }
    }
    result::Z_OK
}

#[cfg(feature = "unstable")]
/// Undeclares the given matching listener, droping and invalidating it.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn zc_publisher_matching_listener_drop(
    this: &mut zc_moved_matching_listener_t,
) -> result::z_result_t {
    zc_publisher_matching_listener_undeclare(this)
}

#[cfg(feature = "unstable")]
/// Gets publisher matching status - i.e. if there are any subscribers matching its key expression.
///
/// @return 0 in case of success, negative error code otherwise (in this case matching_status is not updated).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn zc_publisher_get_matching_status(
    this: &'static z_loaned_publisher_t,
    matching_status: &mut MaybeUninit<zc_matching_status_t>,
) -> result::z_result_t {
    match this.as_rust_type_ref().matching_status().wait() {
        Ok(s) => {
            matching_status.write(zc_matching_status_t {
                matching: s.matching_subscribers(),
            });
            result::Z_OK
        }
        Err(e) => {
            tracing::error!("{}", e);
            result::Z_ENETWORK
        }
    }
}

/// Undeclares the given publisher, droping and invalidating it.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_undeclare_publisher(this_: &mut z_moved_publisher_t) -> result::z_result_t {
    if let Some(p) = this_.take_rust_type() {
        if let Err(e) = p.undeclare().wait() {
            tracing::error!("{}", e);
            return result::Z_ENETWORK;
        }
    }
    result::Z_OK
}

/// Frees memory and resets publisher to its gravestone state. Also attempts undeclare publisher.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_drop(this_: &mut z_moved_publisher_t) {
    z_undeclare_publisher(this_);
}
