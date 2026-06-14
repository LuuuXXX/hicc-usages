use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    visit_mut::VisitMut, FnArg, GenericParam, Generics, Ident, ImplItemFn, Path, ReturnType,
    Signature, Type,
};

fn is_self_path(p: &Path) -> bool {
    p.segments.len() == 1 && p.segments[0].ident == "Self" && p.segments[0].arguments.is_empty()
}

struct SelfReplacer {
    replacement: Type,
}

impl VisitMut for SelfReplacer {
    fn visit_path_mut(&mut self, path: &mut Path) {
        if is_self_path(path) {
            if let Type::Path(tp) = &self.replacement {
                *path = tp.path.clone();
                return;
            }
        }
        syn::visit_mut::visit_path_mut(self, path);
    }
}

/// Generate a unique name string from any Type, suitable for struct/ctor identifiers.
/// E.g. `&[u8]` → `"RefSlice_u8"`, `&mut [u8]` → `"RefMutSlice_u8"`,
/// `&[i32; 3]` → `"RefArray_i32_3"`, `*const T` → `"PtrConst_T"`, `*mut T` → `"PtrMut_T"`,
/// `Iter<'_, K, V>` → `"Iter_K_V"` (lifetimes stripped).
fn extract_type_name_string(ty: &Type) -> Option<String> {
    match ty {
        Type::Reference(r) => {
            let inner_name = extract_type_name_string(&r.elem)?;
            if r.mutability.is_some() {
                Some(format!("RefMut_{}", inner_name))
            } else {
                Some(format!("Ref_{}", inner_name))
            }
        }
        Type::Slice(s) => {
            let inner_name = extract_type_name_string(&s.elem)?;
            Some(format!("Slice_{}", inner_name))
        }
        Type::Array(arr) => {
            let inner_name = extract_type_name_string(&arr.elem)?;
            // Array length is a const expression; use its token string
            let len_str = arr.len.to_token_stream().to_string().replace(' ', "");
            Some(format!("Array_{}_{}", inner_name, len_str))
        }
        Type::Ptr(p) => {
            let inner_name = extract_type_name_string(&p.elem)?;
            if p.mutability.is_some() {
                Some(format!("PtrMut_{}", inner_name))
            } else {
                Some(format!("PtrConst_{}", inner_name))
            }
        }
        Type::Path(tp) if tp.qself.is_none() => {
            let mut parts = Vec::new();
            for seg in &tp.path.segments {
                parts.push(seg.ident.to_string());
                if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments {
                    let arg_strs: Vec<String> = ab
                        .args
                        .iter()
                        .filter_map(|arg| match arg {
                            // Skip lifetime args (they'll be replaced with 'static)
                            syn::GenericArgument::Lifetime(_) => None,
                            syn::GenericArgument::Type(t) => extract_type_name_string(t),
                            syn::GenericArgument::Const(c) => {
                                Some(c.to_token_stream().to_string().replace(' ', ""))
                            }
                            syn::GenericArgument::AssocType(at) => Some(
                                extract_type_name_string(&at.ty)
                                    .unwrap_or_else(|| at.ident.to_string()),
                            ),
                            other => Some(other.to_token_stream().to_string().replace(' ', "")),
                        })
                        .collect();
                    if !arg_strs.is_empty() {
                        parts.push(arg_strs.join("_"));
                    }
                }
            }
            Some(parts.join("_"))
        }
        Type::Tuple(tup) => {
            if tup.elems.is_empty() {
                Some("Unit".to_string())
            } else {
                let elem_strs: Vec<String> = tup
                    .elems
                    .iter()
                    .filter_map(|t| extract_type_name_string(t))
                    .collect();
                Some(format!("Tuple_{}", elem_strs.join("_")))
            }
        }
        Type::TraitObject(tto) => {
            let bound_strs: Vec<String> = tto
                .bounds
                .iter()
                .filter_map(|bound| {
                    if let syn::TypeParamBound::Trait(tb) = bound {
                        let mut parts = Vec::new();
                        for seg in &tb.path.segments {
                            parts.push(seg.ident.to_string());
                            if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments {
                                let arg_strs: Vec<String> = ab
                                    .args
                                    .iter()
                                    .filter_map(|arg| match arg {
                                        syn::GenericArgument::Lifetime(_) => None,
                                        syn::GenericArgument::Type(t) => {
                                            extract_type_name_string(t)
                                        }
                                        syn::GenericArgument::Const(c) => {
                                            Some(c.to_token_stream().to_string().replace(' ', ""))
                                        }
                                        syn::GenericArgument::AssocType(at) => Some(
                                            extract_type_name_string(&at.ty)
                                                .unwrap_or_else(|| at.ident.to_string()),
                                        ),
                                        other => Some(
                                            other.to_token_stream().to_string().replace(' ', ""),
                                        ),
                                    })
                                    .collect();
                                if !arg_strs.is_empty() {
                                    parts.push(arg_strs.join("_"));
                                }
                            }
                        }
                        Some(parts.join("_"))
                    } else {
                        None
                    }
                })
                .collect();
            Some(format!("Dyn_{}", bound_strs.join("_")))
        }
        _ => None,
    }
}

fn extract_type_path_string(ty: &Type) -> Option<String> {
    extract_type_name_string(ty)
}

// ---- Helper: get the self-type's top-level path ident for unsupported detection ----
// For reference/slice/ptr types, recurse into the inner type.
fn get_self_path_ident(ty: &Type) -> Option<Ident> {
    match ty {
        Type::Path(tp) if tp.qself.is_none() => tp.path.segments.first().map(|s| s.ident.clone()),
        Type::Reference(r) => get_self_path_ident(&r.elem),
        Type::Slice(s) => get_self_path_ident(&s.elem),
        Type::Array(arr) => get_self_path_ident(&arr.elem),
        Type::Ptr(p) => get_self_path_ident(&p.elem),
        Type::TraitObject(tto) => tto.bounds.iter().find_map(|bound| {
            if let syn::TypeParamBound::Trait(tb) = bound {
                tb.path.segments.first().map(|s| s.ident.clone())
            } else {
                None
            }
        }),
        _ => None,
    }
}

