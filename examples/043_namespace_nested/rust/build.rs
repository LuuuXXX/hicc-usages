// 自动生成：hicc_usage_namespace_nested build.rs
// 调用 cmake/make 构建 ../cpp/ 下的 C++ 项目并链接到产物

use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cpp_dir = manifest.join("../cpp");
    let build_dir = cpp_dir.join("build");
    let libname = "namespace_nested";
    let lib_path = build_dir.join(if cfg!(windows) { format!("{}.lib", libname) } else { format!("lib{}.a", libname) });

    if !lib_path.exists() {
        std::fs::create_dir_all(&build_dir).ok();
        let cmake_ok = Command::new("cmake")
            .arg("-B").arg(&build_dir)
            .arg("-S").arg(&cpp_dir)
            .arg("-DCMAKE_BUILD_TYPE=Release")
            .status().map(|s| s.success()).unwrap_or(false);
        if cmake_ok {
            let _ = Command::new("cmake")
                .arg("--build").arg(&build_dir)
                .arg("--config").arg("Release")
                .status();
        }
        if !lib_path.exists() {
            let _ = Command::new("make").arg("-C").arg(&cpp_dir).status();
        }
        assert!(lib_path.exists(), "build.rs: failed to build C++ library `{}` in {}", libname, cpp_dir.display());
    }

    let mut build = hicc_build::Build::new();
    use std::ops::DerefMut;
    let cc_build: &mut cc::Build = build.deref_mut();
    cc_build.include(cpp_dir.join("include"));
    cc_build.include(".");

    build.rust_file("src/lib.rs").compile("hicc_usage_namespace_nested_adapter");

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static={}", libname);
    #[cfg(not(all(target_os = "windows", target_env = "msvc")))]
    println!("cargo:rustc-link-lib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=../cpp/include");
    println!("cargo:rerun-if-changed=../cpp/src");
}
