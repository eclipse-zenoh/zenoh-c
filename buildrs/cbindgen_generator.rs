use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, Read, Write},
    path::{Path, PathBuf},
};

use fs2::FileExt;
use phf::phf_map;
use regex::Regex;

use super::{
    common_helpers::{cargo_target_dir, split_type_name, test_feature},
    splitguide::{split_bindings, FuncArg, FunctionSignature},
};
use crate::get_out_rs_path;

const BUGGY_GENERATION_PATH: &str = "include/zenoh-gen-buggy.h";
const GENERATION_PATH: &str = "include/zenoh-gen.h";
const PREPROCESS_PATH: &str = "include/zenoh-cpp.h";

static RUST_TO_C_FEATURES: phf::Map<&'static str, &'static str> = phf_map! {
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

pub fn generate_c_headers() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config::from_root_or_default(crate_dir.clone());
    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .with_src(get_out_rs_path().join("./opaque_types.rs"))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(BUGGY_GENERATION_PATH);

    fix_cbindgen(BUGGY_GENERATION_PATH, GENERATION_PATH);
    std::fs::remove_file(BUGGY_GENERATION_PATH).unwrap();

    preprocess_header(GENERATION_PATH, PREPROCESS_PATH);
    create_generics_header(PREPROCESS_PATH, "include/zenoh_macros.h");
    std::fs::remove_file(PREPROCESS_PATH).unwrap();

    configure();
    let files = split_bindings(GENERATION_PATH).unwrap();
    text_replace(files.iter());

    fs_extra::copy_items(
        &["include"],
        cargo_target_dir(),
        &fs_extra::dir::CopyOptions::default().overwrite(true),
    )
    .expect("include should be copied to CARGO_TARGET_DIR");
}

fn fix_cbindgen(input: &str, output: &str) {
    let bindings = std::fs::read_to_string(input).expect("failed to open input file");
    let bindings = bindings.replace("\n#endif\n  ;", ";\n#endif");

    let mut out = File::create(output).expect("failed to open output file");
    out.write_all(bindings.as_bytes()).unwrap();
}

fn preprocess_header(input: &str, output: &str) {
    let parsed = process_feature_defines(input).expect("failed to open input file");
    let mut out = File::create(output).expect("failed to open output file");
    out.write_all(parsed.as_bytes()).unwrap();
}

