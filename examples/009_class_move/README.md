# 009_class_move

移动语义：`int consume_value() &&`（rvalue 限定的方法）。Rust 端用 `fn method(self, ...)` 表达"消费 self"。

## C++ API

```cpp
class Resource {
    Resource(int);
    int  consume_value() &&;   // rvalue-qualified
    int  peek() const;
    bool is_valid() const;
};
```

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 方法 ref-qualifier | `CXXMethodDecl.refQualifier`（值为 `&&` / `&`）| 决定 Rust 端用 `self`（消费）还是 `&self` |
| 移动构造 | `CXXConstructorDecl.isMove` | 信息性 |

## 手工映射步骤

1. 从 AST 检测 `CXXMethodDecl` 的 `refQualifier == "&&"`
2. Rust 端用 `pub fn method(self, ...) -> ...`（按值接收，等价于 C++ `T&&`）
3. 普通方法仍走 `&self` / `&mut self` 路径

## hicc 限制 / 降级

`T&&` 引用限定方法只能映射到 Rust `self`（按值）。不能直接调用移动构造函数 — 只能通过 `&&` 限定的命名方法。

## 自动化评估

**中**。需要正确识别 ref-qualifier，且 hicc 的 self-by-value 语义需要测试验证。

## 构建 / 验证

```bash
../../scripts/verify-one.sh 009
```
