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

pub(crate) trait OwnedCTypeRef: Sized {
    type OwnedCType;
    fn as_owned_ctype_ref(&self) -> &Self::OwnedCType;
    fn as_owned_ctype_mut(&mut self) -> &mut Self::OwnedCType;
}

pub(crate) trait LoanedCTypeRef: Sized {
    type LoanedCType;
    fn as_loaned_ctype_ref(&self) -> &Self::LoanedCType;
    fn as_loaned_ctype_mut(&mut self) -> &mut Self::LoanedCType;
}

pub(crate) trait RustTypeRef: Sized {
    type RustType;
    fn as_rust_type_ref(&self) -> &Self::RustType;
    fn as_rust_type_mut(&mut self) -> &mut Self::RustType;
}

pub(crate) trait RustTypeRefUninit: Sized {
    type RustType;
    fn as_rust_type_mut_uninit(&mut self) -> &mut MaybeUninit<Self::RustType>;
}

/// Transmute value of C wrapper type into value of corresponding Rust type
pub(crate) trait IntoCType: Sized + Copy {
    type RustType;
    fn into_ctype(self) -> Self::RustType;
}

/// Transmute value of Rust type into value of corresponding C wrapper type
pub(crate) trait IntoRustType: Sized + Copy {
    type CType;
    fn into_rust_type(self) -> Self::CType;
}

macro_rules! validate_equivalence2 {
    ($type_a:ty, $type_b:ty) => {
        const _: () = {
            use const_format::concatcp;
            const TYPE_NAME_A: &str = stringify!($type_a);
            const TYPE_NAME_B: &str = stringify!($type_b);
            const ALIGN_A: usize = std::mem::align_of::<$type_a>();
            const ALIGN_B: usize = std::mem::align_of::<$type_b>();
            if ALIGN_A != ALIGN_B {
                const ERR_MESSAGE: &str = concatcp!(
                    "Alingment mismatch: type ",
                    TYPE_NAME_A,
                    " has alignment ",
                    ALIGN_A,
                    " while type ",
                    TYPE_NAME_B,
                    " has alignment ",
                    ALIGN_B
                );
                panic!("{}", ERR_MESSAGE);
            }
            const SIZE_A: usize = std::mem::size_of::<$type_a>();
            const SIZE_B: usize = std::mem::size_of::<$type_b>();
            if SIZE_A != SIZE_B {
                const ERR_MESSAGE: &str = concatcp!(
                    "Size mismatch: type ",
                    TYPE_NAME_A,
                    " has size ",
                    SIZE_A,
                    " while type ",
                    TYPE_NAME_B,
                    " has size ",
                    SIZE_B
                );
                panic!("{}", ERR_MESSAGE);
            }
        };
    };
}

#[macro_export]
macro_rules! impl_transmute {
    (as_c_owned ($rust_type:ty, $c_type:ty)) => {
        impl $crate::transmute2::OwnedCTypeRef for $rust_type {
            type OwnedCType = $c_type;
            fn as_owned_ctype_ref(&self) -> &Self::OwnedCType {
                unsafe { &*(self as *const Self as *const Self::OwnedCType) }
            }
            fn as_owned_ctype_mut(&mut self) -> &mut Self::OwnedCType {
                unsafe { &mut *(self as *mut Self as *mut Self::OwnedCType) }
            }
        }
    };
    (as_c_loaned ($rust_type:ty, $c_type:ty)) => {
        impl $crate::transmute2::LoanedCTypeRef for $rust_type {
            type LoanedCType = $c_type;
            fn as_loaned_ctype_ref(&self) -> &Self::LoanedCType {
                unsafe { &*(self as *const Self as *const Self::LoanedCType) }
            }
            fn as_loaned_ctype_mut(&mut self) -> &mut Self::LoanedCType {
                unsafe { &mut *(self as *mut Self as *mut Self::LoanedCType) }
            }
        }
    };
    (as_rust ($c_type:ty, $rust_type:ty)) => {
        impl $crate::transmute2::RustTypeRef for $c_type {
            type RustType = $rust_type;
            fn as_rust_type_ref(&self) -> &Self::RustType {
                unsafe { &*(self as *const Self as *const Self::RustType) }
            }
            fn as_rust_type_mut(&mut self) -> &mut Self::RustType {
                unsafe { &mut *(self as *mut Self as *mut Self::RustType) }
            }
        }
        impl $crate::transmute2::RustTypeRefUninit for std::mem::MaybeUninit<$c_type> {
            type RustType = $rust_type;
            fn as_rust_type_mut_uninit(&mut self) -> &mut std::mem::MaybeUninit<Self::RustType> {
                unsafe {
                    let this = self as *mut std::mem::MaybeUninit<$c_type>;
                    &mut *(this as *mut std::mem::MaybeUninit<Self::RustType>)
                }
            }
        }
    };
}

// Declare "owned" C type. This type can be converted in place to it's Rust counterpart
// but can't be copied
#[macro_export]
macro_rules! decl_c_type {
    (owned ($c_owned_type:ty, $rust_owned_type:ty), loaned ($c_loaned_type:ty, $rust_loaned_type:ty)) => {
        validate_equivalence2!($c_owned_type, $rust_owned_type);
        validate_equivalence2!($c_loaned_type, $rust_loaned_type);
        validate_equivalence2!($c_owned_type, $c_loaned_type);
        impl_transmute!(as_c_owned($rust_owned_type, $c_owned_type));
        impl_transmute!(as_c_loaned($rust_loaned_type, $c_loaned_type));
        impl_transmute!(as_rust($c_owned_type, $rust_owned_type));
        impl_transmute!(as_rust($c_loaned_type, $rust_loaned_type));
    };
}
