use std::env;

use crate::buildrs::opaque_types_generator::generate_opaque_types;
mod buildrs;

pub fn get_build_rs_path() -> std::path::PathBuf {
    let file_path = file!();
    let mut path_buf = std::path::PathBuf::new();
    path_buf.push(file_path);
    path_buf.parent().unwrap().to_path_buf()
}

pub fn get_out_rs_path() -> std::path::PathBuf {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    std::path::Path::new(&out_dir).to_path_buf()
}

fn main() {
    // Initialize the directory for collecting API (elements marked with `#[prebindgen]`)
    prebindgen::init_prebindgen_out_dir();

    // Generate the platform-specific types.
    // Specifically handle the cross-compilation case in the dependent crate (e.g. zenoh-c)
    // - run cross compilation with `CROSS_TARGET` set to the target platform, e.g.:
    //   `CROSS_TARGET=x86_64-unknown-linux-gnu cargo build --target x86_64-unknown-linux-gnu`
    // - the zenoh-c/build.rs depends on the zenoh-ffi and uses `prebindgen` crate to extract the FFI API (see `prebindgen` crate documentation)
    // - the zenoh-c itself depends on the zenoh-ffi and includes file with FFI API generated in it's build.rs
    //
    // So the zenoh-ffi/build.rs will be run twice: once with TARGET=host platform 
    // and once with TARGET=target platform.
    // The CROSS_TARGET variable allows to distinguish between the two cases. If it exists and is different
    // from the current TARGET, we should provide #[prebindgen]-marked types for the CROSS_TARGET
    // platform. But on the other hand whe should also provide the types for the current TARGET platform
    // to be able to compile the zenoh-ffi crate on the host platform.
    //
    // This is solved by generating the types twice, both for host and target platforms and by
    // using `skip` option in the `#[prebindgen]` attribute not pass target-specific types to the compiler
    let path_out = get_out_rs_path().join("opaque_types.rs");
    let current_target = std::env::var("TARGET").unwrap();
    let cross_target = std::env::var("CROSS_TARGET")
        .ok()
        .filter(|s| s != &current_target);

    std::fs::remove_file(&path_out).ok();
    if let Some(ref cross_target) = cross_target {
        // Bar definition for the host, wihtout prebindgen generation
        generate_opaque_types(&current_target, &path_out, None);
        // Bar definition for the target, skipped on the host but generated prebindgen data for the target
        generate_opaque_types(cross_target, &path_out, Some(true));

        prebindgen::trace!(
            "Generated opaque types for TARGET={}, CROSS_TARGET={} : {}",
            current_target,
            cross_target,
            path_out.display()
        );
    } else {
        generate_opaque_types(&current_target, &path_out, Some(false));
        prebindgen::trace!(
            "Generated opaque types for TARGET={} : {}",
            current_target,
            path_out.display()
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=buildrs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build-resources");
}
