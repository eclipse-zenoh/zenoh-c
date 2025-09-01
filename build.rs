mod buildrs;

fn main() {
    let source = buildrs::prebindgen_generator::generate_source();
    prebindgen::trace!("Generated source: {}", source.display());
    buildrs::cbindgen_generator::generate_c_headers(&source);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=splitguide.yaml");
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=include");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        let name = std::env::var("CARGO_PKG_NAME").unwrap();
        // Create the shared library name by removing hyphens from the pkg_name
        let soname = format!("lib{}.so", name.replace('-', ""));
        println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,{soname}");
    }
}
