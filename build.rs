use fs2::FileExt;
use std::io::{Read, Write};
use std::{borrow::Cow, collections::HashMap, io::BufWriter, path::Path};

const GENERATION_PATH: &str = "include/zenoh-gen.h";
const SPLITGUIDE_PATH: &str = "splitguide.yaml";

fn main() {
    cbindgen::generate(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .expect("Unable to generate bindings")
        .write_to_file(GENERATION_PATH);

    split_bindings();
    rename_enums();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=splitguide.yaml");
    println!("cargo:rerun-if-changed=cbindgen.toml")
}

fn rename_enums() {
    let split_guide = SplitGuide::from_yaml(SPLITGUIDE_PATH);
    for (name, _) in split_guide.rules.iter() {
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
        let new = buf.replace("_T_", "_");
        // Replace _t_Tag from union variant name
        let new = new.replace("_t_Tag", "_tag_t");

        // Overwrite content
        let mut file = std::fs::File::options()
            .read(false)
            .create(false)
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        file.lock_exclusive().unwrap();
        file.write_all(new.as_bytes()).unwrap();
        file.unlock().unwrap();
    }
}

fn split_bindings() {
    let mut errors = Vec::new();
    let split_guide = SplitGuide::from_yaml(SPLITGUIDE_PATH);
    let bindings = std::fs::read_to_string(GENERATION_PATH).unwrap();
    let mut files = split_guide
        .rules
        .iter()
        .map(|(name, _)| {
            let file = std::fs::File::options()
                .write(true)
                .append(false)
                .create(true)
                .open(format!("include/{}", name))
                .unwrap();
            file.lock_exclusive().unwrap();
            file.set_len(0).unwrap();
            (name.as_str(), BufWriter::new(file))
        })
        .collect::<HashMap<_, _>>();
    let mut records = group_tokens(Tokenizer { inner: &bindings });
    for id in split_guide.requested_ids() {
        if !records.iter().any(|r| r.token.id == id) {
            errors.push(format!(
                "{} not found (requested explicitly by splitguide.yaml)",
                id,
            ))
        }
    }
    for record in &mut records {
        let appropriate_files = split_guide.appropriate_files(record);
        for file in appropriate_files {
            let writer = files.get_mut(file).unwrap();
            record.used = true;
            for comment in &record.comments {
                writeln!(writer, "{}", comment).unwrap();
            }
            writeln!(writer, "{}", &record.token).unwrap();
        }
    }
    for record in records {
        if !record.used && !record.token.id.is_empty() && record.token.tt != TokenType::PrivateToken
        {
            errors.push(format!(
                "Unused {:?} record: {}",
                record.token.tt, record.token.id
            ))
        }
    }
    if !errors.is_empty() {
        panic!("Errors in splitting: {:?}", errors)
    }
    for (_, file) in files {
        file.into_inner().unwrap().unlock().unwrap();
    }
    std::fs::remove_file(GENERATION_PATH).unwrap();
}

enum SplitRule {
    Brand(TokenType),
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
                            .map(|mut s| match s.as_str() {
                                ":functions" => SplitRule::Brand(TokenType::Function),
                                ":typedefs" => SplitRule::Brand(TokenType::Typedef),
                                ":includes" => SplitRule::Brand(TokenType::Include),
                                ":defines" => SplitRule::Brand(TokenType::Define),
                                ":const" => SplitRule::Brand(TokenType::Const),
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
                    SplitRule::Brand(brand) if *brand == record.token.tt => {
                        shared.push(file.as_str())
                    }
                    SplitRule::Exclusive(id) if id == record.token.id => {
                        exclusives.push(file.as_str())
                    }
                    SplitRule::Shared(id) if id == record.token.id => shared.push(file.as_str()),
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

fn group_tokens(stream: Tokenizer) -> Vec<Record> {
    let mut comments_stack = Vec::new();
    let mut records = Vec::new();
    for token in stream {
        match token.tt {
            TokenType::Comment => comments_stack.push(token),
            TokenType::Whitespace => {}
            _ => {
                let comments = comments_stack;
                comments_stack = Vec::new();
                records.push(Record {
                    token,
                    used: false,
                    comments,
                });
            }
        }
    }
    records
}

struct Record<'a> {
    token: Token<'a>,
    used: bool,
    comments: Vec<Token<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenType {
    Comment,
    Typedef,
    Function,
    Const,
    Define,
    PrivateToken,
    Include,
    Ifndef,
    Endif,
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
        f.write_str(&self.span)
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
            .or_else(|| Self::endif(s))
            .or_else(|| Self::include(s))
            .or_else(|| Self::ifndef(s))
            .or_else(|| Self::define(s))
            .or_else(|| Self::typedef(s))
            .or_else(|| Self::r#const(s))
            .or_else(|| Self::function(s))
    }

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
                s.until_incl("\n").unwrap_or(s),
            ))
        } else {
            None
        }
    }

    fn ifndef(s: &'a str) -> Option<Self> {
        let start = "#ifndef ";
        s.starts_with(start).then(|| {
            let span = s.until("\n").unwrap_or(s);
            Token::new(TokenType::Ifndef, &span[start.len()..], span)
        })
    }

    fn define(s: &'a str) -> Option<Self> {
        let start = "#define ";
        s.strip_prefix(start).map(|defined| {
            let span = s.until("\n").unwrap_or(s);
            Token::new(
                if defined.starts_with('_') {
                    TokenType::PrivateToken
                } else {
                    TokenType::Define
                },
                span[start.len()..].split_whitespace().next().unwrap(),
                span,
            )
        })
    }

    fn endif(s: &'a str) -> Option<Self> {
        s.starts_with("#endif")
            .then(|| Token::new(TokenType::Endif, "", "#endif"))
    }

    fn include(s: &'a str) -> Option<Self> {
        Self::_include(s, "#include \"", "\"").or_else(|| Self::_include(s, "#include <", ">"))
    }

    fn _include(s: &'a str, start: &str, end: &str) -> Option<Self> {
        if s.starts_with(start) {
            let span = s.until_incl(end).expect("detected unterminated #include");
            Some(Token::new(
                TokenType::Include,
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
                    "Couldn't parse C file, stopped at: {}",
                    self.inner.lines().next().unwrap()
                )
            }
            result
        }
    }
}
