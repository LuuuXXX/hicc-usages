# 013_inheritance_single: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Animal`（含 virtual ~Animal, virtual sound, pure virtual legs）
- `CXXRecordDecl Dog : Animal`（override sound/legs，新增 breed）
- `CXXRecordDecl Cat : Animal`（override sound/legs）

## hicc 模式

每个具体派生类（Dog/Cat）独立 `import_class!`。基类方法（`name()`）通过 C++ 继承在派生类实例上仍可调用，hicc 只需把方法签名挂在派生类的 `#[cpp(method = ...)]` 上。多态通过 `Animal*` 暂不暴露（需 `#[interface]` trait，本例不展示）。

```rust
#[cpp(class = "inheritance_single_ns::Dog")]
pub class Dog {
    #[cpp(method = "const std::string& name() const")]
    pub fn name(&self) -> &string;
    // ...
}
```

## 自动化可行性：高

读 `CXXRecordDecl` 的 `bases`：基类 `virtual` 方法 → 派生类 override 后签名一致。但抽象基类需用 `#[interface]`，需要单独判定（看 `isAbstract`）。
