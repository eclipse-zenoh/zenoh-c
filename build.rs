use std::env;

use fs_extra::{dir, file};

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

pub fn dump_rust_sources(out_path: &std::path::Path) {
    let build_rs_path = get_build_rs_path();
    let src_path = build_rs_path.join("./src");

    let out_rs_path = get_out_rs_path();
    let opaque_types_src_path = out_rs_path.join("./opaque_types.rs");

    if !out_path.exists() {
        std::fs::create_dir_all(out_path).unwrap();
    }
    fs_extra::copy_items(
        &[src_path],
        out_path,
        &dir::CopyOptions::new().overwrite(true),
    )
    .unwrap();
    fs_extra::file::copy(
        opaque_types_src_path,
        out_path.join("./src/opaque_types/mod.rs"),
        &file::CopyOptions::new().overwrite(true),
    )
    .unwrap();
}

fn main() {
    buildrs::opaque_types_generator::generate_opaque_types();
    prebindgen::init_prebindgen_out_dir();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=buildrs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build-resources");
}
