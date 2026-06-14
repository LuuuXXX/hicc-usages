use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cpp_dir = manifest_dir.join("../cpp");

    // hicc-build parses src/lib.rs and compiles the generated C++ adapter.
    // .include(cpp_dir) lets the adapter find friend_function.h (Deref<Target=cc::Build>).
    hicc_build::Build::new()
        .rust_file("src/lib.rs")
        .include(&cpp_dir)
        .compile("friend_function_hicc");

    // Link the externally-built C++ static library (../cpp/build/libfriend_function.a).
    let cpp_build = manifest_dir.join("../cpp/build");
    println!("cargo::rustc-link-search=native={}", cpp_build.display());
    println!("cargo::rustc-link-lib=friend_function");
    println!("cargo::rustc-link-lib=stdc++");

    println!("cargo::rerun-if-changed=../cpp/build/libfriend_function.a");
    println!("cargo::rerun-if-changed=src/lib.rs");
}
