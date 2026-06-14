mod python;

use hiopt::options;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

// =====================================================================
// CLI — using hiopt (getopt-like)
// =====================================================================

static OPTS: hiopt::Options<'static> = options!["c:", "o:", "l:", "lib:"];

struct Args {
    crate_path: PathBuf,
    output: Option<PathBuf>,
    lang: String,
    lib_name: Option<String>,
}

fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
    let raw: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = raw.iter().map(|s| s.as_str()).collect();

    let mut crate_path = PathBuf::from(".");
    let mut output: Option<PathBuf> = None;
    let mut lang = String::from("c");
    let mut lib_name: Option<String> = None;

    for opt in OPTS.opt_iter(args.as_slice()) {
        let (idx, val) = opt.map_err(|e| format!("option error: {:?}", e))?;
        let val = val.ok_or_else(|| format!("option at index {} requires a value", idx))?;
        match idx {
            0 => crate_path = PathBuf::from(val),
            1 => output = Some(PathBuf::from(val)),
            2 => lang = val.to_string(),
            3 => lib_name = Some(val.to_string()),
            _ => unreachable!(),
        }
    }

    Ok(Args {
        crate_path,
        output,
        lang,
        lib_name,
    })
}

// =====================================================================
// Step 1: Crate info (name, targets, hicc-rs path)
// =====================================================================

struct CrateInfo {
    /// Package name as it appears in Cargo.toml (e.g. "hicc-test-lib").
    pkg_name: String,
    /// Crate name usable in Rust code (hyphens → underscores).
    rust_name: String,
    /// Whether the crate has a library target.
    has_lib: bool,
    /// All direct dependencies of the target crate (excluding dev-dependencies).
    deps: Vec<Dep>,
    /// hicc-rs dependency info (name + path), if found.
    hicc_rs_dep: Option<Dep>,
}

