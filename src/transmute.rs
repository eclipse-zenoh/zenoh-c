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

pub(crate) trait TransmuteRef<T> {
    fn transmute_ref(&self) -> &T;
    fn transmute_mut(&mut self) -> &mut T;
}

pub(crate) trait TransmuteCopy<T: Copy>: TransmuteRef<T> {
    fn transmute(self) -> T;
}


pub(crate) trait InplaceInit<T: Sized>: Sized {
    // Initialize the object in place with a memcpy of the provided value. Assumes that the memory passed to the function is uninitialized
    fn inplace_init(&mut self, value: T) -> &mut Self {
        unsafe { std::ptr::write(self as *mut Self as *mut T, value) };
        self
    }
    // Initialize the object in place with a memcpy of the provided value. Assumes that the memory passed to the function is uninitialized
    fn inplace_default(&mut self) -> &mut Self
    where
        T: Default,
    {
        unsafe { std::ptr::write(self as *mut Self as *mut T, T::default()) };
        self
    }
}

pub(crate) trait InplaceInitDefault: Default {
    // Default implementation of inplace_init for object implementing Default trait. May be less efficient than a custom implementation
    // because it involves a copy of the default value.
    fn inplace_default_impl(&mut self) {
        unsafe { std::ptr::write(self as *mut Self, Self::default()) };
    }
}

// For types implementing Default, we can use provide default implementation of InplaceInit through InplaceInitDefault
impl<T: InplaceInitDefault> InplaceInit<T> for T {
    fn inplace_default(&mut self) -> &mut Self {
        self.inplace_default_impl();
        self
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
        impl InplaceInitDefault for $zenoh_type {}
        decl_transmute_ref!(custom_inplace_init $zenoh_type, $c_type);

    };
    (custom_inplace_init $zenoh_type:ty, $c_type:ty) => {
        validate_equivalence!($zenoh_type, $c_type);
        impl_transmute_ref!($zenoh_type, $c_type);
        impl_transmute_ref!($c_type, $zenoh_type);
        impl InplaceInit<$zenoh_type> for $c_type {
            fn inplace_init(&mut self, value: $zenoh_type) {
                self.transmute_mut().inplace_init(value);
            }
            fn inplace_default(&mut self) {
                self.transmute_mut().inplace_default();
            }
        }
    }
}

#[macro_export]
macro_rules! decl_transmute_copy {
    ($zenoh_type:ty, $c_type:ty) => {
        validate_equivalence!($zenoh_type, $c_type);
        impl_guarded_transmute_ref!($zenoh_type, $c_type);
        impl_guarded_transmute_ref!($c_type, $zenoh_type);
        impl_guarded_transmute_copy!($zenoh_type, $c_type);
        impl_guarded_transmute_copy!($c_type, $zenoh_type);
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
    }
}

macro_rules! impl_transmute_copy {
    ($src_type:ty, $dst_type:ty) => {
        impl $crate::transmute::TransmuteCopy<$dst_type> for $src_type {
            fn transmute(self) -> $dst_type {
                unsafe { std::mem::transmute::<$src_type, $dst_type>(self) }
            }
        }
    }
}
