use std::collections::HashMap;
use std::path::PathBuf;

use crate::buildrs::get_out_opaque_types;

/// Execute `cargo build` for the generated probe project and return a map of
/// target triple -> path to a file with combined stdout/stderr output.
///
/// Behavior:
/// - Always enables the `panic` feature and mirrors the enabled zenoh features
///   from the parent build via `crate::features()`.
/// - Builds for the primary TARGET and, if present and different, for CROSS_TARGET.
/// - If RUSTC_LINKER/CROSS_RUSTC_LINKER are set, passes them via `--config target.<triple>.linker=...`.
/// - Uses `--target-dir` pointing to `get_out_opaque_types()` to keep artifacts under the opaque-types dir.
/// - Never panics on non-zero cargo exit status; instead, captures and returns the compiler output.
pub fn build_probe_project() -> HashMap<String, PathBuf> {
    let opaque_root = get_out_opaque_types();
    let probe_manifest = opaque_root.join("probe").join("Cargo.toml");

    // Collect targets and (optional) linkers
    let host_target = std::env::var("TARGET").expect("TARGET is not set");
    let host_linker = std::env::var("RUSTC_LINKER").ok();

    let mut plans: Vec<(String, Option<String>)> = vec![(host_target.clone(), host_linker)];
    if let Ok(cross_target) = std::env::var("CROSS_TARGET") {
        if cross_target != host_target {
            let cross_linker = std::env::var("CROSS_RUSTC_LINKER").ok();
            plans.push((cross_target, cross_linker));
        }
    }

    let mut outputs: HashMap<String, PathBuf> = HashMap::new();
    for (target, linker) in plans {
        let out_path = run_cargo_build_for_target(&probe_manifest, &opaque_root, &target, linker.as_deref());
        outputs.insert(target, out_path);
    }
    outputs
}

fn run_cargo_build_for_target(
    probe_manifest: &PathBuf,
    opaque_root: &PathBuf,
    target: &str,
    linker: Option<&str>,
) -> PathBuf {
    // Build feature args: enable `panic` and mirror enabled features from parent build
    let mut feature_args: Vec<String> = vec!["-F".into(), "panic".into()];
    for f in prebindgen::get_enabled_features() {
        feature_args.push("-F".into());
        feature_args.push((*f).to_string());
    }

    // Optional linker via --config target.<triple>.linker="<linker>"
    let mut cfg_args: Vec<String> = Vec::new();
    if let Some(linker) = linker {
        cfg_args.push("--config".into());
        cfg_args.push(format!("target.{target}.linker=\"{linker}\""));
    }

    // Determine the cargo target-dir: always use <opaque_root>/target to match cargo's standard structure
    // regardless of whether OPAQUE_TYPES_BUILD_DIR is set.
    let cargo_target_dir = opaque_root.join("target");

    // Prepare output file under <cargo_target_dir>/<target>/probe_build_output.txt
    let out_dir = cargo_target_dir.join(target);
    let out_file = out_dir.join("probe_build_output.txt");
    if let Some(parent) = out_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    // Also prepare a file to store the executed command
    let cmd_file = out_dir.join("probe_build_cmd.txt");
    let file = std::fs::File::create(&out_file)
        .unwrap_or_else(|e| panic!("Failed to create output file {}: {e}", out_file.display()));
    let file_clone = file
        .try_clone()
        .unwrap_or_else(|e| panic!("Failed to clone output file handle {}: {e}", out_file.display()));

    // Build a human-readable command string and store it
    let mut cmd_pieces: Vec<String> = vec!["cargo".into(), "build".into()];
    cmd_pieces.extend(feature_args.iter().cloned());
    cmd_pieces.extend(cfg_args.iter().cloned());
    cmd_pieces.push("--target".into());
    cmd_pieces.push(target.into());
    cmd_pieces.push("--manifest-path".into());
    cmd_pieces.push(probe_manifest.display().to_string());
    cmd_pieces.push("--target-dir".into());
    cmd_pieces.push(cargo_target_dir.display().to_string());

    // Quote pieces containing whitespace for readability
    let cmd_line = cmd_pieces
        .iter()
        .map(|s| if s.chars().any(char::is_whitespace) { format!("\"{}\"", s) } else { s.clone() })
        .collect::<Vec<_>>()
        .join(" ");
    let _ = std::fs::write(&cmd_file, cmd_line.as_bytes());

    // Prepare and run the process
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build")
        .args(&feature_args)
        .args(&cfg_args)
        .arg("--target")
        .arg(target)
        .arg("--manifest-path")
        .arg(probe_manifest)
        .arg("--target-dir")
    .arg(&cargo_target_dir)
        .stdout(std::process::Stdio::from(file))
        .stderr(std::process::Stdio::from(file_clone));

    // Run; regardless of success/failure, the output is in out_file
    let _status = cmd
        .status()
        .unwrap_or_else(|e| panic!("Failed to execute cargo build for {target}: {e}"));
    out_file
}
