# 004_inline_functions

`inline` 函数对 hicc 完全透明，与普通自由函数走同一套路径。

## C++ API

```cpp
inline int square(int x) { return x * x; }
inline int cube(int x)   { return x * x * x; }
```

注意：`inline` 函数**定义在头文件中**（典型 C++ 模式），编译器发射弱符号，多个 TU 引入同一头不会冲突。

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| isInline（可选） | `.isInline`（部分版本是 `storageClass` 或 `inlineAttr`） | 仅信息性 |
| 函数签名 | `.name` + `.returnType.qualType` + `.parameters[].type.qualType` | 同普通函数 |

## 手工映射步骤

1. 把 `inline` 函数当作普通自由函数处理
2. 在 `hicc::cpp! { #include "..." }` 中引入头文件
3. `#[cpp(func = "...")]` 用**非 inline** 形式（hicc-build 类型匹配不需要 `inline` 关键字）

## hicc 限制 / 降级

无。

## 自动化评估

**高**。等同于普通函数。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh
cd ../rust_hicc && cargo test
# 或 ../../scripts/verify-one.sh 004
```
