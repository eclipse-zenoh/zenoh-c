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

use zenoh::{
    config::{Locator, WhatAmI, WhatAmIMatcher},
    scouting::Hello,
};
use zenoh_runtime::ZRuntime;

pub use crate::opaque_types::{z_loaned_hello_t, z_moved_hello_t, z_owned_hello_t};
use crate::{
    result,
    transmute::{IntoCType, LoanedCTypeRef, RustTypeRef, RustTypeRefUninit, TakeRustType},
    z_closure_hello_call, z_closure_hello_loan, z_hello_locators_iterator_t, z_id_t,
    z_loaned_locator_t, z_moved_closure_hello_t, z_moved_config_t, z_owned_string_array_t,
    z_view_string_t, CStringInner, CStringView, ZVector,
};
decl_c_type!(
    owned(z_owned_hello_t, option Hello ),
    loaned(z_loaned_hello_t),
);

decl_c_type!(loaned(z_hello_locators_iterator_t, std::slice::Iter<'static, Locator>));

// C discards this iterator without running the underlying Rust type's drop glue.
const _: () = {
    assert!(
        !std::mem::needs_drop::<<z_hello_locators_iterator_t as RustTypeRef>::RustType>(),
        "Hello locator iterator must not require drop"
    );
};

/// Returns an iterator over the locators of this Hello message without allocating.
///
/// The iterator, returned locators, and their string views borrow from `this_`, which must remain
/// alive and unchanged while they are used. Neither the iterator, locators, nor their views need
/// to be dropped. Copying the iterator creates an independent cursor at the same position.
#[no_mangle]
pub extern "C" fn z_hello_get_locators_iterator(
    this_: &'static z_loaned_hello_t,
) -> z_hello_locators_iterator_t {
    *this_
        .as_rust_type_ref()
        .locators_iter()
        .as_loaned_c_type_ref()
}

/// Gets the next borrowed locator, in the order of `z_hello_locators()`.
/// @param this_: Locator iterator.
/// @param out: An uninitialized memory location for a pointer to the next locator.
/// @return `true` if a locator was written, or `false` at the end (leaving `out` unchanged).
#[no_mangle]
pub extern "C" fn z_hello_locators_iterator_next(
    this_: &mut z_hello_locators_iterator_t,
    out: &mut MaybeUninit<&'static z_loaned_locator_t>,
) -> bool {
    match this_.as_rust_type_mut().next() {
        Some(locator) => {
            out.write(locator.as_loaned_c_type_ref());
            true
        }
        None => false,
    }
}

/// Frees memory and resets hello message to its gravestone state.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_drop(this_: &mut z_moved_hello_t) {
    let _ = this_.take_rust_type();
}

/// Borrows hello message.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_loan(this_: &z_owned_hello_t) -> &z_loaned_hello_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap()
        .as_loaned_c_type_ref()
}

/// Mutably borrows hello message.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_loan_mut(this_: &mut z_owned_hello_t) -> &mut z_loaned_hello_t {
    this_
        .as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// Takes ownership of the mutably borrowed hello
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_hello_take_from_loaned(
    dst: &mut MaybeUninit<z_owned_hello_t>,
    src: &mut z_loaned_hello_t,
) {
    let dst = dst.as_rust_type_mut_uninit();
    let src = src.as_rust_type_mut();
    let src = std::mem::replace(src, Hello::empty());
    dst.write(Some(src));
}

/// Returns ``true`` if `hello message` is valid, ``false`` if it is in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_hello_check(this_: &z_owned_hello_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Constructs hello message in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_hello_null(this_: &mut MaybeUninit<z_owned_hello_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Constructs an owned copy of hello message.
#[no_mangle]
pub extern "C" fn z_hello_clone(dst: &mut MaybeUninit<z_owned_hello_t>, this_: &z_loaned_hello_t) {
    dst.as_rust_type_mut_uninit()
        .write(Some(this_.as_rust_type_ref().clone()));
}

/// @brief Returns id of Zenoh entity that transmitted hello message.
#[no_mangle]
pub extern "C" fn z_hello_zid(this_: &z_loaned_hello_t) -> z_id_t {
    this_.as_rust_type_ref().zid().into_c_type()
}

