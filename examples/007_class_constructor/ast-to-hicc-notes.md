# 007_class_constructor: AST → hicc 手动映射记录

## user-ast.json 关键信息

- `CXXRecordDecl Widget`
- 3 个 `CXXConstructorDecl`：`Widget()`, `Widget(int)`, `Widget(string,int)`
- `~Widget()` 析构（自动）
- `name() const`, `value() const`

## hicc 模式选择

每个 ctor 一个 `make_unique<T, Args>(Args...)` 工厂 + Rust 关联函数包装。
Hicc 自带 Drop 调用 `~Widget()`，无需 `destroy`（析构是 public）。

## 关键代码段

```rust
hicc::import_lib! {
    #[cpp(func = "std::unique_ptr<T> hicc::make_unique<T>()")]
    pub fn widget_default() -> Widget;
    #[cpp(func = "std::unique_ptr<T> hicc::make_unique<T, int>(int)")]
    pub fn widget_from_int(v: i32) -> Widget;
    #[cpp(func = "std::unique_ptr<T> hicc::make_unique<T, const std::string&, int>(const std::string&, int)")]
    pub fn widget_from_named(name: &hicc_std::string, v: i32) -> Widget;
}
```

## 自动化可行性：高

- `CXXConstructorDecl` 的 `type.qualType` 提取参数类型列表
- 为每个 ctor 生成 `make_unique<T, Args...>(Args...)` 工厂
- 多 ctor → 多个 `with_X` Rust 关联函数
