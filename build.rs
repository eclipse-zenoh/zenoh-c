use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use fs2::FileExt;
use phf::phf_map;
use regex::Regex;

const BUGGY_GENERATION_PATH: &str = "include/zenoh-gen-buggy.h";
const GENERATION_PATH: &str = "include/zenoh-gen.h";
const PREPROCESS_PATH: &str = "include/zenoh-cpp.h";
const SPLITGUIDE_PATH: &str = "splitguide.yaml";
const HEADER: &str = r"//
// Copyright (c) 2024 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
// clang-format off
#ifdef DOCS
#define ALIGN(n)
#define ZENOHC_API
#endif
";

static RUST_TO_C_FEATURES: phf::Map<&'static str, &'static str> = phf_map! {
    "unstable" => "UNSTABLE",
    "shared-memory" => "SHARED_MEMORY",
};

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

fn main() {
    generate_opaque_types();
    cbindgen::generate(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .expect("Unable to generate bindings")
        .write_to_file(BUGGY_GENERATION_PATH);

    fix_cbindgen(BUGGY_GENERATION_PATH, GENERATION_PATH);
    std::fs::remove_file(BUGGY_GENERATION_PATH).unwrap();

    preprocess_header(GENERATION_PATH, PREPROCESS_PATH);
    create_generics_header(PREPROCESS_PATH, "include/zenoh_macros.h");
    std::fs::remove_file(PREPROCESS_PATH).unwrap();

    configure();
    let split_guide = SplitGuide::from_yaml(SPLITGUIDE_PATH);
    split_bindings(&split_guide).unwrap();
    text_replace(split_guide.rules.iter().map(|(name, _)| name.as_str()));

    fs_extra::copy_items(
        &["include"],
        cargo_target_dir(),
        &fs_extra::dir::CopyOptions::default().overwrite(true),
    )
    .expect("include should be copied to CARGO_TARGET_DIR");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=splitguide.yaml");
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=build-resources");
    println!("cargo:rerun-if-changed=include");
}

fn get_build_rs_path() -> PathBuf {
    let file_path = file!();
    let mut path_buf = PathBuf::new();
    path_buf.push(file_path);
    path_buf.parent().unwrap().to_path_buf()
}

fn produce_opaque_types_data() -> PathBuf {
    let target = env::var("TARGET").unwrap();
    let current_folder = get_build_rs_path();
    let manifest_path = current_folder.join("./build-resources/opaque-types/Cargo.toml");
    let output_file_path = current_folder.join("./.build_resources_opaque_types.txt");
    let out_file = std::fs::File::create(output_file_path.clone()).unwrap();
    let stdio = Stdio::from(out_file);

    #[allow(unused_mut)]
    let mut feature_args: Vec<&str> = vec![];
    for (rust_feature, _c_feature) in RUST_TO_C_FEATURES.entries() {
        if test_feature(rust_feature) {
            feature_args.push("-F");
            feature_args.push(rust_feature);
        }
    }

    let _ = Command::new("cargo")
        .arg("build")
        .args(feature_args)
        .arg("--target")
        .arg(target)
        .arg("--manifest-path")
        .arg(manifest_path)
        .stderr(stdio)
        .output()
        .unwrap();

    output_file_path
}

fn generate_opaque_types() {
    let type_to_inner_field_name = HashMap::from([("z_id_t", "pub id")]);
    let current_folder = get_build_rs_path();
    let path_in = produce_opaque_types_data();
    let path_out = current_folder.join("./src/opaque_types/mod.rs");

    let data_in = std::fs::read_to_string(path_in).unwrap();
    let mut data_out = String::new();
    let mut docs = get_opaque_type_docs();

    let re = Regex::new(r"type: (\w+), align: (\d+), size: (\d+)").unwrap();
    for (_, [type_name, align, size]) in re.captures_iter(&data_in).map(|c| c.extract()) {
        let inner_field_name = type_to_inner_field_name.get(type_name).unwrap_or(&"_0");
        let s = format!(
            "#[derive(Copy, Clone)]
#[repr(C, align({align}))]
pub struct {type_name} {{
    {inner_field_name}: [u8; {size}],
}}
"
        );
        let doc = docs
            .remove(type_name)
            .unwrap_or_else(|| panic!("Failed to extract docs for opaque type: {type_name}"));
        for d in doc {
            data_out += &d;
            data_out += "\r\n";
        }
        data_out += &s;
    }
    // todo: in order to support rust features in opaque_types, we should respect features here.
    // I will remove it for a while, maybe we'll implement this later
    //for d in docs.keys() {
    //    panic!("Failed to find type information for opaque type: {d}");
    //}
    std::fs::write(path_out, data_out).unwrap();
}

fn get_opaque_type_docs() -> HashMap<String, Vec<String>> {
    let current_folder = get_build_rs_path();
    let path_in = current_folder.join("./build-resources/opaque-types/src/lib.rs");
    let re = Regex::new(r"(?m)^get_opaque_type_data!\(\s*(.*)\s*,\s*(\w+)\s*(,)?\s*\);").unwrap();
    let mut comments = Vec::new();
    let mut opaque_lines = Vec::new();
    let mut res = HashMap::new();

    for line in std::fs::read_to_string(path_in).unwrap().lines() {
        if line.starts_with("///") {
            comments.push(line.to_string());
            continue;
        }
        if line.starts_with("get_opaque_type_data!(") || !opaque_lines.is_empty() {
            opaque_lines.push(line);
        }
        if !opaque_lines.is_empty() && line.ends_with(");") {
            let joined_lines = std::mem::take(&mut opaque_lines).join("");
            let capture = re.captures(&joined_lines).expect("invalid opaque type");
            res.insert(capture[2].to_string(), std::mem::take(&mut comments));
        }
    }
    res
}

// See: https://github.com/rust-lang/cargo/issues/9661
// See: https://github.com/rust-lang/cargo/issues/545
fn cargo_target_dir() -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR should be set"));
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

fn configure() {
    let content = format!(
        r#"#pragma once
#define TARGET_ARCH_{}
"#,
        std::env::var("CARGO_CFG_TARGET_ARCH")
            .unwrap()
            .to_uppercase()
    );

    let mut file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .append(false)
        .create(true)
        .open("include/zenoh_configure.h")
        .unwrap();
    file.lock_exclusive().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    for (rust_feature, c_feature) in RUST_TO_C_FEATURES.entries() {
        if test_feature(rust_feature) {
            file.write_all(format!("#define {}\n", c_feature).as_bytes())
                .unwrap();
        }
    }
    file.unlock().unwrap();
}

fn text_replace<'a>(files: impl Iterator<Item = &'a str>) {
    for name in files {
        let path = format!("include/{}", name);

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
        file.unlock().unwrap();

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
        file.unlock().unwrap();
    }
}

