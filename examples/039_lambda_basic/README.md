# 039_lambda_basic — lambda_basic

## C++ API

TODO: brief description of the C++ feature being demonstrated.

## 关键 AST 字段

| 字段 | 提取方式 |
|------|----------|
| TODO | from `ast/ast.json` |

## 手工映射步骤

1. TODO

## hicc 限制 / 降级

TODO: note any limitations or fallbacks.

## 自动化评估

TODO: rate as 高/中/低 with reasoning.

## 构建 / 验证

```bash
# C++ side
cd cpp && bash standalone.sh    # or: make    # or: cd build && cmake .. && make
# Rust side
cd rust_hicc && cargo test
# End-to-end
../../scripts/verify-one.sh 039
```
