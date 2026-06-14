# 011_class_const: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Temperature`
- `Temperature(float)`, `Temperature(float, const std::string&)`
- const 方法：`value() const`, `unit() const`, `to_fahrenheit() const`
- 非 const 方法：`set_value(float)`, `convert_to(const std::string&)`

## hicc 模式

`#[cpp(method = "T name(args) const")]` 在签名末尾保留 `const`。hicc 会生成 `const` 调用。

```rust
#[cpp(method = "float value() const")]
pub fn value(&self) -> f32;

#[cpp(method = "void set_value(float)")]
pub fn set_value(&mut self, v: f32);
```

## 自动化可行性：高

`CXXMethodDecl` 的 `isConst()` 直接读出。`isConst()=true` → `&self`，否则 `&mut self`。
