use std::slice::from_raw_parts_mut;

use libc::c_void;
use rand::{random, thread_rng, RngCore};

#[no_mangle]
pub extern "C" fn z_random_u8() -> u8 {
    random::<u8>()
}

#[no_mangle]
pub extern "C" fn z_random_u16() -> u16 {
    random::<u16>()
}

#[no_mangle]
pub extern "C" fn z_random_u32() -> u32 {
    random::<u32>()
}

#[no_mangle]
pub extern "C" fn z_random_u64() -> u64 {
    random::<u64>()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn z_random_fill(buf: *mut c_void, len: usize) {
    if buf.is_null() || len == 0 {
        return;
    }
    let b: &mut [u8] = from_raw_parts_mut(buf as *mut u8, len);
    thread_rng().fill_bytes(b);
}
