use zenoh::sample::Sample;
use zenoh::buffers::ZBuf;

#[macro_export]
macro_rules! get_opaque_type_data {
($src_type:ty, $expr:expr) => {
        const _: () = {
            let align = std::mem::align_of::<$src_type>();
            let size = std::mem::size_of::<$src_type>();
            let mut msg: [u8; 61] = *b"type:                                 , align:    , size:    ";
            let mut i = 0;
            while i < 4 {
                msg[i as usize + 46] = b'0' + ((align / 10u32.pow(3 - i) as usize) % 10) as u8;
                msg[i as usize + 57] = b'0' + ((size / 10u32.pow(3 - i) as usize) % 10) as u8;
                i += 1;
            }
            let mut i: usize = 0;
            while i < $expr.len() {
                msg[i as usize + 5] = $expr.as_bytes()[i];
                i += 1;
            }
            panic!("{}", unsafe {
                std::str::from_utf8_unchecked(msg.as_slice())
            });
        };
    }
}

/// A split buffer that owns all of its data.
///
/// To minimize copies and reallocations, Zenoh may provide you data in split buffers.
get_opaque_type_data!(Option<ZBuf>, "z_owned_buffer_t");

/// An owned sample.
///
/// This is a read only type that can only be constructed by cloning a `z_sample_t`.
/// Like all owned types, its memory must be freed by passing a mutable reference to it to `zc_sample_drop`.
get_opaque_type_data!(Option<Sample>, "zc_owned_sample_t");