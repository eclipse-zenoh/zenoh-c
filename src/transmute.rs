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

pub(crate) trait CTypeRef: Sized {
    type CType;
    fn as_ctype_ref(&self) -> &Self::CType;
    fn as_ctype_mut(&mut self) -> &mut Self::CType;
}
pub(crate) trait OwnedCTypeRef: Sized {
    type OwnedCType;
    fn as_owned_c_type_ref(&self) -> &Self::OwnedCType;
    fn as_owned_c_type_mut(&mut self) -> &mut Self::OwnedCType;
}
pub(crate) trait LoanedCTypeRef: Sized {
    type LoanedCType;
    fn as_loaned_c_type_ref(&self) -> &Self::LoanedCType;
    fn as_loaned_c_type_mut(&mut self) -> &mut Self::LoanedCType;
}
pub(crate) trait ViewCTypeRef: Sized {
    type ViewCType;
    fn as_view_c_type_ref(&self) -> &Self::ViewCType;
    fn as_view_c_type_mut(&mut self) -> &mut Self::ViewCType;
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
pub(crate) trait IntoRustType: Sized {
    type RustType;
    fn into_rust_type(self) -> Self::RustType;
}
pub(crate) trait TakeRustType: IntoRustType + Default {
    fn take_rust_type(&mut self) -> Self::RustType {
        std::mem::take(self).into_rust_type()
    }
}
pub(crate) trait IntoCType: Sized {
    type CType;
    fn into_c_type(self) -> Self::CType;
}

macro_rules! validate_equivalence {
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
    (as_moved ($rust_type:ty, $c_type:ty)) => {
    };

    (as_c ($rust_type:ty, $c_type:ty)) => {
        impl $crate::transmute::CTypeRef for $rust_type {
            type CType = $c_type;
            fn as_ctype_ref(&self) -> &Self::CType {
                unsafe { &*(self as *const Self as *const Self::CType) }
            }
            fn as_ctype_mut(&mut self) -> &mut Self::CType {
                unsafe { &mut *(self as *mut Self as *mut Self::CType) }
            }
        }
    };
    (as_c_owned ($rust_type:ty, $c_type:ty)) => {
        impl $crate::transmute::OwnedCTypeRef for $rust_type {
            type OwnedCType = $c_type;
            fn as_owned_c_type_ref(&self) -> &Self::OwnedCType {
                unsafe { &*(self as *const Self as *const Self::OwnedCType) }
            }
            fn as_owned_c_type_mut(&mut self) -> &mut Self::OwnedCType {
                unsafe { &mut *(self as *mut Self as *mut Self::OwnedCType) }
            }
        }
    };

    (as_c_loaned ($rust_type:ty, $c_type:ty)) => {
        impl $crate::transmute::LoanedCTypeRef for $rust_type {
            type LoanedCType = $c_type;
            fn as_loaned_c_type_ref(&self) -> &Self::LoanedCType {
                unsafe { &*(self as *const Self as *const Self::LoanedCType) }
            }
            fn as_loaned_c_type_mut(&mut self) -> &mut Self::LoanedCType {
                unsafe { &mut *(self as *mut Self as *mut Self::LoanedCType) }
            }
        }
    };
    (as_c_view ($rust_type:ty, $c_type:ty)) => {
        impl $crate::transmute::ViewCTypeRef for $rust_type {
            type ViewCType = $c_type;
            fn as_view_c_type_ref(&self) -> &Self::ViewCType {
                unsafe { &*(self as *const Self as *const Self::ViewCType) }
            }
            fn as_view_c_type_mut(&mut self) -> &mut Self::ViewCType {
                unsafe { &mut *(self as *mut Self as *mut Self::ViewCType) }
            }
        }
    };
    (as_rust ($c_type:ty, $rust_type:ty)) => {
        impl $crate::transmute::RustTypeRef for $c_type {
            type RustType = $rust_type;
            fn as_rust_type_ref(&self) -> &Self::RustType {
                unsafe { &*(self as *const Self as *const Self::RustType) }
            }
            fn as_rust_type_mut(&mut self) -> &mut Self::RustType {
                unsafe { &mut *(self as *mut Self as *mut Self::RustType) }
            }
        }
        impl $crate::transmute::RustTypeRefUninit for std::mem::MaybeUninit<$c_type> {
            type RustType = $rust_type;
            fn as_rust_type_mut_uninit(&mut self) -> &mut std::mem::MaybeUninit<Self::RustType> {
                unsafe {
                    let this = self as *mut std::mem::MaybeUninit<$c_type>;
                    &mut *(this as *mut std::mem::MaybeUninit<Self::RustType>)
                }
            }
        }
    };
    (into_rust ($c_type:ty, $rust_type:ty)) => {
        impl $crate::transmute::IntoRustType for $c_type {
            type RustType = $rust_type;
            fn into_rust_type(self) -> Self::RustType {
                unsafe { std::mem::transmute::<$c_type, $rust_type>(self) }
            }
        }
    };
    (take_rust ($c_type:ty, $rust_type:ty)) => {
        impl Default for $c_type {
            fn default() -> Self {
                unsafe { std::mem::transmute::<$rust_type, $c_type>(<$rust_type>::default()) }
            }
        }
        impl $crate::transmute::TakeRustType for $c_type {}
    };
    (into_c ($rust_type:ty, $c_type:ty)) => {
        impl $crate::transmute::IntoCType for $rust_type {
            type CType = $c_type;
            fn into_c_type(self) -> Self::CType {
                unsafe { std::mem::transmute::<$rust_type, $c_type>(self) }
            }
        }
    };
}

macro_rules! impl_owned {
    (owned $c_owned_type:ty, moved $c_moved_type:ty, moved2 $c_moved_type2:ty, inner rust option $rust_inner_type:ty) => {
        impl_transmute!(as_c_owned(Option<$rust_inner_type>, $c_owned_type));
        impl_transmute!(as_rust($c_owned_type, Option<$rust_inner_type>));
        impl_transmute!(into_rust($c_owned_type, Option<$rust_inner_type>));
        impl_transmute!(take_rust($c_owned_type, Option<$rust_inner_type>));

        impl $crate::transmute::IntoRustType for $c_moved_type {
            type RustType = Option<$rust_inner_type>;
            fn into_rust_type(self) -> Self::RustType {
                use $crate::transmute::RustTypeRef;
                let mut this = self;
                // expicit types for better understanding
                let ptr: &mut Option<&mut Option<$rust_inner_type>> =
                    &mut this._ptr.as_mut().map(|r| r.as_rust_type_mut());
                let res: Option<$rust_inner_type> =
                    ptr.as_mut().map(|r| std::mem::take(*r)).flatten();
                res
            }
        }

        impl $crate::transmute::TakeRustType for $c_moved_type {}
        impl Drop for $c_moved_type {
            fn drop(&mut self) {
                self.take();
            }
        }
    };

    (owned $c_owned_type:ty, moved $c_moved_type:ty, inner rust option $rust_inner_type:ty) => {
        impl_transmute!(as_c_owned(Option<$rust_inner_type>, $c_owned_type));
        impl_transmute!(as_rust($c_owned_type, Option<$rust_inner_type>));
        impl_transmute!(into_rust($c_owned_type, Option<$rust_inner_type>));
        impl_transmute!(take_rust($c_owned_type, Option<$rust_inner_type>));

        impl $crate::transmute::IntoRustType for $c_moved_type {
            type RustType = Option<$rust_inner_type>;
            fn into_rust_type(self) -> Self::RustType {
                use $crate::transmute::RustTypeRef;
                let mut this = self;
                // expicit types for better understanding
                let ptr: &mut Option<&mut Option<$rust_inner_type>> =
                    &mut this._ptr.as_mut().map(|r| r.as_rust_type_mut());
                let res: Option<$rust_inner_type> =
                    ptr.as_mut().map(|r| std::mem::take(*r)).flatten();
                res
            }
        }
        impl $crate::transmute::TakeRustType for $c_moved_type {}
        impl Drop for $c_moved_type {
            fn drop(&mut self) {
                self.take();
            }
        }
    };
    (owned $c_owned_type:ty, moved $c_moved_type:ty, inner rust $rust_owned_type:ty) => {
        impl_transmute!(as_c_owned($rust_owned_type, $c_owned_type));
        impl_transmute!(as_rust($c_owned_type, $rust_owned_type));
        impl_transmute!(into_rust($c_owned_type, $rust_owned_type));
        impl_transmute!(take_rust($c_owned_type, $rust_owned_type));

        impl $crate::transmute::IntoRustType for $c_moved_type {
            type RustType = Option<$rust_owned_type>;
            fn into_rust_type(self) -> Self::RustType {
                use $crate::transmute::RustTypeRef;
                let mut this = self;
                this._ptr
                    .as_mut()
                    .map(|r| std::mem::take(r.as_rust_type_mut()))
            }
        }
        impl $crate::transmute::TakeRustType for $c_moved_type {}
        impl Drop for $c_moved_type {
            fn drop(&mut self) {
                self.take();
            }
        }
    };
    (owned rust $c_owned_type:ty, moved $c_moved_type:ty, loaned $c_loaned_type:ty) => {
        impl_transmute!(as_c_owned($c_loaned_type, $c_owned_type));
        impl_transmute!(as_c_loaned($c_owned_type, $c_loaned_type));

        impl $crate::transmute::IntoRustType for $c_moved_type {
            type RustType = Option<$c_owned_type>;
            fn into_rust_type(self) -> Self::RustType {
                let mut this = self;
                this._ptr.as_mut().map(|r| std::mem::take(*r))
            }
        }
        impl Drop for $c_moved_type {
            fn drop(&mut self) {
                std::mem::take(&mut self._ptr);
            }
        }
    };
}

// There are several possible variants how owned/loaned/moved types are implememnted
// Here is the relation between them:
//
// - "Owned" type is a type with "empty" state.
//    - It's guaranteed that in the "empty" state object doesn't hold any external resources (memory, socket, file, etc) and can be safely forgotten without explicit destruction.
//    - The "empty" state itself may be valid or not valid state of an object. E.g. it can be "None" value of an Option type or just default value of a struct.
//    - The "empty" statis a always a "default()" value of an owned type.
//
// - "Inner" type is a type which is contained in the "Owned" type and whcih is necessarily valid. E.g. if the owned type is Option<T> then inner type is T.
//   If the owned type is just T with default value then inner type is T itself. It's ususally the same as "Loaned" type but not always
//
// - "Loaned: type is a type which is used to access the owned type. Usually it's the same as wrapped type, but sometimes (e.g. for type `ZShm`) it's an wrapper type over
//   the inside type of loaned type. E.g. owned type is `Option<ZShm>`, inner type is then `Zshm``, but loaned type is `zshm`
//   (which is just wrapper over `ZShm`` restricting write access)
//
// - "Moved" type - repr "C" structure which contains optional pointer to the owned type. It's used to explictly transfer ownership in C code.
//   In the rust code it's possible only to take "Inner" type from "Moved" type, taking ownership on it.
//
// - "View" type - the type which holds references to external data but doesn't own it. Therefore it's always safe to copy/forget it without explicit destructor.
//   The view type correspods to owned type. E.g. there may be "onwned string" and "view string". View type can be converted to loaned type, same as loaned type of
//   corresponding owned type. This allows to accept either owned data or external data in the same function.
//
// Typically all these types are the same size and alignment and can be converted to each other just with `reinterpret_cast` in C++.
// But this is necessary for C++ only, if type is not used by C++ binding, this restriction can be relaxed.
//

// These macros declares conversions between Rust and C types.
// Typically the "owned" and "loaned" types have the same size and alignment.
// This is necessary for C++ wrapper library to work correctly.
// But for some types which are not covered by C++ this restriction can be relaxed.
// In this case the "inequal" variant should be used.

#[macro_export]
macro_rules! decl_c_type_inequal {
    //
    // Owned with with explicit rust loaned type - rarely used
    //
    (owned ($c_owned_type:ty, option $rust_inner_type:ty $(,)?),
     loaned ($c_loaned_type:ty, $rust_loaned_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type!(
            owned($c_owned_type, option $rust_inner_type),
            moved($c_moved_type)
        );
        validate_equivalence!($c_loaned_type, $rust_loaned_type);
        impl_transmute!(as_c_loaned($rust_loaned_type, $c_loaned_type));
        impl_transmute!(as_rust($c_loaned_type, $rust_loaned_type));
    };
    (owned ($c_owned_type:ty, $rust_owned_type:ty $(,)?),
     loaned ($c_loaned_type:ty, $rust_loaned_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type!(
            owned($c_owned_type, $rust_owned_type),
            moved($c_moved_type)
        );
        validate_equivalence!($c_loaned_type, $rust_loaned_type);
        impl_transmute!(as_c_loaned($rust_loaned_type, $c_loaned_type));
        impl_transmute!(as_rust($c_loaned_type, $rust_loaned_type));
    };

    //
    // Owned with loaned type same as inner type - typical case
    //
    (owned ($c_owned_type:ty, option $rust_inner_type:ty $(,)?),
     loaned ($c_loaned_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type_inequal!(
            owned($c_owned_type, option $rust_inner_type),
            loaned($c_loaned_type, $rust_inner_type),
            moved($c_moved_type)
        );
    };
    (owned ($c_owned_type:ty, $rust_owned_type:ty $(,)?),
     loaned ($c_loaned_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type_inequal!(
            owned($c_owned_type, $rust_owned_type),
            loaned($c_loaned_type, $rust_owned_type),
            moved($c_moved_type)
        );
    };
    //
    // With view type
    //
    (owned ($c_owned_type:ty, option $rust_inner_type:ty $(,)?),
     loaned ($c_loaned_type:ty $(,)?),
     view ($c_view_type:ty, $rust_view_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type_inequal!(
            owned($c_owned_type, option $rust_inner_type),
            loaned($c_loaned_type),
            moved($c_moved_type)
        );
        validate_equivalence!($c_view_type, $rust_view_type);
        impl_transmute!(as_c_view($rust_view_type, $c_view_type));
        impl_transmute!(as_rust($c_view_type, $rust_view_type));
    };
    (owned ($c_owned_type:ty, $rust_owned_type:ty $(,)?),
     loaned ($c_loaned_type:ty $(,)?),
     view ($c_view_type:ty, $rust_view_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type_inequal!(
            owned($c_owned_type, $rust_owned_type),
            loaned($c_loaned_type),
            moved($c_moved_type)
        );
        validate_equivalence!($c_view_type, $rust_view_type);
        impl_transmute!(as_c_view($rust_view_type, $c_view_type));
        impl_transmute!(as_rust($c_view_type, $rust_view_type));
    };
    (owned ($c_owned_type:ty, $rust_owned_type:ty $(,)?),
     loaned ($c_loaned_type:ty, $rust_loaned_type:ty $(,)?),
     view ($c_view_type:ty, $rust_view_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type_inequal!(
            owned($c_owned_type, $rust_owned_type),
            loaned($c_loaned_type, $rust_loaned_type),
            moved($c_moved_type)
        );
        validate_equivalence!($c_view_type, $rust_view_type);
        impl_transmute!(as_c_view($rust_view_type, $c_view_type));
        impl_transmute!(as_rust($c_view_type, $rust_view_type));
    };
}

#[macro_export]
macro_rules! decl_c_type {
    //
    // Owned type only
    //
    (owned ($c_owned_type:ty, option $rust_inner_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        validate_equivalence!($c_owned_type, Option<$rust_inner_type>);
        impl_owned!(owned $c_owned_type, moved $c_moved_type, inner rust option $rust_inner_type);
    };
    (owned ($c_owned_type:ty, option $rust_inner_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?),
     moved2 ($c_moved_type2:ty $(,)?)
     $(,)?) => {
        validate_equivalence!($c_owned_type, Option<$rust_inner_type>);
        impl_owned!(owned $c_owned_type, moved $c_moved_type, moved2 $c_moved_type2, inner rust option $rust_inner_type);
    };

    (owned ($c_owned_type:ty, $rust_owned_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        validate_equivalence!($c_owned_type, $rust_owned_type);
        impl_owned!(owned $c_owned_type, moved $c_moved_type, inner rust $rust_owned_type);
    };
    //
    // Owned with with explicit rust loaned type - rarely used
    //
    (owned ($c_owned_type:ty, option $rust_inner_type:ty $(,)?),
     loaned ($c_loaned_type:ty, $rust_loaned_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type_inequal!(
            owned($c_owned_type, option $rust_inner_type),
            loaned($c_loaned_type, $rust_loaned_type),
            moved($c_moved_type)
        );
        validate_equivalence!($c_loaned_type, $c_owned_type);
    };
    (owned ($c_owned_type:ty, $rust_owned_type:ty $(,)?),
     loaned ($c_loaned_type:ty, $rust_loaned_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type_inequal!(
            owned($c_owned_type, $rust_owned_type),
            loaned($c_loaned_type, $rust_loaned_type),
            moved($c_moved_type)
        );
        validate_equivalence!($c_loaned_type, $c_owned_type);
    };
    //
    // Owned with loaned type same as inner type - typical case
    //
    (owned ($c_owned_type:ty, option $rust_inner_type:ty $(,)?),
     loaned ($c_loaned_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type!(
            owned($c_owned_type, option $rust_inner_type),
            loaned($c_loaned_type, $rust_inner_type),
            moved($c_moved_type)
        );
    };
    (owned ($c_owned_type:ty, $rust_owned_type:ty $(,)?),
     loaned ($c_loaned_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type!(
            owned($c_owned_type, $rust_owned_type),
            loaned($c_loaned_type, $rust_owned_type),
            moved($c_moved_type)
        );
    };
    //
    // With view type
    //
    (owned ($c_owned_type:ty, option $rust_inner_type:ty $(,)?),
     loaned ($c_loaned_type:ty $(,)?),
     view ($c_view_type:ty, $rust_view_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type_inequal!(
            owned($c_owned_type, option $rust_inner_type),
            loaned($c_loaned_type),
            view($c_view_type, $rust_view_type),
            moved($c_moved_type)
        );
        validate_equivalence!($c_owned_type, $c_loaned_type);
        validate_equivalence!($c_view_type, $c_loaned_type);
    };
    (owned ($c_owned_type:ty, $rust_owned_type:ty $(,)?),
     loaned ($c_loaned_type:ty $(,)?),
     view ($c_view_type:ty, $rust_view_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type_inequal!(
            owned($c_owned_type, $rust_owned_type),
            loaned($c_loaned_type, $rust_owned_type),
            view($c_view_type, $rust_view_type),
            moved($c_moved_type)
        );
        validate_equivalence!($c_owned_type, $c_loaned_type);
        validate_equivalence!($c_view_type, $c_loaned_type);
    };
    (owned ($c_owned_type:ty, $rust_owned_type:ty $(,)?),
     loaned ($c_loaned_type:ty, $rust_loaned_type:ty $(,)?),
     view ($c_view_type:ty, $rust_view_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        decl_c_type_inequal!(
            owned($c_owned_type, $rust_owned_type),
            loaned($c_loaned_type, $rust_loaned_type),
            view($c_view_type, $rust_view_type),
            moved($c_moved_type)
        );
        validate_equivalence!($c_owned_type, $c_loaned_type);
        validate_equivalence!($c_view_type, $c_loaned_type);
    };

    //
    // Specific case for closures: c owned type and rust owned type is the same thing: c-repr structure
    //
    (owned ($c_owned_type:ty $(,)?),
     loaned ($c_loaned_type:ty $(,)?),
     moved ($c_moved_type:ty $(,)?)
     $(,)?) => {
        validate_equivalence!($c_owned_type, $c_loaned_type);
        impl_owned!(owned rust $c_owned_type, moved $c_moved_type, loaned $c_loaned_type);
    };

    //
    // Rust type is copyable plain data type, just allow to convert references or copy whole structure
    //
    (copy ($c_type:ty, $rust_type:ty $(,)?) $(,)?) => {
        validate_equivalence!($c_type, $rust_type);
        impl_transmute!(as_c($rust_type, $c_type));
        impl_transmute!(as_rust($c_type, $rust_type));
        impl_transmute!(into_c($rust_type, $c_type));
        impl_transmute!(into_rust($c_type, $rust_type));
    };

    //
    // Specific case: no owned type exists
    //
    (loaned ($c_loaned_type:ty, $rust_loaned_type:ty $(,)?) $(,)?) => {
        validate_equivalence!($c_loaned_type, $rust_loaned_type);
        impl_transmute!(as_c_loaned($rust_loaned_type, $c_loaned_type));
        impl_transmute!(as_rust($c_loaned_type, $rust_loaned_type));
    };
}
