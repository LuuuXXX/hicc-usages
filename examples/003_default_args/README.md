# 003_default_args

C++ 默认参数：`int add(int a, int b = 10)`。Rust 不支持默认参数，所以 Rust 端暴露完整 arity 签名，调用方需要时显式传默认值。

## C++ API

```cpp
int add(int a, int b = 10);
```

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 形参列表 | `.parameters[]` | 列出所有形参（包括有默认值的） |
| 默认值（可选） | `.parameters[].defaultArg` 或 `inner`（如果存在） | 仅信息性，Rust 端不需要 |

注意：clang AST 中默认值的存在通常需要看到完整定义（仅在函数 `hasDefaultArg` 为 true 时才有 `defaultArg` 子节点）。**对 Rust 端**只需提取 `parameters[]` 的完整 arity 即可。

## 手工映射步骤

1. 从 AST 中读 `parameters[]`，得到形参总数（这里 = 2）
2. Rust 端写完整 arity：`fn add(a: i32, b: i32) -> i32`
3. `#[cpp(func = "int add(int, int)")]` 不写默认值（hicc 也不接受 C++ 默认值语法）

## hicc 限制 / 降级

无 — 默认值的存在对 FFI 没有意义，跨 FFI 调用必须传所有参数。

## 自动化评估

**高**。Rust 端只需"忽略默认值，用完整 arity"。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh
cd ../rust_hicc && cargo test
# 或 ../../scripts/verify-one.sh 003
```
