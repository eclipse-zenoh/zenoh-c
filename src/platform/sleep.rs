use std::{thread, time};

#[no_mangle]
pub extern "C" fn zp_sleep_s(time: usize) -> i8 {
    thread::sleep(time::Duration::from_secs(time as u64));
    0
}

#[no_mangle]
pub extern "C" fn zp_sleep_ms(time: usize) -> i8 {
    thread::sleep(time::Duration::from_millis(time as u64));
    0
}

#[no_mangle]
pub extern "C" fn zp_sleep_us(time: usize) -> i8 {
    thread::sleep(time::Duration::from_micros(time as u64));
    0
}
