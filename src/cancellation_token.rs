//
// Copyright (c) 2017, 2025 ZettaScale Technology.
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

use zenoh::{cancellation::CancellationToken, Wait};
use zenoh_ffi_opaque_types::opaque_types::{
    z_loaned_cancellation_token_t, z_moved_cancellation_token_t, z_owned_cancellation_token_t,
};

decl_c_type!(
    owned(z_owned_cancellation_token_t, z_moved_cancellation_token_t,option CancellationToken),
    loaned(z_loaned_cancellation_token_t, CancellationToken),
);

use crate::{
    result::{self, z_result_t},
    transmute::{LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
};

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Borrows cancellation token.
#[no_mangle]
pub extern "C" fn z_cancellation_token_loan(
    this_: &'static z_owned_cancellation_token_t,
) -> &'static z_loaned_cancellation_token_t {
    let this = this_.as_rust_type_ref();
    let this = unsafe { this.as_ref().unwrap_unchecked() };
    this.as_loaned_c_type_ref()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Mutably borrows cancellation token.
#[no_mangle]
pub extern "C" fn z_cancellation_token_loan_mut(
    this_: &mut z_owned_cancellation_token_t,
) -> &mut z_loaned_cancellation_token_t {
    let this = this_.as_rust_type_mut();
    let this = unsafe { this.as_mut().unwrap_unchecked() };
    this.as_loaned_c_type_mut()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs a new cancellation token.
#[no_mangle]
pub extern "C" fn z_cancellation_token_new(
    this_: &mut MaybeUninit<z_owned_cancellation_token_t>,
) -> result::z_result_t {
    this_
        .as_rust_type_mut_uninit()
        .write(Some(CancellationToken::default()));
    result::Z_OK
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Constructs cancellation token in its gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_cancellation_token_null(
    this_: &mut MaybeUninit<z_owned_cancellation_token_t>,
) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Clones the cancellation token into provided uninitialized memory location.
///
/// Cancelling token also cancels all of its clones.
#[no_mangle]
pub extern "C" fn z_cancellation_token_clone(
    dst: &mut MaybeUninit<z_owned_cancellation_token_t>,
    this: &z_loaned_cancellation_token_t,
) {
    let src = Some(this.as_rust_type_ref().clone());
    let dst = dst.as_rust_type_mut_uninit();
    dst.write(src);
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Frees cancellation_token, and resets it to its gravestone state.
#[no_mangle]
pub extern "C" fn z_cancellation_token_drop(this_: &mut z_moved_cancellation_token_t) {
    let _ = this_.take_rust_type();
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if cancellation_token is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "C" fn z_internal_cancellation_token_check(
    this_: &z_owned_cancellation_token_t,
) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Interrupts all associated GET queries. If the query callback is being executed, the call blocks until execution of callback is finished.
/// In case of failure, some operations might not be cancelled.
/// Once cancelled, all newly added GET queries will cancel automatically.
///
/// @return 0 in case of success, negative error code in case of failure.
#[no_mangle]
pub extern "C" fn z_cancellation_token_cancel(
    this_: &mut z_loaned_cancellation_token_t,
) -> z_result_t {
    match this_.as_rust_type_ref().cancel().wait() {
        Ok(_) => result::Z_OK,
        Err(e) => {
            crate::report_error!("{}", e);
            result::Z_EGENERIC
        }
    }
}

/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief Returns ``true`` if cancellation token was cancelled (i .e. if `z_cancellation_token_cancel()` was called), ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_cancellation_token_is_cancelled(this_: &z_loaned_cancellation_token_t) -> bool {
    this_.as_rust_type_ref().is_cancelled()
}
