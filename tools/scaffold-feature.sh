#!/usr/bin/env bash
# Scaffold a new C++ feature directory under examples/.
#
# Usage: scaffold-feature.sh <NNN>_<name>
#   e.g. scaffold-feature.sh 001_hello_world
#
# Creates: examples/<arg>/{cpp,ast,rust_hicc,README.md}
set -euo pipefail

if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <NNN>_<name>" >&2
    exit 1
fi

FEATURE="$1"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TEMPLATE_DIR="$REPO_ROOT/tools/cpp-templates"
TARGET="$REPO_ROOT/examples/$FEATURE"

if [ -e "$TARGET" ]; then
    echo "error: $TARGET already exists" >&2
    exit 2
fi

# Derive lib name from feature id: 001_hello_world -> hello_world
NAME="${FEATURE#*_}"
NUM="${FEATURE%%_*}"

mkdir -p "$TARGET/cpp" "$TARGET/ast" "$TARGET/rust_hicc/src" "$TARGET/rust_hicc/tests"

# --- cpp/ : skeleton + three build systems ---
cat > "$TARGET/cpp/${NAME}.h" <<EOF
#pragma once

// TODO: declare public API for ${NAME}.
EOF

cat > "$TARGET/cpp/${NAME}.cpp" <<EOF
#include "${NAME}.h"

// TODO: implement API for ${NAME}.
EOF

cat > "$TARGET/cpp/main.cpp" <<EOF
#include "${NAME}.h"
#include <iostream>

int main() {
    std::cout << "demo: ${NAME}" << std::endl;
    // TODO: exercise the API.
    return 0;
}
EOF

# Substitute templates
sed "s|<NAME>|${NAME}|g; s|<SOURCES>|${NAME}.cpp|g" \
    "$TEMPLATE_DIR/standalone.sh.tmpl" > "$TARGET/cpp/standalone.sh"
sed "s|<NAME>|${NAME}|g; s|<SOURCES>|${NAME}.cpp|g" \
    "$TEMPLATE_DIR/Makefile.tmpl" > "$TARGET/cpp/Makefile"
sed "s|<NAME>|${NAME}|g; s|<SOURCES>|${NAME}.cpp|g" \
    "$TEMPLATE_DIR/CMakeLists.txt.tmpl" > "$TARGET/cpp/CMakeLists.txt"

chmod +x "$TARGET/cpp/standalone.sh"

# --- rust_hicc/ : Cargo project ---
cat > "$TARGET/rust_hicc/Cargo.toml" <<EOF
[package]
name = "${NAME}"
version = "0.1.0"
edition = "2021"

[dependencies]
hicc = { path = "../../../hicc/hicc", version = "0.2" }

[build-dependencies]
hicc-build = { path = "../../../hicc/hicc-build", version = "0.2" }

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
EOF

cat > "$TARGET/rust_hicc/build.rs" <<EOF
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cpp_dir = manifest_dir.join("../cpp");

    // hicc-build parses src/lib.rs and compiles the generated C++ adapter.
    // .include(cpp_dir) lets the adapter find ${NAME}.h (Deref<Target=cc::Build>).
    hicc_build::Build::new()
        .rust_file("src/lib.rs")
        .include(&cpp_dir)
        .compile("${NAME}_hicc");

    // Link the externally-built C++ static library (../cpp/build/lib${NAME}.a).
    let cpp_build = manifest_dir.join("../cpp/build");
    println!("cargo::rustc-link-search=native={}", cpp_build.display());
    println!("cargo::rustc-link-lib=${NAME}");
    println!("cargo::rustc-link-lib=stdc++");

    println!("cargo::rerun-if-changed=../cpp/build/lib${NAME}.a");
    println!("cargo::rerun-if-changed=src/lib.rs");
}
EOF

cat > "$TARGET/rust_hicc/src/lib.rs" <<EOF
// Declare the C++ API by including our header, then bind via hicc macros.
// The actual implementations are linked from ../cpp/build/lib${NAME}.a.

hicc::cpp! {
    #include "${NAME}.h"
}

hicc::import_lib! {
    #![link_name = "${NAME}_hicc"]

    // TODO: declare bindings, e.g.
    // #[cpp(func = "void hello()")]
    // fn hello();
}
EOF

cat > "$TARGET/rust_hicc/tests/smoke.rs" <<EOF
#[test]
fn smoke() {
    // TODO: exercise the FFI binding and assert behavior.
}
EOF

# --- README.md skeleton ---
cat > "$TARGET/README.md" <<EOF
# ${FEATURE} — ${NAME}

## C++ API

TODO: brief description of the C++ feature being demonstrated.

## 关键 AST 字段

| 字段 | 提取方式 |
|------|----------|
| TODO | from \`ast/ast.json\` |

## 手工映射步骤

1. TODO

## hicc 限制 / 降级

TODO: note any limitations or fallbacks.

## 自动化评估

TODO: rate as 高/中/低 with reasoning.

## 构建 / 验证

\`\`\`bash
# C++ side
cd cpp && bash standalone.sh    # or: make    # or: cd build && cmake .. && make
# Rust side
cd rust_hicc && cargo test
# End-to-end
../../scripts/verify-one.sh ${NUM}
\`\`\`
EOF

echo "scaffolded: $TARGET"
echo "next: edit $TARGET/cpp/${NAME}.h and $TARGET/cpp/${NAME}.cpp, then fill the rust_hicc bindings"
