use core::panic;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

mod build_mods;


fn main() {
    generate_opaque_types();
    cbindgen::generate(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .expect("Unable to generate bindings")
        .write_to_file(BUGGY_GENERATION_PATH);

    fix_cbindgen(BUGGY_GENERATION_PATH, GENERATION_PATH);
    std::fs::remove_file(BUGGY_GENERATION_PATH).unwrap();

    preprocess_header(GENERATION_PATH, PREPROCESS_PATH);
    create_generics_header(PREPROCESS_PATH, "include/zenoh_macros.h");
    std::fs::remove_file(PREPROCESS_PATH).unwrap();

    configure();
    let split_guide = SplitGuide::from_yaml(SPLITGUIDE_PATH);
    split_bindings(&split_guide).unwrap();
    text_replace(split_guide.rules.iter().map(|(name, _)| name.as_str()));

    fs_extra::copy_items(
        &["include"],
        cargo_target_dir(),
        &fs_extra::dir::CopyOptions::default().overwrite(true),
    )
    .expect("include should be copied to CARGO_TARGET_DIR");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=splitguide.yaml");
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=build-resources");
    println!("cargo:rerun-if-changed=include");
}

fn get_build_rs_path() -> PathBuf {
    let file_path = file!();
    let mut path_buf = PathBuf::new();
    path_buf.push(file_path);
    path_buf.parent().unwrap().to_path_buf()
}

fn produce_opaque_types_data() -> PathBuf {
    let target = env::var("TARGET").unwrap();
    let linker = env::var("RUSTC_LINKER").unwrap_or_default();
    let current_folder = get_build_rs_path();
    let manifest_path = current_folder.join("./build-resources/opaque-types/Cargo.toml");
    let output_file_path = current_folder.join("./.build_resources_opaque_types.txt");
    let out_file = std::fs::File::create(output_file_path.clone()).unwrap();
    let stdio = Stdio::from(out_file);

    let mut linker_args = Vec::<String>::new();
    if !linker.is_empty() {
        linker_args.push("--config".to_string());
        linker_args.push(format!("target.{target}.linker=\"{linker}\""));
    }
    #[allow(unused_mut)]
    let mut feature_args: Vec<&str> = vec!["-F", "panic"]; // enable output structure sizes in panic messages during build
    for (rust_feature, _c_feature) in RUST_TO_C_FEATURES.entries() {
        if test_feature(rust_feature) {
            feature_args.push("-F");
            feature_args.push(rust_feature);
        }
    }

    let _ = Command::new("cargo")
        .arg("build")
        .args(feature_args)
        .args(linker_args)
        .arg("--target")
        .arg(target)
        .arg("--manifest-path")
        .arg(manifest_path)
        .stderr(stdio)
        .output()
        .unwrap();

    output_file_path
}

fn split_type_name(type_name: &str) -> (&str, Option<&str>, &str, &str) {
    let mut split = type_name.split('_');
    let prefix = split
        .next()
        .unwrap_or_else(|| panic!("Fist '_' not found in type name: {type_name}"));
    let cat = split
        .next()
        .unwrap_or_else(|| panic!("Second '_' not found in type name: {type_name}"));
    let category = if cat != "owned" && cat != "loaned" && cat != "moved" {
        None
    } else {
        Some(cat)
    };
    let postfix = split.next_back().expect("Type should end with '_t'");
    let prefix_cat_len = prefix.len() + 1 + category.map(|c| c.len() + 1).unwrap_or(0);
    let semantic = &type_name[prefix_cat_len..type_name.len() - postfix.len() - 1];
    (prefix, category, semantic, postfix)
}

fn generate_opaque_types() {
    let type_to_inner_field_name = HashMap::from([("z_id_t", "pub id")]);
    let current_folder = get_build_rs_path();
    let path_in = produce_opaque_types_data();
    let path_out = current_folder.join("./src/opaque_types/mod.rs");

    let data_in = std::fs::read_to_string(path_in).unwrap();
    let mut data_out = String::new();
    let mut docs = get_opaque_type_docs();

    let re = Regex::new(r"type: (\w+), align: (\d+), size: (\d+)").unwrap();
    for (_, [type_name, align, size]) in re.captures_iter(&data_in).map(|c| c.extract()) {
        let inner_field_name = type_to_inner_field_name.get(type_name).unwrap_or(&"_0");
        let (prefix, category, semantic, postfix) = split_type_name(type_name);
        let mut s = String::new();
        if category != Some("owned") {
            s += "#[derive(Copy, Clone)]\n";
        };
        s += format!(
            "#[repr(C, align({align}))]
#[rustfmt::skip]
pub struct {type_name} {{
    {inner_field_name}: [u8; {size}],
}}
"
        )
        .as_str();
        if category == Some("owned") {
            let moved_type_name = format!("{}_{}_{}_{}", prefix, "moved", semantic, postfix);
            // Note: owned type {type_name} should implement "Default" trait, this is
            // done by "decl_c_type!" macro in transmute module.
            s += format!(
                "#[repr(C)]
#[rustfmt::skip]
pub struct {moved_type_name} {{
    _this: {type_name},
}}

#[rustfmt::skip]
impl crate::transmute::TakeCType for {moved_type_name} {{
    type CType = {type_name};
    fn take_c_type(&mut self) -> Self::CType {{
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, {type_name}::gravestone())
    }}
}}

#[rustfmt::skip]
impl Drop for {type_name} {{
    fn drop(&mut self) {{
        use crate::transmute::{{RustTypeRef, Gravestone, IntoRustType}};
        let _ = std::mem::replace(self.as_rust_type_mut(), {type_name}::gravestone().into_rust_type());
    }}
}}
"
            )
            .as_str();
        }

        let doc = docs
            .remove(type_name)
            .unwrap_or_else(|| panic!("Failed to extract docs for opaque type: {type_name}"));
        for d in doc {
            data_out += &d;
            data_out += "\r\n";
        }
        data_out += &s;
    }
    // todo: in order to support rust features in opaque_types, we should respect features here.
    // I will remove it for a while, maybe we'll implement this later
    //for d in docs.keys() {
    //    panic!("Failed to find type information for opaque type: {d}");
    //}
    std::fs::write(path_out, data_out).unwrap();
}

