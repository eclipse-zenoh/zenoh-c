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

use std::{mem::MaybeUninit, ops::Deref};

use zenoh::{
    handlers::Callback,
    internal::traits::{EncodingBuilderTrait, SampleBuilderTrait, TimestampBuilderTrait},
    matching::MatchingStatus,
    pubsub::{Publisher, PublisherBuilder},
    qos::{CongestionControl, Priority},
    session::SessionClosedError,
    Wait,
};

use crate::{
    result::{self},
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_matching_status_call, z_closure_matching_status_loan, z_congestion_control_t,
    z_loaned_keyexpr_t, z_loaned_session_t, z_matching_status_t, z_moved_bytes_t,
    z_moved_closure_matching_status_t, z_moved_encoding_t, z_owned_matching_listener_t,
    z_priority_t, z_timestamp_t, zc_locality_default, zc_locality_t, CMatchingListener, SgNotifier,
    SyncGroup,
};
#[cfg(feature = "unstable")]
use crate::{
    transmute::IntoCType, z_entity_global_id_t, z_moved_source_info_t, z_reliability_default,
    z_reliability_t,
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
    /// If set to ``true``, this message will not be batched. This usually has a positive impact on latency but negative impact on throughput.
    pub is_express: bool,
    #[cfg(feature = "unstable")]
    /// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
    ///
    /// The publisher reliability.
    pub reliability: z_reliability_t,
    /// The allowed destination for this publisher.
    pub allowed_destination: zc_locality_t,
}

impl Default for z_publisher_options_t {
    fn default() -> Self {
        Self {
            encoding: None,
            congestion_control: CongestionControl::DEFAULT_PUSH.into(),
            priority: Priority::default().into(),
            is_express: false,
            #[cfg(feature = "unstable")]
            reliability: z_reliability_default(),
            allowed_destination: zc_locality_default(),
        }
    }
}

/// Constructs the default value for `z_publisher_options_t`.
#[no_mangle]
pub extern "C" fn z_publisher_options_default(this_: &mut MaybeUninit<z_publisher_options_t>) {
    this_.write(z_publisher_options_t::default());
}

pub(crate) struct CPublisher {
    publisher: Publisher<'static>,
    sg: SyncGroup,
}

impl Deref for CPublisher {
    type Target = Publisher<'static>;

    fn deref(&self) -> &Self::Target {
        &self.publisher
    }
}

pub use crate::opaque_types::{z_loaned_publisher_t, z_moved_publisher_t, z_owned_publisher_t};
decl_c_type!(
    owned(z_owned_publisher_t, option CPublisher),
    loaned(z_loaned_publisher_t),
);

pub(crate) fn _declare_publisher_inner(
    session: &'static z_loaned_session_t,
    key_expr: &'static z_loaned_keyexpr_t,
    options: Option<&mut z_publisher_options_t>,
) -> PublisherBuilder<'static, 'static> {
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref().clone().into_owned();
    let mut p = session.declare_publisher(key_expr);
    if let Some(options) = options {
        p = p
            .congestion_control(options.congestion_control.into())
            .priority(options.priority.into())
            .express(options.is_express)
            .allowed_destination(options.allowed_destination.into());
        #[cfg(feature = "unstable")]
        {
            p = p.reliability(options.reliability.into());
        }
        if let Some(encoding) = options.encoding.take() {
            p = p.encoding(encoding.take_rust_type());
        }
    }
    p
}

/// Constructs and declares a publisher for the given key expression.
///
/// Data can be put and deleted with this publisher with the help of the
/// `z_publisher_put()` and `z_publisher_delete()` functions.
///
/// @param session: The Zenoh session.
/// @param publisher: An uninitialized location in memory where publisher will be constructed.
/// @param key_expr: The key expression to publish.
/// @param options: Additional options for the publisher.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_declare_publisher(
    session: &'static z_loaned_session_t,
    publisher: &'static mut MaybeUninit<z_owned_publisher_t>,
    key_expr: &'static z_loaned_keyexpr_t,
    options: Option<&'static mut z_publisher_options_t>,
) -> result::z_result_t {
    let this = publisher.as_rust_type_mut_uninit();
    let p = _declare_publisher_inner(session, key_expr, options);
    match p.wait() {
        Err(e) => {
            crate::report_error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
        Ok(publisher) => {
            this.write(Some(CPublisher {
                publisher,
                sg: SyncGroup::new(),
            }));
            result::Z_OK
        }
    }
}

/// Constructs a publisher in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_publisher_null(this_: &mut MaybeUninit<z_owned_publisher_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Returns ``true`` if publisher is valid, ``false`` otherwise.
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
#[derive(Default)]
pub struct z_publisher_put_options_t {
    ///  The encoding of the data to publish.
    pub encoding: Option<&'static mut z_moved_encoding_t>,
    /// The timestamp of the publication.
    pub timestamp: Option<&'static z_timestamp_t>,
    #[cfg(feature = "unstable")]
    /// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
    ///
    /// The source info for the publication.
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

pub(crate) fn _apply_pubisher_put_options<
    T: SampleBuilderTrait + TimestampBuilderTrait + EncodingBuilderTrait,
>(
    builder: T,
    options: &mut z_publisher_put_options_t,
) -> T {
    let mut builder = builder;
    if let Some(encoding) = options.encoding.take() {
        builder = builder.encoding(encoding.take_rust_type());
    };
    #[cfg(feature = "unstable")]
    if let Some(source_info) = options.source_info.take() {
        builder = builder.source_info(source_info.take_rust_type());
    };
    if let Some(attachment) = options.attachment.take() {
        builder = builder.attachment(attachment.take_rust_type());
    }
    if let Some(timestamp) = options.timestamp {
        builder = builder.timestamp(Some(*timestamp.as_rust_type_ref()));
    }
    builder
}

/// Sends a `PUT` message onto the publisher's key expression, transfering the payload ownership.
///
///
/// The payload and all owned options fields are consumed upon function return.
///
/// @param this_: The publisher.
/// @param payload: The data to publish. Will be consumed.
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
        put = _apply_pubisher_put_options(put, options);
    }

    match put.wait() {
        Ok(_) => result::Z_OK,
        Err(e) if e.downcast_ref::<SessionClosedError>().is_some() => result::Z_ESESSION_CLOSED,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// Represents the set of options that can be applied to the delete operation by a previously declared publisher,
/// whenever issued via `z_publisher_delete()`.
#[repr(C)]
#[derive(Default)]
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
    this.write(z_publisher_delete_options_t::default());
}

pub(crate) fn _apply_pubisher_delete_options<T: TimestampBuilderTrait>(
    builder: T,
    options: &mut z_publisher_delete_options_t,
) -> T {
    let mut builder = builder;
    if let Some(timestamp) = options.timestamp {
        builder = builder.timestamp(Some(*timestamp.as_rust_type_ref()));
    }
    builder
}

/// Sends a `DELETE` message onto the publisher's key expression.
///
/// @return 0 in case of success, negative error code in case of failure.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_delete(
    publisher: &z_loaned_publisher_t,
    options: Option<&mut z_publisher_delete_options_t>,
) -> result::z_result_t {
    let publisher = publisher.as_rust_type_ref();
    let mut del = publisher.delete();
    if let Some(options) = options {
        del = _apply_pubisher_delete_options(del, options);
    }
    if let Err(e) = del.wait() {
        crate::report_error!("{}", e);
        result::Z_EGENERIC
    } else {
        result::Z_OK
    }
}
#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the ID of the publisher.
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