// ---- Helper: collect Ident of all type/const generic params ----
fn generics_idents(generics: &Generics) -> Vec<Ident> {
    generics
        .params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Type(tp) => Some(tp.ident.clone()),
            GenericParam::Const(cp) => Some(cp.ident.clone()),
            _ => None,
        })
        .collect()
}

/// Build a where clause that preserves user's original generic bounds and where predicates,
/// and unconditionally appends `ValueType + 'static` (or `crate::ValueType + 'static` for in_hicc)
/// to each type parameter. This is the ONLY where clause generator — all impl blocks
/// (ValueType, ClassType, MethodsType, methods struct, wrapper, constructor) use the same
/// where clause with the same set of generic parameters.
fn make_where_clause(generics: &Generics, in_hicc: bool) -> TS2 {
    let vt_path = if in_hicc {
        quote! { crate::ValueType }
    } else {
        quote! { ::hicc_rs::ValueType }
    };

    let mut predicates: Vec<TS2> = Vec::new();

    for param in &generics.params {
        match param {
            GenericParam::Type(tp) => {
                let i = &tp.ident;
                let user_bounds = &tp.bounds;
                if user_bounds.is_empty() {
                    predicates.push(quote! { #i: #vt_path + 'static });
                } else {
                    predicates.push(quote! { #i: #user_bounds + #vt_path + 'static });
                }
            }
            GenericParam::Lifetime(lp) => {
                let lt = &lp.lifetime;
                predicates.push(quote! { #lt: 'static });
            }
            GenericParam::Const(_) => {}
        }
    }

    if let Some(wc) = &generics.where_clause {
        for pred in &wc.predicates {
            predicates.push(quote! { #pred });
        }
    }

    if predicates.is_empty() {
        TS2::new()
    } else {
        quote! { where #(#predicates),* }
    }
}

// ---- Helper: strip lifetime params from generics ----
// When lifetimes are replaced with `'static` in the self type, they become
// concrete and should be removed from generic declarations.
fn strip_lifetime_params(generics: &Generics) -> Generics {
    let mut g = generics.clone();
    g.params = g
        .params
        .into_iter()
        .filter(|p| !matches!(p, GenericParam::Lifetime(_)))
        .collect();
    // Also remove lifetime-related where-clause predicates
    g.where_clause = g.where_clause.map(|wc| {
        let mut wc = wc;
        wc.predicates = wc
            .predicates
            .into_iter()
            .filter(|p| !matches!(p, syn::WherePredicate::Lifetime(_)))
            .collect();
        wc
    });
    g
}

// ---- Helper: generate bare generic param decls (stripping bounds from type params) ----
fn bare_generic_params(generics: &Generics) -> Vec<TS2> {
    generics
        .params
        .iter()
        .map(|p| match p {
            GenericParam::Type(tp) => {
                let i = &tp.ident;
                quote! { #i }
            }
            GenericParam::Lifetime(lp) => {
                let lt = &lp.lifetime;
                quote! { #lt }
            }
            GenericParam::Const(cp) => {
                let i = &cp.ident;
                let ty = &cp.ty;
                quote! { const #i: #ty }
            }
        })
        .collect()
}

// ---- Helper: generate fn input type token ----
fn self_input_type(self_type: &Type, receiver: &FnArg, foreign: bool) -> TS2 {
    match receiver {
        FnArg::Receiver(r) if r.reference.is_some() && r.mutability.is_some() => {
            let expanded = crate::make_static_ref(self_type);
            let parsed: Type = syn::parse2(expanded).unwrap();
            if foreign {
                let ref_ty: Type = syn::parse2(quote! { &'static mut #parsed }).unwrap();
                let ft = crate::maybe_wrap_type(&ref_ty, foreign);
                quote! { <#ft as ::hicc_rs::AbiType>::InputType }
            } else {
                quote! { <&'static mut #parsed as ::hicc_rs::AbiType>::InputType }
            }
        }
        FnArg::Receiver(r) if r.reference.is_some() => {
            let expanded = crate::make_static_ref(self_type);
            let parsed: Type = syn::parse2(expanded).unwrap();
            if foreign {
                let ref_ty: Type = syn::parse2(quote! { &'static #parsed }).unwrap();
                let ft = crate::maybe_wrap_type(&ref_ty, foreign);
                quote! { <#ft as ::hicc_rs::AbiType>::InputType }
            } else {
                quote! { <&'static #parsed as ::hicc_rs::AbiType>::InputType }
            }
        }
        _ => {
            let static_ty: Type = syn::parse2(crate::make_static_ref(self_type)).unwrap();
            let ft = crate::maybe_wrap_type(&static_ty, foreign);
            quote! { <#ft as ::hicc_rs::AbiType>::InputType }
        }
    }
}

fn param_input_type(ty: &Type, foreign: bool) -> TS2 {
    let static_ty: Type = syn::parse2(crate::make_static_ref(ty)).unwrap();
    let ft = crate::maybe_wrap_type(&static_ty, foreign);
    quote! { <#ft as ::hicc_rs::AbiType>::InputType }
}

fn param_from_abi(pat: &Box<syn::Pat>, ty: &Type, foreign: bool) -> TS2 {
    let static_ty: Type = syn::parse2(crate::make_static_ref(ty)).unwrap();
    let ft = crate::maybe_wrap_type(&static_ty, foreign);
    let bare_ident = match &**pat {
        syn::Pat::Ident(pi) => &pi.ident,
        _ => unreachable!("param_from_abi: expected Pat::Ident"),
    };
    quote! {
        let #pat = <#ft as ::hicc_rs::AbiType>::from_abi(#bare_ident);
        let #pat = ::hicc_rs::cabi::transmute::<<#ft as ::hicc_rs::AbiType>::Target, #static_ty>(#bare_ident);
    }
}

fn self_from_abi(self_type: &Type, receiver: &FnArg, hicc_self: &Ident, foreign: bool) -> TS2 {
    match receiver {
        FnArg::Receiver(r) if r.reference.is_some() && r.mutability.is_some() => {
            let expanded = crate::make_static_ref(self_type);
            let parsed: Type = syn::parse2(expanded).unwrap();
            if foreign {
                let ref_ty: Type = syn::parse2(quote! { &'static mut #parsed }).unwrap();
                let ft = crate::maybe_wrap_type(&ref_ty, foreign);
                quote! { let #hicc_self = <#ft as ::hicc_rs::AbiType>::from_abi(#hicc_self); }
            } else {
                quote! { let #hicc_self = <&'static mut #parsed as ::hicc_rs::AbiType>::from_abi(#hicc_self); }
            }
        }
        FnArg::Receiver(r) if r.reference.is_some() => {
            let expanded = crate::make_static_ref(self_type);
            let parsed: Type = syn::parse2(expanded).unwrap();
            if foreign {
                let ref_ty: Type = syn::parse2(quote! { &'static #parsed }).unwrap();
                let ft = crate::maybe_wrap_type(&ref_ty, foreign);
                quote! { let #hicc_self = <#ft as ::hicc_rs::AbiType>::from_abi(#hicc_self); }
            } else {
                quote! { let #hicc_self = <&'static #parsed as ::hicc_rs::AbiType>::from_abi(#hicc_self); }
            }
        }
        FnArg::Receiver(r) if r.reference.is_none() && r.mutability.is_some() => {
            let ft = crate::maybe_wrap_type(self_type, foreign);
            quote! { let mut #hicc_self = <#ft as ::hicc_rs::AbiType>::from_abi(#hicc_self); }
        }
        _ => {
            let ft = crate::maybe_wrap_type(self_type, foreign);
            quote! { let #hicc_self = <#ft as ::hicc_rs::AbiType>::from_abi(#hicc_self); }
        }
    }
}

// ---- Check if a method is just `panic!()` ----
fn is_panic_method(f: &ImplItemFn) -> bool {
    if f.block.stmts.len() != 1 {
        return false;
    }
    match &f.block.stmts[0] {
        syn::Stmt::Expr(syn::Expr::Macro(m), _) => m.mac.path.is_ident("panic"),
        syn::Stmt::Macro(m) => m.mac.path.is_ident("panic"),
        _ => false,
    }
}

// ---- Check if method is a declaration (no body) ----
fn is_empty_body(f: &ImplItemFn) -> bool {
    f.block.stmts.is_empty()
}

// ---- Check for unsupported pattern: return type is same generic as self with ref/ptr args ----
// Only applies when self_type is a Type::Path (named struct/generic). For reference, slice,
// array, or pointer self-types, the self-referential recursion pattern doesn't apply.
fn check_unsupported_pattern(self_type: &Type, sig: &Signature) -> Result<(), syn::Error> {
    let self_ident = match self_type {
        Type::Path(tp) if tp.qself.is_none() => tp.path.segments.first().map(|s| s.ident.clone()),
        _ => return Ok(()),
    };
    if let ReturnType::Type(_, ret_ty) = &sig.output {
        if let Some(ret_ident) = get_self_path_ident(ret_ty) {
            if let Some(si) = &self_ident {
                if &ret_ident == si {
                    if has_ref_ptr_arg(ret_ty) {
                        return Err(syn::Error::new(
                            sig.ident.span(),
                            format!(
                                "unsupported: method `{}` returns the same generic type with a reference/pointer type argument, which would cause monomorphization recursion",
                                sig.ident
                            ),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn has_ref_ptr_arg(ty: &Type) -> bool {
    match ty {
        Type::Path(tp) if tp.qself.is_none() => {
            for seg in &tp.path.segments {
                if let syn::PathArguments::AngleBracketed(a) = &seg.arguments {
                    for arg in &a.args {
                        if let syn::GenericArgument::Type(t) = arg {
                            match t {
                                Type::Reference(_) => return true,
                                Type::Ptr(_) => return true,
                                _ => {
                                    if has_ref_ptr_arg(t) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            false
        }
        _ => false,
    }
}

// ---- Generate methods struct ----
fn gen_methods_struct(
    struct_ident: &Ident,
    self_type: &Type,
    generics: &Generics,
    methods: &[&ImplItemFn],
    foreign: bool,
    in_hicc: bool,
) -> TS2 {
    let gp = bare_generic_params(generics);
    let wc = make_where_clause(generics, in_hicc);

    let fields: Vec<TS2> = methods
        .iter()
        .map(|f| {
            let name = &f.sig.ident;
            let fn_ty = gen_fn_ptr_type(&f.sig, self_type, foreign);
            if is_panic_method(f) {
                quote! { pub #name: Option<#fn_ty> }
            } else {
                quote! { pub #name: #fn_ty }
            }
        })
        .collect();

    // Use `pub` even in foreign mode because `Foreign<T>` is now in a `pub mod`
    // (see hicc-rs/src/foreign.rs). The methods struct must be at least as visible
    // as the type it implements ClassType for, otherwise E0446 fires.
    let vis = if foreign {
        quote! { pub }
    } else {
        quote! { pub }
    };
    if gp.is_empty() {
        quote! {
            #[repr(C)]
            #[allow(dead_code)]
            #vis struct #struct_ident {
                #(#fields),*
            }
        }
    } else {
        quote! {
            #[repr(C)]
            #[allow(dead_code)]
            #vis struct #struct_ident<#(#gp),*> #wc {
                #(#fields),*
            }
        }
    }
}

fn replace_self_in_type(ty: &Type, replacement: &Type) -> Type {
    let mut ty = ty.clone();
    SelfReplacer {
        replacement: replacement.clone(),
    }
    .visit_type_mut(&mut ty);
    ty
}

// ---- Helper: compute the CABI return type info for a method ----
// Returns (original_inner_type, abi_cabi_type, is_async)
// For async methods, CABI type is Box<dyn Future<Output = R>> (NOT wrapped in Foreign).
// For sync methods, CABI type is R (wrapped in Foreign if foreign).
fn cabi_return_type_info(
    sig: &Signature,
    self_type: Option<&Type>,
    _foreign: bool,
) -> Option<(Type, Type, bool)> {
    match &sig.output {
        ReturnType::Type(_, ty) => {
            let mut inner: Type = (**ty).clone();
            if let Some(st) = self_type {
                let replacement: Type = syn::parse2(crate::make_static_ref(st))
                    .expect("cabi_return_type_info: make_static_ref should produce valid Type");
                inner = replace_self_in_type(&inner, &replacement);
            }
            if is_unit(&inner) {
                None
            } else {
                let static_inner: Type = syn::parse2(crate::make_static_ref(&inner)).unwrap();
                let is_async = sig.asyncness.is_some();
                if is_async {
                    let boxed_future_ty: Type = syn::parse2(quote! {
                        Box<dyn ::core::future::Future<Output = #static_inner>>
                    })
                    .unwrap();
                    Some((inner, boxed_future_ty, true))
                } else {
                    Some((inner, static_inner, false))
                }
            }
        }
        _ => None,
    }
}

// ---- Generate function pointer type for a method ----
fn gen_fn_ptr_type(sig: &Signature, self_type: &Type, foreign: bool) -> TS2 {
    let mut its = Vec::new();
    if let Some(first) = sig.inputs.first() {
        if matches!(first, FnArg::Receiver(_)) {
            its.push(self_input_type(self_type, first, foreign));
        }
    }
    for input in sig.inputs.iter().skip(1) {
        if let FnArg::Typed(pt) = input {
            its.push(param_input_type(&pt.ty, foreign));
        }
    }
    let rt = match cabi_return_type_info(sig, Some(self_type), foreign) {
        Some((_, cabi_ret_ty, is_async)) => {
            let ft = crate::maybe_wrap_type(&cabi_ret_ty, !is_async && foreign);
            quote! { -> <#ft as ::hicc_rs::AbiType>::OutputType }
        }
        _ => TS2::new(),
    };
    quote! { unsafe extern "C" fn(#(#its),*) #rt }
}

// ---- Helper: wrap expression in into_abi(transmute(...)) for ABI return ----
// Transmutes the concrete return value (from_ty) to <#ft as AbiType>::Target,
// then calls into_abi to produce the OutputType that the extern "C" fn returns.
fn transmuted_into_abi(ft: &TS2, from_ty: &TS2, inner: TS2) -> TS2 {
    quote! {
        <#ft as ::hicc_rs::AbiType>::into_abi(
            ::hicc_rs::cabi::transmute::<#from_ty, <#ft as ::hicc_rs::AbiType>::Target>({ #inner })
        )
    }
}

// ---- Helper: transmute <T as AbiType>::Target to concrete self type ----
// After from_abi(), the value's type is <T as AbiType>::Target which is opaque
// to the compiler for method resolution. This converts it to the concrete type
// (e.g. &'static OnceLock<T>) so .method() calls compile.
//
// For foreign mode, self_from_abi wraps the type in Foreign<T>, so the
// transmute key must use Foreign<T> to match.
// For simple types, uses type ascription (let x: Concrete = x) which is a no-op.
// For types where Target can't be normalized (e.g. dyn Any), falls through to
// a raw pointer read which also avoids Sized-proving issues with specialization.
/// Compute concrete types used in the generated wrapper:
/// - `hicc_self_ty`: the type of `hicc_self` after `from_abi` + `self_after_from_abi`
///   (e.g. `&'static Self`, `&'static mut Self`, or `Self` for owned)
/// - `self_repl_ty`: the type to replace `Self` keyword with (e.g. `MyType<'static, T>`)
fn compute_self_types(self_type: &Type, receiver: &FnArg) -> (TS2, TS2) {
    let expanded = crate::make_static_ref(self_type);
    let parsed: Type = syn::parse2(expanded)
        .expect("compute_self_types: make_static_ref should produce valid Type");
    let self_repl_ty = quote! { #parsed };
    let hicc_self_ty = match receiver {
        FnArg::Receiver(r) if r.reference.is_some() && r.mutability.is_some() => {
            quote! { &'static mut #parsed }
        }
        FnArg::Receiver(r) if r.reference.is_some() => {
            quote! { &'static #parsed }
        }
        _ => {
            quote! { #parsed }
        }
    };
    (hicc_self_ty, self_repl_ty)
}

fn self_after_from_abi(
    self_type: &Type,
    receiver: &FnArg,
    hicc_self: &Ident,
    foreign: bool,
) -> TS2 {
    let (concrete_ty, _) = compute_self_types(self_type, receiver);

    // is_mut = true only for `mut self` (owned, mutable).
    // For `&mut self`, the mut is in the reference, not the binding.
    let is_mut =
        matches!(receiver, FnArg::Receiver(r) if r.mutability.is_some() && r.reference.is_none());

    // In foreign mode, self_from_abi wrapped the type in Foreign<>, so the
    // Target lookup must use Foreign<concrete_ty>, not concrete_ty directly.
    let target_ty = if foreign {
        crate::maybe_wrap_type(
            &syn::parse2(concrete_ty.clone())
                .expect("self_after_from_abi: concrete_ty should parse as Type"),
            true,
        )
    } else {
        concrete_ty.clone()
    };

    // Transmute opaque Target to ConcreteType with explicit type params.
    // We spell out BOTH type params explicitly (no _ inference) because:
    // 1. The compiler can't normalize Target to ConcreteType (specialization opaque)
    // 2. Type inference (_) fails for dyn trait objects (E0282)
    // 3. But <ConcreteTy as AbiType>::Target IS valid as a type expression
    if is_mut {
        quote! {
            let mut #hicc_self = ::hicc_rs::cabi::transmute::<
                <#target_ty as ::hicc_rs::AbiType>::Target,
                #concrete_ty
            >(#hicc_self);
        }
    } else {
        quote! {
            let #hicc_self = ::hicc_rs::cabi::transmute::<
                <#target_ty as ::hicc_rs::AbiType>::Target,
                #concrete_ty
            >(#hicc_self);
        }
    }
}

// ---- Generate global wrapper function for each method ----
fn gen_wrapper_fn(
    fn_ident: &Ident,
    self_type: &Type,
    generics: &Generics,
    method: &ImplItemFn,
    is_decl: bool,
    foreign: bool,
    in_hicc: bool,
) -> TS2 {
    let gp = bare_generic_params(generics);
    let wc = make_where_clause(generics, in_hicc);

    // Build parameter list for the extern "C" fn
    let hicc_self = Ident::new("hicc_self", proc_macro2::Span::call_site());
    let mut abi_params = Vec::new();
    let mut extra_params = Vec::new();

    if let Some(first) = method.sig.inputs.first() {
        if matches!(first, FnArg::Receiver(_)) {
            let input_ty = self_input_type(self_type, first, foreign);
            abi_params.push(quote! { #hicc_self: #input_ty });
        }
    }

    for input in method.sig.inputs.iter().skip(1) {
        if let FnArg::Typed(pt) = input {
            let pat = &pt.pat;
            let bare_ident: Ident = match &**pat {
                syn::Pat::Ident(pi) => pi.ident.clone(),
                _ => unreachable!("expected Pat::Ident for method param"),
            };
            let ty = &*pt.ty;
            let input_ty = param_input_type(ty, foreign);
            abi_params.push(quote! { #bare_ident: #input_ty });
            extra_params.push((bare_ident, pat.clone(), ty.clone()));
        }
    }

    let ret_ty = match cabi_return_type_info(&method.sig, Some(self_type), foreign) {
        Some((original_inner, cabi_ret_ty, is_async)) => {
            Some((original_inner, cabi_ret_ty, is_async))
        }
        _ => None,
    };

    let rty = ret_ty
        .as_ref()
        .map(|(_, cabi_ret_ty, is_async)| {
            let ft = crate::maybe_wrap_type(cabi_ret_ty, !*is_async && foreign);
            quote! { -> <#ft as ::hicc_rs::AbiType>::OutputType }
        })
        .unwrap_or(TS2::new());

    let _is_async = method.sig.asyncness.is_some();

    // After from_abi(), obj has type <T as AbiType>::Target which is opaque.
    // Transmute to concrete type so method calls/deref/field access resolve.
    let self_transmute = method
        .sig
        .inputs
        .first()
        .filter(|a| matches!(a, FnArg::Receiver(_)))
        .map(|receiver| self_after_from_abi(self_type, receiver, &hicc_self, foreign))
        .unwrap_or_else(|| TS2::new());

    let body = if is_empty_body(method) && is_decl {
        let fn_name = &method.sig.ident;
        let call_args: Vec<TS2> = extra_params
            .iter()
            .map(|(bi, _, _)| quote! { #bi })
            .collect();
        let method_self_call = if let Some(first) = method.sig.inputs.first() {
            match first {
                FnArg::Receiver(r) if r.reference.is_some() && r.mutability.is_some() => {
                    quote! { #hicc_self.#fn_name(#(#call_args),*) }
                }
                FnArg::Receiver(r) if r.reference.is_some() => {
                    quote! { #hicc_self.#fn_name(#(#call_args),*) }
                }
                _ => quote! { #hicc_self.#fn_name(#(#call_args),*) },
            }
        } else {
            quote! { #hicc_self.#fn_name(#(#call_args),*) }
        };

        let from_abi_self = self_from_abi(
            self_type,
            method.sig.inputs.first().unwrap(),
            &hicc_self,
            foreign,
        );
        let from_abi_extra: Vec<TS2> = extra_params
            .iter()
            .map(|(_, p, t)| param_from_abi(p, t, foreign))
            .collect();

        if let Some((_original_ret_ty, cabi_ret_ty, is_async_ret)) = &ret_ty {
            let ft = crate::maybe_wrap_type(cabi_ret_ty, !*is_async_ret && foreign);
            let into_abi_call = if *is_async_ret {
                let from_ty = quote! { #cabi_ret_ty };
                let async_inner =
                    quote! { { let v: #cabi_ret_ty = Box::new(#method_self_call); v } };
                transmuted_into_abi(&ft, &from_ty, async_inner)
            } else {
                let from_ty = quote! { #cabi_ret_ty };
                transmuted_into_abi(&ft, &from_ty, quote! { #method_self_call })
            };
            quote! {
                #from_abi_self
                #self_transmute
                #(#from_abi_extra)*
                #into_abi_call
            }
        } else {
            quote! {
                #from_abi_self
                #self_transmute
                #(#from_abi_extra)*
                #method_self_call
            }
        }
    } else {
        let from_abi_self = self_from_abi(
            self_type,
            method.sig.inputs.first().unwrap(),
            &hicc_self,
            foreign,
        );
        let from_abi_extra: Vec<TS2> = extra_params
            .iter()
            .map(|(_, p, t)| param_from_abi(p, t, foreign))
            .collect();

        // Compute types for Self replacement and closure wrapping
        let has_self_receiver = method
            .sig
            .inputs
            .first()
            .map_or(false, |a| matches!(a, FnArg::Receiver(_)));
        let (hicc_self_ty, self_repl_ty) = if has_self_receiver {
            compute_self_types(self_type, method.sig.inputs.first().unwrap())
        } else {
            (quote! { () }, quote! { () })
        };
        let self_replacement: Type =
            syn::parse2(self_repl_ty).expect("gen_wrapper_fn: self_repl_ty should parse as Type");

        let body = method.block.clone();
        struct ReplaceSelfIdents;
        impl syn::visit_mut::VisitMut for ReplaceSelfIdents {
            fn visit_ident_mut(&mut self, ident: &mut Ident) {
                if ident == "self" {
                    *ident = Ident::new("hicc_self", ident.span());
                }
            }
        }
        let mut replaced = body;
        ReplaceSelfIdents.visit_block_mut(&mut replaced);
        if has_self_receiver {
            SelfReplacer {
                replacement: self_replacement,
            }
            .visit_block_mut(&mut replaced);
        }

        // Wrap body in closure with typed hicc_self param when Self is generic,
        // so the body never captures generic variables from the outer scope.
        // This is essential for generic Self types in async move blocks.
        // NOTE: We inline the type directly rather than using a type alias,
        // because type aliases inside function bodies cannot reference outer
        // generic parameters (E0401).
        let closure_wrapped = if has_self_receiver {
            quote! { (|hicc_self: #hicc_self_ty| { #replaced })(hicc_self) }
        } else {
            quote! { { #replaced } }
        };

        if replaced.stmts.is_empty() {
            let self_discard = quote! { let _ = #hicc_self; };
            let param_discards: Vec<TS2> = extra_params
                .iter()
                .map(|(bi, _, _)| quote! { let _ = #bi; })
                .collect();
            if let Some((_, cabi_ret_ty, is_async_ret)) = &ret_ty {
                let ft = crate::maybe_wrap_type(cabi_ret_ty, !*is_async_ret && foreign);
                quote! {

                    #self_discard
                    #(#param_discards)*
                    <#ft as ::hicc_rs::AbiType>::into_abi({ todo!() })
                }
            } else {
                quote! {

                    #self_discard
                    #(#param_discards)*
                    { todo!() }
                }
            }
        } else if let Some((_original_ret_ty, cabi_ret_ty, is_async_ret)) = &ret_ty {
            let ft = crate::maybe_wrap_type(cabi_ret_ty, !*is_async_ret && foreign);
            let into_abi_call = if *is_async_ret {
                let async_body = if has_self_receiver {
                    quote! { { let v: #cabi_ret_ty = (|hicc_self: #hicc_self_ty| -> #cabi_ret_ty { let r: #cabi_ret_ty = Box::new(async move { #replaced }); r })(hicc_self); v } }
                } else {
                    quote! { { let v: #cabi_ret_ty = Box::new(async move { #replaced }); v } }
                };
                transmuted_into_abi(&ft, &quote! { #cabi_ret_ty }, async_body)
            } else {
                transmuted_into_abi(&ft, &quote! { #cabi_ret_ty }, closure_wrapped.clone())
            };
            quote! {

                #from_abi_self
                #self_transmute
                #(#from_abi_extra)*
                #into_abi_call
            }
        } else {
            quote! {

                #from_abi_self
                #self_transmute
                #(#from_abi_extra)*
                #closure_wrapped
            }
        }
    };

    let is_panic = is_panic_method(method);
    if is_panic {
        // panic methods still need the wrapper, but it'll never be called
        if gp.is_empty() {
            quote! {
                #[allow(unreachable_code)]
                unsafe extern "C" fn #fn_ident(#(#abi_params),*) #rty { #body }
            }
        } else {
            quote! {
                #[allow(unreachable_code)]
                unsafe extern "C" fn #fn_ident<#(#gp),*>(#(#abi_params),*) #rty #wc { #body }
            }
        }
    } else if gp.is_empty() {
        quote! {
            unsafe extern "C" fn #fn_ident(#(#abi_params),*) #rty { #body }
        }
    } else {
        quote! {
            unsafe extern "C" fn #fn_ident<#(#gp),*>(#(#abi_params),*) #rty #wc { #body }
        }
    }
}

// ---- Generate constructor function ----
fn gen_constructor(
    fn_ident: &Ident,
    struct_ident: &Ident,
    _self_type: &Type,
    generics: &Generics,
    methods: &[&ImplItemFn],
    wrapper_fn_idents: &[Ident],
    in_hicc: bool,
) -> TS2 {
    let gp = bare_generic_params(generics);
    let wc = make_where_clause(generics, in_hicc);
    let type_args: Vec<TS2> = generics_idents(generics)
        .iter()
        .map(|i| quote! { #i })
        .collect();

    let fields: Vec<TS2> = methods
        .iter()
        .zip(wrapper_fn_idents.iter())
        .map(|(f, wrapper_ident)| {
            let name = &f.sig.ident;
            if is_panic_method(f) {
                quote! { #name: None }
            } else if type_args.is_empty() {
                quote! { #name: #wrapper_ident }
            } else {
                quote! { #name: #wrapper_ident::<#(#type_args),*> }
            }
        })
        .collect();

    let struct_init = if gp.is_empty() {
        quote! { #struct_ident { #(#fields),* } }
    } else {
        quote! { #struct_ident::<#(#type_args),*> { #(#fields),* } }
    };

    if gp.is_empty() {
        quote! {
            const fn #fn_ident() -> #struct_ident {
                #struct_init
            }
        }
    } else {
        quote! {
            const fn #fn_ident<#(#gp),*>() -> #struct_ident<#(#type_args),*> #wc {
                #struct_init
            }
        }
    }
}

// ---- Generate ValueType impl ----
fn gen_value_type(self_type: &Type, generics: &Generics, foreign: bool, in_hicc: bool) -> TS2 {
    let gp = bare_generic_params(generics);
    let wc = make_where_clause(generics, in_hicc);
    let impl_for = crate::maybe_wrap_type(self_type, foreign);

    if gp.is_empty() {
        quote! {
            impl ::hicc_rs::ValueType for #impl_for {
                const N: usize = 0;
                type Type = ::hicc_rs::IsClass;
                type Value = ::hicc_rs::IsValue;
                type Result = Self;
            }
        }
    } else {
        quote! {
            impl<#(#gp),*> ::hicc_rs::ValueType for #impl_for #wc {
                const N: usize = 0;
                type Type = ::hicc_rs::IsClass;
                type Value = ::hicc_rs::IsValue;
                type Result = Self;
            }
        }
    }
}

// ---- Generate ClassType impl ----
fn gen_class_type(
    self_type: &Type,
    generics: &Generics,
    methods_ident: &Ident,
    foreign: bool,
    in_hicc: bool,
) -> TS2 {
    let gp = bare_generic_params(generics);
    let wc = make_where_clause(generics, in_hicc);

    let gpi: Vec<Ident> = generics_idents(generics);
    let type_args: Vec<TS2> = gpi.iter().map(|i| quote! { #i }).collect();

    let methods_ty = if type_args.is_empty() {
        quote! { #methods_ident }
    } else {
        quote! { #methods_ident<#(#type_args),*> }
    };
    let impl_for = crate::maybe_wrap_type(self_type, foreign);

    if gp.is_empty() {
        quote! {
            impl ::hicc_rs::ClassType for #impl_for {
                type Methods = #methods_ty;
            }
        }
    } else {
        quote! {
            impl<#(#gp),*> ::hicc_rs::ClassType for #impl_for #wc {
                type Methods = #methods_ty;
            }
        }
    }
}

// ---- Generate MethodsType impl ----
fn gen_methods_type(
    self_type: &Type,
    generics: &Generics,
    methods_ident: &Ident,
    ctor_ident: &Ident,
    foreign: bool,
    in_hicc: bool,
) -> TS2 {
    let gp = bare_generic_params(generics);
    let wc = make_where_clause(generics, in_hicc);

    let gpi: Vec<Ident> = generics_idents(generics);
    let type_args: Vec<TS2> = gpi.iter().map(|i| quote! { #i }).collect();

    let ctor_call = if type_args.is_empty() {
        quote! { #ctor_ident() }
    } else {
        quote! { #ctor_ident::<#(#type_args),*>() }
    };

    let methods_ty = if type_args.is_empty() {
        quote! { #methods_ident }
    } else {
        quote! { #methods_ident<#(#type_args),*> }
    };
    let class_type = crate::maybe_wrap_type(self_type, foreign);

    if gp.is_empty() {
        quote! {
            impl ::hicc_rs::MethodsType for #methods_ty {
                type Class = #class_type;
                const METHODS: &'static ::hicc_rs::AbiMethods<Self::Class> = &::hicc_rs::AbiClass::<Self::Class>::new_methods(#ctor_call);
                const REF_METHODS: &'static ::hicc_rs::AbiRefMethods<Self::Class> = &::hicc_rs::AbiClass::<Self::Class>::new_ref_methods(#ctor_call);
                const REF_MUT_METHODS: &'static ::hicc_rs::AbiRefMutMethods<Self::Class> = &::hicc_rs::AbiClass::<Self::Class>::new_ref_mut_methods(#ctor_call);
            }
        }
    } else {
        quote! {
            impl<#(#gp),*> ::hicc_rs::MethodsType for #methods_ty #wc {
                type Class = #class_type;
                const METHODS: &'static ::hicc_rs::AbiMethods<Self::Class> = &::hicc_rs::AbiClass::<Self::Class>::new_methods(#ctor_call);
                const REF_METHODS: &'static ::hicc_rs::AbiRefMethods<Self::Class> = &::hicc_rs::AbiClass::<Self::Class>::new_ref_methods(#ctor_call);
                const REF_MUT_METHODS: &'static ::hicc_rs::AbiRefMutMethods<Self::Class> = &::hicc_rs::AbiClass::<Self::Class>::new_ref_mut_methods(#ctor_call);
            }
        }
    }
}

fn is_unit(ty: &Type) -> bool {
    match ty {
        Type::Tuple(t) => t.elems.is_empty(),
        _ => false,
    }
}

// =====================================================================
// Cbindgen ExportType impl generation
// =====================================================================

#[cfg(feature = "cbindgen")]
fn gen_cbindgen_method_field(method: &ImplItemFn, self_type: &Type, foreign: bool) -> TS2 {
    let field_name = method.sig.ident.to_string();
    let panic_method = is_panic_method(method);
    let mut parts = Vec::<TS2>::new();

    parts.push(quote! { _body.push_str(#field_name); });
    parts.push(quote! { _body.push_str(":unsafe extern \"C\" fn("); });
    if let Some(first) = method.sig.inputs.first() {
        if matches!(first, FnArg::Receiver(_)) {
            let input_ty = self_input_type(self_type, first, foreign);
            parts.push(quote! {
                _body.push_str(&<
                    #input_ty as ::hicc_rs::ExportType
                >::export_name(registry));
            });
        }
    }
    for input in method.sig.inputs.iter().skip(1) {
        if let FnArg::Typed(pt) = input {
            let input_ty = param_input_type(&pt.ty, foreign);
            parts.push(quote! {
                _body.push_str(",");
                _body.push_str(&<
                    #input_ty as ::hicc_rs::ExportType
                >::export_name(registry));
            });
        }
    }
    match cabi_return_type_info(&method.sig, Some(self_type), foreign) {
        Some((_, cabi_ret_ty, is_async)) => {
            let ft = crate::maybe_wrap_type(&cabi_ret_ty, !is_async && foreign);
            let output_ty = quote! { <#ft as ::hicc_rs::AbiType>::OutputType };
            parts.push(quote! { _body.push_str(")->"); });
            parts.push(quote! {
                _body.push_str(&<
                    #output_ty as ::hicc_rs::ExportType
                >::export_name(registry));
            });
        }
        None => {
            parts.push(quote! { _body.push_str(")"); });
        }
    }
    if panic_method {
        parts.push(quote! { _body.push_str(",\n"); });
    } else {
        parts.push(quote! { _body.push_str(","); });
    }

    quote! { #(#parts)* }
}

#[cfg(feature = "cbindgen")]
fn gen_cbindgen_export_types(
    struct_ident: &Ident,
    self_type: &Type,
    generics: &Generics,
    methods: &[&ImplItemFn],
    foreign: bool,
    in_hicc: bool,
) -> TS2 {
    let gp = bare_generic_params(generics);
    let wc = make_where_clause(generics, in_hicc);
    let gpi: Vec<Ident> = generics_idents(generics);
    let type_args: Vec<TS2> = gpi.iter().map(|i| quote! { #i }).collect();

    // ---- Methods struct ExportType impl ----
    let method_fields: Vec<TS2> = methods
        .iter()
        .map(|m| gen_cbindgen_method_field(m, self_type, foreign))
        .collect();

    let methods_ty = if type_args.is_empty() {
        quote! { #struct_ident }
    } else {
        quote! { #struct_ident<#(#type_args),*> }
    };
    let abi_class_ty = crate::maybe_wrap_type(self_type, foreign);

    let methods_export = quote! {
        impl<#(#gp),*> ::hicc_rs::ExportType for #methods_ty #wc {
            fn export_name(registry: &mut ::hicc_rs::TypeRegistry) -> ::std::string::String {
                registry.insert_abi_methods::<Self, _>(|registry, _name| {
                    let _ac = <::hicc_rs::AbiClass<#abi_class_ty> as ::hicc_rs::ExportType>::export_name(registry);
                    let _ac_r = <&::hicc_rs::AbiClass<#abi_class_ty> as ::hicc_rs::ExportType>::export_name(registry);
                    let _ac_m = <&mut ::hicc_rs::AbiClass<#abi_class_ty> as ::hicc_rs::ExportType>::export_name(registry);
                    let mut _body = ::std::format!(
                        "hicc_destroy:unsafe extern \"C\" fn({ac}),hicc_make_unique:unsafe extern \"C\" fn({ac})->{ac},\
                         hicc_make_ref_mut:unsafe extern \"C\" fn({acm})->{ac},hicc_size_of:unsafe extern \"C\" fn()->usize,\
                         hicc_write:unsafe extern \"C\" fn({acm},{ac}),hicc_make_ref:unsafe extern \"C\" fn({acr})->{ac},",
                        ac=_ac, acm=_ac_m, acr=_ac_r,
                    );
                    #(#method_fields)*
                    ::std::format!("#[repr(C)]\npub struct {} {{\n{}}}", _name, _body)
                })
            }
        }
    };

    methods_export
}

// =====================================================================
// Core generate function
// =====================================================================

fn generate_from_impl_decl(
    imp: &crate::parse_types::ImplDecl,
    in_hicc: bool,
    foreign: bool,
) -> Result<TS2, syn::Error> {
    let impl_item_fn_list: Vec<(ImplItemFn, bool)> = imp
        .items
        .iter()
        .map(|item| crate::parse_types::impl_item_method_to_fn(item))
        .collect();

    let self_type = &imp.self_ty;
    let generics = &imp.generics;

    let type_path_str = extract_type_path_string(self_type).ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "cannot extract type name from impl",
        )
    })?;

    let static_self_type: Type = syn::parse2(crate::make_static_ref(self_type)).unwrap();
    let stripped_generics = strip_lifetime_params(generics);

    let mut methods: Vec<&ImplItemFn> = Vec::new();
    let mut is_decl_flags: Vec<bool> = Vec::new();
    for (f, is_decl) in &impl_item_fn_list {
        if !f
            .sig
            .inputs
            .first()
            .map(|a| matches!(a, FnArg::Receiver(_)))
            .unwrap_or(false)
        {
            return Err(syn::Error::new(
                f.sig.ident.span(),
                format!(
                    "export_class requires methods with a `self` receiver, but method `{}` has no `self` parameter. Use #[export_lib] for associated functions (static methods).",
                    f.sig.ident
                ),
            ));
        }
        check_unsupported_pattern(self_type, &f.sig)?;
        methods.push(f);
        is_decl_flags.push(*is_decl);
    }

    let generics = &stripped_generics;
    let struct_ident = format_ident!("Hicc{}Methods", type_path_str);

    let type_name_lower = type_path_str.to_lowercase();
    let wrapper_fn_idents: Vec<Ident> = methods
        .iter()
        .map(|f| format_ident!("hicc_{}_{}", type_name_lower, f.sig.ident))
        .collect();

    let ctor_ident = format_ident!("hicc_{}_methods", type_name_lower);

    let methods_struct = gen_methods_struct(
        &struct_ident,
        &static_self_type,
        generics,
        &methods,
        foreign,
        in_hicc,
    );

    let wrapper_fns: Vec<TS2> = methods
        .iter()
        .zip(&wrapper_fn_idents)
        .zip(&is_decl_flags)
        .map(|((m, id), is_decl)| {
            gen_wrapper_fn(
                id,
                &static_self_type,
                generics,
                m,
                *is_decl,
                foreign,
                in_hicc,
            )
        })
        .collect();

    let constructor = gen_constructor(
        &ctor_ident,
        &struct_ident,
        &static_self_type,
        generics,
        &methods,
        &wrapper_fn_idents,
        in_hicc,
    );

    let value_type = gen_value_type(&static_self_type, generics, foreign, in_hicc);
    let class_type = gen_class_type(&static_self_type, generics, &struct_ident, foreign, in_hicc);
    let methods_type = gen_methods_type(
        &static_self_type,
        generics,
        &struct_ident,
        &ctor_ident,
        foreign,
        in_hicc,
    );

    #[cfg(feature = "cbindgen")]
    let cbindgen_types = gen_cbindgen_export_types(
        &struct_ident,
        &static_self_type,
        generics,
        &methods,
        foreign,
        in_hicc,
    );
    #[cfg(not(feature = "cbindgen"))]
    let cbindgen_types = TS2::new();

    let output = quote! {
        #methods_struct
        #(#wrapper_fns)*
        #constructor
        #value_type
        #class_type
        #methods_type
        #cbindgen_types
    };

    Ok(output)
}

// =====================================================================
// Entry points
// =====================================================================

pub(crate) fn export_class_inner(
    input: TS2,
    in_hicc: bool,
    foreign: bool,
) -> Result<TokenStream, syn::Error> {
    // Try parsing as an inherent impl block first (handles fn decls with `;`)
    let ahead: TS2 = input.clone();
    if let Ok(imp) = syn::parse2::<crate::parse_types::ImplDecl>(ahead) {
        let out = generate_from_impl_decl(&imp, in_hicc, foreign)?;
        return postprocess(out, in_hicc);
    }
    // Try parsing as a mod block containing impl blocks
    if let Ok(item_mod) = syn::parse2::<crate::parse_types::ModClassDecl>(input) {
        return export_class_mod(item_mod, in_hicc, foreign);
    }
    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "export_class requires an impl block or a mod block containing impl blocks",
    ))
}

fn export_class_mod(
    item_mod: crate::parse_types::ModClassDecl,
    in_hicc: bool,
    foreign: bool,
) -> Result<TokenStream, syn::Error> {
    let mod_ident = &item_mod.ident;
    let mod_attrs: Vec<&syn::Attribute> = item_mod
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("export_class"))
        .collect();

    let mut generated: Vec<TS2> = Vec::new();
    let mut pass_through: Vec<TS2> = Vec::new();

    for item in &item_mod.items {
        match item {
            crate::parse_types::ModClassItem::Impl(imp) => {
                let out = generate_from_impl_decl(imp, in_hicc, foreign)?;
                generated.push(out);
            }
            crate::parse_types::ModClassItem::Other(other) => {
                pass_through.push(quote! { #other });
            }
        }
    }

    if generated.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "export_class mod block contains no impl blocks",
        ));
    }

    let out = quote! {
        #(#mod_attrs)*
        mod #mod_ident {
            #(#pass_through)*
            #(#generated)*
        }
    };
    postprocess(out, in_hicc)
}

fn postprocess(out: TS2, in_hicc: bool) -> Result<TokenStream, syn::Error> {
    let out_str = out.to_string();
    let out_str = if in_hicc {
        let re = regex_lite::Regex::new(r"::\s*hicc_rs\s*::").unwrap();
        re.replace_all(&out_str, "crate::").to_string()
    } else {
        out_str
    };
    match out_str.parse::<TS2>() {
        Ok(t) => Ok(t.into()),
        Err(e) => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            &format!("reparse: {}", e),
        )),
    }
}
