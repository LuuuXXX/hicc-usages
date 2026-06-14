# 009_class_move: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Holder`
- `Holder()`, `Holder(int, const std::string&)`, copy, move ctor
- `operator=(Holder&&)`, `operator+=(int)`
- `size() const`, `first() const`, `tag() const`

## hicc 模式

同 008（move 工厂模板）。**新增**：`operator+=` 不被 hicc 直接支持，在 `hicc::cpp!` 块内写 `inline void holder_add_to(Holder&, int)` 包装。

## 关键代码段

```rust
hicc::cpp! {
    inline void holder_add_to(class_move_ns::Holder& h, int delta) {
        h += delta;
    }
}

hicc::import_lib! {
    #[cpp(func = "void holder_add_to(class_move_ns::Holder&, int)")]
    pub fn holder_add_to(h: &mut Holder, delta: i32);
}
```

## 自动化可行性：高

operator 重载 → 包装函数的模式机械。识别 `CXXMethodDecl` 的 `name` 含 `operator` 前缀，自动生成 wrapper。
