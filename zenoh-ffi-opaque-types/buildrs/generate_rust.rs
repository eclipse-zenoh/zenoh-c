use std::collections::{HashMap, HashSet};

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use regex::Regex;

use crate::buildrs::{get_out_opaque_types, split_type_name, write_if_changed};

fn generate_single_type(
    out_ts: &mut TokenStream,
    cfg_tokens: &TokenStream,
    type_name: &str,
    size: u64,
    align: u64,
) {
    let (prefix, category, semantic, postfix) = split_type_name(type_name);
    // Build cfg tokens once; stringify for prebindgen and use tokens for #[cfg]
    let type_ident = format_ident!("{}", type_name);
    let align_lit = syn::LitInt::new(&align.to_string(), Span::call_site());
    // Specific case for z_id_t: the identifier is public
    let field_name: TokenStream = if type_name == "z_id_t" {
        quote!(pub id)
    } else {
        quote!(_0)
    };
    let derive_tokens = if category != Some("owned") {
        quote! { #[derive(Copy, Clone)] }
    } else {
        TokenStream::new()
    };
    let prebindgen_attr = if cfg_tokens.is_empty() {
        quote! { #[prebindgen("types")] }
    } else {
        let cfg_tokens_str = cfg_tokens.to_string();
        quote! { #[prebindgen("types", cfg = #cfg_tokens_str)] }
    };
    out_ts.extend(quote! {
        #prebindgen_attr
        #derive_tokens
        #[repr(C, align(#align_lit))]
        #[rustfmt::skip]
        pub struct #type_ident {
            #field_name: [u8; #size as usize],
        }
    });
    if category == Some("owned") {
        let moved_type_name = format!("{}_{}_{}_{}", prefix, "moved", semantic, postfix);
        let moved_ident = format_ident!("{}", moved_type_name);
        out_ts.extend(quote! {
            #prebindgen_attr
            #[repr(C)]
            #[rustfmt::skip]
            pub struct #moved_ident {
                pub _this: #type_ident,
            }
        });
    }
}

/// Generate Rust opaque types code for all targets and write it into OUT_DIR/opaque_types.rs
///
/// layouts: HashMap<target_triple, HashMap<type_name, (size, align)>>
/// docs: name -> Vec<String> (each starts with '///')
pub fn generate_rust_types(
    host_target_triple: &str,
    layouts: &HashMap<&str, HashMap<String, (u64, u64)>>,
) -> std::path::PathBuf {
    let mut out_ts = TokenStream::new();
    let docs = read_docs_from_probe_lib();

    // Collect all targets except host one and build negative conditions based on them
    let mut cfgs = HashSet::new();
    for target in layouts.keys() {
        if *target == host_target_triple {
            continue;
        }
        let triple = prebindgen::utils::TargetTriple::parse(*target).unwrap_or_else(|e| {
            panic!("Failed to parse target triple '{target}': {e}");
        });
        cfgs.insert(triple);
    }
    let cfg_tokens = &cfgs.iter().map(|t| t.to_cfg_tokens()).collect::<Vec<_>>();
    let negated_cfg_tokens = if cfg_tokens.is_empty() {
        TokenStream::new()
    } else if cfg_tokens.len() == 1 {
        quote! { not( #(#cfg_tokens),* ) }
    } else {
        quote! { not(any( #(#cfg_tokens),* )) }
    };

    for (target, types) in layouts {
        let cfg_tokens = if *target == host_target_triple {
            negated_cfg_tokens.clone()
        } else {
            prebindgen::utils::TargetTriple::parse(*target)
                .unwrap_or_else(|e| {
                    panic!("Failed to parse target triple '{target}': {e}");
                })
                .to_cfg_tokens()
        };
        for (type_name, (size, align)) in types {
            let lines = docs
                .get(type_name)
                .unwrap_or_else(|| panic!("Failed to extract docs for opaque type: {type_name}"));
            // Convert collected `/// ...` lines into #[doc = "..."] attributes
            for d in lines {
                let doc_txt = d.trim_start().strip_prefix("///").unwrap_or(d).trim_start();
                let doc_lit = doc_txt;
                out_ts.extend(quote! { #[doc = #doc_lit] });
            }
            generate_single_type(&mut out_ts, &cfg_tokens, type_name, *size, *align);
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
