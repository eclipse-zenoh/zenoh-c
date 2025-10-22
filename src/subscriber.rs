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
    pubsub::{Subscriber, SubscriberBuilder},
    sample::Sample,
    Wait,
};

pub use crate::opaque_types::{z_loaned_subscriber_t, z_moved_subscriber_t, z_owned_subscriber_t};
use crate::{
    keyexpr::*,
    result,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_sample_call, z_closure_sample_loan, z_loaned_session_t, z_moved_closure_sample_t,
    zc_locality_default, zc_locality_t, SgNotifier, SyncGroup, SyncObj,
};
#[cfg(feature = "unstable")]
use crate::{transmute::IntoCType, z_entity_global_id_t};

pub(crate) struct CSubscriber {
    pub(crate) subscriber: Subscriber<()>,
    pub(crate) _sg: SyncGroup,
}

impl Deref for CSubscriber {
    type Target = Subscriber<()>;

    fn deref(&self) -> &Self::Target {
        &self.subscriber
    }
}

decl_c_type!(
    owned(z_owned_subscriber_t, option CSubscriber),
    loaned(z_loaned_subscriber_t),
);

/// Constructs a subscriber in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_subscriber_null(this_: &mut MaybeUninit<z_owned_subscriber_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Borrows subscriber.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_subscriber_loan(this_: &z_owned_subscriber_t) -> &z_loaned_subscriber_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Options passed to the `z_declare_subscriber()` function.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct z_subscriber_options_t {
    /// Restricts the matching publications that will be received by this Subscriber to the ones
    /// that have the compatible allowed_destination.
    pub allowed_origin: zc_locality_t,
}

impl Default for z_subscriber_options_t {
    fn default() -> Self {
        Self {
            allowed_origin: zc_locality_default(),
        }
    }
}

/// Constructs the default value for `z_subscriber_options_t`.
#[no_mangle]
pub extern "C" fn z_subscriber_options_default(this_: &mut MaybeUninit<z_subscriber_options_t>) {
    this_.write(z_subscriber_options_t::default());
}

#[allow(unused_variables, unused_mut)]
pub(crate) fn _declare_subscriber_inner<'a, 'b>(
    session: &'a z_loaned_session_t,
    key_expr: &'b z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    notifier: SgNotifier,
    options: Option<&mut z_subscriber_options_t>,
) -> SubscriberBuilder<'a, 'b, Callback<Sample>> {
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let sync_callback = SyncObj::new(callback, notifier);
    let mut subscriber = session
        .declare_subscriber(key_expr)
        .callback(move |sample| {
            let mut owned_sample = Some(sample);
            z_closure_sample_call(z_closure_sample_loan(&sync_callback), unsafe {
                owned_sample
                    .as_mut()
                    .unwrap_unchecked()
                    .as_loaned_c_type_mut()
            })
        });
    if let Some(options) = options {
        subscriber = subscriber.allowed_origin(options.allowed_origin.into());
    }
    subscriber
}

/// Constructs and declares a subscriber for a given key expression. Dropping subscriber undeclares its callback.
///
/// @param session: The zenoh session.
/// @param subscriber: An uninitialized location in memory, where subscriber will be constructed.
/// @param key_expr: The key expression to subscribe.
/// @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
/// @param options: The options to be passed to the subscriber declaration.
///
/// @return 0 in case of success, negative error code otherwise (in this case subscriber will be in its gravestone state).
#[no_mangle]
pub extern "C" fn z_declare_subscriber(
    session: &z_loaned_session_t,
    subscriber: &mut MaybeUninit<z_owned_subscriber_t>,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    options: Option<&mut z_subscriber_options_t>,
) -> result::z_result_t {
    let this = subscriber.as_rust_type_mut_uninit();
    let sg = SyncGroup::new();
    let s = _declare_subscriber_inner(session, key_expr, callback, sg.notifier(), options);
    match s.wait() {
        Ok(sub) => {
            this.write(Some(CSubscriber {
                subscriber: sub,
                _sg: sg,
            }));
            result::Z_OK
        }
        Err(e) => {
            crate::report_error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
    }
}

/// Constructs and declares a background subscriber. Subscriber callback will be called to process the messages,
/// until the corresponding session is closed or dropped.
///
/// @param session: The zenoh session.
/// @param key_expr: The key expression to subscribe.
/// @param callback: The callback function that will be called each time a data matching the subscribed expression is received.
/// @param options: The options to be passed to the subscriber declaration.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_declare_background_subscriber(
    session: &z_loaned_session_t,
    key_expr: &z_loaned_keyexpr_t,
    callback: &mut z_moved_closure_sample_t,
    options: Option<&mut z_subscriber_options_t>,
) -> result::z_result_t {
    let subscriber = _declare_subscriber_inner(
        session,
        key_expr,
        callback,
        session.as_rust_type_ref().notifier(),
        options,
    );
    match subscriber.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// Returns the key expression of the subscriber.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_subscriber_keyexpr(subscriber: &z_loaned_subscriber_t) -> &z_loaned_keyexpr_t {
    subscriber
        .as_rust_type_ref()
        .key_expr()
        .as_loaned_c_type_ref()
}

/// Undeclares subscriber callback and resets it to its gravestone state.
/// This is equivalent to calling `z_undeclare_subscriber()` and discarding its return value.
#[no_mangle]
pub extern "C" fn z_subscriber_drop(this_: &mut z_moved_subscriber_t) {
    std::mem::drop(this_.take_rust_type())
}

/// Returns ``true`` if subscriber is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_internal_subscriber_check(this_: &z_owned_subscriber_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Undeclares the subscriber.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_undeclare_subscriber(this_: &mut z_moved_subscriber_t) -> result::z_result_t {
    if let Some(s) = this_.take_rust_type() {
        if let Err(e) = s.subscriber.undeclare().wait() {
            crate::report_error!("{}", e);
            return result::Z_EGENERIC;
        }
    }
    result::Z_OK
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the ID of the subscriber.
#[no_mangle]
pub extern "C" fn z_subscriber_id(subscriber: &z_loaned_subscriber_t) -> z_entity_global_id_t {
    subscriber.as_rust_type_ref().id().into_c_type()
}
