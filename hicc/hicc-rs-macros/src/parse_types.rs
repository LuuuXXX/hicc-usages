//! Custom parsing types for export_class and export_lib macros.
//!
//! These types handle fn declarations ending with `;` (no body) natively,
//! eliminating the need for the hacky `replace_semicolons` approach.
//!
//! Inspired by hicc-autogen's `ImportFn` pattern.

use proc_macro2::TokenStream as TS2;
use quote::{quote, ToTokens};
use syn::parse::discouraged::Speculative;
use syn::parse::{self, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

// =====================================================================
// MethodDecl — fn declaration in an impl block ending with `;`
// =====================================================================

/// A method declaration inside an impl block that ends with `;` (no body).
/// E.g.: `fn next(&mut self) -> Option<(&K, &V)>;`
#[derive(Clone)]
pub struct MethodDecl {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub defaultness: Option<syn::Token![default]>,
    pub asyncness: Option<syn::Token![async]>,
    pub unsafety: Option<syn::Token![unsafe]>,
    pub fn_token: syn::Token![fn],
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub paren_token: syn::token::Paren,
    pub inputs: Punctuated<syn::FnArg, syn::Token![,]>,
    pub output: syn::ReturnType,
    #[allow(dead_code)]
    pub semi_token: syn::Token![;],
}

impl Parse for MethodDecl {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis: syn::Visibility = input.parse()?;
        let defaultness: Option<syn::Token![default]> = input.parse()?;
        let asyncness: Option<syn::Token![async]> = input.parse()?;
        let unsafety: Option<syn::Token![unsafe]> = input.parse()?;
        let fn_token: syn::Token![fn] = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        let generics: syn::Generics = input.parse()?;

        let content;
        let paren_token = syn::parenthesized!(content in input);
        let inputs: Punctuated<syn::FnArg, syn::Token![,]> =
            content.parse_terminated(syn::FnArg::parse, syn::Token![,])?;

        let output: syn::ReturnType = input.parse()?;
        let semi_token: syn::Token![;] = input.parse()?;

        Ok(Self {
            attrs,
            vis,
            defaultness,
            asyncness,
            unsafety,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            output,
            semi_token,
        })
    }
}

impl ToTokens for MethodDecl {
    fn to_tokens(&self, tokens: &mut TS2) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.vis.to_tokens(tokens);
        self.defaultness.to_tokens(tokens);
        self.asyncness.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.inputs.to_tokens(tokens);
        });
        self.output.to_tokens(tokens);
        quote! {{}}.to_tokens(tokens);
    }
}

/// Convert a MethodDecl into a syn::ImplItemFn with an empty block.
/// This allows compatibility with existing generation code.
pub fn method_decl_to_impl_item_fn(decl: &MethodDecl) -> syn::ImplItemFn {
    syn::ImplItemFn {
        attrs: decl.attrs.clone(),
        vis: decl.vis.clone(),
        defaultness: decl.defaultness,
        sig: syn::Signature {
            constness: None,
            asyncness: decl.asyncness,
            unsafety: decl.unsafety,
            abi: None,
            fn_token: decl.fn_token,
            ident: decl.ident.clone(),
            generics: decl.generics.clone(),
            paren_token: decl.paren_token,
            inputs: decl.inputs.clone(),
            variadic: None,
            output: decl.output.clone(),
        },
        block: syn::Block {
            brace_token: syn::token::Brace::default(),
            stmts: vec![],
        },
    }
}

// =====================================================================
// ImplItem — enum for items inside an impl block
// =====================================================================

/// An item inside an impl block: either a declaration (`;`-terminated)
/// or a full method definition (with `{...}` body).
#[derive(Clone)]
pub enum ImplItemMethod {
    Decl(MethodDecl),
    Def(syn::ImplItemFn),
}

impl Parse for ImplItemMethod {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let ahead = input.fork();
        if let Ok(decl) = ahead.parse::<MethodDecl>() {
            input.advance_to(&ahead);
            return Ok(ImplItemMethod::Decl(decl));
        }
        let def = input.parse::<syn::ImplItemFn>()?;
        Ok(ImplItemMethod::Def(def))
    }
}

impl ToTokens for ImplItemMethod {
    fn to_tokens(&self, tokens: &mut TS2) {
        match self {
            ImplItemMethod::Decl(d) => d.to_tokens(tokens),
            ImplItemMethod::Def(d) => d.to_tokens(tokens),
        }
    }
}

