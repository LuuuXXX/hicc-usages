# 005_variadic_functions

C 风格 variadic 函数 `int sum(int n, ...)` 不能跨 FFI 直接调用（FFI 没有可变 arity 概念）。**降级方案**：在 C++ 端写固定 arity 包装（`sum2`、`sum3`），Rust 端绑定包装函数。

## C++ API

```cpp
int sum(int n, ...);     // C++ 端可继续使用
int sum2(int a, int b);  // 固定 arity 包装，Rust 绑定此函数
int sum3(int a, int b, int c);
```

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| isVariadic | `.isVariadic`（true 时跳过该函数）| 自动化过滤 |
| 函数签名 | 同普通函数 | 用于包装函数绑定 |

## 手工映射步骤

1. 从 AST 中检测到 `.isVariadic == true` 的 `FunctionDecl`，**不**为其生成 Rust 绑定
2. 在 C++ 端手工写一组固定 arity 包装（`sum2`、`sum3`），覆盖常用情况
3. Rust 端按普通函数路径绑定包装函数

## hicc 限制 / 降级

- `int sum(int n, ...)` 本身**不可绑定**（FFI 类型不安全）
- 必须 C++ 端调整：写一组覆盖常用 arity 的命名包装函数

## 自动化评估

**中**。可以自动检测 `isVariadic` 跳过原函数，但**写多少个包装、写哪些 arity** 需要人工判断。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh
cd ../rust_hicc && cargo test
# 或 ../../scripts/verify-one.sh 005
```
