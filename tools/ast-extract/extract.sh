#!/usr/bin/env bash
# extract.sh - 从 C++ 项目提取宏展开后的 AST 并提炼符号表
#
# 用法: extract.sh <example_dir>
#   example_dir 是 examples/NNN_<name>/ 目录
#
# 产出:
#   <example_dir>/ast/preprocessed.cpp   宏展开后的纯 C++ 源
#   <example_dir>/ast/ast.json           clang AST 的 JSON dump
#   <example_dir>/ast/symbols.json       提炼后的符号表（供 rust_gen 用）
set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "用法: $0 <example_dir>" >&2
    exit 1
fi

EXAMPLE_DIR="$(cd "$1" && pwd)"
NAME="$(basename "$EXAMPLE_DIR" | sed -E 's/^[0-9]+_//')"
CPP_DIR="$EXAMPLE_DIR/cpp"
AST_DIR="$EXAMPLE_DIR/ast"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

if [[ ! -d "$CPP_DIR/include" ]]; then
    echo "[extract] 错误: $CPP_DIR/include 不存在" >&2
    exit 1
fi

mkdir -p "$AST_DIR"

# 1) 宏展开：把所有 .cpp 合并预处理
SRCS=$(find "$CPP_DIR/src" -name '*.cpp' 2>/dev/null | sort)
if [[ -z "$SRCS" ]]; then
    echo "[extract] 错误: $CPP_DIR/src 下没有 .cpp 文件" >&2
    exit 1
fi

echo "[extract] $NAME: 预处理..."
clang++ -std=c++17 -E -I "$CPP_DIR/include" -P $SRCS > "$AST_DIR/preprocessed.cpp"

# 2) AST dump（完整 JSON，包含所有展开的系统头）
# 注意：clang -ast-dump 即使成功也可能返回非零退出码（警告被当 error），所以不检查退出码
echo "[extract] $NAME: AST dump..."
clang++ -std=c++17 -Xclang -ast-dump=json -fsyntax-only \
    -I "$CPP_DIR/include" \
    "$AST_DIR/preprocessed.cpp" > "$AST_DIR/ast.json" 2>/dev/null || true

if [[ ! -s "$AST_DIR/ast.json" ]]; then
    echo "[extract] $NAME: 错误 - ast.json 为空或未生成" >&2
    exit 1
fi

# 3) 提炼符号
echo "[extract] $NAME: 提炼符号..."
python3 "$SCRIPT_DIR/symfilter.py" \
    --ast "$AST_DIR/ast.json" \
    --name "$NAME" \
    --out "$AST_DIR/symbols.json"

echo "[extract] $NAME: 完成 → $AST_DIR/symbols.json"
