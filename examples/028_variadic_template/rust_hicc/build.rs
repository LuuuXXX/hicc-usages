fn main() {
    let cpp_dir = std::path::PathBuf::from("../cpp");
    let mut build = hicc_build::Build::new();
    use std::ops::DerefMut;
    let cc_build: &mut cc::Build = build.deref_mut();
    cc_build.include(&cpp_dir).include(".").cpp(true)
            .file(cpp_dir.join("variadic_template.cpp"));
    build.rust_file("src/lib.rs").compile("variadic_template");

    println!("cargo::rustc-link-lib=variadic_template");
    #[cfg(not(all(target_os = "windows", target_env = "msvc")))]
    println!("cargo::rustc-link-lib=stdc++");
    println!("cargo::rerun-if-changed=src/lib.rs");
    println!("cargo::rerun-if-changed=../cpp/variadic_template.cpp");
    println!("cargo::rerun-if-changed=../cpp/variadic_template.h");
}