/// Returns type of Zenoh entity that transmitted hello message.
#[no_mangle]
pub extern "C" fn z_hello_whatami(this_: &z_loaned_hello_t) -> z_whatami_t {
    match this_.as_rust_type_ref().whatami() {
        WhatAmI::Router => z_whatami_t::ROUTER,
        WhatAmI::Peer => z_whatami_t::PEER,
        WhatAmI::Client => z_whatami_t::CLIENT,
    }
}

/// Constructs an array of non-owned locators (in the form non-null-terminated strings) of Zenoh entity that sent hello message.
///
/// The lifetime of locator strings is bound to `this_`.
#[no_mangle]
pub extern "C" fn z_hello_locators(
    this: &z_loaned_hello_t,
    locators_out: &mut MaybeUninit<z_owned_string_array_t>,
) {
    let this = this.as_rust_type_ref();
    let mut locators = ZVector::with_capacity(this.locators().len());
    for l in this.locators().iter() {
        locators.push(CStringInner::new_borrowed_from_slice(l.as_str().as_bytes()));
    }
    locators_out.as_rust_type_mut_uninit().write(locators);
}

/// Options to pass to `z_scout()`.
#[derive(Clone)]
#[repr(C)]
pub struct z_scout_options_t {
    /// The maximum duration in ms the scouting can take.
    pub timeout_ms: u64,
    /// Type of entities to scout for.
    pub what: z_what_t,
}

