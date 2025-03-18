mod buildrs;

pub fn get_build_rs_path() -> std::path::PathBuf {
    let file_path = file!();
    let mut path_buf = std::path::PathBuf::new();
    path_buf.push(file_path);
    path_buf.parent().unwrap().to_path_buf()
}

fn main() {
    buildrs::opaque_types_generator::generate_opaque_types();
    buildrs::cbindgen_generator::generate_c_headers();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=splitguide.yaml");
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=build-resources");
    println!("cargo:rerun-if-changed=include");
}
