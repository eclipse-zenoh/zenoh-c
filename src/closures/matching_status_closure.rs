use libc::c_void;

use crate::zcu_matching_status_t;
/// A closure is a structure that contains all the elements for stateful, memory-leak-free callbacks:
///
/// Members:
///   void *context: a pointer to an arbitrary state.
///   void *call(const struct z_owned_reply_t*, const void *context): the typical callback function. `context` will be passed as its last argument.
///   void *drop(void*): allows the callback's state to be freed.
///
/// Closures are not guaranteed not to be called concurrently.
///
/// It is guaranteed that:
///
///   - `call` will never be called once `drop` has started.
///   - `drop` will only be called **once**, and **after every** `call` has ended.
///   - The two previous guarantees imply that `call` and `drop` are never called concurrently.
#[repr(C)]
pub struct zcu_owned_closure_matching_status_t {
    context: *mut c_void,
    call: Option<extern "C" fn(&zcu_matching_status_t, *mut c_void)>,
    drop: Option<extern "C" fn(*mut c_void)>,
}

impl zcu_owned_closure_matching_status_t {
    pub fn empty() -> Self {
        zcu_owned_closure_matching_status_t {
            context: std::ptr::null_mut(),
            call: None,
            drop: None,
        }
    }
}
unsafe impl Send for zcu_owned_closure_matching_status_t {}
unsafe impl Sync for zcu_owned_closure_matching_status_t {}
impl Drop for zcu_owned_closure_matching_status_t {
    fn drop(&mut self) {
        if let Some(drop) = self.drop {
            drop(self.context)
        }
    }
}
/// Constructs a null safe-to-drop value of 'zcu_owned_closure_matching_status_t' type
#[no_mangle]
pub extern "C" fn zcu_closure_matching_status_null() -> zcu_owned_closure_matching_status_t {
    zcu_owned_closure_matching_status_t::empty()
}
/// Calls the closure. Calling an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn zcu_closure_matching_status_call(
    closure: &zcu_owned_closure_matching_status_t,
    sample: &zcu_matching_status_t,
) {
    match closure.call {
        Some(call) => call(sample, closure.context),
        None => {
            log::error!("Attempted to call an uninitialized closure!");
        }
    }
}
/// Drops the closure. Droping an uninitialized closure is a no-op.
#[no_mangle]
pub extern "C" fn zcu_closure_matching_status_drop(
    closure: &mut zcu_owned_closure_matching_status_t,
) {
    let mut empty_closure = zcu_owned_closure_matching_status_t::empty();
    std::mem::swap(&mut empty_closure, closure);
}
impl<F: Fn(&zcu_matching_status_t)> From<F> for zcu_owned_closure_matching_status_t {
    fn from(f: F) -> Self {
        let this = Box::into_raw(Box::new(f)) as _;
        extern "C" fn call<F: Fn(&zcu_matching_status_t)>(
            response: &zcu_matching_status_t,
            this: *mut c_void,
        ) {
            let this = unsafe { &*(this as *const F) };
            this(response)
        }
        extern "C" fn drop<F>(this: *mut c_void) {
            std::mem::drop(unsafe { Box::from_raw(this as *mut F) })
        }
        zcu_owned_closure_matching_status_t {
            context: this,
            call: Some(call::<F>),
            drop: Some(drop::<F>),
        }
    }
}
