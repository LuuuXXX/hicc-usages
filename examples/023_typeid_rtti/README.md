# 023_typeid_rtti

✅ **直接（命名包装）**：`typeid().name()` 不能跨 FFI，但 `const char*` 可以。命名访问器 `type_name_of` / `static_type_name_*`。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 多态类 | `CXXRecordDecl.isPolymorphic` | 提示需要 RTTI 访问器 |

## 自动化评估
**中**。生成 `type_name_of` 包装是机械的，但需要识别"哪些类需要 RTTI 暴露"。
