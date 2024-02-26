use chrono::{DateTime, Local};
use libc::c_char;
use std::{
    cmp::min,
    slice,
    time::{Duration, Instant, SystemTime},
};

use crate::{impl_guarded_transmute, GuardedTransmute};

// TODO: properly define size/alignment for every architecture
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zp_time_t([u64; 2]);

impl_guarded_transmute!(zp_time_t, Instant);
impl_guarded_transmute!(Instant, zp_time_t);

#[no_mangle]
pub extern "C" fn zp_time_now() -> zp_time_t {
    let t = Instant::now();
    t.transmute()
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
unsafe fn get_elapsed(time: *const zp_time_t) -> Duration {
    if time.is_null() {
        return Duration::new(0, 0);
    }
    let ttime = (*time).transmute();
    let ret = ttime.elapsed();
    #[allow(forgetting_copy_types)]
    std::mem::forget(ttime);
    ret
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_time_elapsed_s(time: *const zp_time_t) -> usize {
    get_elapsed(time).as_secs() as usize
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_time_elapsed_ms(time: *const zp_time_t) -> usize {
    get_elapsed(time).as_millis() as usize
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn zp_time_elapsed_us(time: *const zp_time_t) -> usize {
    get_elapsed(time).as_micros() as usize
}
