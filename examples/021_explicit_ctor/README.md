# 021_explicit_ctor

✅ **直接**：`explicit` 构造对 FFI 不可见——还是通过 factory 函数暴露。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 是否 explicit | `CXXConstructorDecl.isExplicit` | 信息性 |

## 自动化评估
**高**。explicit 不改变绑定路径。
