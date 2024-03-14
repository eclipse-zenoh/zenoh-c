use chrono::{DateTime, Local};
use libc::c_char;
use std::{
    cmp::min,
    os::raw::c_void,
    slice,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use lazy_static::lazy_static;

// Use initial time stored in static variable as a reference time,
// to be able to return number of ns passed since.
// This is to avoid wrapping Instant into a c type and not
// have to account for its platform-dependent size and alignment.
lazy_static! {
    static ref CLOCK_BASE: Instant = Instant::now();
}

/// Clock
/// Uses monotonic clock
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_clock_t {
    t: u64,
    t_base: *const c_void,
}

#[no_mangle]
pub extern "C" fn z_clock_now() -> z_clock_t {
    z_clock_t {
        t: CLOCK_BASE.elapsed().as_nanos() as u64,
        t_base: &CLOCK_BASE as *const CLOCK_BASE as *const c_void,
    }
}
#[allow(clippy::missing_safety_doc)]
unsafe fn get_elapsed_nanos(time: *const z_clock_t) -> u64 {
    if time.is_null() {
        return 0;
    }
    let now_t = (*((*time).t_base as *const CLOCK_BASE))
        .elapsed()
        .as_nanos() as u64;
    if now_t > (*time).t {
        now_t - (*time).t
    } else {
        0
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_clock_elapsed_s(time: *const z_clock_t) -> u64 {
    get_elapsed_nanos(time) / 1_000_000_000
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_clock_elapsed_ms(time: *const z_clock_t) -> u64 {
    get_elapsed_nanos(time) / 1_000_000
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_clock_elapsed_us(time: *const z_clock_t) -> u64 {
    get_elapsed_nanos(time) / 1_000
}

/// Time
/// Uses system clock
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_time_t {
    t: u64,
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_time_now_as_str(buf: *const c_char, len: usize) -> *const c_char {
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
pub extern "C" fn z_time_now() -> z_time_t {
    z_time_t {
        t: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_nanos() as u64,
    }
}
#[allow(clippy::missing_safety_doc)]
unsafe fn get_elapsed_nanos_system_clock(time: *const z_time_t) -> u64 {
    if time.is_null() {
        return 0;
    }
    let now_t = z_time_now().t;
    if now_t > (*time).t {
        now_t - (*time).t
    } else {
        0
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_time_elapsed_s(time: *const z_time_t) -> u64 {
    get_elapsed_nanos_system_clock(time) / 1_000_000_000
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_time_elapsed_ms(time: *const z_time_t) -> u64 {
    get_elapsed_nanos_system_clock(time) / 1_000_000
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_time_elapsed_us(time: *const z_time_t) -> u64 {
    get_elapsed_nanos_system_clock(time) / 1_000
}
