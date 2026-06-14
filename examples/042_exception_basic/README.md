# 042_exception_basic — exception_basic

## C++ API

`throw` + `catch`，包括 `std::runtime_error` / `std::out_of_range`。C++ 异常类型本身不跨 FFI；hicc 用 `hicc::Exception<T>` 把 `(value | what())` 编码成一个结构体。

## hicc 支持方式

✅ **直接：`hicc::Exception<T>` 返回类型**

Rust 函数签名声明返回 `hicc::Exception<T>`；调用方 `.ok()` 得到 `Result<T, ExceptionInfo>`，错误侧携带 `what()` 字符串（≤63 字节）。

## 关键 AST 字段

| 字段 | 提取方式 |
|------|----------|
| `FunctionDecl.name` | 函数名 |
| `FunctionDecl.returnType` | 普通类型 `int`（与正常函数无异） |
| `CXXThrowExpr` 是否存在 | 决定 Rust 是否应使用 `Exception<T>` 返回 |
| `CallExpr` → 异常构造 | 仅记录可能的 `what()` 模板（runtime_error / out_of_range / logic_error 等） |

## 手工映射步骤

1. AST 检测函数体含 `CXXThrowExpr`。
2. Rust 端返回类型从 `T` 改为 `hicc::Exception<T>`。
3. 冒烟测试同时覆盖成功路径与异常路径（`.ok().unwrap_err().what()`）。

## hicc 限制 / 降级

- ❌ 异常的 C++ 类型本身不跨 FFI（Rust 端只看到字符串 `what()`）。
- ❌ 异常字符串最长 63 字节（`ExceptionInfo` 固定 64 字节 buffer，最后一字节为 `\0`）。
- ✅ 多种 C++ 异常类型都能映射到同一个 `ExceptionInfo` channel。

## 自动化评估

**高**：`CXXThrowExpr` 检测直接；返回类型改写为 `Exception<T>` 完全机械；`what()` 子串匹配由测试覆盖。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh    # or: make    # or: cmake + make
cd rust_hicc && cargo test
../../scripts/verify-one.sh 042_exception_basic
```
