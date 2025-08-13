use std::env;

use crate::buildrs::opaque_types_generator::generate_opaque_types;
mod buildrs;

pub fn get_build_rs_path() -> std::path::PathBuf {
    let file_path = file!();
    let mut path_buf = std::path::PathBuf::new();
    path_buf.push(file_path);
    path_buf.parent().unwrap().to_path_buf()
}

pub fn get_out_dir() -> std::path::PathBuf {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    std::path::Path::new(&out_dir).to_path_buf()
}

fn main() {
    // Initialize the directory for collecting API for dependent crates
    prebindgen::init_prebindgen_out_dir();

    // Copy Cargo.lock from CARGO_LOCK environment variable
    let cargo_lock_path = zenoh_ffi_workspace::get_cargo_lock_path();
    let path_out = get_out_dir().join("opaque_types.rs");
    let target = std::env::var("TARGET").unwrap();
    generate_opaque_types(&path_out, &target, &cargo_lock_path);
    prebindgen::trace!(
        "Generated opaque types for TARGET={} : {}",
        target,
        path_out.display()
    );
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=buildrs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build-resources");
}
