# 027_template_instantiation: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateDecl Pair<T1, T2>`
- 显式实例化 `template class Pair<int>;` 与 `template class Pair<std::string>;`
- `first()`、`second()`、`sum()`、`swap()` 等方法

## hicc 模式：✅ **using 别名 + make_unique 工厂**

每个具现化用 `using` 在 cpp! 块里起新名字，import_class! 用别名导入：

```rust
hicc::cpp! {
    using PairInt = template_instantiation_ns::Pair<int>;
    using PairString = template_instantiation_ns::Pair<std::string>;
}

hicc::import_class! {
    class string = hicc_std::string;
    #[cpp(class = "PairInt")]
    pub class PairInt { ... pub fn new(a: i32, b: i32) -> Self { pair_int_new(a, b) } }
}
```

工厂 `pair_int_new` 用 `hicc::make_unique<PairInt, int, int>(int&&, int&&)` —— 对原始类型用 `&&`（forwarding ref）。
对 `PairString` 用 `const std::string&`（class 类型允许）。

## 自动化可行性：高

显式实例化声明（`ClassTemplateSpecializationDecl` 或 `ExternTemplateDecl`）直接映射为 Rust 端 `using` + import_class!。每个方法直接生成 method 绑定。
