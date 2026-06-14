# 011_class_const

const 成员方法：`Vec2::magnitude() const` / `Vec2::dot(const Vec2&) const`。hicc 把 `&self` 映射到 `const` 方法。

## C++ API

```cpp
class Vec2 {
    Vec2(double, double);
    double x() const; double y() const;
    double magnitude() const;
    double dot(const Vec2&) const;
};
```

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| const 标记 | `CXXMethodDecl.isConst == true` | Rust 用 `&self` |
| 参数 `const Class&` | `ParmVarDecl.type.qualType` 含 `const Class &` | Rust 用 `&Class` |

## 手工映射步骤

1. 对每个 `CXXMethodDecl`，根据 `isConst` 选择 `&self` / `&mut self`
2. `#[cpp(method = "ret f(args) const")]` 中的 `const` 关键字必须保留
3. 参数 `const Vec2&` 映射到 Rust `&Vec2`

## hicc 限制 / 降级

无 — const 方法是 hicc 直接支持。

## 自动化评估

**高**。`isConst` 字段直接决定 Rust 端 receiver 类型。

## 构建 / 验证

```bash
../../scripts/verify-one.sh 011
```
