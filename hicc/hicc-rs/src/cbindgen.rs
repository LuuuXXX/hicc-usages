use crate::future::Notify;
use crate::{AbiClass, AbiType, ClassType, MethodsType, ValueType};
use std::collections::HashMap;
use std::eprintln;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
use syn::visit_mut::VisitMut;
use syn::Type;

pub struct TypeRegistry {
    code: HashMap<String, String>,
    names: HashMap<String, String>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            code: HashMap::new(),
            names: HashMap::new(),
        }
    }
    pub fn insert_export<F>(&mut self, full_name: &str, f: F) -> String
    where
        F: FnOnce(&mut TypeRegistry, &str) -> String,
    {
        if let Some(short) = self.names.get(full_name) {
            return short.clone();
        }
        let short = match syn::parse_str::<Type>(full_name) {
            Ok(ty) => {
                generate_name_via_type("", &ty, |candidate| self.code.contains_key(candidate))
            }
            Err(_) => sanitize_ident(full_name),
        };
        self.names.insert(full_name.to_string(), short.clone());
        self.code.insert(short.clone(), String::new());
        let value = f(self, &short);
        self.code.insert(short.clone(), value);
        short
    }
    pub fn insert_abi_class<T: ClassType, F>(&mut self, f: F) -> String
    where
        F: FnOnce(&mut TypeRegistry, &str) -> String,
    {
        let full_name = ::std::any::type_name::<AbiClass<T>>();
        if let Some(short) = self.names.get(full_name) {
            return short.clone();
        }
        let t_name = ::std::any::type_name::<T>();
        let short = match syn::parse_str::<Type>(t_name) {
            Ok(ty) => generate_name_via_type("AbiClass_", &ty, |candidate| {
                self.code.contains_key(candidate)
            }),
            Err(_) => ::std::format!("AbiClass_{}", sanitize_ident(t_name)),
        };
        self.names.insert(full_name.to_string(), short.clone());
        self.code.insert(short.clone(), String::new());
        let value = f(self, &short);
        self.code.insert(short.clone(), value);
        short
    }
    pub fn insert_abi_methods<T: MethodsType, F>(&mut self, f: F) -> String
    where
        F: FnOnce(&mut TypeRegistry, &str) -> String,
    {
        let full_name = ::std::any::type_name::<T>();
        if let Some(short) = self.names.get(full_name) {
            return short.clone();
        }
        let abi_class_full_name = ::std::any::type_name::<AbiClass<T::Class>>();
        let abi_class_short = self
            .names
            .get(abi_class_full_name)
            .expect("AbiClass<T::Class> must be registered before T::Methods");
        let short = abi_class_short.replacen("AbiClass", "AbiMethods", 1);
        assert!(
            !self.code.contains_key(&short),
            "Name conflict for Methods struct: {}",
            short
        );
        self.names.insert(full_name.to_string(), short.clone());
        self.code.insert(short.clone(), String::new());
        let value = f(self, &short);
        self.code.insert(short.clone(), value);
        short
    }
    pub fn to_cbindgen_code(self, entry: String) -> String {
        let combined = self
            .code
            .into_iter()
            .map(|(_, v)| v)
            .chain(std::iter::once(entry))
            .collect::<Vec<_>>()
            .join("\n");
        eprintln!("=== CBINDGEN COMBINED ===\n{}\n=== END ===", combined);
        let file: syn::File = syn::parse_str(&combined).expect("cbindgen: invalid generated code");
        prettyplease::unparse(&file)
    }
}

struct PathShortener {
    keep: usize,
}

impl VisitMut for PathShortener {
    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        let len = path.segments.len();
        if len > self.keep {
            let mut kept = syn::punctuated::Punctuated::new();
            for seg in path.segments.iter().skip(len - self.keep) {
                kept.push(seg.clone());
            }
            path.segments = kept;
            path.leading_colon = None;
        }
        // Recurse into generic arguments to find and shorten nested paths
        syn::visit_mut::visit_path_mut(self, path);
    }
}

fn sanitize_ident(s: &str) -> String {
    let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    let s: String = s
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let s: String = s.chars().fold(String::new(), |mut acc, c| {
        if c == '_' && acc.ends_with('_') {
        } else {
            acc.push(c);
        }
        acc
    });
    s.trim_matches('_').to_string()
}

