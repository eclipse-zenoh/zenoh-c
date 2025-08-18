use std::collections::HashMap;
use regex::Regex;

/// Parse the outputs of cargo probe builds and extract size/alignment entries.
///
/// Input: path to probe_build_output.txt (as produced by build_probe_project) for a single target.
/// Output: HashMap<type_name, (size, alignment)>
///
/// Behavior:
/// - For each target, ensures there are compilation errors and that they match our expected pattern
///   'type: <name>, align: <n>, size: <m>' that appear inside the panic message.
/// - If there are zero Rust errors in the file, panics with a helpful message.
/// - If there are Rust errors but none match the pattern, panics similarly.
/// - If the number of matched entries differs from the number of Rust errors, panics.
pub fn parse_probe_result(path: &std::path::PathBuf) -> HashMap<String, (u64, u64)> {
    let mut map: HashMap<String, (u64, u64)> = HashMap::new();
    let re_sizes = Regex::new(r"type: (\w+), align: (\d+), size: (\d+)").expect("valid regex");

    let data = std::fs::read_to_string(path).unwrap_or_else(|e| {
        panic!(
            "Failed to read probe output at {}: {e}",
            path.display()
        )
    });

    // Cargo error detection first: if the first non-empty line starts with 'error:',
    // treat this as a cargo error unrelated to our intentional probe panics.
    if let Some(first) = data.lines().find(|l| !l.trim().is_empty()) {
        if first.trim_start().starts_with("error:") {
            panic!(
                "Cargo error encountered while building probe.\nPath: {}\n\nOutput:\n{}",
                path.display(),
                data
            );
        }
    }

    // Count total Rust compiler error lines
    let total_error_count = data
        .lines()
        .filter(|l| l.starts_with("error[E"))
        .count();

    if total_error_count == 0 {
        panic!(
            "No Rust errors found in probe output.\nPath: {}\n\nOutput:\n{}",
            path.display(),
            data
        );
    }

    let mut matched_count = 0usize;
    for cap in re_sizes.captures_iter(&data) {
        matched_count += 1;
        let type_name = cap.get(1).unwrap().as_str().to_string();
        let align: u64 = cap.get(2).unwrap().as_str().parse().unwrap();
        let size: u64 = cap.get(3).unwrap().as_str().parse().unwrap();
        map.insert(type_name, (size, align));
    }

    if matched_count == 0 {
        panic!(
            "Probe output contains errors but none matched size/alignment pattern.\nPath: {}\n\nOutput:\n{}",
            path.display(),
            data
        );
    }

    if matched_count != total_error_count {
        panic!(
            "Mismatch in probe output: found {total_error_count} errors but matched {matched_count} size/alignment entries.\nPath: {}\n\nOutput:\n{}",
            path.display(),
            data
        );
    }

    map
}
