# 017_virtual_override: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Shape`（虚 sides/describe）
- `CXXRecordDecl Triangle : Shape`（只 override sides）
- `CXXRecordDecl Pentagon : Shape`（override sides + describe）

## hicc 模式

派生类独立绑定，C++ vtable 自动 dispatch。Triangle 不 override describe → 用基类的；Pentagon override describe → 用派生类的。Rust 端只需把 describe 声明在两个派生类上：

```rust
#[cpp(class = "virtual_override_ns::Triangle")]
pub class Triangle {
    #[cpp(method = "int sides() const")]
    pub fn sides(&self) -> i32;
    #[cpp(method = "std::string describe() const")]
    pub fn describe(&self) -> string;
}
```

## 自动化可行性：高

`CXXMethodDecl.isOverride()` 在派生类中标记 override 方法。基类的非 override 方法（如 Triangle::describe 来自 Shape）也作为 method 在派生类导入，hicc 会通过 C++ 继承透明调用。
