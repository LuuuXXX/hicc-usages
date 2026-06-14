# 019_operator_overload

💬 **注释式注入**：hicc 不能直接绑定 `operator+`，必须在 C++ 端写命名包装函数（`vec2_add`、`vec2_sub`、`vec2_eq`），Rust 端绑定包装。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 操作符重载 | `CXXMethodDecl` 且 name 形如 `operator+` | 触发"生成命名包装"流程 |

## 自动化评估
**中**。可自动检测 `operator+` 并生成 `vec2_add` 包装 + 绑定，但需要命名约定（哪类 operator → 哪个 Rust 名字）。