fn max_path_segments(ty: &Type) -> usize {
    struct MaxPathFinder(usize);
    impl<'ast> syn::visit::Visit<'ast> for MaxPathFinder {
        fn visit_path(&mut self, path: &'ast syn::Path) {
            self.0 = self.0.max(path.segments.len());
            syn::visit::visit_path(self, path);
        }
    }
    let mut finder = MaxPathFinder(0);
    syn::visit::visit_type(&mut finder, ty);
    finder.0
}

fn generate_name_via_type(prefix: &str, ty: &Type, is_taken: impl Fn(&str) -> bool) -> String {
    let max_keep = max_path_segments(ty);
    let mut shortener = PathShortener { keep: 1 };
    for keep in 1..=max_keep {
        shortener.keep = keep;
        let mut ty_clone = ty.clone();
        VisitMut::visit_type_mut(&mut shortener, &mut ty_clone);
        let candidate = sanitize_ident(&::std::format!("{}{}", prefix, quote::quote!(#ty_clone)));
        if !is_taken(&candidate) {
            return candidate;
        }
    }
    let full = sanitize_ident(&::std::format!("{}{}", prefix, quote::quote!(#ty)));
    let mut n = 0u64;
    let mut candidate = ::std::format!("{}_{}", full, n);
    while is_taken(&candidate) {
        n += 1;
        candidate = ::std::format!("{}_{}", full, n);
    }
    candidate
}

pub trait ExportType {
    fn export_name(registry: &mut TypeRegistry) -> String;
}

impl<T: ValueType> ExportType for T {
    default fn export_name(registry: &mut TypeRegistry) -> String {
        let full_name = ::std::any::type_name::<T>().to_string();
        // Cross-crate types (containing "::") produce names like
        // "example_foo_bar_baz_bar::Point" which are not valid as field types in
        // standalone struct definitions.  Register a short alias so the generated
        // cbindgen code can reference them properly.
        if full_name.contains("::") {
            registry.insert_export(&full_name, |_registry, name| {
                format!("pub type {name} = {full_name};")
            })
        } else {
            full_name
        }
    }
}

impl<T: ClassType> ExportType for AbiClass<T>
where
    T::Methods: ExportType,
{
    fn export_name(registry: &mut TypeRegistry) -> String {
        registry.insert_abi_class::<T, _>(|registry, name| {
            let _mt = <T::Methods as ExportType>::export_name(registry);
            ::std::format!(
                "#[repr(C)]\npub struct {} {{\nmethods:*const {_mt},\nthis:*const (),\nlevel:usize,\n}}",
                name, _mt = _mt,
            )
        })
    }
}

impl<T: ClassType> ExportType for *const AbiClass<T>
where
    T::Methods: ExportType,
{
    fn export_name(registry: &mut TypeRegistry) -> String {
        format!("*const {}", AbiClass::<T>::export_name(registry))
    }
}

impl<T: ClassType> ExportType for *mut AbiClass<T>
where
    T::Methods: ExportType,
{
    fn export_name(registry: &mut TypeRegistry) -> String {
        format!("*mut {}", AbiClass::<T>::export_name(registry))
    }
}

impl<T: ClassType> ExportType for &AbiClass<T>
where
    T::Methods: ExportType,
{
    fn export_name(registry: &mut TypeRegistry) -> String {
        format!("&{}", AbiClass::<T>::export_name(registry))
    }
}

impl<T: ClassType> ExportType for &mut AbiClass<T>
where
    T::Methods: ExportType,
{
    fn export_name(registry: &mut TypeRegistry) -> String {
        format!("&mut {}", AbiClass::<T>::export_name(registry))
    }
}

impl<R: ValueType + AbiType> ExportType for Notify<R>
where
    <R as AbiType>::OutputType: ExportType,
{
    fn export_name(registry: &mut TypeRegistry) -> String {
        let full_name = ::std::any::type_name::<Notify<R>>();
        registry.insert_export(full_name, |registry, name| {
            let output_ty = <<R as AbiType>::OutputType as ExportType>::export_name(registry);
            ::std::format!(
                "#[repr(C)]\npub struct {} {{\non_return:unsafe extern \"C\" fn({output_ty},*const()),\nctx:*const(),\n}}",
                name,
                output_ty = output_ty,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::Type;

    #[test]
    fn test_sanitize_ident_strips_whitespace() {
        assert_eq!(
            sanitize_ident("Box < dyn HiccRuntime >"),
            "Box_dynHiccRuntime"
        );
    }

    #[test]
    fn test_sanitize_ident_collapses_underscores() {
        assert_eq!(sanitize_ident("a___b___c"), "a_b_c");
    }

    #[test]
    fn test_sanitize_ident_trims_leading_trailing() {
        assert_eq!(sanitize_ident("_hello_"), "hello");
        assert_eq!(sanitize_ident("_abc_def_"), "abc_def");
    }

    #[test]
    fn test_sanitize_ident_handles_special_chars() {
        // *mut u8 -> strip whitespace -> *mutu8 -> * becomes _ -> _mutu8 -> trim -> mutu8
        assert_eq!(sanitize_ident("* mut u8"), "mutu8");
        // &u8 -> & at front -> _u8 -> trim -> u8
        assert_eq!(sanitize_ident("& u8"), "u8");
    }

    #[test]
    fn test_path_shortener_keep1_simple() {
        let ty: Type = syn::parse_str("std::boxed::Box").unwrap();
        let mut s = PathShortener { keep: 1 };
        let mut clone = ty.clone();
        VisitMut::visit_type_mut(&mut s, &mut clone);
        let out = format!("{}", quote::quote!(#clone));
        assert_eq!(sanitize_ident(&out), "Box");
    }

    #[test]
    fn test_path_shortener_keep1_nested() {
        let ty: Type =
            syn::parse_str("alloc::boxed::Box<dyn hicc_rs::core_types::future::HiccRuntime>")
                .unwrap();
        let mut s = PathShortener { keep: 1 };
        let mut clone = ty.clone();
        VisitMut::visit_type_mut(&mut s, &mut clone);
        let out = format!("{}", quote::quote!(#clone));
        // Both outer path (alloc::boxed::Box) and inner path (hicc_rs::core_types::future::HiccRuntime)
        // should be shortened to 1 segment: Box and HiccRuntime
        assert_eq!(sanitize_ident(&out), "Box_dynHiccRuntime");
    }

    #[test]
    fn test_path_shortener_keep2_nested() {
        let ty: Type =
            syn::parse_str("alloc::boxed::Box<dyn hicc_rs::core_types::future::HiccRuntime>")
                .unwrap();
        let mut s = PathShortener { keep: 2 };
        let mut clone = ty.clone();
        VisitMut::visit_type_mut(&mut s, &mut clone);
        let out = format!("{}", quote::quote!(#clone));
        // keep=2: boxed::Box, future::HiccRuntime
        assert_eq!(sanitize_ident(&out), "boxed_Box_dynfuture_HiccRuntime");
    }

    #[test]
    fn test_generate_name_via_type_no_conflict() {
        let ty: Type =
            syn::parse_str("alloc::boxed::Box<dyn hicc_rs::core_types::future::HiccRuntime>")
                .unwrap();
        let name = generate_name_via_type("AbiClass_", &ty, |_| false);
        assert_eq!(name, "AbiClass_Box_dynHiccRuntime");
    }

    #[test]
    fn test_generate_name_via_type_conflict_expands() {
        let ty: Type = syn::parse_str("alloc::string::String").unwrap();
        let mut taken = std::collections::HashSet::<String>::new();
        taken.insert("String".into());
        // keep=1 "String" taken → keep=2 "string_String"
        let name = generate_name_via_type("", &ty, |c| taken.contains(c));
        assert_eq!(name, "string_String");
    }

    #[test]
    fn test_generate_name_via_type_all_taken_uses_suffix() {
        let ty: Type = syn::parse_str("alloc::string::String").unwrap();
        let mut taken = std::collections::HashSet::<String>::new();
        taken.insert("String".into());
        taken.insert("string_String".into());
        taken.insert("alloc_string_String".into());
        // keep=1-3 all taken → fallback uses full sanitized type + "_0"
        let name = generate_name_via_type("", &ty, |c| taken.contains(c));
        assert_eq!(name, "alloc_string_String_0");
    }

    #[test]
    fn test_insert_export_basic() {
        let mut reg = TypeRegistry::new();
        let name = reg.insert_export("alloc::string::String", |_reg, name| {
            format!("pub type {} = alloc::string::String;", name)
        });
        assert_eq!(name, "String");
    }

    #[test]
    fn test_insert_export_previously_registered() {
        let mut reg = TypeRegistry::new();
        let name1 = reg.insert_export("alloc::string::String", |_reg, name| {
            format!("pub type {} = alloc::string::String;", name)
        });
        let name2 = reg.insert_export("alloc::string::String", |_reg, name| {
            format!("pub type {} = alloc::string::String;", name)
        });
        assert_eq!(name1, name2);
    }
}
