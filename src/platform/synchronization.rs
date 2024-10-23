use std::{
    mem::MaybeUninit,
    sync::{Condvar, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

use libc::c_void;

pub use crate::opaque_types::{z_loaned_mutex_t, z_moved_mutex_t, z_owned_mutex_t};
use crate::{
    result,
    transmute::{
        LoanedCTypeMut, LoanedCTypeRef, RustTypeMut, RustTypeMutUninit, RustTypeRef, TakeRustType,
    },
};

decl_c_type!(
    owned(z_owned_mutex_t, option(Mutex<()>, Option<MutexGuard<'static, ()>>)),
    loaned(z_loaned_mutex_t),
);

/// Constructs a mutex.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_mutex_init(this_: &mut MaybeUninit<z_owned_mutex_t>) -> result::z_result_t {
    this_.as_rust_type_mut_uninit().write(Some((
        Mutex::<()>::new(()),
        None::<MutexGuard<'static, ()>>,
    )));
    result::Z_OK
}

/// Drops mutex and resets it to its gravestone state.
#[no_mangle]
pub extern "C" fn z_mutex_drop(this_: &mut z_moved_mutex_t) {
    let _ = this_.take_rust_type();
}

/// Returns ``true`` if mutex is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_internal_mutex_check(this_: &z_owned_mutex_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Constructs mutex in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_mutex_null(this_: &mut MaybeUninit<z_owned_mutex_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Mutably borrows mutex.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_mutex_loan_mut(this_: &mut z_owned_mutex_t) -> &mut z_loaned_mutex_t {
    this_.as_rust_type_mut().as_loaned_c_type_mut()
}

/// Takes ownership of the mutably borrowed mutex
#[no_mangle]
pub extern "C" fn z_mutex_take_loaned(
    dst: &mut MaybeUninit<z_owned_mutex_t>,
    src: &mut z_loaned_mutex_t,
) {
    dst.as_rust_type_mut_uninit()
        .write(std::mem::take(src.as_rust_type_mut()));
}

/// Locks mutex. If mutex is already locked, blocks the thread until it aquires the lock.
/// @return 0 in case of success, negative error code in case of failure.
#[no_mangle]
pub extern "C" fn z_mutex_lock(this_: &'static mut z_loaned_mutex_t) -> result::z_result_t {
    let Some(this) = this_.as_rust_type_mut() else {
        return result::Z_ENULL;
    };

    match this.0.lock() {
        Ok(new_lock) => {
            let old_lock = this.1.replace(new_lock);
            std::mem::forget(old_lock);
        }
        Err(_) => {
            return result::Z_EPOISON_MUTEX;
        }
    }
    result::Z_OK
}

/// Unlocks previously locked mutex. If mutex was not locked by the current thread, the behaviour is undefined.
/// @return 0 in case of success, negative error code otherwise.
#[no_mangle]
pub extern "C" fn z_mutex_unlock(this_: &mut z_loaned_mutex_t) -> result::z_result_t {
    let Some(this) = this_.as_rust_type_mut() else {
        return result::Z_ENULL;
    };
    if this.1.is_none() {
        return result::Z_EINVAL_MUTEX;
    } else {
        this.1.take();
    }
    result::Z_OK
}

/// Tries to lock mutex. If mutex is already locked, return immediately.
/// @return 0 in case of success, negative value if failed to aquire the lock.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_mutex_try_lock(
    this: &'static mut z_loaned_mutex_t,
) -> result::z_result_t {
    let Some(this) = this.as_rust_type_mut() else {
        return result::Z_ENULL;
    };
    match this.0.try_lock() {
        Ok(new_lock) => {
            let old_lock = this.1.replace(new_lock);
            std::mem::forget(old_lock);
        }
        Err(_) => {
            return result::Z_EBUSY_MUTEX;
        }
    }
    result::Z_OK
}

pub use crate::opaque_types::{z_loaned_condvar_t, z_moved_condvar_t, z_owned_condvar_t};
decl_c_type_inequal!(
    owned(z_owned_condvar_t, option Condvar),
    loaned(z_loaned_condvar_t),
);

/// Constructs conditional variable.
#[no_mangle]
pub extern "C" fn z_condvar_init(this_: &mut MaybeUninit<z_owned_condvar_t>) {
    this_.as_rust_type_mut_uninit().write(Some(Condvar::new()));
}

/// Constructs conditional variable in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_condvar_null(this_: &mut MaybeUninit<z_owned_condvar_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Drops conditional variable.
#[no_mangle]
pub extern "C" fn z_condvar_drop(this_: &mut z_moved_condvar_t) {
    let _ = this_.take_rust_type();
}

/// Returns ``true`` if conditional variable is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_internal_condvar_check(this_: &z_owned_condvar_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

/// Borrows conditional variable.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_condvar_loan(this_: &z_owned_condvar_t) -> &z_loaned_condvar_t {
    this_
        .as_rust_type_ref()
        .as_ref()
        .unwrap_unchecked()
        .as_loaned_c_type_ref()
}

