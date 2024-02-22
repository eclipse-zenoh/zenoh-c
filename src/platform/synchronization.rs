use std::sync::{Condvar, Mutex, MutexGuard};

use crate::{impl_guarded_transmute, GuardedTransmute};


pub struct ZPMutex<'a> {
    mutex: Mutex<()>,
    lock: Option<MutexGuard<'a, ()>>
}

pub struct ZPMutexPtr {
    data: Option<Box<ZPMutex<'static>>>
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
const EPOISON: i8 = -10;

#[no_mangle]
pub unsafe extern "C" fn zp_mutex_init(m: *mut zp_mutex_t) -> i8 {
    if m.is_null() {
        return EINVAL;
    }
    let t = ZPMutexPtr {
        data: 
            Some(Box::new(
                ZPMutex {
                    mutex: Mutex::new(()),
                    lock: None
                }
            ))
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
        },
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
        let old_lock = mut_data.lock.replace(std::mem::transmute(new_lock.unwrap()));
        std::mem::forget(old_lock);
    } else {
        std::mem::drop(new_lock);
        ret = EBUSY;
    }
    *m = t.transmute();
    return ret;
}


struct ZPCondvarPtr {
    data: Option<Box<Condvar>>
}

/// CondVar
///
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zp_condvar_t(usize);

impl_guarded_transmute!(zp_condvar_t, ZPCondvarPtr);
impl_guarded_transmute!(ZPCondvarPtr, zp_condvar_t);


#[no_mangle]
pub unsafe extern "C" fn zp_condvar_init(c: *mut zp_condvar_t) -> i8 {
    if c.is_null() {
        return EINVAL;
    }
    let t: ZPCondvarPtr = ZPCondvarPtr {
        data: Some(Box::new(Condvar::new()))
    };
    *c = t.transmute();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn zp_condvar_free(c: *mut zp_condvar_t) -> i8 {
    if c.is_null() {
        return EINVAL;
    }
    let mut t = (*c).transmute();
    if t.data.is_none() {
        return EINVAL;
    }
    t.data.take();
    *c = t.transmute();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn zp_condvar_signal(c: *mut zp_condvar_t) -> i8 {
    if c.is_null() {
        return EINVAL;
    }
    let t = (*c).transmute();
    if t.data.is_none() {
        return EINVAL;
    }
    t.data.as_ref().unwrap().notify_one();
    *c = t.transmute();
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn zp_condvar_wait(c: *mut zp_condvar_t, m: *mut zp_mutex_t) -> i8 {
    if c.is_null() {
        return EINVAL;
    }
    let tc = (*c).transmute();
    if tc.data.is_none() {
        return EINVAL;
    }
    if m.is_null() {
        return EINVAL;
    }
    let mut tm = (*m).transmute();
    if tm.data.is_none() ||  tm.data.as_ref().unwrap().lock.is_none() {
        return EINVAL;
    }
    let mut_data = tm.data.as_mut().unwrap();
    let lock = mut_data.lock.take().unwrap();
    match tc.data.as_ref().unwrap().wait(lock) {
        Ok(new_lock) => mut_data.lock = Some(std::mem::transmute(new_lock)),
        Err(_) => return EPOISON
    }
    *c = tc.transmute();
    *m = tm.transmute();
    return 0;
}
