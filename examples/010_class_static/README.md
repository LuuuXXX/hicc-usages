# 010_class_static

静态成员 + 静态方法：`Registry::live_count()` / `Registry::next_id()`。hicc 把静态方法当作自由函数绑定，用全限定名 `Class::method`。

## C++ API

```cpp
class Registry {
    Registry(); ~Registry();
    static int live_count();
    static int next_id();
private:
    static int live_count_;
    static int next_id_;
};
```

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 静态方法 | `CXXMethodDecl.isStatic == true` | 在 `import_lib!` 中作为自由函数绑定 |
| 全限定名 | `Class::method` | `#[cpp(func = "...")]` 中保留全限定 |

## 手工映射步骤

1. 过滤 `CXXMethodDecl` 中 `isStatic == true` 的方法
2. 在 `import_lib!` 中用 `#[cpp(func = "int Registry::live_count()")]` 绑定
3. Rust 端无需 `import_class!`（如果只有静态方法，不需要实例化类型）

## hicc 限制 / 降级

无 — 静态方法是 hicc 的天然支持场景。

## 自动化评估

**高**。静态方法 → 自由函数的映射是机械的。

## 构建 / 验证

```bash
../../scripts/verify-one.sh 010
```
