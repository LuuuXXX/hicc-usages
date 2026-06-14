# 046_constexpr_basic — constexpr_basic

## C++ API

`constexpr int sq(int)` / `constexpr int cube(int)` / `constexpr int kMagic = 7;` 编译期求值。

## hicc 支持方式

✅ **直接：constexpr 对 FFI 完全透明**

`constexpr` 是 C++ 编译期 hint，不改变函数签名。hicc 只看到普通函数签名，绑定方式与 002-005 完全一致。

## 关键 AST 字段

| 字段 | 提取方式 |
|------|----------|
| `FunctionDecl.isConstexpr` | 标识 constexpr（仅信息性，不影响 FFI） |
| `VarDecl.isConstexpr` | 标识 constexpr 变量 |
| 函数 / 变量签名 | 与普通函数相同 |

## 手工映射步骤

1. AST 中识别 constexpr 函数 / 变量（仅作为标记，不影响处理流程）。
2. 函数 → `import_lib!` 自由函数绑定（与 002-005 相同）。
3. constexpr 变量 → 提供顶级 wrapper 返回其值（如 `int magic_value()`）。

## hicc 限制 / 降级

- ✅ 完全无限制。constexpr 在 FFI 端不可见也无影响。
- ⚠️ 跨语言无法共享"编译期求值"语义（Rust 端是普通运行时调用）。

## 自动化评估

**高**：与普通函数模板完全相同，可重用 002-005 流水线。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh    # or: make    # or: cmake + make
cd rust_hicc && cargo test
../../scripts/verify-one.sh 046_constexpr_basic
```
