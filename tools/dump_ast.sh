#!/usr/bin/env bash
# dump_ast.sh <cpp_dir> [<file_stem>]
# 输出 <cpp_dir>/../ast/{stem}.i (宏展开) + <cpp_dir>/../ast/ast.json (JSON AST)
# 用法: bash tools/dump_ast.sh examples/006_class_basic/cpp class_basic
set -euo pipefail

CPP_DIR="${1:?missing cpp dir}"
STEM="${2:-$(ls "$CPP_DIR"/*.cpp | grep -v main.cpp | head -1 | xargs -n1 basename | sed 's/.cpp$//')}"

AST_DIR="$(cd "$CPP_DIR/.." && pwd)/ast"
mkdir -p "$AST_DIR"

# 1. 宏展开后的源（保留 #include 但展开宏）
clang++ -std=c++17 -E -dD -P -I"$CPP_DIR" \
    "$CPP_DIR/$STEM.cpp" \
    -o "$AST_DIR/$STEM.i"

# 2. JSON AST（仅语法解析，不实际编译；ast-dump 写到 stdout）
clang++ -std=c++17 -Xclang -ast-dump=json -fsyntax-only -I"$CPP_DIR" \
    "$CPP_DIR/$STEM.cpp" \
    > "$AST_DIR/ast.json"

# 3. 过滤后的「只含用户自己头文件节点」的精简 AST（用于人工提取关键信息）
HEADER_BASENAME="$(basename "$(ls "$CPP_DIR"/*.h | head -1)")"
CPP_BASENAME="$(basename "$(ls "$CPP_DIR"/*.cpp | grep -v main.cpp | head -1)")"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
python3 "$SCRIPT_DIR/filter_ast.py" "$AST_DIR/ast.json" "$HEADER_BASENAME" "$CPP_BASENAME" "$AST_DIR/user-ast.json"

echo "[dump_ast] wrote: $AST_DIR/$STEM.i ($(wc -c < "$AST_DIR/$STEM.i") bytes)"
echo "[dump_ast] wrote: $AST_DIR/ast.json ($(wc -c < "$AST_DIR/ast.json") bytes — full)"
echo "[dump_ast] wrote: $AST_DIR/user-ast.json ($(wc -c < "$AST_DIR/user-ast.json") bytes — filtered to $HEADER_BASENAME)"
echo "[dump_ast] stem = $STEM"
