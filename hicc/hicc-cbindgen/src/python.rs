//! Generate Python ctypes bindings from the Step-2 Rust source code using `syn`.
//!
//! Pipeline:
//!   1. `syn::parse_file` the Rust source (output of `registry.to_cbindgen_code()`)
//!   2. Walk items for `#[repr(C)]` struct definitions and `extern "C"` entry functions
//!   3. Map Rust types to Python ctypes expressions
//!   4. Emit Python `.py` with low-level structs + wrapper classes (destroy() required) + factory functions

use syn::{Attribute, Fields, File, Item, ReturnType, Type};

// =====================================================================
// Parsed data structures
// =====================================================================

#[derive(Debug)]
struct StructDef {
    name: String,
    fields: Vec<FieldDef>,
    kind: StructKind,
}

#[derive(Debug)]
enum StructKind {
    /// AbiMethods_X — function pointer table for an export_class type
    AbiMethods,
    /// AbiClass_X — the opaque type struct { methods, this_, level }
    AbiClass,
    /// Function table from export_lib (struct with fn‑ptr fields, not AbiMethods/AbiClass)
    FnTable,
    /// Plain repr(C) data struct (e.g. Point)
    Plain,
}

#[derive(Debug)]
enum FieldDef {
    FnPtr {
        ret_py: String,
        name: String,
        params_py: Vec<String>,
    },
    Basic {
        py_type: String,
        name: String,
    },
}

// =====================================================================
// Parsing
// =====================================================================

/// Parse the Rust source into a list of StructDef + entry function info.
fn parse_items(file: &File) -> (Vec<StructDef>, Vec<EntryFn>) {
    let mut structs: Vec<StructDef> = Vec::new();
    let mut entry_fns: Vec<EntryFn> = Vec::new();

    for item in &file.items {
        match item {
            Item::Struct(s) => {
                if !has_repr_c(&s.attrs) {
                    continue;
                }
                let fields = match &s.fields {
                    Fields::Named(nf) => {
                        let mut out = Vec::new();
                        for f in &nf.named {
                            if let Some(name) = &f.ident {
                                out.push(parse_field(&f.ty, &name.to_string()));
                            }
                        }
                        out
                    }
                    _ => continue,
                };
                let name = s.ident.to_string();
                let kind = classify_struct(&name, &fields);
                structs.push(StructDef { name, fields, kind });
            }
            Item::Fn(f) => {
                // extern "C" fn — entry point (from _cbindgen output)
                let is_extern_c = f
                    .sig
                    .abi
                    .as_ref()
                    .and_then(|a| a.name.as_ref())
                    .map(|s| s.value() == "C")
                    .unwrap_or(false);
                if !is_extern_c {
                    continue;
                }
                if let Some(ret_struct) = extract_return_struct(&f.sig.output) {
                    entry_fns.push(EntryFn {
                        name: f.sig.ident.to_string(),
                        ret_struct,
                    });
                }
            }
            _ => {}
        }
    }

    (structs, entry_fns)
}

#[derive(Debug)]
struct EntryFn {
    name: String,
    ret_struct: String,
}

/// Check if an attribute list contains `#[repr(C)]`.
fn has_repr_c(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("repr") {
            return false;
        }
        if let Ok(meta) = attr.meta.require_list() {
            // Token‑based check: the tokens should contain "C"
            let s = meta.tokens.to_string();
            // Look for a bare `C` identifier (not inside a string)
            s.split(|c: char| c == ',' || c == '(' || c == ')' || c == ' ')
                .any(|part| part.trim() == "C")
        } else {
            false
        }
    })
}

/// Classify a struct by its name and field contents.
fn classify_struct(name: &str, fields: &[FieldDef]) -> StructKind {
    if name.starts_with("AbiMethods_") || name.starts_with("class_AbiMethods_") {
        StructKind::AbiMethods
    } else if name.starts_with("AbiClass_") || name.starts_with("class_AbiClass_") {
        StructKind::AbiClass
    } else if fields.iter().any(|f| matches!(f, FieldDef::FnPtr { .. })) {
        StructKind::FnTable
    } else {
        StructKind::Plain
    }
}

/// Convert a single `syn::Type` + field name to a FieldDef.
fn parse_field(ty: &Type, name: &str) -> FieldDef {
    // Check if it's a bare function pointer or is wrapped in Option<…>
    if let Some(bare_fn) = extract_bare_fn(ty) {
        let params_py: Vec<String> = bare_fn
            .inputs
            .iter()
            .map(|arg| rust_type_to_ctypes(&arg.ty, &[]))
            .collect();
        let ret_py = match &bare_fn.output {
            ReturnType::Type(_, t) => rust_type_to_ctypes(t.as_ref(), &[]),
            ReturnType::Default => "None".to_string(),
        };
        return FieldDef::FnPtr {
            ret_py,
            name: name.to_string(),
            params_py,
        };
    }

    // Everything else → basic field
    let py_type = rust_type_to_ctypes(ty, &[]);
    FieldDef::Basic {
        py_type,
        name: name.to_string(),
    }
}

