use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::{format_ident, quote};
use syn::{FnArg, ReturnType, Type};

// ---- Helpers for ABI type references (always use 'static lifetimes) ----
fn abi_input_type(ty: &Type, foreign: bool) -> TS2 {
    let static_ty: Type = syn::parse2(crate::make_static_ref(ty)).unwrap();
    let ft = crate::maybe_wrap_type(&static_ty, foreign);
    quote! { <#ft as ::hicc_rs::AbiType>::InputType }
}

#[allow(dead_code)]
fn abi_output_type(ty: &Type, foreign: bool) -> TS2 {
    let static_ty: Type = syn::parse2(crate::make_static_ref(ty)).unwrap();
    let ft = crate::maybe_wrap_type(&static_ty, foreign);
    quote! { -> <#ft as ::hicc_rs::AbiType>::OutputType }
}

fn abi_from_abi(pat: &syn::Pat, ty: &Type, foreign: bool) -> TS2 {
    let static_ty: Type = syn::parse2(crate::make_static_ref(ty)).unwrap();
    let ft = crate::maybe_wrap_type(&static_ty, foreign);
    let bare_ident = match pat {
        syn::Pat::Ident(pi) => &pi.ident,
        _ => unreachable!("abi_from_abi: expected Pat::Ident"),
    };
    quote! {
        let #pat = <#ft as ::hicc_rs::AbiType>::from_abi(#bare_ident);
        let #pat = ::hicc_rs::cabi::transmute::<<#ft as ::hicc_rs::AbiType>::Target, #static_ty>(#bare_ident);
    }
}

fn abi_into_abi(expr: TS2, ty: &Type, foreign: bool) -> TS2 {
    let static_ty: Type = syn::parse2(crate::make_static_ref(ty)).unwrap();
    let ft = crate::maybe_wrap_type(&static_ty, foreign);
    quote! {
        <#ft as ::hicc_rs::AbiType>::into_abi(
            ::hicc_rs::cabi::transmute::<#static_ty, <#ft as ::hicc_rs::AbiType>::Target>({ #expr })
        )
    }
}

pub(crate) fn export_lib_inner(
    input: TS2,
    export_name_from_attr: &str,
    in_hicc: bool,
    foreign: bool,
) -> Result<TokenStream, syn::Error> {
    let item_mod: crate::parse_types::ModLibDecl = syn::parse2(input)
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), &e.to_string()))?;
    let export_name = export_name_from_attr.to_string();

    let mod_ident = item_mod.ident;
    let mod_attrs: Vec<&syn::Attribute> = item_mod
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("export_lib"))
        .collect();

    let mut lib_fn_list: Vec<syn::ItemFn> = Vec::new();
    let mut pass_through: Vec<TS2> = Vec::new();

    for item in &item_mod.items {
        match item {
            crate::parse_types::ModLibItem::FnDecl(d) => {
                lib_fn_list.push(crate::parse_types::fn_decl_to_item_fn(d));
            }
            crate::parse_types::ModLibItem::FnDef(f) => {
                lib_fn_list.push(f.clone());
            }
            crate::parse_types::ModLibItem::Other(other) => {
                pass_through.push(quote! { #other });
            }
        }
    }

    if lib_fn_list.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "no functions found in export_lib block",
        ));
    }

    let lib_struct = format_ident!("Hicc_{}", export_name);
    let create_fn = format_ident!("{}", export_name);

    let lib_fns: Vec<&syn::ItemFn> = lib_fn_list.iter().collect();

    let mut struct_fields = Vec::new();
    let mut adapter_fns = Vec::new();
    let mut initializers = Vec::new();

    for f in &lib_fns {
        let fn_name = &f.sig.ident;
        let adapter_name = format_ident!("hicc_{}_{}", mod_ident, fn_name);
        let has_body = !f.block.stmts.is_empty();
        let block = &f.block;
        let extra: Vec<_> = f
            .sig
            .inputs
            .iter()
            .filter_map(|i| match i {
                FnArg::Typed(pt) => Some((pt.pat.clone(), pt.ty.clone())),
                _ => None,
            })
            .collect();

        let param_types: Vec<_> = extra
            .iter()
            .map(|(_, t)| abi_input_type(t, foreign))
            .collect();

        let is_async = f.sig.asyncness.is_some();

        let cabi_ret_ty: Option<Type> = match &f.sig.output {
            ReturnType::Type(_, ty) => {
                let inner: Type = (**ty).clone();
                if crate::is_unit(&inner) {
                    None
                } else {
                    let static_inner: Type = syn::parse2(crate::make_static_ref(&inner)).unwrap();
                    if is_async {
                        let output_type: Type = if foreign {
                            syn::parse2(quote! { crate::hicc::Foreign<#static_inner> }).unwrap()
                        } else {
                            static_inner
                        };
                        Some(
                            syn::parse2(quote! {
                                Box<dyn ::core::future::Future<Output = #output_type>>
                            })
                            .unwrap(),
                        )
                    } else {
                        Some(static_inner)
                    }
                }
            }
            _ => None,
        };

        let rt = cabi_ret_ty
            .as_ref()
            .map(|cabi_ty| {
                let ft = crate::maybe_wrap_type(cabi_ty, !is_async && foreign);
                quote! { -> <#ft as ::hicc_rs::AbiType>::OutputType }
            })
            .unwrap_or(quote! {});

        struct_fields.push(quote! {
            pub #fn_name: unsafe extern "C" fn(#(#param_types),*) #rt
        });

        let rty = cabi_ret_ty
            .as_ref()
            .map(|cabi_ty| {
                let ft = crate::maybe_wrap_type(cabi_ty, !is_async && foreign);
                quote! { -> <#ft as ::hicc_rs::AbiType>::OutputType }
            })
            .unwrap_or(quote! {});

        let oty: Option<Type> = match &f.sig.output {
            ReturnType::Type(_, ty) => {
                if crate::is_unit(ty) {
                    None
                } else {
                    Some((*(*ty)).clone())
                }
            }
            _ => None,
        };

        let abi_params: Vec<_> = extra
            .iter()
            .map(|(p, t)| {
                let inp = abi_input_type(t, foreign);
                quote! { #p: #inp }
            })
            .collect();
        let ec: Vec<_> = extra
            .iter()
            .map(|(p, t)| abi_from_abi(p, t, foreign))
            .collect();
        let call_args: Vec<_> = extra.iter().map(|(p, _)| quote! { #p }).collect();

        let body = if has_body {
            if let Some(ref original_ret_ty) = oty {
                if is_async {
                    let cabi_ty = cabi_ret_ty.as_ref().unwrap();
                    let ft = crate::maybe_wrap_type(cabi_ty, false);
                    let boxed_expr = if foreign {
                        quote! { { let v: #cabi_ty = Box::new(async move { crate::hicc::Foreign({ #block }.await) }); v } }
                    } else {
                        quote! { { let v: #cabi_ty = Box::new({ #block }); v } }
                    };
                    let ia = quote! {
                        <#ft as ::hicc_rs::AbiType>::into_abi(
                            ::hicc_rs::cabi::transmute::<_, <#ft as ::hicc_rs::AbiType>::Target>(
                                #boxed_expr
                            )
                        )
                    };
                    quote! { #(#ec)* #ia }
                } else {
                    let ia = abi_into_abi(quote!({ #block }), original_ret_ty, foreign);
                    quote! { #(#ec)* #ia }
                }
            } else {
                quote! { #(#ec)* #block }
            }
        } else if let Some(ref original_ret_ty) = oty {
            if is_async {
                let cabi_ty = cabi_ret_ty.as_ref().unwrap();
                let ft = crate::maybe_wrap_type(cabi_ty, false);
                let boxed_expr = if foreign {
                    quote! { { let v: #cabi_ty = Box::new(async move { crate::hicc::Foreign(#fn_name(#(#call_args),*).await) }); v } }
                } else {
                    quote! { { let v: #cabi_ty = Box::new(#fn_name(#(#call_args),*)); v } }
                };
                let ia = quote! {
                    <#ft as ::hicc_rs::AbiType>::into_abi(
                        ::hicc_rs::cabi::transmute::<_, <#ft as ::hicc_rs::AbiType>::Target>(
                            #boxed_expr
                        )
                    )
                };
                quote! { #(#ec)* #ia }
            } else {
                let ia = abi_into_abi(quote!(#fn_name(#(#call_args),*)), original_ret_ty, foreign);
                quote! { #(#ec)* #ia }
            }
        } else {
            quote! { #(#ec)* #fn_name(#(#call_args),*); }
        };

        adapter_fns.push(quote! {
            unsafe extern "C" fn #adapter_name(#(#abi_params),*) #rty { #body }
        });
        initializers.push(quote! { #fn_name: #adapter_name });
    }

    #[cfg(feature = "cbindgen")]
    let cbindgen_fields: Vec<TS2> = lib_fns
        .iter()
        .map(|f| {
            let field_name = f.sig.ident.to_string();
            let extra: Vec<_> = f
                .sig
                .inputs
                .iter()
                .filter_map(|i| match i {
                    FnArg::Typed(pt) => Some(pt.ty.clone()),
                    _ => None,
                })
                .collect();
            let mut parts = Vec::<TS2>::new();
            parts.push(quote! { _body.push_str(#field_name); });
            parts.push(quote! { _body.push_str(":unsafe extern \"C\" fn("); });
            for (idx, ty) in extra.iter().enumerate() {
                if idx > 0 {
                    parts.push(quote! { _body.push_str(","); });
                }
                let static_ty: Type = syn::parse2(crate::make_static_ref(ty)).unwrap();
                let ft = crate::maybe_wrap_type(&static_ty, foreign);
                parts.push(quote! {
                    _body.push_str(&<
                        <#ft as ::hicc_rs::AbiType>::InputType as ::hicc_rs::ExportType
                    >::export_name(registry));
                });
            }
            let is_async = f.sig.asyncness.is_some();
            match &f.sig.output {
                ReturnType::Type(_, ty) => {
                    if !crate::is_unit(ty) {
                        parts.push(quote! { _body.push_str(")->"); });
                        let inner: Type = (*(*ty)).clone();
                        let static_inner: Type =
                            syn::parse2(crate::make_static_ref(&inner)).unwrap();
                        let cabi_ty: Type = if is_async {
                            let async_output: Type = if foreign {
                                syn::parse2(quote! {
                                    crate::hicc::Foreign<#static_inner>
                                })
                                .unwrap()
                            } else {
                                static_inner.clone()
                            };
                            syn::parse2(quote! {
                                Box<dyn ::core::future::Future<Output = #async_output>>
                            })
                            .unwrap()
                        } else {
                            static_inner.clone()
                        };
                        let ft = crate::maybe_wrap_type(&cabi_ty, !is_async && foreign);
                        parts.push(quote! {
                            _body.push_str(&<
                                <#ft as ::hicc_rs::AbiType>::OutputType as ::hicc_rs::ExportType
                            >::export_name(registry));
                        });
                    } else {
                        parts.push(quote! { _body.push_str(")"); });
                    }
                }
                _ => {
                    parts.push(quote! { _body.push_str(")"); });
                }
            }
            parts.push(quote! { _body.push_str(","); });
            quote! { #(#parts)* }
        })
        .collect();

    #[cfg(feature = "cbindgen")]
    let cbindgen_export_type = quote! {
        impl ::hicc_rs::ExportType for #lib_struct {
            fn export_name(registry: &mut ::hicc_rs::TypeRegistry) -> String {
                let full = ::std::any::type_name::<Self>();
                registry.insert_export(full, |registry, _name| {
                    let mut _body = String::new();
                    #(#cbindgen_fields)*
                    ::std::format!("#[repr(C)]\npub struct {} {{\n{}}}", _name, _body)
                })
            }
        }
    };
    #[cfg(not(feature = "cbindgen"))]
    let cbindgen_export_type = TS2::new();

    #[cfg(feature = "cbindgen")]
    let cbindgen_fn_name = format_ident!("{}_cbindgen", export_name);
    #[cfg(feature = "cbindgen")]
    let cbindgen_helper = quote! {
        pub fn #cbindgen_fn_name(registry: &mut ::hicc_rs::TypeRegistry) -> String {
            let ty = <#lib_struct as ::hicc_rs::ExportType>::export_name(registry);
            ::std::format!(
                "#[unsafe(no_mangle)]\nextern \"C\" fn {create_fn}() -> *const {ty} {{ todo!() }}",
                create_fn = #export_name,
            )
        }
    };
    #[cfg(not(feature = "cbindgen"))]
    let cbindgen_helper = TS2::new();

    let vis_struct = if foreign {
        quote! { pub(crate) }
    } else {
        quote! { pub }
    };

    let vis_fn = quote! { pub };

    let result = quote! {
        #(#mod_attrs)*
        pub mod #mod_ident {
            #(#pass_through)*
            #[repr(C)]
            #[allow(dead_code, non_camel_case_types)]
            #vis_struct struct #lib_struct { #(#struct_fields),* }
            #(#adapter_fns)*
            impl #lib_struct { const METHODS: Self = Self { #(#initializers),* }; }
            #[unsafe(no_mangle)]
            #vis_fn extern "C" fn #create_fn() -> &'static #lib_struct { &#lib_struct::METHODS }
            #cbindgen_export_type
            #cbindgen_helper
        }
    };
    let result_str = result.to_string();
    let result_str = if in_hicc {
        let re = regex_lite::Regex::new(r"::\s*hicc_rs\s*::").unwrap();
        re.replace_all(&result_str, "crate::").to_string()
    } else {
        result_str
    };
    match result_str.parse::<TokenStream>() {
        Ok(t) => Ok(t),
        Err(e) => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            &format!("reparse: {}", e),
        )),
    }
}
