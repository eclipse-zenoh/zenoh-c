use std::{
    collections::HashMap, path::{Path, PathBuf}
};

use regex::Regex;

use super::common_helpers::{features, split_type_name};
use crate::{get_build_rs_path, get_out_rs_path};

pub fn generate_opaque_types(target: &str, path_out: &Path, prebindgen: Option<bool>) {
    let type_to_inner_field_name = HashMap::from([("z_id_t", "pub id")]);
    let (command, path_in) = produce_opaque_types_data(target);

    let data_in = std::fs::read_to_string(path_in).unwrap();
    // check if message begins with "error:", excluding spaces
    if data_in.trim_start().starts_with("error:") {
        panic!(
            "Failed to generate opaque types due to cargo error:\n\nCommand executed:\n\n{command}\n\nCargo output:\n\n{data_in}"
        );
    }
    let mut data_out = String::new();
    let mut docs = get_opaque_type_docs();

    // Count the total number of errors in the input data
    let total_error_count = data_in
        .lines()
        .filter(|line| line.starts_with("error[E"))
        .count();

    // Scan for type size and layout information which is generated as compilation errors
    let mut good_error_count = 0;
    let re = Regex::new(r"type: (\w+), align: (\d+), size: (\d+)").unwrap();
    for (_, [type_name, align, size]) in re.captures_iter(&data_in).map(|c| c.extract()) {
        good_error_count += 1;
        let inner_field_name = type_to_inner_field_name.get(type_name).unwrap_or(&"_0");
        let (prefix, category, semantic, postfix) = split_type_name(type_name);
        // - If Option<false> is passed, the type is passed to both the compiler
        // and prebindgen (#[prebindgen(skip = false)])
        // - If Option<true> is passed, attribute is `#[prebindgen(skip = true)]`
        // which means that the type is not passed to the compiler, but prebindgen data is generated.
        // In this case, only type itself is needed, the traits should be skipped to avoid
        // conflicts with second generation with "None" option.
        // - If None is passed, the prebindgen attribute is not generated
        let skip_traits = prebindgen.unwrap_or(false);
        let prebindgen_attr = if let Some(skip) = prebindgen {
            format!("#[prebindgen(\"types\", skip = {skip})]\n")
        } else {
            String::new()
        };
        let mut s = prebindgen_attr.clone();
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
            s += prebindgen_attr.as_str();
            s += format!(
                "#[repr(C)]
#[rustfmt::skip]
pub struct {moved_type_name} {{
    _this: {type_name},
}}
"
            )
            .as_str();

            if !skip_traits {
                s += format!(
                    "#[rustfmt::skip]
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

    if good_error_count == 0 {
        panic!(
            "Failed to generate opaque types: no valid type information found in the input data\n\nCommand executed:\n\n{command}\n\nCompiler output:\n\n{data_in}"
        );
    }

    if good_error_count != total_error_count {
        panic!(
            "Failed to generate opaque types: there are {total_error_count} errors in the input data, but only {good_error_count} of them were processed as information about opaque types\n\nCommand executed:\n\n{command}\n\nCompiler output:\n\n{data_in}"
        );
    }

    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path_out)
        .unwrap();
    file.write_all(data_out.as_bytes()).unwrap();
}

// Copy manifest and lock files to output directory, modify manifest to point to original source
fn copy_cargo_files_to_out_dir(target: &str) -> PathBuf {
    let project_root = project_root::get_project_root()
        .expect("Failed to get project root");
    panic!("Project root is located at: {}", project_root.display());
    let source_dir = get_build_rs_path().join("./build-resources/opaque-types");
    let source_manifest = source_dir.join("Cargo.toml");
    let source_lock = project_root.join("Cargo.lock");
    
    let out_dir = get_out_rs_path().join(target).join(".build_resources/opaque_types_manifest");
    std::fs::create_dir_all(&out_dir).unwrap();
    
    let dest_manifest = out_dir.join("Cargo.toml");
    
    // Read original manifest and modify it to use absolute path for source
    let mut manifest_content = std::fs::read_to_string(&source_manifest)
        .unwrap_or_else(|_| panic!("Failed to read manifest from {}", source_manifest.display()));
    
    // Add [lib] section with absolute path to original source
    let lib_path = source_dir.join("src/lib.rs").canonicalize()
        .unwrap_or_else(|_| panic!("Failed to canonicalize path {}", source_dir.join("src/lib.rs").display()));
    let lib_section = format!("\n[lib]\npath = \"{}\"\n", lib_path.display());
    manifest_content.push_str(&lib_section);
    
    std::fs::write(&dest_manifest, manifest_content)
        .unwrap_or_else(|_| panic!("Failed to write manifest to {}", dest_manifest.display()));
    
    // Copy Cargo.lock to the destination
    if source_lock.exists() {
        let dest_lock = out_dir.join("Cargo.lock");
        std::fs::copy(&source_lock, &dest_lock)
            .unwrap_or_else(|_| panic!("Failed to copy Cargo.lock to {}", dest_lock.display()));
        let dest_lock_sav = dest_lock.with_extension("sav");
        std::fs::copy(&source_lock, &dest_lock_sav)
            .unwrap_or_else(|_| panic!("Failed to copy Cargo.lock to {}", dest_lock_sav.display()));
    }
    
    dest_manifest
}

fn produce_opaque_types_data(target: &str) -> (String, PathBuf) {
    let linker = std::env::var("RUSTC_LINKER").unwrap_or_default();
    let manifest_path = copy_cargo_files_to_out_dir(target);
    let output_file_path = get_out_rs_path()
        .join(target)
        .join("build_resources_opaque_types.txt");
    std::fs::create_dir_all(output_file_path.parent().unwrap()).unwrap();
    let out_file = std::fs::File::create(output_file_path.clone()).unwrap();
    let stdio = std::process::Stdio::from(out_file);

    let mut linker_args = Vec::<String>::new();
    if !linker.is_empty() {
        linker_args.push("--config".to_string());
        linker_args.push(format!("target.{target}.linker=\"{linker}\""));
    }
    #[allow(unused_mut)]
    let mut feature_args: Vec<&str> = vec!["-F", "panic"];
    for feature in features().iter().filter(|f| !f.is_empty()) {
        feature_args.push("-F");
        feature_args.push(feature);
    }

    let mut command = std::process::Command::new("cargo");
    command
        .arg("build")
        .args(feature_args)
        .args(linker_args)
        .arg("--target")
        .arg(target)
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--target-dir")
        .arg(
            get_out_rs_path()
                .join(target)
                .join(".build_resources/opaque_types"),
        );
    let command_str = format!("{command:?}");
    let _ = command.stderr(stdio).output().unwrap();
    (command_str, output_file_path)
}

fn get_opaque_type_docs() -> HashMap<String, Vec<String>> {
    let current_folder = get_build_rs_path();
    let path_in = current_folder.join("./build-resources/opaque-types/src/lib.rs");
    let re = Regex::new(r"(?m)^get_opaque_type_data!\(\s*(.*)\s*,\s*(\w+)\s*(,)?\s*\);").unwrap();
    let mut comments = Vec::new();
    let mut opaque_lines = Vec::new();
    let mut res = HashMap::new();
    for line in std::fs::read_to_string(&path_in)
        .unwrap_or_else(|_| panic!("failed to read file {}", path_in.display()))
        .lines()
    {
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
