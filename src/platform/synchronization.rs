use std::{
    sync::{Condvar, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

use libc::c_void;

use crate::{impl_guarded_transmute, GuardedTransmute};

pub struct ZPMutex<'a> {
    mutex: Mutex<()>,
    lock: Option<MutexGuard<'a, ()>>,
}

pub struct ZPMutexPtr {
    data: Option<Box<ZPMutex<'static>>>,
}

/// Mutex
///
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zp_mutex_t(usize);

impl_guarded_transmute!(zp_mutex_t, ZPMutexPtr);
impl_guarded_transmute!(ZPMutexPtr, zp_mutex_t);

const EBUSY: i8 = -1;
const EINVAL: i8 = -2;
const EAGAIN: i8 = -3;
const EPOISON: i8 = -10;

#[no_mangle]
pub unsafe extern "C" fn zp_mutex_init(m: *mut zp_mutex_t) -> i8 {
    if m.is_null() {
        return EINVAL;
    }
    let t = ZPMutexPtr {
        data: Some(Box::new(ZPMutex {
            mutex: Mutex::new(()),
            lock: None,
        })),
    };
    *m = t.transmute();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn zp_mutex_free(m: *mut zp_mutex_t) -> i8 {
    if m.is_null() {
        return EINVAL;
    }
    let mut t = (*m).transmute();

    t.data.take();
    *m = t.transmute();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn zp_mutex_lock(m: *mut zp_mutex_t) -> i8 {
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
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn zp_mutex_unlock(m: *mut zp_mutex_t) -> i8 {
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
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn zp_mutex_try_lock(m: *mut zp_mutex_t) -> i8 {
    if m.is_null() {
        return EINVAL;
    }
    let mut t = (*m).transmute();
    if t.data.is_none() {
        return EINVAL;
    }
    let mut_data = t.data.as_mut().unwrap();
    let new_lock = mut_data.mutex.try_lock();
    let mut ret: i8 = 0;
    if new_lock.is_ok() {
        let old_lock = mut_data
            .lock
            .replace(std::mem::transmute(new_lock.unwrap()));
        std::mem::forget(old_lock);
    } else {
        std::mem::drop(new_lock);
        ret = EBUSY;
    }
    *m = t.transmute();
    return ret;
}

struct ZPCondvarPtr {
    data: Option<Box<Condvar>>,
}

/// Condvar
///
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zp_condvar_t(usize);

impl_guarded_transmute!(zp_condvar_t, ZPCondvarPtr);
impl_guarded_transmute!(ZPCondvarPtr, zp_condvar_t);

#[no_mangle]
pub unsafe extern "C" fn zp_condvar_init(cv: *mut zp_condvar_t) -> i8 {
    if cv.is_null() {
        return EINVAL;
    }
    let t: ZPCondvarPtr = ZPCondvarPtr {
        data: Some(Box::new(Condvar::new())),
    };
    *cv = t.transmute();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn zp_condvar_free(cv: *mut zp_condvar_t) -> i8 {
    if cv.is_null() {
        return EINVAL;
    }
    let mut t = (*cv).transmute();
    if t.data.is_none() {
        return EINVAL;
    }
    t.data.take();
    *cv = t.transmute();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn zp_condvar_signal(cv: *mut zp_condvar_t) -> i8 {
    if cv.is_null() {
        return EINVAL;
    }
    let t = (*cv).transmute();
    if t.data.is_none() {
        return EINVAL;
    }
    t.data.as_ref().unwrap().notify_one();
    *cv = t.transmute();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn zp_condvar_wait(cv: *mut zp_condvar_t, m: *mut zp_mutex_t) -> i8 {
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
    return 0;
}

struct ZPTask {
    join_handle: JoinHandle<()>,
}

struct ZPTaskPtr {
    data: Option<Box<ZPTask>>,
}

/// Task
///
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zp_task_t(usize);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct zp_task_attr_t(usize);

impl_guarded_transmute!(zp_task_t, ZPTaskPtr);
impl_guarded_transmute!(ZPTaskPtr, zp_task_t);

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
pub unsafe extern "C" fn zp_task_init(
    task: *mut zp_task_t,
    _attr: *const zp_task_attr_t,
    fun: unsafe extern "C" fn(arg: *mut c_void),
    arg: *mut c_void,
) -> i8 {
    if task.is_null() {
        return EINVAL;
    }

    let mut ttask = ZPTaskPtr {
        data: None
    };
    let fun_arg_pair = FunArgPair { fun, arg };

    let mut ret = 0;
    match thread::Builder::new().spawn(move || { fun_arg_pair.call()}) {
        Ok(join_handle) => ttask.data = Some(Box::new(ZPTask { join_handle })),
        Err(_) => ret = EAGAIN,
    }
    *task = ttask.transmute(); 
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn zp_task_join(task: *mut zp_task_t) -> i8 {
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
    return ret;
}
