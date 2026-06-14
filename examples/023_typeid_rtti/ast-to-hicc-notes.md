# 023_typeid_rtti: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Base`（虚 name）
- `CXXRecordDecl DerivedA : Base` / `DerivedB : Base`
- 自由函数: `type_name_base(const Base&)`, `same_type(const Base&, const Base&)`, `is_derived_a(const Base&)`

## hicc 模式：⚠️ **typeid 必须包装**

`typeid(T)` 返回 `std::type_info`，C++ 内部 ABI 类型，不能直接 FFI。**C++ 端已写好的命名空间级包装函数**（type_name_base 等）直接 import_lib! 绑定。每个派生类独立 import_class!。

```rust
#[cpp(func = "const char* typeid_rtti_ns::type_name_base(const typeid_rtti_ns::Base&)")]
pub fn type_name_base_a(a: &DerivedA) -> *const i8;
```

注意：因为 `Base&` 是抽象类不能直接绑，所以包装函数按派生类参数暴露（多个派生类 → 多个绑定，但都调同一个 C++ 包装）。

## 自动化可行性：中

需要识别 `typeid` 用法 → 强制要求 C++ 端提供包装。Rust 端按每个具体派生类绑定包装函数（类型为派生类时，C++ 隐式转 Base&）。
