#!/usr/bin/env bash
# rust-gen-all.sh - 对所有（或指定）示例做 AST 提取 + Rust 生成
# 用法: rust-gen-all.sh [NNN...]
set -uo pipefail
source "$(dirname "$0")/_common.sh"
setup_root
iter_examples "$@"

run_rust_gen() {
    local d=$1
    [[ ! -d "$d/cpp" ]] && return 1
    tools/ast-extract/extract.sh "$d" 2>&1 | tail -2 && \
        python3 tools/rust-gen/rust_gen.py \
            --symbols "$d/ast/symbols.json" \
            --out "$d/rust" \
            --special tools/rust-gen/special.yaml
}

for_each_example run_rust_gen "AST 提取 + Rust 生成"
