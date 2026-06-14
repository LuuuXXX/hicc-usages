use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cpp_dir = manifest_dir.join("../cpp");

    // hicc-build parses src/lib.rs and compiles the generated C++ adapter.
    // .include(cpp_dir) lets the adapter find explicit_ctor.h (Deref<Target=cc::Build>).
    hicc_build::Build::new()
        .rust_file("src/lib.rs")
        .include(&cpp_dir)
        .compile("explicit_ctor_hicc");

    // Link the externally-built C++ static library (../cpp/build/libexplicit_ctor.a).
    let cpp_build = manifest_dir.join("../cpp/build");
    println!("cargo::rustc-link-search=native={}", cpp_build.display());
    println!("cargo::rustc-link-lib=explicit_ctor");
    println!("cargo::rustc-link-lib=stdc++");

    println!("cargo::rerun-if-changed=../cpp/build/libexplicit_ctor.a");
    println!("cargo::rerun-if-changed=src/lib.rs");
}
