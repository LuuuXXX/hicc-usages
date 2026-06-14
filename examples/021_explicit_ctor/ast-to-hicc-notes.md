# 021_explicit_ctor: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Distance`
- `CXXConstructorDecl Distance(double)` ← explicit
- `CXXConstructorDecl Distance(int, int)` ← explicit
- `CXXRecordDecl Wrapper`
- `CXXConstructorDecl Wrapper(const std::string&, int)` ← explicit

## hicc 模式

`explicit` 对 FFI 透明（FFI 不存在隐式转换）。每个 ctor 一个 `make_unique<T, Args...>(Args&&...)` 工厂。多 ctor 时，Rust 端用不同方法名（`from_meters` / `from_m_cm`）。

```rust
#[cpp(func = "std::unique_ptr<...Distance> hicc::make_unique<...Distance, double>(double&&)")]
pub fn distance_from_meters(m: f64) -> Distance;
```

## 自动化可行性：高

`CXXConstructorDecl.isExplicit` 可读，但 explicit 与否对 FFI 无影响，**不需要特殊处理**。多 ctor 时按参数类型给 Rust fn 命名后缀。
