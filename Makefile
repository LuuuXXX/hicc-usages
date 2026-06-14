## hicc-usages 顶层 Makefile
##
## 用法：
##   make              # 全流程：cpp-build → rust-gen → rust-test
##   make all          # 同上
##   make cpp          # 仅编译测试所有 C++
##   make rust         # 仅 AST 提取 + Rust 生成 + Rust 测试
##   make cpp-build    # 仅 C++ 编译
##   make rust-gen     # 仅 AST 提取 + Rust 生成
##   make rust-test    # 仅 Rust 冒烟测试
##   make gen-build    # 从模板重新生成所有示例的 Makefile/CMakeLists.txt
##   make clean        # 清理所有 build/target
##   make verify N=001 # 单特性端到端
##   make new N=099 NAME=my_feature  # 创建新特性骨架

ROOT := $(shell pwd)

.PHONY: all cpp cpp-build rust-gen rust-test gen-build verify new clean help

all: cpp rust

cpp: cpp-build
	@echo "✓ C++ 构建与测试完成"

rust: rust-gen rust-test
	@echo "✓ Rust 生成与测试完成"

cpp-build:
	@bash scripts/cpp-build-all.sh

rust-gen:
	@bash scripts/rust-gen-all.sh

rust-test: rust-gen
	@bash scripts/rust-test-all.sh

gen-build:
	@bash scripts/gen-build-files.sh

verify:
	@if [ -z "$(N)" ]; then echo "用法: make verify N=001"; exit 1; fi
	@bash scripts/verify-one.sh $(N)

new:
	@if [ -z "$(N)" ] || [ -z "$(NAME)" ]; then echo "用法: make new N=099 NAME=my_feature"; exit 1; fi
	@bash scripts/scaffold.sh $(N) $(NAME)

clean:
	@echo "清理 examples/*/cpp/build 和 examples/*/rust/target..."
	@find examples -type d -name build -path "*/cpp/*" -exec rm -rf {} + 2>/dev/null || true
	@find examples -type d -name target -path "*/rust/*" -exec rm -rf {} + 2>/dev/null || true
	@find examples -type d -name ast -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	@echo "✓ 清理完成"

help:
	@echo "hicc-usages Makefile"
	@echo ""
	@echo "目标："
	@echo "  all (默认)  全流程：C++ 编译 → Rust 生成 → Rust 测试"
	@echo "  cpp         仅编译并测试所有 C++ 项目"
	@echo "  rust        AST 提取 + Rust 生成 + 测试"
	@echo "  cpp-build   仅 C++ 编译+测试"
	@echo "  rust-gen    仅 AST 提取 + Rust 生成"
	@echo "  rust-test   仅 Rust 冒烟测试（依赖 rust-gen）"
	@echo "  gen-build   从模板重新生成所有示例的 Makefile/CMakeLists.txt"
	@echo "  verify N=001  单特性端到端验证"
	@echo "  new N=099 NAME=foo  创建新特性骨架"
	@echo "  clean       清理 build/target/ast 缓存"
	@echo "  help        显示此帮助"