fn get_crate_info(manifest_path: &Path) -> Result<CrateInfo, Box<dyn std::error::Error>> {
    let manifest_abs = manifest_path.canonicalize()?;
    let manifest_str = manifest_abs.to_string_lossy().to_string();

    let output = Command::new("cargo")
        .args(["metadata", "--manifest-path"])
        .arg(&manifest_str)
        .args(["--format-version", "1"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let packages = metadata["packages"]
        .as_array()
        .ok_or("No packages in metadata")?;

    let pkg = packages
        .iter()
        .find(|p| p["manifest_path"].as_str() == Some(&manifest_str))
        .ok_or_else(|| format!("Package not found for manifest: {}", manifest_str))?;

    let pkg_name = pkg["name"].as_str().ok_or("Cannot find package name")?;
    let has_lib = pkg["targets"]
        .as_array()
        .map(|targets| {
            targets.iter().any(|t| {
                t["kind"]
                    .as_array()
                    .map(|k| {
                        k.iter().any(|k| {
                            matches!(
                                k.as_str(),
                                Some(
                                    "lib"
                                        | "rlib"
                                        | "dylib"
                                        | "cdylib"
                                        | "staticlib"
                                        | "proc-macro"
                                )
                            )
                        })
                    })
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false);

    // Collect all direct dependencies from the target package.
    let deps = parse_deps_from_metadata(pkg)?;
    let hicc_rs_dep = deps.iter().find(|d| d.name == "hicc-rs").cloned();
    // If hicc-rs is not a direct dep, fall back to scanning the full tree.
    let hicc_rs_dep = match hicc_rs_dep {
        Some(d) => Some(d),
        None => find_hicc_rs_dep(&metadata)?,
    };

    Ok(CrateInfo {
        pkg_name: pkg_name.to_string(),
        rust_name: pkg_name.replace('-', "_"),
        has_lib,
        deps,
        hicc_rs_dep,
    })
}

/// A single dependency entry: either a path dep (local crate) or registry dep.
#[derive(Clone)]
struct Dep {
    name: String,
    /// Local path, if this is a path dependency.
    path: Option<PathBuf>,
    /// Version requirement string (e.g. "^1.0"), used for registry deps.
    version: Option<String>,
    /// Feature flags to enable.
    features: Vec<String>,
    /// Whether default features should be used.
    use_default_features: bool,
}

impl Dep {
    /// Format this dependency as a TOML table entry.
    fn to_toml_entry(&self) -> String {
        let mut parts = Vec::new();
        if let Some(p) = &self.path {
            let p_str = p.to_string_lossy();
            parts.push(format!("path = {p_str:?}"));
        } else if let Some(v) = &self.version {
            parts.push(format!("version = {v:?}"));
        }
        if !self.use_default_features {
            parts.push("default-features = false".into());
        }
        if !self.features.is_empty() {
            let feat_list: Vec<String> = self.features.iter().map(|f| format!("{f:?}")).collect();
            parts.push(format!("features = [{}]", feat_list.join(", ")));
        }
        format!("{} = {{ {} }}", self.name, parts.join(", "))
    }
}

/// Parse all direct dependencies from a cargo-metadata package entry
/// (excluding dev-dependencies and build-dependencies).
fn parse_deps_from_metadata(
    pkg: &serde_json::Value,
) -> Result<Vec<Dep>, Box<dyn std::error::Error>> {
    let mut deps = Vec::new();
    if let Some(dep_array) = pkg["dependencies"].as_array() {
        for dep in dep_array {
            let name = dep["name"].as_str().ok_or("dep missing name")?.to_string();
            let path = dep["path"].as_str().map(|p| {
                let pb = PathBuf::from(p);
                if pb.is_absolute() {
                    pb
                } else {
                    pb.canonicalize().unwrap_or(pb)
                }
            });
            let version = dep["req"].as_str().map(|s| s.to_string());
            let features: Vec<String> = dep["features"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            let use_default_features = dep["uses_default_features"].as_bool().unwrap_or(true);
            // Skip optional deps — they are only pulled in when enabled.
            if dep["optional"].as_bool().unwrap_or(false) {
                continue;
            }
            deps.push(Dep {
                name,
                path,
                version,
                features,
                use_default_features,
            });
        }
    }
    Ok(deps)
}

fn find_hicc_rs_dep(
    metadata: &serde_json::Value,
) -> Result<Option<Dep>, Box<dyn std::error::Error>> {
    // Find hicc-rs anywhere in the resolved package tree.
    if let Some(packages) = metadata["packages"].as_array() {
        for p in packages {
            if p["name"].as_str() == Some("hicc-rs") {
                if let Some(mp) = p["manifest_path"].as_str() {
                    if let Some(parent) = Path::new(mp).parent() {
                        return Ok(Some(Dep {
                            name: "hicc-rs".to_string(),
                            path: Some(parent.to_path_buf()),
                            version: None,
                            features: vec![],
                            use_default_features: true,
                        }));
                    }
                }
            }
        }
    }
    Ok(None)
}

// =====================================================================
// Step 2: Discover *_cbindgen functions via cargo expand
// =====================================================================

struct CbindgenFunction {
    /// Module path, e.g. "lib::demo"
    base_path: String,
}

struct ExpandResult {
    functions: Vec<CbindgenFunction>,
}

fn expand_lib(manifest_path: &Path) -> Result<ExpandResult, Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args([
            "expand",
            "--features",
            "cbindgen",
            "--ugly",
            "--lib",
            "--manifest-path",
        ])
        .arg(manifest_path)
        .env("RUSTC_BOOTSTRAP", "1")
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "cargo expand failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let expanded = String::from_utf8(output.stdout)?;
    let mut file: syn::File = syn::parse_str(&expanded)?;
    file.items
        .retain(|item| !matches!(item, syn::Item::Fn(f) if f.sig.ident == "main"));

    let mut functions = Vec::new();
    walk_items_for_cbindgen(&file.items, "", &mut functions);
    Ok(ExpandResult { functions })
}

fn walk_items_for_cbindgen(items: &[syn::Item], prefix: &str, out: &mut Vec<CbindgenFunction>) {
    for item in items {
        match item {
            syn::Item::Fn(func) => {
                let name = func.sig.ident.to_string();
                if let Some(base) = name.strip_suffix("_cbindgen") {
                    if func.sig.inputs.len() == 1 {
                        let bp = if prefix.is_empty() {
                            base.to_string()
                        } else {
                            format!("{}::{}", prefix, base)
                        };
                        out.push(CbindgenFunction { base_path: bp });
                    }
                }
            }
            syn::Item::Mod(m) => {
                if let Some((_, items)) = &m.content {
                    let np = if prefix.is_empty() {
                        m.ident.to_string()
                    } else {
                        format!("{}::{}", prefix, m.ident)
                    };
                    walk_items_for_cbindgen(items, &np, out);
                }
            }
            _ => {}
        }
    }
}

// =====================================================================
// Step 3: Create & run a temporary helper crate to call *_cbindgen
// =====================================================================

fn run_helper_for_lib(
    crate_dir: &Path,
    crate_info: &CrateInfo,
    expand_result: &ExpandResult,
) -> Result<String, Box<dyn std::error::Error>> {
    let tmp_dir = tempdir()?;
    let src_dir = tmp_dir.path().join("src");
    std::fs::create_dir_all(&src_dir)?;

    let crate_abs = crate_dir.canonicalize()?;

    // Build helper Cargo.toml.
    // Include ALL dependencies so that types from dependency crates
    // (e.g. structs used in #[export_class] interfaces) resolve correctly,
    // even when the helper crate lives outside the original workspace.
    let mut dep_lines = String::new();

    // 1. The target library itself — enable its `cbindgen` feature so
    //    #[cfg(feature = "cbindgen")] cbindgen functions become available.
    dep_lines.push_str(&format!(
        r#"{pkg} = {{ path = "{path}", features = ["cbindgen"] }}"#,
        pkg = crate_info.pkg_name,
        path = crate_abs.display(),
    ));

    // 2. All direct dependencies of the target crate (except hicc-rs, which
    //    is added separately below so the helper can use it directly).
    for dep in &crate_info.deps {
        if dep.name == "hicc-rs" {
            continue;
        }
        dep_lines.push_str(&format!("\n{}", dep.to_toml_entry()));
    }

    // 3. hicc-rs as a direct dep so the helper can use `hicc_rs::TypeRegistry`.
    if let Some(hicc_dep) = &crate_info.hicc_rs_dep {
        dep_lines.push_str(&format!("\n{}", hicc_dep.to_toml_entry()));
    }
    let helper_cargo = format!(
        r#"[package]
name = "hicc_cbindgen_helper"
version = "0.1.0"
edition = "2021"

[dependencies]
{}
"#,
        dep_lines
    );
    std::fs::write(tmp_dir.path().join("Cargo.toml"), &helper_cargo)?;

    // Build helper src/main.rs
    let crate_ident = &crate_info.rust_name;
    let mut main_body = String::new();
    main_body.push_str("fn main() {\n");
    main_body.push_str("    let mut registry = hicc_rs::TypeRegistry::new();\n");
    main_body.push_str("    let mut entries = Vec::new();\n");
    for func in &expand_result.functions {
        main_body.push_str(&format!(
            "    entries.push({}::{}_cbindgen(&mut registry));\n",
            crate_ident, func.base_path
        ));
    }
    main_body.push_str("    let code = registry.to_cbindgen_code(entries.join(\"\\n\"));\n");
    main_body.push_str("    println!(\"{}\", code);\n");
    main_body.push_str("}\n");
    std::fs::write(src_dir.join("main.rs"), &main_body)?;

    eprintln!("Building helper crate...");
    let status = Command::new("cargo")
        .args(["build", "-q"])
        .current_dir(tmp_dir.path())
        .env("RUSTC_BOOTSTRAP", "1")
        .status()?;
    if !status.success() {
        let out = Command::new("cargo")
            .args(["build"])
            .current_dir(tmp_dir.path())
            .env("RUSTC_BOOTSTRAP", "1")
            .output()?;
        return Err(format!(
            "Helper crate build failed:\n{}",
            String::from_utf8_lossy(&out.stderr)
        )
        .into());
    }

    eprintln!("Running helper crate...");
    let output = Command::new("cargo")
        .args(["run", "-q"])
        .current_dir(tmp_dir.path())
        .env("RUSTC_BOOTSTRAP", "1")
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "Helper crate run failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let code = String::from_utf8(output.stdout)?;
    Ok(code)
}

// =====================================================================
// Step 4: Generate C header via cbindgen crate
// =====================================================================

/// Restructure cbindgen's Cython .pxd for the companion-header pattern.
///
/// cbindgen wraps everything inside `cdef extern from *:` blocks. This function
/// restructures the output into `cdef extern from "{basename}.h":` containing:
///   - forward declarations for all struct types
///   - struct definitions (`struct Name: ...`)
///   - function declarations
///
/// Cython will emit `#include "{basename}.h"`, giving the C compiler real
/// function prototypes for `demo()` etc.
fn restructure_cython_pxd(pxd: &str, basename: &str) -> String {
    let mut imports: Vec<String> = Vec::new();
    let mut ctypedef_lines: Vec<String> = Vec::new();
    let mut struct_defs: Vec<(String, Vec<String>)> = Vec::new();
    let mut func_decls: Vec<String> = Vec::new();

    let mut in_extern = false;
    let mut in_cur_struct = false;
    let mut cur_header = String::new();
    let mut cur_body: Vec<String> = Vec::new();

    let flush_cur = |defs: &mut Vec<(String, Vec<String>)>, h: &mut String, b: &mut Vec<String>| {
        if !h.is_empty() {
            defs.push((std::mem::take(h), std::mem::take(b)));
        }
    };

    for line in pxd.lines() {
        let trimmed = line.trim();

        // Collect imports (keep at top of output)
        if trimmed.starts_with("from ") || trimmed.starts_with("cimport ") {
            imports.push(line.to_string());
            continue;
        }

        // Switch to extern-processing mode on `cdef extern`.
        // (cbindgen outputs ALL non-import content inside `cdef extern from *:`)
        if trimmed.starts_with("cdef extern") {
            flush_cur(&mut struct_defs, &mut cur_header, &mut cur_body);
            in_extern = true;
            in_cur_struct = false;
            continue;
        }

        if !in_extern {
            continue;
        }

        // === Inside cdef extern from *: ===

        // Skip empty lines and pass.
        if trimmed.is_empty() || trimmed == "pass" {
            continue;
        }

        // ctypedef lines (bint bool → int bool to match <stdbool.h>)
        if trimmed.starts_with("ctypedef ") {
            flush_cur(&mut struct_defs, &mut cur_header, &mut cur_body);
            in_cur_struct = false;
            if trimmed == "ctypedef bint bool" {
                ctypedef_lines.push("    ctypedef int bool".to_string());
            } else {
                ctypedef_lines.push(format!("    {}", trimmed));
            }
            continue;
        }

        // cdef struct — forward declaration (no colon) or definition (with colon)
        if trimmed.starts_with("cdef struct ") {
            flush_cur(&mut struct_defs, &mut cur_header, &mut cur_body);
            let rest = &trimmed["cdef struct ".len()..];
            if rest.ends_with(':') {
                cur_header = format!("    struct {}", rest);
                in_cur_struct = true;
            } else {
                // Forward declaration — ignore; we emit our own below.
                in_cur_struct = false;
            }
            continue;
        }

        // Struct body lines (4-space indent in raw cbindgen output)
        if in_cur_struct && line.starts_with("    ") {
            let body = line.trim_end().replace(" bool ", " int ");
            cur_body.push(format!("        {}", body.trim()));
            continue;
        }

        // Function declarations: contain '(' or end with ';'
        if trimmed.contains("(") || trimmed.ends_with(';') {
            flush_cur(&mut struct_defs, &mut cur_header, &mut cur_body);
            in_cur_struct = false;
            func_decls.push(format!("    {}", trimmed));
            continue;
        }
    }

    // Flush last struct (if any)
    flush_cur(&mut struct_defs, &mut cur_header, &mut cur_body);

    // Collect struct type names for forward declarations.
    let mut struct_names: Vec<String> = Vec::new();
    for (header, _) in &struct_defs {
        let hdr = header.trim();
        if let Some(name) = hdr
            .strip_prefix("struct ")
            .and_then(|s| s.strip_suffix(':'))
        {
            if !struct_names.contains(&name.to_string()) {
                struct_names.push(name.to_string());
            }
        }
    }

    // === Build output ===
    let mut out = String::new();

    for line in &imports {
        out.push_str(line);
        out.push('\n');
    }
    out.push('\n');

    out.push_str(&format!("cdef extern from \"{}\":\n", basename));

    // 1. Forward declarations (no semicolons — Cython syntax)
    for name in &struct_names {
        out.push_str(&format!("    struct {}\n", name));
    }
    if !struct_names.is_empty() {
        out.push('\n');
    }

    // 2. ctypedef lines
    for line in &ctypedef_lines {
        out.push_str(line);
        out.push('\n');
    }
    if !ctypedef_lines.is_empty() {
        out.push('\n');
    }

    // 3. Struct definitions
    for (header, body) in &struct_defs {
        out.push_str(header);
        out.push('\n');
        for b in body {
            out.push_str(b);
            out.push('\n');
        }
        out.push('\n');
    }

    // 4. Function declarations
    for line in &func_decls {
        out.push_str(line);
        out.push('\n');
    }

    out
}

/// Result of header generation — the primary output (string) and an optional
/// companion C header (used when generating `.pxd` files).
struct HeaderResult {
    /// Primary output content.
    content: String,
    /// Companion C header content (Some when `lang == "cython"`).
    companion_h: Option<String>,
}

fn generate_c_header(
    rust_code: &str,
    lang: &str,
    crate_abs_path: &Path,
    pkg_name: &str,
    output_path: Option<&Path>,
    lib_name: Option<&str>,
) -> Result<HeaderResult, Box<dyn std::error::Error>> {
    let tmp_dir = tempdir()?;
    let src_dir = tmp_dir.path().join("src");
    std::fs::create_dir_all(&src_dir)?;

    let lib_rs = format!(
        "#![allow(dead_code, non_camel_case_types, non_snake_case)]\n{}",
        rust_code
    );
    std::fs::write(src_dir.join("lib.rs"), &lib_rs)?;

    // Include the original crate as a dependency so cbindgen can resolve
    // #[repr(C)] struct definitions (e.g. Point) via with_parse_deps(true).
    let cargo_toml = format!(
        r#"[package]
name = "hicc_cbindgen_temp"
version = "0.1.0"
edition = "2021"

[dependencies]
{name} = {{ path = {path:?} }}
"#,
        name = pkg_name,
        path = crate_abs_path,
    );
    std::fs::write(tmp_dir.path().join("Cargo.toml"), cargo_toml)?;

    let gen_for_lang = |lang: cbindgen::Language| -> Result<String, Box<dyn std::error::Error>> {
        let bindings = cbindgen::Builder::new()
            .with_crate(tmp_dir.path())
            .with_language(lang)
            .with_parse_deps(true)
            .generate()?;
        let mut buf = Vec::new();
        bindings.write(&mut buf);
        let mut out = String::from_utf8(buf)?;

        // Strip spurious typedefs pulled in from dependency crates (e.g.
        // prettyplease's FixupContext/Precedence).
        out = out
            .lines()
            .filter(|line| !(line.contains("FixupContext") || line.contains("Precedence")))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(out)
    };

    if lang == "cython" {
        let mut c_header = gen_for_lang(cbindgen::Language::C)?;

        // cbindgen's C output may reference struct tags inside function-pointer
        // parameters before defining them; insert forward declarations.
        let mut struct_names: Vec<String> = Vec::new();
        for cap in regex::Regex::new(r"(?:typedef\s+)?struct\s+(\w+)\s*\{")
            .unwrap()
            .captures_iter(&c_header)
        {
            let name = cap[1].to_string();
            if !struct_names.contains(&name) {
                struct_names.push(name);
            }
        }
        if let Some(pos) = c_header.find("\n\n") {
            let mut fwd = String::new();
            for name in &struct_names {
                fwd.push_str(&format!("struct {};\n", name));
            }
            fwd.push('\n');
            c_header.insert_str(pos + 2, &fwd);
        }

        let cython_output = gen_for_lang(cbindgen::Language::Cython)?;
        let companion_basename = match output_path {
            Some(p) => format!("{}.h", p.file_stem().unwrap().to_string_lossy()),
            None => format!("{}.h", pkg_name.replace('-', "_")),
        };
        let pxd = restructure_cython_pxd(&cython_output, &companion_basename);

        return Ok(HeaderResult {
            content: pxd,
            companion_h: Some(c_header),
        });
    }

    if lang == "python" {
        let lib_name_str = match lib_name {
            Some(name) => name.to_string(),
            None => format!("lib{}.so", pkg_name.replace('-', "_")),
        };
        let c_header_output = gen_for_lang(cbindgen::Language::C)?;
        let py_code = python::generate_python(rust_code, &lib_name_str, Some(&c_header_output));
        return Ok(HeaderResult {
            content: py_code,
            companion_h: None,
        });
    }

    let language = match lang {
        "cxx" | "c++" | "cpp" => cbindgen::Language::Cxx,
        _ => cbindgen::Language::C,
    };
    let mut content = gen_for_lang(language)?;

    // Insert forward declarations for struct tags that cbindgen may reference
    // inside function-pointer parameters before defining them.
    let mut struct_names: Vec<String> = Vec::new();
    for cap in regex::Regex::new(r"(?:typedef\s+)?struct\s+(\w+)\s*\{")
        .unwrap()
        .captures_iter(&content)
    {
        let name = cap[1].to_string();
        if !struct_names.contains(&name) {
            struct_names.push(name);
        }
    }
    if let Some(pos) = content.find("\n\n") {
        let mut fwd = String::new();
        for name in &struct_names {
            fwd.push_str(&format!("struct {};\n", name));
        }
        fwd.push('\n');
        content.insert_str(pos + 2, &fwd);
    }

    Ok(HeaderResult {
        content,
        companion_h: None,
    })
}

// =====================================================================
// Main
// =====================================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;
    let manifest_path = args.crate_path.join("Cargo.toml");
    if !manifest_path.exists() {
        return Err(format!("No Cargo.toml found at {:?}", args.crate_path).into());
    }

    let crate_info = get_crate_info(&manifest_path)?;
    if !crate_info.has_lib {
        return Err(format!(
            "Crate '{}' is not a library (no lib target). hicc-cbindgen requires a library crate.",
            crate_info.rust_name
        )
        .into());
    }

    eprintln!("[1/3] Expanding library and discovering cbindgen functions...");
    let functions = expand_lib(&manifest_path)?;
    if functions.functions.is_empty() {
        return Err(
            "No *_cbindgen functions found — does the crate use #[export_lib] or #[export_class]?"
                .into(),
        );
    }
    eprintln!("Found {} cbindgen function(s):", functions.functions.len());
    for f in &functions.functions {
        eprintln!("  - {}_cbindgen", f.base_path);
    }

    eprintln!("[2/3] Creating and running helper crate...");
    let rust_code = run_helper_for_lib(&args.crate_path, &crate_info, &functions)?;
    eprintln!("Helper produced {} bytes of Rust code", rust_code.len());

    if args.lang == "python" {
        eprintln!("[3/3] Generating Python bindings...");
    } else {
        eprintln!("[3/3] Generating C header via cbindgen...");
    }
    let crate_abs = args.crate_path.canonicalize()?;
    let result = generate_c_header(
        &rust_code,
        &args.lang,
        &crate_abs,
        &crate_info.pkg_name,
        args.output.as_deref(),
        args.lib_name.as_deref(),
    )?;

    match &args.output {
        Some(path) => {
            std::fs::write(path, &result.content)?;
            if args.lang == "python" {
                eprintln!("Python bindings written to {}", path.display());
            } else {
                eprintln!("C header written to {}", path.display());
            }
            if let Some(companion_h) = &result.companion_h {
                let h_path = path.with_extension("h");
                std::fs::write(&h_path, companion_h)?;
                eprintln!("  (companion C header: {})", h_path.display());
            }
        }
        None => {
            println!("{}", result.content);
        }
    }

    Ok(())
}
