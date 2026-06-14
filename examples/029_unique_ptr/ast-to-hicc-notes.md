# 029_unique_ptr: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateSpecializationDecl unique_ptr<Resource, default_delete<Resource>>`
- 工厂 `Resource* make_resource(int, const std::string&)` —— 返回 `unique_ptr<Resource>` 在 C++ 里
- 方法：`id()`、`name()`

## hicc 模式：✅ **默认 deleter → 直接当作对象**

`std::unique_ptr<T>` 默认 deleter 时，hicc 把工厂返回值映射为**类对象本身**（不是 `hicc::unique_ptr<T>`）。
**Rust 端 Resource 的 Drop = C++ 端 unique_ptr 的释放**。

```rust
#[cpp(func = "std::unique_ptr<Resource> hicc::make_unique<Resource, int, const std::string&>(int&&, const std::string&)")]
pub fn make_resource(id: i32, name: &hicc_std::string) -> Resource;
```

## 已知 hicc bug：避开 `make_unique_arg`

如果 `std::unique_ptr<T>` 作为**函数参数**（消费语义），hicc 内部生成 `make_unique_arg` 未定义符号。
**绕开**：去掉 consume_resource 绑定，用 Rust 端 Drop 演示消费语义即可。

## 自动化可行性：高

唯一注意点：识别 unique_ptr 的 deleter 模板参数；默认 deleter → 直接绑类对象，自定义 deleter → 见 031。
