#!/usr/bin/env bash
# 一键构建 + 运行：g++ 直接编译，产出可执行文件并跑
set -euo pipefail
cd "$(dirname "$0")"
g++ -std=c++17 -O2 -Wall -Wextra -I. \
    hello_world.cpp main.cpp \
    -o /tmp/hello_world_standalone
/tmp/hello_world_standalone
