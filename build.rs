extern crate cbindgen;

use std::env;
use std::path::PathBuf;

use cbindgen::Config;

const CONF_FILE: &str = "cbindgen.toml";
const OUT_FILE: &str = "include/zenoh/net.h";

fn main() {
    let base_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Generate C bindings
    let bindings = cbindgen::Builder::new()
        .with_config(Config::from_file(CONF_FILE).expect("Failed to read cbindgen config"))
        .with_crate(&base_dir)
        .generate()
        .expect("Unable to generate C bindings");

    let out_file = base_dir.join(OUT_FILE);
    if out_file.is_file() {
        std::fs::remove_file(&out_file).unwrap();
    }
    if !bindings.write_to_file(&out_file) {
        panic!("Failed to write C bindings into {:?}", out_file);
    }
}
