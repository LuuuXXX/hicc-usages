# 014_inheritance_multiple: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Drawable`（纯虚 draw、虚 shape）
- `CXXRecordDecl Serializable`（纯虚 serialize、虚 bytes）
- `CXXRecordDecl Circle : Drawable, Serializable`（实现所有纯虚 + 新增 radius）
- `CXXRecordDecl Square : Drawable, Serializable`

## hicc 模式

**hicc 当前不支持多继承**（`#[interface]` 文档明示）。绕开：只绑定具体派生类（Circle/Square），多继承基类不暴露。基类方法（shape/serialize/bytes/draw）通过派生类 override 后的版本调用。

```rust
#[cpp(class = "inheritance_multiple_ns::Circle")]
pub class Circle {
    #[cpp(method = "void draw() const")]
    pub fn draw(&self);
    #[cpp(method = "std::string shape() const")]
    pub fn shape(&self) -> string;
    // ...
}
```

## 自动化可行性：中

派生类 `bases` 多于 1 个 → 不能用 `#[interface]`，只能选具体派生类绑定。需要为每个 `bases` 项目提供派生类调用的全部方法（继承自不同基类），扫描时需要递归遍历 base 类的 method decl。
