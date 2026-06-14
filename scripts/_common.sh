#!/usr/bin/env bash
# _common.sh - 三个 *-all.sh 脚本共享的参数解析和遍历逻辑
#
# 用法（在其他脚本里 source）:
#   source "$(dirname "$0")/_common.sh"
#   setup_root                    # cd 到项目根目录
#   iter_examples "$@"            # 设置 DIRS 数组（参数为空则全部）
#   for_each_example run_callback "label"  # 遍历每个示例目录，调用回调

# 切换到项目根目录（脚本所在目录的上一级）
setup_root() {
    ROOT="$(cd "$(dirname "${BASH_SOURCE[1]}")/.." && pwd)"
    cd "$ROOT"
}

# 解析命令行参数，填充 DIRS 数组
# 支持 NNN 数字、完整路径；空参数则遍历所有 examples/*
iter_examples() {
    DIRS=()
    if [[ $# -gt 0 ]]; then
        for n in "$@"; do
            if [[ "$n" =~ ^[0-9]+$ ]]; then
                DIRS+=(examples/${n}_*)
            else
                DIRS+=("$n")
            fi
        done
    else
        DIRS=(examples/*)
    fi
}

# 遍历每个有效的示例目录，对每个调用回调函数（参数：目录路径、特性名）
# 回调返回 0 表示 PASS，非 0 表示 FAIL
# 自动维护 PASS / FAIL / FAILED_LIST，循环结束后打印汇总
for_each_example() {
    local callback=$1
    local label=$2
    PASS=0
    FAIL=0
    FAILED_LIST=()

    for d in "${DIRS[@]}"; do
        name=$(basename "$d")
        [[ "$name" == examples ]] && continue
        nnn=${name%%_*}
        [[ ! "$nnn" =~ ^[0-9]+$ ]] && continue

        echo "=== [$name] $label ==="
        if $callback "$d" "$name"; then
            echo "  ✅ PASS"
            PASS=$((PASS+1))
        else
            echo "  ❌ FAIL"
            FAIL=$((FAIL+1))
            FAILED_LIST+=("$name")
        fi
    done

    echo
    echo "=== $label 汇总 ==="
    echo "PASS: $PASS  FAIL: $FAIL  TOTAL: $((PASS+FAIL))"
    [[ ${#FAILED_LIST[@]} -gt 0 ]] && echo "失败: ${FAILED_LIST[*]}"

    [[ $FAIL -eq 0 ]]
}
