#!/usr/bin/env bash
# rust-test-all.sh - 对所有（或指定）示例的 Rust crate 跑 cargo test
# 用法: rust-test-all.sh [NNN...]
set -uo pipefail
source "$(dirname "$0")/_common.sh"
setup_root
iter_examples "$@"

run_rust_test() {
    local d=$1
    [[ ! -d "$d/rust" ]] && return 1
    (cd "$d/rust" && cargo test --quiet 2>&1 | tail -10)
}

for_each_example run_rust_test "cargo test"
