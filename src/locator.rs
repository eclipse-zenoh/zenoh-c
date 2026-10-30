//
// Copyright (c) 2026 ZettaScale Technology.
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

use zenoh::config::Locator;

pub use crate::opaque_types::z_loaned_locator_t;
use crate::{
    transmute::{RustTypeRef, RustTypeRefUninit},
    z_view_string_t, CStringView,
};

decl_c_type!(loaned(z_loaned_locator_t, Locator));

fn write_view(out: &mut MaybeUninit<z_view_string_t>, text: &str) {
    out.as_rust_type_mut_uninit()
        .write(CStringView::new_borrowed_from_slice(text.as_bytes()));
}

/// Constructs a borrowed view of the complete locator string, without allocating.
/// The owner must remain alive and unchanged while the view is used. The view needs no drop.
#[no_mangle]
pub extern "C" fn z_locator_as_view_string(
    this_: &z_loaned_locator_t,
    out: &mut MaybeUninit<z_view_string_t>,
) {
    write_view(out, this_.as_rust_type_ref().as_str());
}

/// Constructs a borrowed view of the locator's protocol, without allocating.
/// The owner must remain alive and unchanged while the view is used. The view needs no drop.
#[no_mangle]
pub extern "C" fn z_locator_protocol(
    this_: &z_loaned_locator_t,
    out: &mut MaybeUninit<z_view_string_t>,
) {
    write_view(out, this_.as_rust_type_ref().protocol().as_str());
}

/// Constructs a borrowed view of the locator's address, without allocating.
/// An empty locator produces an empty view.
/// The owner must remain alive and unchanged while the view is used. The view needs no drop.
#[no_mangle]
pub extern "C" fn z_locator_address(
    this_: &z_loaned_locator_t,
    out: &mut MaybeUninit<z_view_string_t>,
) {
    write_view(out, this_.as_rust_type_ref().address().as_str());
}

#[cfg(test)]
mod iterator_tests {
    use std::sync::OnceLock;

    use super::*;
    use crate::transmute::LoanedCTypeRef;

    fn bytes(out: &MaybeUninit<z_view_string_t>) -> &[u8] {
        unsafe { out.assume_init_ref() }.as_rust_type_ref().slice()
    }

    #[test]
    fn empty_locator_component_views() {
        let locator = Locator::empty();
        for getter in [
            z_locator_as_view_string,
            z_locator_protocol,
            z_locator_address,
        ] {
            let mut out = MaybeUninit::uninit();
            out.as_rust_type_mut_uninit()
                .write(CStringView::new_borrowed_from_slice(b"previous value"));
            getter(locator.as_loaned_c_type_ref(), &mut out);
            assert!(bytes(&out).is_empty());
        }
    }

    #[test]
    fn locator_component_views_borrow_owner() {
        static LOCATOR: OnceLock<Locator> = OnceLock::new();
        let locator = LOCATOR
            .get_or_init(|| "tcp/host:7447".parse().unwrap())
            .as_loaned_c_type_ref();
        let mut text = MaybeUninit::uninit();
        let mut protocol = MaybeUninit::uninit();
        let mut address = MaybeUninit::uninit();
        z_locator_as_view_string(locator, &mut text);
        z_locator_protocol(locator, &mut protocol);
        z_locator_address(locator, &mut address);
        assert_eq!(bytes(&text), b"tcp/host:7447");
        assert_eq!(
            bytes(&text).as_ptr(),
            locator.as_rust_type_ref().as_str().as_ptr()
        );
        assert_eq!(bytes(&protocol), b"tcp");
        assert_eq!(bytes(&protocol).as_ptr(), bytes(&text).as_ptr());
        assert_eq!(bytes(&address), b"host:7447");
        assert_eq!(bytes(&address).as_ptr(), bytes(&text)[4..].as_ptr());
    }
}
