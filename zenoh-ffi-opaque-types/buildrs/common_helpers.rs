use std::path::Path;

pub fn get_build_rs_path() -> std::path::PathBuf {
    let file_path = file!();
    let mut path_buf = std::path::PathBuf::new();
    path_buf.push(file_path);
    path_buf.parent().unwrap().to_path_buf()
}

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
