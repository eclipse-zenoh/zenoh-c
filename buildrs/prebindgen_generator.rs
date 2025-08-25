use std::path::PathBuf;

use itertools::Itertools;

pub fn generate_source() -> PathBuf {
    let source_ffi = prebindgen::Source::new(zenoh_ffi::PREBINDGEN_OUT_DIR);
    let source_ffi_opaque_types =
        prebindgen::Source::new(zenoh_ffi_opaque_types::PREBINDGEN_OUT_DIR);

    let replace_types = prebindgen::map::ReplaceTypes::builder()
        .replace_type("MaybeUninit", "std::mem::MaybeUninit")
        .replace_type("c_char", "libc::c_char")
        .replace_type("c_void", "libc::c_void")
        .replace_type("c_uint", "libc::c_uint")
        .replace_type("result::z_result_t", "z_result_t")
        .build();

    let strip_derives = prebindgen::map::StripDerives::builder()
        .strip_derive("Default")
        .build();

    let strip_macros = prebindgen::map::StripMacros::builder()
        .strip_macro("default")
        .build();

    let ffi_converter = prebindgen::batching::FfiConverter::builder("zenoh_ffi")
        .edition(prebindgen::RustEdition::Edition2021)
        .strip_transparent_wrapper("std::mem::MaybeUninit")
        .strip_transparent_wrapper("::std::mem::MaybeUninit")
        .strip_transparent_wrapper("Option")
        .build();

    source_ffi
        .items_except_groups(&["move"]) // the move operations are generated in the header
        .chain(source_ffi_opaque_types.items_all())
        .map(strip_derives.into_closure())
        .map(strip_macros.into_closure())
        .map(replace_types.into_closure())
        .batching(ffi_converter.into_closure())
        .collect::<prebindgen::collect::Destination>()
        .write("zenoh_ffi.rs")
}
