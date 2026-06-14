#!/usr/bin/env bash
# scaffold.sh - 为 NNN_<name> 创建 C++ 项目骨架
# 用法: scaffold.sh <NNN> <name>
# 例如: scaffold.sh 001 hello_world
set -euo pipefail

NNN="$1"
NAME="$2"
DIR="examples/${NNN}_${NAME}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if [[ -d "$DIR" ]]; then
    echo "[scaffold] 已存在: $DIR" >&2
    exit 1
fi

CPP_DIR="$DIR/cpp"
mkdir -p "$CPP_DIR/include/hicc_usages" "$CPP_DIR/src" "$CPP_DIR/test"

# 1) 头文件
cat > "$CPP_DIR/include/hicc_usages/${NAME}.h" <<EOF
#pragma once

#include <cstddef>
#include <iostream>

namespace hicc_usages::${NAME} {

// TODO: 在此声明 ${NAME} 特性的 C++ API
// 推荐模式：
//   void free_function();
//   class ClassX {
//   public:
//       static ClassX* create();      // factory
//       static void free(ClassX*);    // deleter
//       // 普通方法
//   };

}  // namespace hicc_usages::${NAME}
EOF

# 2) 源文件
cat > "$CPP_DIR/src/${NAME}.cpp" <<EOF
#include "hicc_usages/${NAME}.h"

namespace hicc_usages::${NAME} {

// TODO: 在此实现 ${NAME} 特性的 C++ API

}  // namespace hicc_usages::${NAME}
EOF

# 3) 测试程序
cat > "$CPP_DIR/test/main.cpp" <<EOF
#include "hicc_usages/${NAME}.h"
#include <cassert>

int main() {
    // TODO: 在此调用 ${NAME} 特性的 C++ API 并验证
    // 示例：assert(...);
    std::cout << "[${NAME}] C++ test OK" << std::endl;
    return 0;
}
EOF

# 4) Makefile
cat > "$CPP_DIR/Makefile" <<EOF
NAME := ${NAME}
CXX ?= clang++
CXXFLAGS := -std=c++17 -O2 -fPIC -Wall -Iinclude
SRCS := \$(wildcard src/*.cpp)
OBJS := \$(patsubst src/%.cpp,build/%.o,\$(SRCS))

all: build/lib\$(NAME).a

build/lib\$(NAME).a: \$(OBJS)
	@mkdir -p build
	\$(AR) rcs \$@ \$^

build/%.o: src/%.cpp
	@mkdir -p build
	\$(CXX) \$(CXXFLAGS) -c \$< -o \$@

test: all
	\$(CXX) \$(CXXFLAGS) test/main.cpp -Lbuild -l\$(NAME) -o build/test_\$(NAME) && ./build/test_\$(NAME)

clean:
	rm -rf build

.PHONY: all test clean
EOF

# 5) CMakeLists.txt
cat > "$CPP_DIR/CMakeLists.txt" <<EOF
cmake_minimum_required(VERSION 3.16)
project(hicc_usage_${NAME} CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB ${NAME}_SOURCES CONFIGURE_DEPENDS "src/*.cpp")
add_library(${NAME} STATIC \${${NAME}_SOURCES})
target_include_directories(${NAME} PUBLIC include)

enable_testing()
add_executable(test_${NAME} test/main.cpp)
target_link_libraries(test_${NAME} PRIVATE ${NAME})
add_test(NAME ${NAME}_test COMMAND test_${NAME})
EOF

# 6) 单特性 README
cat > "$DIR/README.md" <<EOF
# ${NNN}_${NAME}

C++ 特性：${NAME}

## 目录

- \`cpp/\` — 独立 C++ 项目（Makefile + CMakeLists.txt 双构建系统）
- \`ast/\` — 工具产出（\`tools/ast-extract/extract.sh\` 生成，gitignore）
- \`rust/\` — 工具产出（\`tools/rust-gen/rust_gen.py\` 生成）

## 验证

\`\`\`bash
# C++ 测试
(cd cpp && make test)

# AST 提取 + Rust 生成
../../tools/ast-extract/extract.sh .

# Rust 测试
(cd rust && cargo test)
\`\`\`
EOF

echo "[scaffold] $DIR 骨架已创建"
echo "  下一步：编辑 $CPP_DIR/include/hicc_usages/${NAME}.h 和 src/${NAME}.cpp"
