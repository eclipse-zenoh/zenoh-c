use std::{borrow::Cow, cell::UnsafeCell, collections::HashMap};

use libc::c_void;

use crate::{z_bytes_null, z_bytes_t};

/// The body of a loop over an attachment's key-value pairs.
///
/// `key` and `value` are loaned to the body for the duration of a single call.
/// `context` is passed transparently through the iteration driver.
///
/// Returning `0` is treated as `continue`.
/// Returning any other value is treated as `break`.
pub type z_attachement_iter_body_t =
    extern "C" fn(key: z_bytes_t, value: z_bytes_t, context: *mut c_void) -> i8;

/// The driver of a loop over an attachement's key-value pairs.
///
/// This function is expected to call `loop_body` once for each key-value pair
/// within `iterator`, passing `context`, and returning any non-zero value immediately (breaking iteration).
pub type z_attachement_iter_driver_t = extern "C" fn(
    iterator: *mut c_void,
    loop_body: z_attachement_iter_body_t,
    context: *mut c_void,
) -> i8;

/// The v-table for an attachement.
#[repr(C)]
pub struct z_attachement_vtable_t {
    /// See `z_attachement_iteration_driver_t`'s documentation.
    iteration_driver: z_attachement_iter_driver_t,
    /// Returns the number of key-value pairs within the attachement.
    len: extern "C" fn(*const c_void) -> usize,
}

/// A v-table based map of byte slice to byte slice.
///
/// `vtable == NULL` marks the gravestone value, as this type is often optional.
/// Users are encouraged to use `z_attachement_null` and `z_attachement_check` to interact.
#[repr(C)]
pub struct z_attachement_t {
    data: *mut c_void,
    vtable: Option<&'static z_attachement_vtable_t>,
}

/// Returns the gravestone value for `z_attachement_t`.
#[no_mangle]
pub extern "C" fn z_attachement_check(this: &z_attachement_t) -> bool {
    this.vtable.is_some()
}

/// Returns the gravestone value for `z_attachement_t`.
#[no_mangle]
pub extern "C" fn z_attachement_null() -> z_attachement_t {
    z_attachement_t {
        data: core::ptr::null_mut(),
        vtable: None,
    }
}

/// Iterate over `this`'s key-value pairs, breaking if `body` returns a non-zero
/// value for a key-value pair, and returning the latest return value.
///
/// `context` is passed to `body` to allow stateful closures.
///
/// This function takes no ownership whatsoever.
#[no_mangle]
pub extern "C" fn z_attachement_iterate(
    this: z_attachement_t,
    body: z_attachement_iter_body_t,
    context: *mut c_void,
) -> i8 {
    (this.vtable.unwrap().iteration_driver)(this.data, body, context)
}

/// Returns the number of key-value pairs in `this`.
#[no_mangle]
pub extern "C" fn z_attachement_len(this: z_attachement_t) -> usize {
    (this.vtable.unwrap().len)(this.data)
}

