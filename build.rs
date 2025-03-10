use build_mods::{cbindgen::generate_c_headers, opaque_types::generate_opaque_types};

mod build_mods;

fn main() {
    generate_opaque_types();
    generate_c_headers();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=splitguide.yaml");
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=build-resources");
    println!("cargo:rerun-if-changed=include");
}
