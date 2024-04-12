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

pub(crate) struct StaticRef<T: 'static>(&'static T);
impl<T> StaticRef<T> {
    pub(crate) fn new<'a>(value: &'a T) -> Self {
        StaticRef(unsafe { std::mem::transmute::<&'a T, &'static T>(value) })
    }
}
impl<T> Clone for StaticRef<T> {
    fn clone(&self) -> Self {
        StaticRef(self.0)
    }
}
impl<T> Copy for StaticRef<T> {}

pub(crate) trait TransmuteRef<T> {
    fn transmute_ref(&self) -> &T;
    fn transmute_mut(&mut self) -> &mut T;
}

pub(crate) trait TransmuteValue<T: Copy>: TransmuteRef<T> {
    fn transmute_value(self) -> T;
}

pub(crate) trait Inplace<T: Sized>: Sized {
    // Initialize the object in place with a memcpy of the provided value. Assumes that the memory passed to the function is uninitialized
    fn init(this: *mut std::mem::MaybeUninit<Self>, value: T) {
        let this = this as *mut T;
        unsafe { std::ptr::write(this, value) };
    }
    // Initialize the object in place with an empty value
    fn empty(this: *mut std::mem::MaybeUninit<Self>);
    // TODO: for effective inplace_init, we can provide a method that takes a closure that initializes the object in place
}

pub(crate) trait InplaceDrop<T: Sized>: Sized + Inplace<T> {
    // Drop the object in place and replaces it with default value
    fn drop(this: &mut Self) {
        unsafe { std::ptr::drop_in_place(this as *mut Self) };
        Inplace::empty(this as *mut Self as *mut std::mem::MaybeUninit<Self>);
    }
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
impl<T: InplaceDefault> Inplace<T> for T {
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
macro_rules! decl_transmute_ref {
    (default_inplace_init $zenoh_type:ty, $c_type:ty) => {
        impl InplaceDefault for $zenoh_type {}
        decl_transmute_ref!(custom_inplace_init $zenoh_type, $c_type);

    };
    (custom_inplace_init $zenoh_type:ty, $c_type:ty) => {
        validate_equivalence!($zenoh_type, $c_type);
        impl_transmute_ref!($zenoh_type, $c_type);
        impl_transmute_ref!($c_type, $zenoh_type);
        impl Inplace<$zenoh_type> for $c_type {
            fn init(this: *mut std::mem::MaybeUninit<Self>, value: $zenoh_type) {
                let this = this as *mut std::mem::MaybeUninit<$zenoh_type>;
                Inplace::init(this, value);
            }
            fn empty(this: *mut std::mem::MaybeUninit<Self>) {
                let this = this as *mut std::mem::MaybeUninit<$zenoh_type>;
                Inplace::empty(this);
            }
        }
    }
}

#[macro_export]
macro_rules! decl_transmute_copy {
    ($zenoh_type:ty, $c_type:ty) => {
        validate_equivalence!($zenoh_type, $c_type);
        impl_transmute_ref!($zenoh_type, $c_type);
        impl_transmute_ref!($c_type, $zenoh_type);
        impl_transmute_copy!($zenoh_type, $c_type);
        impl_transmute_copy!($c_type, $zenoh_type);
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
        impl $crate::transmute::TransmuteValue<$dst_type> for $src_type {
            fn transmute_value(self) -> $dst_type {
                unsafe { std::mem::transmute::<$src_type, $dst_type>(self) }
            }
        }
    };
}