fn create_generics_header(path_in: &str, path_out: &str) {
    let mut file_out = std::fs::File::options()
        .read(false)
        .write(true)
        .truncate(true)
        .append(false)
        .create(true)
        .open(path_out)
        .unwrap();

    file_out
        .write_all(
            "#pragma once
// clang-format off

"
            .as_bytes(),
        )
        .unwrap();

    //
    // C part
    //
    file_out
        .write_all(
            "
#ifndef __cplusplus

"
            .as_bytes(),
        )
        .unwrap();

    // Collect all function signatures to be wrapped by macros and verify that all necessary functions are present for each entity
    let (move_funcs, take_funcs) = make_move_take_signatures(path_in);
    let loan_funcs = find_loan_functions(path_in);
    let loan_mut_funcs = find_loan_mut_functions(path_in);
    let take_from_loaned_funcs = find_take_from_loaned_functions(path_in);
    let drop_funcs = find_drop_functions(path_in);
    let null_funcs = find_null_functions(path_in);
    let check_funcs = find_check_functions(path_in);
    let call_funcs = find_call_functions(path_in);
    let closure_constructors = find_closure_constructors(path_in);
    let recv_funcs = find_recv_functions(path_in);
    let clone_funcs = find_clone_functions(path_in);

    let drops = drop_funcs
        .iter()
        .map(|f| &f.entity_name)
        .collect::<HashSet<_>>();
    let moves = move_funcs
        .iter()
        .map(|f| &f.entity_name)
        .collect::<HashSet<_>>();
    let takes = take_funcs
        .iter()
        .map(|f| &f.entity_name)
        .collect::<HashSet<_>>();
    let nulls = null_funcs
        .iter()
        .map(|f| &f.entity_name)
        .collect::<HashSet<_>>();
    let checks = check_funcs
        .iter()
        .map(|f| &f.entity_name)
        .collect::<HashSet<_>>();

    let mut msgs = Vec::new();

    // More checks can be added here

    if drops != nulls {
        msgs.push(format!(
            "the list of z_xxx_drop and z_internal_xxx_null functions are different:\n missing z_internal_xxx_null for {:?}\n missing z_xxx_drop for {:?}",
            drops.difference(&nulls),
            nulls.difference(&drops)
        ));
    }

    if drops != checks {
        msgs.push(format!(
            "the list of z_xxx_drop and z_internal_xxx_check functions are different:\n missing z_internal_xxx_check for {:?}\n missing z_xxx_drop for {:?}",
            drops.difference(&checks),
            checks.difference(&drops)
        ));
    }

    if drops != moves {
        msgs.push(format!(
            "the list of z_xxx_drop and z_xxx_move functions are different:\n missing z_xxx_move for {:?}\n missing z_xxx_drop for {:?}",
            drops.difference(&moves),
            moves.difference(&drops)
        ));
    }

    if drops != takes {
        msgs.push(format!(
            "the list of z_xxx_drop and z_xxx_take functions are different:\n missing z_xxx_take for {:?}\n missing z_xxx_drop for {:?}",
            drops.difference(&takes),
            takes.difference(&drops)
        ));
    }

    if !msgs.is_empty() {
        panic!("Some functions are missing:\n{}", msgs.join("\n"));
    }

    let out = generate_move_functions_c(&move_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_loan_c(&loan_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_loan_mut_c(&loan_mut_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_drop_c(&drop_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_move_c(&move_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_null_c(&null_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_take_functions(&take_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_take_c(&take_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_take_from_loaned_c(&take_from_loaned_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_check_c(&check_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_call_c(&call_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_closure_c(&closure_constructors);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_recv_c(&recv_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_clone_c(&clone_funcs);
    file_out.write_all(out.as_bytes()).unwrap();

    //
    // C++ part
    //
    file_out
        .write_all("\n#else  // #ifndef __cplusplus\n".as_bytes())
        .unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_move_functions_cpp(&move_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_loan_cpp(&loan_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_loan_mut_cpp(&loan_mut_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_drop_cpp(&drop_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_move_cpp(&move_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_null_cpp(&null_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_take_functions(&take_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_take_cpp(&take_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_take_from_loaned_cpp(&take_from_loaned_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_check_cpp(&check_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_call_cpp(&call_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_closure_cpp(&closure_constructors);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_recv_cpp(&recv_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_clone_cpp(&clone_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_loan_to_owned_type_cpp(&[loan_funcs, loan_mut_funcs].concat());
    file_out.write_all(out.as_bytes()).unwrap();

    file_out
        .write_all("\n#endif  // #ifndef __cplusplus\n\n".as_bytes())
        .unwrap();
}

fn configure() {
    let mut file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .append(false)
        .create(true)
        .open("include/zenoh_configure.h")
        .unwrap();
    file.lock_exclusive().unwrap();

    let version = std::fs::read_to_string("version.txt").unwrap();
    let version = version.trim();
    let version_parts: Vec<&str> = version.split('.').collect();
    if version_parts.len() < 3 {
        panic!("Invalid version format: \"{}\" in file version.txt. Major.Minor.Patch parts are required", version);
    }
    let major = version_parts[0];
    let minor = version_parts[1];
    let patch = version_parts[2];
    let tweak = version_parts
        .get(3)
        .map(|val| format!(" {val}"))
        .unwrap_or("".to_string());
    file.write_all(
        format!(
            r#"#pragma once
#define ZENOH_C "{}"
#define ZENOH_C_MAJOR {}
#define ZENOH_C_MINOR {}
#define ZENOH_C_PATCH {}
#define ZENOH_C_TWEAK{}

#define TARGET_ARCH_{}
"#,
            version,
            major,
            minor,
            patch,
            tweak,
            std::env::var("CARGO_CFG_TARGET_ARCH")
                .unwrap()
                .to_uppercase()
        )
        .as_bytes(),
    )
    .unwrap();

    for (rust_feature, c_feature) in RUST_TO_C_FEATURES.entries() {
        if test_feature(rust_feature) {
            file.write_all(format!("#define {}\n", c_feature).as_bytes())
                .unwrap();
        }
    }
    fs2::FileExt::unlock(&file).unwrap();
}

fn text_replace(files: impl Iterator<Item = impl AsRef<Path>>) {
    for name in files {
        let path = PathBuf::from("include").join(name);

        // Read content
        let mut file = std::fs::File::options()
            .read(true)
            .create(false)
            .write(false)
            .open(&path)
            .unwrap();
        file.lock_exclusive().unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        fs2::FileExt::unlock(&file).unwrap();

        // Remove _T_ from enum variant name
        let buf = buf.replace("_T_", "_");
        // Replace _t_Tag from union variant name
        let buf = buf.replace("_t_Tag", "_tag_t");
        // Insert `ZENOHC_API` macro before `extern const`.
        // The cbindgen can prefix functions (see `[fn] prefix=...` in cbindgn.toml), but not extern variables.
        // So have to do it here.
        let buf = buf.replace("extern const", "ZENOHC_API extern const");

        // Overwrite content
        let mut file = std::fs::File::options()
            .read(false)
            .create(false)
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        file.lock_exclusive().unwrap();
        file.write_all(buf.as_bytes()).unwrap();
        fs2::FileExt::unlock(&file).unwrap();
    }
}

/// Evaluates conditional feature macros in the form #if (logical expression of define(FEATURE_NAME))
/// and removes the code under those that evaluate to false
/// Note: works only on single string conditional expressions
fn process_feature_defines(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(input_path)?;
    let lines = std::io::BufReader::new(file).lines();
    let mut out = String::new();
    let mut skip = false;
    let mut nest_level: usize = 0;
    for line in lines.map_while(Result::ok) {
        if line.starts_with("#ifdef") && skip {
            nest_level += 1;
        } else if line.starts_with("#endif") && skip {
            nest_level -= 1;
            skip = nest_level != 0;
            continue;
        } else if line.starts_with("#if ") {
            skip = skip || evaluate_c_defines_line(&line);
            if skip {
                nest_level += 1;
            }
        }
        if !skip {
            out += &line;
            out += "\n";
        }
    }

    Ok(out)
}

fn evaluate_c_defines_line(line: &str) -> bool {
    let mut s = line.to_string();
    for (rust_feature, c_feature) in RUST_TO_C_FEATURES.entries() {
        s = s.replace(
            &format!("defined({})", c_feature),
            match test_feature(rust_feature) {
                true => "true",
                false => "false",
            },
        );
    }

    s = s.replace("#if", "");
    match evalexpr::eval(&s) {
        Ok(v) => v == evalexpr::Value::from(false),
        Err(_) => panic!("Failed to evaluate {}", &s),
    }
}

fn make_move_take_signatures(path_in: &str) -> (Vec<FunctionSignature>, Vec<FunctionSignature>) {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"(\w+)_drop\(struct (\w+) \*(\w+)\);").unwrap();
    let mut move_funcs = Vec::<FunctionSignature>::new();
    let mut take_funcs = Vec::<FunctionSignature>::new();

    for (_, [func_name_prefix, arg_type, arg_name]) in
        re.captures_iter(&bindings).map(|c| c.extract())
    {
        let (prefix, _, semantic, postfix) = split_type_name(arg_type);
        let z_owned_type = format!("{}_{}_{}_{}*", prefix, "owned", semantic, postfix);
        let z_moved_type = format!("{}_{}_{}_{}*", prefix, "moved", semantic, postfix);
        let move_f = FunctionSignature::new(
            semantic,
            &z_moved_type,
            func_name_prefix.to_string() + "_move",
            vec![FuncArg::new(&z_owned_type, arg_name)],
        );
        let take_f = FunctionSignature::new(
            semantic,
            "void",
            func_name_prefix.to_string() + "_take",
            vec![
                FuncArg::new(&z_owned_type, arg_name),
                FuncArg::new(&z_moved_type, "x"),
            ],
        );
        move_funcs.push(move_f);
        take_funcs.push(take_f);
    }
    (move_funcs, take_funcs)
}

fn find_loan_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"const struct (\w+) \*(\w+)_loan\(const struct (\w+) \*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [return_type, func_name, arg_type, arg_name]) in
        re.captures_iter(&bindings).map(|c| c.extract())
    {
        let (_, _, semantic, _) = split_type_name(arg_type);
        let f = FunctionSignature::new(
            semantic,
            &("const ".to_string() + return_type + "*"),
            func_name.to_string() + "_loan",
            vec![FuncArg::new(
                &("const ".to_string() + arg_type + "*"),
                arg_name,
            )],
        );
        res.push(f);
    }
    res
}

fn find_loan_mut_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"struct (\w+) \*(\w+)_loan_mut\(struct (\w+) \*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [return_type, func_name, arg_type, arg_name]) in
        re.captures_iter(&bindings).map(|c| c.extract())
    {
        let (_, _, semantic, _) = split_type_name(arg_type);
        let f = FunctionSignature::new(
            semantic,
            &(return_type.to_string() + "*"),
            func_name.to_string() + "_loan_mut",
            vec![FuncArg::new(&(arg_type.to_string() + "*"), arg_name)],
        );
        res.push(f);
    }
    res
}

fn find_take_from_loaned_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"void (\w+)_take_from_loaned\(struct (\w+) \*(\w+)").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [func_name, arg_type, arg_name]) in re.captures_iter(&bindings).map(|c| c.extract()) {
        let (prefix, _, semantic, postfix) = split_type_name(arg_type);
        let z_owned_type = format!("{}_{}_{}_{}*", prefix, "owned", semantic, postfix);
        let z_loaned_type = format!("{}_{}_{}_{}*", prefix, "loaned", semantic, postfix);
        let f = FunctionSignature::new(
            semantic,
            "void",
            func_name.to_string() + "_take_from_loaned",
            vec![
                FuncArg::new(&z_owned_type, arg_name),
                FuncArg::new(&z_loaned_type, "src"),
            ],
        );
        res.push(f);
    }
    res
}

fn find_drop_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"(.+?) +(\w+_drop)\(struct (\w+) \*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [return_type, func_name, arg_type, arg_name]) in
        re.captures_iter(&bindings).map(|c| c.extract())
    {
        // if necessary, other prefixes like "extern", "static", etc. can be removed here
        let return_type = return_type
            .split(' ')
            .filter(|x| *x != "ZENOHC_API")
            .collect::<Vec<_>>()
            .join(" ");
        let (_, _, semantic, _) = split_type_name(arg_type);
        let arg_type = arg_type.to_string() + "*";
        let f = FunctionSignature::new(
            semantic,
            return_type.as_str(),
            func_name.to_string(),
            vec![FuncArg::new(&arg_type, arg_name)],
        );
        res.push(f);
    }
    res
}

fn find_null_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r" (z.?_internal_\w+_null)\(struct (\w+) \*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [func_name, arg_type, arg_name]) in re.captures_iter(&bindings).map(|c| c.extract()) {
        let (_, _, semantic, _) = split_type_name(arg_type);
        let f = FunctionSignature::new(
            semantic,
            "void",
            func_name.to_string(),
            vec![FuncArg::new(&(arg_type.to_string() + "*"), arg_name)],
        );
        res.push(f);
    }
    res
}

fn find_check_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"bool (z.?_internal_\w+_check)\(const struct (\w+) \*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [func_name, arg_type, arg_name]) in re.captures_iter(&bindings).map(|c| c.extract()) {
        let (_, _, semantic, _) = split_type_name(arg_type);
        let f = FunctionSignature::new(
            semantic,
            "bool",
            func_name.to_string(),
            vec![FuncArg::new(
                &("const ".to_string() + arg_type + "*"),
                arg_name,
            )],
        );
        res.push(f);
    }
    res
}

fn find_call_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(
        r"(\w+) (\w+)_call\(const struct (\w+) \*(\w+),\s+(\w*)\s*struct (\w+) (\*?)(\w+)\);",
    )
    .unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (
        _,
        [return_type, func_name, closure_type, closure_name, arg_cv, arg_type, arg_deref, arg_name],
    ) in re.captures_iter(&bindings).map(|c| c.extract())
    {
        let arg_cv: String = if arg_cv.is_empty() {
            "".to_string()
        } else {
            "const ".to_string()
        };
        let (_, _, semantic, _) = split_type_name(arg_type);
        let f = FunctionSignature::new(
            semantic,
            return_type,
            func_name.to_string() + "_call",
            vec![
                FuncArg::new(&("const ".to_string() + closure_type + "*"), closure_name),
                FuncArg::new(&(arg_cv + arg_type + arg_deref), arg_name),
            ],
        );
        res.push(f);
    }
    res
}

fn find_closure_constructors(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(
        r"(\w+) (\w+)_closure_(\w+)\(struct\s+(\w+)\s+\*(\w+),\s+void\s+\(\*call\)(\([\s\w,\*]*\)),\s+void\s+\(\*drop\)(\(.*\)),\s+void\s+\*context\);"
    )
    .unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    let multiple_spaces = Regex::new(r"\s\s+").unwrap();
    for (
        _,
        [return_type, prefix, suffix, closure_type, closure_name, call_signature_raw, drop_signature],
    ) in re.captures_iter(&bindings).map(|c| c.extract())
    {
        let mut call_signature: String = call_signature_raw.to_string().replace("struct ", "");
        call_signature = call_signature.replace("enum ", "");
        call_signature = multiple_spaces
            .replace_all(&call_signature, " ")
            .to_string();
        let (_, _, semantic, _) = split_type_name(closure_type);
        let f = FunctionSignature::new(
            semantic,
            return_type,
            prefix.to_string() + "_closure_" + suffix,
            vec![
                FuncArg::new(&(closure_type.to_string() + "*"), closure_name),
                FuncArg::new(&("void (*call)".to_string() + &call_signature), "call"),
                FuncArg::new(&("void (*drop)".to_string() + drop_signature), "drop"),
                FuncArg::new("void*", "context"),
            ],
        );
        res.push(f);
    }
    res
}

fn find_recv_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"(\w+)\s+z_(\w+)_handler_(\w+)_recv\(const\s+struct\s+(\w+)\s+\*(\w+),\s+struct\s+(\w+)\s+\*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [return_type, handler_type, value_type, arg1_type, arg1_name, arg2_type, arg2_name]) in
        re.captures_iter(&bindings).map(|c| c.extract())
    {
        let (_, _, semantic, _) = split_type_name(arg1_type);
        let f = FunctionSignature::new(
            semantic,
            return_type,
            "z_".to_string() + handler_type + "_handler_" + value_type + "_recv",
            vec![
                FuncArg::new(&("const ".to_string() + arg1_type + "*"), arg1_name),
                FuncArg::new(&(arg2_type.to_string() + "*"), arg2_name),
            ],
        );
        res.push(f);
    }
    res
}

fn find_clone_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(
        r"(\w+)\s+z_(\w+)_clone\(struct\s+(\w+)\s+\*(\w+),\s+const\s+struct\s+(\w+)\s+\*(\w+)\);",
    )
    .unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [return_type, func_name, dst_type, dst_name, src_type, src_name]) in
        re.captures_iter(&bindings).map(|c| c.extract())
    {
        let (_, _, semantic, _) = split_type_name(dst_type);
        let f = FunctionSignature::new(
            semantic,
            return_type,
            "z_".to_string() + func_name + "_clone",
            vec![
                FuncArg::new(&(dst_type.to_string() + "*"), dst_name),
                FuncArg::new(&(src_type.to_string() + "*"), src_name),
            ],
        );
        res.push(f);
    }
    res
}

fn generate_generic_c(macro_func: &[FunctionSignature], generic_name: &str, decay: bool) -> String {
    let va_args = macro_func
        .iter()
        .any(|f| f.args.len() != macro_func[0].args.len());
    let mut args = macro_func
        .first()
        .unwrap_or_else(|| panic!("no sigatures found for building generic {generic_name}"))
        .args
        .iter()
        .map(|a| a.name.to_string())
        .collect::<Vec<_>>();
    let mut out = if va_args {
        format!(
            "#define {generic_name}({}, ...) \\
        _Generic(({})",
            args.join(", "),
            args[0],
        )
    } else {
        format!(
            "#define {generic_name}({}) \\
    _Generic(({})",
            args.join(", "),
            args[0],
        )
    };
    if decay {
        args[0] = format!("&{}", args[0]);
    }

    for func in macro_func {
        let owned_type = if decay {
            func.args[0].typename.clone().decay().typename
        } else {
            func.args[0].typename.typename.clone()
        };
        let func_name = &func.func_name;
        out += ", \\\n";
        out += &format!("        {owned_type} : {func_name}");
    }
    out += " \\\n";
    if va_args {
        out += &format!("    )({}, __VA_ARGS__)", args.join(", "));
    } else {
        out += &format!("    )({})", args.join(", "));
    }
    out
}

fn generate_generic_loan_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_loan", true)
}

fn generate_generic_loan_mut_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_loan_mut", true)
}

fn generate_generic_move_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_move", true)
}

fn generate_generic_take_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_take", false)
}

fn generate_generic_take_from_loaned_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_take_from_loaned", false)
}

fn generate_generic_drop_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_drop", false)
}

fn generate_generic_clone_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_clone", false)
}

fn generate_take_functions(macro_func: &[FunctionSignature]) -> String {
    let mut out = String::new();
    for sig in macro_func {
        let (prefix, _, semantic, _) = split_type_name(&sig.args[0].typename.typename);
        out += &format!(
            "static inline void {}({} {}, {} {}) {{ *{} = {}->_this; {}_internal_{}_null(&{}->_this); }}\n",
            sig.func_name,
            sig.args[0].typename.typename,
            sig.args[0].name,
            sig.args[1].typename.typename,
            sig.args[1].name,
            sig.args[0].name,
            sig.args[1].name,
            prefix,
            semantic,
            sig.args[1].name,
        );
    }
    out
}

fn generate_move_functions_c(macro_func: &[FunctionSignature]) -> String {
    let mut out = String::new();
    for sig in macro_func {
        out += &format!(
            "static inline {} {}({} x) {{ return ({})(x); }}\n",
            sig.return_type.typename,
            sig.func_name,
            sig.args[0].typename.typename,
            sig.return_type.typename
        );
    }
    out
}

fn generate_move_functions_cpp(macro_func: &[FunctionSignature]) -> String {
    let mut out = String::new();
    for sig in macro_func {
        out += &format!(
            "static inline {} {}({} x) {{ return reinterpret_cast<{}>(x); }}\n",
            sig.return_type.typename,
            sig.func_name,
            sig.args[0].typename.typename,
            sig.return_type.typename
        );
    }
    out
}

fn generate_generic_null_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_internal_null", false)
}

fn generate_generic_check_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_internal_check", true)
}

fn generate_generic_call_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_call", false)
}

fn generate_generic_closure_c(macro_func: &[FunctionSignature]) -> String {
    let mut out = "typedef void(*z_closure_drop_callback_t)(void *context);\n".to_string();
    for f in macro_func {
        let callback_typename = f.func_name.clone() + "_callback_t";
        let prototype = f.args[1]
            .clone()
            .typename
            .typename
            .replace(" (*call)", &format!("(*{})", &callback_typename));

        out += &format!("typedef {};\n", prototype);
    }
    out += "\n";
    out += &generate_generic_c(macro_func, "z_closure", false);
    out
}

fn generate_generic_recv_c(macro_func: &[FunctionSignature]) -> String {
    let try_recv_funcs: Vec<FunctionSignature> = macro_func
        .iter()
        .filter(|f| f.func_name.contains("try_recv"))
        .cloned()
        .collect();
    let recv_funcs: Vec<FunctionSignature> = macro_func
        .iter()
        .filter(|f| !f.func_name.contains("try_recv"))
        .cloned()
        .collect();
    generate_generic_c(&try_recv_funcs, "z_try_recv", false)
        + "\n\n"
        + generate_generic_c(&recv_funcs, "z_recv", false).as_str()
}

fn generate_generic_cpp(
    macro_func: &[FunctionSignature],
    generic_name: &str,
    decay: bool,
) -> String {
    let mut out = "".to_owned();

    let (body_start, body_end) = if macro_func.iter().any(|f| f.args.len() > 1) {
        ("\n    ", "\n")
    } else {
        (" ", " ")
    };

    for func in macro_func {
        let func_name = &func.func_name;
        let return_type = &func.return_type.typename;
        let arg_name = &func.args[0].name;
        let arg_type = if decay {
            func.args[0].typename.clone().with_ref().typename
        } else {
            func.args[0].typename.typename.clone()
        };
        let x = if decay {
            "&".to_string() + arg_name
        } else {
            arg_name.to_owned()
        };
        out += "\n";
        out += &format!("inline {return_type} {generic_name}({arg_type} {arg_name}");
        for i in 1..func.args.len() {
            if (i % 2) == 0 {
                out += ",\n    "
            } else {
                out += ", ";
            }

            out += &format!("{} {}", func.args[i].typename.typename, func.args[i].name);
        }
        out += &format!(") {{{body_start}");
        if return_type != "void" {
            out += "return ";
        }
        out += &format!("{func_name}({x}");
        for i in 1..func.args.len() {
            out += &format!(", {}", func.args[i].name);
        }
        out += &format!(");{body_end}}};");
    }
    out
}

fn generate_generic_loan_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_loan", true)
}

fn generate_generic_loan_to_owned_type_cpp(macro_func: &[FunctionSignature]) -> String {
    let mut processed_loaned_types = HashSet::<String>::new();
    let mut out = "template<class T> struct z_loaned_to_owned_type_t {};
template<class T> struct z_owned_to_loaned_type_t {};"
        .to_owned();
    for f in macro_func {
        let loaned = f.return_type.clone().without_cv().without_ptr().typename;
        if processed_loaned_types.contains(&loaned) {
            continue;
        } else {
            processed_loaned_types.insert(loaned.clone());
        }
        let owned = f.args[0]
            .typename
            .clone()
            .without_cv()
            .without_ptr()
            .typename;
        if owned.contains("view") {
            continue;
        }
        out += &format!(
            "
template<> struct z_loaned_to_owned_type_t<{loaned}> {{ typedef {owned} type; }};
template<> struct z_owned_to_loaned_type_t<{owned}> {{ typedef {loaned} type; }};"
        );
    }
    out
}

fn generate_generic_loan_mut_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_loan_mut", true)
}

