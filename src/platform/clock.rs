use chrono::{DateTime, Local};
use libc::c_char;
use std::{
    cmp::min,
    slice,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use lazy_static::lazy_static;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref CLOCK_BASE: Instant = Instant::now();
}

/// Clock
/// Uses monotonic clock
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zp_clock_t {
    t: f64,
}

#[no_mangle]
pub extern "C" fn zp_clock_now() -> zp_clock_t {
    zp_clock_t {
        t: CLOCK_BASE.elapsed().as_secs_f64(),
    }
}
#[allow(clippy::missing_safety_doc)]
unsafe fn get_elapsed_seconds(time: *const zp_clock_t) -> f64 {
    if time.is_null() {
        return 0.0;
    }
    zp_clock_now().t - (*time).t
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_clock_elapsed_s(time: *const zp_clock_t) -> u64 {
    get_elapsed_seconds(time) as u64
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_clock_elapsed_ms(time: *const zp_clock_t) -> u64 {
    (get_elapsed_seconds(time) * 1000.0) as u64
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_clock_elapsed_us(time: *const zp_clock_t) -> u64 {
    (get_elapsed_seconds(time) * 1000000.0) as u64
}

/// Time
/// Uses system clock
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zp_time_t {
    t: f64,
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

#[no_mangle]
pub extern "C" fn zp_time_now() -> zp_time_t {
    zp_time_t {
        t: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_secs_f64(),
    }
}
#[allow(clippy::missing_safety_doc)]
unsafe fn get_elapsed_seconds_system_clock(time: *const zp_time_t) -> f64 {
    if time.is_null() {
        return 0.0;
    }
    0.0f64.max(zp_time_now().t - (*time).t)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_time_elapsed_s(time: *const zp_time_t) -> u64 {
    get_elapsed_seconds_system_clock(time) as u64
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_time_elapsed_ms(time: *const zp_time_t) -> u64 {
    (get_elapsed_seconds_system_clock(time) * 1000.0) as u64
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_time_elapsed_us(time: *const zp_time_t) -> u64 {
    (get_elapsed_seconds_system_clock(time) * 1000000.0) as u64
}
