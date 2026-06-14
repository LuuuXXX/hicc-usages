# 012_class_volatile: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Sensor`
- `read() volatile const`, `write(int) volatile` — volatile 修饰的方法
- `safe_read() const`, `safe_write(int)` — 非 volatile 桥接
- `id() const`, `counter() const`

## hicc 模式

Rust 无 volatile 语义，对 `volatile` 方法的 FFI 调用 ABI 不匹配，会触发 UB。**要求 C++ 端提供非 volatile 桥接函数**（本例已内置），Rust 只绑定桥接方法。

```rust
#[cpp(method = "int safe_read() const")]
pub fn safe_read(&self) -> i32;
```

## 自动化可行性：中

`CXXMethodDecl` 的 `isVolatile()` 可读，但**必须有人为 volatile 方法手写桥接**（C++ 改动），无法纯靠 AST 自动生成 Rust 绑定。
