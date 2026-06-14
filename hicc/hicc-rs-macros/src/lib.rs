use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::quote;
use syn::Type;

mod export_class;
mod export_lib;
mod parse_types;

// =====================================================================
// Shared utilities
// =====================================================================

struct ParsedAttrs {
    in_hicc: bool,
    foreign: bool,
    name: Option<String>,
}

fn parse_export_attrs(attr_str: &str, allow_name: bool) -> Result<ParsedAttrs, syn::Error> {
    let mut in_hicc = false;
    let mut foreign = false;
    let mut name = None;

    for part in attr_str.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if part == "in_hicc" {
            in_hicc = true;
            continue;
        }
        if part == "foreign" {
            foreign = true;
            continue;
        }
        if let Some(eq_pos) = part.find('=') {
            let key = part[..eq_pos].trim();
            if key == "name" {
                if !allow_name {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "attribute `name` is not supported on this macro",
                    ));
                }
                let val = part[eq_pos + 1..].trim().trim_matches('"');
                if !val.is_empty() {
                    name = Some(val.to_string());
                }
                continue;
            }
        }
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("unknown attribute `{}`", part),
        ));
    }

    Ok(ParsedAttrs {
        in_hicc,
        foreign,
        name,
    })
}

pub(crate) fn is_unit(ty: &Type) -> bool {
    match ty {
        Type::Tuple(t) => t.elems.is_empty(),
        _ => false,
    }
}

pub(crate) fn maybe_wrap_type(ty: &Type, foreign: bool) -> TS2 {
    if foreign {
        quote! { crate::hicc::Foreign<#ty> }
    } else {
        quote! { #ty }
    }
}

pub(crate) fn make_static_ref(ty: &Type) -> TS2 {
    match ty {
        Type::Reference(r) => {
            let inner = make_static_ref(&r.elem);
            if r.mutability.is_some() {
                quote! { &'static mut #inner }
            } else {
                quote! { &'static #inner }
            }
        }
        Type::Path(tp) if tp.qself.is_none() => {
            let segs: Vec<TS2> = tp
                .path
                .segments
                .iter()
                .map(|seg| {
                    let id = &seg.ident;
                    if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments {
                        let args: Vec<TS2> = ab
                            .args
                            .iter()
                            .map(|arg| match arg {
                                syn::GenericArgument::Type(t) => make_static_ref(t),
                                syn::GenericArgument::Lifetime(_) => {
                                    quote! { 'static }
                                }
                                other => quote! { #other },
                            })
                            .collect();
                        quote! { #id<#(#args),*> }
                    } else {
                        quote! { #id }
                    }
                })
                .collect();
            quote! { #(#segs)::* }
        }
        Type::Tuple(tup) => {
            let elems: Vec<TS2> = tup.elems.iter().map(|t| make_static_ref(t)).collect();
            quote! { ( #(#elems),* ) }
        }
        Type::Ptr(p) => {
            let inner = make_static_ref(&p.elem);
            if p.mutability.is_some() {
                quote! { *mut #inner }
            } else {
                quote! { *const #inner }
            }
        }
        Type::Array(arr) => {
            let inner = make_static_ref(&arr.elem);
            let len = &arr.len;
            quote! { [#inner; #len] }
        }
        Type::Slice(s) => {
            let inner = make_static_ref(&s.elem);
            quote! { [#inner] }
        }
        Type::TraitObject(tto) => {
            let bounds: Vec<TS2> = tto
                .bounds
                .iter()
                .map(|bound| match bound {
                    syn::TypeParamBound::Trait(tb) => {
                        let segs: Vec<TS2> = tb
                            .path
                            .segments
                            .iter()
                            .map(|seg| {
                                let id = &seg.ident;
                                if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments {
                                    let args: Vec<TS2> = ab
                                        .args
                                        .iter()
                                        .map(|arg| match arg {
                                            syn::GenericArgument::Type(t) => make_static_ref(t),
                                            syn::GenericArgument::Lifetime(_) => quote! { 'static },
                                            other => quote! { #other },
                                        })
                                        .collect();
                                    quote! { #id<#(#args),*> }
                                } else {
                                    quote! { #id }
                                }
                            })
                            .collect();
                        quote! { #(#segs)::* }
                    }
                    syn::TypeParamBound::Lifetime(_) => {
                        quote! { 'static }
                    }
                    _ => quote! { #bound },
                })
                .collect();
            quote! { dyn #(#bounds)+* }
        }
        other => quote! { #other },
    }
}

// =====================================================================
// export_class
// =====================================================================

#[proc_macro_attribute]
pub fn export_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: TS2 = item.into();
    let parsed = match parse_export_attrs(&attr.to_string(), false) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };
    match export_class::export_class_inner(input, parsed.in_hicc, parsed.foreign) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error().into(),
    }
}

// =====================================================================
// export_lib
// =====================================================================

#[proc_macro_attribute]
pub fn export_lib(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: TS2 = item.into();
    let parsed = match parse_export_attrs(&attr.to_string(), true) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };
    let export_name = parsed.name.unwrap_or_else(|| "hicc_export_lib".to_string());
    match export_lib::export_lib_inner(input, &export_name, parsed.in_hicc, parsed.foreign) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error().into(),
    }
}