/// Mutably borrows conditional variable.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_condvar_loan_mut(
    this: &mut z_owned_condvar_t,
) -> &mut z_loaned_condvar_t {
    this.as_rust_type_mut()
        .as_mut()
        .unwrap_unchecked()
        .as_loaned_c_type_mut()
}

/// Wakes up one blocked thread waiting on this condiitonal variable.
/// @return 0 in case of success, negative error code in case of failure.
#[no_mangle]
pub extern "C" fn z_condvar_signal(this_: &z_loaned_condvar_t) -> result::z_result_t {
    let this = this_.as_rust_type_ref();
    this.notify_one();
    result::Z_OK
}

/// Blocks the current thread until the conditional variable receives a notification.
///
/// The function atomically unlocks the guard mutex `m` and blocks the current thread.
/// When the function returns the lock will have been re-aquired again.
/// Note: The function may be subject to spurious wakeups.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_condvar_wait(
    this: &z_loaned_condvar_t,
    m: &mut z_loaned_mutex_t,
) -> result::z_result_t {
    let this = this.as_rust_type_ref();
    let Some(m) = m.as_rust_type_mut() else {
        return result::Z_ENULL;
    };
    if m.1.is_none() {
        return result::Z_EINVAL_MUTEX; // lock was not aquired prior to wait call
    }

    let lock = m.1.take().unwrap();
    match this.wait(lock) {
        Ok(new_lock) => m.1 = Some(new_lock),
        Err(_) => return result::Z_EPOISON_MUTEX,
    }

    result::Z_OK
}

pub use crate::opaque_types::{z_moved_task_t, z_owned_task_t};
decl_c_type!(
    owned(z_owned_task_t, option JoinHandle<()>),
);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_task_attr_t(usize);

/// Constructs task in a gravestone state.
#[no_mangle]
pub extern "C" fn z_internal_task_null(this_: &mut MaybeUninit<z_owned_task_t>) {
    this_.as_rust_type_mut_uninit().write(None);
}

/// Detaches the task and releases all allocated resources.
#[no_mangle]
pub extern "C" fn z_task_detach(this_: &mut z_moved_task_t) {
    let _ = this_.take_rust_type();
}

/// Joins the task and releases all allocated resources
#[no_mangle]
pub extern "C" fn z_task_join(this_: &mut z_moved_task_t) -> result::z_result_t {
    let Some(task) = this_.take_rust_type() else {
        return result::Z_OK;
    };
    match task.join() {
        Ok(_) => result::Z_OK,
        Err(_) => result::Z_EINVAL_MUTEX,
    }
}

/// Drop the task. Same as `z_task_detach`. Use `z_task_join` to wait for the task completion.
#[no_mangle]
pub extern "C" fn z_task_drop(this_: &mut z_moved_task_t) {
    let _ = this_.take_rust_type();
}

/// Returns ``true`` if task is valid, ``false`` otherwise.
#[no_mangle]
pub extern "C" fn z_internal_task_check(this_: &z_owned_task_t) -> bool {
    this_.as_rust_type_ref().is_some()
}

struct FunArgPair {
    fun: unsafe extern "C" fn(arg: *mut c_void) -> *mut c_void,
    arg: *mut c_void,
}

impl FunArgPair {
    unsafe fn call(self) {
        (self.fun)(self.arg);
    }
}

unsafe impl Send for FunArgPair {}

/// Constructs a new task.
///
/// @param this_: An uninitialized memory location where task will be constructed.
/// @param _attr: Attributes of the task (currently unused).
/// @param fun: Function to be executed by the task.
/// @param arg: Argument that will be passed to the function `fun`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_task_init(
    this: &mut MaybeUninit<z_owned_task_t>,
    _attr: *const z_task_attr_t,
    fun: unsafe extern "C" fn(arg: *mut c_void) -> *mut c_void,
    arg: *mut c_void,
) -> result::z_result_t {
    let this = this.as_rust_type_mut_uninit();
    let fun_arg_pair = FunArgPair { fun, arg };

    match thread::Builder::new().spawn(move || fun_arg_pair.call()) {
        Ok(join_handle) => {
            this.write(Some(join_handle));
        }
        Err(_) => return result::Z_EAGAIN_MUTEX,
    }
    result::Z_OK
}
