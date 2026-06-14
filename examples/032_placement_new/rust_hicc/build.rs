use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cpp_dir = manifest_dir.join("../cpp");

    // hicc-build parses src/lib.rs and compiles the generated C++ adapter.
    // .include(cpp_dir) lets the adapter find placement_new.h (Deref<Target=cc::Build>).
    hicc_build::Build::new()
        .rust_file("src/lib.rs")
        .include(&cpp_dir)
        .compile("placement_new_hicc");

    // Link the externally-built C++ static library (../cpp/build/libplacement_new.a).
    let cpp_build = manifest_dir.join("../cpp/build");
    println!("cargo::rustc-link-search=native={}", cpp_build.display());
    println!("cargo::rustc-link-lib=placement_new");
    println!("cargo::rustc-link-lib=stdc++");

    println!("cargo::rerun-if-changed=../cpp/build/libplacement_new.a");
    println!("cargo::rerun-if-changed=src/lib.rs");
}
