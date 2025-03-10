use std::{env, path::PathBuf};

use phf::phf_map;

pub mod opaque_types;
pub mod cbindgen;

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


pub fn split_type_name(type_name: &str) -> (&str, Option<&str>, &str, &str) {
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

pub static RUST_TO_C_FEATURES: phf::Map<&'static str, &'static str> = phf_map! {
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

pub fn test_feature(feature: &str) -> bool {
    match feature {
        #[cfg(feature = "shared-memory")]
        "shared-memory" => true,
        #[cfg(feature = "unstable")]
        "unstable" => true,
        #[cfg(feature = "auth_pubkey")]
        "auth_pubkey" => true,
        #[cfg(feature = "auth_usrpwd")]
        "auth_usrpwd" => true,
        #[cfg(feature = "transport_multilink")]
        "transport_multilink" => true,
        #[cfg(feature = "transport_compression")]
        "transport_compression" => true,
        #[cfg(feature = "transport_quic")]
        "transport_quic" => true,
        #[cfg(feature = "transport_tcp")]
        "transport_tcp" => true,
        #[cfg(feature = "transport_tls")]
        "transport_tls" => true,
        #[cfg(feature = "transport_udp")]
        "transport_udp" => true,
        #[cfg(feature = "transport_unixsock-stream")]
        "transport_unixsock-stream" => true,
        #[cfg(feature = "transport_ws")]
        "transport_ws" => true,
        #[cfg(feature = "transport_vsock")]
        "transport_vsock" => true,
        _ => false,
    }
}



