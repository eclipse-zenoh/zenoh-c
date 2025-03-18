use std::{collections::BTreeSet, env, path::PathBuf};

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

pub fn features() -> BTreeSet<&'static str> {
    zenoh::FEATURES.split(" zenoh/").collect()
}

pub fn test_feature(feature: &str) -> bool {
    zenoh::FEATURES.contains(format!(" zenoh/{feature}").as_str())
}

// See: https://github.com/rust-lang/cargo/issues/9661
// See: https://github.com/rust-lang/cargo/issues/545
pub fn cargo_target_dir() -> PathBuf {
    // OUT_DIR is a path to the directory where the build script writes its output.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR should be set"));
    // PROFILE is the profile that the build script is being run for. This will be one of "debug" or "release".
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