/// Convert ImplItemMethod to a syn::ImplItemFn reference (for use in generate).
/// For declarations, creates an ImplItemFn with empty block.
pub fn impl_item_method_to_fn(item: &ImplItemMethod) -> (syn::ImplItemFn, bool) {
    match item {
        ImplItemMethod::Decl(d) => (method_decl_to_impl_item_fn(d), true),
        ImplItemMethod::Def(d) => (d.clone(), false),
    }
}

// =====================================================================
// ImplDecl — impl block with mixed declaration/definition items
// =====================================================================

/// An inherent impl block that can contain both `;`-terminated fn declarations
/// and `{}`-terminated fn definitions.
#[derive(Clone)]
pub struct ImplDecl {
    pub attrs: Vec<syn::Attribute>,
    pub defaultness: Option<syn::Token![default]>,
    pub unsafety: Option<syn::Token![unsafe]>,
    pub impl_token: syn::Token![impl],
    pub generics: syn::Generics,
    pub self_ty: syn::Type,
    pub where_clause: Option<syn::WhereClause>,
    pub brace_token: syn::token::Brace,
    pub items: Vec<ImplItemMethod>,
}

impl Parse for ImplDecl {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let defaultness: Option<syn::Token![default]> = input.parse()?;
        let unsafety: Option<syn::Token![unsafe]> = input.parse()?;
        let impl_token: syn::Token![impl] = input.parse()?;
        let mut generics: syn::Generics = input.parse()?;

        let self_ty: syn::Type = input.parse()?;
        let where_clause: Option<syn::WhereClause> = input.parse()?;
        generics.where_clause = where_clause.clone();

        let content;
        let brace_token = syn::braced!(content in input);

        let mut items = Vec::new();
        while !content.is_empty() {
            let item: ImplItemMethod = content.parse()?;
            items.push(item);
        }

        Ok(Self {
            attrs,
            defaultness,
            unsafety,
            impl_token,
            generics,
            self_ty,
            where_clause,
            brace_token,
            items,
        })
    }
}

impl ToTokens for ImplDecl {
    fn to_tokens(&self, tokens: &mut TS2) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.defaultness.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.impl_token.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.self_ty.to_tokens(tokens);
        self.where_clause.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            for item in &self.items {
                item.to_tokens(tokens);
            }
        });
    }
}

// =====================================================================
// FnDecl — standalone fn declaration ending with `;` (for export_lib)
// =====================================================================

/// A standalone function declaration ending with `;` (no body).
/// E.g.: `fn add(x: i32, y: i32) -> i32;`
/// Call path resolution uses the fn ident directly (no crate:: prefix);
/// users must bring the target function into scope via `use` statements.
pub struct FnDecl {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub asyncness: Option<syn::Token![async]>,
    pub fn_token: syn::Token![fn],
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub paren_token: syn::token::Paren,
    pub inputs: Punctuated<syn::FnArg, syn::Token![,]>,
    pub output: syn::ReturnType,
    #[allow(dead_code)]
    pub semi_token: syn::Token![;],
}

impl Parse for FnDecl {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis: syn::Visibility = input.parse()?;
        let asyncness: Option<syn::Token![async]> = input.parse()?;
        let fn_token: syn::Token![fn] = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        let generics: syn::Generics = input.parse()?;

        let content;
        let paren_token = syn::parenthesized!(content in input);
        let inputs: Punctuated<syn::FnArg, syn::Token![,]> =
            content.parse_terminated(syn::FnArg::parse, syn::Token![,])?;

        for arg in &inputs {
            if let syn::FnArg::Receiver(_) = arg {
                return Err(syn::Error::new(
                    arg.span(),
                    "export_lib functions cannot have a self receiver",
                ));
            }
        }

        let output: syn::ReturnType = input.parse()?;
        let semi_token: syn::Token![;] = input.parse()?;

        Ok(Self {
            attrs,
            vis,
            asyncness,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            output,
            semi_token,
        })
    }
}

impl ToTokens for FnDecl {
    fn to_tokens(&self, tokens: &mut TS2) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.vis.to_tokens(tokens);
        self.asyncness.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.inputs.to_tokens(tokens);
        });
        self.output.to_tokens(tokens);
        quote! {{}}.to_tokens(tokens);
    }
}

