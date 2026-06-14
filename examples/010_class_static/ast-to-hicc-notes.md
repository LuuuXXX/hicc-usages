# 010_class_static: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Counter`
- `Counter()`, `~Counter()`
- 实例方法：`inc()`, `count() const`, `id() const`
- 静态方法：`alive()`, `next_id()`, `species()`（返回 `const std::string&`）
- 静态字段：`s_alive_`, `s_next_id_`, `s_species_`, `s_total_created`

## hicc 模式

- **实例方法**：走 `import_class!` 的 `#[cpp(method = ...)]`
- **静态方法/字段**：hicc 不直接支持，用 `cpp!` 块写命名空间级 `inline` 包装函数，再 `import_lib!` 导出

## 关键代码段

```rust
hicc::cpp! {
    inline int class_static_alive() { return class_static_ns::Counter::alive(); }
    inline void class_static_add_total(int n) { class_static_ns::Counter::s_total_created += n; }
}

hicc::import_lib! {
    #[cpp(func = "int class_static_alive()")]
    pub fn alive() -> i32;
    #[cpp(func = "void class_static_add_total(int)")]
    pub fn add_total(n: i32);
}
```

## 自动化可行性：高

识别 `CXXMethodDecl` 中 `storageClass == static`（或 method 的 isStatic）→ 自动生成包装函数。静态字段 `VarDecl` 同理。
