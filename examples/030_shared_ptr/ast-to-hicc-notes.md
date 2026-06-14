# 030_shared_ptr: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateSpecializationDecl shared_ptr<Counter>`
- 工厂 `shared_ptr<Counter> make_counter(int)`
- 辅助函数 `clone_counter(const shared_ptr<Counter>&)`、`use_count(const shared_ptr<Counter>&)`

## hicc 模式：✅ **hicc::shared_ptr<T>**

`std::shared_ptr<T>` 在 hicc 中映射到 Rust 端 `hicc::shared_ptr<T>` 类型（不是 T 本身，与 unique_ptr 默认 deleter 不同）。
引用计数、clone、weak_ptr 都通过 hicc::shared_ptr 自带 API。

```rust
#[cpp(func = "std::shared_ptr<Counter> shared_ptr_ns::make_counter(int)")]
pub fn make_counter(start: i32) -> hicc::shared_ptr<Counter>;

#[cpp(func = "long shared_ptr_ns::use_count(const std::shared_ptr<Counter>&)")]
pub fn use_count(p: &hicc::shared_ptr<Counter>) -> i64;
```

通过 `&hicc::shared_ptr<T>` 接收引用，`.get()` 取 ClassRef 访问方法。

## 自动化可行性：高

`shared_ptr<T>` 参数/返回 → `hicc::shared_ptr<T>` Rust 类型。仅需识别 ClassTemplateSpecializationDecl 中 primary template = std::shared_ptr。
