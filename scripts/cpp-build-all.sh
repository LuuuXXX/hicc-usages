#!/usr/bin/env bash
# cpp-build-all.sh - 编译并测试所有（或指定）C++ 示例
# 用法: cpp-build-all.sh [NNN...]
set -uo pipefail
source "$(dirname "$0")/_common.sh"
setup_root
iter_examples "$@"

run_cpp_build() {
    local d=$1
    [[ ! -d "$d/cpp" ]] && return 1
    (cd "$d/cpp" && make clean >/dev/null 2>&1; make test)
}

for_each_example run_cpp_build "C++ 构建+测试"
