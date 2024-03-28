use std::{
    sync::{Condvar, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

use libc::c_void;

use crate::{impl_guarded_transmute, GuardedTransmute};

pub struct ZMutex<'a> {
    mutex: Mutex<()>,
    lock: Option<MutexGuard<'a, ()>>,
}

pub struct ZMutexPtr {
    data: Option<Box<ZMutex<'static>>>,
}

/// Mutex
///
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_mutex_t(usize);

impl_guarded_transmute!(noderefs z_mutex_t, ZMutexPtr);
impl_guarded_transmute!(noderefs ZMutexPtr, z_mutex_t);

// using the same error codes as in GNU pthreads, but with negative sign
// due to convention to return negative values on error
const EBUSY: i8 = -16;
const EINVAL: i8 = -22;
const EAGAIN: i8 = -11;
const EPOISON: i8 = -22; // same as EINVAL

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_mutex_init(m: *mut z_mutex_t) -> i8 {
    if m.is_null() {
        return EINVAL;
    }
    let t = ZMutexPtr {
        data: Some(Box::new(ZMutex {
            mutex: Mutex::new(()),
            lock: None,
        })),
    };
    *m = t.transmute();
    0
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_mutex_free(m: *mut z_mutex_t) -> i8 {
    if m.is_null() {
        return EINVAL;
    }
    let mut t = (*m).transmute();

    t.data.take();
    *m = t.transmute();
    0
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_mutex_lock(m: *mut z_mutex_t) -> i8 {
    if m.is_null() {
        return EINVAL;
    }
    let mut t = (*m).transmute();
    if t.data.is_none() {
        return EINVAL;
    }
    let mut_data = t.data.as_mut().unwrap();
    match mut_data.mutex.lock() {
        Ok(new_lock) => {
            let old_lock = mut_data.lock.replace(std::mem::transmute(new_lock));
            std::mem::forget(old_lock);
        }
        Err(_) => {
            return EPOISON;
        }
    }

    *m = t.transmute();
    0
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_mutex_unlock(m: *mut z_mutex_t) -> i8 {
    if m.is_null() {
        return EINVAL;
    }
    let mut t = (*m).transmute();
    if t.data.is_none() {
        return EINVAL;
    }
    let mut_data = t.data.as_mut().unwrap();
    if mut_data.lock.is_none() {
        return EINVAL;
    } else {
        mut_data.lock.take();
    }
    *m = t.transmute();
    0
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_mutex_try_lock(m: *mut z_mutex_t) -> i8 {
    if m.is_null() {
        return EINVAL;
    }
    let mut t = (*m).transmute();
    if t.data.is_none() {
        return EINVAL;
    }
    let mut_data = t.data.as_mut().unwrap();
    let mut ret: i8 = 0;
    match mut_data.mutex.try_lock() {
        Ok(new_lock) => {
            let old_lock = mut_data.lock.replace(std::mem::transmute(new_lock));
            std::mem::forget(old_lock);
        }
        Err(_) => {
            ret = EBUSY;
        }
    }
    *m = t.transmute();
    ret
}

struct ZCondvarPtr {
    data: Option<Box<Condvar>>,
}

/// Condvar
///
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_condvar_t(usize);

impl_guarded_transmute!(noderefs z_condvar_t, ZCondvarPtr);
impl_guarded_transmute!(noderefs ZCondvarPtr, z_condvar_t);

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_condvar_init(cv: *mut z_condvar_t) -> i8 {
    if cv.is_null() {
        return EINVAL;
    }
    let t: ZCondvarPtr = ZCondvarPtr {
        data: Some(Box::new(Condvar::new())),
    };
    *cv = t.transmute();
    0
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_condvar_free(cv: *mut z_condvar_t) -> i8 {
    if cv.is_null() {
        return EINVAL;
    }
    let mut t = (*cv).transmute();
    if t.data.is_none() {
        return EINVAL;
    }
    t.data.take();
    *cv = t.transmute();
    0
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_condvar_signal(cv: *mut z_condvar_t) -> i8 {
    if cv.is_null() {
        return EINVAL;
    }
    let t = (*cv).transmute();
    if t.data.is_none() {
        return EINVAL;
    }
    t.data.as_ref().unwrap().notify_one();
    *cv = t.transmute();
    0
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_condvar_wait(cv: *mut z_condvar_t, m: *mut z_mutex_t) -> i8 {
    if cv.is_null() {
        return EINVAL;
    }
    let tcv = (*cv).transmute();
    if tcv.data.is_none() {
        return EINVAL;
    }
    if m.is_null() {
        return EINVAL;
    }
    let mut tm = (*m).transmute();
    if tm.data.is_none() || tm.data.as_ref().unwrap().lock.is_none() {
        return EINVAL;
    }
    let mut_data = tm.data.as_mut().unwrap();
    let lock = mut_data.lock.take().unwrap();
    match tcv.data.as_ref().unwrap().wait(lock) {
        Ok(new_lock) => mut_data.lock = Some(std::mem::transmute(new_lock)),
        Err(_) => return EPOISON,
    }
    *cv = tcv.transmute();
    *m = tm.transmute();
    0
}

struct ZTask {
    join_handle: JoinHandle<()>,
}

struct ZTaskPtr {
    data: Option<Box<ZTask>>,
}

/// Task
///
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_task_t(usize);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_task_attr_t(usize);

impl_guarded_transmute!(noderefs z_task_t, ZTaskPtr);
impl_guarded_transmute!(noderefs ZTaskPtr, z_task_t);

struct FunArgPair {
    fun: unsafe extern "C" fn(arg: *mut c_void),
    arg: *mut c_void,
}

impl FunArgPair {
    unsafe fn call(self) {
        (self.fun)(self.arg);
    }
}

unsafe impl Send for FunArgPair {}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_task_init(
    task: *mut z_task_t,
    _attr: *const z_task_attr_t,
    fun: unsafe extern "C" fn(arg: *mut c_void),
    arg: *mut c_void,
) -> i8 {
    if task.is_null() {
        return EINVAL;
    }

    let mut ttask = ZTaskPtr { data: None };
    let fun_arg_pair = FunArgPair { fun, arg };

    let mut ret = 0;
    match thread::Builder::new().spawn(move || fun_arg_pair.call()) {
        Ok(join_handle) => ttask.data = Some(Box::new(ZTask { join_handle })),
        Err(_) => ret = EAGAIN,
    }
    *task = ttask.transmute();
    ret
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_task_join(task: *mut z_task_t) -> i8 {
    if task.is_null() {
        return EINVAL;
    }
    let mut ttask = (*task).transmute();
    if ttask.data.is_none() {
        return EINVAL;
    }
    let data = ttask.data.take();
    let ret = match data.unwrap().join_handle.join() {
        Ok(_) => 0,
        Err(_) => EINVAL,
    };
    *task = ttask.transmute();
    ret
}