fn split_bindings(split_guide: &SplitGuide) -> Result<(), String> {
    let bindings = std::fs::read_to_string(GENERATION_PATH).unwrap();
    let mut files = split_guide
        .rules
        .iter()
        .map(|(name, _)| {
            let file = std::fs::File::options()
                .write(true)
                .truncate(true)
                .append(false)
                .create(true)
                .open(format!("include/{}", name))
                .unwrap();
            file.lock_exclusive().unwrap();
            file.set_len(0).unwrap();
            (name.as_str(), BufWriter::new(file))
        })
        .collect::<HashMap<_, _>>();
    for file in files.values_mut() {
        file.write_all(HEADER.as_bytes())
            .map_err(|e| e.to_string())?;
    }
    let mut records = group_tokens(Tokenizer {
        filename: GENERATION_PATH,
        inner: &bindings,
    })?;
    for id in split_guide.requested_ids() {
        if !records.iter().any(|r| r.contains_id(id)) {
            return Err(format!(
                "{} not found (requested explicitly by splitguide.yaml)",
                id,
            ));
        }
    }
    for record in &mut records {
        let appropriate_files = split_guide.appropriate_files(record);
        for file in appropriate_files {
            let writer = files.get_mut(file).unwrap();
            record.used = true;
            write!(writer, "{}", &record).unwrap();
        }
    }
    for record in &records {
        record.is_used()?;
    }
    for (_, file) in files {
        file.into_inner().unwrap().unlock().unwrap();
    }
    std::fs::remove_file(GENERATION_PATH).unwrap();
    Ok(())
}

