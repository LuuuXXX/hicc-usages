# 024_template_function: AST → hicc 手动映射

## user-ast.json 关键信息

- `FunctionTemplateDecl add` → `FunctionDecl add T (const T&, const T&)`
- `FunctionTemplateDecl max_of` → `FunctionDecl max_of const T &(const T&, const T&)`
- `FunctionTemplateDecl describe` → `FunctionDecl describe std::string (const T&)`

## hicc 模式：⚠️ **必须显式实例化 + cpp! 包装**

模板函数符号不固定，C++ 端用 `template ... add<int>(...)` 显式实例化。Rust 端不能直接绑模板，因为：
1. hicc 不接 `const T&` 对原始类型 → 必须 cpp! 包装转 by value
2. 同一模板的不同实例化对应不同 Rust fn 名（add_int / add_double）

```rust
hicc::cpp! {
    inline int add_int_wrap(int a, int b) { return template_function_ns::add<int>(a, b); }
}
hicc::import_lib! {
    #[cpp(func = "int add_int_wrap(int, int)")]
    pub fn add_int(a: i32, b: i32) -> i32;
}
```

## 自动化可行性：中

需要枚举所有具现化（C++ 端 `template ... f<T>(...)` 声明），为每个生成包装。Rust fn 命名规则：`<base>_<T>`（如 `add_int`、`add_double`）。如果模板返回 `const T&`，**必须**用包装转 by value。
