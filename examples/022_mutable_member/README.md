# 022_mutable_member

✅ **直接**：`mutable` 字段在 const 方法中可变，对 FFI 透明。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| mutable 字段 | `FieldDecl.isMutable` | 信息性 |
| const 方法 | `CXXMethodDecl.isConst` | Rust `&self` |

## 自动化评估
**高**。mutable 不影响绑定决策。
