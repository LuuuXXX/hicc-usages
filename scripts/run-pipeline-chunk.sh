#!/usr/bin/env bash
# run-pipeline-chunk.sh - 跑某个分片的完整流水线（cpp build + rust gen + rust test）
#
# 用于 CI 矩阵并行：把 48 个示例分成 N 个分片，每个 job 跑一个分片
#
# 用法:
#   bash scripts/run-pipeline-chunk.sh CHUNK_INDEX TOTAL_CHUNKS
#   例：bash scripts/run-pipeline-chunk.sh 0 4   # 跑第 0 个分片（共 4 个）
set -uo pipefail
source "$(dirname "$0")/_common.sh"

CHUNK_INDEX=${1:?用法: run-pipeline-chunk.sh CHUNK_INDEX TOTAL_CHUNKS}
TOTAL_CHUNKS=${2:?用法: run-pipeline-chunk.sh CHUNK_INDEX TOTAL_CHUNKS}

setup_root

# 收集所有示例（按目录名排序保证分片稳定）
ALL_DIRS=()
for d in examples/*/; do
    name=$(basename "$d")
    nnn=${name%%_*}
    [[ ! "$nnn" =~ ^[0-9]+$ ]] && continue
    ALL_DIRS+=("$d")
done
TOTAL=${#ALL_DIRS[@]}

# 计算该分片的起止
PER_CHUNK=$(( (TOTAL + TOTAL_CHUNKS - 1) / TOTAL_CHUNKS ))
START=$((CHUNK_INDEX * PER_CHUNK))
END=$((START + PER_CHUNK))
[[ $END -gt $TOTAL ]] && END=$TOTAL

echo "分片 $CHUNK_INDEX/$TOTAL_CHUNKS: 共 $TOTAL 个示例，本分片处理索引 $START..$((END-1)) (共 $((END-START)) 个)"

PASS=0
FAIL=0
FAILED_LIST=()

for ((i=START; i<END; i++)); do
    d="${ALL_DIRS[$i]}"
    name=$(basename "$d")
    echo "=== [$name] pipeline ==="

    # 1. C++ 构建+测试
    if ! (cd "$d/cpp" && make clean >/dev/null 2>&1 && make test) > /tmp/cpp_$$.log 2>&1; then
        echo "  ❌ C++ FAIL"
        cat /tmp/cpp_$$.log | tail -3
        FAIL=$((FAIL+1))
        FAILED_LIST+=("$name [cpp]")
        rm -f /tmp/cpp_$$.log
        continue
    fi
    rm -f /tmp/cpp_$$.log

    # 2. AST 提取 + Rust 生成
    if ! tools/ast-extract/extract.sh "$d" > /tmp/gen_$$.log 2>&1 || \
       ! python3 tools/rust-gen/rust_gen.py \
           --symbols "$d/ast/symbols.json" \
           --out "$d/rust" \
           --special tools/rust-gen/special.yaml >> /tmp/gen_$$.log 2>&1; then
        echo "  ❌ Rust gen FAIL"
        cat /tmp/gen_$$.log | tail -3
        FAIL=$((FAIL+1))
        FAILED_LIST+=("$name [gen]")
        rm -f /tmp/gen_$$.log
        continue
    fi
    rm -f /tmp/gen_$$.log

    # 3. cargo test
    if ! (cd "$d/rust" && cargo test --quiet) > /tmp/test_$$.log 2>&1; then
        echo "  ❌ cargo test FAIL"
        cat /tmp/test_$$.log | tail -5
        FAIL=$((FAIL+1))
        FAILED_LIST+=("$name [test]")
        rm -f /tmp/test_$$.log
        continue
    fi
    rm -f /tmp/test_$$.log

    echo "  ✅ PASS"
    PASS=$((PASS+1))
done

echo
echo "=== 分片 $CHUNK_INDEX 汇总 ==="
echo "PASS: $PASS  FAIL: $FAIL  TOTAL: $((PASS+FAIL))"
[[ ${#FAILED_LIST[@]} -gt 0 ]] && echo "失败: ${FAILED_LIST[*]}"

[[ $FAIL -eq 0 ]]
