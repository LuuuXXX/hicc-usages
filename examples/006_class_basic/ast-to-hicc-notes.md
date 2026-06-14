# 006_class_basic: AST → hicc 手动映射记录

## user-ast.json 关键信息

- `CXXRecordDecl Counter` (file class_basic.h:7)
- `CXXConstructorDecl Counter()` —— 默认构造
- `CXXConstructorDecl Counter(const std::string&)` —— explicit 命名构造
- `CXXMethodDecl inc() / inc_by(int) / reset() / count() const / name() const`
- `FieldDecl count_ (int), name_ (std::string)`

## hicc 模式选择

- 普通类 → `import_class!`
- 成员函数：const → `&self`，非 const → `&mut self`
- **构造函数不能直接放 `import_class!`** —— 必须用 `import_lib!` 中的工厂函数（C++ `hicc::make_unique<T>(...)`）+ `#[method(class, name)]` 宏包装到 Rust 关联函数

## 关键代码段

```rust
hicc::import_class! {
    #[cpp(class = "class_basic_ns::Counter")]
    pub class Counter {
        #[cpp(method = "void inc()")]
        pub fn inc(&mut self);
        #[cpp(method = "int count() const")]
        pub fn count(&self) -> i32;
        #[cpp(method = "const std::string& name() const")]
        pub fn name(&self) -> &hicc_std::string;
    }
}

hicc::import_lib! {
    #![link_name = "class_basic"]
    #[cpp(func = "std::unique_ptr<class_basic_ns::Counter> hicc::make_unique<class_basic_ns::Counter>()")]
    #[method(class = Counter, name = new)]
    pub fn counter_new() -> Counter;
}
```

## AST 提取要点

- `CXXMethodDecl` 的 `type.qualType` 中末尾 `const` 决定 `&self` vs `&mut self`
- 返回 `const std::string&` → Rust 端用 `&hicc_std::string`（生命周期由 `&self` 隐式约束）
- 构造函数：从 `CXXConstructorDecl` 的 `type.qualType` 提取参数列表，再 wrap 成 `make_unique<T, Args>(Args...)` 工厂

## 自动化可行性：中-高

- 类成员函数映射机械
- 构造函数需要：1) 识别 CXXConstructorDecl；2) 生成 make_unique 工厂；3) 决定如何处理 explicit / 多构造函数（同 C++ 类多个 ctor → Rust 多个 with_xxx 关联函数）
- 字段访问（FieldDecl）需要决定暴露策略（直接暴露还是只通过 getter）
