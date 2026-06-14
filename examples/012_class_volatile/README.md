# 012_class_volatile

volatile 成员方法：`inc() volatile` / `get() const volatile`。hicc 通过 `#[cpp(method = "ret f() volatile")]` 支持。

## C++ API

```cpp
class VCounter {
    void inc() volatile;
    int  get() const volatile;
    void reset() volatile;
};
VCounter* vcounter_new();
void      vcounter_free(VCounter*);
```

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| volatile 标记 | `CXXMethodDecl.isVolatile == true` | Rust 用 `&mut self`（volatile 暗示可观察副作用） |
| `const volatile` 组合 | `isConst && isVolatile` | Rust 用 `&self`（read-only 但 volatile） |

## 手工映射步骤

1. 检测 `isVolatile`，在 `#[cpp(method = "...")]` 中保留 `volatile` 关键字
2. `const volatile` → Rust `&self`；`volatile`（非 const）→ Rust `&mut self`

## hicc 限制 / 降级

无 — 直接支持。

## 自动化评估

**高**。`isVolatile` 字段决定方法签名后缀。

## 构建 / 验证

```bash
../../scripts/verify-one.sh 012
```
