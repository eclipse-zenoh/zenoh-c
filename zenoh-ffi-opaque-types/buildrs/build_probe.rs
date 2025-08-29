use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

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
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    // Collect targets and (optional) linkers
    let host = std::env::var("HOST").expect("HOST environment variable not set");
    let target = std::env::var("TARGET").expect("TARGET environment variable not set");
    let target_linker = std::env::var("RUSTC_LINKER").ok();
    let cross_target = std::env::var("CROSS_TARGET").ok();
    let cross_linker = std::env::var("CROSS_RUSTC_LINKER").ok();

    let mut plans: Vec<(String, Option<String>)> = if let Some(ref cross_target) = cross_target {
        vec![
            (host.clone(), None),
            (target.clone(), target_linker.clone()),
            (cross_target.clone(), cross_linker.clone()),
        ]
    } else {
        vec![(host.clone(), None), (target.clone(), target_linker.clone())]
    };
    // deduplicate plans
    plans.sort_unstable();
    plans.dedup();
    // Only 1 or 2 elements should remain after deduplication. Panic with current plan status
    // and values of environment variables read above
    if plans.len() > 2 {
        panic!("Seems like CROSS_TARGET variable doesn't match --target cargo parameter or CROSS_TARGET_LINKER variable doesn't match the linker for the target\n\
        The variables are:\n\
        HOST: {}\n\
        TARGET: {}\n\
        CROSS_TARGET: {}\n\
        RUSTC_LINKER: {}\n\
        CROSS_RUSTC_LINKER: {}",
        host, target, cross_target.unwrap_or_default(), target_linker.unwrap_or_default(), cross_linker.unwrap_or_default());
    }

    let mut outputs: HashMap<String, PathBuf> = HashMap::new();
    for (target, linker) in plans {
        let out_path = run_cargo_build_for_target(
            &probe_manifest,
            &opaque_root,
            &profile,
            &target,
            linker.as_deref(),
        );
        outputs.insert(target, out_path);
    }
    outputs
}

fn run_cargo_build_for_target(
    probe_manifest: &Path,
    opaque_root: &Path,
    profile: &str,
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

    // Prepare output file under <cargo_target_dir>/<target>/<profile>/probe_build_output.txt
    let out_dir = cargo_target_dir.join(target).join(profile);
    let out_file = out_dir.join("probe_build_output.txt");
    if let Some(parent) = out_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    // Also prepare a file to store the executed command
    let cmd_file = out_dir.join("probe_build_cmd.txt");
    let file = std::fs::File::create(&out_file)
        .unwrap_or_else(|e| panic!("Failed to create output file {}: {e}", out_file.display()));
    let file_clone = file.try_clone().unwrap_or_else(|e| {
        panic!(
            "Failed to clone output file handle {}: {e}",
            out_file.display()
        )
    });

    // Build a human-readable command string and store it
    let mut cmd_pieces: Vec<String> = vec!["cargo".into(), "build".into()];
    cmd_pieces.extend(feature_args.iter().cloned());
    cmd_pieces.extend(cfg_args.iter().cloned());
    cmd_pieces.push("--target".into());
    cmd_pieces.push(target.into());
    // Use explicit profile to mirror the parent build when release; for debug, omit flag
    if profile == "release" {
        cmd_pieces.push("--release".into());
    }
    cmd_pieces.push("--manifest-path".into());
    cmd_pieces.push(probe_manifest.display().to_string());
    cmd_pieces.push("--target-dir".into());
    cmd_pieces.push(cargo_target_dir.display().to_string());

    // Quote pieces containing whitespace for readability
    let cmd_line = cmd_pieces
        .iter()
        .map(|s| {
            if s.chars().any(char::is_whitespace) {
                format!("\"{}\"", s)
            } else {
                s.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    let _ = std::fs::write(&cmd_file, cmd_line.as_bytes());

    // Prepare and run the process
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build")
        .args(&feature_args)
        .args(&cfg_args)
        .arg("--target")
        .arg(target);
    if profile == "release" {
        cmd.arg("--release");
    }
    cmd.arg("--manifest-path")
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
