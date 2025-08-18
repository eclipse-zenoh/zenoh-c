use std::slice::from_raw_parts_mut;

use libc::c_void;
use prebindgen_proc_macro::prebindgen;
use rand::{random, rng, RngCore};

/// Generates random `uint8_t`.
#[prebindgen]
pub fn z_random_u8() -> u8 {
    random::<u8>()
}

/// Generates random `uint16_t`.
#[prebindgen]
pub fn z_random_u16() -> u16 {
    random::<u16>()
}

/// Generates random `uint32_t`.
#[prebindgen]
pub fn z_random_u32() -> u32 {
    random::<u32>()
}

/// Generates random `uint64_t`.
#[prebindgen]
pub fn z_random_u64() -> u64 {
    random::<u64>()
}

/// Fills buffer with random data.
#[prebindgen]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn z_random_fill(buf: *mut c_void, len: usize) {
    if buf.is_null() || len == 0 {
        return;
    }
    let b: &mut [u8] = from_raw_parts_mut(buf as *mut u8, len);
    rng().fill_bytes(b);
}
