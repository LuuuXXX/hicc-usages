#!/usr/bin/env bash
# verify-one.sh - 单特性端到端验证
# 用法: verify-one.sh <NNN>
set -euo pipefail
NNN="$1"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

DIR=$(ls -d examples/${NNN}_* 2>/dev/null | head -1)
if [[ -z "$DIR" ]]; then
    echo "[verify-one] 未找到 examples/${NNN}_*" >&2
    exit 1
fi

name=$(basename "$DIR")
echo "=== verify-one: $name ==="

# 1) C++ 测试
echo "[1/4] C++ 构建+测试"
(cd "$DIR/cpp" && make test)

# 2) AST 提取
echo "[2/4] AST 提取"
tools/ast-extract/extract.sh "$DIR"

# 3) Rust 生成
echo "[3/4] Rust 生成"
python3 tools/rust-gen/rust_gen.py \
    --symbols "$DIR/ast/symbols.json" \
    --out "$DIR/rust" \
    --special tools/rust-gen/special.yaml

# 4) Rust 测试
echo "[4/4] cargo test"
(cd "$DIR/rust" && cargo test)

echo
echo "✅ verify-one: $name 全部通过"