fn _publisher_matching_listener_declare_inner<'a>(
    publisher: &'a z_loaned_publisher_t,
    notifier: SgNotifier,
    callback: &mut z_moved_closure_matching_status_t,
) -> zenoh::matching::MatchingListenerBuilder<'a, Callback<MatchingStatus>> {
    use crate::SyncObj;

    let publisher = publisher.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let sync_callback = SyncObj::new(callback, notifier);
    let listener = publisher
        .matching_listener()
        .callback_mut(move |matching_status| {
            let status = z_matching_status_t {
                matching: matching_status.matching(),
            };
            z_closure_matching_status_call(z_closure_matching_status_loan(&sync_callback), &status);
        });
    listener
}

/// @brief Constructs matching listener, registering a callback for notifying subscribers matching with a given publisher.
///
/// @param publisher: A publisher to associate with matching listener.
/// @param matching_listener: An uninitialized memory location where matching listener will be constructed. The matching listener's callback will be automatically dropped when the publisher is dropped.
/// @param callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber disconnects or when the first subscriber connects).
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_publisher_declare_matching_listener(
    publisher: &'static z_loaned_publisher_t,
    matching_listener: &mut MaybeUninit<z_owned_matching_listener_t>,
    callback: &mut z_moved_closure_matching_status_t,
) -> result::z_result_t {
    let this = matching_listener.as_rust_type_mut_uninit();
    let sg = SyncGroup::new();
    let listener = _publisher_matching_listener_declare_inner(publisher, sg.notifier(), callback);
    match listener.wait() {
        Ok(listener) => {
            this.write(Some(CMatchingListener { listener, _sg: sg }));
            result::Z_OK
        }
        Err(e) => {
            this.write(None);
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// @brief Declares a matching listener, registering a callback for notifying subscribers matching with a given publisher.
/// The callback will be run in the background until the corresponding publisher is dropped.
///
/// @param publisher: A publisher to associate with matching listener.
/// @param callback: A closure that will be called every time the matching status of the publisher changes (If last subscriber disconnects or when the first subscriber connects).
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_publisher_declare_background_matching_listener(
    publisher: &'static z_loaned_publisher_t,
    callback: &mut z_moved_closure_matching_status_t,
) -> result::z_result_t {
    let listener = _publisher_matching_listener_declare_inner(
        publisher,
        publisher.as_rust_type_ref().sg.notifier(),
        callback,
    );
    match listener.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// @brief Gets publisher matching status - i.e. if there are any subscribers matching its key expression.
///
/// @return 0 in case of success, negative error code otherwise (in this case matching_status is not updated).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_get_matching_status(
    this: &'static z_loaned_publisher_t,
    matching_status: &mut MaybeUninit<z_matching_status_t>,
) -> result::z_result_t {
    match this.as_rust_type_ref().matching_status().wait() {
        Ok(s) => {
            matching_status.write(z_matching_status_t {
                matching: s.matching(),
            });
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_ENETWORK
        }
    }
}

/// Frees memory and resets publisher to its gravestone state.
/// This is equivalent to calling `z_undeclare_publisher()` and discarding its return value.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_publisher_drop(this: &mut z_moved_publisher_t) {
    std::mem::drop(this.take_rust_type())
}

#[no_mangle]
/// @brief Undeclares the given publisher.
///
/// @return 0 in case of success, negative error code otherwise.
pub extern "C" fn z_undeclare_publisher(this_: &mut z_moved_publisher_t) -> result::z_result_t {
    if let Some(p) = this_.take_rust_type() {
        if let Err(e) = p.publisher.undeclare().wait() {
            crate::report_error!("{}", e);
            return result::Z_ENETWORK;
        }
    }
    result::Z_OK
}
