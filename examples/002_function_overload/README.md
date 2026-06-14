# 002_function_overload

C++ 函数重载：`add(int, int)` 和 `add(double, double)`。Rust 没有重载，所以 Rust 端用类型后缀（`add_i32` / `add_f64`）区分。

## C++ API

```cpp
int    add(int a, int b);
double add(double a, double b);
```

## 关键 AST 字段

两个 `FunctionDecl` 同名 `add`，靠 `parameters[].type.qualType` 区分：

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 形参类型集合 | `.parameters[].type.qualType` | 判断重载分支（int vs double） |
| 返回类型 | `.returnType.qualType` | 各分支 Rust 返回类型 |

## 手工映射步骤

1. 从 AST 中找出同名 `add` 的两条 `FunctionDecl`
2. 对每条按返回/参数类型生成 Rust 函数名：`add_<rust_type_suffix>`（int→i32、double→f64）
3. `#[cpp(func = "...")]` 中保留**原始 C++ 签名**（带类型），hicc-build 据此匹配重载分支

## hicc 限制 / 降级

无 — hicc 通过 `#[cpp(func=...)]` 字符串中的类型匹配 C++ 重载，Rust 端的名字不影响。

## 自动化评估

**高**。重载解析是机械的（同名函数 × 类型后缀），AST 字段明确。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh
cd ../rust_hicc && cargo test
# 或 ../../scripts/verify-one.sh 002
```
