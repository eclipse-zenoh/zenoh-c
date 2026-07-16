use std::{env, fs};

use fs_extra::{dir, file};

mod buildrs;

pub fn get_build_rs_path() -> std::path::PathBuf {
    // Use file!() to get the path to build.rs
    // On Windows through CMake, this might be a relative path, so we need to make it absolute
    let file_path = file!();
    let base_path = std::path::PathBuf::from(file_path);

    let absolute_path = if base_path.is_absolute() {
        base_path
    } else {
        // If relative, resolve relative to current directory (which should be the source root during build)
        std::env::current_dir()
            .ok()
            .map(|cwd| cwd.join(&base_path))
            .unwrap_or(base_path)
    };

    absolute_path.parent().unwrap().to_path_buf()
}

pub fn get_out_rs_path() -> std::path::PathBuf {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    std::path::Path::new(&out_dir).to_path_buf()
}

pub fn dump_rust_sources(out_path: &std::path::Path) {
    let build_rs_path = get_build_rs_path();
    let src_path = build_rs_path.join("src");

    let out_rs_path = get_out_rs_path();
    let opaque_types_src_path = out_rs_path.join("opaque_types.rs");

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
        out_path.join("src/opaque_types/mod.rs"),
        &file::CopyOptions::new().overwrite(true),
    )
    .unwrap();
}

fn sync_opaque_types_lockfile() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .expect("CARGO_MANIFEST_DIR is not set");
    let root_lock = manifest_dir.join("Cargo.lock");
    if !root_lock.exists() {
        panic!("Missing Cargo.lock at {}", root_lock.display());
    }

    let opaque_types_dir = get_build_rs_path().join("build-resources/opaque-types");
    let opaque_lock = opaque_types_dir.join("Cargo.lock");

    println!(
        "cargo:warning=Copying Cargo.lock from {} to {}",
        root_lock.display(),
        opaque_lock.display()
    );

    if let Err(err) = fs::copy(&root_lock, &opaque_lock) {
        panic!(
            "Failed to copy Cargo.lock to {}: {}",
            opaque_lock.display(),
            err
        );
    }

    println!("cargo:warning=Successfully copied Cargo.lock");
}

fn main() {
    sync_opaque_types_lockfile();
    buildrs::opaque_types_generator::generate_opaque_types();
    buildrs::cbindgen_generator::generate_c_headers();
    if let Some(out_path) = env::var_os("ZENOHC_DUMP_SRC_DIR") {
        let out_path = std::path::Path::new(&out_path);
        dump_rust_sources(out_path);
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=buildrs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=splitguide.yaml");
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=build-resources");
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=Cargo.lock");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        let name = std::env::var("CARGO_PKG_NAME").unwrap();
        // Create the shared library name by removing hyphens from the pkg_name
        let soname = format!("lib{}.so", name.replace('-', ""));
        println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,{}", soname);
    }
}
