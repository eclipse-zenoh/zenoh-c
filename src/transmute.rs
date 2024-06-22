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

use std::mem::MaybeUninit;

pub fn unwrap_ref_unchecked<T>(value: &Option<T>) -> &T {
    debug_assert!(value.is_some());
    unsafe { value.as_ref().unwrap_unchecked() }
}

pub fn unwrap_ref_unchecked_mut<T>(value: &mut Option<T>) -> &mut T {
    debug_assert!(value.is_some());
    unsafe { value.as_mut().unwrap_unchecked() }
}

pub(crate) trait TransmuteRef: Sized {
    type DST: Sized;
    fn transmute_ref(&self) -> &Self::DST;
    fn transmute_mut(&mut self) -> &mut Self::DST;
    fn transmute_uninit(
        this: &mut std::mem::MaybeUninit<Self>,
    ) -> &mut std::mem::MaybeUninit<Self::DST> {
        unsafe {
            let this = this.assume_init_mut();
            let this = this.transmute_mut();
            std::mem::transmute(this)
        }
    }
}

pub(crate) trait TransmuteFromHandle<T: Sized>: Sized {
    fn transmute_ref(&self) -> &'static T;
    fn transmute_mut(&mut self) -> &'static mut T;
}

pub(crate) trait TransmuteIntoHandle<T: Sized>: Sized {
    fn transmute_handle(&self) -> &'static T;
    fn transmute_handle_mut(&mut self) -> &'static mut T;
}

pub(crate) trait TransmuteCopy<T: Copy> {
    fn transmute_copy(self) -> T;
}

pub(crate) trait TransmuteUninitPtr<T: Sized>: Sized {
    fn transmute_uninit_ptr(self) -> *mut std::mem::MaybeUninit<T>;
}

pub(crate) trait InplaceClear: Sized {
    // Initialize the object in place with some "empty" state which doesn't require dropping. Drop previous value if needed
    fn inplace_clear(&mut self) {
        let this = self as *mut Self;
        unsafe { std::ptr::drop_in_place(this) };
        let this = this as *mut std::mem::MaybeUninit<Self>;
        let this = unsafe { &mut *this };
        Self::inplace_clear_uninit(this);
    }
    // Initialize the memory with some "empty" object state which doesn't require dropping
    fn inplace_clear_uninit(this: &mut std::mem::MaybeUninit<Self>) -> &mut Self;
}

// Default implementation of `InplaceClear` trait for object implementing Default trait. May be less efficient than a custom implementation
// because for `empty` operation it performs a copy of the default value from stack to provided memory
pub(crate) trait InplaceClearDefault: Default {
    fn inplace_clear_with_default(&mut self) {
        *self = <Self as Default>::default();
    }
    fn inplace_clear_uninit_with_default(this: &mut std::mem::MaybeUninit<Self>) -> &mut Self {
        this.write(<Self as Default>::default())
    }
}

// For types implementing Default, we can use provide default implementation of `InplaceClear`
// This should be done explicitly by implementing `InplaceClearDefault` trait. It's also possible to provide a custom implementation
// of `InplaceClear` trait for more efficient inplace initialization if needed
impl<T: InplaceClearDefault> InplaceClear for T {
    fn inplace_clear(&mut self) {
        InplaceClearDefault::inplace_clear_with_default(self);
    }
    fn inplace_clear_uninit(this: &mut std::mem::MaybeUninit<Self>) -> &mut Self {
        InplaceClearDefault::inplace_clear_uninit_with_default(this)
    }
}

pub(crate) trait CTypeUninit: Sized + TransmuteRef<DST = Self::RustType> {
    type RustType: Sized + InplaceClear + TransmuteRef<DST = Self>;
    // Initialize the object in place with a memcpy of the provided value. Assumes that the memory passed to the function is uninitialized
    fn init(this: &mut std::mem::MaybeUninit<Self>, value: Self::RustType) -> &mut Self::RustType {
        let this = this.write(*value.transmute_ref());
        std::mem::forget(value);
        this.transmute_mut()
    }

    // Initialize the object in place with an empty value
    fn empty(this: &mut std::mem::MaybeUninit<Self>) -> &mut Self::RustType {
        let this = TransmuteRef::transmute_uninit(this);
        InplaceClear::inplace_clear_uninit(this)
    }
    // TODO: for effective inplace_init, we can provide a method that takes a closure that initializes the object in place
}

pub(crate) trait CTypeInit: Sized + TransmuteRef<DST = Self::RustType> {
    type RustType: Sized + TransmuteRef<DST = Self> + InplaceClear;
    // Move the object out of this, leaving it in empty state
    fn extract(&mut self) -> Self {
        let mut out: MaybeUninit<Self::RustType> = MaybeUninit::uninit();
        let out = InplaceClear::inplace_clear_uninit(&mut out);
        let this = self.transmute_mut();
        // std::mem::swap(&mut out, self.transmute_mut());
        // out
        unimplemented!("extract")
    }
    // TODO: for effective inplace_init, we can provide a method that takes a closure that initializes the object in place
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

    // Move the object out of this, leaving it in empty state
    fn extract(&mut self) -> Self {
        let mut out: MaybeUninit<Self> = MaybeUninit::uninit();
        Self::empty(&mut out);
        let mut out = unsafe { out.assume_init() };
        std::mem::swap(&mut out, self);
        out
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
        impl_transmute_ref!($zenoh_type, $c_type);
        impl_transmute_ref!($c_type, $zenoh_type);
        impl_transmute_uninit_ptr!($zenoh_type, $c_type);
        impl_transmute_uninit_ptr!($c_type, $zenoh_type);
    };
}

#[macro_export]
macro_rules! decl_transmute_handle {
    ($zenoh_type:ty, $c_type:ty) => {
        validate_equivalence!($zenoh_type, $c_type);
        impl_transmute_handle!($c_type, $zenoh_type);
    };
}

macro_rules! impl_transmute_ref {
    ($src_type:ty, $dst_type:ty) => {
        impl $crate::transmute::TransmuteRef for $src_type {
            type DST = $dst_type;
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

macro_rules! impl_transmute_handle {
    ($c_type:ty, $zenoh_type:ty) => {
        impl $crate::transmute::TransmuteFromHandle<$zenoh_type> for $c_type {
            fn transmute_ref(&self) -> &'static $zenoh_type {
                unsafe {
                    (self as *const Self as *const $zenoh_type)
                        .as_ref()
                        .unwrap()
                }
            }
            fn transmute_mut(&mut self) -> &'static mut $zenoh_type {
                unsafe { (self as *mut Self as *mut $zenoh_type).as_mut().unwrap() }
            }
        }
        impl $crate::transmute::TransmuteIntoHandle<$c_type> for $zenoh_type {
            fn transmute_handle(&self) -> &'static $c_type {
                unsafe { (self as *const Self as *const $c_type).as_ref().unwrap() }
            }
            fn transmute_handle_mut(&mut self) -> &'static mut $c_type {
                unsafe { (self as *mut Self as *mut $c_type).as_mut().unwrap() }
            }
        }
    };
}