enum SplitRule {
    Brand(RecordType),
    Exclusive(String),
    Shared(String),
}
struct SplitGuide {
    rules: Vec<(String, Vec<SplitRule>)>,
}
impl SplitGuide {
    fn from_yaml<P: AsRef<Path>>(path: P) -> Self {
        let map: HashMap<String, Vec<String>> =
            serde_yaml::from_reader(std::fs::File::open(path).unwrap()).unwrap();
        SplitGuide {
            rules: map
                .into_iter()
                .map(|(name, rules)| {
                    (
                        name,
                        rules
                            .into_iter()
                            .filter_map(|s| {
                                let mut split = s.split('#');
                                let val = split.next().unwrap();
                                for feature in split {
                                    if !test_feature(feature) {
                                        return None;
                                    }
                                }
                                Some(val.to_owned())
                            })
                            .map(|mut s| match s.as_str() {
                                ":functions" => SplitRule::Brand(RecordType::Function),
                                ":typedefs" => SplitRule::Brand(RecordType::Typedef),
                                ":includes" => SplitRule::Brand(RecordType::PreprInclude),
                                ":defines" => SplitRule::Brand(RecordType::PreprDefine),
                                ":const" => SplitRule::Brand(RecordType::Const),
                                ":multiples" => SplitRule::Brand(RecordType::Multiple),
                                _ if s.ends_with('!') => {
                                    s.pop();
                                    SplitRule::Exclusive(s)
                                }
                                _ => SplitRule::Shared(s),
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
    fn appropriate_files(&self, record: &Record) -> Vec<&str> {
        let mut shared = Vec::new();
        let mut exclusives = Vec::new();
        for (file, rules) in &self.rules {
            for rule in rules {
                match rule {
                    SplitRule::Brand(brand) if *brand == record.rt => shared.push(file.as_str()),
                    SplitRule::Exclusive(id) if record.contains_id(id) => {
                        exclusives.push(file.as_str())
                    }
                    SplitRule::Shared(id) if record.contains_id(id) => shared.push(file.as_str()),
                    _ => {}
                }
            }
        }
        if exclusives.is_empty() {
            shared
        } else {
            exclusives
        }
    }
    fn requested_ids(&self) -> impl Iterator<Item = &str> {
        self.rules.iter().flat_map(|(_, rules)| {
            rules.iter().filter_map(|r| match r {
                SplitRule::Brand(_) => None,
                SplitRule::Exclusive(s) | SplitRule::Shared(s) => Some(s.as_str()),
            })
        })
    }
}

fn group_tokens(stream: Tokenizer) -> Result<Vec<Record>, String> {
    let mut records = Vec::new();
    let mut record_collect = Record::new();
    for token in stream {
        record_collect.add_token(token)?;
        if record_collect.is_ready() {
            let mut record = Record::new();
            std::mem::swap(&mut record_collect, &mut record);
            records.push(record);
        }
    }
    records.push(record_collect);
    Ok(records)
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum RecordType {
    Empty,
    Multiple,
    PrivateToken,
    Typedef,
    Function,
    Const,
    PreprDefine,
    PreprInclude,
}

impl RecordType {
    fn update(&mut self, rt: RecordType) {
        match *self {
            RecordType::Empty => *self = rt,
            RecordType::Multiple => {}
            _ => *self = RecordType::Multiple,
        }
    }
}

struct Record<'a> {
    used: bool,
    rt: RecordType,
    nesting: i32,
    ids: Vec<Cow<'a, str>>,
    tokens: Vec<Token<'a>>,
}

impl<'a> Record<'a> {
    fn new() -> Self {
        Self {
            used: false,
            rt: RecordType::Empty,
            nesting: 0,
            ids: Vec::new(),
            tokens: Vec::new(),
        }
    }

    fn is_used(&self) -> Result<(), String> {
        if self.used || self.rt == RecordType::Empty || self.rt == RecordType::PrivateToken {
            Ok(())
        } else {
            let token_ids = self.tokens.iter().map(|t| t.id).collect::<Vec<_>>();
            Err(format!("Unused {:?} record: {:?}", self.rt, token_ids))
        }
    }

    fn is_ready(&self) -> bool {
        self.nesting == 0 && self.rt != RecordType::Empty
    }

    fn contains_id(&self, id: &str) -> bool {
        self.ids.iter().any(|v| v == id)
    }

    fn push_token(&mut self, token: Token<'a>) {
        self.tokens.push(token);
    }

    fn push_record_type_token(&mut self, token: Token<'a>, rt: RecordType) {
        self.rt.update(rt);
        if !token.id.is_empty() {
            self.ids.push(token.id.into());
        }
        self.push_token(token)
    }

    fn push_prepr_if(&mut self, token: Token<'a>) {
        self.nesting += 1;
        self.push_token(token)
    }

    fn push_prepr_endif(&mut self, token: Token<'a>) -> Result<(), String> {
        self.nesting -= 1;
        if self.nesting < 0 {
            return Err("unmatched #endif".into());
        }
        self.push_token(token);
        Ok(())
    }

    fn add_token(&mut self, token: Token<'a>) -> Result<(), String> {
        match token.tt {
            TokenType::Comment => self.push_token(token),
            TokenType::Typedef => self.push_record_type_token(token, RecordType::Typedef),
            TokenType::Function => self.push_record_type_token(token, RecordType::Function),
            TokenType::Const => self.push_record_type_token(token, RecordType::Const),
            TokenType::PrivateToken => self.push_record_type_token(token, RecordType::PrivateToken),
            TokenType::PreprDefine => self.push_record_type_token(token, RecordType::PreprDefine),
            TokenType::PreprInclude => self.push_record_type_token(token, RecordType::PreprInclude),
            TokenType::PreprIf => self.push_prepr_if(token),
            TokenType::PreprElse => self.push_token(token),
            TokenType::PreprEndif => self.push_prepr_endif(token)?,
            TokenType::Whitespace => self.push_token(token),
        }
        Ok(())
    }
}

// Print all comments first, skip whitespaces
impl<'a> std::fmt::Display for Record<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tokens
            .iter()
            .filter(|t| t.tt == TokenType::Comment)
            .map(|t| t.fmt(f))
            .find(|r| r.is_err())
            .unwrap_or(Ok(()))?;
        self.tokens
            .iter()
            .filter(|t| t.tt != TokenType::Comment && t.tt != TokenType::Whitespace)
            .map(|t| t.fmt(f))
            .find(|r| r.is_err())
            .unwrap_or(Ok(()))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenType {
    Comment,
    Typedef,
    Function,
    Const,
    PrivateToken,
    PreprDefine,
    PreprInclude,
    PreprIf,
    PreprElse,
    PreprEndif,
    Whitespace,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Token<'a> {
    tt: TokenType,
    id: &'a str,
    span: Cow<'a, str>,
}
impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_str(format!("{:?} [", self.tt).as_str())?;
        f.write_str(&self.span)?;
        // f.write_str("]")?;

        // Each token is finalized with endline character on output
        f.write_str("\n")?;
        Ok(())
    }
}
impl<'a> Token<'a> {
    fn new<I: Into<Cow<'a, str>>>(tt: TokenType, id: &'a str, span: I) -> Self {
        Token {
            tt,
            id,
            span: span.into(),
        }
    }

    fn next(s: &'a str) -> Option<Self> {
        Self::whitespace(s)
            .or_else(|| Self::comment(s))
            .or_else(|| Self::prepr_endif(s))
            .or_else(|| Self::prepr_include(s))
            .or_else(|| Self::prepr_define(s))
            .or_else(|| Self::prepr_if(s))
            .or_else(|| Self::prepr_else(s))
            .or_else(|| Self::typedef(s))
            .or_else(|| Self::r#const(s))
            .or_else(|| Self::function(s))
    }

    //
    // Each token is consumed without end of line characters.
    // When performing output of tokens, endline is added to each token.
    // This guarantees that tokens can be shuffled as necessary
    //
    fn typedef(s: &'a str) -> Option<Self> {
        if s.starts_with("typedef") {
            let mut len = 0;
            let mut accolades = 0;
            for c in s.chars() {
                len += c.len_utf8();
                if c == '{' {
                    accolades += 1;
                } else if c == '}' {
                    accolades -= 1;
                } else if c == ';' && accolades == 0 {
                    break;
                }
            }
            let span = &s[..len];
            let id_len = span
                .chars()
                .rev()
                .skip(1)
                .take_while(|&c| c.is_alphanumeric() || c == '_')
                .fold(1, |acc, c| acc + c.len_utf8());
            let id = &span[(span.len() - id_len)..(span.len() - 1)];
            Some(Token::new(
                if id.starts_with('_') {
                    TokenType::PrivateToken
                } else {
                    TokenType::Typedef
                },
                id,
                span,
            ))
        } else {
            None
        }
    }

    fn function(s: &'a str) -> Option<Self> {
        s.until_incl(";").and_then(|span| {
            span.contains('(').then(|| {
                let mut iter = span.chars().rev();
                let id_end = iter
                    .by_ref()
                    .take_while(|&c| c != '(')
                    .fold(1, |acc, c| acc + c.len_utf8());
                let id_len = iter
                    .take_while(|&c| c.is_alphanumeric() || c == '_')
                    .fold(0, |acc, c| acc + c.len_utf8());
                let id = &span[(span.len() - id_end - id_len)..(span.len() - id_end)];
                Token::new(TokenType::Function, id, span)
            })
        })
    }

    fn r#const(s: &'a str) -> Option<Self> {
        s.until_incl(";").and_then(|span| {
            span.contains("const").then(|| {
                let id_len = span
                    .chars()
                    .rev()
                    .skip(1)
                    .take_while(|&c| c.is_alphanumeric() || c == '_')
                    .fold(0, |acc, c| acc + c.len_utf8());
                let id = &span[(span.len() - 1 - id_len)..(span.len() - 1)];
                Token::new(TokenType::Const, id, span)
            })
        })
    }

    fn whitespace(s: &'a str) -> Option<Self> {
        let mut len = 0;
        for c in s.chars() {
            if c.is_whitespace() {
                len += c.len_utf8()
            } else {
                break;
            }
        }
        if len > 0 {
            Some(Token::new(TokenType::Whitespace, "", &s[..len]))
        } else {
            None
        }
    }

    fn comment(s: &'a str) -> Option<Self> {
        if s.starts_with("/*") {
            Some(Token::new(
                TokenType::Comment,
                "",
                s.until_incl("*/").unwrap_or(s),
            ))
        } else if s.starts_with("//") {
            Some(Token::new(
                TokenType::Comment,
                "",
                s.until("\n").unwrap_or(s),
            ))
        } else {
            None
        }
    }

    fn prepr_if(s: &'a str) -> Option<Self> {
        if s.starts_with("#if ") || s.starts_with("#ifdef ") || s.starts_with("#ifndef ") {
            let span = s.until("\n").unwrap_or(s);
            Some(Token::new(TokenType::PreprIf, span, span))
        } else {
            None
        }
    }

    fn prepr_define(s: &'a str) -> Option<Self> {
        let start = "#define ";
        s.strip_prefix(start).map(|defined| {
            let span = s.until("\n").unwrap_or(s);
            Token::new(
                if defined.starts_with('_') {
                    TokenType::PrivateToken
                } else {
                    TokenType::PreprDefine
                },
                span[start.len()..].split_whitespace().next().unwrap(),
                span,
            )
        })
    }

    fn prepr_endif(s: &'a str) -> Option<Self> {
        s.starts_with("#endif")
            .then(|| Token::new(TokenType::PreprEndif, "", s.until("\n").unwrap_or(s)))
    }

    fn prepr_else(s: &'a str) -> Option<Self> {
        s.starts_with("#else")
            .then(|| Token::new(TokenType::PreprElse, "", s.until("\n").unwrap_or(s)))
    }

    fn prepr_include(s: &'a str) -> Option<Self> {
        Self::r#include(s, "#include \"", "\"").or_else(|| Self::r#include(s, "#include <", ">"))
    }

    fn r#include(s: &'a str, start: &str, end: &str) -> Option<Self> {
        if s.starts_with(start) {
            let span = s.until_incl(end).expect("detected unterminated #include");
            Some(Token::new(
                TokenType::PreprInclude,
                &span[start.len()..(span.len() - end.len())],
                span,
            ))
        } else {
            None
        }
    }
}
trait Until: Sized {
    fn until(self, pattern: &str) -> Option<Self>;
    fn until_incl(self, pattern: &str) -> Option<Self>;
}
impl Until for &str {
    fn until(self, pattern: &str) -> Option<Self> {
        self.find(pattern).map(|l| &self[..l])
    }
    fn until_incl(self, pattern: &str) -> Option<Self> {
        self.find(pattern).map(|l| &self[..(l + pattern.len())])
    }
}
struct Tokenizer<'a> {
    filename: &'a str,
    inner: &'a str,
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            None
        } else {
            let result = Token::next(self.inner);
            if let Some(result) = &result {
                self.inner = &self.inner[result.span.len()..];
            } else {
                panic!(
                    "Couldn't parse C file {}, stopped at: {}",
                    self.filename,
                    self.inner.lines().next().unwrap()
                )
            }
            result
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ctype {
    typename: String,
}

impl Ctype {
    pub fn new(typename: &str) -> Self {
        Ctype {
            typename: typename.to_owned(),
        }
    }

    pub fn without_cv(self) -> Self {
        Ctype {
            typename: self.typename.replace("const ", ""),
        }
    }

    pub fn without_ptr(self) -> Self {
        Ctype {
            typename: self.typename.replace(['*', '&'], ""),
        }
    }

    pub fn with_ref(self) -> Self {
        Ctype {
            typename: self.typename.replace('*', "&"),
        }
    }

    pub fn decay(self) -> Self {
        self.without_cv().without_ptr()
    }
}

#[derive(Clone, Debug)]
pub struct FuncArg {
    typename: Ctype,
    name: String,
}

impl FuncArg {
    pub fn new(typename: &str, name: &str) -> Self {
        FuncArg {
            typename: Ctype::new(typename),
            name: name.to_owned(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct FunctionSignature {
    return_type: Ctype,
    func_name: String,
    args: Vec<FuncArg>,
}

pub fn create_generics_header(path_in: &str, path_out: &str) {
    let mut file_out = std::fs::File::options()
        .read(false)
        .write(true)
        .truncate(true)
        .append(false)
        .create(true)
        .open(path_out)
        .unwrap();

    let header = "#pragma once

// clang-format off
#ifndef __cplusplus


";
    file_out.write_all(header.as_bytes()).unwrap();

    let type_name_to_loan_func = find_loan_functions(path_in);
    let out = generate_generic_loan_c(&type_name_to_loan_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let type_name_to_loan_mut_func = find_loan_mut_functions(path_in);
    let out = generate_generic_loan_mut_c(&type_name_to_loan_mut_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let type_name_to_drop_func = find_drop_functions(path_in);
    let out = generate_generic_drop_c(&type_name_to_drop_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_move_c(&type_name_to_drop_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let type_name_to_null_func = find_null_functions(path_in);
    let out = generate_generic_null_c(&type_name_to_null_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let type_name_to_check_func = find_check_functions(path_in);
    let out = generate_generic_check_c(&type_name_to_check_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let type_name_to_call_func = find_call_functions(path_in);
    let out = generate_generic_call_c(&type_name_to_call_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_closure_c(&type_name_to_call_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let recv_funcs = find_recv_functions(path_in);
    let out = generate_generic_recv_c(&recv_funcs);
    file_out.write_all(out.as_bytes()).unwrap();

    file_out
        .write_all("\n#else  // #ifndef __cplusplus\n".as_bytes())
        .unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_loan_cpp(&type_name_to_loan_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_loan_mut_cpp(&type_name_to_loan_mut_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_drop_cpp(&type_name_to_drop_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_move_cpp(&type_name_to_drop_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_null_cpp(&type_name_to_null_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_check_cpp(&type_name_to_check_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_call_cpp(&type_name_to_call_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_closure_cpp(&type_name_to_call_func);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_recv_cpp(&recv_funcs);
    file_out.write_all(out.as_bytes()).unwrap();
    file_out.write_all("\n\n".as_bytes()).unwrap();

    let out = generate_generic_loan_to_owned_type_cpp(
        &[type_name_to_loan_func, type_name_to_loan_mut_func].concat(),
    );
    file_out.write_all(out.as_bytes()).unwrap();

    file_out
        .write_all("\n#endif  // #ifndef __cplusplus".as_bytes())
        .unwrap();
}

pub fn find_loan_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"const struct (\w+) \*(\w+)_loan\(const struct (\w+) \*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [return_type, func_name, arg_type, arg_name]) in
        re.captures_iter(&bindings).map(|c| c.extract())
    {
        let f = FunctionSignature {
            return_type: Ctype::new(&("const ".to_string() + return_type + "*")),
            func_name: func_name.to_string() + "_loan",
            args: vec![FuncArg::new(
                &("const ".to_string() + arg_type + "*"),
                arg_name,
            )],
        };
        res.push(f);
    }
    res
}

pub fn find_loan_mut_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"struct (\w+) \*(\w+)_loan_mut\(struct (\w+) \*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [return_type, func_name, arg_type, arg_name]) in
        re.captures_iter(&bindings).map(|c| c.extract())
    {
        let f = FunctionSignature {
            return_type: Ctype::new(&(return_type.to_string() + "*")),
            func_name: func_name.to_string() + "_loan_mut",
            args: vec![FuncArg::new(&(arg_type.to_string() + "*"), arg_name)],
        };
        res.push(f);
    }
    res
}

pub fn find_drop_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"(\w+)_drop\(struct (\w+) \*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [func_name, arg_type, arg_name]) in re.captures_iter(&bindings).map(|c| c.extract()) {
        let f = FunctionSignature {
            return_type: Ctype::new("void"),
            func_name: func_name.to_string() + "_drop",
            args: vec![FuncArg::new(&(arg_type.to_string() + "*"), arg_name)],
        };
        res.push(f);
    }
    res
}

pub fn find_null_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"(\w+)_null\(struct (\w+) \*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [func_name, arg_type, arg_name]) in re.captures_iter(&bindings).map(|c| c.extract()) {
        let f = FunctionSignature {
            return_type: Ctype::new("void"),
            func_name: func_name.to_string() + "_null",
            args: vec![FuncArg::new(&(arg_type.to_string() + "*"), arg_name)],
        };
        res.push(f);
    }
    res
}

pub fn find_check_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"bool (\w+)_check\(const struct (\w+) \*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [func_name, arg_type, arg_name]) in re.captures_iter(&bindings).map(|c| c.extract()) {
        let f = FunctionSignature {
            return_type: Ctype::new("bool"),
            func_name: func_name.to_string() + "_check",
            args: vec![FuncArg::new(
                &("const ".to_string() + arg_type + "*"),
                arg_name,
            )],
        };
        res.push(f);
    }
    res
}

pub fn find_call_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(
        r"(\w+) (\w+)_call\(const struct (\w+) \*(\w+),\s+(\w*)\s*struct (\w+) \*(\w+)\);",
    )
    .unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [return_type, func_name, closure_type, closure_name, arg_cv, arg_type, arg_name]) in
        re.captures_iter(&bindings).map(|c| c.extract())
    {
        let arg_cv: String = if arg_cv.is_empty() {
            "".to_string()
        } else {
            "const ".to_string()
        };
        let f = FunctionSignature {
            return_type: Ctype::new(return_type),
            func_name: func_name.to_string() + "_call",
            args: vec![
                FuncArg::new(&("const ".to_string() + closure_type + "*"), closure_name),
                FuncArg::new(&(arg_cv + arg_type + "*"), arg_name),
            ],
        };
        res.push(f);
    }
    res
}

pub fn find_recv_functions(path_in: &str) -> Vec<FunctionSignature> {
    let bindings = std::fs::read_to_string(path_in).unwrap();
    let re = Regex::new(r"(\w+)\s+z_(\w+)_handler_(\w+)_recv\(const\s+struct\s+(\w+)\s+\*(\w+),\s+struct\s+(\w+)\s+\*(\w+)\);").unwrap();
    let mut res = Vec::<FunctionSignature>::new();

    for (_, [return_type, handler_type, value_type, arg1_type, arg1_name, arg2_type, arg2_name]) in
        re.captures_iter(&bindings).map(|c| c.extract())
    {
        let f = FunctionSignature {
            return_type: Ctype::new(return_type),
            func_name: "z_".to_string() + handler_type + "_handler_" + value_type + "_recv",
            args: vec![
                FuncArg::new(&("const ".to_string() + arg1_type + "*"), arg1_name),
                FuncArg::new(&(arg2_type.to_string() + "*"), arg2_name),
            ],
        };
        res.push(f);
    }
    res
}

pub fn generate_generic_c(
    macro_func: &[FunctionSignature],
    generic_name: &str,
    decay: bool,
) -> String {
    let va_args = macro_func.iter().any(|f| f.args.len() > 1);
    let mut out = if va_args {
        format!(
            "#define {generic_name}(x, ...) \\
    _Generic((x)"
        )
    } else {
        format!(
            "#define {generic_name}(x) \\
    _Generic((x)"
        )
    };

    let x = if decay { "&x" } else { "x" };

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
        out += &format!("    )({x}, __VA_ARGS__)");
    } else {
        out += &format!("    )({x})");
    }
    out
}

pub fn generate_generic_loan_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_loan", true)
}

pub fn generate_generic_loan_mut_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_loan_mut", true)
}

pub fn generate_generic_drop_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_drop", false)
}

pub fn generate_generic_move_c(_macro_func: &[FunctionSignature]) -> String {
    "#define z_move(x) (&x)".to_string()
}

pub fn generate_generic_null_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_null", false)
}

pub fn generate_generic_check_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_check", true)
}

pub fn generate_generic_call_c(macro_func: &[FunctionSignature]) -> String {
    generate_generic_c(macro_func, "z_call", false)
}

pub fn generate_generic_recv_c(macro_func: &[FunctionSignature]) -> String {
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

pub fn generate_generic_closure_c(_macro_func: &[FunctionSignature]) -> String {
    "#define z_closure(x, callback, dropper, ctx) \\
    {{(x)->context = (void*)(ctx); (x)->call = (callback); (x)->drop = (dropper);}}"
        .to_owned()
}

pub fn generate_generic_cpp(
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
            out += &format!(", {} {}", func.args[i].typename.typename, func.args[i].name);
        }
        out += &format!(") {{{body_start}return {func_name}({x}");
        for i in 1..func.args.len() {
            out += &format!(", {}", func.args[i].name);
        }
        out += &format!(");{body_end}}};");
    }
    out
}

pub fn generate_generic_loan_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_loan", true)
}

pub fn generate_generic_loan_to_owned_type_cpp(macro_func: &[FunctionSignature]) -> String {
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

pub fn generate_generic_loan_mut_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_loan_mut", true)
}

pub fn generate_generic_drop_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_drop", false)
}

pub fn generate_generic_move_cpp(macro_func: &[FunctionSignature]) -> String {
    let mut move_funcs = Vec::<FunctionSignature>::new();
    for f in macro_func {
        move_funcs.push(FunctionSignature {
            return_type: f.args[0].typename.clone(),
            func_name: "".to_string(),
            args: [f.args[0].clone()].to_vec(),
        });
    }
    generate_generic_cpp(&move_funcs, "z_move", true)
}

pub fn generate_generic_null_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_null", false)
}

pub fn generate_generic_check_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_check", true)
}

pub fn generate_generic_call_cpp(macro_func: &[FunctionSignature]) -> String {
    generate_generic_cpp(macro_func, "z_call", false)
}

pub fn generate_generic_recv_cpp(macro_func: &[FunctionSignature]) -> String {
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

pub fn generate_generic_closure_cpp(macro_func: &[FunctionSignature]) -> String {
    let mut out = "".to_owned();

    for func in macro_func {
        let return_type = &func.return_type.typename;
        let closure_name = &func.args[0].name;
        let closure_type = func.args[0]
            .typename
            .clone()
            .without_cv()
            .typename
            .replace("loaned", "owned");
        let arg_type = &func.args[1].typename.typename;
        out += "\n";
        out += &format!(
            "inline void z_closure(
    {closure_type} {closure_name},
    {return_type} (*call)({arg_type}, void*),
    void (*drop)(void*),
    void *context) {{
    {closure_name}->context = context;
    {closure_name}->drop = drop;
    {closure_name}->call = call;
}};"
        );
    }
    out
}

fn test_feature(feature: &str) -> bool {
    match feature {
        #[cfg(feature = "shared-memory")]
        "shared-memory" => true,
        #[cfg(feature = "unstable")]
        "unstable" => true,
        _ => false,
    }
}

/// Evaluates conditional feature macros in the form #if (logical expression of define(FEATURE_NAME))
/// and removes the code under those that evaluate to false
/// Note: works only on single string conditional expressions
pub fn process_feature_defines(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(input_path)?;
    let lines = std::io::BufReader::new(file).lines();
    let mut out = String::new();
    let mut skip = false;
    let mut nest_level: usize = 0;
    for line in lines.flatten() {
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

pub fn evaluate_c_defines_line(line: &str) -> bool {
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
