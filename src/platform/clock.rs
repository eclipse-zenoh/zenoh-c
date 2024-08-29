use std::{
    os::raw::c_void,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use libc::c_char;

use crate::CopyableToCArray;

// Use initial time stored in static variable as a reference time,
// to be able to return number of ns passed since.
// This is to avoid wrapping Instant into a c type and not
// have to account for its platform-dependent size and alignment.
lazy_static! {
    static ref CLOCK_BASE: Instant = Instant::now();
}

/// Monotonic clock
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_clock_t {
    t: u64,
    t_base: *const c_void,
}

/// Returns monotonic clock time point corresponding to the current time instant.
#[no_mangle]
pub extern "C" fn z_clock_now() -> z_clock_t {
    z_clock_t {
        t: CLOCK_BASE.elapsed().as_nanos() as u64,
        t_base: &CLOCK_BASE as *const CLOCK_BASE as *const c_void,
    }
}

/// Get number of nanoseconds passed since creation of `time`.
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

/// Get number of seconds passed since creation of `time`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_clock_elapsed_s(time: *const z_clock_t) -> u64 {
    get_elapsed_nanos(time) / 1_000_000_000
}

/// Get number of milliseconds passed since creation of `time`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_clock_elapsed_ms(time: *const z_clock_t) -> u64 {
    get_elapsed_nanos(time) / 1_000_000
}

/// Get number of microseconds passed since creation of `time`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_clock_elapsed_us(time: *const z_clock_t) -> u64 {
    get_elapsed_nanos(time) / 1_000
}

/// Returns system clock time point corresponding to the current time instant.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct z_time_t {
    t: u64,
}

/// Converts current system time into null-terminated human readable string and writes it to the `buf`.
///
/// @param buf: A buffer where the string will be writtent
/// @param len: Maximum number of characters to write (including terminating 0). The string will be truncated
/// if it is longer than `len`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_time_now_as_str(buf: *const c_char, len: usize) -> *const c_char {
    if len == 0 || buf.is_null() {
        return buf;
    }
    let datetime: DateTime<Local> = SystemTime::now().into();
    let s = datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let res = s.as_str().copy_to_c_array(buf as *mut c_void, len - 1);
    *((buf as usize + res) as *mut c_char) = 0;
    buf
}

/// Initialize clock with current time instant.
#[no_mangle]
pub extern "C" fn z_time_now() -> z_time_t {
    z_time_t {
        t: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_nanos() as u64,
    }
}

/// Get number of nanoseconds passed since creation of `time`.
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

/// Get number of seconds passed since creation of `time`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_time_elapsed_s(time: *const z_time_t) -> u64 {
    get_elapsed_nanos_system_clock(time) / 1_000_000_000
}

/// Get number of milliseconds passed since creation of `time`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_time_elapsed_ms(time: *const z_time_t) -> u64 {
    get_elapsed_nanos_system_clock(time) / 1_000_000
}

/// Get number of microseconds passed since creation of `time`.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_time_elapsed_us(time: *const z_time_t) -> u64 {
    get_elapsed_nanos_system_clock(time) / 1_000
}