/// Try to extract a `syn::TypeBareFn` from a type, unwrapping `Option<…>` if present.
fn extract_bare_fn(ty: &Type) -> Option<&syn::TypeBareFn> {
    match ty {
        Type::BareFn(b) => Some(b),
        // Option<unsafe extern "C" fn(…)>
        Type::Path(p) => {
            let seg = p.path.segments.last()?;
            if seg.ident != "Option" {
                return None;
            }
            let args = match &seg.arguments {
                syn::PathArguments::AngleBracketed(ab) => &ab.args,
                _ => return None,
            };
            if args.len() != 1 {
                return None;
            }
            let inner_ty = match &args[0] {
                syn::GenericArgument::Type(t) => t,
                _ => return None,
            };
            match inner_ty {
                Type::BareFn(b) => Some(b),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Extract the return struct name from an `extern "C" fn` return type.
/// Handles `*const X` and `&X` patterns.
fn extract_return_struct(output: &ReturnType) -> Option<String> {
    let ty = match output {
        ReturnType::Type(_, t) => t.as_ref(),
        ReturnType::Default => return None,
    };
    // *const X or & X
    let inner = match ty {
        Type::Ptr(ptr) => &ptr.elem,
        Type::Reference(ref_) => &ref_.elem,
        _ => return None,
    };
    // X — a named type
    match inner.as_ref() {
        Type::Path(p) => {
            let name = p.path.segments.last()?.ident.to_string();
            Some(name)
        }
        _ => None,
    }
}

// =====================================================================
// Rust type → Python ctypes expression mapping
// =====================================================================

/// Convert a Rust type expression to a Python ctypes type string.
///
/// `extra_names` provides additional known struct names (populated during
/// code generation phase but not needed during the initial parse pass).
fn rust_type_to_ctypes(ty: &Type, _extra: &[String]) -> String {
    match ty {
        Type::Ptr(ptr) => {
            // Special case: *const () → void pointer
            if matches!(&*ptr.elem, Type::Tuple(t) if t.elems.is_empty()) {
                return "ctypes.c_void_p".to_string();
            }
            // *const i8 / *mut i8 → C string (null-terminated)
            if let Type::Path(p) = &*ptr.elem {
                if let Some(seg) = p.path.segments.last() {
                    if seg.ident == "i8" {
                        return "ctypes.c_char_p".to_string();
                    }
                }
            }
            let inner = rust_type_to_ctypes(&ptr.elem, _extra);
            format!("ctypes.POINTER({})", inner)
        }
        Type::Reference(ref_) => {
            let inner = rust_type_to_ctypes(&ref_.elem, _extra);
            format!("ctypes.POINTER({})", inner)
        }
        Type::Path(p) => {
            let seg = match p.path.segments.last() {
                Some(s) => s,
                None => return "ctypes.c_void_p".to_string(),
            };
            let name = seg.ident.to_string();
            match name.as_str() {
                // Rust integer primitives
                "i8" => return "ctypes.c_int8".to_string(),
                "i16" => return "ctypes.c_int16".to_string(),
                "i32" => return "ctypes.c_int32".to_string(),
                "i64" => return "ctypes.c_int64".to_string(),
                "i128" => return "ctypes.c_int128".to_string(),
                "u8" => return "ctypes.c_uint8".to_string(),
                "u16" => return "ctypes.c_uint16".to_string(),
                "u32" => return "ctypes.c_uint32".to_string(),
                "u64" => return "ctypes.c_uint64".to_string(),
                "usize" | "uintptr_t" => return "ctypes.c_size_t".to_string(),
                "isize" | "intptr_t" => return "ctypes.c_ssize_t".to_string(),
                // Rust floating-point
                "f32" => return "ctypes.c_float".to_string(),
                "f64" => return "ctypes.c_double".to_string(),
                // Boolean
                "bool" => return "ctypes.c_bool".to_string(),
                // Void / unit — only meaningful behind a pointer
                "()" | "c_void" | "std::ffi::c_void" => {
                    return "ctypes.c_void_p".to_string();
                }
                // Integer types that might appear in cbindgen output
                "int8_t" => return "ctypes.c_int8".to_string(),
                "uint8_t" => return "ctypes.c_uint8".to_string(),
                "int16_t" => return "ctypes.c_int16".to_string(),
                "uint16_t" => return "ctypes.c_uint16".to_string(),
                "int32_t" => return "ctypes.c_int32".to_string(),
                "uint32_t" => return "ctypes.c_uint32".to_string(),
                "int64_t" => return "ctypes.c_int64".to_string(),
                "uint64_t" => return "ctypes.c_uint64".to_string(),
                "size_t" => return "ctypes.c_size_t".to_string(),
                "ptrdiff_t" => return "ctypes.c_ssize_t".to_string(),
                // char
                "char" | "c_char" => return "ctypes.c_char".to_string(),
                _ => {}
            }
            // Check generic args for constructing e.g. array types
            if !seg.arguments.is_empty() {
                return "ctypes.c_void_p".to_string();
            }
            // Unknown named type — assume it's a struct defined elsewhere
            name
        }
        Type::Array(arr) => {
            let elem = rust_type_to_ctypes(&arr.elem, _extra);
            let len_ref: &syn::Expr = &arr.len;
            if let syn::Expr::Lit(lit) = len_ref {
                if let syn::Lit::Int(n) = &lit.lit {
                    let count = n.base10_digits();
                    return format!("{} * {}", elem, count);
                }
            }
            format!("ctypes.POINTER({})", elem)
        }
        Type::Tuple(tup) => {
            if tup.elems.is_empty() {
                return "ctypes.c_void_p".to_string();
            }
            "ctypes.c_void_p".to_string()
        }
        // Any other complex type — conservative fallback
        _ => "ctypes.c_void_p".to_string(),
    }
}

// =====================================================================
// Python code generation
// =====================================================================

/// Main entry: produce a complete `.py` file string from the Rust source.
///
/// `rust_code` is the output of `registry.to_cbindgen_code()` — it contains all
/// types registered in the TypeRegistry (AbiMethods, AbiClass, FnTable).
/// `expanded_source` is the full `cargo expand` output — it may contain additional
/// `#[repr(C)]` plain structs (e.g. `Point`) that are referenced by name in Rust
/// function signatures but NOT registered in the TypeRegistry.
/// `c_header` is an optional C header generated by cbindgen — it contains
/// cross-crate `#[repr(C)]` struct definitions (e.g. `Point`, `Rectangle`) that
/// cannot be discovered from the expanded Rust source alone.
pub fn generate_python(rust_code: &str, lib_name: &str, c_header: Option<&str>) -> String {
    let file: File = syn::parse_str(rust_code)
        .expect("python.rs: failed to parse Rust code for Python generation");
    let (mut structs, entry_fns) = parse_items(&file);

    // ── Parse C header for plain structs (e.g. Point, Rectangle) ──
    let c_structs = c_header.map(parse_c_header_structs).unwrap_or_default();
    let existing_lower: std::collections::HashSet<String> =
        structs.iter().map(|s| s.name.to_lowercase()).collect();
    for cs in &c_structs {
        if existing_lower.contains(&cs.name.to_lowercase()) {
            continue;
        }
        if cs.name.starts_with("AbiMethods")
            || cs.name.starts_with("AbiClass")
            || cs.name.starts_with("Hicc_")
        {
            continue;
        }
        let fields: Vec<FieldDef> = cs
            .fields
            .iter()
            .map(|f| FieldDef::Basic {
                py_type: c_type_to_ctypes(&f.c_type, &c_structs),
                name: f.name.clone(),
            })
            .collect();
        structs.push(StructDef {
            name: cs.name.clone(),
            fields,
            kind: StructKind::Plain,
        });
    }

    let struct_names: Vec<String> = structs.iter().map(|s| s.name.clone()).collect();
    let abiclass_names: Vec<String> = structs
        .iter()
        .filter(|s| matches!(s.kind, StructKind::AbiClass))
        .map(|s| s.name.clone())
        .collect();

    // Pre-compute RAII names with collision resolution.
    // Two different AbiClass names can produce the same RAII name when
    // they differ only by letter casing (e.g. AbiClass_string_String vs
    // AbiClass_String_String).  We detect such collisions and assign the
    // lowercase-prefixed variant its stripped name as-is (no capitalisation),
    // while the uppercase-prefixed variant keeps the default behaviour.
    let abi_to_raii: std::collections::HashMap<String, String> = {
        // 1st pass: collect preliminary (abi_name, stripped, raii)
        let mut preliminary: Vec<(String, String, String)> = Vec::new();
        for abi in &abiclass_names {
            let stripped = abi.strip_prefix("AbiClass_").unwrap_or(abi);
            let raii = abiclass_to_raii_name(abi);
            preliminary.push((abi.clone(), stripped.to_string(), raii));
        }
        // 2nd pass: detect collisions and assign unique names
        let mut raii_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for (_, _, raii) in &preliminary {
            *raii_counts.entry(raii.clone()).or_insert(0) += 1;
        }
        let mut map = std::collections::HashMap::new();
        for (abi_name, stripped, raii) in &preliminary {
            if raii_counts.get(raii).copied().unwrap_or(0) == 1 {
                map.insert(abi_name.clone(), raii.clone());
            } else {
                // Collision: lowercase-prefixed → use stripped (original case)
                //             uppercase-prefixed → keep the default raii
                if stripped.starts_with(|c: char| c.is_lowercase()) {
                    map.insert(abi_name.clone(), stripped.clone());
                } else {
                    map.insert(abi_name.clone(), raii.clone());
                }
            }
        }
        map
    };

    let mut out = String::new();

    // ── Header ──────────────────────────────────────────────────────────
    out.push_str(&format!(
        r#""""Auto-generated by hicc-cbindgen -l python

Dynamic library: {lib_name}
"""
from __future__ import annotations
import ctypes
from typing import Optional

"#,
    ));

    // ── Library loading ─────────────────────────────────────────────────
    out.push_str(&format!(
        r#"# ---- Load dynamic library ----
_lib = ctypes.cdll.LoadLibrary({lib_name:?})

"#,
    ));

    // ── Forward declarations (all struct kinds) ──────────────────────
    let fwd_names: Vec<&str> = structs.iter().map(|s| s.name.as_str()).collect();

    if !fwd_names.is_empty() {
        out.push_str("# ---- Forward declarations ----\n");
        for name in &fwd_names {
            out.push_str(&format!("class {name}(ctypes.Structure):\n    pass\n\n"));
        }
    }

    // ── AbiMethods structs ──────────────────────────────────────────────
    for s in &structs {
        if !matches!(s.kind, StructKind::AbiMethods) {
            continue;
        }
        out.push_str(&format!(
            "# ---- {name} ----\n{name}._fields_ = [\n",
            name = s.name,
        ));
        for field in &s.fields {
            match field {
                FieldDef::FnPtr {
                    ret_py,
                    name,
                    params_py,
                } => {
                    let mut sig = vec![ret_py.as_str()];
                    sig.extend(params_py.iter().map(|s| s.as_str()));
                    let sig_str = sig.join(", ");
                    out.push_str(&format!("    ({name:?}, ctypes.CFUNCTYPE({sig_str})),\n"));
                }
                FieldDef::Basic { py_type, name } => {
                    out.push_str(&format!("    ({name:?}, {py_type}),\n"));
                }
            }
        }
        out.push_str("]\n\n");
    }

    // ── AbiClass structs ────────────────────────────────────────────────
    for s in &structs {
        if !matches!(s.kind, StructKind::AbiClass) {
            continue;
        }
        out.push_str(&format!("{name}._fields_ = [\n", name = s.name));
        for field in &s.fields {
            match field {
                FieldDef::Basic { py_type, name } => {
                    out.push_str(&format!("    ({name:?}, {py_type}),\n"));
                }
                FieldDef::FnPtr { .. } => {
                    // AbiClass should not have bare fn ptrs — skip
                }
            }
        }
        out.push_str("]\n\n");
    }

    // ── Plain structs (forward-declared above, now assign _fields_) ────
    for s in &structs {
        if !matches!(s.kind, StructKind::Plain) {
            continue;
        }
        out.push_str(&format!("{name}._fields_ = [\n", name = s.name));
        for field in &s.fields {
            match field {
                FieldDef::Basic { py_type, name } => {
                    out.push_str(&format!("    ({name:?}, {py_type}),\n"));
                }
                FieldDef::FnPtr {
                    ret_py,
                    name,
                    params_py,
                } => {
                    let mut sig = vec![ret_py.as_str()];
                    sig.extend(params_py.iter().map(|s| s.as_str()));
                    let sig_str = sig.join(", ");
                    out.push_str(&format!("    ({name:?}, ctypes.CFUNCTYPE({sig_str})),\n"));
                }
            }
        }
        out.push_str("]\n\n");
    }

    // ── Function table structs (export_lib, forward-declared above) ────
    for s in &structs {
        if !matches!(s.kind, StructKind::FnTable) {
            continue;
        }
        out.push_str(&format!("{name}._fields_ = [\n", name = s.name));
        for field in &s.fields {
            match field {
                FieldDef::FnPtr {
                    ret_py,
                    name,
                    params_py,
                } => {
                    let mut sig = vec![ret_py.as_str()];
                    sig.extend(params_py.iter().map(|s| s.as_str()));
                    let sig_str = sig.join(", ");
                    out.push_str(&format!("    ({name:?}, ctypes.CFUNCTYPE({sig_str})),\n"));
                }
                FieldDef::Basic { py_type, name } => {
                    out.push_str(&format!("    ({name:?}, {py_type}),\n"));
                }
            }
        }
        out.push_str("]\n\n");
    }

    // ── Entry point setup ───────────────────────────────────────────────
    for ef in &entry_fns {
        out.push_str(&format!(
            r#"# ---- Entry point ----
_lib.{fn_name}.restype = ctypes.POINTER({ret_struct})
_fn_{fn_name} = _lib.{fn_name}()

"#,
            fn_name = ef.name,
            ret_struct = ef.ret_struct,
        ));
    }

    // ── RAII wrappers ───────────────────────────────────────────────────
    for s in &structs {
        if !matches!(s.kind, StructKind::AbiClass) {
            continue;
        }
        let raii_name = abi_to_raii
            .get(&s.name)
            .cloned()
            .unwrap_or_else(|| abiclass_to_raii_name(&s.name));

        // Find corresponding AbiMethods
        let methods_name = s.name.replacen("AbiClass_", "AbiMethods_", 1);
        let methods_def = structs.iter().find(|s2| s2.name == methods_name);
        out.push_str(&format!("class {raii_name}:\n"));
        out.push_str(&format!(
            r#"    """Wrapper for {abi_name} — MUST call destroy() to free Rust heap memory.

Python's __del__ timing is unpredictable and NOT safe for releasing
Rust objects. You must explicitly call destroy() when done with this
object, or risk leaking Rust heap memory.
"""
"#,
            abi_name = s.name,
        ));

        // __init__
        out.push_str(&format!(
            r#"    def __init__(self, inner: {cname}):
        self._inner = inner

"#,
            cname = s.name,
        ));

        // destroy() — explicit cleanup method. Python GC timing is
        // unpredictable and __del__ cannot safely call hicc_destroy,
        // so the caller must invoke destroy() to free Rust heap memory.
        out.push_str(
            r#"    def destroy(self):
        if hasattr(self, '_inner') and self._inner is not None:
            _inner = self._inner
            self._inner = None
            _inner.methods[0].hicc_destroy(_inner)

"#,
        );

        // Method wrappers from AbiMethods
        if let Some(am) = methods_def {
            for field in &am.fields {
                if let FieldDef::FnPtr {
                    ret_py,
                    name,
                    params_py,
                } = field
                {
                    if name == "hicc_destroy" {
                        continue; // already wrapped as destroy()
                    }
                    // Strip hicc_ prefix for Python method name (like destroy)
                    let py_name = name.strip_prefix("hicc_").unwrap_or(name);
                    let ffi_name = name.as_str();
                    gen_method_wrapper(
                        &mut out,
                        py_name,
                        ffi_name,
                        ret_py,
                        params_py,
                        &s.name,
                        &struct_names,
                        &abi_to_raii,
                    );
                }
            }
        }

        out.push_str("\n");
    }

    // ── Factory functions ───────────────────────────────────────────────
    for ef in &entry_fns {
        let table = structs.iter().find(|s| s.name == ef.ret_struct);
        if let Some(t) = table {
            for field in &t.fields {
                if let FieldDef::FnPtr {
                    name,
                    params_py,
                    ret_py,
                } = field
                {
                    gen_factory_function(
                        &mut out,
                        name,
                        ret_py,
                        params_py,
                        &ef.name,
                        &struct_names,
                        &abiclass_names,
                        &abi_to_raii,
                    );
                }
            }
        }
    }

    out
}

// =====================================================================
// Helpers
// =====================================================================

/// Resolve RAII name from an AbiClass name, using the collision-resolved map
/// if available, falling back to the default `abiclass_to_raii_name`.
fn resolve_raii_name(
    abi_name: &str,
    abi_to_raii: &std::collections::HashMap<String, String>,
) -> String {
    abi_to_raii
        .get(abi_name)
        .cloned()
        .unwrap_or_else(|| abiclass_to_raii_name(abi_name))
}

/// Convert an AbiClass_* name to a Python RAII class name.
///   "AbiClass_Container_i32" → "Container_i32"
///   "AbiClass_str"           → "hicc_str" (avoid shadowing builtin `str`)
fn abiclass_to_raii_name(abi_name: &str) -> String {
    let stripped = abi_name.strip_prefix("AbiClass_").unwrap_or(abi_name);

    // Python reserved names → prefix with hicc_
    match stripped {
        "str" | "int" | "float" | "bool" | "list" | "dict" | "tuple" | "type" | "object"
        | "import" | "class" | "id" => {
            return format!("hicc_{}", stripped);
        }
        _ => {}
    }

    // Capitalise first letter for Python convention
    if let Some(first) = stripped.chars().next() {
        if first.is_lowercase() {
            let mut chars = stripped.chars();
            let f = chars.next().unwrap();
            return format!("{}{}", f.to_uppercase(), chars.as_str());
        }
    }

    stripped.to_string()
}

/// Generate a wrapper method on the RAII class.
/// `py_name` is the Python method name (e.g. "make_ref", "size_of").
/// `ffi_name` is the ctypes field name (e.g. "hicc_make_ref", "hicc_size_of").
fn gen_method_wrapper(
    out: &mut String,
    py_name: &str,
    ffi_name: &str,
    ret_py: &str,
    params_py: &[String],
    abi_class_name: &str,
    struct_names: &[String],
    abi_to_raii: &std::collections::HashMap<String, String>,
) {
    if params_py.is_empty() {
        let ret_trimmed = ret_py.trim();
        let ret_is_abiclass = struct_names.iter().any(|n| n.as_str() == ret_trimmed);
        out.push_str(&format!("    def {py_name}(self):\n"));
        out.push_str("        \"\"\"Ownership:\n");
        out.push_str(
            "        - `self`: borrowed — caller retains ownership; call destroy() to free\n",
        );
        if ret_trimmed == "None" {
            out.push_str("        - no return value\n");
        } else if ret_is_abiclass {
            let raii_ret = resolve_raii_name(ret_trimmed, abi_to_raii);
            out.push_str(&format!("        - returns new `{raii_ret}` — caller responsible for calling destroy() to free\n"));
        } else if ret_trimmed.starts_with("ctypes.POINTER(") {
            out.push_str("        - returns borrowed pointer — no ownership, no need to free\n");
        } else if ret_trimmed.starts_with("ctypes.c_") {
            out.push_str("        - returns value — no ownership concerns\n");
        } else {
            out.push_str("        - returns value — no ownership concerns\n");
        }
        out.push_str("        \"\"\"\n");
        if ret_trimmed == "None" {
            out.push_str(&format!("        self._inner.methods[0].{ffi_name}()\n\n"));
        } else {
            out.push_str(&format!(
                "        return self._inner.methods[0].{ffi_name}()\n\n"
            ));
        }
        return;
    }

    let consumes_self = params_py
        .first()
        .map(|p| {
            let clean = p.trim();
            clean == abi_class_name
        })
        .unwrap_or(false);

    let self_arg = if consumes_self {
        "self._inner".to_string()
    } else {
        "ctypes.byref(self._inner)".to_string()
    };

    let user_params: Vec<&String> = params_py.iter().skip(1).collect();
    let mut py_params = Vec::new();
    let mut call_args = vec![self_arg];
    let mut temps = Vec::new();
    let mut post_cleanup = Vec::new(); // statements after the FFI call
    let mut doc_notes: Vec<String> = Vec::new(); // ownership notes for docstring

    if consumes_self {
        doc_notes.push(
            "        - `self`: ownership consumed — object invalid after call; Rust frees via Drop"
                .to_string(),
        );
    } else {
        doc_notes.push(
            "        - `self`: borrowed — caller retains ownership; call destroy() to free"
                .to_string(),
        );
    }

    for (i, p) in user_params.iter().enumerate() {
        let base = p.trim();
        let pname = format!("arg{}", i);
        if base == "ctypes.c_char_p" {
            py_params.push(format!("{pname}: bytes"));
            call_args.push(format!("ctypes.c_char_p({pname})"));
            doc_notes.push(format!(
                "        - `{pname}`: C string (null-terminated bytes) — no ownership concerns"
            ));
            continue;
        }
        // Detect ctypes.POINTER(inner_type) — create a temp value and pass pointer
        if let Some(inner) = base.strip_prefix("ctypes.POINTER(") {
            if let Some(inner_type) = inner.strip_suffix(")") {
                if inner_type.starts_with("ctypes.c_") {
                    let tmp = format!("_val{}", i);
                    temps.push(format!("        {tmp} = {}({pname})", inner_type));
                    py_params.push(format!("{pname}: int"));
                    call_args.push(format!("ctypes.byref({tmp})"));
                    doc_notes.push(format!("        - `{pname}`: value parameter (output pointer) — no ownership concerns"));
                } else if inner_type.starts_with("AbiClass_") {
                    // Pointer to AbiClass: borrowed — caller retains ownership
                    let raii_name = resolve_raii_name(inner_type, abi_to_raii);
                    py_params.push(format!("{pname}: {raii_name}"));
                    call_args.push(format!("ctypes.byref({pname}._inner)"));
                    doc_notes.push(format!(
                        "        - `{pname}`: borrowed — caller retains ownership"
                    ));
                } else {
                    // Complex type: pass as-is with byref
                    py_params.push(pname.clone());
                    call_args.push(format!("ctypes.byref({pname})"));
                }
                continue;
            }
        }
        if base.starts_with("ctypes.") {
            py_params.push(format!("{pname}: int"));
            call_args.push(format!("{base}({pname})"));
            doc_notes.push(format!(
                "        - `{pname}`: value parameter — no ownership concerns"
            ));
        } else if base.starts_with("AbiClass_") && struct_names.iter().any(|n| n.as_str() == base) {
            // Value-passed AbiClass parameter: Rust receives a copy and
            // drops it automatically (owned: frees heap; Ref: mem::forget).
            // Python side only nullifies _inner to prevent reuse — no
            // hicc_destroy call (would double-free for owned types).
            let raii_name = resolve_raii_name(base, abi_to_raii);
            let inner_name = format!("_inner_{}", pname);
            temps.push(format!("        {} = {}.{}", inner_name, pname, "_inner"));
            post_cleanup.push(format!("        {}.{} = None", pname, "_inner"));
            py_params.push(format!("{pname}: {}", raii_name));
            call_args.push(inner_name);
            doc_notes.push(format!("        - `{pname}`: ownership transferred to Rust — Rust frees via Drop; do not use `{pname}` after call"));
        } else {
            py_params.push(pname.clone());
            call_args.push(pname);
        }
    }

    let py_params_str = py_params.join(", ");
    let call_args_str = call_args.join(", ");
    let temps_str = if temps.is_empty() {
        String::new()
    } else {
        temps.join("\n") + "\n"
    };
    let cleanup_str = if post_cleanup.is_empty() {
        String::new()
    } else {
        post_cleanup.join("\n") + "\n"
    };
    let maybe_comma = if py_params.is_empty() { "" } else { ", " };

    out.push_str(&format!(
        "    def {py_name}(self{comma}{params}):\n",
        comma = maybe_comma,
        params = py_params_str,
    ));

    // Always emit docstring with ownership notes for all params and return value
    let ret_trimmed = ret_py.trim();
    let ret_is_abiclass = struct_names.iter().any(|n| n.as_str() == ret_trimmed);
    out.push_str("        \"\"\"Ownership:\n");
    for note in &doc_notes {
        out.push_str(note);
        out.push_str("\n");
    }
    if ret_trimmed == "None" {
        out.push_str("        - no return value\n");
    } else if ret_is_abiclass {
        let raii_ret = resolve_raii_name(ret_trimmed, abi_to_raii);
        out.push_str(&format!("        - returns new `{raii_ret}` — caller responsible for calling destroy() to free\n"));
    } else if ret_trimmed.starts_with("ctypes.POINTER(") {
        out.push_str("        - returns borrowed pointer — no ownership, no need to free\n");
    } else if ret_trimmed.starts_with("ctypes.c_") {
        out.push_str("        - returns value — no ownership concerns\n");
    } else {
        out.push_str("        - returns value — no ownership concerns\n");
    }
    out.push_str("        \"\"\"\n");

    out.push_str(&temps_str);

    // --- Return type dispatch ---
    let ret_is_scalar = ret_trimmed.starts_with("ctypes.c_") || ret_trimmed == "None";

    if ret_trimmed == "None" {
        // void return
        if consumes_self {
            out.push_str(&format!(
                "        self._inner.methods[0].{ffi_name}({call_args})\n",
                call_args = call_args_str,
            ));
            out.push_str("        self._inner = None\n");
            out.push_str(&cleanup_str);
            out.push_str("\n");
        } else {
            out.push_str(&cleanup_str);
            out.push_str(&format!(
                "        return self._inner.methods[0].{ffi_name}({call_args})\n\n",
                call_args = call_args_str,
            ));
        }
    } else if ret_is_abiclass {
        // Return is a by-value AbiClass → wrap in RAII
        let raii_ret = resolve_raii_name(ret_trimmed, abi_to_raii);
        out.push_str(&format!(
            "        _inner = self._inner.methods[0].{ffi_name}({call_args})\n",
            call_args = call_args_str,
        ));
        if consumes_self {
            out.push_str("        self._inner = None\n");
        }
        out.push_str(&cleanup_str);
        out.push_str(&format!("        return {raii_ret}(_inner)\n\n"));
    } else if ret_is_scalar {
        // Scalar return (e.g. c_int32, c_bool, c_size_t)
        if consumes_self {
            out.push_str(&format!(
                "        _ret = self._inner.methods[0].{ffi_name}({call_args})\n",
                call_args = call_args_str,
            ));
            out.push_str("        self._inner = None\n");
            out.push_str(&cleanup_str);
            out.push_str("        return _ret\n\n");
        } else {
            out.push_str(&cleanup_str);
            out.push_str(&format!(
                "        return self._inner.methods[0].{ffi_name}({call_args})\n\n",
                call_args = call_args_str,
            ));
        }
    } else if ret_trimmed.starts_with("ctypes.POINTER(") || consumes_self {
        // Returns a pointer (or consumed self with complex return) → dereference check
        let double_deref = ret_trimmed.starts_with("ctypes.POINTER(ctypes.POINTER(");
        out.push_str(&format!(
            "        _ptr = self._inner.methods[0].{ffi_name}({call_args})\n",
            call_args = call_args_str,
        ));
        if consumes_self {
            out.push_str("        self._inner = None\n");
        }
        out.push_str(&cleanup_str);
        out.push_str("        if _ptr:\n");
        if double_deref {
            out.push_str("            return _ptr[0][0]\n");
        } else {
            out.push_str("            return _ptr[0]\n");
        }
        out.push_str("        return None\n\n");
    } else {
        out.push_str(&cleanup_str);
        out.push_str(&format!(
            "        return self._inner.methods[0].{ffi_name}({call_args})\n\n",
            call_args = call_args_str,
        ));
    }
}

/// Generate a module-level factory function.
fn gen_factory_function(
    out: &mut String,
    name: &str,
    ret_py: &str,
    params_py: &[String],
    entry_fn_name: &str,
    struct_names: &[String],
    abiclass_names: &[String],
    abi_to_raii: &std::collections::HashMap<String, String>,
) {
    let mut py_params = Vec::new();
    let mut call_args = Vec::new();
    let mut temps = Vec::new();
    let mut post_cleanup = Vec::new(); // nullify statements after FFI call
    let mut doc_notes: Vec<String> = Vec::new(); // ownership notes for docstring

    for (i, p) in params_py.iter().enumerate() {
        let base = p.trim();
        let pname = format!("arg{}", i);

        // C string parameter: accept bytes, auto-convert to null-terminated pointer
        if base == "ctypes.c_char_p" {
            py_params.push(format!("{pname}: bytes"));
            call_args.push(format!("ctypes.c_char_p({pname})"));
            doc_notes.push(format!(
                "    - `{pname}`: C string (null-terminated bytes) — no ownership concerns"
            ));
            continue;
        }

        // Case 1: ctypes.POINTER(InnerType) — could be pointer to scalar or AbiClass
        if let Some(inner) = base.strip_prefix("ctypes.POINTER(") {
            if let Some(inner_type) = inner.strip_suffix(")") {
                if inner_type.starts_with("ctypes.c_") {
                    let tmp = format!("_val{}", i);
                    temps.push(format!("    {tmp} = {}({pname})", inner_type));
                    py_params.push(format!("{pname}: int"));
                    call_args.push(format!("ctypes.byref({tmp})"));
                    doc_notes.push(format!(
                        "    - `{pname}`: value parameter (output pointer) — no ownership concerns"
                    ));
                } else if abiclass_names.iter().any(|n| n == inner_type) {
                    // Pointer to AbiClass: accept RAII wrapper, pass byref(_inner)
                    let raii_name = resolve_raii_name(inner_type, abi_to_raii);
                    py_params.push(format!("{pname}: {raii_name}"));
                    call_args.push(format!("ctypes.byref({pname}._inner)"));
                    doc_notes.push(format!(
                        "    - `{pname}`: borrowed — caller retains ownership"
                    ));
                } else if struct_names.iter().any(|n| n.as_str() == inner_type) {
                    py_params.push(format!("{pname}: {inner_type}"));
                    call_args.push(format!("ctypes.byref({pname})"));
                    doc_notes.push(format!(
                        "    - `{pname}`: borrowed — caller retains ownership"
                    ));
                } else {
                    // Unknown inner type: pass as-is
                    py_params.push(pname.clone());
                    call_args.push(format!("ctypes.byref({pname})"));
                }
                continue;
            }
        }

        // Case 2: ctypes.c_* scalar types — accept int, wrap with constructor
        if base.starts_with("ctypes.c_") {
            py_params.push(format!("{pname}: int"));
            call_args.push(format!("{base}({pname})"));
            doc_notes.push(format!(
                "    - `{pname}`: value parameter — no ownership concerns"
            ));
            continue;
        }

        // Case 3: AbiClass value-passed parameter (Rust side frees the copy)
        if abiclass_names.iter().any(|n| n == base) {
            let raii_name = resolve_raii_name(base, abi_to_raii);
            let inner_name = format!("_inner_{}", pname);
            temps.push(format!("    {} = {}.{}", inner_name, pname, "_inner"));
            post_cleanup.push(format!("    {}.{} = None", pname, "_inner"));
            py_params.push(format!("{pname}: {raii_name}"));
            call_args.push(inner_name);
            doc_notes.push(format!(
                "    - `{pname}`: ownership transferred to Rust — Rust frees via Drop; do not use `{pname}` after call"
            ));
            continue;
        }

        // Case 4: void / no parameter
        if base == "None" || base.is_empty() {
            continue;
        }

        // Case 5: other named types (plain structs etc.) — pass through
        py_params.push(pname.clone());
        call_args.push(pname);
    }

    let py_params_str = py_params.join(", ");
    let call_args_str = call_args.join(", ");
    let temps_str = if temps.is_empty() {
        String::new()
    } else {
        temps.join("\n") + "\n"
    };
    let cleanup_str = if post_cleanup.is_empty() {
        String::new()
    } else {
        post_cleanup.join("\n") + "\n"
    };

    let ret_trimmed = ret_py.trim();
    let ret_is_abiclass = abiclass_names.iter().any(|n| n == ret_trimmed);

    // Emit def line
    out.push_str(&format!("def {name}({params}):\n", params = py_params_str));

    // Always emit docstring with ownership notes for all params and return value
    out.push_str("    \"\"\"Ownership:\n");
    for note in &doc_notes {
        out.push_str(note);
        out.push_str("\n");
    }
    if ret_trimmed == "None" {
        out.push_str("    - no return value\n");
    } else if ret_is_abiclass {
        let raii_ret = resolve_raii_name(ret_trimmed, abi_to_raii);
        out.push_str(&format!(
            "    - returns new `{raii_ret}` — caller responsible for calling destroy() to free\n"
        ));
    } else if ret_trimmed.starts_with("ctypes.POINTER(") {
        out.push_str("    - returns borrowed pointer — no ownership, no need to free\n");
    } else if ret_trimmed.starts_with("ctypes.c_") {
        out.push_str("    - returns value — no ownership concerns\n");
    } else {
        out.push_str("    - returns value — no ownership concerns\n");
    }
    out.push_str("    \"\"\"\n");

    // Emit function body based on return type
    if ret_is_abiclass {
        let raii_ret = resolve_raii_name(ret_trimmed, abi_to_raii);
        out.push_str(&temps_str);
        out.push_str(&format!(
            "    _inner = _fn_{entry_fn_name}.contents.{name}({call_args})\n",
            entry_fn_name = entry_fn_name,
            call_args = call_args_str,
        ));
        out.push_str(&cleanup_str);
        out.push_str(&format!("    return {raii_ret}(_inner)\n\n"));
    } else if ret_trimmed == "None" || params_py.is_empty() && ret_trimmed == "None" {
        out.push_str(&temps_str);
        out.push_str(&cleanup_str);
        out.push_str(&format!(
            "    _fn_{entry_fn_name}.contents.{name}({call_args})\n\n",
            entry_fn_name = entry_fn_name,
            call_args = call_args_str,
        ));
    } else if ret_trimmed.starts_with("ctypes.POINTER(") {
        out.push_str(&temps_str);
        out.push_str(&format!(
            "    _ptr = _fn_{entry_fn_name}.contents.{name}({call_args})\n",
            entry_fn_name = entry_fn_name,
            call_args = call_args_str,
        ));
        out.push_str(&cleanup_str);
        out.push_str("    if _ptr:\n");
        out.push_str("        return _ptr[0]\n");
        out.push_str("    return None\n\n");
    } else {
        out.push_str(&temps_str);
        out.push_str(&cleanup_str);
        out.push_str(&format!(
            "    return _fn_{entry_fn_name}.contents.{name}({call_args})\n\n",
            entry_fn_name = entry_fn_name,
            call_args = call_args_str,
        ));
    }
}

// =====================================================================
// C Header parsing for cross-crate POD struct support
// =====================================================================

#[derive(Debug)]
pub struct CHeaderStruct {
    pub name: String,
    pub fields: Vec<CHeaderField>,
}

#[derive(Debug)]
pub struct CHeaderField {
    pub c_type: String,
    pub name: String,
}

/// Parse a C header string and extract typedef struct Name { ... } Name; definitions.
/// Also resolves typedef aliases (`typedef struct Orig Alias;`) by replacing alias
/// types with the canonical struct name in field definitions.
pub fn parse_c_header_structs(header: &str) -> Vec<CHeaderStruct> {
    let mut structs: Vec<CHeaderStruct> = Vec::new();
    let mut in_struct = false;
    let mut current_name = String::new();
    let mut current_fields: Vec<CHeaderField> = Vec::new();

    // First pass: collect typedef aliases (e.g. `typedef struct Point MyPoint;`)
    let mut aliases: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for line in header.lines() {
        let trimmed = line.trim();
        // Match: "typedef struct X AliasName;" (no '{')
        if trimmed.starts_with("typedef struct ")
            && !trimmed.contains('{')
            && trimmed.ends_with(';')
        {
            let body = trimmed
                .strip_prefix("typedef struct ")
                .unwrap()
                .strip_suffix(';')
                .unwrap()
                .trim();
            let parts: Vec<&str> = body.split_whitespace().collect();
            if parts.len() == 2 {
                let orig = parts[0];
                let alias = parts[1];
                aliases.insert(alias.to_string(), orig.to_string());
            }
        }
    }

    // Second pass: extract struct definitions and resolve aliases in field types
    for line in header.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("typedef struct ") && trimmed.contains('{') {
            let after_typedef = trimmed.strip_prefix("typedef struct ").unwrap().trim();
            let name_part = if let Some(idx) = after_typedef.find('{') {
                after_typedef[..idx].trim()
            } else {
                continue;
            };
            if name_part.is_empty() {
                continue;
            }
            current_name = name_part.to_string();
            current_fields.clear();
            in_struct = true;
            continue;
        }

        if in_struct {
            if trimmed.starts_with('}') {
                if !current_name.is_empty() {
                    structs.push(CHeaderStruct {
                        name: current_name.clone(),
                        fields: std::mem::take(&mut current_fields),
                    });
                }
                current_name.clear();
                in_struct = false;
                continue;
            }

            if !trimmed.is_empty()
                && !trimmed.starts_with("//")
                && !trimmed.starts_with("/*")
                && !trimmed.contains("(*")
            {
                let field_line = trimmed.strip_suffix(';').unwrap_or(trimmed);
                if let Some((c_type, field_name)) = split_c_type_and_name(field_line) {
                    let c_type_trimmed = c_type.trim();
                    let resolved = if let Some(orig) = aliases.get(c_type_trimmed) {
                        format!("struct {}", orig)
                    } else {
                        c_type_trimmed.to_string()
                    };
                    current_fields.push(CHeaderField {
                        c_type: resolved,
                        name: field_name.to_string(),
                    });
                }
            }
        }
    }

    structs
}

fn split_c_type_and_name(field_line: &str) -> Option<(&str, &str)> {
    let trimmed = field_line.trim();
    if trimmed.is_empty() {
        return None;
    }
    let tokens: Vec<&str> = trimmed.split_whitespace().collect();
    if tokens.len() < 2 {
        return None;
    }
    let last = tokens.last()?;
    if let Some(name) = last.strip_prefix('*') {
        if name.is_empty() {
            if tokens.len() < 3 {
                return None;
            }
            let name = tokens[tokens.len() - 2];
            let type_end = trimmed.len() - name.len() - last.len() - 1;
            return Some((trimmed[..type_end].trim(), name.trim_end_matches('*')));
        }
        let type_part = &trimmed[..trimmed.len() - last.len()];
        return Some((type_part.trim(), name));
    }
    let name = last;
    let type_part = &trimmed[..trimmed.len() - name.len()];
    Some((type_part.trim(), name))
}

/// Map a C type string to a ctypes expression.
fn c_type_to_ctypes(c_type: &str, known_structs: &[CHeaderStruct]) -> String {
    let c_type = c_type.trim();

    if c_type.ends_with('*') {
        let inner = c_type.trim_end_matches('*').trim();
        let inner = inner.strip_prefix("const ").unwrap_or(inner).trim();
        let inner_ct = c_type_to_ctypes(inner, known_structs);
        return format!("ctypes.POINTER({})", inner_ct);
    }

    if let Some(struct_name) = c_type.strip_prefix("struct ") {
        let name = struct_name.trim();
        if known_structs.iter().any(|s| s.name == name) {
            return name.to_string();
        }
        return "ctypes.c_void_p".to_string();
    }

    match c_type {
        "void" => "None".to_string(),
        "int8_t" => "ctypes.c_int8".to_string(),
        "uint8_t" => "ctypes.c_uint8".to_string(),
        "int16_t" => "ctypes.c_int16".to_string(),
        "uint16_t" => "ctypes.c_uint16".to_string(),
        "int32_t" => "ctypes.c_int32".to_string(),
        "uint32_t" => "ctypes.c_uint32".to_string(),
        "int64_t" => "ctypes.c_int64".to_string(),
        "uint64_t" => "ctypes.c_uint64".to_string(),
        "float" => "ctypes.c_float".to_string(),
        "double" => "ctypes.c_double".to_string(),
        "bool" | "_Bool" => "ctypes.c_bool".to_string(),
        "char" => "ctypes.c_char".to_string(),
        "size_t" | "uintptr_t" => "ctypes.c_size_t".to_string(),
        "ssize_t" | "intptr_t" | "ptrdiff_t" => "ctypes.c_ssize_t".to_string(),
        _ => {
            if known_structs.iter().any(|s| s.name == c_type) {
                return c_type.to_string();
            }
            "ctypes.c_void_p".to_string()
        }
    }
}
