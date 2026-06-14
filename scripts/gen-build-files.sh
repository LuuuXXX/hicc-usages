#!/usr/bin/env bash
# gen-build-files.sh - 从 tools/scaffold/ 模板重新生成所有示例的 Makefile 和 CMakeLists.txt
#
# 用法:
#   bash scripts/gen-build-files.sh           # 生成所有 48 个示例
#   bash scripts/gen-build-files.sh 024 025   # 只生成指定示例
#
# 模板更新后跑这个脚本即可批量同步所有示例的构建文件。
set -euo pipefail

cd "$(dirname "$0")/.."

TEMPLATE_DIR="tools/scaffold"
EXAMPLES_DIR="examples"

if [ ! -f "$TEMPLATE_DIR/Makefile.tmpl" ]; then
    echo "ERROR: 模板 $TEMPLATE_DIR/Makefile.tmpl 不存在" >&2
    exit 1
fi

# 收集要处理的示例目录
dirs=()
if [ $# -gt 0 ]; then
    for arg in "$@"; do
        # 支持 NNN 数字 或 完整路径
        if [[ "$arg" =~ ^[0-9]{3}$ ]]; then
            matches=($EXAMPLES_DIR/${arg}_*)
            if [ ${#matches[@]} -eq 1 ]; then
                dirs+=("${matches[0]}")
            else
                echo "ERROR: 找不到示例 $arg" >&2
                exit 1
            fi
        elif [ -d "$arg" ]; then
            dirs+=("$arg")
        else
            echo "ERROR: 无效参数 $arg" >&2
            exit 1
        fi
    done
else
    for d in "$EXAMPLES_DIR"/*/; do
        dirs+=("$d")
    done
fi

count=0
for d in "${dirs[@]}"; do
    name=$(basename "$d")
    # 提取 NNN_xxx 中的 xxx 部分（去掉前缀编号和下划线）
    feature_name="${name#???_}"

    if [ ! -d "$d/cpp" ]; then
        continue
    fi

    # 生成 Makefile
    sed "s/__NAME__/$feature_name/g" "$TEMPLATE_DIR/Makefile.tmpl" > "$d/cpp/Makefile"
    # 生成 CMakeLists.txt
    sed "s/__NAME__/$feature_name/g" "$TEMPLATE_DIR/CMakeLists.txt.tmpl" > "$d/cpp/CMakeLists.txt"

    count=$((count + 1))
done

echo "已生成 $count 个示例的 Makefile 和 CMakeLists.txt"
