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

// pub fn as_static_ref<'a, T: 'a>(value: &'a T) -> &'static T {
//     unsafe { std::mem::transmute::<&'a T, &'static T>(value) }
// }

pub fn unwrap_ref_unchecked<T>(value: &Option<T>) -> &T {
    debug_assert!(value.is_some());
    unsafe { value.as_ref().unwrap_unchecked() }
}

pub(crate) trait TransmuteRef<T: Sized>: Sized {
    fn transmute_ref(&self) -> &T;
    fn transmute_mut(&mut self) -> &mut T;
}

pub(crate) trait TransmuteCopy<T: Copy> {
    fn transmute_copy(self) -> T;
}

pub(crate) trait TransmuteUninitPtr<T: Sized>: Sized {
    fn transmute_uninit_ptr(self) -> *mut std::mem::MaybeUninit<T>;
}

pub(crate) trait Inplace: Sized {
    // Initialize the object in place with a memcpy of the provided value. Assumes that the memory passed to the function is uninitialized
    fn init(this: *mut std::mem::MaybeUninit<Self>, value: Self) {
        let this = this as *mut Self;
        unsafe { std::ptr::write(this, value) };
    }

    // Initialize the object in place with an empty value
    fn empty(this: *mut std::mem::MaybeUninit<Self>);

    // Drop the object in place and replaces it with empty value
    fn drop(this: &mut Self) {
        let this = this as *mut Self;
        unsafe { std::ptr::drop_in_place(this) };
        Inplace::empty(this as *mut std::mem::MaybeUninit<Self>);
    }
    // TODO: for effective inplace_init, we can provide a method that takes a closure that initializes the object in place
}

pub(crate) trait InplaceDefault: Default {
    // Default implementation of inplace_init for object implementing Default trait. May be less efficient than a custom implementation
    // because for `empty` operation it performs a copy of the default value from stack to provided memory
    fn default(this: *mut std::mem::MaybeUninit<Self>) {
        let this = this as *mut Self;
        unsafe { std::ptr::write(this, <Self as Default>::default()) };
    }
}

// For types implementing Default, we can use provide default implementation of InplaceInit through InplaceInitDefault
impl<T: InplaceDefault> Inplace for T {
    fn empty(this: *mut std::mem::MaybeUninit<Self>) {
        InplaceDefault::default(this);
    }
}

macro_rules! validate_equivalence {
    ($type_a:ty, $type_b:ty) => {
        const _: () = {
            let align_a = std::mem::align_of::<$type_a>();
            let align_b = std::mem::align_of::<$type_b>();
            if align_a != align_b {
                panic!(
                    "Alingment mismatch: type `{}` has align {}, type `{}` has align {}",
                    stringify!($type_a),
                    align_a,
                    stringify!($type_b),
                    align_b
                );
            }
            let size_a = std::mem::size_of::<$type_a>();
            let size_b = std::mem::size_of::<$type_b>();
            if size_a != size_b {
                panic!(
                    "Size mismatch: type `{}` has size {}, type `{}` has size {}",
                    stringify!($type_a),
                    size_a,
                    stringify!($type_b),
                    size_b
                );
            }
        };
    };
}

#[macro_export]
macro_rules! decl_transmute_owned {
    ($zenoh_type:ty, $c_type:ty) => {
        impl $crate::transmute::InplaceDefault for $zenoh_type {}
        decl_transmute_owned!(custom_inplace_init $zenoh_type, $c_type);

    };
    (custom_inplace_init $zenoh_type:ty, $c_type:ty) => {
        validate_equivalence!($zenoh_type, $c_type);
        impl_transmute_ref!($zenoh_type, $c_type);
        impl_transmute_ref!($c_type, $zenoh_type);
        impl_transmute_uninit_ptr!($zenoh_type, $c_type);
        impl_transmute_uninit_ptr!($c_type, $zenoh_type);
    }
}

#[macro_export]
macro_rules! decl_transmute_copy {
    ($zenoh_type:ty, $c_type:ty) => {
        validate_equivalence!($zenoh_type, $c_type);
        impl_transmute_copy!($zenoh_type, $c_type);
        impl_transmute_copy!($c_type, $zenoh_type);
        impl_transmute_uninit_ptr!($zenoh_type, $c_type);
        impl_transmute_uninit_ptr!($c_type, $zenoh_type);
    };
}

macro_rules! impl_transmute_ref {
    ($src_type:ty, $dst_type:ty) => {
        impl $crate::transmute::TransmuteRef<$dst_type> for $src_type {
            fn transmute_ref(&self) -> &$dst_type {
                unsafe { std::mem::transmute::<&$src_type, &$dst_type>(self) }
            }
            fn transmute_mut(&mut self) -> &mut $dst_type {
                unsafe { std::mem::transmute::<&mut $src_type, &mut $dst_type>(self) }
            }
        }
    };
}

macro_rules! impl_transmute_copy {
    ($src_type:ty, $dst_type:ty) => {
        impl $crate::transmute::TransmuteCopy<$dst_type> for $src_type {
            fn transmute_copy(self) -> $dst_type {
                unsafe { std::mem::transmute::<$src_type, $dst_type>(self) }
            }
        }
    };
}

macro_rules! impl_transmute_uninit_ptr {
    ($src_type:ty, $dst_type:ty) => {
        impl $crate::transmute::TransmuteUninitPtr<$dst_type> for *mut MaybeUninit<$src_type> {
            fn transmute_uninit_ptr(self) -> *mut std::mem::MaybeUninit<$dst_type> {
                self as *mut std::mem::MaybeUninit<$dst_type>
            }
        }
    };
}
