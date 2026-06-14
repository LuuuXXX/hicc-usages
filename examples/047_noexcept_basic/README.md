# 047_noexcept_basic — noexcept_basic

## C++ API

`struct SafeAdder { int add(int) const noexcept; ... };` 类含 `noexcept` 成员方法。

## hicc 支持方式

⚠️ **限制标注：C++ 端去掉 noexcept**

hicc 的 `#[cpp(method = "...")]` 解析器**不接受** `noexcept` 关键字。这是本项目**唯一**对 C++ 源做改动的示例：把 `int add(int) const noexcept;` 改为 `int add(int) const;`，签名才能在 hicc 中表达。语义（不抛异常保证）仍由 C++ 端在运行时维护。

## 关键 AST 字段

| 字段 | 提取方式 |
|------|----------|
| `FunctionDecl.exceptionSpecType` | `EST_BasicNoexcept` → 标识需修改 |
| `CXXRecordDecl` | 类签名照常绑定 |
| `ParmVarDecl` / `FunctionDecl.returnType` | 同 006-009 |

## 手工映射步骤

1. AST 检测 `exceptionSpecType == EST_BasicNoexcept` 的方法。
2. **修改 C++ 头文件**：从签名中去掉 `noexcept`（不影响实现，运行时仍 no-throw）。
3. Rust 端 `import_class!` 绑定类与方法，与 006-009 相同。

## hicc 限制 / 降级

- ❌ `noexcept` 关键字不能出现在 `#[cpp(...)]` 签名中。
- ⚠️ 仅 047 一例修改 C++ 源（其余 47 例均保持 C++ 不动）。
- ✅ `throw()` / `noexcept(false)` / `noexcept(expr)` 也需同样处理。

## 自动化评估

**高**：检测与剥离 noexcept 完全机械化（AST 字段已直接表达）。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh    # or: make    # or: cmake + make
cd rust_hicc && cargo test
../../scripts/verify-one.sh 047_noexcept_basic
```
