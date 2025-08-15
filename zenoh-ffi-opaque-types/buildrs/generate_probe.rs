use std::path::Path;

use crate::buildrs::{get_out_opaque_types, write_if_changed};

pub fn generate_probe_project() {
    // Create target/opaque-types/probe structure
    let opaque_root = get_out_opaque_types();
    let probe_dir = opaque_root.join("probe");
    let probe_src_dir = probe_dir.join("src");
    std::fs::create_dir_all(&probe_src_dir).expect("Failed to create probe/src directory");

    // 1) Workspace Cargo.toml at opaque-types/
    let workspace_manifest = opaque_root.join("Cargo.toml");
    let workspace_toml = build_workspace_manifest_content();
    let _ = write_if_changed(&workspace_manifest, workspace_toml.as_bytes())
        .expect("Failed to write workspace Cargo.toml");

    // 2) Copy project's Cargo.toml into probe/
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let project_manifest = std::path::Path::new(&manifest_dir).join("Cargo.toml");
    let probe_manifest = probe_dir.join("Cargo.toml");
    let filtered = filter_probe_manifest(&project_manifest)
        .unwrap_or_else(|e| panic!("Failed to read and filter project Cargo.toml: {e}"));
    let _ = write_if_changed(&probe_manifest, filtered.as_bytes())
        .unwrap_or_else(|e| panic!("Failed to write filtered probe Cargo.toml: {e}"));

    // 3) Copy src/probe.rs to probe/src/lib.rs
    let src_probe = std::path::Path::new(&manifest_dir).join("src/probe.rs");
    let dst_lib = probe_src_dir.join("lib.rs");
    copy_file_if_changed(&src_probe, &dst_lib)
        .unwrap_or_else(|e| panic!("Failed to copy src/probe.rs to probe/src/lib.rs: {e}"));

    // 4) Copy Cargo.lock to opaque-types/ from CARGO_LOCK env var
    let cargo_lock_path = std::env::var("CARGO_LOCK").unwrap_or_else(|_| {
        panic!("CARGO_LOCK environment variable must be set to the path of Cargo.lock")
    });
    let cargo_lock_src = std::path::PathBuf::from(cargo_lock_path);
    let cargo_lock_dst = opaque_root.join("Cargo.lock");
    copy_file_if_changed(&cargo_lock_src, &cargo_lock_dst)
        .unwrap_or_else(|e| panic!("Failed to copy Cargo.lock: {e}"));
}

fn copy_file_if_changed(src: &Path, dst: &Path) -> std::io::Result<bool> {
    let content = std::fs::read(src)?;
    write_if_changed(dst, &content)
}

fn build_workspace_manifest_content() -> String {
    // dummy workspace replacing the real one for generated `probe` package
    r#"[workspace]
    resolver = "2"
    members = [
        "probe"
    ]

    [workspace.package]
    version = "0.0.1"
    repository = ""
    homepage = ""
    authors = []
    edition = "2021"
    license = ""
    categories = []
    "#
    .to_string()
}

fn filter_probe_manifest(src: &Path) -> std::io::Result<String> {
    let text = std::fs::read_to_string(src)?;
    let mut out = String::new();
    let mut skip_build_deps = false;
    for line in text.lines() {
        let t = line.trim_start();
        if t.starts_with("build = ") {
            // drop build script to avoid recursion and missing file in probe
            continue;
        }
        if t.starts_with("[build-dependencies]") {
            // Skip the entire build-dependencies section
            skip_build_deps = true;
            continue;
        }
        if skip_build_deps {
            if t.starts_with('[') {
                skip_build_deps = false; // next section begins
                out.push_str(line);
                out.push('\n');
            } else {
                continue;
            }
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    Ok(out)
}