fn generate_generic_drop_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_drop", false)
}

fn generate_generic_move_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_move", true)
}

fn generate_generic_take_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_take", false)
}

fn generate_generic_take_from_loaned_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_take_from_loaned", false)
}

fn generate_generic_null_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_internal_null", false)
}

fn generate_generic_check_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_internal_check", true)
}

fn generate_generic_call_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_call", false)
}

fn generate_generic_clone_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_clone", false)
}

fn generate_generic_recv_cpp(macro_func: &[FunctionSignature]) -> String {
    let try_recv_funcs: Vec<FunctionSignature> = macro_func
        .iter()
        .filter(|f| f.func_name.contains("try_recv"))
        .cloned()
        .collect();
    let recv_funcs: Vec<FunctionSignature> = macro_func
        .iter()
        .filter(|f| !f.func_name.contains("try_recv"))
        .cloned()
        .collect();
    generate_generic_cpp(&try_recv_funcs, "z_try_recv", false)
        + "\n\n"
        + generate_generic_cpp(&recv_funcs, "z_recv", false).as_str()
}

fn generate_generic_closure_cpp(macro_func: &[FunctionSignature]) -> String {
    // replace function pointer types with using typedefs defined with extern "C" linkage
    let mut out =
        "extern \"C\" using z_closure_drop_callback_t = void(void* context);\n".to_string();
    let mut processed = Vec::<FunctionSignature>::with_capacity(macro_func.len());
    for f in macro_func {
        let mut processed_f = f.clone();
        let prototype = processed_f.args[1]
            .typename
            .typename
            .replace(&format!(" (*{})", &processed_f.args[1].name), "");
        let callback_typename = f.func_name.clone() + "_callback_t";
        out += &format!(
            "extern \"C\" using {} = {};\n",
            callback_typename, prototype
        );
        processed_f.args[1].typename.typename = callback_typename + "*";
        processed_f.args[2].typename.typename = "z_closure_drop_callback_t*".to_string();
        processed.push(processed_f);
    }

    out += &generate_generic_cpp(&processed, "z_closure", false);
    out
}
