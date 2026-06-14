# 015_virtual_basic: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Shape`（虚 area/perimeter/describe，有默认实现）
- `CXXRecordDecl Rectangle : Shape`（override area/perimeter）
- `CXXRecordDecl Ellipse : Shape`（override area/perimeter，调用 `std::sqrt`）

## hicc 模式

派生类独立绑定。C++ 中 `rect.area()` 自动走派生类 vtable，hicc 只需 `#[cpp(method = "float area() const")]` 在派生类上声明同名方法即可。

```rust
#[cpp(class = "virtual_basic_ns::Rectangle")]
pub class Rectangle {
    #[cpp(method = "float area() const")]
    pub fn area(&self) -> f32;
    // ...
}
```

## 自动化可行性：高

派生类 override 的方法名 + 签名与基类一致。扫描派生类 `methods` 时若 `isOverride()` 或同名 virtual 在 base 中存在，直接生成 `#[cpp(method = "...")]`。
