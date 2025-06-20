//
// Copyright (c) 2017, 2024 ZettaScale Technology.
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

use std::{ffi::CStr, mem::MaybeUninit, ops::Deref};

use libc::c_char;
#[cfg(feature = "unstable")]
use zenoh::{handlers::Callback, matching::MatchingStatus};
use zenoh::{
    qos::{CongestionControl, Priority},
    query::{Querier, QueryConsolidation, QueryTarget},
    session::SessionClosedError,
    Wait,
};

use crate::{
    result,
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_reply_call, z_closure_reply_loan, z_congestion_control_t, z_loaned_keyexpr_t,
    z_loaned_querier_t, z_loaned_session_t, z_moved_bytes_t, z_moved_closure_reply_t,
    z_moved_encoding_t, z_moved_querier_t, z_owned_querier_t, z_priority_t,
    z_query_consolidation_t, z_query_target_t, SgNotifier, SyncGroup, SyncObj,
};
#[cfg(feature = "unstable")]
use crate::{
    transmute::IntoCType, z_closure_matching_status_call, z_closure_matching_status_loan,
    z_entity_global_id_t, z_matching_status_t, z_moved_closure_matching_status_t,
    z_moved_source_info_t, z_owned_matching_listener_t, zc_locality_default, zc_locality_t,
    zc_reply_keyexpr_default, zc_reply_keyexpr_t,
};

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Options passed to the `z_declare_querier()` function.
#[repr(C)]
pub struct z_querier_options_t {
    /// The Queryables that should be target of the querier queries.
    pub target: z_query_target_t,
    /// The replies consolidation strategy to apply on replies to the querier queries.
    pub consolidation: z_query_consolidation_t,
    /// The congestion control to apply when routing the querier queries.
    pub congestion_control: z_congestion_control_t,
    /// If set to ``true``, the querier queries will not be batched. This usually has a positive impact on latency but negative impact on throughput.
    pub is_express: bool,
    #[cfg(feature = "unstable")]
    /// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
    ///
    /// The allowed destination for the querier queries.
    pub allowed_destination: zc_locality_t,
    #[cfg(feature = "unstable")]
    /// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
    ///
    /// The accepted replies for the querier queries.
    pub accept_replies: zc_reply_keyexpr_t,
    /// The priority of the querier queries.
    pub priority: z_priority_t,
    /// The timeout for the querier queries in milliseconds. 0 means default query timeout from zenoh configuration.
    pub timeout_ms: u64,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs the default value for `z_querier_options_t`.
#[no_mangle]
pub extern "C" fn z_querier_options_default(this_: &mut MaybeUninit<z_querier_options_t>) {
    this_.write(z_querier_options_t {
        target: QueryTarget::default().into(),
        consolidation: QueryConsolidation::default().into(),
        congestion_control: CongestionControl::DEFAULT_REQUEST.into(),
        priority: Priority::default().into(),
        is_express: false,
        #[cfg(feature = "unstable")]
        allowed_destination: zc_locality_default(),
        #[cfg(feature = "unstable")]
        accept_replies: zc_reply_keyexpr_default(),
        timeout_ms: 0,
    });
}

pub(crate) struct CQuerier {
    querier: Querier<'static>,
    sg: SyncGroup,
}

impl Deref for CQuerier {
    type Target = Querier<'static>;

