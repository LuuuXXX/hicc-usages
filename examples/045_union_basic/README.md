# 045_union_basic — union_basic

## C++ API

命名联合体（`union { int as_int; float as_float; };`）配合 `Tag` 枚举做判别，封装在 `ValueBox` 类中。

## hicc 支持方式

⚠️ **C++ 端调整：包装类**

Union 类型不能在 `#[cpp(...)]` 签名中命名。包装类 `ValueBox` 提供 typed setter/getter，Rust 只看到 box 与 tag int。

## 关键 AST 字段

| 字段 | 提取方式 |
|------|----------|
| `CXXRecordDecl` (是 `union`) | 识别 union 定义 |
| `FieldDecl` in union | 联合体的可选成员（`as_int` / `as_float`） |
| `CXXRecordDecl` (包装类) | 借用 006-009 类绑定模板 |
| `EnumDecl` (`Tag`) | 提供 tag int 边界值 |

## 手工映射步骤

1. AST 检测 union 字段及其类型。
2. C++ 端定义包装类 `ValueBox`：内部 union + tag，提供每个字段的 typed setter/getter。
3. Rust 端绑定 `ValueBox` 为 hicc class（factory + free pair）。
4. tag int 通过 `tag()` 方法暴露，让 Rust 知道当前 active 字段。

## hicc 限制 / 降级

- ❌ Union 本身不能命名。
- ✅ 包装类方式，Rust 端可类型安全访问每个分支。
- ⚠️ 若 union 含非平凡类型（如 `std::string`），需手工管理 placement new / destructor。

## 自动化评估

**中**：包装类模板可生成，但 setter/getter 名与 tag 判别逻辑需要约定。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh    # or: make    # or: cmake + make
cd rust_hicc && cargo test
../../scripts/verify-one.sh 045_union_basic
```
