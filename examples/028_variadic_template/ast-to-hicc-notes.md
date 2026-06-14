# 028_variadic_template: AST → hicc 手动映射

## user-ast.json 关键信息

- `FunctionTemplateDecl sum<typename... Args>`（变参模板函数）
- `FunctionTemplateDecl format<typename... Args>`
- 调用点 `sum<int,int,int>(...)`、`format<string,string,string>(...)` 等

## hicc 模式：✅ **每个 arity 一个具现化包装**

变参模板不能跨 FFI 直接暴露。**手动在 cpp! 块为每个调用的 arity 写一个包装**：

```rust
hicc::cpp! {
    inline int sum_two(int a, int b) { return variadic_template_ns::sum<int, int>(a, b); }
    inline int sum_three(int a, int b, int c) { return variadic_template_ns::sum<int, int, int>(a, b, c); }
    inline std::string format_three(const std::string& a, const std::string& b, const std::string& c) {
        return variadic_template_ns::format<std::string, std::string, std::string>(a, b, c);
    }
}
```

每个包装独立 import_lib! 绑定。string 用 `class string = hicc_std::string;` 注册。

## 自动化可行性：中

需识别 `FunctionTemplateDecl` + 调用点 `DeclRefExpr` 的具现化 arity；为每个唯一的 `(name, arg_count, arg_types)` 三元组生成一个 inline 包装函数。对自动展开包长度（arity 1..N）可行，但需调用点信息驱动。
