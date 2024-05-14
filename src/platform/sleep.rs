use std::{thread, time};

/// Puts current thread to sleep for specified amount of seconds.
#[no_mangle]
pub extern "C" fn z_sleep_s(time: usize) -> i8 {
    thread::sleep(time::Duration::from_secs(time as u64));
    0
}

/// Puts current thread to sleep for specified amount of milliseconds.
#[no_mangle]
pub extern "C" fn z_sleep_ms(time: usize) -> i8 {
    thread::sleep(time::Duration::from_millis(time as u64));
    0
}

/// Puts current thread to sleep for specified amount of microseconds.
#[no_mangle]
pub extern "C" fn z_sleep_us(time: usize) -> i8 {
    thread::sleep(time::Duration::from_micros(time as u64));
    0
}
