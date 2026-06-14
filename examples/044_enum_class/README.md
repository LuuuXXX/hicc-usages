# 044_enum_class

C++ 特性：enum_class

## 目录

- `cpp/` — 独立 C++ 项目（Makefile + CMakeLists.txt 双构建系统）
- `ast/` — 工具产出（`tools/ast-extract/extract.sh` 生成，gitignore）
- `rust/` — 工具产出（`tools/rust-gen/rust_gen.py` 生成）

## 验证

```bash
# C++ 测试
(cd cpp && make test)

# AST 提取 + Rust 生成
../../tools/ast-extract/extract.sh .

# Rust 测试
(cd rust && cargo test)
```