/// A map of maybe-owned vector of bytes to owned vector of bytes.
///
/// In Zenoh C, this map is backed by Rust's standard HashMap, with a DoS-resistant hasher
#[repr(C)]
pub struct z_owned_bytes_map_t {
    _0: [u64; 2],
    _1: [usize; 4],
}
impl core::ops::Deref for z_owned_bytes_map_t {
    type Target = Option<UnsafeCell<HashMap<Cow<'static, [u8]>, Cow<'static, [u8]>>>>;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

/// Constructs a new map.
#[no_mangle]
pub extern "C" fn z_bytes_map_new() -> z_owned_bytes_map_t {
    unsafe { core::mem::transmute(Some(HashMap::<Cow<[u8]>, Cow<[u8]>>::new())) }
}

/// Constructs the gravestone value for `z_owned_bytes_map_t`
#[no_mangle]
pub extern "C" fn z_bytes_map_null() -> z_owned_bytes_map_t {
    unsafe { core::mem::transmute(None::<HashMap<Cow<[u8]>, Cow<[u8]>>>) }
}

/// Returns `true` if the map is not in its gravestone state
#[no_mangle]
pub extern "C" fn z_bytes_map_check(this: &z_owned_bytes_map_t) -> bool {
    this.is_some()
}
/// Destroys the map, resetting `this` to its gravestone value.
///
/// This function is double-free safe, passing a pointer to the gravestone value will have no effect.
#[no_mangle]
pub extern "C" fn z_bytes_map_drop(this: &mut z_owned_bytes_map_t) {
    let this = core::mem::replace(this, z_bytes_map_null());
    if z_bytes_map_check(&this) {
        core::mem::drop(unsafe { core::mem::transmute::<_, HashMap<Cow<[u8]>, Cow<[u8]>>>(this) })
    }
}

/// Returns the value associated with `key`, returning a gravestone value if:
/// - `this` or `key` is in gravestone state.
/// - `this` has no value associated to `key`
#[no_mangle]
pub extern "C" fn z_bytes_map_get(this: &z_owned_bytes_map_t, key: z_bytes_t) -> z_bytes_t {
    let (Some(this), Some(key)) = (this.as_ref(), key.as_slice()) else {
        return z_bytes_null();
    };
    if let Some(value) = unsafe { &*this.get() }.get(key) {
        value.as_ref().into()
    } else {
        z_bytes_null()
    }
}

/// Associates `value` to `key` in the map, copying them to obtain ownership: `key` and `value` are not aliased past the function's return.
///
/// Calling this with `NULL` or the gravestone value is undefined behaviour.
#[no_mangle]
pub extern "C" fn z_bytes_map_insert_by_copy(
    this: &z_owned_bytes_map_t,
    key: z_bytes_t,
    value: z_bytes_t,
) {
    if let (Some(this), Some(key), Some(value)) = (this.as_ref(), key.as_slice(), value.as_slice())
    {
        unsafe { &mut *this.get() }
            .insert(Cow::Owned(key.to_owned()), Cow::Owned(value.to_owned()));
    }
}

/// Associates `value` to `key` in the map, aliasing them.
///
/// Note that once `key` is aliased, reinserting at the same key may alias the previous instance, or the new instance of `key`.
///
/// Calling this with `NULL` or the gravestone value is undefined behaviour.
#[no_mangle]
pub extern "C" fn z_bytes_map_insert_by_alias(
    this: &z_owned_bytes_map_t,
    key: z_bytes_t,
    value: z_bytes_t,
) {
    if let (Some(this), Some(key), Some(value)) = (this.as_ref(), key.as_slice(), value.as_slice())
    {
        unsafe {
            (*this.get()).insert(
                Cow::Borrowed(core::mem::transmute(key)),
                Cow::Borrowed(core::mem::transmute(value)),
            )
        };
    }
}

/// Returns the number of key-value pairs in the map.
///
/// Calling this with `NULL` or the gravestone value is undefined behaviour.
#[no_mangle]
extern "C" fn z_bytes_map_len(this: &z_owned_bytes_map_t) -> usize {
    this.as_ref()
        .map_or(0, |this| unsafe { &*this.get() }.len())
}

/// Iterates over the key-value pairs in the map.
///
/// `body` will be called once per pair, with `ctx` as its last argument.
/// If `body` returns a non-zero value, the iteration will stop immediately and the value will be returned.
/// Otherwise, this will return 0 once all pairs have been visited.
/// `body` is not given ownership of the key nor value, which alias the pairs in the map.
/// It is safe to keep these aliases until existing keys are modified/removed, or the map is destroyed.
/// Note that this map is unordered.
///
/// Calling this with `NULL` or the gravestone value is undefined behaviour.
#[no_mangle]
pub extern "C" fn z_bytes_map_iter(
    this: &z_owned_bytes_map_t,
    body: z_attachement_iter_body_t,
    ctx: *mut c_void,
) -> i8 {
    if let Some(this) = this.as_ref() {
        for (key, value) in unsafe { &*this.get() }.iter() {
            let result = body(key.as_ref().into(), value.as_ref().into(), ctx);
            if result != 0 {
                return result;
            }
        }
    }
    0
}

const Z_BYTES_MAP_VTABLE: z_attachement_vtable_t = z_attachement_vtable_t {
    len: unsafe { core::mem::transmute(z_bytes_map_len as extern "C" fn(_) -> usize) },
    iteration_driver: unsafe {
        core::mem::transmute(z_bytes_map_iter as extern "C" fn(_, _, _) -> i8)
    },
};

/// Aliases `this` into a generic `z_attachement_t`, allowing it to be passed to corresponding APIs.
#[no_mangle]
pub extern "C" fn z_bytes_map_as_attachement(this: &z_owned_bytes_map_t) -> z_attachement_t {
    if z_bytes_map_check(this) {
        z_attachement_t {
            data: this as *const z_owned_bytes_map_t as *mut _,
            vtable: Some(&Z_BYTES_MAP_VTABLE),
        }
    } else {
        z_attachement_t {
            data: core::ptr::null_mut(),
            vtable: None,
        }
    }
}

extern "C" fn bytes_map_from_attachement_iterator(
    key: z_bytes_t,
    value: z_bytes_t,
    ctx: *mut c_void,
) -> i8 {
    let map = unsafe { &*ctx.cast::<z_owned_bytes_map_t>() };
    z_bytes_map_insert_by_copy(map, key, value);
    0
}
extern "C" fn bytes_map_from_attachement_iterator_by_alias(
    key: z_bytes_t,
    value: z_bytes_t,
    ctx: *mut c_void,
) -> i8 {
    let map = unsafe { &*ctx.cast::<z_owned_bytes_map_t>() };
    z_bytes_map_insert_by_alias(map, key, value);
    0
}

/// Constructs a map from the provided attachement, copying keys and values.
///
/// If `this` is at gravestone value, the returned value will also be at gravestone value.
#[no_mangle]
pub extern "C" fn z_bytes_map_from_attachement(this: z_attachement_t) -> z_owned_bytes_map_t {
    if z_attachement_check(&this) {
        let mut map = z_bytes_map_new();
        z_attachement_iterate(
            this,
            bytes_map_from_attachement_iterator,
            &mut map as *mut _ as *mut _,
        );
        map
    } else {
        z_bytes_map_null()
    }
}

/// Constructs a map from the provided attachement, aliasing the attachement's keys and values.
///
/// If `this` is at gravestone value, the returned value will also be at gravestone value.
#[no_mangle]
pub extern "C" fn z_bytes_map_from_attachement_aliasing(
    this: z_attachement_t,
) -> z_owned_bytes_map_t {
    if z_attachement_check(&this) {
        let mut map = z_bytes_map_new();
        z_attachement_iterate(
            this,
            bytes_map_from_attachement_iterator_by_alias,
            &mut map as *mut _ as *mut _,
        );
        map
    } else {
        z_bytes_map_null()
    }
}