fn get_opaque_type_docs() -> HashMap<String, Vec<String>> {
    let current_folder = get_build_rs_path();
    let path_in = current_folder.join("./build-resources/opaque-types/src/lib.rs");
    let re = Regex::new(r"(?m)^get_opaque_type_data!\(\s*(.*)\s*,\s*(\w+)\s*(,)?\s*\);").unwrap();
    let mut comments = Vec::new();
    let mut opaque_lines = Vec::new();
    let mut res = HashMap::new();

    for line in std::fs::read_to_string(path_in).unwrap().lines() {
        if line.starts_with("///") {
            comments.push(line.to_string());
            continue;
        }
        if line.starts_with("get_opaque_type_data!(") || !opaque_lines.is_empty() {
            opaque_lines.push(line);
        }
        if !opaque_lines.is_empty() && line.ends_with(");") {
            let joined_lines = std::mem::take(&mut opaque_lines).join("");
            let capture = re.captures(&joined_lines).expect("invalid opaque type");
            res.insert(capture[2].to_string(), std::mem::take(&mut comments));
        }
    }
    res
}

// See: https://github.com/rust-lang/cargo/issues/9661
// See: https://github.com/rust-lang/cargo/issues/545
fn cargo_target_dir() -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR should be set"));
    let profile = env::var("PROFILE").expect("PROFILE should be set");

    let mut target_dir = None;
    let mut out_dir_path = out_dir.as_path();
    while let Some(parent) = out_dir_path.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }
        out_dir_path = parent;
    }

    target_dir
        .expect("OUT_DIR should be a child of a PROFILE directory")
        .to_path_buf()
}
