use std::collections::HashMap;
use std::path::PathBuf;
use phf::phf_map;
use regex::Regex;

use super::{split_type_name, test_feature};

static RUST_TO_C_FEATURES: phf::Map<&'static str, &'static str> = phf_map! {
    "unstable" => "Z_FEATURE_UNSTABLE_API",
    "shared-memory" => "Z_FEATURE_SHARED_MEMORY",
    "auth_pubkey" => "Z_FEATURE_AUTH_PUBKEY",
    "auth_usrpwd" => "Z_FEATURE_AUTH_USRPWD",
    "transport_multilink" => "Z_FEATURE_TRANSPORT_MULTILINK",
    "transport_compression" => "Z_FEATURE_TRANSPORT_COMPRESSION",
    "transport_quic"  => "Z_FEATURE_TRANSPORT_QUIC",
    "transport_tcp" =>  "Z_FEATURE_TRANSPORT_TCP",
    "transport_tls" =>  "Z_FEATURE_TRANSPORT_TLS",
    "transport_udp" =>  "Z_FEATURE_TRANSPORT_UDP",
    "transport_unixsock-stream" =>  "Z_FEATURE_TRANSPORT_UNIXSOCK_STREAM",
    "transport_ws" =>  "Z_FEATURE_TRANSPORT_WS",
    "transport_vsock" => "Z_FEATURE_VSOCK"
};

fn get_build_rs_path() -> PathBuf {
    let file_path = file!();
    let mut path_buf = PathBuf::new();
    path_buf.push(file_path);
    path_buf.parent().unwrap().to_path_buf()
}

fn produce_opaque_types_data() -> PathBuf {
    let target = std::env::var("TARGET").unwrap();
    let linker = std::env::var("RUSTC_LINKER").unwrap_or_default();
    let current_folder = get_build_rs_path();
    let manifest_path = current_folder.join("./build-resources/opaque-types/Cargo.toml");
    let output_file_path = current_folder.join("./.build_resources_opaque_types.txt");
    let out_file = std::fs::File::create(output_file_path.clone()).unwrap();
    let stdio = std::process::Stdio::from(out_file);

    let mut linker_args = Vec::<String>::new();
    if !linker.is_empty() {
        linker_args.push("--config".to_string());
        linker_args.push(format!("target.{target}.linker=\"{linker}\""));
    }
    #[allow(unused_mut)]
    let mut feature_args: Vec<&str> = vec!["-F", "panic"];
    for (rust_feature, _c_feature) in RUST_TO_C_FEATURES.entries() {
        if test_feature(rust_feature) {
            feature_args.push("-F");
            feature_args.push(rust_feature);
        }
    }

    let _ = std::process::Command::new("cargo")
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

pub fn generate_opaque_types() {
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
