use std::path::Path;

pub fn get_out_dir() -> std::path::PathBuf {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    std::path::Path::new(&out_dir).to_path_buf()
}

pub fn get_out_opaque_types() -> std::path::PathBuf {
    match std::env::var("OPAQUE_TYPES_BUILD_DIR") {
        Ok(opaque_types_build_dir) => {
            println!(
                "cargo:warning=OPAQUE_TYPES_BUILD_DIR = {}",
                opaque_types_build_dir
            );
            opaque_types_build_dir.into()
        }
        Err(_) => get_target_dir().join("opaque-types"),
    }
}

pub fn get_target_dir() -> std::path::PathBuf {
    // OUT_DIR typically looks like: target/<profile>/build/<crate-hash>/out
    // We need to go 4 levels up to reach the target directory root
    let mut p = get_out_dir();
    for _ in 0..4 {
        p = p
            .parent()
            .unwrap_or_else(|| panic!("Invalid OUT_DIR, cannot get target dir from {}", get_out_dir().display()))
            .to_path_buf();
    }
    p
}

pub fn write_if_changed(path: &Path, content: &[u8]) -> std::io::Result<bool> {
    use std::io::Read;
    let mut needs_write = true;
    if let Ok(mut f) = std::fs::File::open(path) {
        let mut existing = Vec::new();
        if f.read_to_end(&mut existing).is_ok() && existing == content {
            needs_write = false;
        }
    }
    if needs_write {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
    }
    Ok(needs_write)
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