impl Default for z_scout_options_t {
    fn default() -> Self {
        z_scout_options_t {
            timeout_ms: DEFAULT_SCOUTING_TIMEOUT,
            what: DEFAULT_SCOUTING_WHAT,
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum z_whatami_t {
    ROUTER = 0x01,
    PEER = 0x02,
    CLIENT = 0x04,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum z_what_t {
    ROUTER = 0x01,
    PEER = 0x02,
    CLIENT = 0x04,
    ROUTER_PEER = 0x03,        // 0x01 | 0x02,
    ROUTER_CLIENT = 0x05,      // 0x01 | 0x04,
    PEER_CLIENT = 0x06,        // 0x02 | 0x04,
    ROUTER_PEER_CLIENT = 0x07, // 0x01 | 0x02 | 0x04,
}

pub const DEFAULT_SCOUTING_WHAT: z_what_t = z_what_t::ROUTER_PEER;
pub const DEFAULT_SCOUTING_TIMEOUT: u64 = 1000;

/// Constructs the default values for the scouting operation.
#[no_mangle]
pub extern "C" fn z_scout_options_default(this_: &mut MaybeUninit<z_scout_options_t>) {
    this_.write(z_scout_options_t::default());
}

/// Scout for routers and/or peers.
///
/// @param config: A set of properties to configure scouting session.
/// @param callback: A closure that will be called on each hello message received from discoverd Zenoh entities.
/// @param options: A set of scouting options
///
/// @return 0 if successful, negative error values upon failure.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn z_scout(
    config: &mut z_moved_config_t,
    callback: &mut z_moved_closure_hello_t,
    options: Option<&z_scout_options_t>,
) -> result::z_result_t {
    let callback = callback.take_rust_type();
    let options = options.cloned().unwrap_or_default();

    let Ok(what) = WhatAmIMatcher::try_from(options.what as u8) else {
        crate::report_error!("Invalid WhatAmIMatcher value: {:?}", options.what);
        return result::Z_EINVAL;
    };

    #[allow(clippy::unnecessary_cast)] // Required for multi-target
    let timeout = options.timeout_ms;
    let Some(config) = config.take_rust_type() else {
        crate::report_error!("Config not provided");
        return result::Z_EINVAL;
    };

    ZRuntime::Application.block_in_place(async move {
        let res = zenoh::scout(what, config)
            .callback(move |h| {
                let mut owned_h = Some(h);
                z_closure_hello_call(z_closure_hello_loan(&callback), unsafe {
                    owned_h.as_mut().unwrap_unchecked().as_loaned_c_type_mut()
                })
            })
            .await;

        match res {
            Ok(_) => {
                tokio::time::sleep(std::time::Duration::from_millis(timeout)).await;
                result::Z_OK
            }
            Err(e) => {
                crate::report_error!("{}", e);
                result::Z_EGENERIC
            }
        }
    })
}

/// Constructs a non-owned non-null-terminated string from the kind of zenoh entity.
///
/// The string has static storage (i.e. valid until the end of the program).
/// @param whatami: A whatami bitmask of zenoh entity kind.
/// @param str_out: An uninitialized memory location where strring will be constructed.
///
/// @return 0 if successful, negative error values if whatami contains an invalid bitmask.
#[no_mangle]
pub extern "C" fn z_whatami_to_view_string(
    whatami: z_whatami_t,
    str_out: &mut MaybeUninit<z_view_string_t>,
) -> result::z_result_t {
    match WhatAmIMatcher::try_from(whatami as u8) {
        Err(_) => result::Z_EINVAL,
        Ok(w) => {
            let s = w.to_str();
            let slice = CStringView::new_borrowed_from_slice(s.as_bytes());
            str_out.as_rust_type_mut_uninit().write(slice);
            result::Z_OK
        }
    }
}

#[cfg(test)]
mod iterator_tests {
    use std::sync::OnceLock;

    use zenoh_protocol::{core::ZenohIdProto, scouting::HelloProto};

    use super::*;
    use crate::{z_string_array_get, z_string_array_len, z_string_array_loan};

    #[test]
    fn hello_locators_iterator_matches_array() {
        static HELLO: OnceLock<Hello> = OnceLock::new();
        let hello = HELLO.get_or_init(|| {
            HelloProto {
                version: zenoh_protocol::VERSION,
                whatami: WhatAmI::default(),
                zid: ZenohIdProto::default(),
                locators: vec![
                    "tcp/127.0.0.1:7447".parse().unwrap(),
                    "udp/127.0.0.1:7448".parse().unwrap(),
                ],
            }
            .into()
        });
        let hello_c = hello.as_loaned_c_type_ref();
        let mut array = MaybeUninit::uninit();
        z_hello_locators(hello_c, &mut array);
        let owned_array = unsafe { array.assume_init() };
        let array = unsafe { z_string_array_loan(&owned_array) };
        let mut iter = z_hello_get_locators_iterator(hello_c);
        let mut copy = iter;
        let mut out = MaybeUninit::uninit();
        for i in 0..z_string_array_len(array) {
            assert!(z_hello_locators_iterator_next(&mut iter, &mut out));
            let view = unsafe { out.assume_init_ref() }.as_rust_type_ref();
            assert!(std::ptr::eq(view, &hello.locators()[i]));
            let expected = z_string_array_get(array, i).unwrap().as_rust_type_ref();
            assert_eq!(view.as_str().as_bytes(), expected.slice());
            assert_eq!(view.as_str().as_bytes().as_ptr(), expected.slice().as_ptr());
        }
        let last = unsafe { out.assume_init() };
        let saved = last.as_rust_type_ref().as_str().as_bytes();
        let mut out = MaybeUninit::new(last);
        assert!(!z_hello_locators_iterator_next(&mut iter, &mut out));
        assert!(!z_hello_locators_iterator_next(&mut iter, &mut out));
        assert_eq!(
            unsafe { out.assume_init_ref() }
                .as_rust_type_ref()
                .as_str()
                .as_bytes(),
            saved
        );
        assert!(z_hello_locators_iterator_next(&mut copy, &mut out));
        assert_eq!(
            unsafe { out.assume_init_ref() }
                .as_rust_type_ref()
                .as_str()
                .as_bytes(),
            hello.locators()[0].as_str().as_bytes()
        );
        drop(owned_array);
    }

    #[test]
    fn empty_hello_locators_iterator() {
        assert!(!std::mem::needs_drop::<z_hello_locators_iterator_t>());
        static HELLO: OnceLock<Hello> = OnceLock::new();
        let hello = HELLO.get_or_init(Hello::empty);
        let mut iter = z_hello_get_locators_iterator(hello.as_loaned_c_type_ref());
        assert!(!z_hello_locators_iterator_next(
            &mut iter,
            &mut MaybeUninit::uninit()
        ));
    }
}
