use std::collections::HashMap;

use regex::Regex;

/// Parse the outputs of cargo probe builds and extract size/alignment entries.
///
/// Input: stdout and stderr of cargo check command.
/// Output: HashMap<type_name, (size, alignment)>
///
/// Behavior:
/// - For each target, ensures there are compilation errors and that they match our expected pattern
///   'type: <name>, align: <n>, size: <m>' that appear inside the panic message.
/// - If there are zero Rust errors in the file, panics with a helpful message.
/// - If there are Rust errors but none match the pattern, panics similarly.
/// - If the number of matched entries differs from the number of Rust errors, panics.
pub fn parse_probe_result(target: &str, stdout: &str, stderr: &str) -> HashMap<String, (u64, u64)> {
    let mut map: HashMap<String, (u64, u64)> = HashMap::new();
    let re_sizes = Regex::new(r"type: (\w+), align: (\d+), size: (\d+)").expect("valid regex");

    // Cargo error detection first: if the first non-empty line starts with 'error:',
    // treat this as a cargo error unrelated to our intentional probe panics.
    if let Some(first) = stderr.lines().find(|l| !l.trim().is_empty()) {
        if first.trim_start().starts_with("error:") {
            panic!(
                "Cargo error encountered while building probe for target {}\nStdout:\n{}\n\nStderr:\n{}",
                target,
                stdout, stderr

            );
        }
    }

    // Count Rust compiler errors count by error id:
    // fill hashmap by error integer id:
    let mut error_count_map: HashMap<usize, usize> = HashMap::new();
    let regex = Regex::new(r"error\[E(\d+)\]").unwrap();
    for line in stderr.lines() {
        if let Some(captures) = regex.captures(line) {
            let error_id: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            *error_count_map.entry(error_id).or_insert(0) += 1;
        }
    }

    // Panic if error other than E0080 (evaluation of constant value failed) found
    if error_count_map.keys().any(|&id| id != 80) {
        panic!(
            "Unexpected Rust errors found in probe output for target {}\nStdout:\n{}\n\nStderr:\n{}",
            target,           stdout, stderr
        );
    }

    // Panic if there are no E0080 errors
    let total_error_count = *error_count_map.get(&80).unwrap_or(&0);
    if total_error_count == 0 {
        panic!(
            "No E0080 errors found in probe output for target {}\nStdout:\n{}\n\nStderr:\n{}",
            target, stdout, stderr
        );
    }

    let mut matched_count = 0usize;
    for cap in re_sizes.captures_iter(&stderr) {
        matched_count += 1;
        let type_name = cap.get(1).unwrap().as_str().to_string();
        let align: u64 = cap.get(2).unwrap().as_str().parse().unwrap();
        let size: u64 = cap.get(3).unwrap().as_str().parse().unwrap();
        map.insert(type_name, (size, align));
    }

    if matched_count == 0 {
        panic!(
            "Probe output contains errors but none matched size/alignment pattern for target {}.\nStdout:\n{}\n\nStderr:\n{}",
            target, stdout, stderr
        );
    }

    // It can happen that matched count is greater than total error count: I've observed repeating of the error messages without exact line:
    // ```
    // error: could not compile `zenoh-ffi-opaque-types` (lib) due to 66 previous errors
    // warning: build failed, waiting for other jobs to finish...
    // evaluation panicked: type: z_loaned_condvar_t, align: 8, size: 16
    // ```
    // So panic only if matched count is less than error count: this means that there is some error not related to the probe.
    if matched_count < total_error_count {
        panic!(
            "Mismatch in probe output: found {total_error_count} E0080 errors but matched only {matched_count} size/alignment entries for target {}.\nStdout:\n{}\n\nStderr:\n{}",
            target, stdout, stderr
        );
    }

    map
}