    fn deref(&self) -> &Self::Target {
        &self.querier
    }
}

decl_c_type!(
    owned(z_owned_querier_t, option CQuerier),
    loaned(z_loaned_querier_t),
);

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs and declares a querier on the given key expression.
///
/// The queries can be send with the help of the `z_querier_get()` function.
///
/// @param session: The Zenoh session.
/// @param querier: An uninitialized location in memory where querier will be constructed.
/// @param key_expr: The key expression to send queries on.
/// @param options: Additional options for the querier.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_declare_querier(
    session: &z_loaned_session_t,
    querier: &mut MaybeUninit<z_owned_querier_t>,
    key_expr: &z_loaned_keyexpr_t,
    options: Option<&mut z_querier_options_t>,
) -> result::z_result_t {
    let this = querier.as_rust_type_mut_uninit();
    let session = session.as_rust_type_ref();
    let key_expr = key_expr.as_rust_type_ref().clone().into_owned();
    let mut q = session.declare_querier(key_expr);
    if let Some(options) = options {
        q = q
            .congestion_control(options.congestion_control.into())
            .priority(options.priority.into())
            .express(options.is_express)
            .target(options.target.into())
            .consolidation(options.consolidation);
        if options.timeout_ms != 0 {
            q = q.timeout(std::time::Duration::from_millis(options.timeout_ms));
        }
        #[cfg(feature = "unstable")]
        {
            q = q
                .accept_replies(options.accept_replies.into())
                .allowed_destination(options.allowed_destination.into());
        }
    }
    match q.wait() {
        Err(e) => {
            tracing::error!("{}", e);
            this.write(None);
            result::Z_EGENERIC
        }
        Ok(querier) => {
            this.write(Some(CQuerier {
                querier,
                sg: SyncGroup::new(),
            }));
            result::Z_OK
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs a querier in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_internal_querier_null(this_: &mut MaybeUninit<z_owned_querier_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if querier is valid, ``false`` otherwise.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_internal_querier_check(this_: &z_owned_querier_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows querier.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_querier_loan(this_: &z_owned_querier_t) -> &z_loaned_querier_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Mutably borrows querier.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_querier_loan_mut(
    this: &mut z_owned_querier_t,
) -> &mut z_loaned_querier_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Options passed to the `z_querier_get()` function.
#[repr(C)]
pub struct z_querier_get_options_t {
    /// An optional payload to attach to the query.
    pub payload: Option<&'static mut z_moved_bytes_t>,
    /// An optional encoding of the query payload and or attachment.
    pub encoding: Option<&'static mut z_moved_encoding_t>,
    #[cfg(feature = "unstable")]
    /// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
    ///
    /// The source info for the query.
    pub source_info: Option<&'static mut z_moved_source_info_t>,
    /// An optional attachment to attach to the query.
    pub attachment: Option<&'static mut z_moved_bytes_t>,
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs the default value for `z_querier_get_options_t`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_querier_get_options_default(this: &mut MaybeUninit<z_querier_get_options_t>) {
    this.write(z_querier_get_options_t {
        payload: None,
        encoding: None,
        #[cfg(feature = "unstable")]
        source_info: None,
        attachment: None,
    });
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Query data from the matching queryables in the system.
/// Replies are provided through a callback function.
///
/// @param querier: The querier to make query from.
/// @param parameters: The query's parameters, similar to a url's query segment.
/// @param callback: The callback function that will be called on reception of replies for this query. It will be automatically dropped once all replies are processed.
/// @param options: Additional options for the get. All owned fields will be consumed.
///
/// @return 0 in case of success, a negative error value upon failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn z_querier_get(
    querier: &z_loaned_querier_t,
    parameters: *const c_char,
    callback: &mut z_moved_closure_reply_t,
    options: Option<&mut z_querier_get_options_t>,
) -> result::z_result_t {
    let querier = querier.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let mut get = querier.get();
    if let Some(options) = options {
        if let Some(payload) = options.payload.take() {
            get = get.payload(payload.take_rust_type());
        }
        if let Some(encoding) = options.encoding.take() {
            get = get.encoding(encoding.take_rust_type());
        }
        #[cfg(feature = "unstable")]
        if let Some(source_info) = options.source_info.take() {
            get = get.source_info(source_info.take_rust_type());
        }
        if let Some(attachment) = options.attachment.take() {
            get = get.attachment(attachment.take_rust_type());
        }
    }
    if !parameters.is_null() {
        get = get.parameters(CStr::from_ptr(parameters).to_str().unwrap());
    }
    let sync_callback = SyncObj::new(callback, querier.sg.notifier());
    match get
        .callback(move |response| {
            let mut owned_response = Some(response);
            z_closure_reply_call(
                z_closure_reply_loan(&sync_callback),
                owned_response
                    .as_mut()
                    .unwrap_unchecked()
                    .as_loaned_c_type_mut(),
            )
        })
        .wait()
    {
        Ok(()) => result::Z_OK,
        Err(e) if e.downcast_ref::<SessionClosedError>().is_some() => result::Z_ESESSION_CLOSED,
        Err(e) => {
            tracing::error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the ID of the querier.
#[no_mangle]
pub extern "C" fn z_querier_id(querier: &z_loaned_querier_t) -> z_entity_global_id_t {
    querier.as_rust_type_ref().id().into_c_type()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns the key expression of the querier.
#[no_mangle]
pub extern "C" fn z_querier_keyexpr(querier: &z_loaned_querier_t) -> &z_loaned_keyexpr_t {
    querier.as_rust_type_ref().key_expr().as_loaned_c_type_ref()
}

#[cfg(feature = "unstable")]
fn _querier_matching_listener_declare_inner<'a>(
    querier: &'a z_loaned_querier_t,
    callback: &mut z_moved_closure_matching_status_t,
    notifier: SgNotifier,
) -> zenoh::matching::MatchingListenerBuilder<'a, Callback<MatchingStatus>> {
    let querier = querier.as_rust_type_ref();
    let callback = callback.take_rust_type();
    let sync_callback = SyncObj::new(callback, notifier);
    let listener = querier
        .matching_listener()
        .callback_mut(move |matching_status| {
            let status = z_matching_status_t {
                matching: matching_status.matching(),
            };
            z_closure_matching_status_call(z_closure_matching_status_loan(&sync_callback), &status);
        });
    listener
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs matching listener, registering a callback for notifying queryables matching with a given querier's key expression and target.
///
/// @param querier: A querier to associate with matching listener.
/// @param matching_listener: An uninitialized memory location where matching listener will be constructed. The matching listener's callback will be automatically dropped when the querier is dropped.
/// @param callback: A closure that will be called every time the matching status of the querier changes (If last queryable disconnects or when the first queryable connects).
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_querier_declare_matching_listener(
    querier: &'static z_loaned_querier_t,
    matching_listener: &mut MaybeUninit<z_owned_matching_listener_t>,
    callback: &mut z_moved_closure_matching_status_t,
) -> result::z_result_t {
    use crate::CMatchingListener;

    let this = matching_listener.as_rust_type_mut_uninit();
    let sg = SyncGroup::new();
    let listener = _querier_matching_listener_declare_inner(querier, callback, sg.notifier());
    match listener.wait() {
        Ok(listener) => {
            this.write(Some(CMatchingListener { listener, _sg: sg }));
            result::Z_OK
        }
        Err(e) => {
            this.write(None);
            tracing::error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Declares a matching listener, registering a callback for notifying queryables matching the given querier key expression and target.
/// The callback will be run in the background until the corresponding querier is dropped.
///
/// @param querier: A querier to associate with matching listener.
/// @param callback: A closure that will be called every time the matching status of the querier changes (If last queryable disconnects or when the first queryable connects).
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_querier_declare_background_matching_listener(
    querier: &'static z_loaned_querier_t,
    callback: &mut z_moved_closure_matching_status_t,
) -> result::z_result_t {
    let listener = _querier_matching_listener_declare_inner(
        querier,
        callback,
        querier.as_rust_type_ref().sg.notifier(),
    );
    match listener.background().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            tracing::error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

#[cfg(feature = "unstable")]
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Gets querier matching status - i.e. if there are any queryables matching its key expression and target.
///
/// @return 0 in case of success, negative error code otherwise (in this case matching_status is not updated).
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_querier_get_matching_status(
    this: &'static z_loaned_querier_t,
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
            tracing::error!("{}", e);
            result::Z_ENETWORK
        }
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Frees memory and resets querier to its gravestone state.
/// This is equivalent to calling `z_undeclare_querier()` and discarding its return value.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_querier_drop(this: &mut z_moved_querier_t) {
    std::mem::drop(this.take_rust_type())
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Undeclares the given querier.
///
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_undeclare_querier(this_: &mut z_moved_querier_t) -> result::z_result_t {
    if let Some(q) = this_.take_rust_type() {
        if let Err(e) = q.querier.undeclare().wait() {
            tracing::error!("{}", e);
            return result::Z_ENETWORK;
        }
    }
    result::Z_OK
}
