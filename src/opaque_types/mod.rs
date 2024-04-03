/// A split buffer that owns all of its data.
///
/// To minimize copies and reallocations, Zenoh may provide you data in split buffers.
#[repr(C, align(8))]
pub struct z_owned_buffer_t {
    _0: [u8; 40]
}
/// An owned sample.
///
/// This is a read only type that can only be constructed by cloning a `z_sample_t`.
/// Like all owned types, its memory must be freed by passing a mutable reference to it to `zc_sample_drop`.
#[repr(C, align(8))]
pub struct zc_owned_sample_t {
    _0: [u8; 224]
}