pub fn fn_decl_to_item_fn(decl: &FnDecl) -> syn::ItemFn {
    syn::ItemFn {
        attrs: decl.attrs.clone(),
        vis: decl.vis.clone(),
        sig: syn::Signature {
            constness: None,
            asyncness: decl.asyncness,
            unsafety: None,
            abi: None,
            fn_token: decl.fn_token,
            ident: decl.ident.clone(),
            generics: decl.generics.clone(),
            paren_token: decl.paren_token,
            inputs: decl.inputs.clone(),
            variadic: None,
            output: decl.output.clone(),
        },
        block: Box::new(syn::Block {
            brace_token: syn::token::Brace::default(),
            stmts: vec![],
        }),
    }
}

// =====================================================================
// ModClassDecl — mod block for export_class
// =====================================================================

/// Items inside an export_class mod block.
pub enum ModClassItem {
    Impl(ImplDecl),
    Other(syn::Item),
}

/// A mod block for export_class, containing impl blocks and other items
/// (like `use` statements).
pub struct ModClassDecl {
    pub attrs: Vec<syn::Attribute>,
    pub mod_token: syn::Token![mod],
    pub ident: syn::Ident,
    pub brace_token: syn::token::Brace,
    pub items: Vec<ModClassItem>,
}

impl Parse for ModClassDecl {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let mod_token: syn::Token![mod] = input.parse()?;
        let ident: syn::Ident = input.parse()?;

        let content;
        let brace_token = syn::braced!(content in input);

        let mut items = Vec::new();
        while !content.is_empty() {
            let ahead = content.fork();
            if let Ok(impl_decl) = ahead.parse::<ImplDecl>() {
                content.advance_to(&ahead);
                items.push(ModClassItem::Impl(impl_decl));
                continue;
            }
            let item: syn::Item = content.parse()?;
            items.push(ModClassItem::Other(item));
        }

        Ok(Self {
            attrs,
            mod_token,
            ident,
            brace_token,
            items,
        })
    }
}

impl ToTokens for ModClassDecl {
    fn to_tokens(&self, tokens: &mut TS2) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.mod_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            for item in &self.items {
                match item {
                    ModClassItem::Impl(i) => i.to_tokens(tokens),
                    ModClassItem::Other(i) => i.to_tokens(tokens),
                }
            }
        });
    }
}

// =====================================================================
// ModLibItem — items inside an export_lib mod block
// =====================================================================

/// Items inside an export_lib mod block.
pub enum ModLibItem {
    FnDecl(FnDecl),
    FnDef(syn::ItemFn),
    Other(syn::Item),
}

/// A mod block for export_lib, containing fn declarations, fn definitions,
/// and other items.
pub struct ModLibDecl {
    pub attrs: Vec<syn::Attribute>,
    pub mod_token: syn::Token![mod],
    pub ident: syn::Ident,
    pub brace_token: syn::token::Brace,
    pub items: Vec<ModLibItem>,
}

impl Parse for ModLibDecl {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let mod_token: syn::Token![mod] = input.parse()?;
        let ident: syn::Ident = input.parse()?;

        let content;
        let brace_token = syn::braced!(content in input);

        let mut items = Vec::new();
        while !content.is_empty() {
            let ahead = content.fork();
            if let Ok(fn_decl) = ahead.parse::<FnDecl>() {
                content.advance_to(&ahead);
                items.push(ModLibItem::FnDecl(fn_decl));
                continue;
            }

            let ahead = content.fork();
            if let Ok(fn_def) = ahead.parse::<syn::ItemFn>() {
                content.advance_to(&ahead);
                for arg in &fn_def.sig.inputs {
                    if let syn::FnArg::Receiver(_) = arg {
                        return Err(syn::Error::new(
                            arg.span(),
                            "export_lib functions cannot have a self receiver",
                        ));
                    }
                }
                items.push(ModLibItem::FnDef(fn_def));
                continue;
            }

            let item: syn::Item = content.parse()?;
            items.push(ModLibItem::Other(item));
        }

        Ok(Self {
            attrs,
            mod_token,
            ident,
            brace_token,
            items,
        })
    }
}

impl ToTokens for ModLibDecl {
    fn to_tokens(&self, tokens: &mut TS2) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.mod_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            for item in &self.items {
                match item {
                    ModLibItem::FnDecl(d) => d.to_tokens(tokens),
                    ModLibItem::FnDef(d) => d.to_tokens(tokens),
                    ModLibItem::Other(i) => i.to_tokens(tokens),
                }
            }
        });
    }
}
