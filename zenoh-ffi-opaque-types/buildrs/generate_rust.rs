use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::buildrs::{get_out_opaque_types, split_type_name, write_if_changed};
use target_lexicon::{OperatingSystem, Triple};

/// Generate Rust opaque types code for all targets and write it into OUT_DIR/opaque_types.rs
///
/// layouts: HashMap<target_triple, HashMap<type_name, (size, align)>>
/// docs: name -> Vec<String> (each starts with '///')
pub fn generate_rust_types(
    layouts: &HashMap<&str, HashMap<String, (u64, u64)>>,
) -> std::path::PathBuf {
    let mut out_ts = TokenStream::new();
    let mut documented: HashSet<String> = HashSet::new();
    let docs = read_docs_from_probe_lib();

    for (target, types) in layouts {
        for (type_name, (size, align)) in types {
            let is_pub_id = type_name.as_str() == "z_id_t";
            let (prefix, category, semantic, postfix) = split_type_name(type_name);

            // Docs once per type
            if !documented.contains(type_name) {
                let lines = docs.get(type_name).unwrap_or_else(|| {
                    panic!("Failed to extract docs for opaque type: {type_name}")
                });
                // Convert collected `/// ...` lines into #[doc = "..."] attributes
                for d in lines {
                    let doc_txt = d.trim_start().strip_prefix("///").unwrap_or(d).trim_start();
                    let doc_lit = doc_txt;
                    out_ts.extend(quote! { #[doc = #doc_lit] });
                }
                documented.insert(type_name.clone());
            }

            // Build a correct cfg expression for the target triple using parse_cfg
            let cfg_str = cfg_from_target_triple(target);
            let type_ident = format_ident!("{}", type_name);
            let align_val = *align;
            let align_lit = syn::LitInt::new(&align_val.to_string(), Span::call_site());
            let size_val = *size;
            let field_ident = if is_pub_id {
                format_ident!("id")
            } else {
                format_ident!("_0")
            };
            let derive_tokens = if category != Some("owned") {
                quote! { #[derive(Copy, Clone)] }
            } else {
                TokenStream::new()
            };

            let struct_tokens = if is_pub_id {
                quote! {
                    #[prebindgen("types", cfg = #cfg_str)]
                    #derive_tokens
                    #[repr(C, align(#align_lit))]
                    #[rustfmt::skip]
                    pub struct #type_ident {
                        pub #field_ident: [u8; #size_val as usize],
                    }
                }
            } else {
                quote! {
                    #[prebindgen("types", cfg = #cfg_str)]
                    #derive_tokens
                    #[repr(C, align(#align_lit))]
                    #[rustfmt::skip]
                    pub struct #type_ident {
                        #field_ident: [u8; #size_val as usize],
                    }
                }
            };
            out_ts.extend(struct_tokens);

            if category == Some("owned") {
                let moved_type_name = format!("{}_{}_{}_{}", prefix, "moved", semantic, postfix);
                let moved_ident = format_ident!("{}", moved_type_name);
                out_ts.extend(quote! {
                    #[prebindgen("types", cfg = #cfg_str)]
                    #[repr(C)]
                    #[rustfmt::skip]
                    pub struct #moved_ident {
                        _this: #type_ident,
                    }

                    #[rustfmt::skip]
            impl crate::transmute::TakeCType for #moved_ident {
                        type CType = #type_ident;
                        fn take_c_type(&mut self) -> Self::CType {
                            // Replace with a zeroed-bytes instance; safe because the type is repr(C) of [u8; N]
                            let replacement = #type_ident { #field_ident: [0u8; #size_val as usize] };
                            std::mem::replace(&mut self._this, replacement)
                        }
                    }
                });
            }
        }
    }

    // Write to OUT_DIR/opaque_types.rs
    let out_path = std::env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let mut path = std::path::PathBuf::from(out_path);
    path.push("opaque_types.rs");
    // Format nicely: parse tokens into a syn::File and pretty print
    let file_syntax: syn::File =
        syn::parse2(out_ts).expect("generated tokens should form a valid Rust file");
    let formatted = prettyplease::unparse(&file_syntax);
    let _ = write_if_changed(&path, formatted.as_bytes())
        .unwrap_or_else(|e| panic!("Failed to write {}: {e}", path.display()));
    path
}

fn read_docs_from_probe_lib() -> HashMap<String, Vec<String>> {
    // Probe project is generated under <opaque_root>/probe/src/lib.rs
    let probe_lib = get_out_opaque_types()
        .join("probe")
        .join("src")
        .join("lib.rs");
    let text = std::fs::read_to_string(&probe_lib).unwrap_or_else(|e| {
        panic!(
            "Failed to read probe lib.rs at {}: {e}",
            probe_lib.display()
        )
    });
    let re = Regex::new(r"(?m)^get_opaque_type_data!\(\s*(.*)\s*,\s*(\w+)\s*(,)?\s*\);")
        .expect("valid regex");
    let mut comments: Vec<String> = Vec::new();
    let mut opaque_lines: Vec<&str> = Vec::new();
    let mut res: HashMap<String, Vec<String>> = HashMap::new();
    for line in text.lines() {
        if line.starts_with("///") {
            comments.push(line.to_string());
            continue;
        }
        if line.starts_with("get_opaque_type_data!(") || !opaque_lines.is_empty() {
            opaque_lines.push(line);
        }
        if !opaque_lines.is_empty() && line.trim_end().ends_with(");") {
            let joined = opaque_lines.join("");
            opaque_lines.clear();
            let cap = re
                .captures(&joined)
                .expect("invalid opaque type macro line");
            res.insert(cap[2].to_string(), std::mem::take(&mut comments));
        }
    }
    res
}

/// Convert a Rust target triple string (e.g., "aarch64-apple-darwin") into a cfg expression string
/// like: all(target_arch = "aarch64", target_vendor = "apple", target_os = "darwin") and
/// include target_env when present.
fn cfg_from_target_triple(target: &str) -> String {
    // Use target-lexicon (standard Rust ecosystem crate) to parse triples reliably
    let triple: Triple = target
        .parse()
        .unwrap_or_else(|e| panic!("Failed to parse target triple '{target}': {e}"));

    // Map OS into Rust cfg target_os values. Rust uses `macos` even when the triple OS is `darwin`.
    let os_cfg: String = match triple.operating_system {
        OperatingSystem::Darwin(_) => "macos".to_string(),
        ref os => os.to_string(),
    };

    let arch = triple.architecture.to_string();
    let vendor = triple.vendor.to_string();
    let env = triple.environment.to_string();

    let mut parts: Vec<String> = Vec::with_capacity(4);
    parts.push(format!("target_arch = \"{}\"", arch));
    parts.push(format!("target_vendor = \"{}\"", vendor));
    parts.push(format!("target_os = \"{}\"", os_cfg));
    // Only include env when meaningful (not "unknown" or empty)
    if !env.is_empty() && env != "unknown" {
        parts.push(format!("target_env = \"{}\"", env));
    }

    if parts.len() == 1 {
        parts.remove(0)
    } else {
        format!("all({})", parts.join(", "))
    }
}
