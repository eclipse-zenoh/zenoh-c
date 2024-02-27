use chrono::{DateTime, Local};
use libc::c_char;
use std::{
    cmp::min,
    slice,
    time::{Instant, SystemTime},
};

use lazy_static::lazy_static;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref INSTANT_BASE: Instant = Instant::now();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct zp_time_t {
    t: f64,
}

#[no_mangle]
pub extern "C" fn zp_time_now() -> zp_time_t {
    zp_time_t {
        t: INSTANT_BASE.elapsed().as_secs_f64(),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_time_now_as_str(buf: *const c_char, len: usize) -> *const c_char {
    if len == 0 {
        return buf;
    }
    let datetime: DateTime<Local> = SystemTime::now().into();
    let s = datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let sb = s.as_bytes();
    let max_len = min(len - 1, s.len());
    let b = slice::from_raw_parts_mut(buf as *mut u8, max_len + 1);
    b[0..max_len].copy_from_slice(&sb[0..max_len]);
    b[max_len] = 0;
    buf
}

#[allow(clippy::missing_safety_doc)]
unsafe fn get_elapsed_seconds(time: *const zp_time_t) -> f64 {
    if time.is_null() {
        return 0.0;
    }
    zp_time_now().t - (*time).t
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_time_elapsed_s(time: *const zp_time_t) -> u64 {
    get_elapsed_seconds(time) as u64
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_time_elapsed_ms(time: *const zp_time_t) -> u64 {
    (get_elapsed_seconds(time) * 1000.0) as u64
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_time_elapsed_us(time: *const zp_time_t) -> u64 {
    (get_elapsed_seconds(time) * 1000000.0) as u64
}
