use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cpp_dir = manifest_dir.join("../cpp");

    // hicc-build parses src/lib.rs and compiles the generated C++ adapter.
    // .include(cpp_dir) lets the adapter find noexcept_basic.h (Deref<Target=cc::Build>).
    hicc_build::Build::new()
        .rust_file("src/lib.rs")
        .include(&cpp_dir)
        .compile("noexcept_basic_hicc");

    // Link the externally-built C++ static library (../cpp/build/libnoexcept_basic.a).
    let cpp_build = manifest_dir.join("../cpp/build");
    println!("cargo::rustc-link-search=native={}", cpp_build.display());
    println!("cargo::rustc-link-lib=noexcept_basic");
    println!("cargo::rustc-link-lib=stdc++");

    println!("cargo::rerun-if-changed=../cpp/build/libnoexcept_basic.a");
    println!("cargo::rerun-if-changed=src/lib.rs");
}
