use std::{collections::HashMap, path::Path};

use crate::buildrs::get_out_opaque_types;

/// Execute `cargo check` for the generated probe project and return a map of
/// target triple -> (stdout, stderr) of the cargo check command.
///
/// Behavior:
/// - Always enables the `panic` feature and mirrors the enabled zenoh features
///   from the parent build via `crate::features()`.
/// - Builds for the primary TARGET and, if present and different, for CROSS_TARGET.
/// - If RUSTC_LINKER/CROSS_RUSTC_LINKER are set, passes them via `--config target.<triple>.linker=...`.
/// - Uses `--target-dir` pointing to `get_out_opaque_types()` to keep artifacts under the opaque-types dir.
/// - Never panics on non-zero cargo exit status; instead, captures and returns the compiler output.
pub fn build_probe_project() -> HashMap<String, (String, String)> {
    let opaque_root = get_out_opaque_types();
    let probe_manifest = opaque_root.join("probe").join("Cargo.toml");
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    // Collect targets and (optional) linkers
    let host = std::env::var("HOST").expect("HOST environment variable not set");
    let target = std::env::var("TARGET").expect("TARGET environment variable not set");
    let target_linker = std::env::var("RUSTC_LINKER").ok().filter(|s| !s.is_empty());
    let cross_target = std::env::var("CROSS_TARGET").ok().filter(|s| !s.is_empty());
    let cross_linker = std::env::var("CROSS_RUSTC_LINKER")
        .ok()
        .filter(|s| !s.is_empty());

    let mut plans: Vec<(String, Option<String>)> = if let Some(ref cross_target) = cross_target {
        vec![
            (host.clone(), None),
            (target.clone(), target_linker.clone()),
            (cross_target.clone(), cross_linker.clone()),
        ]
    } else {
        vec![
            (host.clone(), None),
            (target.clone(), target_linker.clone()),
        ]
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

    let mut outputs = HashMap::new();
    for (target, linker) in plans {
        let output = run_cargo_build_for_target(
            &probe_manifest,
            &opaque_root,
            &profile,
            &target,
            linker.as_deref(),
        );
        outputs.insert(target, output);
    }
    outputs
}

fn run_cargo_build_for_target(
    probe_manifest: &Path,
    opaque_root: &Path,
    profile: &str,
    target: &str,
    linker: Option<&str>,
) -> (String, String) {
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
    let _ = std::fs::create_dir_all(&out_dir);
    let stdout_file = out_dir.join("probe_build_stdout.txt");
    let stderr_file = out_dir.join("probe_build_stderr.txt");
    let cmd_file = out_dir.join("probe_build_cmd.txt");

    // Prepare and run the process
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("check")
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
        .arg(&cargo_target_dir);

    let cmd_line = format!("{:?}", cmd);
    let _ = std::fs::write(&cmd_file, cmd_line.as_bytes());

    // Run; regardless of success/failure, the output is in out_file
    let output = cmd
        .output()
        .unwrap_or_else(|e| panic!("Failed to execute cargo check for {target}: {e}"));

    // Store the output to the files
    let _ = std::fs::write(&stdout_file, &output.stdout);
    let _ = std::fs::write(&stderr_file, &output.stderr);

    println!(
        "cargo:warning=Generated probe check output for {}: {} (stdout), {} (stderr)",
        target,
        stdout_file.display(),
        stderr_file.display()
    );

    let output_stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let output_stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output_stdout, output_stderr)
}
