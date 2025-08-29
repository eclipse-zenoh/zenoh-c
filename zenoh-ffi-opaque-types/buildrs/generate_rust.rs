use std::collections::{HashMap, HashSet};

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use regex::Regex;

use crate::buildrs::{get_out_opaque_types, split_type_name, write_if_changed};

/// Generate Rust opaque types code for all targets and write it into OUT_DIR/opaque_types.rs
///
/// layouts: HashMap<target_triple, HashMap<type_name, (size, align)>>
/// docs: name -> Vec<String> (each starts with '///')
pub fn generate_rust_types(
    layouts: &HashMap<&str, HashMap<String, (u64, u64)>>,
) -> std::path::PathBuf {
    let mut out_ts = TokenStream::new();
    let mut documented: HashSet<String> = HashSet::new();
    // Collect dummy types (by type name) and the list of cfgs used for concrete targets
    let mut dummy_types: HashMap<String, TokenStream> = HashMap::new();
    // Keep cfg predicates as tokens, with a set of stringified forms to deduplicate
    let mut all_cfg_tokens: Vec<TokenStream> = Vec::new();
    let mut all_cfg_seen: HashSet<String> = HashSet::new();
    let docs = read_docs_from_probe_lib();

    for (target, types) in layouts {
        for (type_name, (size, align)) in types {
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

            // Build cfg tokens once; stringify for prebindgen and use tokens for #[cfg]
            let cfg_tokens = prebindgen::utils::TargetTriple::parse(target)
                .unwrap_or_else(|e| {
                    panic!("Failed to parse target triple '{target}': {e}");
                })
                .to_cfg_tokens();
            let cfg_str: String = cfg_tokens.to_string();
            if all_cfg_seen.insert(cfg_str.clone()) {
                all_cfg_tokens.push(cfg_tokens.clone());
            }
            let type_ident = format_ident!("{}", type_name);
            let align_val = *align;
            let align_lit = syn::LitInt::new(&align_val.to_string(), Span::call_site());
            let size_val = *size;
            // Specific case for z_id_t: the identifier is public
            let field_name: TokenStream = if type_name.as_str() == "z_id_t" {
                quote!(pub id)
            } else {
                quote!(_0)
            };

            let derive_tokens = if category != Some("owned") {
                quote! { #[derive(Copy, Clone)] }
            } else {
                TokenStream::new()
            };
            let struct_tokens = quote! {
                #[prebindgen("types", cfg = #cfg_str)]
                #derive_tokens
                #[repr(C, align(#align_lit))]
                #[rustfmt::skip]
                pub struct #type_ident {
                    #field_name: [u8; #size_val as usize],
                }
            };
            let dummy_struct_tokens = quote! {
                #[repr(C)]
                #[rustfmt::skip]
                pub struct #type_ident {
                    #field_name: [u8; #size_val as usize],
                }
            };
            out_ts.extend(struct_tokens);
            // Store dummy type once per name
            dummy_types.entry(type_name.clone()).or_insert(dummy_struct_tokens);

            if category == Some("owned") {
                let moved_type_name = format!("{}_{}_{}_{}", prefix, "moved", semantic, postfix);
                let moved_ident = format_ident!("{}", moved_type_name);
                let moved_struct_tokens = quote! {
                    #[prebindgen("types", cfg = #cfg_str)]
                    #[repr(C)]
                    #[rustfmt::skip]
                    pub struct #moved_ident {
                        pub _this: #type_ident,
                    }
                };
                let dummy_moved_struct_tokens = quote! {
                    #[repr(C)]
                    #[rustfmt::skip]
                    pub struct #moved_ident {
                        pub _this: #type_ident,
                    }
                };
                out_ts.extend(moved_struct_tokens);
                dummy_types
                    .entry(moved_type_name)
                    .or_insert(dummy_moved_struct_tokens);
            }
        }
    }

    // Build a negated cfg predicate using quote!: not(any(cfg1, cfg2, ...)) or not(cfg1)
    assert!(!all_cfg_tokens.is_empty());
    let negated_cfg_tokens: TokenStream = if all_cfg_tokens.len() == 1 {
        let single = &all_cfg_tokens[0];
        quote! { not(#single) }
    } else {
        let list = &all_cfg_tokens;
        quote! { not(any( #(#list),* )) }
    };
    for (_name, ts) in dummy_types.into_iter() {
        out_ts.extend(quote! {
            #[cfg(#negated_cfg_tokens)]
            #ts
        });
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
