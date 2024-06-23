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

use std::mem::MaybeUninit;

pub(crate) trait IntoCTypeRef: Sized {
    type CType;
    fn as_ctype_ref(&self) -> &Self::CType;
    fn as_ctype_mut(&mut self) -> &mut Self::CType;
}

pub(crate) trait IntoRustTypeRef: Sized {
    type RustType;
    fn as_rust_type_ref(&self) -> &Self::RustType;
    fn as_rust_type_mut(&mut self) -> &mut Self::RustType;
    fn as_rust_type_uninit(this: &mut MaybeUninit<Self>) -> &mut MaybeUninit<Self::RustType>;
}

pub(crate) trait IntoCType {
    type RustType;
    fn into_ctype(self) -> Self::RustType;
}

pub(crate) trait IntoRustType {
    type CType;
    fn into_rust_type(self) -> Self::CType;
}
